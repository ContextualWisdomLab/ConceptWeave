use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

use conceptweave_alignment::{
    AlignmentDecision, AlignmentError, align_relational_proposal, validate_alignment,
};
use conceptweave_client::{ReleaseMetadata, SemanticReleaseClient, TrustedPublisherKey};
use conceptweave_discovery::{ProposalError, ProposedSourceType, propose_relational_model};
use conceptweave_domain::{CandidateKind, PublicationState, TruthStatus};
use conceptweave_governance::{
    FilePublicationStore, GovernanceError, PublicationStoreError, ReviewRequest,
    StewardReviewAuthority, review, sign_published_manifest,
};
use conceptweave_observation::{
    ArrayTypeObservation, CollationProvider, ColumnArrayDimensionsObservation,
    ColumnCollationObservation, ColumnExpressionObservation, ColumnGenerationObservation,
    ColumnIdentityObservation, ColumnObservationV3, ConstraintDeferrability, DomainObservation,
    EnumObservation, ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType,
    ForeignKeyObservation, IndexTablespace, PostgresSchemaSnapshotV3, PostgresTypeKind,
    QualifiedTypeName, RelationKind, RelationObservation, RelationOwnerObservation,
    RelationTablespaceObservation, ReplicaIdentityMode, SchemaObjectLocation,
    TableConstraintObservation, TypeKindObservation, TypeOwnerObservation,
};
use conceptweave_postgres_adapter::{PostgresTlsAdapter, PostgresUnixAdapter};
use conceptweave_source_port::{
    ObservationCancellation, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
    SourceObservationFailure, SourceObservationPort,
};
use ring::{
    rand::SystemRandom,
    signature::{Ed25519KeyPair, KeyPair},
};
use tokio_postgres::{Config, NoTls};

#[tokio::test]
async fn postgres18_range_subtype_changes_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_range_subtype_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA {schema}; CREATE TYPE {schema}.span AS RANGE (subtype = int4)"
        ))
        .await
        .unwrap();
    let before = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "DROP TYPE {schema}.span CASCADE; CREATE TYPE {schema}.span AS RANGE (subtype = int8)"
        ))
        .await
        .unwrap();
    let after = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    let [original] = before.range_catalog().unwrap() else {
        panic!("one range definition expected");
    };
    let [changed] = after.range_catalog().unwrap() else {
        panic!("one range definition expected");
    };
    assert_eq!(original.subtype().type_name(), "int4");
    assert_eq!(changed.subtype().type_name(), "int8");
    assert_eq!(
        original.subtype_operator_class().operator_class_name(),
        "int4_ops"
    );
    assert_eq!(
        changed.subtype_operator_class().operator_class_name(),
        "int8_ops"
    );
    let receipt = after
        .range_catalog_source_receipt(QualifiedTypeName::new(&schema, "span").unwrap())
        .unwrap();
    assert_eq!(receipt.source_digest(), after.snapshot_digest());
    assert_eq!(
        receipt.canonical_location(),
        format!("/schemas/{schema}/ranges/span/catalog")
    );
    assert!(
        after
            .range_catalog_source_receipt(QualifiedTypeName::new(&schema, "unknown").unwrap())
            .is_err()
    );
    assert_ne!(before.snapshot_digest(), after.snapshot_digest());
}

#[tokio::test]
async fn postgres18_range_collation_and_difference_function_are_bound() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_range_collation_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA {schema}; \
             CREATE FUNCTION {schema}.text_diff(text, text) RETURNS float8 \
               LANGUAGE SQL IMMUTABLE STRICT AS $$ SELECT (length($1)-length($2))::float8 $$; \
             CREATE TYPE {schema}.span AS RANGE \
               (subtype=text, collation=\"C\", subtype_diff={schema}.text_diff)"
        ))
        .await
        .unwrap();
    let result = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    let snapshot = result.unwrap();
    let [range] = snapshot.range_catalog().unwrap() else {
        panic!("one range definition expected");
    };
    assert_eq!(range.subtype().type_name(), "text");
    assert_eq!(range.collation().unwrap().collation_name(), "C");
    assert_eq!(
        range.subtype_difference().unwrap().procedure_name(),
        "text_diff"
    );
    assert!(
        snapshot
            .collation_definitions()
            .unwrap()
            .iter()
            .any(|definition| { definition.collation().collation_name() == "C" })
    );
}

#[tokio::test]
async fn postgres18_range_difference_function_body_changes_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_range_function_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA {schema}; \
             CREATE FUNCTION {schema}.text_diff(text, text) RETURNS float8 \
               LANGUAGE SQL IMMUTABLE STRICT AS $$ SELECT (length($1)-length($2))::float8 $$; \
             CREATE TYPE {schema}.span AS RANGE (subtype=text, subtype_diff={schema}.text_diff)"
        ))
        .await
        .unwrap();
    let before = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "CREATE OR REPLACE FUNCTION {schema}.text_diff(text, text) RETURNS float8 \
             LANGUAGE SQL IMMUTABLE STRICT AS $$ SELECT (length($1)+length($2))::float8 $$"
        ))
        .await
        .unwrap();
    let after = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!(
            "COMMENT ON FUNCTION {schema}.text_diff(text, text) IS 'unmodeled'"
        ))
        .await
        .unwrap();
    let commented = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!("DROP SCHEMA {schema} CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    let after = after.unwrap();
    assert_ne!(before.snapshot_digest(), after.snapshot_digest());
    assert!(matches!(
        commented,
        Err(SourceObservationFailure::InvalidCapturedMetadata)
    ));
}

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, key: &str) -> bool {
        key == "fixture_source"
    }

    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "fixture_source").then(|| "fixture_policy".to_owned())
    }

    fn authorizes_schema_scope(&self, connection: &ResolvedSourceConnection, _: &[String]) -> bool {
        connection.connection_policy_binding() == "fixture_policy"
    }

    fn authorizes_resource_envelope(
        &self,
        _: &ResolvedSourceConnection,
        envelope: ObservationResourceEnvelope,
    ) -> bool {
        envelope.limits().max_concurrent_queries() == 1
    }
}

#[tokio::test]
async fn postgres18_quoted_identifiers_keep_exact_source_coordinates() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!(" cw/~ quoted {} ", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE DOMAIN \"{schema}\".\" domain/~ \" AS integer \
               CONSTRAINT \" domain check \" CHECK (VALUE >= 0); \
             CREATE TYPE \"{schema}\".\" enum/~ \" AS ENUM ('first'); \
             CREATE TABLE \"{schema}\".\" table/~ \" \
               (\" /~ \" \"{schema}\".\" domain/~ \" NOT NULL, \
                \" ~1~0 \" text, \
                state \"{schema}\".\" enum/~ \", \
                CONSTRAINT \" pk/~ \" PRIMARY KEY (\" /~ \"))"
        ))
        .await
        .unwrap();
    let result = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();

    let snapshot = result.unwrap();
    let relation = &snapshot.relations()[0];
    assert_eq!(relation.schema_name(), schema);
    assert_eq!(relation.relation_name(), " table/~ ");
    assert_eq!(relation.columns()[0].column_name(), " /~ ");
    assert_eq!(relation.columns()[1].column_name(), " ~1~0 ");
    assert_eq!(relation.constraints()[0].constraint_name(), " pk/~ ");
    assert_eq!(relation.indexes()[0].index_name(), " pk/~ ");
    assert_eq!(snapshot.domains()[0].domain_name(), " domain/~ ");
    assert_eq!(snapshot.enums()[0].enum_name(), " enum/~ ");
    let slash_column =
        SchemaObjectLocation::column(&schema, " table/~ ", RelationKind::Table, " /~ ").unwrap();
    let literal_column =
        SchemaObjectLocation::column(&schema, " table/~ ", RelationKind::Table, " ~1~0 ").unwrap();
    assert_ne!(
        slash_column.canonical_location(),
        literal_column.canonical_location()
    );
    for coordinate in [
        SchemaObjectLocation::relation(&schema, " table/~ ", RelationKind::Table).unwrap(),
        slash_column,
        literal_column,
        SchemaObjectLocation::constraint(&schema, " table/~ ", RelationKind::Table, " pk/~ ")
            .unwrap(),
        SchemaObjectLocation::index(&schema, " table/~ ", RelationKind::Table, " pk/~ ").unwrap(),
        SchemaObjectLocation::domain(&schema, " domain/~ ").unwrap(),
        SchemaObjectLocation::enum_(&schema, " enum/~ ").unwrap(),
    ] {
        assert!(coordinate.canonical_location().contains("~1~0"));
        assert!(snapshot.source_receipt(coordinate).is_ok());
    }
}

#[tokio::test]
async fn postgres18_dropped_column_tombstone_does_not_hide_live_columns() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_dropped_column_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TABLE \"{schema}\".item (first_id integer, retired text, last_id integer); \
             ALTER TABLE \"{schema}\".item DROP COLUMN retired; \
             CREATE INDEX item_last_idx ON \"{schema}\".item (last_id)"
        ))
        .await
        .unwrap();
    let dropped_ordinal: i16 = client
        .query_one(
            "SELECT attnum FROM pg_catalog.pg_attribute \
             WHERE attrelid = to_regclass($1) AND attisdropped",
            &[&format!("\"{schema}\".item")],
        )
        .await
        .unwrap()
        .get(0);
    assert_eq!(dropped_ordinal, 2);

    let result = adapter(config)
        .observe(authorized_with_limits(&schema, 64, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();

    let snapshot = result.unwrap();
    let columns = snapshot.relations()[0]
        .columns()
        .iter()
        .map(|column| (column.column_name(), column.ordinal_position()))
        .collect::<Vec<_>>();
    assert_eq!(columns, [("first_id", 1), ("last_id", 3)]);
    assert_eq!(
        snapshot.relations()[0].indexes()[0].key_attributes()[0].attribute_name(),
        Some("last_id")
    );
    assert!(
        snapshot
            .source_receipt(
                SchemaObjectLocation::column(&schema, "item", RelationKind::Table, "retired")
                    .unwrap(),
            )
            .is_err()
    );
    assert!(
        snapshot
            .source_receipt(
                SchemaObjectLocation::column(&schema, "item", RelationKind::Table, "last_id")
                    .unwrap(),
            )
            .is_ok()
    );
}

struct NotCancelled;

impl ObservationCancellation for NotCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

struct Cancelled;

struct FixtureSteward<'a> {
    proposal_id: &'a str,
    source_digest: &'a str,
}

impl StewardReviewAuthority for FixtureSteward<'_> {
    fn authorize_review(&self, request: &ReviewRequest) -> Result<Option<String>, GovernanceError> {
        assert_eq!(request.proposal_id(), self.proposal_id);
        assert_eq!(request.source_digest(), self.source_digest);
        assert_eq!(request.steward_id(), "fixture-steward");
        assert!(request.alignment_digest().starts_with("sha256:"));
        Ok(Some("fixture-audit-receipt".to_owned()))
    }
}

struct DeniedSteward;

fn take_u64(bytes: &mut &[u8]) -> usize {
    let (number, remaining) = bytes.split_at(8);
    *bytes = remaining;
    usize::try_from(u64::from_be_bytes(number.try_into().unwrap())).unwrap()
}

fn take_blob<'a>(bytes: &mut &'a [u8]) -> &'a [u8] {
    let len = take_u64(bytes);
    let (blob, remaining) = bytes.split_at(len);
    *bytes = remaining;
    blob
}

fn take_text<'a>(bytes: &mut &'a [u8]) -> &'a str {
    std::str::from_utf8(take_blob(bytes)).unwrap()
}

fn take_byte(bytes: &mut &[u8]) -> u8 {
    let value = bytes[0];
    *bytes = &bytes[1..];
    value
}

fn take_optional_text<'a>(bytes: &mut &'a [u8]) -> Option<&'a str> {
    (take_byte(bytes) == 1).then(|| take_text(bytes))
}

fn take_optional_strings<'a>(bytes: &mut &'a [u8]) -> Option<Vec<&'a str>> {
    (take_byte(bytes) == 1).then(|| {
        let count = take_u64(bytes);
        (0..count).map(|_| take_text(bytes)).collect()
    })
}

fn take_optional_i32(bytes: &mut &[u8]) -> Option<i32> {
    (take_byte(bytes) == 1).then(|| {
        let (value, remaining) = bytes.split_at(4);
        *bytes = remaining;
        i32::from_be_bytes(value.try_into().unwrap())
    })
}

fn take_optional_u32(bytes: &mut &[u8]) -> Option<u32> {
    (take_byte(bytes) == 1).then(|| {
        let (value, remaining) = bytes.split_at(4);
        *bytes = remaining;
        u32::from_be_bytes(value.try_into().unwrap())
    })
}

impl StewardReviewAuthority for DeniedSteward {
    fn authorize_review(&self, _: &ReviewRequest) -> Result<Option<String>, GovernanceError> {
        Ok(None)
    }
}

impl ObservationCancellation for Cancelled {
    fn is_cancelled(&self) -> bool {
        true
    }
}

fn authorized_with_limits(
    schema: &str,
    max_rows: u64,
    max_bytes: u64,
) -> conceptweave_source_port::AuthorizedObservationRequest {
    ObservationRequest::new(
        "fixture_source",
        vec![schema.to_owned()],
        ObservationRequestBudget::new(1, 128).unwrap(),
        ObservationLimits::with_timeouts(10_000, 3_000, max_rows, max_bytes, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn authorized(schema: &str) -> conceptweave_source_port::AuthorizedObservationRequest {
    authorized_with_limits(schema, 64, 8_192)
}

fn authorized_two(schemas: [&str; 2]) -> conceptweave_source_port::AuthorizedObservationRequest {
    ObservationRequest::new(
        "fixture_source",
        schemas.map(str::to_owned).to_vec(),
        ObservationRequestBudget::new(2, 256).unwrap(),
        ObservationLimits::with_timeouts(10_000, 3_000, 64, 8_192, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn adapter(config: Config) -> PostgresUnixAdapter {
    PostgresUnixAdapter::new(BTreeMap::from([(
        "fixture_source".to_owned(),
        ("fixture_policy".to_owned(), config),
    )]))
}

#[test]
fn relational_proposal_rejects_unmodeled_shapes_and_incomplete_references() {
    let authorized = || authorized_with_limits("public", 128, 16_384);
    let view = RelationObservation::new("public", "summary", RelationKind::View, vec![]).unwrap();
    let snapshot = PostgresSchemaSnapshotV3::new(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![view],
        vec![],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        propose_relational_model(&snapshot),
        Err(ProposalError::UnsupportedRelationKind)
    ));

    let column = |name: &str| {
        ColumnObservationV3::new(
            name,
            1,
            "integer",
            QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            true,
            None,
        )
        .unwrap()
    };
    let child = RelationObservation::new(
        "public",
        "child",
        RelationKind::Table,
        vec![column("parent_id")],
    )
    .unwrap()
    .with_constraints(vec![TableConstraintObservation::ForeignKey(
        ForeignKeyObservation::new(
            "child_parent_fk",
            vec!["parent_id".to_owned()],
            "public",
            "parent",
            vec!["id".to_owned()],
        )
        .unwrap(),
    )])
    .unwrap();
    let missing_parent = PostgresSchemaSnapshotV3::new(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![child.clone()],
        vec![],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        propose_relational_model(&missing_parent),
        Err(ProposalError::MissingForeignKeyTarget)
    ));

    let parent =
        RelationObservation::new("public", "parent", RelationKind::Table, vec![column("id")])
            .unwrap();
    let unknown_state = PostgresSchemaSnapshotV3::new(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![child, parent],
        vec![],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        propose_relational_model(&unknown_state),
        Err(ProposalError::UnobservedForeignKeyState)
    ));

    let partial = PostgresSchemaSnapshotV3::new(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![
            RelationObservation::new("public", "plain", RelationKind::Table, vec![column("id")])
                .unwrap(),
        ],
        vec![],
        vec![],
    )
    .unwrap();
    assert!(matches!(
        propose_relational_model(&partial),
        Err(ProposalError::IncompleteSourceObservation)
    ));

    let identity_unobserved = PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![
            RelationObservation::new("public", "plain", RelationKind::Table, vec![column("id")])
                .unwrap(),
        ],
        vec![],
        vec![],
        vec![],
        vec![
            TypeKindObservation::plain(
                QualifiedTypeName::new("public", "plain").unwrap(),
                PostgresTypeKind::Composite,
            )
            .unwrap(),
        ],
    )
    .unwrap()
    .with_observed_relation_tablespaces(vec![
        RelationTablespaceObservation::new(
            "public",
            "plain",
            IndexTablespace::database_default("pg_default").unwrap(),
        )
        .unwrap(),
    ])
    .unwrap()
    .with_observed_relation_owners(vec![
        RelationOwnerObservation::new("public", "plain", 42, "fixture_owner").unwrap(),
    ])
    .unwrap()
    .with_observed_type_owners(vec![
        TypeOwnerObservation::new(
            QualifiedTypeName::new("public", "plain").unwrap(),
            42,
            "fixture_owner",
        )
        .unwrap(),
    ])
    .unwrap()
    .with_observed_column_collations(vec![
        ColumnCollationObservation::uncollatable("public", "plain", RelationKind::Table, "id")
            .unwrap(),
    ])
    .unwrap()
    .with_observed_column_generations(vec![
        ColumnGenerationObservation::not_generated("public", "plain", RelationKind::Table, "id")
            .unwrap(),
    ])
    .unwrap()
    .with_observed_column_expressions(vec![
        ColumnExpressionObservation::no_expression("public", "plain", RelationKind::Table, "id")
            .unwrap(),
    ])
    .unwrap();
    let identity_observed = identity_unobserved
        .clone()
        .with_observed_column_identities(vec![
            ColumnIdentityObservation::not_identity("public", "plain", RelationKind::Table, "id")
                .unwrap(),
        ])
        .unwrap();
    let complete = |snapshot: PostgresSchemaSnapshotV3, observe_periods: bool| {
        let dimensions = snapshot
            .relations()
            .iter()
            .flat_map(|relation| {
                relation.columns().iter().map(|column| {
                    ColumnArrayDimensionsObservation::new(
                        relation.schema_name(),
                        relation.relation_name(),
                        relation.kind(),
                        column.column_name(),
                        0,
                    )
                    .unwrap()
                })
            })
            .collect();
        let snapshot = snapshot
            .with_observed_not_null_constraints(vec![])
            .unwrap()
            .with_observed_constraint_timings(vec![])
            .unwrap();
        let snapshot = if observe_periods {
            snapshot.with_observed_constraint_periods(vec![]).unwrap()
        } else {
            snapshot
        };
        snapshot
            .with_observed_foreign_key_catalog(vec![])
            .unwrap()
            .with_observed_collation_definitions(vec![])
            .unwrap()
            .with_observed_column_array_dimensions(dimensions)
            .unwrap()
    };
    assert!(matches!(
        propose_relational_model(&complete(identity_unobserved, true)),
        Err(ProposalError::IncompleteSourceObservation)
    ));
    assert!(matches!(
        propose_relational_model(&complete(identity_observed.clone(), false)),
        Err(ProposalError::IncompleteSourceObservation)
    ));
    assert!(propose_relational_model(&complete(identity_observed, true)).is_ok());
    let missing_row_kind = PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![RelationObservation::new("public", "plain", RelationKind::Table, vec![]).unwrap()],
        vec![],
        vec![],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_column_collations(vec![])
    .unwrap()
    .with_observed_column_generations(vec![])
    .unwrap()
    .with_observed_column_expressions(vec![])
    .unwrap()
    .with_observed_column_identities(vec![])
    .unwrap();
    assert!(matches!(
        propose_relational_model(&complete(missing_row_kind, true)),
        Err(ProposalError::IncompleteSourceObservation)
    ));
    let missing_domain_kind = PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![],
        vec![
            DomainObservation::new(
                "public",
                "risk_code",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
            )
            .unwrap(),
        ],
        vec![],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_collation_definitions(vec![])
    .unwrap();
    assert!(matches!(
        propose_relational_model(&missing_domain_kind),
        Err(ProposalError::IncompleteSourceObservation)
    ));

    let enums = vec![
        EnumObservation::new("public", "stage", vec!["draft".to_owned()]).unwrap(),
        EnumObservation::new("public", "severity", vec!["high".to_owned()]).unwrap(),
    ];
    let incomplete_type_only = PostgresSchemaSnapshotV3::new(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![],
        vec![],
        enums.clone(),
    )
    .unwrap();
    assert!(matches!(
        propose_relational_model(&incomplete_type_only),
        Err(ProposalError::IncompleteSourceObservation)
    ));
    let array_types = ["stage", "severity"]
        .into_iter()
        .map(|name| {
            ArrayTypeObservation::new(
                QualifiedTypeName::new("public", format!("_{name}")).unwrap(),
                QualifiedTypeName::new("public", name).unwrap(),
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let type_kinds = ["stage", "severity"]
        .into_iter()
        .flat_map(|name| {
            [
                TypeKindObservation::plain(
                    QualifiedTypeName::new("public", name).unwrap(),
                    PostgresTypeKind::Enum,
                )
                .unwrap(),
                TypeKindObservation::plain(
                    QualifiedTypeName::new("public", format!("_{name}")).unwrap(),
                    PostgresTypeKind::Base,
                )
                .unwrap(),
            ]
        })
        .collect::<Vec<_>>();
    let missing_arrays = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![],
        vec![],
        enums.clone(),
        type_kinds.clone(),
    )
    .unwrap()
    .with_observed_collation_definitions(vec![])
    .unwrap();
    let missing_type_kinds = PostgresSchemaSnapshotV3::new_with_array_types(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![],
        vec![],
        enums.clone(),
        array_types.clone(),
    )
    .unwrap()
    .with_observed_collation_definitions(vec![])
    .unwrap();
    let missing_collations = PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![],
        vec![],
        enums.clone(),
        array_types.clone(),
        type_kinds.clone(),
    )
    .unwrap();
    let missing_enum_kind = PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
        &authorized(),
        "fixture",
        "2026-09-25T00:00:00Z",
        vec![],
        vec![],
        enums.clone(),
        array_types.clone(),
        type_kinds
            .iter()
            .filter(|kind| kind.type_name().type_name() != "severity")
            .cloned()
            .collect(),
    )
    .unwrap()
    .with_observed_collation_definitions(vec![])
    .unwrap();
    for incomplete in [&missing_arrays, &missing_type_kinds, &missing_collations] {
        assert!(matches!(
            propose_relational_model(incomplete),
            Err(ProposalError::IncompleteSourceObservation)
        ));
    }
    assert!(matches!(
        propose_relational_model(&missing_enum_kind),
        Err(ProposalError::IncompleteSourceObservation)
    ));
    let complete_type_only = |enums, observed_at| {
        PostgresSchemaSnapshotV3::new_with_array_types_and_type_kinds(
            &authorized(),
            "fixture",
            observed_at,
            vec![],
            vec![],
            enums,
            array_types.clone(),
            type_kinds.clone(),
        )
        .unwrap()
        .with_observed_type_owners(
            type_kinds
                .iter()
                .map(|kind| {
                    TypeOwnerObservation::new(kind.type_name().clone(), 42, "fixture_owner")
                        .unwrap()
                })
                .collect(),
        )
        .unwrap()
        .with_observed_collation_definitions(vec![])
        .unwrap()
    };
    let type_only = complete_type_only(enums.clone(), "2026-09-25T00:00:00Z");
    let reordered = complete_type_only(enums.into_iter().rev().collect(), "2026-09-25T01:00:00Z");
    let type_proposal = propose_relational_model(&type_only).unwrap();
    assert_eq!(type_proposal, propose_relational_model(&reordered).unwrap());
    assert!(type_proposal.concepts().is_empty());
    assert!(type_proposal.relations().is_empty());
    assert_eq!(type_proposal.source_types().len(), 2);
    assert_eq!(
        type_proposal.source_types()[0].candidate().evidence()[0].source_digest(),
        type_only.snapshot_digest()
    );

    let first_id = type_proposal.source_types()[0].candidate().candidate_id();
    let second_id = type_proposal.source_types()[1].candidate().candidate_id();
    let decisions = vec![
        AlignmentDecision::map(first_id, "risk.severity", "Severity", "Fixture glossary").unwrap(),
        AlignmentDecision::exclude(second_id, "Not part of this model").unwrap(),
    ];
    let aligned = align_relational_proposal(
        &type_proposal,
        type_proposal.proposal_id(),
        decisions.clone(),
    )
    .unwrap();
    let reversed = align_relational_proposal(
        &type_proposal,
        type_proposal.proposal_id(),
        decisions.into_iter().rev().collect(),
    )
    .unwrap();
    assert_eq!(aligned, reversed);
    let validated = validate_alignment(&aligned).unwrap();
    assert_eq!(validated.proposal_id(), type_proposal.proposal_id());
    assert_eq!(
        review(
            &validated,
            "fixture-steward",
            "Type-only proposal",
            &DeniedSteward
        )
        .unwrap_err(),
        GovernanceError::NoSemanticConcept
    );
    assert_eq!(validated.source_digest(), type_only.snapshot_digest());
    assert_eq!(validated.validation().source_bound_candidates(), 2);
    assert_eq!(validated.validation().unique_semantic_ids(), 1);
    assert_eq!(validated.validation().mapped_relations_with_endpoints(), 0);
    assert_eq!(validated.validation().mapped_fields_with_concepts(), 0);
    assert_eq!(
        validated.candidates()[0].candidate().publication_state(),
        PublicationState::Validated
    );
    assert_eq!(
        validated.candidates()[1].candidate().publication_state(),
        PublicationState::Rejected
    );
    assert!(matches!(
        align_relational_proposal(&type_proposal, "stale", vec![]),
        Err(AlignmentError::StaleProposal)
    ));
    assert!(matches!(
        align_relational_proposal(
            &type_proposal,
            type_proposal.proposal_id(),
            vec![AlignmentDecision::exclude(first_id, "Omit").unwrap()]
        ),
        Err(AlignmentError::MissingDecision)
    ));
    assert!(matches!(
        align_relational_proposal(
            &type_proposal,
            type_proposal.proposal_id(),
            vec![
                AlignmentDecision::exclude(first_id, "Omit").unwrap(),
                AlignmentDecision::exclude(second_id, "Omit").unwrap(),
                AlignmentDecision::exclude("absent", "Omit").unwrap(),
            ]
        ),
        Err(AlignmentError::UnknownCandidate)
    ));
    assert!(matches!(
        align_relational_proposal(
            &type_proposal,
            type_proposal.proposal_id(),
            vec![
                AlignmentDecision::exclude(first_id, "Omit").unwrap(),
                AlignmentDecision::exclude(first_id, "Omit again").unwrap(),
            ]
        ),
        Err(AlignmentError::DuplicateDecision)
    ));
    assert_eq!(
        AlignmentDecision::map(first_id, "\0", "Name", "Fixture").unwrap_err(),
        AlignmentError::InvalidText("semantic_id")
    );
    assert!(matches!(
        align_relational_proposal(
            &type_proposal,
            type_proposal.proposal_id(),
            vec![
                AlignmentDecision::map(first_id, "same", "First", "Fixture").unwrap(),
                AlignmentDecision::map(second_id, "same", "Second", "Fixture").unwrap(),
            ]
        )
        .and_then(|candidate| validate_alignment(&candidate)),
        Err(AlignmentError::DuplicateSemanticIdentity)
    ));
}

#[tokio::test]
async fn postgres18_collation_version_drift_changes_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_collversion_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
         CREATE COLLATION \"{schema}\".casefold \
           (provider = icu, locale = 'und-u-ks-level1', deterministic = false, version = '0'); \
         CREATE TABLE \"{schema}\".record (title text COLLATE \"{schema}\".casefold)"
        ))
        .await
        .unwrap();
    let result = async {
        let before = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 128, 16_384), &NotCancelled)
            .await?;
        let [definition] = before.collation_definitions().unwrap() else {
            panic!("one referenced collation definition must be captured");
        };
        assert_eq!(definition.provider(), CollationProvider::Icu);
        assert_eq!(definition.fields().recorded_version(), Some("0"));
        assert_ne!(
            definition.fields().recorded_version(),
            definition.fields().actual_version()
        );
        let location = SchemaObjectLocation::collation(&schema, "casefold").unwrap();
        let receipt = before.source_receipt(location.clone()).unwrap();
        assert_eq!(receipt.location(), &location);
        assert_eq!(receipt.source_digest(), before.snapshot_digest());
        assert!(
            before
                .source_receipt(SchemaObjectLocation::collation(&schema, "missing").unwrap())
                .is_err()
        );
        client
            .batch_execute(&format!(
                "ALTER COLLATION \"{schema}\".casefold REFRESH VERSION"
            ))
            .await
            .unwrap();
        let after = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 128, 16_384), &NotCancelled)
            .await?;
        assert_ne!(before.snapshot_digest(), after.snapshot_digest());
        let [refreshed] = after.collation_definitions().unwrap() else {
            panic!("one referenced collation definition must be captured");
        };
        assert_eq!(
            refreshed.fields().recorded_version(),
            refreshed.fields().actual_version()
        );
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_anonymized_governance_shape_replays_without_business_rows() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_governance_shape_{}", std::process::id());
    client
        .batch_execute(&include_str!("fixtures/grc_shape.sql").replace("__SCHEMA__", &schema))
        .await
        .unwrap();

    let result = async {
        let source_adapter = adapter(config.clone());
        let observe = || {
            source_adapter.observe(
                authorized_with_limits(&schema, 1024, 262_144),
                &NotCancelled,
            )
        };
        let first = observe().await?;
        let replay = observe().await?;
        assert_eq!(first.snapshot_digest(), replay.snapshot_digest());
        assert_eq!(first.relations().len(), 4);
        assert_eq!(first.domains().len(), 1);
        assert_eq!(first.enums().len(), 1);
        assert_eq!(first.foreign_key_catalog().unwrap().len(), 4);
        let catalog_periods = client
            .query(
                "SELECT t.relname::text, con.conname::text, con.conperiod \
                 FROM pg_catalog.pg_constraint con \
                 JOIN pg_catalog.pg_class t ON t.oid = con.conrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.relnamespace \
                 WHERE n.nspname = $1 AND con.contype IN ('p', 'u', 'f')",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                (
                    row.get::<_, String>(0),
                    row.get::<_, String>(1),
                    row.get::<_, bool>(2),
                )
            })
            .collect::<BTreeSet<_>>();
        let observed_periods = first
            .constraint_periods()
            .unwrap()
            .iter()
            .map(|period| {
                (
                    period.relation_name().to_owned(),
                    period.constraint_name().to_owned(),
                    period.has_period_semantics(),
                )
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_periods, catalog_periods);
        let catalog_relations = client
            .query(
                "SELECT c.relname::text FROM pg_catalog.pg_class c \
                 JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
                 WHERE n.nspname = $1 AND c.relkind = 'r'",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| row.get::<_, String>(0))
            .collect::<BTreeSet<_>>();
        let observed_relations = first
            .relations()
            .iter()
            .map(|relation| relation.relation_name().to_owned())
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_relations, catalog_relations);
        let replica_mode: String = client
            .query_one(
                "SELECT c.relreplident::text FROM pg_catalog.pg_class c \
                 JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
                 WHERE n.nspname = $1 AND c.relname = 'risk_record'",
                &[&schema],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(replica_mode, "i");
        assert_eq!(
            first
                .relations()
                .iter()
                .find(|relation| relation.relation_name() == "risk_record")
                .unwrap()
                .replica_identity_mode(),
            Some(ReplicaIdentityMode::Index)
        );

        let catalog_columns = client
            .query(
                "SELECT c.relname::text, a.attname::text, a.attnum, \
                 pg_catalog.format_type(a.atttypid, a.atttypmod), tn.nspname::text, \
                 ty.typname::text, a.attnotnull, pg_catalog.col_description(a.attrelid, a.attnum) \
                 FROM pg_catalog.pg_attribute a \
                 JOIN pg_catalog.pg_class c ON c.oid = a.attrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
                 JOIN pg_catalog.pg_type ty ON ty.oid = a.atttypid \
                 JOIN pg_catalog.pg_namespace tn ON tn.oid = ty.typnamespace \
                 WHERE n.nspname = $1 AND c.relkind = 'r' AND a.attnum > 0 AND NOT a.attisdropped",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let relation_name: String = row.get(0);
                let column_name: String = row.get(1);
                let column = first
                    .relations()
                    .iter()
                    .find(|relation| relation.relation_name() == relation_name)
                    .unwrap()
                    .columns()
                    .iter()
                    .find(|column| column.column_name() == column_name)
                    .unwrap();
                assert_eq!(column.ordinal_position(), row.get::<_, i16>(2) as u32);
                assert_eq!(column.data_type(), row.get::<_, String>(3));
                assert_eq!(column.type_binding().schema_name(), row.get::<_, String>(4));
                assert_eq!(column.type_binding().type_name(), row.get::<_, String>(5));
                assert_eq!(column.nullable(), !row.get::<_, bool>(6));
                assert_eq!(
                    column.source_comment(),
                    row.get::<_, Option<String>>(7).as_deref()
                );
                (relation_name, column_name)
            })
            .collect::<BTreeSet<_>>();
        let observed_columns = first
            .relations()
            .iter()
            .flat_map(|relation| {
                relation.columns().iter().map(|column| {
                    (
                        relation.relation_name().to_owned(),
                        column.column_name().to_owned(),
                    )
                })
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_columns, catalog_columns);

        let catalog_indexes = client
            .query(
                "SELECT t.relname::text, i.relname::text, am.amname::text, \
                 pg_catalog.pg_get_indexdef(i.oid), x.indisunique, x.indnullsnotdistinct, \
                 x.indisready, x.indisvalid, x.indislive, \
                 x.indnkeyatts, x.indnatts, pg_catalog.pg_get_expr(x.indpred, x.indrelid), \
                 x.indisprimary, x.indisexclusion, x.indimmediate, x.indisclustered, \
                 x.indcheckxmin, x.indisreplident, \
                 ARRAY(SELECT a.attname::text \
                   FROM generate_series(0, x.indnatts - 1) AS s(pos) \
                   JOIN pg_catalog.pg_attribute a ON a.attrelid = t.oid \
                     AND a.attnum = x.indkey[s.pos] ORDER BY s.pos) \
                 FROM pg_catalog.pg_index x \
                 JOIN pg_catalog.pg_class t ON t.oid = x.indrelid \
                 JOIN pg_catalog.pg_class i ON i.oid = x.indexrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.relnamespace \
                 JOIN pg_catalog.pg_am am ON am.oid = i.relam \
                 WHERE n.nspname = $1 AND t.relkind = 'r'",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let relation_name: String = row.get(0);
                let index_name: String = row.get(1);
                let index = first
                    .relations()
                    .iter()
                    .find(|relation| relation.relation_name() == relation_name)
                    .unwrap()
                    .indexes()
                    .iter()
                    .find(|index| index.index_name() == index_name)
                    .unwrap();
                assert_eq!(
                    index.access_method(),
                    Some(row.get::<_, String>(2).as_str())
                );
                assert_eq!(
                    index.index_definition(),
                    Some(row.get::<_, String>(3).as_str())
                );
                assert_eq!(index.is_unique(), row.get::<_, bool>(4));
                assert_eq!(index.nulls_not_distinct(), Some(row.get::<_, bool>(5)));
                assert_eq!(index.ready(), Some(row.get::<_, bool>(6)));
                assert_eq!(index.valid(), Some(row.get::<_, bool>(7)));
                assert_eq!(index.live(), Some(row.get::<_, bool>(8)));
                let key_count = usize::try_from(row.get::<_, i16>(9)).unwrap();
                let total_count = usize::try_from(row.get::<_, i16>(10)).unwrap();
                assert_eq!(index.key_attributes().len(), key_count);
                assert_eq!(index.include_attributes().len(), total_count - key_count);
                assert_eq!(index.predicate(), row.get::<_, Option<String>>(11).as_deref());
                let flags = index.catalog_flags().unwrap();
                assert_eq!(flags.primary(), row.get::<_, bool>(12));
                assert_eq!(flags.exclusion(), row.get::<_, bool>(13));
                assert_eq!(flags.immediate(), row.get::<_, bool>(14));
                assert_eq!(flags.clustered(), row.get::<_, bool>(15));
                assert_eq!(flags.check_xmin(), row.get::<_, bool>(16));
                assert_eq!(flags.replica_identity(), row.get::<_, bool>(17));
                if relation_name == "risk_record" && index_name == "risk_record_key" {
                    assert!(flags.replica_identity());
                }
                let catalog_key_names: Vec<String> = row.get(18);
                let observed_key_names = index
                    .key_attributes()
                    .iter()
                    .chain(index.include_attributes())
                    .map(|attribute| attribute.attribute_name().unwrap().to_owned())
                    .collect::<Vec<_>>();
                assert_eq!(observed_key_names, catalog_key_names);
                (relation_name, index_name)
            })
            .collect::<BTreeSet<_>>();
        let observed_indexes = first
            .relations()
            .iter()
            .flat_map(|relation| {
                relation.indexes().iter().map(|index| {
                    (
                        relation.relation_name().to_owned(),
                        index.index_name().to_owned(),
                    )
                })
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_indexes, catalog_indexes);

        let catalog_constraints = client
            .query(
                "SELECT t.relname::text, con.conname::text FROM pg_catalog.pg_constraint con \
                 JOIN pg_catalog.pg_class t ON t.oid = con.conrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.relnamespace \
                 WHERE n.nspname = $1 AND t.relkind = 'r' AND con.contype IN ('p','u','f','c')",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
            .collect::<BTreeSet<_>>();
        let observed_constraints = first
            .relations()
            .iter()
            .flat_map(|relation| {
                relation.constraints().iter().map(|constraint| {
                    (
                        relation.relation_name().to_owned(),
                        constraint.constraint_name().to_owned(),
                    )
                })
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_constraints, catalog_constraints);

        let catalog_foreign_keys = client
            .query(
                "SELECT t.relname::text, con.conname::text, rn.nspname::text, \
                 rt.relname::text, ix.relname::text, con.convalidated, con.conenforced, \
                 con.confupdtype::text, con.confdeltype::text, con.confmatchtype::text, \
                 con.condeferrable, con.condeferred, \
                 ARRAY(SELECT a.attname::text FROM unnest(con.conkey) WITH ORDINALITY AS k(attnum, ord) \
                   JOIN pg_catalog.pg_attribute a ON a.attrelid = con.conrelid AND a.attnum = k.attnum \
                   ORDER BY k.ord), \
                 ARRAY(SELECT a.attname::text FROM unnest(con.confkey) WITH ORDINALITY AS k(attnum, ord) \
                   JOIN pg_catalog.pg_attribute a ON a.attrelid = con.confrelid AND a.attnum = k.attnum \
                   ORDER BY k.ord) \
                 FROM pg_catalog.pg_constraint con \
                 JOIN pg_catalog.pg_class t ON t.oid = con.conrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.relnamespace \
                 JOIN pg_catalog.pg_class rt ON rt.oid = con.confrelid \
                 JOIN pg_catalog.pg_namespace rn ON rn.oid = rt.relnamespace \
                 JOIN pg_catalog.pg_class ix ON ix.oid = con.conindid \
                 WHERE n.nspname = $1 AND con.contype = 'f'",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| {
                let relation_name: String = row.get(0);
                let constraint_name: String = row.get(1);
                let relation = first
                    .relations()
                    .iter()
                    .find(|relation| relation.relation_name() == relation_name)
                    .unwrap();
                let TableConstraintObservation::ForeignKey(key) = relation
                    .constraints()
                    .iter()
                    .find(|constraint| constraint.constraint_name() == constraint_name)
                    .unwrap()
                else {
                    panic!("catalog foreign key must remain a foreign key");
                };
                assert_eq!(key.referenced_schema_name(), row.get::<_, String>(2));
                assert_eq!(key.referenced_table_name(), row.get::<_, String>(3));
                assert_eq!(key.validated(), Some(row.get::<_, bool>(5)));
                assert_eq!(key.enforced(), Some(row.get::<_, bool>(6)));
                assert_eq!(key.column_names(), row.get::<_, Vec<String>>(12));
                assert_eq!(key.referenced_column_names(), row.get::<_, Vec<String>>(13));
                let behavior = key.reference_behavior().unwrap();
                assert_eq!(row.get::<_, String>(7), "a");
                assert_eq!(behavior.update_action(), ForeignKeyAction::NoAction);
                assert_eq!(row.get::<_, String>(8), "a");
                assert_eq!(behavior.delete_action(), ForeignKeyAction::NoAction);
                assert_eq!(row.get::<_, String>(9), "s");
                assert_eq!(behavior.match_type(), ForeignKeyMatchType::Simple);
                assert!(!row.get::<_, bool>(10));
                assert!(!row.get::<_, bool>(11));
                assert_eq!(behavior.deferrability(), ForeignKeyDeferrability::NotDeferrable);
                let catalog = first
                    .foreign_key_catalog()
                    .unwrap()
                    .iter()
                    .find(|entry| entry.relation_name() == relation_name && entry.constraint_name() == constraint_name)
                    .unwrap();
                assert_eq!(catalog.referenced_index_name(), row.get::<_, String>(4));
                let receipt = first
                    .source_receipt(
                        SchemaObjectLocation::constraint(
                            &schema,
                            &relation_name,
                            RelationKind::Table,
                            &constraint_name,
                        )
                        .unwrap(),
                    )
                    .unwrap();
                assert_eq!(receipt.source_digest(), first.snapshot_digest());
                (relation_name, constraint_name)
            })
            .collect::<BTreeSet<_>>();
        let observed_foreign_keys = first
            .foreign_key_catalog()
            .unwrap()
            .iter()
            .map(|entry| (entry.relation_name().to_owned(), entry.constraint_name().to_owned()))
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_foreign_keys, catalog_foreign_keys);

        for row in client
            .query(
                "SELECT t.relname::text, con.conname::text, con.conpfeqop, \
                 con.conppeqop, con.conffeqop FROM pg_catalog.pg_constraint con \
                 JOIN pg_catalog.pg_class t ON t.oid = con.conrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.relnamespace \
                 WHERE n.nspname = $1 AND con.contype = 'f'",
                &[&schema],
            )
            .await
            .unwrap()
        {
            let relation_name: String = row.get(0);
            let constraint_name: String = row.get(1);
            let observed = first
                .foreign_key_catalog()
                .unwrap()
                .iter()
                .find(|entry| {
                    entry.relation_name() == relation_name
                        && entry.constraint_name() == constraint_name
                })
                .unwrap();
            for (oids, operators) in [
                (row.get::<_, Vec<u32>>(2), observed.primary_foreign_operators()),
                (row.get::<_, Vec<u32>>(3), observed.primary_primary_operators()),
                (row.get::<_, Vec<u32>>(4), observed.foreign_foreign_operators()),
            ] {
                assert_eq!(oids.len(), operators.len());
                for (oid, operator) in oids.iter().zip(operators) {
                    let signature = client
                        .query_one(
                            "SELECT ns.nspname::text, op.oprname::text, \
                             ln.nspname::text, lt.typname::text, \
                             rn.nspname::text, rt.typname::text \
                             FROM pg_catalog.pg_operator op \
                             JOIN pg_catalog.pg_namespace ns ON ns.oid = op.oprnamespace \
                             JOIN pg_catalog.pg_type lt ON lt.oid = op.oprleft \
                             JOIN pg_catalog.pg_namespace ln ON ln.oid = lt.typnamespace \
                             JOIN pg_catalog.pg_type rt ON rt.oid = op.oprright \
                             JOIN pg_catalog.pg_namespace rn ON rn.oid = rt.typnamespace \
                             WHERE op.oid = $1",
                            &[oid],
                        )
                        .await
                        .unwrap();
                    assert_eq!(operator.schema_name(), signature.get::<_, String>(0));
                    assert_eq!(operator.operator_name(), signature.get::<_, String>(1));
                    assert_eq!(operator.left_type().schema_name(), signature.get::<_, String>(2));
                    assert_eq!(operator.left_type().type_name(), signature.get::<_, String>(3));
                    assert_eq!(operator.right_type().schema_name(), signature.get::<_, String>(4));
                    assert_eq!(operator.right_type().type_name(), signature.get::<_, String>(5));
                }
            }
        }

        let mut catalog_not_null = BTreeMap::new();
        for row in client
            .query(
                "SELECT t.relname::text, con.conname::text, a.attname::text, \
                 con.conkey, con.convalidated, con.conenforced, con.conislocal, \
                 con.coninhcount, con.connoinherit \
                 FROM pg_catalog.pg_constraint con \
                 JOIN pg_catalog.pg_class t ON t.oid = con.conrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.relnamespace \
                 LEFT JOIN pg_catalog.pg_attribute a ON a.attrelid = t.oid \
                   AND a.attnum = con.conkey[1] \
                 WHERE n.nspname = $1 AND t.relkind = 'r' AND con.contype = 'n'",
                &[&schema],
            )
            .await
            .unwrap()
        {
            let key = (row.get::<_, String>(0), row.get::<_, String>(1));
            let column_positions: Vec<i16> = row.get(3);
            assert_eq!(column_positions.len(), 1);
            let value = (
                row.get::<_, Option<String>>(2).unwrap(),
                row.get::<_, bool>(4),
                row.get::<_, bool>(5),
                row.get::<_, bool>(6),
                u16::try_from(row.get::<_, i16>(7)).unwrap(),
                row.get::<_, bool>(8),
            );
            assert!(catalog_not_null.insert(key, value).is_none());
        }
        let mut observed_not_null = BTreeMap::new();
        for constraint in first.not_null_constraints().unwrap() {
            let key = (
                constraint.relation_name().to_owned(),
                constraint.constraint_name().to_owned(),
            );
            let value = (
                constraint.column_name().to_owned(),
                constraint.validated(),
                constraint.enforced(),
                constraint.is_local(),
                constraint.inheritance_ancestor_count(),
                constraint.no_inherit(),
            );
            assert!(observed_not_null.insert(key, value).is_none());
        }
        assert_eq!(observed_not_null, catalog_not_null);
        for relation_and_constraint in observed_not_null.keys() {
            let receipt = first
                .source_receipt(
                    SchemaObjectLocation::constraint(
                        &schema,
                        &relation_and_constraint.0,
                        RelationKind::Table,
                        &relation_and_constraint.1,
                    )
                    .unwrap(),
                )
                .unwrap();
            assert_eq!(receipt.source_digest(), first.snapshot_digest());
        }

        let catalog_types = client
            .query(
                "SELECT t.typtype::text, t.typname::text FROM pg_catalog.pg_type t \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace \
                 WHERE n.nspname = $1 AND t.typtype IN ('d','e')",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
            .collect::<BTreeSet<_>>();
        let observed_types = first
            .domains()
            .iter()
            .map(|domain| ("d".to_owned(), domain.domain_name().to_owned()))
            .chain(
                first
                    .enums()
                    .iter()
                    .map(|enumeration| ("e".to_owned(), enumeration.enum_name().to_owned())),
            )
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_types, catalog_types);
        for row in client
            .query(
                "SELECT t.typname::text, bn.nspname::text, b.typname::text, \
                 t.typtypmod, t.typndims, t.typnotnull, t.typdefault, \
                 cn.nspname::text, c.collname::text, \
                 pg_catalog.obj_description(t.oid, 'pg_type') \
                 FROM pg_catalog.pg_type t \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace \
                 JOIN pg_catalog.pg_type b ON b.oid = t.typbasetype \
                 JOIN pg_catalog.pg_namespace bn ON bn.oid = b.typnamespace \
                 LEFT JOIN pg_catalog.pg_collation c ON c.oid = t.typcollation \
                 LEFT JOIN pg_catalog.pg_namespace cn ON cn.oid = c.collnamespace \
                 WHERE n.nspname = $1 AND t.typtype = 'd'",
                &[&schema],
            )
            .await
            .unwrap()
        {
            let name: String = row.get(0);
            let domain = first
                .domains()
                .iter()
                .find(|domain| domain.domain_name() == name)
                .unwrap();
            assert_eq!(domain.base_type().schema_name(), row.get::<_, String>(1));
            assert_eq!(domain.base_type().type_name(), row.get::<_, String>(2));
            assert_eq!(domain.type_modifier(), Some(row.get::<_, i32>(3)));
            assert_eq!(
                domain.array_dimensions(),
                Some(u32::try_from(row.get::<_, i32>(4)).unwrap())
            );
            assert_eq!(domain.not_null(), Some(row.get::<_, bool>(5)));
            assert_eq!(
                domain.default_expression(),
                row.get::<_, Option<String>>(6).as_deref()
            );
            assert_eq!(
                domain.collation().map(|collation| (
                    collation.schema_name().to_owned(),
                    collation.collation_name().to_owned(),
                )),
                row.get::<_, Option<String>>(7)
                    .zip(row.get::<_, Option<String>>(8))
            );
            assert_eq!(
                domain.source_comment(),
                row.get::<_, Option<String>>(9).as_deref()
            );
        }
        let mut catalog_domain_checks = BTreeMap::new();
        for row in client
            .query(
                "SELECT d.typname::text, con.conname::text, \
                 pg_catalog.pg_get_constraintdef(con.oid, false), \
                 con.convalidated, con.conenforced \
                 FROM pg_catalog.pg_constraint con \
                 JOIN pg_catalog.pg_type d ON d.oid = con.contypid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = d.typnamespace \
                 WHERE n.nspname = $1 AND d.typtype = 'd' AND con.contype = 'c'",
                &[&schema],
            )
            .await
            .unwrap()
        {
            let key = (row.get::<_, String>(0), row.get::<_, String>(1));
            let value = (
                row.get::<_, String>(2),
                row.get::<_, bool>(3),
                row.get::<_, bool>(4),
            );
            assert!(catalog_domain_checks.insert(key, value).is_none());
        }
        let mut observed_domain_checks = BTreeMap::new();
        for domain in first.domains() {
            for check in domain.check_constraints() {
                let key = (
                    domain.domain_name().to_owned(),
                    check.constraint_name().to_owned(),
                );
                let value = (
                    check.check_definition().to_owned(),
                    check.validated(),
                    check.enforced(),
                );
                assert!(observed_domain_checks.insert(key, value).is_none());
            }
        }
        assert_eq!(observed_domain_checks, catalog_domain_checks);
        let mut catalog_enum_labels = BTreeMap::<String, Vec<String>>::new();
        for row in client
            .query(
                "SELECT t.typname::text, e.enumlabel::text FROM pg_catalog.pg_enum e \
                 JOIN pg_catalog.pg_type t ON t.oid = e.enumtypid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace \
                 WHERE n.nspname = $1 ORDER BY t.typname, e.enumsortorder, e.oid",
                &[&schema],
            )
            .await
            .unwrap()
        {
            catalog_enum_labels
                .entry(row.get(0))
                .or_default()
                .push(row.get(1));
        }
        let observed_enum_labels = first
            .enums()
            .iter()
            .map(|enumeration| {
                (
                    enumeration.enum_name().to_owned(),
                    enumeration.labels().to_vec(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        assert_eq!(observed_enum_labels, catalog_enum_labels);
        let catalog_type_kinds = client
            .query(
                "SELECT t.typname::text, t.typtype::text FROM pg_catalog.pg_type t \
                 JOIN pg_catalog.pg_namespace n ON n.oid = t.typnamespace WHERE n.nspname = $1",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
            .collect::<BTreeSet<_>>();
        let observed_type_kinds = first
            .type_kinds()
            .unwrap()
            .iter()
            .map(|observed| {
                let kind = match observed.kind() {
                    PostgresTypeKind::Base => "b",
                    PostgresTypeKind::Composite => "c",
                    PostgresTypeKind::Domain => "d",
                    PostgresTypeKind::Enum => "e",
                    PostgresTypeKind::Pseudo => "p",
                    PostgresTypeKind::Range => "r",
                    PostgresTypeKind::Multirange => "m",
                };
                (observed.type_name().type_name().to_owned(), kind.to_owned())
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_type_kinds, catalog_type_kinds);
        let catalog_array_types = client
            .query(
                "SELECT a.typname::text, e.typname::text FROM pg_catalog.pg_type e \
                 JOIN pg_catalog.pg_type a ON a.oid = e.typarray \
                 JOIN pg_catalog.pg_namespace n ON n.oid = e.typnamespace \
                 WHERE n.nspname = $1 AND e.typarray <> 0 AND a.typelem = e.oid",
                &[&schema],
            )
            .await
            .unwrap()
            .into_iter()
            .map(|row| (row.get::<_, String>(0), row.get::<_, String>(1)))
            .collect::<BTreeSet<_>>();
        let observed_array_types = first
            .array_types()
            .unwrap()
            .iter()
            .map(|pair| {
                (
                    pair.array_type().type_name().to_owned(),
                    pair.element_type().type_name().to_owned(),
                )
            })
            .collect::<BTreeSet<_>>();
        assert_eq!(observed_array_types, catalog_array_types);
        let proposal = propose_relational_model(&first).unwrap();
        let replay_proposal = propose_relational_model(&replay).unwrap();
        assert_eq!(proposal, replay_proposal);
        assert_eq!(proposal.source_digest(), first.snapshot_digest());
        assert_eq!(proposal.concepts().len(), 4);
        assert_eq!(proposal.relations().len(), 4);
        assert_eq!(proposal.source_types().len(), 2);
        let field_count = proposal
            .concepts()
            .iter()
            .map(|concept| concept.fields().len())
            .sum::<usize>();
        assert_eq!(field_count, 13);
        let decisions = proposal
            .concepts()
            .iter()
            .enumerate()
            .map(|(index, concept)| {
                AlignmentDecision::map(
                    concept.candidate().candidate_id(),
                    format!("fixture.concept.{index}"),
                    format!("Fixture concept {index}"),
                    "Explicit fixture glossary mapping",
                )
                .unwrap()
            })
            .chain(
                proposal
                    .concepts()
                    .iter()
                    .flat_map(|concept| concept.fields())
                    .enumerate()
                    .map(|(index, field)| {
                        AlignmentDecision::map(
                            field.candidate().candidate_id(),
                            format!("fixture.field.{index}"),
                            format!("Fixture field {index}"),
                            "Explicit fixture property mapping",
                        )
                        .unwrap()
                    }),
            )
            .chain(
                proposal
                    .relations()
                    .iter()
                    .enumerate()
                    .map(|(index, relation)| {
                        AlignmentDecision::map(
                            relation.candidate().candidate_id(),
                            format!("fixture.relation.{index}"),
                            format!("Fixture relation {index}"),
                            "Explicit fixture relationship mapping",
                        )
                        .unwrap()
                    }),
            )
            .chain(
                proposal
                    .source_types()
                    .iter()
                    .enumerate()
                    .map(|(index, source_type)| {
                        AlignmentDecision::map(
                            source_type.candidate().candidate_id(),
                            format!("fixture.type.{index}"),
                            format!("Fixture type {index}"),
                            "Explicit fixture type mapping",
                        )
                        .unwrap()
                    }),
            )
            .collect::<Vec<_>>();
        let aligned =
            align_relational_proposal(&proposal, proposal.proposal_id(), decisions.clone())
                .unwrap();
        assert_eq!(
            aligned,
            align_relational_proposal(
                &replay_proposal,
                replay_proposal.proposal_id(),
                decisions.clone(),
            )
            .unwrap()
        );
        let validated = validate_alignment(&aligned).unwrap();
        assert_eq!(validated.candidates().len(), 10 + field_count);
        assert_eq!(
            validated.validation().source_bound_candidates(),
            10 + field_count
        );
        assert_eq!(
            validated.validation().unique_semantic_ids(),
            10 + field_count
        );
        assert_eq!(validated.validation().mapped_relations_with_endpoints(), 4);
        assert_eq!(
            validated.validation().mapped_fields_with_concepts(),
            field_count
        );
        assert!(validated.candidates().iter().all(|candidate| {
            candidate.candidate().publication_state() == PublicationState::Validated
                && candidate.candidate().truth_status() == TruthStatus::Inferred
                && candidate.candidate().evidence()[0].source_digest() == first.snapshot_digest()
        }));
        assert_eq!(
            review(
                &validated,
                "fixture-steward",
                "Review GRC mapping",
                &DeniedSteward
            )
            .unwrap_err(),
            GovernanceError::ReviewDenied
        );
        let authority = FixtureSteward {
            proposal_id: proposal.proposal_id(),
            source_digest: first.snapshot_digest(),
        };
        let reviewed = review(
            &validated,
            "fixture-steward",
            "Review GRC mapping",
            &authority,
        )
        .unwrap();
        let replay_aligned = align_relational_proposal(
            &replay_proposal,
            replay_proposal.proposal_id(),
            decisions.clone(),
        )
        .unwrap();
        let replay_reviewed = review(
            &validate_alignment(&replay_aligned).unwrap(),
            "fixture-steward",
            "Review GRC mapping",
            &authority,
        )
        .unwrap();
        let metadata =
            || ReleaseMetadata::new("fixture-release", "1.0.0", "fixture-ontology").unwrap();
        let publication_root = std::env::temp_dir().join(format!(
            "cw-publish-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&publication_root).unwrap();
        let replay_root = publication_root.join("replay");
        std::fs::create_dir(&replay_root).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&publication_root, std::fs::Permissions::from_mode(0o755))
                .unwrap();
            assert!(matches!(
                FilePublicationStore::new(&publication_root),
                Err(PublicationStoreError::InvalidRoot)
            ));
        }
        #[cfg(unix)]
        for root in [&publication_root, &replay_root] {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(root, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let store = FilePublicationStore::new(&publication_root).unwrap();
        let replay_store = FilePublicationStore::new(&replay_root).unwrap();
        let published = store.publish(&reviewed, metadata()).unwrap();
        let replay_published = replay_store.publish(&replay_reviewed, metadata()).unwrap();
        assert!(matches!(
            store.publish(&reviewed, metadata()),
            Err(PublicationStoreError::DuplicateReleaseId)
        ));
        let conflicting_review = review(
            &validated,
            "fixture-steward",
            "Different review reason",
            &authority,
        )
        .unwrap();
        assert!(matches!(
            store.publish(&conflicting_review, metadata()),
            Err(PublicationStoreError::DuplicateReleaseId)
        ));
        let race_root = publication_root.join("race");
        std::fs::create_dir(&race_root).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&race_root, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let race_store = FilePublicationStore::new(&race_root).unwrap();
        let race_results = std::thread::scope(|scope| {
            let first = scope.spawn(|| race_store.publish(&reviewed, metadata()));
            let second = scope.spawn(|| race_store.publish(&conflicting_review, metadata()));
            [first.join().unwrap(), second.join().unwrap()]
        });
        assert_eq!(
            race_results.iter().filter(|result| result.is_ok()).count(),
            1
        );
        assert_eq!(
            race_results
                .iter()
                .filter(|result| matches!(result, Err(PublicationStoreError::DuplicateReleaseId)))
                .count(),
            1
        );
        assert_eq!(
            published.artifact_bytes(),
            replay_published.artifact_bytes()
        );
        assert_eq!(published.release(), replay_published.release());
        assert_eq!(published.manifest_pin(), replay_published.manifest_pin());
        let mut artifact = published.artifact_bytes();
        assert_eq!(
            take_text(&mut artifact),
            "conceptweave.governed_semantic_artifact.v2"
        );
        for _ in 0..3 {
            take_text(&mut artifact);
        }
        let mut encoded_alignment = take_blob(&mut artifact);
        assert_eq!(
            take_text(&mut encoded_alignment),
            "conceptweave.validated_alignment.v2"
        );
        assert_eq!(take_text(&mut encoded_alignment), proposal.proposal_id());
        assert_eq!(take_text(&mut encoded_alignment), first.snapshot_digest());
        assert_eq!(take_u64(&mut encoded_alignment), 23);
        for _ in 0..23 {
            take_text(&mut encoded_alignment);
            let decision = encoded_alignment[1];
            encoded_alignment = &encoded_alignment[2..];
            for _ in 0..if decision == 1 { 3 } else { 1 } {
                take_text(&mut encoded_alignment);
            }
            let evidence_count = take_u64(&mut encoded_alignment);
            for _ in 0..evidence_count * 3 {
                take_text(&mut encoded_alignment);
            }
        }
        for _ in 0..4 {
            take_u64(&mut encoded_alignment);
        }
        let mut encoded_fields = BTreeMap::new();
        for _ in 0..take_u64(&mut encoded_alignment) {
            let field_id = take_text(&mut encoded_alignment).to_owned();
            let concept_id = take_text(&mut encoded_alignment).to_owned();
            encoded_fields.insert(field_id, concept_id);
        }
        assert_eq!(&encoded_fields, validated.field_parents());
        assert_eq!(encoded_fields.len(), 13);
        let mut encoded_relations = BTreeMap::new();
        for _ in 0..take_u64(&mut encoded_alignment) {
            let relation_id = take_text(&mut encoded_alignment).to_owned();
            let from_id = take_text(&mut encoded_alignment).to_owned();
            let to_id = take_text(&mut encoded_alignment).to_owned();
            encoded_relations.insert(relation_id, (from_id, to_id));
        }
        assert_eq!(&encoded_relations, validated.relation_endpoints());
        assert_eq!(encoded_relations.len(), 4);
        assert_eq!(validated.proposal(), &proposal);
        assert_eq!(
            take_text(&mut encoded_alignment),
            "conceptweave.relational_source_details.v1"
        );
        let mut concepts = proposal.concepts().iter().collect::<Vec<_>>();
        concepts.sort_by_key(|concept| concept.candidate().candidate_id());
        assert_eq!(take_u64(&mut encoded_alignment), concepts.len());
        for concept in concepts {
            assert_eq!(
                take_text(&mut encoded_alignment),
                concept.candidate().candidate_id()
            );
            assert_eq!(take_text(&mut encoded_alignment), concept.source_schema());
            assert_eq!(take_text(&mut encoded_alignment), concept.source_relation());
            assert_eq!(
                take_optional_text(&mut encoded_alignment),
                concept.source_comment()
            );
            assert_eq!(
                take_optional_strings(&mut encoded_alignment),
                concept
                    .primary_key_columns()
                    .map(|values| values.iter().map(String::as_str).collect())
            );
            assert_eq!(take_u64(&mut encoded_alignment), concept.fields().len());
            for field in concept.fields() {
                assert_eq!(
                    take_text(&mut encoded_alignment),
                    field.candidate().candidate_id()
                );
                assert_eq!(take_text(&mut encoded_alignment), field.source_name());
                assert_eq!(take_text(&mut encoded_alignment), field.display_type());
                assert_eq!(
                    take_text(&mut encoded_alignment),
                    field.type_binding().schema_name()
                );
                assert_eq!(
                    take_text(&mut encoded_alignment),
                    field.type_binding().type_name()
                );
                assert_eq!(
                    take_u64(&mut encoded_alignment),
                    field.ordinal_position() as usize
                );
                assert_eq!(
                    take_byte(&mut encoded_alignment),
                    u8::from(field.nullable())
                );
                assert_eq!(
                    take_optional_text(&mut encoded_alignment),
                    field.source_comment()
                );
            }
        }
        let mut relations = proposal.relations().iter().collect::<Vec<_>>();
        relations.sort_by_key(|relation| relation.candidate().candidate_id());
        assert_eq!(take_u64(&mut encoded_alignment), relations.len());
        for relation in relations {
            assert_eq!(
                take_text(&mut encoded_alignment),
                relation.candidate().candidate_id()
            );
            assert_eq!(
                take_text(&mut encoded_alignment),
                relation.from_concept_id()
            );
            assert_eq!(take_text(&mut encoded_alignment), relation.to_concept_id());
            assert_eq!(
                take_text(&mut encoded_alignment),
                relation.source_constraint()
            );
            assert_eq!(
                take_u64(&mut encoded_alignment),
                relation.column_pairs().len()
            );
            for (local, referenced) in relation.column_pairs() {
                assert_eq!(take_text(&mut encoded_alignment), local);
                assert_eq!(take_text(&mut encoded_alignment), referenced);
            }
            let mut behavior = [0u8; 6];
            for item in &mut behavior {
                *item = take_byte(&mut encoded_alignment);
            }
            assert_eq!(behavior, [0, 0, 0, 0, 1, 1]);
            assert_eq!(take_optional_strings(&mut encoded_alignment), None);
        }
        let mut source_types = proposal.source_types().iter().collect::<Vec<_>>();
        source_types.sort_by_key(|item| item.candidate().candidate_id());
        assert_eq!(take_u64(&mut encoded_alignment), source_types.len());
        for source_type in source_types {
            match source_type {
                ProposedSourceType::Domain {
                    candidate,
                    observation,
                } => {
                    assert_eq!(take_byte(&mut encoded_alignment), 0);
                    assert_eq!(take_text(&mut encoded_alignment), candidate.candidate_id());
                    assert_eq!(take_text(&mut encoded_alignment), observation.schema_name());
                    assert_eq!(take_text(&mut encoded_alignment), observation.domain_name());
                    assert_eq!(
                        take_text(&mut encoded_alignment),
                        observation.base_type().schema_name()
                    );
                    assert_eq!(
                        take_text(&mut encoded_alignment),
                        observation.base_type().type_name()
                    );
                    assert_eq!(
                        take_optional_i32(&mut encoded_alignment),
                        observation.type_modifier()
                    );
                    assert_eq!(
                        take_optional_u32(&mut encoded_alignment),
                        observation.array_dimensions()
                    );
                    if let Some(collation) = observation.collation() {
                        assert_eq!(take_byte(&mut encoded_alignment), 1);
                        assert_eq!(take_text(&mut encoded_alignment), collation.schema_name());
                        assert_eq!(
                            take_text(&mut encoded_alignment),
                            collation.collation_name()
                        );
                    } else {
                        assert_eq!(take_byte(&mut encoded_alignment), 0);
                    }
                    let not_null = (take_byte(&mut encoded_alignment) == 1)
                        .then(|| take_byte(&mut encoded_alignment) == 1);
                    assert_eq!(not_null, observation.not_null());
                    assert_eq!(
                        take_optional_text(&mut encoded_alignment),
                        observation.default_expression()
                    );
                    assert_eq!(
                        take_u64(&mut encoded_alignment),
                        observation.check_constraints().len()
                    );
                    for check in observation.check_constraints() {
                        assert_eq!(take_text(&mut encoded_alignment), check.constraint_name());
                        assert_eq!(take_text(&mut encoded_alignment), check.check_definition());
                        assert_eq!(
                            take_byte(&mut encoded_alignment),
                            u8::from(check.validated())
                        );
                        assert_eq!(
                            take_byte(&mut encoded_alignment),
                            u8::from(check.enforced())
                        );
                    }
                    assert_eq!(
                        take_optional_text(&mut encoded_alignment),
                        observation.source_comment()
                    );
                }
                ProposedSourceType::Enum {
                    candidate,
                    observation,
                } => {
                    assert_eq!(take_byte(&mut encoded_alignment), 1);
                    assert_eq!(take_text(&mut encoded_alignment), candidate.candidate_id());
                    assert_eq!(take_text(&mut encoded_alignment), observation.schema_name());
                    assert_eq!(take_text(&mut encoded_alignment), observation.enum_name());
                    assert_eq!(take_u64(&mut encoded_alignment), observation.labels().len());
                    for label in observation.labels() {
                        assert_eq!(take_text(&mut encoded_alignment), label);
                    }
                    assert_eq!(
                        take_optional_text(&mut encoded_alignment),
                        observation.source_comment()
                    );
                }
                ProposedSourceType::Composite { .. } => {
                    panic!("this fixture has no composite source type")
                }
            }
        }
        assert!(encoded_alignment.is_empty());
        assert_eq!(published.release().concept_ids().len(), 4);
        assert_eq!(
            published.release().publication_state(),
            PublicationState::Published
        );
        assert_eq!(
            published.release().truth_status(),
            TruthStatus::Authoritative
        );
        assert!(
            published
                .release()
                .provenance()
                .iter()
                .all(|item| item.source_digest() == first.snapshot_digest())
        );
        let unpinned = SemanticReleaseClient::new("1.0.0").unwrap();
        assert!(
            unpinned
                .validate_for_authoritative_use(published.release())
                .is_err()
        );
        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let signing_key = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        let key = TrustedPublisherKey::new("fixture-publisher", signing_key.public_key().as_ref())
            .unwrap();
        let signed =
            sign_published_manifest(&published, "fixture-publisher", &signing_key).unwrap();
        let pinned = SemanticReleaseClient::with_signed_release_manifests(
            "1.0.0",
            vec![],
            &[key],
            &[signed],
        )
        .unwrap();
        pinned
            .validate_for_authoritative_use(published.release())
            .unwrap();
        pinned
            .verify_detached_artifact(published.release(), published.artifact_bytes())
            .unwrap();
        assert_eq!(
            store
                .read_verified(&pinned, published.release())
                .unwrap()
                .unwrap(),
            published.artifact_bytes()
        );
        assert!(matches!(
            store.read_verified(&unpinned, published.release()),
            Err(PublicationStoreError::ReleaseNotAdmitted)
        ));
        assert_eq!(
            pinned
                .resolve_concept(published.release(), "fixture.concept.0")
                .unwrap(),
            Some("fixture.concept.0")
        );
        let mut tampered = published.artifact_bytes().to_vec();
        tampered.push(0);
        assert!(
            pinned
                .verify_detached_artifact(published.release(), &tampered)
                .is_err()
        );
        let record_path = std::fs::read_dir(&publication_root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "release")
            })
            .unwrap();
        let mut record = std::fs::read(&record_path).unwrap();
        *record.last_mut().unwrap() ^= 1;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&record_path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        std::fs::write(&record_path, record).unwrap();
        assert!(matches!(
            store.read_verified(&pinned, published.release()),
            Err(PublicationStoreError::InvalidRecord)
        ));
        std::fs::remove_dir_all(&publication_root).unwrap();
        let excluded_concept_id = proposal.concepts()[0].candidate().candidate_id();
        let excluded_parent = decisions
            .clone()
            .into_iter()
            .map(|decision| {
                if decision.candidate_id() == excluded_concept_id {
                    AlignmentDecision::exclude(excluded_concept_id, "Fixture endpoint excluded")
                        .unwrap()
                } else {
                    decision
                }
            })
            .collect();
        let invalid =
            align_relational_proposal(&proposal, proposal.proposal_id(), excluded_parent).unwrap();
        assert_eq!(
            validate_alignment(&invalid).unwrap_err(),
            AlignmentError::ExcludedFieldParent
        );
        let excluded = decisions
            .into_iter()
            .map(|decision| {
                if decision.candidate_id() == excluded_concept_id
                    || proposal.concepts()[0]
                        .fields()
                        .iter()
                        .any(|field| field.candidate().candidate_id() == decision.candidate_id())
                {
                    AlignmentDecision::exclude(decision.candidate_id(), "Fixture parent excluded")
                        .unwrap()
                } else {
                    decision
                }
            })
            .collect();
        let invalid =
            align_relational_proposal(&proposal, proposal.proposal_id(), excluded).unwrap();
        assert_eq!(
            validate_alignment(&invalid).unwrap_err(),
            AlignmentError::ExcludedRelationEndpoint
        );
        assert!(proposal.source_types().iter().any(|source_type| matches!(
            source_type,
            ProposedSourceType::Domain { candidate, observation }
                if candidate.kind() == CandidateKind::Constraint
                    && observation.domain_name() == "impact_score"
                    && candidate.evidence()[0].source_digest() == first.snapshot_digest()
        )));
        assert!(proposal.source_types().iter().any(|source_type| matches!(
            source_type,
            ProposedSourceType::Enum { candidate, observation }
                if candidate.kind() == CandidateKind::Dimension
                    && observation.enum_name() == "risk_level"
                    && observation.labels() == ["low", "moderate", "high"]
                    && candidate.evidence()[0].source_digest() == first.snapshot_digest()
        )));
        for concept in proposal.concepts() {
            assert_eq!(
                concept.candidate().publication_state(),
                PublicationState::Draft
            );
            assert_eq!(concept.candidate().truth_status(), TruthStatus::Inferred);
            assert_eq!(
                concept.candidate().evidence()[0].source_digest(),
                first.snapshot_digest()
            );
            for field in concept.fields() {
                assert_eq!(field.evidence().source_digest(), first.snapshot_digest());
                assert_eq!(field.candidate().kind(), CandidateKind::PhysicalMapping);
                assert_eq!(
                    field.candidate().publication_state(),
                    PublicationState::Draft
                );
                assert_eq!(field.candidate().evidence(), [field.evidence().clone()]);
            }
            assert!(concept.primary_key_columns().is_some());
        }
        let tenant_id_candidates = proposal
            .concepts()
            .iter()
            .flat_map(|concept| concept.fields())
            .filter(|field| field.source_name() == "tenant_id")
            .map(|field| field.candidate().candidate_id())
            .collect::<BTreeSet<_>>();
        assert_eq!(tenant_id_candidates.len(), 4);
        let risk = proposal
            .concepts()
            .iter()
            .find(|concept| concept.source_relation() == "risk_record")
            .unwrap();
        assert_eq!(risk.source_comment(), Some("Anonymized risk catalog shape"));
        assert_eq!(
            risk.fields()
                .iter()
                .find(|field| field.source_name() == "title")
                .unwrap()
                .source_comment(),
            Some("Reviewable risk title")
        );
        for relationship in proposal.relations() {
            assert!(proposal.concepts().iter().any(
                |concept| concept.candidate().candidate_id() == relationship.from_concept_id()
            ));
            assert!(
                proposal
                    .concepts()
                    .iter()
                    .any(|concept| concept.candidate().candidate_id()
                        == relationship.to_concept_id())
            );
            assert!(relationship.validated());
            assert!(relationship.enforced());
            assert_eq!(
                relationship.reference_behavior().delete_action(),
                ForeignKeyAction::NoAction
            );
            assert_eq!(
                relationship.candidate().evidence()[0].source_digest(),
                first.snapshot_digest()
            );
        }
        let receipt = first
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "risk_control_link",
                    RelationKind::Table,
                    "risk_control_risk_fk",
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), first.snapshot_digest());
        client
            .batch_execute(&format!(
                "COMMENT ON COLUMN \"{schema}\".risk_record.title IS 'Revised review title'"
            ))
            .await
            .unwrap();
        let changed = observe().await?;
        assert_ne!(first.snapshot_digest(), changed.snapshot_digest());
        let changed_proposal = propose_relational_model(&changed).unwrap();
        assert_ne!(proposal.proposal_id(), changed_proposal.proposal_id());
        let changed_title = changed_proposal
            .concepts()
            .iter()
            .find(|concept| concept.source_relation() == "risk_record")
            .unwrap()
            .fields()
            .iter()
            .find(|field| field.source_name() == "title")
            .unwrap();
        let original_title = risk
            .fields()
            .iter()
            .find(|field| field.source_name() == "title")
            .unwrap();
        assert_eq!(
            original_title.candidate().candidate_id(),
            changed_title.candidate().candidate_id()
        );
        assert_ne!(
            original_title.evidence().source_digest(),
            changed_title.evidence().source_digest()
        );
        assert_eq!(changed_title.source_comment(), Some("Revised review title"));
        client
            .batch_execute(&format!(
                "COMMENT ON COLUMN \"{schema}\".risk_record.title IS 'Reviewable risk title'"
            ))
            .await
            .unwrap();
        let restored = observe().await?;
        assert_eq!(first.snapshot_digest(), restored.snapshot_digest());
        assert_eq!(
            proposal.proposal_id(),
            propose_relational_model(&restored).unwrap().proposal_id()
        );
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_foreign_key_preserves_comparison_and_referential_evidence() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_fk_fixture_{}", std::process::id());
    let external_schema = format!("cw_fk_external_fixture_{}", std::process::id());
    client.batch_execute(&format!(
        "CREATE SCHEMA \"{schema}\"; \
         CREATE TABLE \"{schema}\".parent (a integer, b integer, CONSTRAINT parent_key UNIQUE (a, b)); \
         CREATE TABLE \"{schema}\".child (x integer, y integer, \
           CONSTRAINT child_parent_fk FOREIGN KEY (x, y) REFERENCES \"{schema}\".parent (b, a) \
           ON UPDATE CASCADE ON DELETE SET NULL (x) DEFERRABLE INITIALLY DEFERRED)"
    )).await.unwrap();

    let result = async {
        let snapshot = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let child = snapshot
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let TableConstraintObservation::ForeignKey(foreign_key) = child
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == "child_parent_fk")
            .unwrap()
        else {
            panic!("captured constraint must be a foreign key");
        };
        assert_eq!(foreign_key.column_names(), &["x", "y"]);
        assert_eq!(foreign_key.referenced_column_names(), &["b", "a"]);
        let behavior = foreign_key.reference_behavior().unwrap();
        assert_eq!(behavior.update_action(), ForeignKeyAction::Cascade);
        assert_eq!(behavior.delete_action(), ForeignKeyAction::SetNull);
        assert_eq!(
            behavior.delete_target_columns(),
            Some(["x".to_owned()].as_slice())
        );
        assert_eq!(
            behavior.deferrability(),
            ForeignKeyDeferrability::InitiallyDeferred
        );
        let catalog = snapshot.foreign_key_catalog().unwrap();
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog[0].referenced_index_name(), "parent_key");
        assert_eq!(catalog[0].primary_foreign_operators().len(), 2);
        let receipt = snapshot
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "child",
                    RelationKind::Table,
                    "child_parent_fk",
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
        let replay = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_eq!(snapshot.snapshot_digest(), replay.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child DROP CONSTRAINT child_parent_fk; \
             ALTER TABLE \"{schema}\".child ADD CONSTRAINT child_parent_fk \
             FOREIGN KEY (x, y) REFERENCES \"{schema}\".parent (b, a) \
             DEFERRABLE INITIALLY IMMEDIATE"
            ))
            .await
            .unwrap();
        let changed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(snapshot.snapshot_digest(), changed.snapshot_digest());
        let changed_child = changed
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let TableConstraintObservation::ForeignKey(changed_key) = changed_child
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == "child_parent_fk")
            .unwrap()
        else {
            panic!("captured constraint must be a foreign key");
        };
        assert_eq!(
            changed_key.reference_behavior().unwrap().update_action(),
            ForeignKeyAction::NoAction
        );
        assert_eq!(
            changed_key.reference_behavior().unwrap().deferrability(),
            ForeignKeyDeferrability::InitiallyImmediate
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child DROP CONSTRAINT child_parent_fk; \
                 ALTER TABLE \"{schema}\".child ADD CONSTRAINT child_parent_fk \
                 FOREIGN KEY (x, y) REFERENCES \"{schema}\".parent (b, a) MATCH FULL NOT VALID"
            ))
            .await
            .unwrap();
        let not_valid = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let not_valid_child = not_valid
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let TableConstraintObservation::ForeignKey(not_valid_key) = not_valid_child
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == "child_parent_fk")
            .unwrap()
        else {
            panic!("captured constraint must be a foreign key");
        };
        assert_eq!(not_valid_key.validated(), Some(false));
        assert_eq!(
            not_valid_key.reference_behavior().unwrap().match_type(),
            ForeignKeyMatchType::Full
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child VALIDATE CONSTRAINT child_parent_fk"
            ))
            .await
            .unwrap();
        let valid = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(not_valid.snapshot_digest(), valid.snapshot_digest());
        client
            .batch_execute(&format!(
                "CREATE SCHEMA \"{external_schema}\"; \
             CREATE TABLE \"{external_schema}\".other_parent (id integer PRIMARY KEY); \
             ALTER TABLE \"{schema}\".child ADD COLUMN outside_id integer; \
             ALTER TABLE \"{schema}\".child ADD CONSTRAINT outside_fk \
             FOREIGN KEY (outside_id) REFERENCES \"{external_schema}\".other_parent (id)"
            ))
            .await
            .unwrap();
        assert_eq!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::InvalidCapturedMetadata)
        );
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!(
            "DROP SCHEMA IF EXISTS \"{external_schema}\" CASCADE; DROP SCHEMA \"{schema}\" CASCADE"
        ))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn missing_binding_and_tcp_transport_fail_before_source_io() {
    let unbound_adapter = PostgresUnixAdapter::new(BTreeMap::new());
    assert_eq!(
        unbound_adapter
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let tcp = Config::from_str("host=127.0.0.1 dbname=postgres").unwrap();
    assert_eq!(
        adapter(tcp)
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let stale = PostgresUnixAdapter::new(BTreeMap::from([(
        "fixture_source".to_owned(),
        (
            "new_policy".to_owned(),
            Config::from_str("host=/tmp dbname=postgres").unwrap(),
        ),
    )]));
    assert_eq!(
        stale
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let unix = Config::from_str("host=/tmp dbname=postgres").unwrap();
    assert_eq!(
        adapter(unix)
            .observe(authorized("public"), &Cancelled)
            .await
            .err(),
        Some(SourceObservationFailure::Cancelled)
    );
}

#[tokio::test]
async fn postgres18_unenforced_foreign_key_retains_false_state_without_ri_triggers() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_unenforced_fk_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TABLE \"{schema}\".parent (id integer PRIMARY KEY); \
             CREATE TABLE \"{schema}\".child (guarded_id integer, advisory_id integer, \
               CONSTRAINT guarded_fk FOREIGN KEY (guarded_id) REFERENCES \"{schema}\".parent(id), \
               CONSTRAINT advisory_fk FOREIGN KEY (advisory_id) REFERENCES \"{schema}\".parent(id) NOT ENFORCED)"
        ))
        .await
        .unwrap();

    let result = async {
        let observed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let child = observed
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let foreign_key = |name| {
            let TableConstraintObservation::ForeignKey(key) = child
                .constraints()
                .iter()
                .find(|constraint| constraint.constraint_name() == name)
                .unwrap()
            else {
                panic!("observed constraint must be a foreign key");
            };
            key
        };
        assert_eq!(foreign_key("advisory_fk").validated(), Some(false));
        assert_eq!(foreign_key("advisory_fk").enforced(), Some(false));
        assert_eq!(foreign_key("guarded_fk").validated(), Some(true));
        assert_eq!(foreign_key("guarded_fk").enforced(), Some(true));
        observed
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "child",
                    RelationKind::Table,
                    "advisory_fk",
                )
                .unwrap(),
            )
            .unwrap();
        let proposal = propose_relational_model(&observed).unwrap();
        let advisory = proposal
            .relations()
            .iter()
            .find(|relation| relation.source_constraint() == "advisory_fk")
            .unwrap();
        assert!(!advisory.validated());
        assert!(!advisory.enforced());
        assert_eq!(advisory.candidate().truth_status(), TruthStatus::Inferred);
        assert_eq!(
            advisory.candidate().publication_state(),
            PublicationState::Draft
        );
        let trigger_count: i64 = client
            .query_one(
                "SELECT count(*) FROM pg_catalog.pg_trigger t \
                 JOIN pg_catalog.pg_constraint c ON c.oid = t.tgconstraint \
                 WHERE c.conname = 'advisory_fk' AND c.connamespace = to_regnamespace($1)",
                &[&schema],
            )
            .await
            .unwrap()
            .get(0);
        assert_eq!(trigger_count, 0);
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child DROP CONSTRAINT advisory_fk; \
                 ALTER TABLE \"{schema}\".child ADD CONSTRAINT advisory_fk \
                 FOREIGN KEY (advisory_id) REFERENCES \"{schema}\".parent(id)"
            ))
            .await
            .unwrap();
        let enforced = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(observed.snapshot_digest(), enforced.snapshot_digest());
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_unenforced_check_preserves_catalog_state_and_receipt() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_unenforced_check_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TABLE \"{schema}\".item (score integer, \
               CONSTRAINT score_positive CHECK (score > 0) NOT ENFORCED)"
        ))
        .await
        .unwrap();

    let result = async {
        let observed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 64, 65_536), &NotCancelled)
            .await?;
        let catalog = client
            .query_one(
                "SELECT c.convalidated, c.conenforced, pg_catalog.pg_get_constraintdef(c.oid) \
                 FROM pg_catalog.pg_constraint c \
                 JOIN pg_catalog.pg_namespace n ON n.oid = c.connamespace \
                 WHERE n.nspname = $1 AND c.conname = 'score_positive'",
                &[&schema],
            )
            .await
            .unwrap();
        let TableConstraintObservation::Check(check) = &observed.relations()[0].constraints()[0]
        else {
            panic!("observed constraint must be CHECK");
        };
        assert_eq!(check.validated(), catalog.get::<_, bool>(0));
        assert_eq!(check.enforced(), catalog.get::<_, bool>(1));
        assert!(!check.validated());
        assert!(!check.enforced());
        assert_eq!(check.definition(), catalog.get::<_, String>(2));
        observed
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "score_positive",
                )
                .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT score_positive; \
                 ALTER TABLE \"{schema}\".item ADD CONSTRAINT score_positive CHECK (score > 0)"
            ))
            .await
            .unwrap();
        let enforced = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 64, 65_536), &NotCancelled)
            .await?;
        assert_ne!(observed.snapshot_digest(), enforced.snapshot_digest());
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_tcp_requires_valid_ca_and_host_name() {
    let (Ok(dsn), Ok(ca_path), Ok(wrong_ca_path), Ok(plaintext_dsn)) = (
        std::env::var("CONCEPTWEAVE_PG18_TLS_TEST_DSN"),
        std::env::var("CONCEPTWEAVE_PG18_TLS_CA_DER"),
        std::env::var("CONCEPTWEAVE_PG18_TLS_WRONG_CA_DER"),
        std::env::var("CONCEPTWEAVE_PG18_PLAINTEXT_TEST_DSN"),
    ) else {
        return;
    };
    assert!(dsn.contains("host=localhost"));
    let config = Config::from_str(&dsn).unwrap();
    let ca = std::fs::read(ca_path).unwrap();
    let wrong_ca = std::fs::read(wrong_ca_path).unwrap();
    let connection = |config: Config, ca: Vec<u8>| {
        PostgresTlsAdapter::new(BTreeMap::from([(
            "fixture_source".to_owned(),
            ("fixture_policy".to_owned(), config, ca),
        )]))
    };
    assert!(connection(config.clone(), b"invalid certificate".to_vec()).is_err());
    assert!(
        connection(
            Config::from_str("host=/tmp dbname=postgres").unwrap(),
            ca.clone()
        )
        .is_err()
    );

    assert_eq!(
        connection(config.clone(), wrong_ca)
            .unwrap()
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );
    assert_eq!(
        connection(Config::from_str(&plaintext_dsn).unwrap(), ca.clone())
            .unwrap()
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let adapter = connection(config.clone(), ca.clone()).unwrap();
    let snapshot = adapter
        .observe(authorized("public"), &NotCancelled)
        .await
        .unwrap();
    assert!(snapshot.relations().is_empty());

    let wrong_host = Config::from_str(&dsn.replace("host=localhost", "host=127.0.0.1")).unwrap();
    assert_eq!(
        connection(wrong_host, ca)
            .unwrap()
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );
}

#[tokio::test]
async fn postgres18_column_collation_is_exact_source_evidence() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_collation_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE COLLATION \"{schema}\".casefold (provider = icu, locale = 'und-u-ks-level1', deterministic = false); \
             CREATE TABLE \"{schema}\".records (id integer, title text, alias text COLLATE \"{schema}\".casefold)"
        ))
        .await
        .unwrap();

    let result = async {
        let before = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let title = before
            .column_collations()
            .unwrap()
            .iter()
            .find(|column| column.column_name() == "title")
            .unwrap();
        assert_eq!(title.collation().unwrap().schema_name(), "pg_catalog");
        assert_eq!(title.collation().unwrap().collation_name(), "default");
        assert_eq!(title.deterministic(), Some(true));
        let catalog_rows = client
            .query(
                "SELECT c.collencoding, c.collprovider::text, c.collisdeterministic, \
                 d.encoding, d.datlocprovider::text, d.datcollate, d.datctype, \
                 d.datlocale, d.daticurules, d.datcollversion, \
                 pg_catalog.pg_database_collation_actual_version(d.oid) \
                 FROM pg_catalog.pg_collation c \
                 JOIN pg_catalog.pg_namespace n ON n.oid = c.collnamespace \
                 JOIN pg_catalog.pg_database d ON d.datname = current_database() \
                 WHERE n.nspname = 'pg_catalog' AND c.collname = 'default'",
                &[],
            )
            .await
            .unwrap();
        let [catalog] = catalog_rows.as_slice() else {
            panic!("expected one pg_catalog.default collation row");
        };
        let definition = before
            .collation_definitions()
            .unwrap()
            .iter()
            .find(|item| {
                item.collation().schema_name() == "pg_catalog"
                    && item.collation().collation_name() == "default"
            })
            .unwrap();
        assert_eq!(definition.encoding(), catalog.get::<_, i32>(0));
        assert_eq!(definition.encoding(), -1);
        assert_eq!(definition.provider(), CollationProvider::DatabaseDefault);
        assert_eq!(catalog.get::<_, String>(1), "d");
        assert_eq!(definition.deterministic(), catalog.get::<_, bool>(2));
        assert_eq!(definition.database_encoding(), catalog.get::<_, i32>(3));
        let database_default = definition.database_default().unwrap();
        assert_eq!(
            database_default.provider(),
            CollationProvider::try_from(catalog.get::<_, String>(4).as_str()).unwrap()
        );
        let fields = database_default.fields();
        for (observed, index) in [
            (fields.lc_collate(), 5),
            (fields.lc_ctype(), 6),
            (fields.locale(), 7),
            (fields.icu_rules(), 8),
            (fields.recorded_version(), 9),
            (fields.actual_version(), 10),
        ] {
            assert_eq!(observed, catalog.get::<_, Option<String>>(index).as_deref());
        }
        let alias = before
            .column_collations()
            .unwrap()
            .iter()
            .find(|column| column.column_name() == "alias")
            .unwrap();
        assert_eq!(alias.collation().unwrap().schema_name(), schema);
        assert_eq!(alias.collation().unwrap().collation_name(), "casefold");
        assert_eq!(alias.deterministic(), Some(false));
        assert!(
            before
                .column_collations()
                .unwrap()
                .iter()
                .find(|column| column.column_name() == "id")
                .unwrap()
                .collation()
                .is_none()
        );

        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".records ALTER COLUMN title TYPE text COLLATE \"C\""
            ))
            .await
            .unwrap();
        let after = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(before.snapshot_digest(), after.snapshot_digest());
        let title = after
            .column_collations()
            .unwrap()
            .iter()
            .find(|column| column.column_name() == "title")
            .unwrap();
        assert_eq!(title.collation().unwrap().collation_name(), "C");
        let receipt = after
            .source_receipt(
                SchemaObjectLocation::column(&schema, "records", RelationKind::Table, "title")
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), after.snapshot_digest());
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_identity_sequence_settings_change_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_identity_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TABLE \"{schema}\".record \
               (id bigint GENERATED ALWAYS AS IDENTITY \
                (START WITH 5 INCREMENT BY 3 MINVALUE 2 MAXVALUE 100 CACHE 7 CYCLE), \
                title text)"
        ))
        .await
        .unwrap();
    let result = async {
        let first = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let catalog = client
            .query_one(
                "SELECT ps.seqstart, ps.seqincrement, ps.seqmin, ps.seqmax, ps.seqcache, ps.seqcycle \
                 FROM pg_catalog.pg_sequence ps \
                 JOIN pg_catalog.pg_class s ON s.oid = ps.seqrelid \
                 JOIN pg_catalog.pg_namespace n ON n.oid = s.relnamespace \
                 WHERE n.nspname = $1 AND s.relname = 'record_id_seq'",
                &[&schema],
            )
            .await
            .unwrap();
        let identity = first
            .column_identities()
            .unwrap()
            .iter()
            .find(|identity| identity.column_name() == "id")
            .unwrap();
        assert!(identity.is_generated_always());
        let sequence = identity.sequence().unwrap();
        assert_eq!(sequence.sequence_name().schema_name(), schema);
        assert_eq!(sequence.sequence_name().type_name(), "record_id_seq");
        assert_eq!(sequence.sequence_type().type_name(), "int8");
        assert_eq!(
            (
                sequence.start(),
                sequence.increment(),
                sequence.minimum(),
                sequence.maximum(),
                sequence.cache(),
                sequence.cycle(),
            ),
            (
                catalog.get::<_, i64>(0),
                catalog.get::<_, i64>(1),
                catalog.get::<_, i64>(2),
                catalog.get::<_, i64>(3),
                catalog.get::<_, i64>(4),
                catalog.get::<_, bool>(5),
            )
        );
        client
            .batch_execute(&format!(
                "ALTER SEQUENCE \"{schema}\".record_id_seq INCREMENT BY 4"
            ))
            .await
            .unwrap();
        let changed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(first.snapshot_digest(), changed.snapshot_digest());
        assert_eq!(
            changed
                .column_identities()
                .unwrap()
                .iter()
                .find(|identity| identity.column_name() == "id")
                .unwrap()
                .sequence()
                .unwrap()
                .increment(),
            4
        );
        client
            .batch_execute(&format!(
                "COMMENT ON SEQUENCE \"{schema}\".record_id_seq IS 'reviewed sequence'"
            ))
            .await
            .unwrap();
        let commented = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(changed.snapshot_digest(), commented.snapshot_digest());
        assert_eq!(
            commented
                .column_identities()
                .unwrap()
                .iter()
                .find(|identity| identity.column_name() == "id")
                .unwrap()
                .sequence()
                .unwrap()
                .source_comment(),
            Some("reviewed sequence")
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".record ALTER COLUMN id SET GENERATED BY DEFAULT"
            ))
            .await
            .unwrap();
        let by_default = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(commented.snapshot_digest(), by_default.snapshot_digest());
        assert!(by_default
            .column_identities()
            .unwrap()
            .iter()
            .find(|identity| identity.column_name() == "id")
            .unwrap()
            .is_generated_by_default());
        client
            .batch_execute(&format!("CREATE SEQUENCE \"{schema}\".standalone"))
            .await
            .unwrap();
        assert!(matches!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await,
            Err(SourceObservationFailure::InvalidCapturedMetadata)
        ));
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_nondefault_table_tablespace_changes_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let can_create_tablespace: bool = client
        .query_one(
            "SELECT rolsuper FROM pg_catalog.pg_roles WHERE rolname = current_user",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    if !can_create_tablespace {
        eprintln!("skipping tablespace fixture: setup requires a superuser");
        connection_task.abort();
        return;
    }
    let suffix = std::process::id();
    let schema = format!("cw_table_space_fixture_{suffix}");
    let tablespace = format!("cw_table_space_{suffix}");
    let directory = std::env::temp_dir().join(&tablespace);
    std::fs::create_dir(&directory).unwrap();
    client
        .batch_execute(&format!(
            "CREATE TABLESPACE \"{tablespace}\" LOCATION '{}'",
            directory.display()
        ))
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; CREATE TABLE \"{schema}\".record (id integer)"
        ))
        .await
        .unwrap();
    let before = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    assert_eq!(before.relations().len(), 1);
    client
        .batch_execute(&format!(
            "ALTER TABLE \"{schema}\".record SET TABLESPACE \"{tablespace}\""
        ))
        .await
        .unwrap();
    let observed = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    client
        .batch_execute(&format!("DROP TABLESPACE \"{tablespace}\""))
        .await
        .unwrap();
    std::fs::remove_dir(&directory).unwrap();
    connection_task.abort();
    let after = observed.unwrap();
    let [original] = before.relation_tablespaces().unwrap() else {
        panic!("one table tablespace expected");
    };
    let [moved] = after.relation_tablespaces().unwrap() else {
        panic!("one table tablespace expected");
    };
    assert!(original.tablespace().is_database_default());
    assert_eq!(moved.tablespace().name(), tablespace);
    assert!(!moved.tablespace().is_database_default());
    assert_ne!(before.snapshot_digest(), after.snapshot_digest());
}

#[tokio::test]
async fn postgres18_declared_array_dimensions_change_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_array_dimensions_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; CREATE TABLE \"{schema}\".record (payload integer[])"
        ))
        .await
        .unwrap();
    let before = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "ALTER TABLE \"{schema}\".record ALTER COLUMN payload TYPE integer[][]"
        ))
        .await
        .unwrap();
    let after = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    let catalog_dimensions: i16 = client
        .query_one(
            "SELECT a.attndims FROM pg_catalog.pg_attribute a \
             JOIN pg_catalog.pg_class c ON c.oid = a.attrelid \
             JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
             WHERE n.nspname = $1 AND c.relname = 'record' AND a.attname = 'payload'",
            &[&schema],
        )
        .await
        .unwrap()
        .get(0);
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    let after = after.unwrap();
    assert_eq!(catalog_dimensions, 2);
    assert_eq!(
        before.relations()[0].columns()[0].data_type(),
        after.relations()[0].columns()[0].data_type()
    );
    assert_eq!(before.column_array_dimensions().unwrap()[0].dimensions(), 1);
    assert_eq!(after.column_array_dimensions().unwrap()[0].dimensions(), 2);
    assert_ne!(before.snapshot_digest(), after.snapshot_digest());
    let receipt = after
        .source_receipt(
            SchemaObjectLocation::column(&schema, "record", RelationKind::Table, "payload")
                .unwrap(),
        )
        .unwrap();
    assert_eq!(receipt.source_digest(), after.snapshot_digest());
}

#[tokio::test]
async fn postgres18_nondefault_column_storage_settings_fail_closed() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    for (index, setting) in [
        "SET STORAGE EXTERNAL",
        "SET COMPRESSION pglz",
        "SET STATISTICS 500",
    ]
    .into_iter()
    .enumerate()
    {
        let schema = format!("cw_column_storage_fixture_{}_{}", std::process::id(), index);
        client
            .batch_execute(&format!(
                "CREATE SCHEMA \"{schema}\"; CREATE TABLE \"{schema}\".record (payload text)"
            ))
            .await
            .unwrap();
        let before = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await;
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".record ALTER COLUMN payload {setting}"
            ))
            .await
            .unwrap();
        let after = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await;
        client
            .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
            .await
            .unwrap();
        assert!(before.is_ok(), "baseline for {setting}: {before:?}");
        assert!(
            matches!(
                after,
                Err(SourceObservationFailure::InvalidCapturedMetadata)
            ),
            "nondefault {setting} must fail closed: {after:?}"
        );
    }
    connection_task.abort();
}

#[tokio::test]
async fn postgres18_type_owner_changes_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let can_create_role: bool = client
        .query_one(
            "SELECT rolsuper FROM pg_catalog.pg_roles WHERE rolname = current_user",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    if !can_create_role {
        eprintln!("skipping type-owner fixture: setup requires a superuser");
        connection_task.abort();
        return;
    }
    let suffix = std::process::id();
    let schema = format!("cw_type_owner_fixture_{suffix}");
    let owner = format!("cw_type_owner_{suffix}");
    client
        .batch_execute(&format!(
            "CREATE ROLE \"{owner}\" NOLOGIN; CREATE SCHEMA \"{schema}\"; \
             CREATE DOMAIN \"{schema}\".score AS integer; \
             CREATE TYPE \"{schema}\".stage AS ENUM ('new', 'done')"
        ))
        .await
        .unwrap();
    let before = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "ALTER DOMAIN \"{schema}\".score OWNER TO \"{owner}\""
        ))
        .await
        .unwrap();
    let domain_changed = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "ALTER TYPE \"{schema}\".stage OWNER TO \"{owner}\""
        ))
        .await
        .unwrap();
    let after = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!(
            "DROP SCHEMA \"{schema}\" CASCADE; DROP ROLE \"{owner}\""
        ))
        .await
        .unwrap();
    connection_task.abort();
    let after = after.unwrap();
    assert_ne!(before.snapshot_digest(), domain_changed.snapshot_digest());
    assert_ne!(domain_changed.snapshot_digest(), after.snapshot_digest());
    let owners = after.type_owners().unwrap();
    assert_eq!(owners.len(), after.type_kinds().unwrap().len());
    assert!(
        owners
            .iter()
            .filter(|item| {
                matches!(item.type_name().type_name(), "score" | "stage")
                    && item.owner_role_name() == owner
            })
            .count()
            == 2
    );
}

#[tokio::test]
async fn postgres18_table_owner_changes_source_identity() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let can_create_role: bool = client
        .query_one(
            "SELECT rolsuper FROM pg_catalog.pg_roles WHERE rolname = current_user",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    if !can_create_role {
        eprintln!("skipping table-owner fixture: setup requires a superuser");
        connection_task.abort();
        return;
    }
    let suffix = std::process::id();
    let schema = format!("cw_owner_fixture_{suffix}");
    let owner = format!("cw_owner_{suffix}");
    client
        .batch_execute(&format!(
            "CREATE ROLE \"{owner}\" NOLOGIN; CREATE SCHEMA \"{schema}\"; \
             CREATE TABLE \"{schema}\".record (id integer)"
        ))
        .await
        .unwrap();
    let before = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await
        .unwrap();
    client
        .batch_execute(&format!(
            "ALTER TABLE \"{schema}\".record OWNER TO \"{owner}\""
        ))
        .await
        .unwrap();
    let after = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    let owner_oid: u32 = client
        .query_one(
            "SELECT c.relowner FROM pg_catalog.pg_class c \
             JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
             WHERE n.nspname = $1 AND c.relname = 'record'",
            &[&schema],
        )
        .await
        .unwrap()
        .get(0);
    client
        .batch_execute(&format!(
            "DROP SCHEMA \"{schema}\" CASCADE; DROP ROLE \"{owner}\""
        ))
        .await
        .unwrap();
    connection_task.abort();
    let after = after.unwrap();
    assert_eq!(after.relation_owners().unwrap()[0].owner_oid(), owner_oid);
    assert_eq!(after.relation_owners().unwrap()[0].owner_role_name(), owner);
    assert_ne!(before.snapshot_digest(), after.snapshot_digest());
}

#[tokio::test]
async fn postgres18_custom_table_access_method_fails_closed() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let can_create_access_method: bool = client
        .query_one(
            "SELECT r.rolsuper FROM pg_catalog.pg_roles r WHERE r.rolname = current_user",
            &[],
        )
        .await
        .unwrap()
        .get(0);
    if !can_create_access_method {
        eprintln!("skipping custom table access method fixture: setup requires a superuser");
        connection_task.abort();
        return;
    }
    let suffix = std::process::id();
    let schema = format!("cw_table_am_fixture_{suffix}");
    let method = format!("cw_table_am_{suffix}");
    client
        .batch_execute(&format!(
            "CREATE ACCESS METHOD \"{method}\" TYPE TABLE HANDLER pg_catalog.heap_tableam_handler; \
             CREATE SCHEMA \"{schema}\"; \
             CREATE TABLE \"{schema}\".record (id integer) USING \"{method}\""
        ))
        .await
        .unwrap();
    let observed_method: String = client
        .query_one(
            "SELECT am.amname::text FROM pg_catalog.pg_class c \
             JOIN pg_catalog.pg_am am ON am.oid = c.relam \
             JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
             WHERE n.nspname = $1 AND c.relname = 'record'",
            &[&schema],
        )
        .await
        .unwrap()
        .get(0);
    let observed = adapter(config)
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    client
        .batch_execute(&format!(
            "DROP SCHEMA \"{schema}\" CASCADE; DROP ACCESS METHOD \"{method}\""
        ))
        .await
        .unwrap();
    connection_task.abort();
    assert_eq!(observed_method, method);
    assert!(matches!(
        observed,
        Err(SourceObservationFailure::InvalidCapturedMetadata)
    ));
}

#[tokio::test]
async fn postgres18_domain_constraint_comment_fails_closed() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_domain_comment_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE DOMAIN \"{schema}\".score AS integer \
               CONSTRAINT nonnegative CHECK (VALUE >= 0)"
        ))
        .await
        .unwrap();
    let result: Result<(), SourceObservationFailure> = async {
        let original = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_eq!(original.domains()[0].check_constraints().len(), 1);
        client
            .batch_execute(&format!(
                "COMMENT ON CONSTRAINT nonnegative ON DOMAIN \"{schema}\".score \
                 IS 'reviewed restriction'"
            ))
            .await
            .unwrap();
        assert!(matches!(
            adapter(config)
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await,
            Err(SourceObservationFailure::InvalidCapturedMetadata)
        ));
        Ok(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_domain_and_enum_acl_changes_fail_closed() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_type_acl_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TYPE \"{schema}\".stage AS ENUM ('open', 'closed'); \
             CREATE DOMAIN \"{schema}\".score AS integer CHECK (VALUE >= 0)"
        ))
        .await
        .unwrap();
    let result: Result<(), SourceObservationFailure> = async {
        let original = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_eq!(original.enums().len(), 1);
        assert_eq!(original.domains().len(), 1);
        for type_name in ["stage", "score"] {
            client
                .batch_execute(&format!(
                    "REVOKE USAGE ON TYPE \"{schema}\".{type_name} FROM PUBLIC"
                ))
                .await
                .unwrap();
            assert!(matches!(
                adapter(config.clone())
                    .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                    .await,
                Err(SourceObservationFailure::InvalidCapturedMetadata)
            ));
            client
                .batch_execute(&format!("DROP TYPE \"{schema}\".{type_name}"))
                .await
                .unwrap();
        }
        Ok(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_standalone_composite_type_retains_column_receipts() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_composite_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TYPE \"{schema}\".assessment AS (risk_id uuid, label text)"
        ))
        .await
        .unwrap();
    let result: Result<(), SourceObservationFailure> = async {
        let first = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let [relation] = first.relations() else {
            panic!("standalone composite type must be the sole relation");
        };
        assert_eq!(relation.kind(), RelationKind::CompositeType);
        assert_eq!(relation.relation_name(), "assessment");
        assert!(first.type_kinds().unwrap().iter().any(|kind| {
            kind.type_name().schema_name() == schema
                && kind.type_name().type_name() == "assessment"
                && kind.kind() == PostgresTypeKind::Composite
        }));
        let linked: bool = client
            .query_one(
                "SELECT t.typtype = 'c' AND t.typrelid = c.oid \
                 FROM pg_catalog.pg_class c \
                 JOIN pg_catalog.pg_namespace n ON n.oid = c.relnamespace \
                 JOIN pg_catalog.pg_type t ON t.oid = c.reltype \
                 WHERE n.nspname = $1 AND c.relname = 'assessment'",
                &[&schema],
            )
            .await
            .unwrap()
            .get(0);
        assert!(linked);
        assert_eq!(
            relation
                .columns()
                .iter()
                .map(|column| (column.ordinal_position(), column.column_name()))
                .collect::<Vec<_>>(),
            vec![(1, "risk_id"), (2, "label")]
        );
        let receipt = first
            .source_receipt(
                SchemaObjectLocation::column(
                    &schema,
                    "assessment",
                    RelationKind::CompositeType,
                    "label",
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), first.snapshot_digest());
        let proposal = propose_relational_model(&first).unwrap();
        assert!(proposal.concepts().is_empty());
        assert!(proposal.relations().is_empty());
        let [
            ProposedSourceType::Composite {
                candidate,
                observation,
                fields,
            },
        ] = proposal.source_types()
        else {
            panic!("composite source type must have one review candidate");
        };
        assert_eq!(
            candidate.evidence()[0].source_digest(),
            first.snapshot_digest()
        );
        let type_receipt = first
            .source_receipt(
                SchemaObjectLocation::relation(&schema, "assessment", RelationKind::CompositeType)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            candidate.evidence()[0].location(),
            type_receipt.location().canonical_location()
        );
        assert_eq!(observation.columns().len(), 2);
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].source_name(), "risk_id");
        assert_eq!(fields[1].source_name(), "label");
        assert_eq!(
            fields[1].evidence().location(),
            receipt.location().canonical_location()
        );
        assert!(matches!(
            align_relational_proposal(
                &proposal,
                proposal.proposal_id(),
                vec![AlignmentDecision::exclude(candidate.candidate_id(), "Omit type").unwrap()],
            ),
            Err(AlignmentError::MissingDecision)
        ));
        let orphan = align_relational_proposal(
            &proposal,
            proposal.proposal_id(),
            vec![
                AlignmentDecision::exclude(candidate.candidate_id(), "Omit type").unwrap(),
                AlignmentDecision::map(
                    fields[0].candidate().candidate_id(),
                    "risk.orphan",
                    "Orphan",
                    "Invalid fixture mapping",
                )
                .unwrap(),
                AlignmentDecision::exclude(fields[1].candidate().candidate_id(), "Omit field")
                    .unwrap(),
            ],
        )
        .unwrap();
        assert_eq!(
            validate_alignment(&orphan).unwrap_err(),
            AlignmentError::ExcludedFieldParent
        );
        let aligned = align_relational_proposal(
            &proposal,
            proposal.proposal_id(),
            std::iter::once(
                AlignmentDecision::exclude(candidate.candidate_id(), "Source-only fixture")
                    .unwrap(),
            )
            .chain(fields.iter().map(|field| {
                AlignmentDecision::exclude(field.candidate().candidate_id(), "Source-only field")
                    .unwrap()
            }))
            .collect(),
        )
        .unwrap();
        assert_eq!(validate_alignment(&aligned).unwrap().candidates().len(), 3);
        let mapped = align_relational_proposal(
            &proposal,
            proposal.proposal_id(),
            std::iter::once(
                AlignmentDecision::map(
                    candidate.candidate_id(),
                    "risk.assessment",
                    "Assessment",
                    "Explicit fixture concept mapping",
                )
                .unwrap(),
            )
            .chain(fields.iter().enumerate().map(|(index, field)| {
                AlignmentDecision::map(
                    field.candidate().candidate_id(),
                    format!("risk.assessment.field.{index}"),
                    field.source_name(),
                    "Explicit fixture attribute mapping",
                )
                .unwrap()
            }))
            .collect(),
        )
        .unwrap();
        let validated = validate_alignment(&mapped).unwrap();
        assert_eq!(validated.validation().mapped_fields_with_concepts(), 2);
        assert_eq!(
            validated
                .field_parents()
                .get(fields[1].candidate().candidate_id()),
            Some(&candidate.candidate_id().to_owned())
        );
        let reviewed = review(
            &validated,
            "fixture-steward",
            "Review composite type mapping",
            &FixtureSteward {
                proposal_id: proposal.proposal_id(),
                source_digest: first.snapshot_digest(),
            },
        )
        .unwrap();
        let publication_root = std::env::temp_dir().join(format!(
            "cw-composite-release-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&publication_root).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&publication_root, std::fs::Permissions::from_mode(0o700))
                .unwrap();
        }
        let store = FilePublicationStore::new(&publication_root).unwrap();
        let published = store
            .publish(
                &reviewed,
                ReleaseMetadata::new("composite-fixture", "1.0.0", "fixture-ontology").unwrap(),
            )
            .unwrap();
        assert_eq!(published.release().concept_ids(), &["risk.assessment"]);
        assert_eq!(published.release().provenance().len(), 3);
        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let signing_key = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        let key = TrustedPublisherKey::new("fixture-publisher", signing_key.public_key().as_ref())
            .unwrap();
        let signed =
            sign_published_manifest(&published, "fixture-publisher", &signing_key).unwrap();
        let pinned = SemanticReleaseClient::with_signed_release_manifests(
            "1.0.0",
            vec![],
            &[key],
            &[signed],
        )
        .unwrap();
        assert_eq!(
            pinned
                .resolve_concept(published.release(), "risk.assessment")
                .unwrap(),
            Some("risk.assessment")
        );
        assert_eq!(
            store
                .read_verified(&pinned, published.release())
                .unwrap()
                .unwrap(),
            published.artifact_bytes()
        );
        std::fs::remove_dir_all(&publication_root).unwrap();
        let replay = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_eq!(replay.snapshot_digest(), first.snapshot_digest());
        client
            .batch_execute(&format!(
                "COMMENT ON TYPE \"{schema}\".assessment IS 'type-only comment'"
            ))
            .await
            .unwrap();
        assert!(matches!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await,
            Err(SourceObservationFailure::InvalidCapturedMetadata)
        ));
        client
            .batch_execute(&format!(
                "COMMENT ON TYPE \"{schema}\".assessment IS NULL; \
                 CREATE TABLE \"{schema}\".assessment_record \
                   (id integer PRIMARY KEY, payload \"{schema}\".assessment)"
            ))
            .await
            .unwrap();
        let mixed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_eq!(mixed.relations().len(), 2);
        let payload = mixed
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "assessment_record")
            .unwrap()
            .columns()
            .iter()
            .find(|column| column.column_name() == "payload")
            .unwrap();
        assert_eq!(payload.type_binding().schema_name(), schema);
        assert_eq!(payload.type_binding().type_name(), "assessment");
        client
            .batch_execute(&format!(
                "COMMENT ON TYPE \"{schema}\".assessment_record IS 'table row type comment'"
            ))
            .await
            .unwrap();
        assert!(matches!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await,
            Err(SourceObservationFailure::InvalidCapturedMetadata)
        ));
        client
            .batch_execute(&format!(
                "COMMENT ON TYPE \"{schema}\".assessment_record IS NULL; \
                 ALTER TYPE \"{schema}\".assessment ADD ATTRIBUTE priority integer"
            ))
            .await
            .unwrap();
        let changed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(mixed.snapshot_digest(), first.snapshot_digest());
        assert_ne!(changed.snapshot_digest(), mixed.snapshot_digest());
        let changed_proposal = propose_relational_model(&changed).unwrap();
        assert_ne!(changed_proposal.proposal_id(), proposal.proposal_id());
        assert_eq!(changed_proposal.source_types().len(), 1);
        let changed_candidate = changed_proposal.source_types()[0].candidate();
        let changed_fields = changed_proposal.source_types()[0].fields();
        assert_eq!(changed_fields.len(), 3);
        let changed_mapped = align_relational_proposal(
            &changed_proposal,
            changed_proposal.proposal_id(),
            std::iter::once(
                AlignmentDecision::map(
                    changed_candidate.candidate_id(),
                    "risk.assessment",
                    "Assessment",
                    "Explicit fixture concept mapping",
                )
                .unwrap(),
            )
            .chain(changed_fields.iter().enumerate().map(|(index, field)| {
                AlignmentDecision::map(
                    field.candidate().candidate_id(),
                    format!("risk.assessment.field.{index}"),
                    field.source_name(),
                    "Explicit fixture attribute mapping",
                )
                .unwrap()
            }))
            .chain(changed_proposal.concepts().iter().flat_map(|concept| {
                std::iter::once(
                    AlignmentDecision::exclude(
                        concept.candidate().candidate_id(),
                        "Exclude fixture table",
                    )
                    .unwrap(),
                )
                .chain(concept.fields().iter().map(|field| {
                    AlignmentDecision::exclude(
                        field.candidate().candidate_id(),
                        "Exclude fixture field",
                    )
                    .unwrap()
                }))
            }))
            .collect(),
        );
        let changed_reviewed = review(
            &validate_alignment(&changed_mapped.unwrap()).unwrap(),
            "fixture-steward",
            "Review composite type mapping",
            &FixtureSteward {
                proposal_id: changed_proposal.proposal_id(),
                source_digest: changed.snapshot_digest(),
            },
        )
        .unwrap();
        assert_ne!(
            reviewed.request().source_digest(),
            changed_proposal.source_digest()
        );
        assert_ne!(
            reviewed.request().alignment_digest(),
            changed_reviewed.request().alignment_digest()
        );
        client
            .batch_execute(&format!(
                "GRANT USAGE ON TYPE \"{schema}\".assessment TO PUBLIC"
            ))
            .await
            .unwrap();
        assert!(matches!(
            adapter(config)
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await,
            Err(SourceObservationFailure::InvalidCapturedMetadata)
        ));
        Ok(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_range_and_multirange_kinds_are_source_evidence() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_type_kind_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE TYPE \"{schema}\".span AS RANGE (subtype = integer); \
             CREATE DOMAIN \"{schema}\".labels AS text[]; \
             CREATE TABLE \"{schema}\".record \
               (id integer PRIMARY KEY, value \"{schema}\".span, values \"{schema}\".span_multirange, spans \"{schema}\".span[], numbers integer[], legacy oidvector)"
        ))
        .await
        .unwrap();
    let result = adapter(config.clone())
        .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
        .await;
    let pair = client
        .query_one(
            "SELECT rn.nspname::text, rt.typname::text, mn.nspname::text, mt.typname::text \
             FROM pg_catalog.pg_range r \
             JOIN pg_catalog.pg_type rt ON rt.oid = r.rngtypid \
             JOIN pg_catalog.pg_namespace rn ON rn.oid = rt.typnamespace \
             JOIN pg_catalog.pg_type mt ON mt.oid = r.rngmultitypid \
             JOIN pg_catalog.pg_namespace mn ON mn.oid = mt.typnamespace \
             WHERE rn.nspname = $1 AND rt.typname = 'span'",
            &[&schema],
        )
        .await
        .unwrap();
    let source_range = (pair.get::<_, String>(0), pair.get::<_, String>(1));
    let source_multirange = (pair.get::<_, String>(2), pair.get::<_, String>(3));
    let source_array: String = client
        .query_one(
            "SELECT a.typname::text FROM pg_catalog.pg_type e \
             JOIN pg_catalog.pg_type a ON a.oid = e.typarray \
             JOIN pg_catalog.pg_namespace n ON n.oid = e.typnamespace \
             WHERE n.nspname = $1 AND e.typname = 'span' AND a.typelem = e.oid",
            &[&schema],
        )
        .await
        .unwrap()
        .get(0);
    let shell_result = if result.is_ok() {
        client
            .batch_execute(&format!("CREATE TYPE \"{schema}\".unresolved"))
            .await
            .unwrap();
        Some(
            adapter(config)
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await
                .err(),
        )
    } else {
        None
    };
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    let snapshot = result.unwrap();
    assert!(snapshot.array_types().unwrap().iter().any(|array| {
        array.array_type().schema_name() == schema
            && array.array_type().type_name() == source_array
            && array.element_type().schema_name() == schema
            && array.element_type().type_name() == "span"
    }));
    assert!(snapshot.array_types().unwrap().iter().any(|array| {
        array.array_type().schema_name() == "pg_catalog"
            && array.array_type().type_name() == "_int4"
            && array.element_type().schema_name() == "pg_catalog"
            && array.element_type().type_name() == "int4"
    }));
    assert!(
        !snapshot
            .array_types()
            .unwrap()
            .iter()
            .any(|array| array.array_type().type_name() == "oidvector")
    );
    assert!(snapshot.array_types().unwrap().iter().any(|array| {
        array.array_type().schema_name() == "pg_catalog"
            && array.array_type().type_name() == "_text"
            && array.element_type().schema_name() == "pg_catalog"
            && array.element_type().type_name() == "text"
    }));
    assert_eq!(
        shell_result,
        Some(Some(SourceObservationFailure::InvalidCapturedMetadata))
    );
    let kinds = snapshot.type_kinds().expect("type kinds must be observed");
    let range = kinds
        .iter()
        .find(|kind| {
            kind.type_name().schema_name() == schema && kind.type_name().type_name() == "span"
        })
        .unwrap();
    let multirange = kinds
        .iter()
        .find(|kind| {
            kind.type_name().schema_name() == schema
                && kind.type_name().type_name() == "span_multirange"
        })
        .unwrap();
    assert_eq!(range.kind(), PostgresTypeKind::Range);
    assert_eq!(multirange.kind(), PostgresTypeKind::Multirange);
    assert_eq!(
        (
            range.type_name().schema_name(),
            range.type_name().type_name()
        ),
        (source_range.0.as_str(), source_range.1.as_str())
    );
    assert_eq!(
        (
            multirange.type_name().schema_name(),
            multirange.type_name().type_name()
        ),
        (source_multirange.0.as_str(), source_multirange.1.as_str())
    );
    assert_eq!(range.range_counterpart(), Some(multirange.type_name()));
    assert_eq!(multirange.range_counterpart(), Some(range.type_name()));
    assert_eq!(
        snapshot.relations()[0].columns()[1].type_binding(),
        range.type_name()
    );
    assert_eq!(
        snapshot.relations()[0].columns()[2].type_binding(),
        multirange.type_name()
    );
}

#[tokio::test]
async fn postgres18_catalog_is_observed_in_one_read_only_transaction() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_type_fixture_{}", std::process::id());
    let second_schema = format!("cw_type_fixture_{}_b", std::process::id());
    let fixture = format!(
        "CREATE SCHEMA \"{schema}\"; \
         CREATE TYPE \"{schema}\".stage AS ENUM ('new', 'done'); \
         CREATE DOMAIN \"{schema}\".risk_score AS integer DEFAULT 5 CHECK (VALUE >= 0); \
         COMMENT ON DOMAIN \"{schema}\".risk_score IS 'risk score'"
    );
    client.batch_execute(&fixture).await.unwrap();

    let result = async {
        let snapshot = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(snapshot.relations().is_empty());
        assert_eq!(snapshot.domains().len(), 1);
        assert_eq!(snapshot.enums().len(), 1);
        assert_eq!(snapshot.domains()[0].source_comment(), Some("risk score"));
        assert_eq!(snapshot.domains()[0].check_constraints().len(), 1);
        assert_eq!(snapshot.enums()[0].labels(), &["new", "done"]);
        let receipt = snapshot
            .source_receipt(SchemaObjectLocation::domain(&schema, "risk_score").unwrap())
            .unwrap();
        assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
        snapshot
            .source_receipt(SchemaObjectLocation::enum_(&schema, "stage").unwrap())
            .unwrap();

        client
            .batch_execute(&format!(
                "ALTER TYPE \"{schema}\".stage ADD VALUE 'review'; \
                 ALTER DOMAIN \"{schema}\".risk_score SET DEFAULT 7"
            ))
            .await
            .unwrap();
        let changed = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(changed.snapshot_digest(), snapshot.snapshot_digest());
        assert_eq!(changed.enums()[0].labels(), &["new", "done", "review"]);
        assert_ne!(
            changed.domains()[0].default_expression(),
            snapshot.domains()[0].default_expression()
        );

        assert_eq!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 1, 8_192), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::RowLimitExceeded { max_rows: 1 })
        );
        assert_eq!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 64, 40), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::ByteLimitExceeded { max_bytes: 40 })
        );

        client
            .batch_execute(&format!(
                "CREATE TABLE \"{schema}\".item (id integer, other integer); \
                 COMMENT ON TABLE \"{schema}\".item IS 'items'"
            ))
            .await
            .unwrap();
        let with_table = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_eq!(with_table.relations().len(), 1);
        assert_eq!(with_table.relations()[0].source_comment(), Some("items"));
        assert_eq!(with_table.relations()[0].columns().len(), 2);
        assert_eq!(
            with_table.relations()[0].columns()[0]
                .type_binding()
                .schema_name(),
            "pg_catalog"
        );
        with_table
            .source_receipt(
                SchemaObjectLocation::relation(&schema, "item", RelationKind::Table).unwrap(),
            )
            .unwrap();
        with_table
            .source_receipt(
                SchemaObjectLocation::column(&schema, "item", RelationKind::Table, "id").unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "COMMENT ON COLUMN \"{schema}\".item.id IS 'identifier'"
            ))
            .await
            .unwrap();
        let with_column_comment = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(
            with_column_comment.snapshot_digest(),
            with_table.snapshot_digest()
        );

        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_nonnegative CHECK (id >= 0) NOT VALID"
            ))
            .await
            .unwrap();
        let unvalidated = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = unvalidated
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert!(matches!(
            &item.constraints()[0],
            TableConstraintObservation::Check(check)
                if check.constraint_name() == "item_nonnegative"
                    && check.definition().contains("id >= 0")
                    && !check.validated()
                    && check.enforced()
                    && !check.no_inherit()
        ));
        unvalidated
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_nonnegative",
                )
                .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item VALIDATE CONSTRAINT item_nonnegative"
            ))
            .await
            .unwrap();
        let validated = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(unvalidated.snapshot_digest(), validated.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_unique UNIQUE (id)"
            ))
            .await
            .unwrap();
        let with_unique_constraint = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await
            .expect("ordinary UNIQUE constraint capture");
        let item = with_unique_constraint
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert!(item.constraints().iter().any(|constraint| matches!(
            constraint,
            TableConstraintObservation::Unique(unique)
                if unique.constraint_name() == "item_unique"
                    && unique.column_names() == ["id"]
                    && unique.nulls_not_distinct() == Some(false)
        )));
        with_unique_constraint
            .source_receipt(
                SchemaObjectLocation::constraint(&schema, "item", RelationKind::Table, "item_unique")
                    .unwrap(),
            )
            .unwrap();
        assert_ne!(validated.snapshot_digest(), with_unique_constraint.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_deferred UNIQUE (other) DEFERRABLE"
            ))
            .await
            .unwrap();
        let initially_immediate = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(initially_immediate.constraint_timings().unwrap().iter().any(|timing| {
            timing.constraint_name() == "item_deferred"
                && timing.deferrability() == ConstraintDeferrability::InitiallyImmediate
        }));
        initially_immediate
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_deferred",
                )
                .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_deferred; \
                 ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_deferred UNIQUE (other) DEFERRABLE INITIALLY DEFERRED"
            ))
            .await
            .unwrap();
        let initially_deferred = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(initially_deferred.constraint_timings().unwrap().iter().any(|timing| {
            timing.constraint_name() == "item_deferred"
                && timing.deferrability() == ConstraintDeferrability::InitiallyDeferred
        }));
        assert_ne!(
            initially_immediate.snapshot_digest(),
            initially_deferred.snapshot_digest()
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_deferred; \
                 ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_unique"
            ))
            .await
            .unwrap();
        let after_drop = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_eq!(
            after_drop.relations()[0].constraints().len(),
            1,
            "dropped internal triggers must not block the remaining CHECK"
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_other_not_null \
                 NOT NULL other NOT VALID"
            ))
            .await
            .unwrap();
        let with_not_null = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(with_not_null
            .not_null_constraints()
            .unwrap()
            .iter()
            .any(|constraint| {
                constraint.relation_name() == "item"
                    && constraint.constraint_name() == "item_other_not_null"
                    && constraint.column_name() == "other"
                    && !constraint.validated()
            }));
        with_not_null
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_other_not_null",
                )
                .unwrap(),
            )
            .unwrap();
        assert_ne!(after_drop.snapshot_digest(), with_not_null.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item VALIDATE CONSTRAINT item_other_not_null"
            ))
            .await
            .unwrap();
        let validated_not_null = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(validated_not_null.not_null_constraints().unwrap()[0].validated());
        assert_ne!(with_not_null.snapshot_digest(), validated_not_null.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_other_not_null"
            ))
            .await
            .unwrap();
        let restored = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_eq!(after_drop.snapshot_digest(), restored.snapshot_digest());
        assert!(restored
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_other_not_null",
                )
                .unwrap(),
            )
            .is_err());
        client
            .batch_execute(&format!(
                "CREATE TABLE \"{schema}\".keyed (id integer PRIMARY KEY DEFERRABLE INITIALLY DEFERRED)"
            ))
            .await
            .unwrap();
        let with_primary_key = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let keyed = with_primary_key
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "keyed")
            .unwrap();
        assert!(keyed.constraints().iter().any(|constraint| matches!(
            constraint,
            TableConstraintObservation::PrimaryKey(primary)
                if primary.constraint_name() == "keyed_pkey"
                    && primary.column_names() == ["id"]
        )));
        assert!(with_primary_key.constraint_timings().unwrap().iter().any(|timing| {
            timing.constraint_name() == "keyed_pkey"
                && timing.deferrability() == ConstraintDeferrability::InitiallyDeferred
        }));
        assert!(with_primary_key
            .not_null_constraints()
            .unwrap()
            .iter()
            .any(|constraint| {
                constraint.relation_name() == "keyed"
                    && constraint.constraint_name() == "keyed_id_not_null"
                    && constraint.column_name() == "id"
                    && constraint.validated()
            }));
        for name in ["keyed_pkey", "keyed_id_not_null"] {
            let receipt = with_primary_key
                .source_receipt(
                    SchemaObjectLocation::constraint(&schema, "keyed", RelationKind::Table, name)
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(receipt.source_digest(), with_primary_key.snapshot_digest());
        }
        assert_ne!(after_drop.snapshot_digest(), with_primary_key.snapshot_digest());
        client
            .batch_execute(&format!("DROP TABLE \"{schema}\".keyed"))
            .await
            .unwrap();
        client
            .batch_execute(&format!(
                "CREATE FUNCTION \"{schema}\".keep_item() RETURNS trigger LANGUAGE plpgsql \
                 AS $$ BEGIN RETURN NEW; END $$; \
                 CREATE TRIGGER item_live_trigger BEFORE INSERT ON \"{schema}\".item \
                 FOR EACH ROW EXECUTE FUNCTION \"{schema}\".keep_item()"
            ))
            .await
            .unwrap();
        assert_eq!(
            adapter(config.clone())
                .observe(authorized(&schema), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::InvalidCapturedMetadata)
        );
        client
            .batch_execute(&format!(
                "DROP TRIGGER item_live_trigger ON \"{schema}\".item"
            ))
            .await
            .unwrap();
        adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;

        client
            .batch_execute(&format!("CREATE TABLE \"{schema}\".\" \" (\" \" integer)"))
            .await
            .unwrap();
        let quoted = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(quoted.relations().iter().any(|relation| {
            relation.relation_name() == " " && relation.columns()[0].column_name() == " "
        }));
        quoted
            .source_receipt(
                SchemaObjectLocation::column(&schema, " ", RelationKind::Table, " ").unwrap(),
            )
            .unwrap();

        client
            .batch_execute(&format!(
                "CREATE SCHEMA \"{second_schema}\"; \
                 CREATE TYPE \"{second_schema}\".stage AS ENUM ('other')"
            ))
            .await
            .unwrap();
        let forward = adapter(config.clone())
            .observe(authorized_two([&schema, &second_schema]), &NotCancelled)
            .await?;
        let reverse = adapter(config.clone())
            .observe(authorized_two([&second_schema, &schema]), &NotCancelled)
            .await?;
        assert_eq!(forward.snapshot_digest(), reverse.snapshot_digest());
        assert_eq!(forward.enums().len(), 2);
        assert_ne!(forward.enums()[0].labels(), forward.enums()[1].labels());
        forward
            .source_receipt(SchemaObjectLocation::enum_(&second_schema, "stage").unwrap())
            .unwrap();

        client
            .batch_execute(&format!(
                "CREATE TABLE \"{schema}\".defaulted (id integer DEFAULT 1, \
                 doubled integer GENERATED ALWAYS AS (id * 2) STORED, \
                 tripled integer GENERATED ALWAYS AS (id * 3) VIRTUAL)"
            ))
            .await
            .unwrap();
        let with_expressions = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let expressions = with_expressions.column_expressions().unwrap();
        let generations = with_expressions.column_generations().unwrap();
        assert!(
            expressions
                .iter()
                .any(|value| { value.column_name() == "id" && value.is_default_expression() })
        );
        assert!(
            expressions.iter().any(|value| {
                value.column_name() == "doubled" && value.is_generation_expression()
            })
        );
        assert!(
            generations
                .iter()
                .any(|value| { value.column_name() == "tripled" && value.is_virtual_generated() })
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".defaulted ALTER COLUMN id SET DEFAULT 2"
            ))
            .await
            .unwrap();
        let changed_default = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(
            with_expressions.snapshot_digest(),
            changed_default.snapshot_digest()
        );
        client
            .batch_execute(&format!("DROP TABLE \"{schema}\".defaulted"))
            .await
            .unwrap();

        client
            .batch_execute(&format!(
                "CREATE INDEX item_id_idx ON \"{schema}\".item (id)"
            ))
            .await
            .unwrap();
        let with_index = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_index
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert_eq!(item.indexes().len(), 1);
        assert_eq!(
            item.indexes()[0].key_attributes()[0].attribute_name(),
            Some("id")
        );
        assert_eq!(item.indexes()[0].key_semantics().unwrap().len(), 1);
        with_index
            .source_receipt(
                SchemaObjectLocation::index(&schema, "item", RelationKind::Table, "item_id_idx")
                    .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "COMMENT ON INDEX \"{schema}\".item_id_idx IS 'lookup'"
            ))
            .await
            .unwrap();
        let changed_index = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(
            with_index.snapshot_digest(),
            changed_index.snapshot_digest()
        );
        client
            .batch_execute(&format!(
                "CREATE UNIQUE INDEX item_id_unique_idx ON \"{schema}\".item (id) NULLS NOT DISTINCT"
            ))
            .await
            .unwrap();
        let with_unique = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_unique
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert_eq!(item.indexes().len(), 2);
        assert!(item.indexes().iter().any(|index| {
            index.index_name() == "item_id_unique_idx"
                && index.is_unique()
                && index.nulls_not_distinct() == Some(true)
        }));
        assert_ne!(changed_index.snapshot_digest(), with_unique.snapshot_digest());
        client
            .batch_execute(&format!(
                "CREATE INDEX item_expr_idx ON \"{schema}\".item (id, (other + 1)) INCLUDE (other) WHERE id > 0"
            ))
            .await
            .unwrap();
        let with_expression_index = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_expression_index
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        let expression_index = item
            .indexes()
            .iter()
            .find(|index| index.index_name() == "item_expr_idx")
            .unwrap();
        assert_eq!(expression_index.key_attributes()[0].attribute_name(), Some("id"));
        assert!(expression_index.key_attributes()[1]
            .expression_text()
            .unwrap()
            .contains("other"));
        assert_eq!(
            expression_index.include_attributes()[0].attribute_name(),
            Some("other")
        );
        assert!(expression_index.predicate().unwrap().contains("id"));
        with_expression_index
            .source_receipt(
                SchemaObjectLocation::index(&schema, "item", RelationKind::Table, "item_expr_idx")
                    .unwrap(),
            )
            .unwrap();
        assert_ne!(with_unique.snapshot_digest(), with_expression_index.snapshot_digest());
        client
            .batch_execute(&format!(
                "CREATE INDEX item_hash_idx ON \"{schema}\".item USING hash (id)"
            ))
            .await
            .unwrap();
        let with_hash = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_hash
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert!(item.indexes().iter().any(|index| {
            index.index_name() == "item_hash_idx" && index.access_method() == Some("hash")
        }));
        client
            .batch_execute(&format!(
                "ALTER INDEX \"{schema}\".item_id_idx SET (fillfactor = 80)"
            ))
            .await
            .unwrap();
        let with_storage_option = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_storage_option
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        let index = item
            .indexes()
            .iter()
            .find(|index| index.index_name() == "item_id_idx")
            .unwrap();
        assert_eq!(index.storage_options().unwrap()[0].name(), "fillfactor");
        assert_eq!(index.storage_options().unwrap()[0].value(), "80");
        assert_ne!(with_hash.snapshot_digest(), with_storage_option.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER INDEX \"{schema}\".item_expr_idx ALTER COLUMN 2 SET STATISTICS 1000"
            ))
            .await
            .unwrap();
        assert_eq!(
            adapter(config.clone())
                .observe(authorized(&schema), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::InvalidCapturedMetadata)
        );
        Ok::<_, SourceObservationFailure>(())
    }
    .await;

    client
        .batch_execute(&format!(
            "DROP SCHEMA IF EXISTS \"{second_schema}\" CASCADE; \
             DROP SCHEMA \"{schema}\" CASCADE"
        ))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}
