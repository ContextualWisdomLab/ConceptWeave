mod support;

use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError,
    ParentNotNullConstraintCoordinate, PartitionParentRelationCoordinate, PostgresSchemaSnapshotV3,
    QualifiedTypeName, RelationKind, RelationObservation, RelationPartitionObservation,
    RelationPartitionSnapshot,
};
use support::authorized_source;

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
        &authorized_source("warehouse", &["public"]),
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
    .expect_err("independent relation and NOT NULL pg_inherits witnesses must agree");
    assert_field(error, "relation_partition_not_null_parent");
}
