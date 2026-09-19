use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError,
    ParentNotNullConstraintCoordinate, PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind,
    RelationObservation,
};
use conceptweave_relation_partition::{
    PartitionParentRelationCoordinate, RelationPartitionObservation, RelationPartitionSnapshot,
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

fn relation(name: &str, kind: RelationKind) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![ColumnObservationV3::new(
            "id",
            1,
            "bigint",
            QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
            false,
            None,
        )
        .unwrap()],
    )
    .unwrap()
}

fn base_snapshot(
    child: NotNullConstraintObservation,
) -> PostgresSchemaSnapshotV3 {
    let parent = NotNullConstraintObservation::new(
        "public",
        "events",
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

    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-partition-v1",
        "2026-09-14T13:30:00Z",
        vec![
            relation("events", RelationKind::PartitionedTable),
            relation("events_2026", RelationKind::Table),
        ],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_not_null_constraints(vec![parent, child])
    .unwrap()
}

fn membership() -> Vec<RelationPartitionObservation> {
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
    ]
}

#[test]
fn partition_membership_requires_inherited_parent_not_null_linkage() {
    let unlinked_child = NotNullConstraintObservation::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "id_nn",
        "id",
        true,
        true,
        true,
        0,
        false,
    )
    .unwrap();

    let error = RelationPartitionSnapshot::new(&base_snapshot(unlinked_child), membership())
        .expect_err("a partition must inherit the parent partitioned-table NOT NULL constraint");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_not_null_inheritance",
        }
    );
}

#[test]
fn inherited_parent_not_null_linkage_agrees_with_partition_membership() {
    let linked_child = NotNullConstraintObservation::new(
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
            "events",
            RelationKind::PartitionedTable,
            "id_nn",
        )
        .unwrap(),
    )
    .unwrap()
    .with_partition_parent_relation("public", "events", false)
    .unwrap();

    RelationPartitionSnapshot::new(&base_snapshot(linked_child), membership())
        .expect("a child constraint linked to the exact direct partition parent is coherent");
}
