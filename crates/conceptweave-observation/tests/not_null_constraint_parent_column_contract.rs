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
        vec![
            ColumnObservationV3::new(
                "raw_value",
                1,
                "numeric",
                catalog_type("numeric"),
                false,
                None,
            )
            .expect("raw-value column fixture is valid"),
            ColumnObservationV3::new(
                "quality_flag",
                2,
                "numeric",
                catalog_type("numeric"),
                false,
                None,
            )
            .expect("quality-flag column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
}

fn not_null_constraint(
    relation_name: &str,
    relation_kind: RelationKind,
    constraint_name: &str,
    column_name: &str,
    is_local: bool,
    inheritance_ancestor_count: u16,
) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        relation_name,
        relation_kind,
        constraint_name,
        column_name,
        true,
        true,
        is_local,
        inheritance_ancestor_count,
        false,
    )
    .expect("NOT NULL constraint fixture is valid")
}

fn partition_child_constraint(
    constraint_name: &str,
    column_name: &str,
    parent_constraint_name: &str,
) -> NotNullConstraintObservation {
    not_null_constraint(
        "metric_2026",
        RelationKind::Table,
        constraint_name,
        column_name,
        false,
        1,
    )
    .with_parent_constraint(
        ParentNotNullConstraintCoordinate::new(
            "public",
            "metric",
            RelationKind::PartitionedTable,
            parent_constraint_name,
        )
        .expect("parent constraint coordinate is valid"),
    )
    .expect("partition-child parent linkage state is valid")
}

#[test]
fn partition_parent_constraint_must_reference_corresponding_column() {
    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T01:42:00Z",
        vec![
            relation("metric", RelationKind::PartitionedTable),
            relation("metric_2026", RelationKind::Table),
        ],
        Vec::new(),
        Vec::new(),
        vec![
            not_null_constraint(
                "metric",
                RelationKind::PartitionedTable,
                "metric_raw_value_not_null",
                "raw_value",
                true,
                0,
            ),
            not_null_constraint(
                "metric",
                RelationKind::PartitionedTable,
                "metric_quality_flag_not_null",
                "quality_flag",
                true,
                0,
            ),
            partition_child_constraint(
                "metric_raw_value_not_null",
                "raw_value",
                "metric_quality_flag_not_null",
            ),
            partition_child_constraint(
                "metric_quality_flag_not_null",
                "quality_flag",
                "metric_quality_flag_not_null",
            ),
        ],
    )
    .expect_err("conparentid must reference the parent NOT NULL row for the same partition column");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_parent_column",
        }
    );
}
