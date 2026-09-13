use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn constraint(column_name: &str) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_required",
        column_name,
        true,
        true,
        true,
        0,
        false,
    )
    .expect("NOT NULL constraint fixture is valid")
}

#[test]
fn duplicate_not_null_constraint_name_on_one_relation_fails_closed() {
    let relation = RelationObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "raw_value",
                1,
                "numeric",
                catalog_type("numeric"),
                false,
                None,
            )
            .expect("first column fixture is valid"),
            ColumnObservationV3::new(
                "normalized_value",
                2,
                "numeric",
                catalog_type("numeric"),
                false,
                None,
            )
            .expect("second column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid");

    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T07:10:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        vec![constraint("raw_value"), constraint("normalized_value")],
    )
    .expect_err("constraint names must remain unique within the owning relation");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_name",
        }
    );
}
