//! PostgreSQL 18 catalog observation behind the Source Observation capability.
//!
//! This first adapter path admits complete schema-scoped domain and enum evidence when the
//! selected schemas contain no modeled relations. Relation-bearing captures fail closed until
//! their catalog families can be mapped without dropping material facts.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{collections::BTreeMap, future::Future, time::Duration};

use conceptweave_observation::{
    DomainCheckConstraintObservation, DomainObservation, EnumObservation, PostgresSchemaSnapshotV3,
    QualifiedCollationName, QualifiedTypeName,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
    SourceObservationPort,
};
use futures_util::TryStreamExt;
use tokio_postgres::{
    Config, IsolationLevel, NoTls, Row, config::Host, error::SqlState, types::ToSql,
};

/// Unix-socket PostgreSQL adapter with credential-bearing configurations keyed by the exact
/// registry source key and immutable policy binding. TCP is rejected until a verifying TLS
/// connector is configured; configurations and their credentials never enter domain values.
pub struct PostgresUnixAdapter {
    connections: BTreeMap<String, (String, Config)>,
}

impl PostgresUnixAdapter {
    /// Installs adapter-local connection configurations for exact authorized source bindings.
    /// Only explicitly configured Unix sockets are accepted by this transport.
    #[must_use]
    pub fn new(connections: BTreeMap<String, (String, Config)>) -> Self {
        Self { connections }
    }
}

impl SourceObservationPort for PostgresUnixAdapter {
    type Snapshot = PostgresSchemaSnapshotV3;

    async fn observe<'a>(
        &'a self,
        request: AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure> {
        let (active_binding, config) = self
            .connections
            .get(request.source_connection().source_connection_key())
            .ok_or(SourceObservationFailure::SourceUnavailable)?;
        if active_binding != request.source_connection().connection_policy_binding() {
            return Err(SourceObservationFailure::SourceUnavailable);
        }
        if config.get_hosts().is_empty()
            || config
                .get_hosts()
                .iter()
                .any(|host| matches!(host, Host::Tcp(_)))
        {
            return Err(SourceObservationFailure::SourceUnavailable);
        }
        let (mut client, connection) =
            bounded(&request, cancellation, config.connect(NoTls)).await?;
        let connection_task = tokio::spawn(connection);
        let result = capture_type_only(&mut client, &request, cancellation).await;
        connection_task.abort();
        result
    }
}

#[derive(Default)]
struct CaptureMeter {
    rows: u64,
    bytes: u64,
}

impl CaptureMeter {
    fn add(
        &mut self,
        request: &AuthorizedObservationRequest,
        byte_count: usize,
    ) -> Result<(), SourceObservationFailure> {
        let limits = request.request().limits();
        self.rows = self.rows.saturating_add(1);
        self.bytes = self.bytes.saturating_add(byte_count as u64);
        if self.rows > limits.max_rows() {
            return Err(SourceObservationFailure::RowLimitExceeded {
                max_rows: limits.max_rows(),
            });
        }
        if self.bytes > limits.max_bytes() {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: limits.max_bytes(),
            });
        }
        Ok(())
    }
}

struct TypeRow {
    oid: u32,
    name: String,
    kind: String,
    base_schema: Option<String>,
    base_name: Option<String>,
    modifier: i32,
    dimensions: i32,
    collation_schema: Option<String>,
    collation_name: Option<String>,
    not_null: bool,
    default_expression: Option<String>,
    comment: Option<String>,
    oversized: bool,
}

async fn capture_type_only(
    client: &mut tokio_postgres::Client,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
) -> Result<PostgresSchemaSnapshotV3, SourceObservationFailure> {
    let limits = request.request().limits();
    let transaction = bounded(
        request,
        cancellation,
        client
            .build_transaction()
            .isolation_level(IsolationLevel::RepeatableRead)
            .read_only(true)
            .start(),
    )
    .await?;
    let transaction_timeout = request
        .remaining_operation_budget()
        .ok_or(SourceObservationFailure::OperationTimeout)?
        .as_millis()
        .min(i32::MAX as u128)
        .max(1);
    let statement_timeout = limits.statement_timeout_ms().min(i32::MAX as u64);
    let lock_timeout = statement_timeout.min(1_000);
    bounded(
        request,
        cancellation,
        transaction.batch_execute(&format!(
            "SET LOCAL transaction_timeout = '{transaction_timeout}ms'; \
             SET LOCAL statement_timeout = '{statement_timeout}ms'; \
             SET LOCAL lock_timeout = '{lock_timeout}ms'"
        )),
    )
    .await?;
    let version: i32 = bounded(
        request,
        cancellation,
        transaction.query_one("SELECT current_setting('server_version_num')::int", &[]),
    )
    .await?
    .try_get(0)
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    if !(180_000..190_000).contains(&version) {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    let transaction_mode = bounded(
        request,
        cancellation,
        transaction.query_one(
            "SELECT current_setting('transaction_read_only') = 'on', \
             current_setting('transaction_isolation') = 'repeatable read'",
            &[],
        ),
    )
    .await?;
    if !field::<bool>(&transaction_mode, 0)? || !field::<bool>(&transaction_mode, 1)? {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }

    let mut meter = CaptureMeter::default();
    let max_bytes = limits.max_bytes().min(i64::MAX as u64) as i64;
    let mut types = Vec::new();
    for schema in request.request().allowed_schema_names() {
        let schema_row = bounded(
            request,
            cancellation,
            transaction.query_opt(
                "SELECT oid FROM pg_catalog.pg_namespace WHERE nspname = $1",
                &[schema],
            ),
        )
        .await?
        .ok_or(SourceObservationFailure::SourceUnavailable)?;
        meter.add(request, schema.len() + 4)?;
        let schema_oid: u32 = field(&schema_row, 0)?;
        let relation_exists = bounded(
            request,
            cancellation,
            transaction.query_opt(
                "SELECT 1 FROM pg_catalog.pg_class WHERE relnamespace = $1 AND relkind IN ('r','p','v','m','f','S','c') LIMIT 1",
                &[&schema_oid],
            ),
        )
        .await?
        .is_some();
        if relation_exists {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }

        let stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT t.oid, t.typname::text, t.typtype::text, bn.nspname::text, bt.typname::text, \
                 t.typtypmod, t.typndims, cn.nspname::text, coll.collname::text, t.typnotnull, \
                 CASE WHEN octet_length(metadata.default_text) <= $2::bigint THEN metadata.default_text END, \
                 CASE WHEN octet_length(metadata.comment_text) <= $2::bigint THEN metadata.comment_text END, \
                 COALESCE(octet_length(metadata.default_text) > $2::bigint, false) \
                   OR COALESCE(octet_length(metadata.comment_text) > $2::bigint, false) \
                 FROM pg_catalog.pg_type t \
                 LEFT JOIN pg_catalog.pg_type bt ON bt.oid = t.typbasetype \
                 LEFT JOIN pg_catalog.pg_namespace bn ON bn.oid = bt.typnamespace \
                 LEFT JOIN pg_catalog.pg_collation coll ON coll.oid = t.typcollation \
                 LEFT JOIN pg_catalog.pg_namespace cn ON cn.oid = coll.collnamespace \
                 CROSS JOIN LATERAL (SELECT \
                   COALESCE(pg_catalog.pg_get_expr(t.typdefaultbin, 0), t.typdefault) AS default_text, \
                   pg_catalog.obj_description(t.oid, 'pg_type') AS comment_text) metadata \
                 WHERE t.typnamespace = $1 AND t.typtype IN ('d','e') ORDER BY t.typname",
                vec![&schema_oid as &(dyn ToSql + Sync), &max_bytes],
            ),
        )
        .await?;
        tokio::pin!(stream);
        while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
            let observed = TypeRow {
                oid: field(&row, 0)?,
                name: field(&row, 1)?,
                kind: field(&row, 2)?,
                base_schema: field(&row, 3)?,
                base_name: field(&row, 4)?,
                modifier: field(&row, 5)?,
                dimensions: field(&row, 6)?,
                collation_schema: field(&row, 7)?,
                collation_name: field(&row, 8)?,
                not_null: field(&row, 9)?,
                default_expression: field(&row, 10)?,
                comment: field(&row, 11)?,
                oversized: field(&row, 12)?,
            };
            if observed.oversized {
                return Err(SourceObservationFailure::ByteLimitExceeded {
                    max_bytes: limits.max_bytes(),
                });
            }
            let bytes = 16
                + observed.name.len()
                + observed.kind.len()
                + observed.base_schema.as_ref().map_or(0, String::len)
                + observed.base_name.as_ref().map_or(0, String::len)
                + observed.collation_schema.as_ref().map_or(0, String::len)
                + observed.collation_name.as_ref().map_or(0, String::len)
                + observed.default_expression.as_ref().map_or(0, String::len)
                + observed.comment.as_ref().map_or(0, String::len);
            meter.add(request, bytes)?;
            types.push((schema.clone(), observed));
        }
    }

    let mut domains = Vec::new();
    let mut enums = Vec::new();
    for (schema, observed) in types {
        match observed.kind.as_str() {
            "d" => {
                let (Some(base_schema), Some(base_name)) =
                    (observed.base_schema, observed.base_name)
                else {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                };
                let mut domain = DomainObservation::new(
                    &schema,
                    observed.name,
                    QualifiedTypeName::new(base_schema, base_name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                )
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
                .with_type_modifier(observed.modifier)
                .with_array_dimensions(
                    observed
                        .dimensions
                        .try_into()
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                )
                .with_not_null(observed.not_null);
                match (observed.collation_schema, observed.collation_name) {
                    (Some(collation_schema), Some(collation_name)) => {
                        domain = domain.with_collation(
                            QualifiedCollationName::new(collation_schema, collation_name)
                                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                        );
                    }
                    (None, None) => {}
                    _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
                }
                if let Some(default_expression) = observed.default_expression {
                    domain = domain.with_default_expression(default_expression);
                }
                if let Some(comment) = observed.comment {
                    domain = domain.with_source_comment(comment);
                }
                let stream = bounded(
                    request,
                    cancellation,
                    transaction.query_raw(
                        "SELECT conname::text, contype::text, \
                         CASE WHEN octet_length(pg_catalog.pg_get_constraintdef(oid, false)) <= $2::bigint \
                           THEN pg_catalog.pg_get_constraintdef(oid, false) END, \
                         convalidated, conenforced, \
                         octet_length(pg_catalog.pg_get_constraintdef(oid, false)) > $2::bigint \
                         FROM pg_catalog.pg_constraint WHERE contypid = $1 ORDER BY conname, oid",
                        vec![&observed.oid as &(dyn ToSql + Sync), &max_bytes],
                    ),
                )
                .await?;
                tokio::pin!(stream);
                let mut checks = Vec::new();
                while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
                    let name: String = field(&row, 0)?;
                    let kind: String = field(&row, 1)?;
                    let definition: Option<String> = field(&row, 2)?;
                    let validated: bool = field(&row, 3)?;
                    let enforced: bool = field(&row, 4)?;
                    let oversized: bool = field(&row, 5)?;
                    if oversized {
                        return Err(SourceObservationFailure::ByteLimitExceeded {
                            max_bytes: limits.max_bytes(),
                        });
                    }
                    let definition =
                        definition.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                    meter.add(request, name.len() + kind.len() + definition.len() + 2)?;
                    if kind != "c" {
                        return Err(SourceObservationFailure::InvalidCapturedMetadata);
                    }
                    checks.push(
                        DomainCheckConstraintObservation::new(
                            name, definition, validated, enforced,
                        )
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                    );
                }
                domains.push(
                    domain
                        .with_check_constraints(checks)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                );
            }
            "e" => {
                if observed.base_schema.is_some()
                    || observed.base_name.is_some()
                    || observed.modifier != -1
                    || observed.dimensions != 0
                    || observed.collation_schema.is_some()
                    || observed.collation_name.is_some()
                    || observed.not_null
                    || observed.default_expression.is_some()
                {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
                let stream = bounded(
                    request,
                    cancellation,
                    transaction.query_raw(
                        "SELECT enumlabel::text FROM pg_catalog.pg_enum WHERE enumtypid = $1 ORDER BY enumsortorder, oid",
                        &[&observed.oid],
                    ),
                )
                .await?;
                tokio::pin!(stream);
                let mut labels = Vec::new();
                while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
                    let label: String = field(&row, 0)?;
                    meter.add(request, label.len())?;
                    labels.push(label);
                }
                let mut observed_enum = EnumObservation::new(schema, observed.name, labels)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
                if let Some(comment) = observed.comment {
                    observed_enum = observed_enum.with_source_comment(comment);
                }
                enums.push(observed_enum);
            }
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
    }
    let observed_at_utc: String = field(
        &bounded(
            request,
            cancellation,
            transaction.query_one(
                "SELECT to_char(clock_timestamp() AT TIME ZONE 'UTC', 'YYYY-MM-DD\"T\"HH24:MI:SS.MS\"Z\"')",
                &[],
            ),
        )
        .await?,
        0,
    )?;
    let snapshot = PostgresSchemaSnapshotV3::new(
        request,
        "postgres18_type_only_adapter_v1",
        observed_at_utc,
        Vec::new(),
        domains,
        enums,
    )
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    bounded(request, cancellation, transaction.commit()).await?;
    Ok(snapshot)
}

fn field<T>(row: &Row, index: usize) -> Result<T, SourceObservationFailure>
where
    T: tokio_postgres::types::FromSqlOwned,
{
    row.try_get(index)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)
}

async fn bounded<T, F>(
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    future: F,
) -> Result<T, SourceObservationFailure>
where
    F: Future<Output = Result<T, tokio_postgres::Error>>,
{
    if cancellation.is_cancelled() {
        return Err(SourceObservationFailure::Cancelled);
    }
    let remaining = request
        .remaining_operation_budget()
        .ok_or(SourceObservationFailure::OperationTimeout)?;
    tokio::select! {
        result = tokio::time::timeout(remaining, future) => match result {
            Ok(Ok(value)) => Ok(value),
            Ok(Err(error)) if error.code() == Some(&SqlState::QUERY_CANCELED) => {
                Err(SourceObservationFailure::StatementTimeout)
            }
            Ok(Err(_)) => Err(SourceObservationFailure::SourceUnavailable),
            Err(_) => Err(SourceObservationFailure::OperationTimeout),
        },
        () = wait_for_cancellation(cancellation) => Err(SourceObservationFailure::Cancelled),
    }
}

async fn wait_for_cancellation(cancellation: &dyn ObservationCancellation) {
    loop {
        if cancellation.is_cancelled() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
