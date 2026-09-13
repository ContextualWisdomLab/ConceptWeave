use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError,
    ParentNotNullConstraintCoordinate, PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind,
    RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn child_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "metric_2026",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "raw_value",
            1,
            "numeric",
            catalog_type("numeric"),
            false,
            None,
        )
        .expect("child column fixture is valid")],
    )
    .expect("child relation fixture is valid")
}

fn child_constraint_with_state(
    parent_relation_name: &str,
    is_local: bool,
    inheritance_ancestor_count: u16,
) -> Result<NotNullConstraintObservation, ObservationError> {
    NotNullConstraintObservation::new(
        "public",
        "metric_2026",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        is_local,
        inheritance_ancestor_count,
        false,
    )?
    .with_parent_constraint(ParentNotNullConstraintCoordinate::new(
        "public",
        parent_relation_name,
        RelationKind::PartitionedTable,
        "metric_raw_value_not_null",
    )?)
}

fn child_constraint(parent_relation_name: &str) -> NotNullConstraintObservation {
    child_constraint_with_state(parent_relation_name, false, 1)
        .expect("partition-child NOT NULL constraint is valid")
}

fn snapshot(constraint: NotNullConstraintObservation) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T07:02:00Z",
        vec![child_relation()],
        Vec::new(),
        Vec::new(),
        vec![constraint],
    )
    .expect("partition-child NOT NULL family is valid")
}

#[test]
fn resolved_partition_parent_constraint_is_governed_identity() {
    let first = snapshot(child_constraint("metric"));
    let second = snapshot(child_constraint("metric_archive"));

    assert_ne!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "conparentid must be resolved to a stable source coordinate before governed hashing"
    );
}

#[test]
fn catalog_oid_is_not_part_of_public_parent_constraint_contract() {
    let parent = ParentNotNullConstraintCoordinate::new(
        "public",
        "metric",
        RelationKind::PartitionedTable,
        "metric_raw_value_not_null",
    )
    .expect("parent constraint coordinate is valid");

    assert_eq!(parent.schema_name(), "public");
    assert_eq!(parent.relation_name(), "metric");
    assert_eq!(parent.relation_kind(), RelationKind::PartitionedTable);
    assert_eq!(parent.constraint_name(), "metric_raw_value_not_null");
}

#[test]
fn parent_constraint_coordinate_requires_partitioned_table_relation_kind() {
    let error = ParentNotNullConstraintCoordinate::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_raw_value_not_null",
    )
    .expect_err("conparentid cannot resolve to a non-partitioned parent relation");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_parent_relation_kind",
        }
    );
}

#[test]
fn partition_parent_link_rejects_locally_defined_child_constraint() {
    let error = child_constraint_with_state("metric", true, 1)
        .expect_err("partition-child conparentid rows cannot remain locally defined");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_parent_locality",
        }
    );
}

#[test]
fn partition_parent_link_requires_exactly_one_inheritance_ancestor() {
    for inheritance_ancestor_count in [0, 2] {
        let error = child_constraint_with_state("metric", false, inheritance_ancestor_count)
            .expect_err("partition-child conparentid rows must have exactly one direct ancestor");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "not_null_constraint_parent_inheritance_ancestor_count",
            }
        );
    }
}
