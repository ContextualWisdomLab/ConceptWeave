use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedTypeName, RelationKind, RelationObservation,
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
        .expect("column fixture is valid")],
    )
    .expect("child relation fixture is valid")
}

fn local_child_constraint() -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        "metric_2026",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        true,
        0,
        false,
    )
    .expect("local NOT NULL constraint fixture is valid")
}

#[test]
fn direct_partition_parent_witness_requires_nonblank_coordinates() {
    for (schema_name, relation_name, field) in [
        (
            "",
            "metric",
            "not_null_constraint_partition_parent_schema_name",
        ),
        (
            "public",
            "",
            "not_null_constraint_partition_parent_relation_name",
        ),
    ] {
        let error = local_child_constraint()
            .with_partition_parent_relation(schema_name, relation_name)
            .expect_err("blank pg_inherits relation coordinates must fail closed");

        assert_eq!(error, ObservationError::InvalidObservationField { field });
    }
}

#[test]
fn direct_partition_parent_witness_without_conparentid_is_rejected() {
    let child = local_child_constraint()
        .with_partition_parent_relation("public", "metric")
        .expect("direct parent coordinate is syntactically valid before family validation");

    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T05:42:00Z",
        vec![child_relation()],
        Vec::new(),
        Vec::new(),
        vec![child],
    )
    .expect_err("a pg_inherits witness without nonzero conparentid is contradictory source evidence");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_partition_parent_relation",
        }
    );
}
