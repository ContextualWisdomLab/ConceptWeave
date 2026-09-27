//! Bounded same-transaction extension closure for already resolved procedures.

use super::{CaptureMeter, bounded, field};
use conceptweave_observation::{
    ExtensionConfigurationTable, ProcedureExtensionDependenciesObservation,
    ProcedureExtensionDependency, ReferencedProcedureDefinitionObservation, RelationKind,
    SourceExtensionDefinition,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use std::collections::{BTreeMap, BTreeSet};
use tokio_postgres::{Transaction, types::ToSql};

struct ExtensionHeader {
    name: String,
    owner: u32,
    owner_name: Option<String>,
    schema: String,
    relocatable: bool,
    version: String,
    configuration: Option<Vec<ExtensionConfigurationTable>>,
    count: usize,
}

pub(super) async fn capture(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    definitions: &BTreeMap<u32, ReferencedProcedureDefinitionObservation>,
) -> Result<
    (
        Vec<ProcedureExtensionDependenciesObservation>,
        Vec<SourceExtensionDefinition>,
    ),
    SourceObservationFailure,
> {
    let oids: Vec<u32> = definitions.keys().copied().collect();
    let mut edges: BTreeMap<u32, Vec<ProcedureExtensionDependency>> =
        oids.iter().map(|oid| (*oid, Vec::new())).collect();
    let mut extension_oids = BTreeSet::new();
    let stream = bounded(request, cancellation, tx.query_raw(
        "SELECT ref.oid,d.objsubid,d.refobjsubid,d.deptype::text,d.refobjid,e.extname::text,d.refclassid='pg_catalog.pg_extension'::regclass \
         FROM unnest($1::oid[]) ref(oid) JOIN pg_catalog.pg_depend d \
         ON d.classid='pg_catalog.pg_proc'::regclass AND d.objid=ref.oid \
         AND (d.refclassid='pg_catalog.pg_extension'::regclass OR d.deptype IN ('e','x')) \
         LEFT JOIN pg_catalog.pg_extension e ON e.oid=d.refobjid ORDER BY ref.oid,d.deptype,e.extname",
        vec![&oids as &(dyn ToSql + Sync)],
    )).await?;
    tokio::pin!(stream);
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        if field::<i32>(&row, 1)? != 0 || field::<i32>(&row, 2)? != 0 || !field::<bool>(&row, 6)? {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let kind: String = field(&row, 3)?;
        let [kind] = kind.as_bytes() else {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        };
        let name: String = field(&row, 5)?;
        meter.add(request, name.len() + 17)?;
        let edge = ProcedureExtensionDependency::new(char::from(*kind), name)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        edges
            .get_mut(&field::<u32>(&row, 0)?)
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?
            .push(edge);
        extension_oids.insert(field::<u32>(&row, 4)?);
    }
    let extension_oids: Vec<u32> = extension_oids.into_iter().collect();
    let max_bytes = request.request().limits().max_bytes().min(i64::MAX as u64) as i64;
    let max_rows = request.request().limits().max_rows().min(i64::MAX as u64) as i64;
    let stream = bounded(request,cancellation,tx.query_raw(
        "WITH sized AS (SELECT ref.oid,e.oid IS NULL AS missing,e.extname,e.extowner,e.extrelocatable,e.extversion,e.extconfig,e.extcondition,r.rolname::text AS owner_name,n.nspname::text AS object_schema, \
         octet_length(e.extname::text)::bigint+COALESCE(octet_length(r.rolname::text),0)+octet_length(n.nspname::text)+octet_length(e.extversion)+9 AS byte_count \
         FROM unnest($1::oid[]) ref(oid) LEFT JOIN pg_catalog.pg_extension e ON e.oid=ref.oid \
         LEFT JOIN pg_catalog.pg_roles r ON r.oid=e.extowner LEFT JOIN pg_catalog.pg_namespace n ON n.oid=e.extnamespace) \
         SELECT oid,missing,byte_count,CASE WHEN byte_count <= $2 THEN extname::text END,extowner, \
         CASE WHEN byte_count <= $2 THEN owner_name END,CASE WHEN byte_count <= $2 THEN object_schema END,extrelocatable, \
         CASE WHEN byte_count <= $2 THEN extversion END,extconfig IS NULL,extcondition IS NULL, \
         COALESCE(cardinality(extconfig),0),COALESCE(cardinality(extcondition),0),array_ndims(extconfig),array_ndims(extcondition),array_lower(extconfig,1),array_lower(extcondition,1), \
         CASE WHEN cardinality(extconfig)::bigint <= $3 AND cardinality(extcondition)::bigint <= $3 THEN \
         EXISTS(SELECT 1 FROM unnest(extconfig) x WHERE x IS NULL) OR EXISTS(SELECT 1 FROM unnest(extcondition) x WHERE x IS NULL) ELSE false END \
         FROM sized ORDER BY oid",
        vec![&extension_oids as &(dyn ToSql + Sync),&max_bytes,&max_rows],
    )).await?;
    tokio::pin!(stream);
    let mut headers = BTreeMap::new();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        if field::<bool>(&row, 1)? {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let count: i32 = field(&row, 11)?;
        let conditions: i32 = field(&row, 12)?;
        if i64::from(count) > max_rows || i64::from(conditions) > max_rows {
            return Err(SourceObservationFailure::RowLimitExceeded {
                max_rows: request.request().limits().max_rows(),
            });
        }
        let bytes: i64 = field(&row, 2)?;
        if bytes > max_bytes {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        let is_null: bool = field(&row, 9)?;
        if count < 0
            || count != conditions
            || is_null != field::<bool>(&row, 10)?
            || field::<bool>(&row, 17)?
            || (count > 0
                && !(13..=16).all(|index| field::<Option<i32>>(&row, index) == Ok(Some(1))))
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        meter.add(
            request,
            usize::try_from(bytes)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
                + 9,
        )?;
        let header = ExtensionHeader {
            name: field(&row, 3)?,
            owner: field(&row, 4)?,
            owner_name: field(&row, 5)?,
            schema: field(&row, 6)?,
            relocatable: field(&row, 7)?,
            version: field(&row, 8)?,
            configuration: if is_null { None } else { Some(Vec::new()) },
            count: count as usize,
        };
        if headers.insert(field::<u32>(&row, 0)?, header).is_some() {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
    }
    if headers.len() != extension_oids.len() {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    let stream = bounded(request,cancellation,tx.query_raw(
        "WITH sized AS (SELECT e.oid,t.position,n.nspname::text AS schema_name,c.relname::text AS table_name,c.relkind::text AS kind,e.extcondition[t.position] AS condition, \
         octet_length(n.nspname::text)::bigint+octet_length(c.relname::text)+octet_length(e.extcondition[t.position])+17 AS byte_count \
         FROM unnest($1::oid[]) ref(oid) JOIN pg_catalog.pg_extension e ON e.oid=ref.oid \
         CROSS JOIN LATERAL unnest(e.extconfig) WITH ORDINALITY t(table_oid,position) \
         LEFT JOIN pg_catalog.pg_class c ON c.oid=t.table_oid LEFT JOIN pg_catalog.pg_namespace n ON n.oid=c.relnamespace) \
         SELECT oid,position,byte_count,CASE WHEN byte_count <= $2 THEN schema_name END,CASE WHEN byte_count <= $2 THEN table_name END,kind,CASE WHEN byte_count <= $2 THEN condition END \
         FROM sized ORDER BY oid,position",
        vec![&extension_oids as &(dyn ToSql + Sync),&max_bytes],
    )).await?;
    tokio::pin!(stream);
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let bytes: i64 = field(&row, 2)?;
        if bytes > max_bytes {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        meter.add(
            request,
            usize::try_from(bytes)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        )?;
        let kind = match field::<String>(&row, 5)?.as_str() {
            "r" => RelationKind::Table,
            "p" => RelationKind::PartitionedTable,
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        };
        let table = ExtensionConfigurationTable::new(
            field::<String>(&row, 3)?,
            field::<String>(&row, 4)?,
            kind,
            field::<String>(&row, 6)?,
        )
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let config = headers
            .get_mut(&field::<u32>(&row, 0)?)
            .and_then(|header| header.configuration.as_mut())
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        if field::<i64>(&row, 1)? != config.len() as i64 + 1 {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        config.push(table);
    }
    let extensions = headers
        .into_values()
        .map(|header| {
            if header.configuration.as_ref().map_or(0, Vec::len) != header.count {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            SourceExtensionDefinition::new(
                header.name,
                header.owner,
                header.owner_name,
                header.schema,
                header.relocatable,
                header.version,
                header.configuration,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let dependencies = edges
        .into_iter()
        .map(|(oid, edges)| {
            ProcedureExtensionDependenciesObservation::new(
                definitions[&oid].location().clone(),
                edges,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((dependencies, extensions))
}
