//! Same-transaction, bounded labels for the already resolved procedure closure.

use super::{CaptureMeter, bounded, field};
use conceptweave_observation::{
    ProcedureSecurityLabel, ProcedureSecurityLabelsObservation,
    ReferencedProcedureDefinitionObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use futures_util::TryStreamExt;
use std::collections::BTreeMap;
use tokio_postgres::{Transaction, types::ToSql};

pub(super) async fn capture(
    tx: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    definitions: &BTreeMap<u32, ReferencedProcedureDefinitionObservation>,
) -> Result<Vec<ProcedureSecurityLabelsObservation>, SourceObservationFailure> {
    let oids: Vec<u32> = definitions.keys().copied().collect();
    let max_bytes = request.request().limits().max_bytes().min(i64::MAX as u64) as i64;
    let stream = bounded(
        request,
        cancellation,
        tx.query_raw(
            "WITH sized AS (SELECT ref.oid, l.objsubid, l.provider, l.label, \
         octet_length(l.provider)::bigint + octet_length(l.label) + 4 AS byte_count \
         FROM unnest($1::oid[]) ref(oid) JOIN pg_catalog.pg_seclabel l ON l.objoid = ref.oid \
         AND l.classoid = 'pg_catalog.pg_proc'::regclass) \
         SELECT oid, objsubid, byte_count, \
         CASE WHEN byte_count <= $2 THEN provider END, CASE WHEN byte_count <= $2 THEN label END \
         FROM sized ORDER BY oid, provider COLLATE \"C\"",
            vec![&oids as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut labels: BTreeMap<u32, Vec<ProcedureSecurityLabel>> =
        oids.iter().map(|oid| (*oid, Vec::new())).collect();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        if field::<i32>(&row, 1)? != 0 {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
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
        let label =
            ProcedureSecurityLabel::new(field::<String>(&row, 3)?, field::<String>(&row, 4)?)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        labels
            .get_mut(&field::<u32>(&row, 0)?)
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?
            .push(label);
    }
    labels
        .into_iter()
        .map(|(oid, labels)| {
            ProcedureSecurityLabelsObservation::new(definitions[&oid].location().clone(), labels)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)
        })
        .collect()
}
