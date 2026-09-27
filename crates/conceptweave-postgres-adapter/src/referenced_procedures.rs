//! Same-transaction definition and owner evidence for the parsed procedure closure.

use super::{CaptureMeter, bounded, field};
use conceptweave_observation::{
    QualifiedTypeName, ReferencedProcedureDefinitionObservation, ReferencedProcedureLocation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use tokio_postgres::{Transaction, types::ToSql};

pub(super) async fn capture(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    procedure_oids: Vec<u32>,
) -> Result<Vec<ReferencedProcedureDefinitionObservation>, SourceObservationFailure> {
    let max_bytes = request.request().limits().max_bytes().min(i64::MAX as u64) as i64;
    let stream = bounded(
        request,
        cancellation,
        transaction.query_raw(
            "WITH material AS MATERIALIZED ( \
         SELECT ref.oid, n.nspname::text AS schema_name, p.proname::text AS procedure_name, \
           p.proowner, r.rolname::text AS owner_name, p.pronargs, \
           rn.nspname::text AS result_schema, rt.typname::text AS result_name, \
           pg_catalog.pg_get_functiondef(p.oid) AS definition, \
           ARRAY(SELECT an.nspname::text FROM unnest(p.proargtypes::oid[]) \
             WITH ORDINALITY arg(type_oid, position) \
             LEFT JOIN pg_catalog.pg_type at ON at.oid = arg.type_oid \
             LEFT JOIN pg_catalog.pg_namespace an ON an.oid = at.typnamespace \
             ORDER BY arg.position) AS argument_schemas, \
           ARRAY(SELECT at.typname::text FROM unnest(p.proargtypes::oid[]) \
             WITH ORDINALITY arg(type_oid, position) \
             LEFT JOIN pg_catalog.pg_type at ON at.oid = arg.type_oid \
             ORDER BY arg.position) AS argument_names \
         FROM unnest($1::oid[]) ref(oid) \
         LEFT JOIN pg_catalog.pg_proc p ON p.oid = ref.oid \
         LEFT JOIN pg_catalog.pg_namespace n ON n.oid = p.pronamespace \
         LEFT JOIN pg_catalog.pg_roles r ON r.oid = p.proowner \
         LEFT JOIN pg_catalog.pg_type rt ON rt.oid = p.prorettype \
         LEFT JOIN pg_catalog.pg_namespace rn ON rn.oid = rt.typnamespace \
         ), sized AS (SELECT *, \
           octet_length(schema_name)::bigint + octet_length(procedure_name) \
           + octet_length(owner_name) + octet_length(result_schema) + octet_length(result_name) \
           + octet_length(definition) + octet_length(array_to_string(argument_schemas, '')) \
           + octet_length(array_to_string(argument_names, '')) + 8 AS byte_count FROM material) \
         SELECT byte_count > $2, oid, proowner, pronargs, byte_count, \
           CASE WHEN byte_count <= $2 THEN schema_name END, \
           CASE WHEN byte_count <= $2 THEN procedure_name END, \
           CASE WHEN byte_count <= $2 THEN owner_name END, \
           CASE WHEN byte_count <= $2 THEN result_schema END, \
           CASE WHEN byte_count <= $2 THEN result_name END, \
           CASE WHEN byte_count <= $2 THEN definition END, \
           CASE WHEN byte_count <= $2 THEN argument_schemas END, \
           CASE WHEN byte_count <= $2 THEN argument_names END FROM sized ORDER BY oid",
            vec![&procedure_oids as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut definitions = Vec::new();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        if field::<Option<bool>>(&row, 0)?
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?
        {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        let owner_oid: u32 = field(&row, 2)?;
        let argument_count: i16 = field(&row, 3)?;
        let byte_count: i64 = field(&row, 4)?;
        meter.add(
            request,
            usize::try_from(byte_count)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        )?;
        let schema: String = field(&row, 5)?;
        let name: String = field(&row, 6)?;
        let owner: String = field(&row, 7)?;
        let result_schema: String = field(&row, 8)?;
        let result_name: String = field(&row, 9)?;
        let definition: String = field(&row, 10)?;
        let argument_schemas: Vec<String> = field(&row, 11)?;
        let argument_names: Vec<String> = field(&row, 12)?;
        if usize::try_from(argument_count).ok() != Some(argument_schemas.len())
            || argument_schemas.len() != argument_names.len()
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let input_types = argument_schemas
            .into_iter()
            .zip(argument_names)
            .map(|(schema, name)| QualifiedTypeName::new(schema, name))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let location = ReferencedProcedureLocation::new(schema, name, input_types)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let result_type = QualifiedTypeName::new(result_schema, result_name)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        definitions.push(
            ReferencedProcedureDefinitionObservation::new(
                location,
                result_type,
                owner_oid,
                owner,
                definition,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
    }
    if definitions.len() != procedure_oids.len() {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    Ok(definitions)
}
