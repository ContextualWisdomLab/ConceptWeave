use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError,
    ParentNotNullConstraintCoordinate, PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind,
    RelationObservation,
};
use conceptweave_relation_partition::{
    PartitionParentRelationCoordinate, RelationPartitionLocation, RelationPartitionObservation,
    RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "warehouse"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "warehouse").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "warehouse"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        let request_budget = resource_envelope.request_budget();
        let limits = resource_envelope.limits();
        source_connection.source_connection_key() == "warehouse"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && request_budget.max_schema_count() <= 1
            && request_budget.max_schema_bytes() <= 256
            && limits.operation_timeout_ms() <= 1_000
            && limits.statement_timeout_ms() <= 1_000
            && limits.max_rows() <= 10
            && limits.max_bytes() <= 1_024
            && limits.max_concurrent_queries() <= 1
    }
}

fn authorized_source() -> AuthorizedObservationRequest {
    ObservationRequest::new(
        "warehouse",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn relation(name: &str, kind: RelationKind, nullable: bool) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![ColumnObservationV3::new(
            "id",
            1,
            "bigint",
            QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
            nullable,
            None,
        )
        .unwrap()],
    )
    .unwrap()
}

fn snapshot(relations: Vec<RelationObservation>) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-partition-v1",
        "2026-09-14T12:00:00Z",
        relations,
        vec![],
        vec![],
    )
    .unwrap()
}

fn assert_field(error: ObservationError, expected: &'static str) {
    assert_eq!(
        error,
        ObservationError::InvalidObservationField { field: expected }
    );
}

#[test]
fn declarative_partition_membership_changes_governed_identity() {
    let base = snapshot(vec![
        relation("events", RelationKind::PartitionedTable, true),
        relation("events_2026", RelationKind::Table, true),
    ]);

    let ordinary = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "events",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::non_partition(
                "public",
                "events_2026",
                RelationKind::Table,
            )
            .unwrap(),
        ],
    )
    .unwrap();

    let partitioned = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "events",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();

    assert_ne!(ordinary.snapshot_digest(), partitioned.snapshot_digest());
}

#[test]
fn complete_family_is_required_and_input_order_is_canonical() {
    let base = snapshot(vec![
        relation("events", RelationKind::PartitionedTable, true),
        relation("events_2026", RelationKind::Table, true),
    ]);

    let incomplete = RelationPartitionSnapshot::new(
        &base,
        vec![RelationPartitionObservation::non_partition(
            "public",
            "events",
            RelationKind::PartitionedTable,
        )
        .unwrap()],
    )
    .expect_err("missing relispartition evidence must fail closed");
    assert_field(incomplete, "relation_partition_completeness");

    let first = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events").unwrap(),
                false,
            )
            .unwrap(),
            RelationPartitionObservation::non_partition(
                "public",
                "events",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let second = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "events",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn partition_parent_must_resolve_to_observed_partitioned_table() {
    let missing_parent = snapshot(vec![relation("events_2026", RelationKind::Table, true)]);
    let error = RelationPartitionSnapshot::new(
        &missing_parent,
        vec![RelationPartitionObservation::partition(
            "public",
            "events_2026",
            RelationKind::Table,
            PartitionParentRelationCoordinate::new("public", "events").unwrap(),
            false,
        )
        .unwrap()],
    )
    .expect_err("an unresolved declarative-partition parent must fail closed");
    assert_field(error, "relation_partition_parent_coordinate");

    let wrong_kind = snapshot(vec![
        relation("events", RelationKind::Table, true),
        relation("events_2026", RelationKind::Table, true),
    ]);
    let error = RelationPartitionSnapshot::new(
        &wrong_kind,
        vec![
            RelationPartitionObservation::non_partition("public", "events", RelationKind::Table)
                .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .expect_err("a declarative partition must resolve to a partitioned-table parent");
    assert_field(error, "relation_partition_parent_kind");
}

#[test]
fn detach_pending_membership_fails_closed_before_identity() {
    let error = RelationPartitionObservation::partition(
        "public",
        "events_2026",
        RelationKind::Table,
        PartitionParentRelationCoordinate::new("public", "events").unwrap(),
        true,
    )
    .expect_err("detach-pending pg_inherits state must not look stably attached");
    assert_field(error, "relation_partition_detach_pending");
}

#[test]
fn declarative_partition_parent_graph_must_be_acyclic() {
    let base = snapshot(vec![
        relation("events_a", RelationKind::PartitionedTable, true),
        relation("events_b", RelationKind::PartitionedTable, true),
    ]);
    let error = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::partition(
                "public",
                "events_a",
                RelationKind::PartitionedTable,
                PartitionParentRelationCoordinate::new("public", "events_b").unwrap(),
                false,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_b",
                RelationKind::PartitionedTable,
                PartitionParentRelationCoordinate::new("public", "events_a").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .expect_err("a declarative partition hierarchy cannot contain a parent cycle");
    assert_field(error, "relation_partition_cycle");
}

#[test]
fn relation_membership_must_agree_with_not_null_partition_witness() {
    let base = snapshot(vec![
        relation("events_a", RelationKind::PartitionedTable, false),
        relation("events_b", RelationKind::PartitionedTable, false),
        relation("events_2026", RelationKind::Table, false),
    ]);

    let parent_a = NotNullConstraintObservation::new(
        "public",
        "events_a",
        RelationKind::PartitionedTable,
        "id_nn",
        "id",
        true,
        true,
        true,
        0,
        false,
    )
    .unwrap();
    let parent_b = NotNullConstraintObservation::new(
        "public",
        "events_b",
        RelationKind::PartitionedTable,
        "id_nn",
        "id",
        true,
        true,
        true,
        0,
        false,
    )
    .unwrap();
    let child = NotNullConstraintObservation::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "id_nn",
        "id",
        true,
        true,
        false,
        1,
        false,
    )
    .unwrap()
    .with_parent_constraint(
        ParentNotNullConstraintCoordinate::new(
            "public",
            "events_b",
            RelationKind::PartitionedTable,
            "id_nn",
        )
        .unwrap(),
    )
    .unwrap()
    .with_partition_parent_relation("public", "events_b", false)
    .unwrap();

    let base = base
        .with_observed_not_null_constraints(vec![parent_a, parent_b, child])
        .unwrap();

    let error = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "events_a",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::non_partition(
                "public",
                "events_b",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events_a").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .expect_err("independent relation and NOT NULL partition witnesses must agree");
    assert_field(error, "relation_partition_not_null_parent");
}

#[test]
fn receipt_is_bound_to_exact_observed_relation_coordinate() {
    let base = snapshot(vec![relation("events", RelationKind::PartitionedTable, true)]);
    let governed = RelationPartitionSnapshot::new(
        &base,
        vec![RelationPartitionObservation::non_partition(
            "public",
            "events",
            RelationKind::PartitionedTable,
        )
        .unwrap()],
    )
    .unwrap();

    let receipt = governed
        .source_receipt(
            RelationPartitionLocation::new("public", "events", RelationKind::PartitionedTable)
                .unwrap(),
        )
        .unwrap();
    assert_eq!(receipt.source_id(), "warehouse");
    assert_eq!(receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(receipt.source_digest(), governed.snapshot_digest());
    assert_eq!(receipt.extractor_revision(), "extractor-relation-partition-v1");
    assert_eq!(receipt.observed_at_utc(), "2026-09-14T12:00:00Z");

    let unknown = governed
        .source_receipt(
            RelationPartitionLocation::new("public", "missing", RelationKind::Table).unwrap(),
        )
        .expect_err("receipts must not be minted for unobserved coordinates");
    assert_eq!(
        unknown,
        ObservationError::UnknownObservationLocation {
            location: "/schemas/public/relations/table/missing/partition-membership".to_owned(),
        }
    );
}
