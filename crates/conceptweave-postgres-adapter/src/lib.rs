//! PostgreSQL 18 catalog observation behind the Source Observation capability.
//!
//! This bounded path admits schema-scoped domains, enums, ordinary tables, local CHECK,
//! NOT NULL, and ordinary key constraints, and bounded indexes.
//! Unsupported relation metadata fails closed until it can be mapped without losing facts.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

use std::{collections::BTreeMap, future::Future, time::Duration};

use conceptweave_observation::{
    CheckConstraintObservation, ColumnExpressionObservation, ColumnGenerationObservation,
    ColumnObservationV3, DomainCheckConstraintObservation, DomainObservation, EnumObservation,
    IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics,
    IndexObservation, IndexStorageOption, IndexTablespace, NotNullConstraintObservation,
    OperatorClassOption, PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedCollationName,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
    ReplicaIdentityMode, TableConstraintObservation, UniqueConstraintObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
    SourceObservationPort,
};
use futures_util::TryStreamExt;
use tokio_postgres::{
    Config, IsolationLevel, NoTls, Row, Transaction, config::Host, error::SqlState, types::ToSql,
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
        let result = capture_catalog(&mut client, &request, cancellation).await;
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

struct RelationRow {
    schema: String,
    oid: u32,
    name: String,
    replica_identity: ReplicaIdentityMode,
    comment: Option<String>,
}

async fn capture_catalog(
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
            "SET LOCAL search_path = pg_catalog; \
             SET LOCAL transaction_timeout = '{transaction_timeout}ms'; \
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
    let mut relation_rows = Vec::new();
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
        let relation_stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT c.oid, c.relname::text, c.relkind::text, c.relpersistence::text, \
                 c.relispartition, c.relreplident::text, c.relrowsecurity, c.relforcerowsecurity, \
                 c.relhasrules, c.relhastriggers, c.reloptions IS NOT NULL, c.relacl IS NOT NULL, \
                 c.reloftype <> 0, \
                 CASE WHEN octet_length(pg_catalog.obj_description(c.oid, 'pg_class')) <= $2::bigint \
                   THEN pg_catalog.obj_description(c.oid, 'pg_class') END, \
                 COALESCE(octet_length(pg_catalog.obj_description(c.oid, 'pg_class')) > $2::bigint, false) \
                 FROM pg_catalog.pg_class c WHERE c.relnamespace = $1 \
                   AND c.relkind IN ('r','p','v','m','f','S','c') ORDER BY c.relname",
                vec![&schema_oid as &(dyn ToSql + Sync), &max_bytes],
            ),
        )
        .await?;
        tokio::pin!(relation_stream);
        while let Some(row) = bounded(request, cancellation, relation_stream.try_next()).await? {
            let oid: u32 = field(&row, 0)?;
            let name: String = field(&row, 1)?;
            let kind: String = field(&row, 2)?;
            let persistence: String = field(&row, 3)?;
            let replica: String = field(&row, 5)?;
            let comment: Option<String> = field(&row, 13)?;
            if field::<bool>(&row, 14)? {
                return Err(SourceObservationFailure::ByteLimitExceeded {
                    max_bytes: limits.max_bytes(),
                });
            }
            meter.add(
                request,
                24 + name.len()
                    + kind.len()
                    + persistence.len()
                    + replica.len()
                    + comment.as_ref().map_or(0, String::len),
            )?;
            // relhasrules and relhastriggers are lazy hints; the catalog-row guards below
            // decide whether any unsupported rule or trigger still exists.
            if kind != "r"
                || persistence != "p"
                || [4, 6, 7, 10, 11, 12]
                    .into_iter()
                    .any(|index| field::<bool>(&row, index) != Ok(false))
            {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            let replica_identity = match replica.as_str() {
                "d" => ReplicaIdentityMode::Default,
                "n" => ReplicaIdentityMode::Nothing,
                "f" => ReplicaIdentityMode::Full,
                "i" => ReplicaIdentityMode::Index,
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            relation_rows.push(RelationRow {
                schema: schema.clone(),
                oid,
                name,
                replica_identity,
                comment,
            });
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
    let mut relations = Vec::new();
    let mut column_generations = Vec::new();
    let mut column_expressions = Vec::new();
    let mut not_null_constraints = Vec::new();
    for relation in relation_rows {
        let (observed, generations, expressions, not_null) = capture_relation(
            &transaction,
            request,
            cancellation,
            &mut meter,
            max_bytes,
            relation,
        )
        .await?;
        relations.push(observed);
        column_generations.extend(generations);
        column_expressions.extend(expressions);
        not_null_constraints.extend(not_null);
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
    let extractor_revision = if relations.is_empty() {
        "postgres18_type_only_adapter_v2"
    } else {
        "postgres18_not_null_primary_adapter_v1"
    };
    let snapshot = if relations.is_empty() {
        PostgresSchemaSnapshotV3::new(
            request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )
    } else {
        PostgresSchemaSnapshotV3::new_with_column_expressions(
            request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
            column_generations,
            column_expressions,
        )
    }
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
    .with_observed_not_null_constraints(not_null_constraints)
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    bounded(request, cancellation, transaction.commit()).await?;
    Ok(snapshot)
}

async fn capture_relation(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    max_bytes: i64,
    relation: RelationRow,
) -> Result<
    (
        RelationObservation,
        Vec<ColumnGenerationObservation>,
        Vec<ColumnExpressionObservation>,
        Vec<NotNullConstraintObservation>,
    ),
    SourceObservationFailure,
> {
    let unsupported = bounded(
        request,
        cancellation,
        transaction.query_one(
            "SELECT EXISTS(SELECT 1 FROM pg_catalog.pg_inherits WHERE inhrelid = $1 OR inhparent = $1), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_trigger WHERE tgrelid = $1), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_rewrite WHERE ev_class = $1), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_policy WHERE polrelid = $1), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel WHERE classoid = 'pg_class'::regclass AND objoid = $1), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_depend WHERE classid = 'pg_class'::regclass AND objid = $1 AND deptype = 'e'), \
             (SELECT relchecks FROM pg_catalog.pg_class WHERE oid = $1)",
            &[&relation.oid],
        ),
    )
    .await?;
    meter.add(request, 8)?;
    if (0..6).any(|index| field::<bool>(&unsupported, index) != Ok(false)) {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    let expected_check_count: i16 = field(&unsupported, 6)?;
    if expected_check_count < 0 {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }

    let stream = bounded(
        request,
        cancellation,
        transaction.query_raw(
            "SELECT a.attname::text, a.attnum, \
             CASE WHEN a.attisdropped THEN NULL ELSE pg_catalog.format_type(a.atttypid, a.atttypmod) END, \
             tn.nspname::text, t.typname::text, a.attnotnull, \
             CASE WHEN octet_length(pg_catalog.col_description(a.attrelid, a.attnum)) <= $2::bigint \
               THEN pg_catalog.col_description(a.attrelid, a.attnum) END, \
             a.atthasdef, a.attgenerated::text, a.attidentity::text, a.attisdropped, \
             a.attcollation <> 0, a.attacl IS NOT NULL, a.attoptions IS NOT NULL, \
             COALESCE(octet_length(pg_catalog.col_description(a.attrelid, a.attnum)) > $2::bigint, false), \
             CASE WHEN octet_length(pg_catalog.pg_get_expr(ad.adbin, ad.adrelid)) <= $2::bigint \
               THEN pg_catalog.pg_get_expr(ad.adbin, ad.adrelid) END, \
             COALESCE(octet_length(pg_catalog.pg_get_expr(ad.adbin, ad.adrelid)) > $2::bigint, false), \
             a.atthasmissing \
             FROM pg_catalog.pg_attribute a \
             LEFT JOIN pg_catalog.pg_attrdef ad ON ad.adrelid = a.attrelid AND ad.adnum = a.attnum \
             LEFT JOIN pg_catalog.pg_type t ON t.oid = a.atttypid \
             LEFT JOIN pg_catalog.pg_namespace tn ON tn.oid = t.typnamespace \
             WHERE a.attrelid = $1 AND a.attnum > 0 ORDER BY a.attnum",
            vec![&relation.oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut columns = Vec::new();
    let mut generations = Vec::new();
    let mut expressions = Vec::new();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let name: String = field(&row, 0)?;
        let ordinal: i16 = field(&row, 1)?;
        let data_type: Option<String> = field(&row, 2)?;
        let type_schema: Option<String> = field(&row, 3)?;
        let type_name: Option<String> = field(&row, 4)?;
        let not_null: bool = field(&row, 5)?;
        let comment: Option<String> = field(&row, 6)?;
        let generated: String = field(&row, 8)?;
        let identity: String = field(&row, 9)?;
        let expression: Option<String> = field(&row, 15)?;
        if field::<bool>(&row, 14)? || field::<bool>(&row, 16)? {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        meter.add(
            request,
            20 + name.len()
                + data_type.as_ref().map_or(0, String::len)
                + type_schema.as_ref().map_or(0, String::len)
                + type_name.as_ref().map_or(0, String::len)
                + comment.as_ref().map_or(0, String::len)
                + expression.as_ref().map_or(0, String::len),
        )?;
        if ordinal <= 0
            || !identity.is_empty()
            || field::<bool>(&row, 7)? != expression.is_some()
            || field::<bool>(&row, 17)?
            || [10, 11, 12, 13]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(false))
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let (Some(data_type), Some(type_schema), Some(type_name)) =
            (data_type, type_schema, type_name)
        else {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        };
        let generation = match generated.as_str() {
            "" => ColumnGenerationObservation::not_generated(
                &relation.schema,
                &relation.name,
                RelationKind::Table,
                &name,
            ),
            "s" => ColumnGenerationObservation::stored(
                &relation.schema,
                &relation.name,
                RelationKind::Table,
                &name,
            ),
            "v" => ColumnGenerationObservation::virtual_generated(
                &relation.schema,
                &relation.name,
                RelationKind::Table,
                &name,
            ),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let observed_expression = match (generated.as_str(), expression) {
            ("", None) => ColumnExpressionObservation::no_expression(
                &relation.schema,
                &relation.name,
                RelationKind::Table,
                &name,
            ),
            ("", Some(value)) => ColumnExpressionObservation::default_expression(
                &relation.schema,
                &relation.name,
                RelationKind::Table,
                &name,
                value,
            ),
            ("s" | "v", Some(value)) => ColumnExpressionObservation::generation_expression(
                &relation.schema,
                &relation.name,
                RelationKind::Table,
                &name,
                value,
            ),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        generations.push(generation);
        expressions.push(observed_expression);
        columns.push(
            ColumnObservationV3::new(
                name,
                ordinal as u32,
                data_type,
                QualifiedTypeName::new(type_schema, type_name)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                !not_null,
                comment,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
    }
    let indexes = capture_indexes(
        transaction,
        request,
        cancellation,
        meter,
        max_bytes,
        relation.oid,
    )
    .await?;
    let (constraints, not_null) = capture_constraints(
        transaction,
        request,
        cancellation,
        meter,
        &relation,
        expected_check_count,
        (&columns, &indexes),
    )
    .await?;
    let mut observed =
        RelationObservation::new(relation.schema, relation.name, RelationKind::Table, columns)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
            .with_replica_identity_mode(relation.replica_identity)
            .with_constraints(constraints)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
            .with_indexes(indexes)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    if let Some(comment) = relation.comment {
        observed = observed.with_source_comment(comment);
    }
    Ok((observed, generations, expressions, not_null))
}

async fn capture_constraints(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    relation: &RelationRow,
    expected_count: i16,
    evidence: (&[ColumnObservationV3], &[IndexObservation]),
) -> Result<
    (
        Vec<TableConstraintObservation>,
        Vec<NotNullConstraintObservation>,
    ),
    SourceObservationFailure,
> {
    let (columns, indexes) = evidence;
    let max_bytes = request.request().limits().max_bytes().min(i64::MAX as u64) as i64;
    let stream = bounded(
        request,
        cancellation,
        transaction.query_raw(
            "SELECT c.conname::text, c.contype::text, \
             CASE WHEN octet_length(pg_catalog.pg_get_constraintdef(c.oid, false)) <= $2::bigint \
               THEN pg_catalog.pg_get_constraintdef(c.oid, false) END, \
             COALESCE(octet_length(pg_catalog.pg_get_constraintdef(c.oid, false)) > $2::bigint, false), \
             c.convalidated, c.conenforced, c.connoinherit, c.conislocal, c.coninhcount, \
             c.conparentid = 0, c.condeferrable, c.condeferred, c.conindid = 0, \
             c.confrelid = 0, c.contypid = 0, c.conperiod, c.conbin IS NOT NULL, \
             c.connamespace = t.relnamespace, \
             EXISTS(SELECT 1 FROM pg_catalog.pg_description \
               WHERE classoid = 'pg_constraint'::regclass AND objoid = c.oid), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel \
               WHERE classoid = 'pg_constraint'::regclass AND objoid = c.oid), \
             c.conkey, backing.relname::text, c.conexclop IS NULL, c.confkey IS NULL, \
             c.conpfeqop IS NULL, c.conppeqop IS NULL, c.conffeqop IS NULL, \
             c.confdelsetcols IS NULL, c.confupdtype::text, c.confdeltype::text, \
             c.confmatchtype::text \
             FROM pg_catalog.pg_constraint c \
             JOIN pg_catalog.pg_class t ON t.oid = c.conrelid \
             LEFT JOIN pg_catalog.pg_class backing ON backing.oid = c.conindid \
             WHERE c.conrelid = $1 ORDER BY c.conname, c.oid",
            vec![&relation.oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut constraints = Vec::new();
    let mut not_null = Vec::new();
    let mut check_count = 0;
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let name: String = field(&row, 0)?;
        let kind: String = field(&row, 1)?;
        let definition: Option<String> = field(&row, 2)?;
        let key: Option<Vec<i16>> = field(&row, 20)?;
        let backing_name: Option<String> = field(&row, 21)?;
        if field::<bool>(&row, 3)? {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        meter.add(
            request,
            32 + name.len()
                + kind.len()
                + definition.as_ref().map_or(0, String::len)
                + key.as_ref().map_or(0, |values| values.len() * 2)
                + backing_name.as_ref().map_or(0, String::len),
        )?;
        if field::<i16>(&row, 8)? != 0
            || [9, 13, 14, 17]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(true))
            || [10, 11, 15, 18, 19]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(false))
            || !field::<bool>(&row, 7)?
            || !(22..=27).all(|index| field::<bool>(&row, index) == Ok(true))
            || !(28..=30).all(|index| field::<String>(&row, index) == Ok(" ".to_owned()))
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let constraint = match kind.as_str() {
            "c" if field::<bool>(&row, 12)? && field::<bool>(&row, 16)? => {
                check_count += 1;
                TableConstraintObservation::Check(
                    CheckConstraintObservation::new(
                        name,
                        definition.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?,
                        field(&row, 4)?,
                        field(&row, 5)?,
                        field(&row, 6)?,
                    )
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                )
            }
            "n" if field::<bool>(&row, 12)?
                && !field::<bool>(&row, 16)?
                && field::<bool>(&row, 5)?
                && backing_name.is_none() =>
            {
                let key = key.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                let [position] = key.as_slice() else {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                };
                let column = columns
                    .iter()
                    .find(|column| column.ordinal_position() == *position as u32)
                    .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                not_null.push(
                    NotNullConstraintObservation::new(
                        &relation.schema,
                        &relation.name,
                        RelationKind::Table,
                        name,
                        column.column_name(),
                        field(&row, 4)?,
                        field(&row, 5)?,
                        field(&row, 7)?,
                        field::<i16>(&row, 8)? as u16,
                        field(&row, 6)?,
                    )
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                );
                continue;
            }
            "p" | "u"
                if !field::<bool>(&row, 12)?
                    && !field::<bool>(&row, 16)?
                    && field::<bool>(&row, 4)?
                    && field::<bool>(&row, 5)?
                    && field::<bool>(&row, 6)? =>
            {
                let key = key.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                let names = key
                    .into_iter()
                    .map(|position| {
                        columns
                            .iter()
                            .find(|column| column.ordinal_position() == position as u32)
                            .map(|column| column.column_name().to_owned())
                            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                if backing_name.as_deref() != Some(name.as_str()) {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
                let index = indexes
                    .iter()
                    .find(|index| index.index_name() == name)
                    .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                let is_primary = kind == "p";
                if !index.is_unique()
                    || index
                        .catalog_flags()
                        .is_none_or(|flags| flags.primary() != is_primary)
                    || index.key_attributes().len() != names.len()
                    || index
                        .key_attributes()
                        .iter()
                        .zip(&names)
                        .any(|(attribute, column)| {
                            attribute.attribute_name() != Some(column.as_str())
                        })
                    || index.predicate().is_some()
                    || index.ready() != Some(true)
                    || index.valid() != Some(true)
                    || index.live() != Some(true)
                    || (is_primary && index.nulls_not_distinct() == Some(true))
                {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
                if is_primary {
                    TableConstraintObservation::PrimaryKey(
                        PrimaryKeyObservation::new(name, names)
                            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                    )
                } else {
                    TableConstraintObservation::Unique(
                        UniqueConstraintObservation::new(name, names)
                            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
                            .with_nulls_not_distinct(
                                index
                                    .nulls_not_distinct()
                                    .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?,
                            ),
                    )
                }
            }
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        };
        constraints.push(constraint);
    }
    if check_count != expected_count as usize {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    Ok((constraints, not_null))
}

async fn capture_indexes(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    max_bytes: i64,
    relation_oid: u32,
) -> Result<Vec<IndexObservation>, SourceObservationFailure> {
    let stream = bounded(
        request,
        cancellation,
        transaction.query_raw(
            "SELECT c.oid, c.relname::text, c.relkind::text, c.relpersistence::text, \
             c.relispartition, c.reloptions IS NOT NULL, c.relacl IS NOT NULL, \
             am.amname::text, i.indnatts, i.indnkeyatts, i.indisunique, i.indnullsnotdistinct, \
             i.indisprimary, i.indisexclusion, i.indimmediate, i.indisclustered, \
             i.indcheckxmin, i.indisready, i.indisvalid, i.indislive, i.indisreplident, \
             i.indexprs IS NOT NULL, i.indpred IS NOT NULL, ts.spcname::text, \
             c.reltablespace = 0, \
             CASE WHEN octet_length(pg_catalog.pg_get_indexdef(c.oid)) <= $2::bigint \
               THEN pg_catalog.pg_get_indexdef(c.oid) END, \
             CASE WHEN octet_length(pg_catalog.obj_description(c.oid, 'pg_class')) <= $2::bigint \
               THEN pg_catalog.obj_description(c.oid, 'pg_class') END, \
             COALESCE(octet_length(pg_catalog.pg_get_indexdef(c.oid)) > $2::bigint, false) \
               OR COALESCE(octet_length(pg_catalog.obj_description(c.oid, 'pg_class')) > $2::bigint, false), \
             c.relnamespace = t.relnamespace, \
             EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel WHERE classoid = 'pg_class'::regclass AND objoid = c.oid), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_depend WHERE classid = 'pg_class'::regclass AND objid = c.oid AND deptype = 'e'), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_description WHERE classoid = 'pg_class'::regclass AND objoid = c.oid AND objsubid > 0), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_attribute WHERE attrelid = c.oid AND attnum > 0 \
               AND (attacl IS NOT NULL OR attfdwoptions IS NOT NULL OR attisdropped \
                 OR attstattarget <> -1)), \
             CASE WHEN octet_length(pg_catalog.pg_get_expr(i.indpred, i.indrelid, false)) <= $2::bigint \
               THEN pg_catalog.pg_get_expr(i.indpred, i.indrelid, false) END, \
             COALESCE(octet_length(pg_catalog.pg_get_expr(i.indpred, i.indrelid, false)) > $2::bigint, false), \
             CASE WHEN COALESCE(octet_length(pg_catalog.array_to_string(c.reloptions, ',', '<NULL>')), 0) <= $2::bigint \
               THEN c.reloptions END, \
             COALESCE(octet_length(pg_catalog.array_to_string(c.reloptions, ',', '<NULL>')) > $2::bigint, false) \
             FROM pg_catalog.pg_index i \
             JOIN pg_catalog.pg_class c ON c.oid = i.indexrelid \
             JOIN pg_catalog.pg_class t ON t.oid = i.indrelid \
             LEFT JOIN pg_catalog.pg_am am ON am.oid = c.relam \
             JOIN pg_catalog.pg_database db ON db.datname = pg_catalog.current_database() \
             LEFT JOIN pg_catalog.pg_tablespace ts \
               ON ts.oid = CASE WHEN c.reltablespace = 0 THEN db.dattablespace ELSE c.reltablespace END \
             WHERE i.indrelid = $1 ORDER BY c.relname",
            vec![&relation_oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut indexes = Vec::new();
    while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
        let index_oid: u32 = field(&row, 0)?;
        let name: String = field(&row, 1)?;
        let kind: String = field(&row, 2)?;
        let persistence: String = field(&row, 3)?;
        let access_method: Option<String> = field(&row, 7)?;
        let total: i16 = field(&row, 8)?;
        let key_count: i16 = field(&row, 9)?;
        let tablespace: Option<String> = field(&row, 23)?;
        let definition: Option<String> = field(&row, 25)?;
        let comment: Option<String> = field(&row, 26)?;
        let predicate: Option<String> = field(&row, 33)?;
        let storage_options: Option<Vec<String>> = field(&row, 35)?;
        if field::<bool>(&row, 27)? || field::<bool>(&row, 34)? || field::<bool>(&row, 36)? {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        meter.add(
            request,
            40 + name.len()
                + kind.len()
                + persistence.len()
                + access_method.as_ref().map_or(0, String::len)
                + tablespace.as_ref().map_or(0, String::len)
                + definition.as_ref().map_or(0, String::len)
                + comment.as_ref().map_or(0, String::len)
                + predicate.as_ref().map_or(0, String::len),
        )?;
        if kind != "i"
            || persistence != "p"
            || access_method.is_none()
            || total <= 0
            || key_count <= 0
            || key_count > total
            || definition.is_none()
            || tablespace.is_none()
            || field::<bool>(&row, 5)? != storage_options.is_some()
            || field::<bool>(&row, 22)? != predicate.is_some()
            || [4, 6, 29, 30, 31, 32]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(false))
            || !field::<bool>(&row, 28)?
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }

        let attributes = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT s.position, i.indkey[s.position], a.attname::text, \
                 ocn.nspname::text, oc.opcname::text, cn.nspname::text, coll.collname::text, \
                 i.indoption[s.position], \
                 CASE WHEN i.indkey[s.position] = 0 \
                   AND octet_length(pg_catalog.pg_get_indexdef(i.indexrelid, s.position + 1, false)) <= $2::bigint \
                   THEN pg_catalog.pg_get_indexdef(i.indexrelid, s.position + 1, false) END, \
                 COALESCE(i.indkey[s.position] = 0 \
                   AND octet_length(pg_catalog.pg_get_indexdef(i.indexrelid, s.position + 1, false)) > $2::bigint, false), \
                 CASE WHEN COALESCE(octet_length(pg_catalog.array_to_string(ia.attoptions, ',', '<NULL>')), 0) <= $2::bigint \
                   THEN ia.attoptions END, \
                 COALESCE(octet_length(pg_catalog.array_to_string(ia.attoptions, ',', '<NULL>')) > $2::bigint, false), \
                 ia.attnum \
                 FROM pg_catalog.pg_index i \
                 CROSS JOIN LATERAL pg_catalog.generate_series(0, i.indnatts - 1) s(position) \
                 LEFT JOIN pg_catalog.pg_attribute a ON a.attrelid = i.indrelid \
                   AND a.attnum = i.indkey[s.position] AND NOT a.attisdropped \
                 LEFT JOIN pg_catalog.pg_attribute ia ON ia.attrelid = i.indexrelid \
                   AND ia.attnum = s.position + 1 \
                 LEFT JOIN pg_catalog.pg_opclass oc ON oc.oid = i.indclass[s.position] \
                 LEFT JOIN pg_catalog.pg_namespace ocn ON ocn.oid = oc.opcnamespace \
                 LEFT JOIN pg_catalog.pg_collation coll ON coll.oid = i.indcollation[s.position] \
                 LEFT JOIN pg_catalog.pg_namespace cn ON cn.oid = coll.collnamespace \
                 WHERE i.indexrelid = $1 ORDER BY s.position",
                vec![&index_oid as &(dyn ToSql + Sync), &max_bytes],
            ),
        )
        .await?;
        tokio::pin!(attributes);
        let mut keys = Vec::new();
        let mut includes = Vec::new();
        let mut semantics = Vec::new();
        let mut expression_count = 0;
        while let Some(attribute) = bounded(request, cancellation, attributes.try_next()).await? {
            let position: i32 = field(&attribute, 0)?;
            let attnum: i16 = field(&attribute, 1)?;
            let column: Option<String> = field(&attribute, 2)?;
            let opclass_schema: Option<String> = field(&attribute, 3)?;
            let opclass_name: Option<String> = field(&attribute, 4)?;
            let collation_schema: Option<String> = field(&attribute, 5)?;
            let collation_name: Option<String> = field(&attribute, 6)?;
            let options: Option<i16> = field(&attribute, 7)?;
            let expression: Option<String> = field(&attribute, 8)?;
            let operator_options: Option<Vec<String>> = field(&attribute, 10)?;
            let index_attnum: Option<i16> = field(&attribute, 12)?;
            if field::<bool>(&attribute, 9)? || field::<bool>(&attribute, 11)? {
                return Err(SourceObservationFailure::ByteLimitExceeded {
                    max_bytes: request.request().limits().max_bytes(),
                });
            }
            meter.add(
                request,
                12 + column.as_ref().map_or(0, String::len)
                    + opclass_schema.as_ref().map_or(0, String::len)
                    + opclass_name.as_ref().map_or(0, String::len)
                    + collation_schema.as_ref().map_or(0, String::len)
                    + collation_name.as_ref().map_or(0, String::len)
                    + expression.as_ref().map_or(0, String::len),
            )?;
            if position < 0 || position as usize != keys.len() + includes.len() {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            let position = position as u32 + 1;
            if index_attnum != Some(position as i16) {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            if position > key_count as u32 {
                if attnum <= 0
                    || expression.is_some()
                    || opclass_schema.is_some()
                    || opclass_name.is_some()
                    || collation_schema.is_some()
                    || collation_name.is_some()
                    || options.is_some()
                    || operator_options.is_some()
                {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
                includes.push(
                    IndexAttributeObservation::column(
                        position,
                        IndexAttributeKind::Include,
                        column.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?,
                    )
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                );
                continue;
            }
            let (Some(opclass_schema), Some(opclass_name), Some(options)) =
                (opclass_schema, opclass_name, options)
            else {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            };
            let collation = match (collation_schema, collation_name) {
                (Some(schema), Some(name)) => Some(
                    QualifiedCollationName::new(schema, name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                ),
                (None, None) => None,
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            let key = match (attnum, column, expression) {
                (0, None, Some(value)) => {
                    expression_count += 1;
                    IndexAttributeObservation::expression(position, IndexAttributeKind::Key, value)
                }
                (number, Some(value), None) if number > 0 => {
                    IndexAttributeObservation::column(position, IndexAttributeKind::Key, value)
                }
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            keys.push(key.map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?);
            let class_options = capture_options(operator_options, request, meter)?
                .into_iter()
                .map(|(name, value)| OperatorClassOption::new(name, value))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
            semantics.push(
                IndexKeySemantics::new(
                    position,
                    collation,
                    QualifiedOperatorClassName::new(opclass_schema, opclass_name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                    options as u16,
                )
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
                .with_operator_class_options(class_options)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            );
        }
        if keys.len() != key_count as usize
            || keys.len() + includes.len() != total as usize
            || field::<bool>(&row, 21)? != (expression_count > 0)
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let flags = IndexCatalogFlags::new(
            field(&row, 12)?,
            field(&row, 13)?,
            field(&row, 14)?,
            field(&row, 15)?,
            field(&row, 16)?,
            field(&row, 20)?,
        );
        let tablespace = if field::<bool>(&row, 24)? {
            IndexTablespace::database_default(tablespace.unwrap())
        } else {
            IndexTablespace::named(tablespace.unwrap())
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let storage_options = capture_options(storage_options, request, meter)?
            .into_iter()
            .map(|(name, value)| IndexStorageOption::new(name, value))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let mut index = IndexObservation::new(
            name,
            field(&row, 10)?,
            Some(field(&row, 11)?),
            keys,
            includes,
        )
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
        .with_access_method(access_method.unwrap())
        .with_key_semantics(semantics)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
        .with_catalog_flags(flags)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
        .with_storage_options(storage_options)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
        .with_tablespace(tablespace)
        .with_ready(field(&row, 17)?)
        .with_valid(field(&row, 18)?)
        .with_live(field(&row, 19)?)
        .with_index_definition(definition.unwrap());
        if let Some(predicate) = predicate {
            index = index.with_predicate(predicate);
        }
        if let Some(comment) = comment {
            index = index.with_source_comment(comment);
        }
        indexes.push(index);
    }
    Ok(indexes)
}

fn capture_options(
    options: Option<Vec<String>>,
    request: &AuthorizedObservationRequest,
    meter: &mut CaptureMeter,
) -> Result<Vec<(String, String)>, SourceObservationFailure> {
    options
        .unwrap_or_default()
        .into_iter()
        .map(|entry| {
            meter.add(request, entry.len())?;
            let (name, value) = entry
                .split_once('=')
                .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
            if name.trim().is_empty() {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            Ok((name.to_owned(), value.to_owned()))
        })
        .collect()
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
