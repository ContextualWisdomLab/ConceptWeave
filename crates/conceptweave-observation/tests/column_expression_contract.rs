use conceptweave_observation::{
    ColumnExpressionObservation, ColumnGenerationObservation, ColumnObservationV3, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn two_column_relation() -> RelationObservation {
    RelationObservation::new(
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
            .expect("base-column fixture is valid"),
            ColumnObservationV3::new(
                "value_normalized",
                2,
                "numeric",
                catalog_type("numeric"),
                false,
                None,
            )
            .expect("generated-column fixture is valid"),
        ],
    )
    .expect("metric relation fixture is valid")
}

fn ordinary(column_name: &str) -> ColumnGenerationObservation {
    ColumnGenerationObservation::not_generated(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("ordinary-column generation evidence is valid")
}

fn stored(column_name: &str) -> ColumnGenerationObservation {
    ColumnGenerationObservation::stored(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("stored generation evidence is valid")
}

fn no_expression(column_name: &str) -> ColumnExpressionObservation {
    ColumnExpressionObservation::no_expression(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("explicit no-expression evidence is valid")
}

fn default_expression(column_name: &str, expression: &str) -> ColumnExpressionObservation {
    ColumnExpressionObservation::default_expression(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
        expression,
    )
    .expect("default-expression evidence is valid")
}

fn generation_expression(column_name: &str, expression: &str) -> ColumnExpressionObservation {
    ColumnExpressionObservation::generation_expression(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
        expression,
    )
    .expect("generation-expression evidence is valid")
}

fn generation_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_column_generations(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T03:20:00Z",
        vec![two_column_relation()],
        Vec::new(),
        Vec::new(),
        vec![ordinary("raw_value"), stored("value_normalized")],
    )
    .expect("generation family is internally valid")
}

fn expression_snapshot(
    expressions: Vec<ColumnExpressionObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    generation_snapshot().with_observed_column_expressions(expressions)
}

#[test]
fn distinct_default_expressions_have_distinct_governed_identity() {
    let first = expression_snapshot(vec![
        default_expression("raw_value", "0::numeric"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect("first expression family is valid");
    let second = expression_snapshot(vec![
        default_expression("raw_value", "1::numeric"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect("second expression family is valid");

    assert_ne!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "a changed PostgreSQL column default changes future write semantics and must change governed identity"
    );
}

#[test]
fn distinct_generation_expressions_have_distinct_governed_identity() {
    let first = expression_snapshot(vec![
        no_expression("raw_value"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect("first generation expression is valid");
    let second = expression_snapshot(vec![
        no_expression("raw_value"),
        generation_expression("value_normalized", "(raw_value / 1000::numeric)"),
    ])
    .expect("second generation expression is valid");

    assert_ne!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "SET EXPRESSION AS can change generated-column behavior without changing attgenerated"
    );
}

#[test]
fn observed_no_expression_is_distinct_from_unobserved_expression_family() {
    let unobserved = generation_snapshot();
    let observed = expression_snapshot(vec![
        no_expression("raw_value"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect("observed expression family is valid");

    assert_ne!(unobserved.snapshot_digest(), observed.snapshot_digest());
    assert_eq!(
        observed
            .column_expressions()
            .expect("observed expression family remains queryable")
            .len(),
        2
    );
}

#[test]
fn observed_expression_family_must_cover_every_bounded_column() {
    let error = expression_snapshot(vec![default_expression("raw_value", "0::numeric")])
        .expect_err("a claimed expression family missing value_normalized must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_expression_completeness",
        }
    );
}

#[test]
fn duplicate_column_expression_coordinate_fails_closed() {
    let error = expression_snapshot(vec![
        no_expression("raw_value"),
        default_expression("raw_value", "0::numeric"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect_err("one exact column cannot carry two pg_attrdef expression states");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_expression_coordinate",
        }
    );
}

#[test]
fn column_expression_input_order_does_not_change_identity() {
    let first = expression_snapshot(vec![
        default_expression("raw_value", "0::numeric"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect("first ordering is valid");
    let second = expression_snapshot(vec![
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
        default_expression("raw_value", "0::numeric"),
    ])
    .expect("second ordering is valid");

    assert_eq!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn generation_mode_and_expression_kind_must_agree() {
    let generated_without_expression = expression_snapshot(vec![
        no_expression("raw_value"),
        no_expression("value_normalized"),
    ])
    .expect_err("an observed generated column requires its generation expression");
    assert_eq!(
        generated_without_expression,
        ObservationError::InvalidObservationField {
            field: "column_expression_generation",
        }
    );

    let default_on_generated = expression_snapshot(vec![
        no_expression("raw_value"),
        default_expression("value_normalized", "0::numeric"),
    ])
    .expect_err("a generated column cannot carry ordinary default-expression evidence");
    assert_eq!(
        default_on_generated,
        ObservationError::InvalidObservationField {
            field: "column_expression_generation",
        }
    );

    let generation_on_ordinary = expression_snapshot(vec![
        generation_expression("raw_value", "1::numeric"),
        generation_expression("value_normalized", "(raw_value / 100::numeric)"),
    ])
    .expect_err("an ordinary column cannot carry generation-expression evidence");
    assert_eq!(
        generation_on_ordinary,
        ObservationError::InvalidObservationField {
            field: "column_expression_generation",
        }
    );
}
