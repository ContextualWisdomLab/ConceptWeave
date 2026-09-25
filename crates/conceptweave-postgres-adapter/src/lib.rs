//! PostgreSQL 18 catalog observation behind the Source Observation capability.
//!
//! This bounded path admits schema-scoped domains, enums, ordinary tables, local CHECK,
//! NOT NULL, ordinary key and foreign-key constraints, and bounded indexes.
//! Unsupported relation metadata fails closed until it can be mapped without losing facts.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod collations;
mod foreign_keys;
mod ranges;

use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    time::Duration,
};

use conceptweave_observation::{
    ArrayTypeObservation, CheckConstraintObservation, ColumnArrayDimensionsObservation,
    ColumnCollationObservation, ColumnExpressionObservation, ColumnGenerationObservation,
    ColumnIdentityObservation, ColumnObservationV3, ConstraintDeferrability,
    ConstraintPeriodObservation, ConstraintTimingObservation, DomainCheckConstraintObservation,
    DomainObservation, EnumObservation, IdentitySequenceObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics, IndexObservation,
    IndexStorageOption, IndexTablespace, NotNullConstraintObservation, OperatorClassOption,
    PostgresSchemaSnapshotV3, PostgresTypeKind, PrimaryKeyObservation, QualifiedCollationName,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
    RelationOwnerObservation, RelationTablespaceObservation, ReplicaIdentityMode,
    TableConstraintObservation, TypeKindObservation, TypeOwnerObservation,
    UniqueConstraintObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
    SourceObservationPort,
};
use futures_util::TryStreamExt;
use rustls::{ClientConfig, RootCertStore, pki_types::CertificateDer};
use tokio_postgres::{
    Config, IsolationLevel, NoTls, Row, Transaction,
    config::{Host, SslMode},
    error::SqlState,
    types::ToSql,
};
use tokio_postgres_rustls::MakeRustlsConnect;

/// Unix-socket PostgreSQL adapter with credential-bearing configurations keyed by the exact
/// registry source key and immutable policy binding. TCP is rejected by this transport;
/// configurations and their credentials never enter domain values.
pub struct PostgresUnixAdapter {
    connections: BTreeMap<String, (String, Config)>,
}

/// TCP PostgreSQL adapter that verifies the server certificate chain and configured host name
/// against one adapter-local DER trust anchor per exact source binding.
pub struct PostgresTlsAdapter {
    connections: BTreeMap<String, (String, Config, MakeRustlsConnect)>,
}

impl PostgresTlsAdapter {
    /// Installs TCP configurations with explicit DER trust anchors. Invalid anchors and non-TCP
    /// hosts are rejected before any source I/O; TLS is required even if a config requested less.
    pub fn new(
        connections: BTreeMap<String, (String, Config, Vec<u8>)>,
    ) -> Result<Self, SourceObservationFailure> {
        let connections = connections
            .into_iter()
            .map(|(key, (binding, mut config, ca_der))| {
                if config.get_hosts().is_empty()
                    || config
                        .get_hosts()
                        .iter()
                        .any(|host| !matches!(host, Host::Tcp(_)))
                {
                    return Err(SourceObservationFailure::SourceUnavailable);
                }
                let mut roots = RootCertStore::empty();
                roots
                    .add(CertificateDer::from(ca_der))
                    .map_err(|_| SourceObservationFailure::SourceUnavailable)?;
                config.ssl_mode(SslMode::Require);
                let tls = MakeRustlsConnect::new(
                    ClientConfig::builder()
                        .with_root_certificates(roots)
                        .with_no_client_auth(),
                );
                Ok((key, (binding, config, tls)))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { connections })
    }
}

impl SourceObservationPort for PostgresTlsAdapter {
    type Snapshot = PostgresSchemaSnapshotV3;

    async fn observe<'a>(
        &'a self,
        request: AuthorizedObservationRequest,
        cancellation: &'a dyn ObservationCancellation,
    ) -> Result<Self::Snapshot, SourceObservationFailure> {
        let (active_binding, config, tls) = self
            .connections
            .get(request.source_connection().source_connection_key())
            .ok_or(SourceObservationFailure::SourceUnavailable)?;
        if active_binding != request.source_connection().connection_policy_binding() {
            return Err(SourceObservationFailure::SourceUnavailable);
        }
        let (mut client, connection) =
            bounded(&request, cancellation, config.connect(tls.clone())).await?;
        let connection_task = tokio::spawn(connection);
        let result = capture_catalog(&mut client, &request, cancellation).await;
        connection_task.abort();
        result
    }
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
    kind: RelationKind,
    replica_identity: ReplicaIdentityMode,
    tablespace: Option<IndexTablespace>,
    owner_oid: u32,
    owner_name: String,
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
    let mut type_kinds = Vec::new();
    let mut type_owners = Vec::new();
    let mut array_types = Vec::new();
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
                 COALESCE(octet_length(pg_catalog.obj_description(c.oid, 'pg_class')) > $2::bigint, false), \
                 EXISTS(SELECT 1 FROM pg_catalog.pg_depend d \
                   JOIN pg_catalog.pg_class owner_table ON owner_table.oid = d.refobjid \
                   JOIN pg_catalog.pg_attribute a ON a.attrelid = d.refobjid \
                     AND a.attnum = d.refobjsubid AND a.attidentity <> '' \
                   WHERE d.classid = 'pg_class'::regclass AND d.objid = c.oid \
                     AND d.objsubid = 0 AND d.refclassid = 'pg_class'::regclass \
                     AND d.deptype = 'i' AND owner_table.relnamespace = c.relnamespace), \
                 EXISTS(SELECT 1 FROM pg_catalog.pg_am am WHERE am.oid = c.relam \
                   AND am.amname = 'heap' AND am.amtype = 't' \
                   AND am.amhandler = 'pg_catalog.heap_tableam_handler'::regproc), \
                 EXISTS(SELECT 1 FROM pg_catalog.pg_type t WHERE t.oid = c.reltype \
                   AND t.typtype = 'c' AND t.typisdefined AND t.typrelid = c.oid \
                   AND t.typnamespace = c.relnamespace AND t.typname = c.relname \
                   AND t.typowner = c.relowner AND t.typacl IS NULL \
                   AND NOT EXISTS(SELECT 1 FROM pg_catalog.pg_description d \
                     WHERE d.classoid = 'pg_type'::regclass AND d.objoid = t.oid) \
                   AND NOT EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel l \
                     WHERE l.classoid = 'pg_type'::regclass AND l.objoid = t.oid) \
                   AND NOT EXISTS(SELECT 1 FROM pg_catalog.pg_depend d \
                     WHERE d.classid = 'pg_type'::regclass AND d.objid = t.oid \
                       AND d.deptype = 'e')), \
                 c.relam = 0 AND c.reltablespace = 0 AND c.reltoastrelid = 0, \
                 ts.spcname::text, c.reltablespace = 0, c.relowner, owner_role.rolname::text \
                 FROM pg_catalog.pg_class c \
                 JOIN pg_catalog.pg_database db ON db.datname = pg_catalog.current_database() \
                 LEFT JOIN pg_catalog.pg_tablespace ts \
                   ON ts.oid = CASE WHEN c.reltablespace = 0 THEN db.dattablespace ELSE c.reltablespace END \
                 LEFT JOIN pg_catalog.pg_roles owner_role ON owner_role.oid = c.relowner \
                 WHERE c.relnamespace = $1 \
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
            let tablespace_name: Option<String> = field(&row, 19)?;
            let owner_oid: u32 = field(&row, 21)?;
            let owner_name: String = field::<Option<String>>(&row, 22)?
                .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
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
                    + tablespace_name.as_ref().map_or(0, String::len)
                    + owner_name.len()
                    + comment.as_ref().map_or(0, String::len),
            )?;
            if kind == "S" && field::<bool>(&row, 15)? {
                continue;
            }
            // relhasrules and relhastriggers are lazy hints; the catalog-row guards below
            // decide whether any unsupported rule or trigger still exists.
            // Row-type pg_type metadata is separate from pg_class for both tables and
            // standalone composites; unmodeled changes must not retain the same digest.
            let relation_kind = match kind.as_str() {
                "r" => RelationKind::Table,
                "c" => RelationKind::CompositeType,
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            if persistence != "p"
                || [4, 6, 7, 10, 11, 12]
                    .into_iter()
                    .any(|index| field::<bool>(&row, index) != Ok(false))
                || match relation_kind {
                    RelationKind::Table => !field::<bool>(&row, 16)? || !field::<bool>(&row, 17)?,
                    RelationKind::CompositeType => {
                        field::<bool>(&row, 16)?
                            || !field::<bool>(&row, 17)?
                            || !field::<bool>(&row, 18)?
                            || replica != "n"
                    }
                    _ => unreachable!(),
                }
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
            let tablespace = if relation_kind == RelationKind::Table {
                let name =
                    tablespace_name.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
                Some(
                    if field::<bool>(&row, 20)? {
                        IndexTablespace::database_default(name)
                    } else {
                        IndexTablespace::named(name)
                    }
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                )
            } else {
                None
            };
            relation_rows.push(RelationRow {
                schema: schema.clone(),
                oid,
                name,
                kind: relation_kind,
                replica_identity,
                tablespace,
                owner_oid,
                owner_name,
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

        let kind_stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT t.typname::text, t.typtype::text, t.typisdefined, \
                 bn.nspname::text, bt.typname::text, cn.nspname::text, ct.typname::text, \
                 t.typowner, owner.rolname::text, \
                 t.typacl IS NOT NULL OR \
                   EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel l \
                     WHERE l.classoid = 'pg_type'::regclass AND l.objoid = t.oid) OR \
                   EXISTS(SELECT 1 FROM pg_catalog.pg_depend d \
                     WHERE d.classid = 'pg_type'::regclass AND d.objid = t.oid \
                       AND d.deptype = 'e') OR \
                   (t.typtype NOT IN ('d','e') AND \
                     EXISTS(SELECT 1 FROM pg_catalog.pg_description d \
                       WHERE d.classoid = 'pg_type'::regclass AND d.objoid = t.oid)) \
                 FROM pg_catalog.pg_type t \
                 JOIN pg_catalog.pg_roles owner ON owner.oid = t.typowner \
                 LEFT JOIN pg_catalog.pg_type bt ON bt.oid = t.typbasetype \
                 LEFT JOIN pg_catalog.pg_namespace bn ON bn.oid = bt.typnamespace \
                 LEFT JOIN pg_catalog.pg_range r ON r.rngtypid = t.oid \
                 LEFT JOIN pg_catalog.pg_range mr ON mr.rngmultitypid = t.oid \
                 LEFT JOIN pg_catalog.pg_type ct ON ct.oid = COALESCE(r.rngmultitypid, mr.rngtypid) \
                 LEFT JOIN pg_catalog.pg_namespace cn ON cn.oid = ct.typnamespace \
                 WHERE t.typnamespace = $1 ORDER BY t.typname",
                &[&schema_oid],
            ),
        )
        .await?;
        tokio::pin!(kind_stream);
        while let Some(row) = bounded(request, cancellation, kind_stream.try_next()).await? {
            let name: String = field(&row, 0)?;
            let kind: String = field(&row, 1)?;
            let defined: bool = field(&row, 2)?;
            let base_schema: Option<String> = field(&row, 3)?;
            let base_name: Option<String> = field(&row, 4)?;
            let counterpart_schema: Option<String> = field(&row, 5)?;
            let counterpart_name: Option<String> = field(&row, 6)?;
            let owner_oid: u32 = field(&row, 7)?;
            let owner_name: String = field(&row, 8)?;
            let unmodeled_metadata: bool = field(&row, 9)?;
            meter.add(
                request,
                8 + name.len()
                    + kind.len()
                    + base_schema.as_ref().map_or(0, String::len)
                    + base_name.as_ref().map_or(0, String::len)
                    + counterpart_schema.as_ref().map_or(0, String::len)
                    + counterpart_name.as_ref().map_or(0, String::len)
                    + owner_name.len(),
            )?;
            if !defined || unmodeled_metadata {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            let coordinate = QualifiedTypeName::new(schema, name)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
            type_owners.push(
                TypeOwnerObservation::new(coordinate.clone(), owner_oid, owner_name)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            );
            let base = match (base_schema, base_name) {
                (Some(schema), Some(name)) => Some(
                    QualifiedTypeName::new(schema, name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                ),
                (None, None) => None,
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            let counterpart = match (counterpart_schema, counterpart_name) {
                (Some(schema), Some(name)) => Some(
                    QualifiedTypeName::new(schema, name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                ),
                (None, None) => None,
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            };
            let observation = match (kind.as_str(), base, counterpart) {
                ("b", None, None) => TypeKindObservation::plain(coordinate, PostgresTypeKind::Base),
                ("c", None, None) => {
                    TypeKindObservation::plain(coordinate, PostgresTypeKind::Composite)
                }
                ("e", None, None) => TypeKindObservation::plain(coordinate, PostgresTypeKind::Enum),
                ("p", None, None) => {
                    TypeKindObservation::plain(coordinate, PostgresTypeKind::Pseudo)
                }
                ("d", Some(base), None) => Ok(TypeKindObservation::domain(coordinate, base)),
                ("r", None, Some(counterpart)) => {
                    Ok(TypeKindObservation::range(coordinate, counterpart))
                }
                ("m", None, Some(counterpart)) => {
                    Ok(TypeKindObservation::multirange(coordinate, counterpart))
                }
                _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
            }
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
            type_kinds.push(observation);
        }

        let array_stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT e.typname::text, a.typname::text, a.typnamespace = e.typnamespace, \
                 a.typelem = e.oid FROM pg_catalog.pg_type e \
                 LEFT JOIN pg_catalog.pg_type a ON a.oid = e.typarray \
                 WHERE e.typnamespace = $1 AND e.typarray <> 0 ORDER BY e.typname",
                &[&schema_oid],
            ),
        )
        .await?;
        tokio::pin!(array_stream);
        while let Some(row) = bounded(request, cancellation, array_stream.try_next()).await? {
            let element_name: String = field(&row, 0)?;
            let array_name: Option<String> = field(&row, 1)?;
            let same_schema: Option<bool> = field(&row, 2)?;
            let reciprocal: Option<bool> = field(&row, 3)?;
            let array_name = array_name.ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
            meter.add(request, 16 + element_name.len() + array_name.len())?;
            if same_schema != Some(true) || reciprocal != Some(true) {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            array_types.push(
                ArrayTypeObservation::new(
                    QualifiedTypeName::new(schema, array_name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                    QualifiedTypeName::new(schema, element_name)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                )
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            );
        }
    }

    // Base-type input/output and storage behavior is not represented by the bounded snapshot.
    // The only admitted schema-local base types are generated true arrays with reciprocal pairs.
    let true_array_names = array_types
        .iter()
        .map(|array| {
            (
                array.array_type().schema_name(),
                array.array_type().type_name(),
            )
        })
        .collect::<BTreeSet<_>>();
    if type_kinds.iter().any(|item| {
        item.kind() == PostgresTypeKind::Base
            && !true_array_names
                .contains(&(item.type_name().schema_name(), item.type_name().type_name()))
    }) {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
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
                         CASE WHEN octet_length(pg_catalog.pg_get_constraintdef(c.oid, false)) <= $2::bigint \
                           THEN pg_catalog.pg_get_constraintdef(c.oid, false) END, \
                         convalidated, conenforced, \
                         octet_length(pg_catalog.pg_get_constraintdef(c.oid, false)) > $2::bigint, \
                         c.connamespace <> t.typnamespace OR c.conrelid <> 0 OR \
                           c.conparentid <> 0 OR NOT c.conislocal OR c.coninhcount <> 0 OR \
                           c.connoinherit OR c.condeferrable OR c.condeferred OR \
                           c.conindid <> 0 OR c.confrelid <> 0 OR c.conkey IS NOT NULL OR \
                           c.conbin IS NULL OR \
                           EXISTS(SELECT 1 FROM pg_catalog.pg_description d \
                             WHERE d.classoid = 'pg_constraint'::regclass AND d.objoid = c.oid) OR \
                           EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel l \
                             WHERE l.classoid = 'pg_constraint'::regclass AND l.objoid = c.oid) OR \
                           EXISTS(SELECT 1 FROM pg_catalog.pg_depend d \
                             WHERE d.classid = 'pg_constraint'::regclass AND d.objid = c.oid \
                               AND d.deptype = 'e') \
                         FROM pg_catalog.pg_constraint c \
                         JOIN pg_catalog.pg_type t ON t.oid = c.contypid \
                         WHERE c.contypid = $1 ORDER BY c.conname, c.oid",
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
                    if kind != "c" || field::<bool>(&row, 6)? {
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
    let mut relation_tablespaces = Vec::new();
    let mut relation_owners = Vec::new();
    let mut relation_oids = Vec::new();
    let mut key_timings = Vec::new();
    let mut column_collations = Vec::new();
    let mut column_array_dimensions = Vec::new();
    let mut column_generations = Vec::new();
    let mut column_expressions = Vec::new();
    let mut column_identities = Vec::new();
    let mut not_null_constraints = Vec::new();
    let mut constraint_timings = Vec::new();
    for relation in relation_rows {
        relation_owners.push(
            RelationOwnerObservation::new(
                relation.schema.clone(),
                relation.name.clone(),
                relation.owner_oid,
                relation.owner_name.clone(),
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
        if let Some(tablespace) = &relation.tablespace {
            relation_tablespaces.push(
                RelationTablespaceObservation::new(
                    relation.schema.clone(),
                    relation.name.clone(),
                    tablespace.clone(),
                )
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            );
        }
        relation_oids.push(relation.oid);
        let (
            observed,
            dimensions,
            collations,
            generations,
            expressions,
            identities,
            not_null,
            timings,
        ) = capture_relation(
            &transaction,
            request,
            cancellation,
            &mut meter,
            max_bytes,
            relation,
        )
        .await?;
        relations.push(observed);
        column_array_dimensions.extend(dimensions);
        column_collations.extend(collations);
        column_generations.extend(generations);
        column_expressions.extend(expressions);
        column_identities.extend(identities);
        not_null_constraints.extend(not_null);
        key_timings.push(timings.clone());
        constraint_timings.extend(timings);
    }
    let builtin_type_names = domains
        .iter()
        .map(DomainObservation::base_type)
        .chain(relations.iter().flat_map(|relation| {
            relation
                .columns()
                .iter()
                .map(ColumnObservationV3::type_binding)
        }))
        .filter(|binding| binding.schema_name() == "pg_catalog")
        .map(|binding| binding.type_name().to_owned())
        .collect::<BTreeSet<_>>();
    if !builtin_type_names.is_empty() {
        let names = builtin_type_names.iter().cloned().collect::<Vec<_>>();
        let stream = bounded(
            request,
            cancellation,
            transaction.query_raw(
                "SELECT a.typname::text, a.typtype::text, a.typisdefined, \
                 a.typsubscript = 'pg_catalog.array_subscript_handler'::regproc, \
                 a.typinput = 'pg_catalog.array_in'::regproc, \
                 en.nspname::text, e.typname::text, e.typarray = a.oid \
                 FROM pg_catalog.pg_type a \
                 LEFT JOIN pg_catalog.pg_type e ON e.oid = a.typelem \
                 LEFT JOIN pg_catalog.pg_namespace en ON en.oid = e.typnamespace \
                 WHERE a.typnamespace = 'pg_catalog'::regnamespace \
                   AND a.typname::text = ANY($1::text[]) ORDER BY a.typname",
                &[&names],
            ),
        )
        .await?;
        tokio::pin!(stream);
        let mut seen = BTreeSet::new();
        while let Some(row) = bounded(request, cancellation, stream.try_next()).await? {
            let name: String = field(&row, 0)?;
            let kind: String = field(&row, 1)?;
            let defined: bool = field(&row, 2)?;
            let array_handler: bool = field(&row, 3)?;
            let array_input: bool = field(&row, 4)?;
            let element_schema: Option<String> = field(&row, 5)?;
            let element_name: Option<String> = field(&row, 6)?;
            let reciprocal: Option<bool> = field(&row, 7)?;
            meter.add(
                request,
                8 + name.len() + element_name.as_ref().map_or(0, String::len),
            )?;
            let true_array = reciprocal == Some(true);
            if !seen.insert(name.clone())
                || !defined
                || (kind == "b" && array_input && !true_array)
                || (true_array && !array_handler)
            {
                return Err(SourceObservationFailure::InvalidCapturedMetadata);
            }
            if true_array {
                let (Some(element_schema), Some(element_name)) = (element_schema, element_name)
                else {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                };
                if kind != "b" || element_schema != "pg_catalog" {
                    return Err(SourceObservationFailure::InvalidCapturedMetadata);
                }
                array_types.push(
                    ArrayTypeObservation::new(
                        QualifiedTypeName::new("pg_catalog", name)
                            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                        QualifiedTypeName::new(element_schema, element_name)
                            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                    )
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                );
            }
        }
        if seen != builtin_type_names {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
    }
    let (foreign_key_catalog, foreign_key_triggers) = foreign_keys::capture(
        &transaction,
        request,
        cancellation,
        &mut meter,
        &relation_oids,
        &mut relations,
    )
    .await?;
    foreign_keys::validate_triggers(
        &transaction,
        request,
        cancellation,
        &mut meter,
        &relation_oids,
        &key_timings,
        &foreign_key_triggers,
    )
    .await?;
    let constraint_periods = relations
        .iter()
        .flat_map(|relation| {
            relation
                .constraints()
                .iter()
                .filter(|constraint| {
                    matches!(
                        constraint,
                        TableConstraintObservation::PrimaryKey(_)
                            | TableConstraintObservation::Unique(_)
                            | TableConstraintObservation::ForeignKey(_)
                    )
                })
                .map(|constraint| {
                    ConstraintPeriodObservation::new(
                        relation.schema_name(),
                        relation.relation_name(),
                        relation.kind(),
                        constraint.constraint_name(),
                        false,
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    let range_catalog = ranges::capture(&transaction, request, cancellation, &mut meter).await?;
    let collation_references =
        collations::references(&domains, &relations, &column_collations, &range_catalog);
    let collation_definitions = collations::capture(
        &transaction,
        request,
        cancellation,
        &mut meter,
        collation_references,
    )
    .await?;
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
    let has_relations = !relations.is_empty();
    let snapshot = PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
        request,
        "postgres18_identity_sequence_adapter_v1",
        observed_at_utc,
        relations,
        domains,
        enums,
        array_types,
        type_kinds,
    )
    .and_then(|snapshot| {
        if relation_tablespaces.is_empty() {
            Ok(snapshot)
        } else {
            snapshot.with_observed_relation_tablespaces(relation_tablespaces)
        }
    })
    .and_then(|snapshot| {
        if relation_owners.is_empty() {
            Ok(snapshot)
        } else {
            snapshot.with_observed_relation_owners(relation_owners)
        }
    })
    .and_then(|snapshot| snapshot.with_observed_type_owners(type_owners))
    .and_then(|snapshot| snapshot.with_observed_range_catalog(range_catalog))
    .and_then(|snapshot| {
        if has_relations {
            snapshot
                .with_observed_column_collations(column_collations)
                .and_then(|snapshot| snapshot.with_observed_column_generations(column_generations))
                .and_then(|snapshot| snapshot.with_observed_column_expressions(column_expressions))
                .and_then(|snapshot| snapshot.with_observed_column_identities(column_identities))
        } else {
            Ok(snapshot)
        }
    })
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
    .with_observed_not_null_constraints(not_null_constraints)
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    let snapshot = if has_relations {
        snapshot
            .with_observed_constraint_timings(constraint_timings)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
            .with_observed_constraint_periods(constraint_periods)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
            .with_observed_foreign_key_catalog(foreign_key_catalog)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
    } else {
        snapshot
    };
    let snapshot = snapshot
        .with_observed_collation_definitions(collation_definitions)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    let snapshot = if has_relations {
        snapshot
            .with_observed_column_array_dimensions(column_array_dimensions)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
    } else {
        snapshot
    };
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
        Vec<ColumnArrayDimensionsObservation>,
        Vec<ColumnCollationObservation>,
        Vec<ColumnGenerationObservation>,
        Vec<ColumnExpressionObservation>,
        Vec<ColumnIdentityObservation>,
        Vec<NotNullConstraintObservation>,
        Vec<ConstraintTimingObservation>,
    ),
    SourceObservationFailure,
> {
    let unsupported = bounded(
        request,
        cancellation,
        transaction.query_one(
            "SELECT EXISTS(SELECT 1 FROM pg_catalog.pg_inherits WHERE inhrelid = $1 OR inhparent = $1), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_trigger WHERE tgrelid = $1 AND NOT tgisinternal), \
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
             a.atthasmissing, cn.nspname::text, co.collname::text, co.collisdeterministic, \
             a.attstorage IS DISTINCT FROM t.typstorage \
               OR a.attcompression::text <> '' OR a.attstattarget IS NOT NULL, a.attndims \
             FROM pg_catalog.pg_attribute a \
             LEFT JOIN pg_catalog.pg_attrdef ad ON ad.adrelid = a.attrelid AND ad.adnum = a.attnum \
             LEFT JOIN pg_catalog.pg_type t ON t.oid = a.atttypid \
             LEFT JOIN pg_catalog.pg_namespace tn ON tn.oid = t.typnamespace \
             LEFT JOIN pg_catalog.pg_collation co ON co.oid = a.attcollation \
             LEFT JOIN pg_catalog.pg_namespace cn ON cn.oid = co.collnamespace \
             WHERE a.attrelid = $1 AND a.attnum > 0 AND NOT a.attisdropped ORDER BY a.attnum",
            vec![&relation.oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(stream);
    let mut columns = Vec::new();
    let mut dimensions = Vec::new();
    let mut collations = Vec::new();
    let mut generations = Vec::new();
    let mut expressions = Vec::new();
    let mut identities = Vec::new();
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
        let collation_schema: Option<String> = field(&row, 18)?;
        let collation_name: Option<String> = field(&row, 19)?;
        let collation_deterministic: Option<bool> = field(&row, 20)?;
        let array_dimensions: i16 = field(&row, 22)?;
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
                + expression.as_ref().map_or(0, String::len)
                + collation_schema.as_ref().map_or(0, String::len)
                + collation_name.as_ref().map_or(0, String::len),
        )?;
        if ordinal <= 0
            || array_dimensions < 0
            || field::<bool>(&row, 7)? != expression.is_some()
            || field::<bool>(&row, 17)?
            || field::<bool>(&row, 21)?
            || (relation.kind == RelationKind::CompositeType
                && (not_null
                    || expression.is_some()
                    || !generated.is_empty()
                    || !identity.is_empty()))
            || [10, 12, 13]
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
        dimensions.push(
            ColumnArrayDimensionsObservation::new(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
                array_dimensions as u16,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
        let collation = match (
            field::<bool>(&row, 11)?,
            collation_schema,
            collation_name,
            collation_deterministic,
        ) {
            (false, None, None, None) => ColumnCollationObservation::uncollatable(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            (true, Some(schema), Some(collation), Some(deterministic)) => {
                ColumnCollationObservation::collatable(
                    &relation.schema,
                    &relation.name,
                    relation.kind,
                    &name,
                    QualifiedCollationName::new(schema, collation)
                        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                    deterministic,
                )
            }
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        collations.push(collation);
        let generation = match generated.as_str() {
            "" => ColumnGenerationObservation::not_generated(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            "s" => ColumnGenerationObservation::stored(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            "v" => ColumnGenerationObservation::virtual_generated(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let observed_expression = match (generated.as_str(), expression) {
            ("", None) => ColumnExpressionObservation::no_expression(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            ("", Some(value)) => ColumnExpressionObservation::default_expression(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
                value,
            ),
            ("s" | "v", Some(value)) => ColumnExpressionObservation::generation_expression(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
                value,
            ),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        generations.push(generation);
        expressions.push(observed_expression);
        let observed_identity = match identity.as_str() {
            "" => ColumnIdentityObservation::not_identity(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            "a" => ColumnIdentityObservation::generated_always(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            "d" => ColumnIdentityObservation::generated_by_default(
                &relation.schema,
                &relation.name,
                relation.kind,
                &name,
            ),
            _ => return Err(SourceObservationFailure::InvalidCapturedMetadata),
        }
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        identities.push(observed_identity);
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
    let identity_stream = bounded(
        request,
        cancellation,
        transaction.query_raw(
            "SELECT a.attname::text, sn.nspname::text, s.relname::text, \
             tn.nspname::text, st.typname::text, ps.seqstart, ps.seqincrement, \
             ps.seqmin, ps.seqmax, ps.seqcache, ps.seqcycle, \
             s.relkind::text, s.relpersistence::text, s.relacl IS NOT NULL, \
             s.reloptions IS NOT NULL, s.relowner = t.relowner, ps.seqtypid = a.atttypid, \
             CASE WHEN octet_length(pg_catalog.obj_description(s.oid, 'pg_class')) <= $2::bigint \
               THEN pg_catalog.obj_description(s.oid, 'pg_class') END, \
             COALESCE(octet_length(pg_catalog.obj_description(s.oid, 'pg_class')) > $2::bigint, false), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel l \
               WHERE l.classoid = 'pg_class'::regclass AND l.objoid = s.oid), \
             EXISTS(SELECT 1 FROM pg_catalog.pg_depend e \
               WHERE e.classid = 'pg_class'::regclass AND e.objid = s.oid AND e.deptype = 'e') \
             FROM pg_catalog.pg_attribute a \
             JOIN pg_catalog.pg_class t ON t.oid = a.attrelid \
             LEFT JOIN pg_catalog.pg_depend d ON d.classid = 'pg_class'::regclass \
               AND d.refclassid = 'pg_class'::regclass AND d.refobjid = a.attrelid \
               AND d.refobjsubid = a.attnum AND d.objsubid = 0 AND d.deptype = 'i' \
             LEFT JOIN pg_catalog.pg_class s ON s.oid = d.objid \
             LEFT JOIN pg_catalog.pg_namespace sn ON sn.oid = s.relnamespace \
             LEFT JOIN pg_catalog.pg_sequence ps ON ps.seqrelid = s.oid \
             LEFT JOIN pg_catalog.pg_type st ON st.oid = ps.seqtypid \
             LEFT JOIN pg_catalog.pg_namespace tn ON tn.oid = st.typnamespace \
             WHERE a.attrelid = $1 AND a.attnum > 0 AND a.attidentity <> '' \
             ORDER BY a.attnum, s.oid",
            vec![&relation.oid as &(dyn ToSql + Sync), &max_bytes],
        ),
    )
    .await?;
    tokio::pin!(identity_stream);
    while let Some(row) = bounded(request, cancellation, identity_stream.try_next()).await? {
        let column_name: String = field(&row, 0)?;
        let sequence_schema: Option<String> = field(&row, 1)?;
        let sequence_name: Option<String> = field(&row, 2)?;
        let type_schema: Option<String> = field(&row, 3)?;
        let type_name: Option<String> = field(&row, 4)?;
        let comment: Option<String> = field(&row, 17)?;
        if field::<bool>(&row, 18)? {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        meter.add(
            request,
            64 + column_name.len()
                + sequence_schema.as_ref().map_or(0, String::len)
                + sequence_name.as_ref().map_or(0, String::len)
                + type_schema.as_ref().map_or(0, String::len)
                + type_name.as_ref().map_or(0, String::len)
                + comment.as_ref().map_or(0, String::len),
        )?;
        let (Some(sequence_schema), Some(sequence_name), Some(type_schema), Some(type_name)) =
            (sequence_schema, sequence_name, type_schema, type_name)
        else {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        };
        if sequence_schema != relation.schema
            || field::<String>(&row, 11)? != "S"
            || field::<String>(&row, 12)? != "p"
            || [13, 14, 19, 20]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(false))
            || [15, 16]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(true))
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let sequence = IdentitySequenceObservation::new(
            QualifiedTypeName::new(sequence_schema, sequence_name)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            QualifiedTypeName::new(type_schema, type_name)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
            field(&row, 5)?,
            field(&row, 6)?,
            field(&row, 7)?,
            field(&row, 8)?,
            field(&row, 9)?,
            field(&row, 10)?,
            comment,
        )
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let identity = identities
            .iter_mut()
            .find(|identity| identity.column_name() == column_name && !identity.is_not_identity())
            .ok_or(SourceObservationFailure::InvalidCapturedMetadata)?;
        *identity = identity
            .clone()
            .with_sequence(sequence)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    }
    if identities
        .iter()
        .any(|identity| !identity.is_not_identity() && identity.sequence().is_none())
    {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
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
    let (constraints, not_null, timings) = capture_constraints(
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
        RelationObservation::new(relation.schema, relation.name, relation.kind, columns)
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    if relation.kind == RelationKind::Table {
        observed = observed.with_replica_identity_mode(relation.replica_identity);
    }
    observed = observed
        .with_constraints(constraints)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?
        .with_indexes(indexes)
        .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
    if let Some(comment) = relation.comment {
        observed = observed.with_source_comment(comment);
    }
    Ok((
        observed,
        dimensions,
        collations,
        generations,
        expressions,
        identities,
        not_null,
        timings,
    ))
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
        Vec<ConstraintTimingObservation>,
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
    let mut timings = Vec::new();
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
        if kind == "f" {
            continue;
        }
        if field::<i16>(&row, 8)? != 0
            || [9, 13, 14, 17]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(true))
            || [15, 18, 19]
                .into_iter()
                .any(|index| field::<bool>(&row, index) != Ok(false))
            || !field::<bool>(&row, 7)?
            || !(22..=27).all(|index| field::<bool>(&row, index) == Ok(true))
            || !(28..=30).all(|index| field::<String>(&row, index) == Ok(" ".to_owned()))
        {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let constraint = match kind.as_str() {
            "c" if field::<bool>(&row, 12)?
                && field::<bool>(&row, 16)?
                && !field::<bool>(&row, 10)?
                && !field::<bool>(&row, 11)? =>
            {
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
                && !field::<bool>(&row, 10)?
                && !field::<bool>(&row, 11)?
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
                let deferrability = match (field(&row, 10)?, field(&row, 11)?) {
                    (false, false) => ConstraintDeferrability::NotDeferrable,
                    (true, false) => ConstraintDeferrability::InitiallyImmediate,
                    (true, true) => ConstraintDeferrability::InitiallyDeferred,
                    (false, true) => return Err(SourceObservationFailure::InvalidCapturedMetadata),
                };
                timings.push(
                    ConstraintTimingObservation::new(
                        &relation.schema,
                        &relation.name,
                        RelationKind::Table,
                        &name,
                        deferrability,
                    )
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                );
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
    Ok((constraints, not_null, timings))
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
