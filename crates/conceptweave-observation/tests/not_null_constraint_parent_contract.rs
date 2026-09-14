use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError,
    ParentNotNullConstraintCoordinate, PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind,
    RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn relation(relation_name: &str, relation_kind: RelationKind) -> RelationObservation {
    RelationObservation::new(
        "public",
        relation_name,
        relation_kind,
        vec![ColumnObservationV3::new(
            "raw_value",
            1,
            "numeric",
            catalog_type("numeric"),
            false,
            None,
        )
        .expect("column fixture is valid")],
    )
    .expect("relation fixture is valid")
}

fn child_relation() -> RelationObservation {
    relation("metric_2026", RelationKind::Table)
}

fn parent_relation(parent_relation_name: &str) -> RelationObservation {
    relation(parent_relation_name, RelationKind::PartitionedTable)
}

fn parent_constraint(parent_relation_name: &str) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        parent_relation_name,
        RelationKind::PartitionedTable,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        true,
        0,
        false,
    )
    .expect("partition-parent NOT NULL constraint is valid")
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
    )?)?
    .with_partition_parent_relation("public", parent_relation_name, false)
}

fn child_constraint(parent_relation_name: &str) -> NotNullConstraintObservation {
    child_constraint_with_state(parent_relation_name, false, 1)
        .expect("partition-child NOT NULL constraint is valid")
}

fn partitioned_constraint_with_parent(
    relation_name: &str,
    parent_relation_name: &str,
) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        relation_name,
        RelationKind::PartitionedTable,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        false,
        1,
        false,
    )
    .expect("partitioned child NOT NULL constraint is valid before parent attachment")
    .with_parent_constraint(
        ParentNotNullConstraintCoordinate::new(
            "public",
            parent_relation_name,
            RelationKind::PartitionedTable,
            "metric_raw_value_not_null",
        )
        .expect("partition parent coordinate is valid"),
    )
    .expect("non-self parent coordinate can be attached before family validation")
    .with_partition_parent_relation("public", parent_relation_name, false)
    .expect("direct partition-parent witness is valid and not detach-pending")
}

fn snapshot(
    parent_relation_name: &str,
    constraint: NotNullConstraintObservation,
) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T07:02:00Z",
        vec![parent_relation(parent_relation_name), child_relation()],
        Vec::new(),
        Vec::new(),
        vec![parent_constraint(parent_relation_name), constraint],
    )
    .expect("partition-child NOT NULL family is valid")
}

#[test]
fn resolved_partition_parent_constraint_is_governed_identity() {
    let first = snapshot("metric", child_constraint("metric"));
    let second = snapshot("metric_archive", child_constraint("metric_archive"));

    assert_ne!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "conparentid must be resolved to a stable source coordinate before governed hashing"
    );
}

#[test]
fn unresolved_partition_parent_constraint_is_rejected() {
    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T07:02:00Z",
        vec![child_relation()],
        Vec::new(),
        Vec::new(),
        vec![child_constraint("metric")],
    )
    .expect_err("conparentid must resolve to an observed parent NOT NULL constraint row");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_parent_coordinate",
        }
    );
}

#[test]
fn cyclic_partition_parent_constraints_are_rejected() {
    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T04:42:00Z",
        vec![parent_relation("metric_a"), parent_relation("metric_b")],
        Vec::new(),
        Vec::new(),
        vec![
            partitioned_constraint_with_parent("metric_a", "metric_b"),
            partitioned_constraint_with_parent("metric_b", "metric_a"),
        ],
    )
    .expect_err("a declarative partition parent-constraint graph cannot contain a cycle");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_parent_cycle",
        }
    );
}

#[test]
fn partition_parent_constraint_requires_direct_relation_witness() {
    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T05:42:00Z",
        vec![parent_relation("metric"), child_relation()],
        Vec::new(),
        Vec::new(),
        vec![
            parent_constraint("metric"),
            NotNullConstraintObservation::new(
                "public",
                "metric_2026",
                RelationKind::Table,
                "metric_raw_value_not_null",
                "raw_value",
                true,
                true,
                false,
                1,
                false,
            )
            .expect("partition-child NOT NULL constraint is valid before parent attachment")
            .with_parent_constraint(
                ParentNotNullConstraintCoordinate::new(
                    "public",
                    "metric",
                    RelationKind::PartitionedTable,
                    "metric_raw_value_not_null",
                )
                .expect("partition parent coordinate is valid"),
            )
            .expect("conparentid attachment remains structurally valid before family validation"),
        ],
    )
    .expect_err(
        "conparentid cannot establish that the referenced partitioned table is the child's direct pg_inherits parent",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_partition_parent_relation",
        }
    );
}

#[test]
fn mismatched_partition_parent_relation_witness_is_rejected() {
    let child = child_constraint("metric")
        .with_partition_parent_relation("public", "metric_archive", false)
        .expect("the raw pg_inherits coordinate is syntactically valid before family validation");
    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T05:42:00Z",
        vec![
            parent_relation("metric"),
            parent_relation("metric_archive"),
            child_relation(),
        ],
        Vec::new(),
        Vec::new(),
        vec![
            parent_constraint("metric"),
            parent_constraint("metric_archive"),
            child,
        ],
    )
    .expect_err("the pg_inherits direct parent must match the relation owning conparentid");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_partition_parent_relation",
        }
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

#[test]
fn partition_parent_link_rejects_no_inherit_child_constraint() {
    let child = NotNullConstraintObservation::new(
        "public",
        "metric_2026",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        false,
        1,
        true,
    )
    .expect("ordinary-table NO INHERIT is valid before partition-parent linkage is attached");
    let parent = ParentNotNullConstraintCoordinate::new(
        "public",
        "metric",
        RelationKind::PartitionedTable,
        "metric_raw_value_not_null",
    )
    .expect("partition parent coordinate is valid");

    let error = child
        .with_parent_constraint(parent)
        .expect_err("a partition-inherited NOT NULL constraint cannot be NO INHERIT");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_parent_no_inherit",
        }
    );
}
