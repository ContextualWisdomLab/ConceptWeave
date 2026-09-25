use conceptweave_observation::{
    QualifiedCollationName, QualifiedOperatorClassName, QualifiedRangeProcedure, QualifiedTypeName,
    RangeCatalogObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use tokio_postgres::{Transaction, types::ToSql};

use super::{CaptureMeter, bounded, field};

fn optional_procedure(
    schema: Option<String>,
    name: Option<String>,
) -> Result<Option<QualifiedRangeProcedure>, SourceObservationFailure> {
    match (schema, name) {
        (None, None) => Ok(None),
        (Some(schema), Some(name)) => QualifiedRangeProcedure::new(schema, name)
            .map(Some)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata),
        _ => Err(SourceObservationFailure::InvalidCapturedMetadata),
    }
}

pub(super) async fn capture(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
) -> Result<Vec<RangeCatalogObservation>, SourceObservationFailure> {
    let mut observations = Vec::new();
    for schema in request.request().allowed_schema_names() {
        let stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT n.nspname::text, t.typname::text, sn.nspname::text, s.typname::text, \
                 onsp.nspname::text, opc.opcname::text, am.amname::text, \
                 cn.nspname::text, coll.collname::text, \
                 cpn.nspname::text, cp.proname::text, \
                 dpn.nspname::text, dp.proname::text, \
                 r.rngcollation = 0 OR coll.oid IS NOT NULL, \
                 r.rngcanonical = 0 OR (cp.pronargs = 1 AND cp.proargtypes[0] = t.oid \
                   AND cp.prorettype = t.oid), \
                 r.rngsubdiff = 0 OR (dp.pronargs = 2 AND dp.proargtypes[0] = s.oid \
                   AND dp.proargtypes[1] = s.oid \
                   AND dp.prorettype = 'pg_catalog.float8'::regtype) \
                 FROM pg_catalog.pg_range r \
                 JOIN pg_catalog.pg_type t ON t.oid = r.rngtypid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace \
                 JOIN pg_catalog.pg_type s ON s.oid = r.rngsubtype \
                 JOIN pg_catalog.pg_namespace sn ON sn.oid = s.typnamespace \
                 JOIN pg_catalog.pg_opclass opc ON opc.oid = r.rngsubopc \
                 JOIN pg_catalog.pg_namespace onsp ON onsp.oid = opc.opcnamespace \
                 JOIN pg_catalog.pg_am am ON am.oid = opc.opcmethod \
                 LEFT JOIN pg_catalog.pg_collation coll ON coll.oid = r.rngcollation \
                 LEFT JOIN pg_catalog.pg_namespace cn ON cn.oid = coll.collnamespace \
                 LEFT JOIN pg_catalog.pg_proc cp ON cp.oid = r.rngcanonical \
                 LEFT JOIN pg_catalog.pg_namespace cpn ON cpn.oid = cp.pronamespace \
                 LEFT JOIN pg_catalog.pg_proc dp ON dp.oid = r.rngsubdiff \
                 LEFT JOIN pg_catalog.pg_namespace dpn ON dpn.oid = dp.pronamespace \
                 WHERE n.nspname = $1 ORDER BY t.typname",
                vec![schema as &(dyn ToSql + Sync)],
            ),
        )
        .await?;
        tokio::pin!(stream);
        while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
            let range_schema: String = field(&row, 0)?;
            let range_name: String = field(&row, 1)?;
            let subtype_schema: String = field(&row, 2)?;
            let subtype_name: String = field(&row, 3)?;
            let opclass_schema: String = field(&row, 4)?;
            let opclass_name: String = field(&row, 5)?;
            let access_method: String = field(&row, 6)?;
            let collation_schema: Option<String> = field(&row, 7)?;
            let collation_name: Option<String> = field(&row, 8)?;
            let canonical_schema: Option<String> = field(&row, 9)?;
            let canonical_name: Option<String> = field(&row, 10)?;
            let diff_schema: Option<String> = field(&row, 11)?;
            let diff_name: Option<String> = field(&row, 12)?;
            let bytes = 24
                + [
                    &range_schema,
                    &range_name,
                    &subtype_schema,
                    &subtype_name,
                    &opclass_schema,
                    &opclass_name,
                    &access_method,
                ]
                .into_iter()
                .map(String::len)
                .sum::<usize>()
                + [
                    &collation_schema,
                    &collation_name,
                    &canonical_schema,
                    &canonical_name,
                    &diff_schema,
                    &diff_name,
                ]
                .into_iter()
                .flatten()
                .map(String::len)
                .sum::<usize>();
            meter.add(request, bytes)?;
            if access_method != "btree"
                || !(13..=15).all(|index| field::<bool>(&row, index) == Ok(true))
            {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            let collation = match (collation_schema, collation_name) {
                (None, None) => None,
                (Some(schema), Some(name)) => Some(
                    QualifiedCollationName::new(schema, name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                ),
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            observations.push(RangeCatalogObservation::new(
                QualifiedTypeName::new(range_schema, range_name)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                QualifiedTypeName::new(subtype_schema, subtype_name)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                QualifiedOperatorClassName::new(opclass_schema, opclass_name)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                collation,
                optional_procedure(canonical_schema, canonical_name)?,
                optional_procedure(diff_schema, diff_name)?,
            ));
        }
    }
    Ok(observations)
}
