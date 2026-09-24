use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, NotNullConstraintObservation,
    ObservationError, PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn constraint_for(
    relation_name: &str,
    constraint_name: &str,
    column_name: &str,
) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        relation_name,
        RelationKind::Table,
        constraint_name,
        column_name,
        true,
        true,
        true,
        0,
        false,
    )
    .expect("NOT NULL constraint fixture is valid")
}

fn constraint(column_name: &str) -> NotNullConstraintObservation {
    constraint_for("metric", "metric_required", column_name)
}

fn relation(
    relation_name: &str,
    column_name: &str,
    nullable: bool,
    standard_constraint_name: Option<&str>,
) -> RelationObservation {
    let relation = RelationObservation::new(
        "public",
        relation_name,
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                column_name,
                1,
                "numeric",
                catalog_type("numeric"),
                nullable,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid");

    match standard_constraint_name {
        Some(constraint_name) => relation
            .with_constraints(vec![TableConstraintObservation::Check(
                CheckConstraintObservation::new(
                    constraint_name,
                    format!("{column_name} > 0"),
                    true,
                    true,
                    false,
                )
                .expect("CHECK constraint fixture is valid"),
            )])
            .expect("relation CHECK fixture is valid"),
        None => relation,
    }
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

#[test]
fn not_null_name_colliding_with_other_constraint_kind_on_same_relation_fails_closed() {
    let relation = relation("metric", "raw_value", false, Some("metric_required"));

    let error = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T09:20:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        vec![constraint("raw_value")],
    )
    .expect_err("all pg_constraint names must be unique within one owning relation");

    assert_eq!(
        error,
        ObservationError::DuplicateConstraintName {
            schema_name: "public".to_owned(),
            table_name: "metric".to_owned(),
            constraint_name: "metric_required".to_owned(),
        }
    );
}

#[test]
fn same_constraint_name_on_different_relations_remains_valid() {
    // Keep the CHECK-only relation nullable so NOT NULL-family completeness does not require an
    // unrelated NOT NULL row there; the sample relation is the sole non-null column in this fixture.
    let metric = relation("metric", "raw_value", true, Some("shared_constraint_name"));
    let sample = relation("sample", "sample_value", false, None);

    PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-14T09:21:00Z",
        vec![metric, sample],
        Vec::new(),
        Vec::new(),
        vec![constraint_for(
            "sample",
            "shared_constraint_name",
            "sample_value",
        )],
    )
    .expect("PostgreSQL permits the same constraint name on different owning relations");
}
