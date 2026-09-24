use conceptweave_observation::{
    ColumnObservationV3, ObservationError, PostgresSchemaSnapshotV3, QualifiedTypeName,
    RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    ColumnTypeModifierLocation, ColumnTypeModifierObservation, PartitionParentRelationCoordinate,
    RelationPartitionObservation, RelationPartitionSnapshot, RelationPartitionTypeModifierSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "warehouse_primary"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "warehouse_primary").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "warehouse_primary"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == "warehouse_primary"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && resource_envelope.request_budget().max_schema_count() <= 1
            && resource_envelope.request_budget().max_schema_bytes() <= 256
            && resource_envelope.limits().operation_timeout_ms() <= 1_000
            && resource_envelope.limits().statement_timeout_ms() <= 1_000
            && resource_envelope.limits().max_rows() <= 10
            && resource_envelope.limits().max_bytes() <= 1_024
            && resource_envelope.limits().max_concurrent_queries() <= 1
    }
}

fn authorized_source() -> AuthorizedObservationRequest {
    ObservationRequest::new(
        "warehouse_primary",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn column(name: &str, ordinal: u32, data_type: &str, type_name: &str) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        ordinal,
        data_type,
        QualifiedTypeName::new("pg_catalog", type_name).unwrap(),
        true,
        None,
    )
    .unwrap()
}

fn relation(
    name: &str,
    kind: RelationKind,
    columns: Vec<ColumnObservationV3>,
) -> RelationObservation {
    RelationObservation::new("public", name, kind, columns).unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-partition-atttypmod-v1",
        "2026-09-15T04:44:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                vec![
                    column("account_code", 1, "character varying", "varchar"),
                    column("account_email", 2, "text", "text"),
                ],
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                vec![
                    column("account_email", 1, "text", "text"),
                    column("account_code", 2, "character varying", "varchar"),
                ],
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap()
}

fn relation_partition_snapshot(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
    RelationPartitionSnapshot::new(
        base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "accounts_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "accounts").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn modifier(
    relation_name: &str,
    relation_kind: RelationKind,
    column_name: &str,
    atttypmod: i32,
) -> ColumnTypeModifierObservation {
    ColumnTypeModifierObservation::new(
        "public",
        relation_name,
        relation_kind,
        column_name,
        atttypmod,
    )
    .unwrap()
}

fn complete_modifiers(
    parent_varchar: i32,
    child_varchar: i32,
) -> Vec<ColumnTypeModifierObservation> {
    vec![
        modifier(
            "accounts",
            RelationKind::PartitionedTable,
            "account_code",
            parent_varchar,
        ),
        modifier(
            "accounts",
            RelationKind::PartitionedTable,
            "account_email",
            -1,
        ),
        modifier("accounts_2026", RelationKind::Table, "account_email", -1),
        modifier(
            "accounts_2026",
            RelationKind::Table,
            "account_code",
            child_varchar,
        ),
    ]
}

#[test]
fn raw_type_modifier_mismatch_fails_even_when_rendered_type_text_matches() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);

    let error = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 68),
    )
    .expect_err(
        "PostgreSQL build_attrmap_by_name rejects equal type OIDs with different raw atttypmod",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_type_modifier",
        }
    );
}

#[test]
fn complete_equal_raw_type_modifiers_preserve_name_mapping_across_physical_order() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);

    let snapshot = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 36),
    )
    .expect("same-name parent and child columns with equal raw atttypmod are admissible");

    assert_eq!(snapshot.observations().len(), 4);
    assert!(snapshot.snapshot_digest().starts_with("sha256:"));
}

#[test]
fn raw_negative_one_type_modifier_is_preserved_as_catalog_evidence() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let snapshot = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(-1, -1),
    )
    .expect("atttypmod=-1 is a valid exact PostgreSQL catalog value");

    let observation = snapshot
        .observations()
        .iter()
        .find(|observation| {
            observation.relation_name() == "accounts" && observation.column_name() == "account_code"
        })
        .unwrap();
    assert_eq!(observation.type_modifier(), -1);
}

#[test]
fn structured_type_modifier_evidence_must_cover_every_bounded_column_once() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let mut observations = complete_modifiers(36, 36);
    observations.pop();

    let error =
        RelationPartitionTypeModifierSnapshot::new(&base, &relation_partition, observations)
            .expect_err("absence is not equivalent to atttypmod=-1");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_type_modifier_completeness",
        }
    );
}

#[test]
fn structured_type_modifier_changes_successor_identity_without_rewriting_predecessor() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let varchar_32 = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 36),
    )
    .unwrap();
    let unconstrained = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(-1, -1),
    )
    .unwrap();

    assert_ne!(
        varchar_32.snapshot_digest(),
        unconstrained.snapshot_digest()
    );
    assert_eq!(
        relation_partition.snapshot_digest(),
        relation_partition_snapshot(&base).snapshot_digest()
    );
}

#[test]
fn exact_column_receipt_uses_structured_modifier_successor_digest() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let snapshot = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 36),
    )
    .unwrap();

    let location = ColumnTypeModifierLocation::new(
        "public",
        "accounts_2026",
        RelationKind::Table,
        "account_code",
    )
    .unwrap();
    let receipt = snapshot.source_receipt(location).unwrap();

    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.location().column_name(), "account_code");
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/columns/account_code/type-modifier")
    );
}
