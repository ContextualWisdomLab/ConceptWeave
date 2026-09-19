use conceptweave_observation::{
    ColumnGenerationObservation, ConstraintDeferrability, ConstraintPeriodObservation,
    ConstraintTimingObservation, RelationKind,
};

#[test]
fn catalog_family_coordinates_preserve_quoted_whitespace_identifiers() {
    let generation = ColumnGenerationObservation::not_generated(
        "  ",
        "\t",
        RelationKind::Table,
        " \t ",
    )
    .expect("generated-column coordinates preserve exact quoted identifiers");
    assert_eq!(generation.schema_name(), "  ");
    assert_eq!(generation.relation_name(), "\t");
    assert_eq!(generation.column_name(), " \t ");

    let timing = ConstraintTimingObservation::new(
        "\t ",
        "  ",
        RelationKind::Table,
        "\t",
        ConstraintDeferrability::NotDeferrable,
    )
    .expect("constraint-timing coordinates preserve exact quoted identifiers");
    assert_eq!(timing.schema_name(), "\t ");
    assert_eq!(timing.relation_name(), "  ");
    assert_eq!(timing.constraint_name(), "\t");

    let period = ConstraintPeriodObservation::new(
        " ",
        "\t ",
        RelationKind::Table,
        "  ",
        false,
    )
    .expect("temporal-constraint coordinates preserve exact quoted identifiers");
    assert_eq!(period.schema_name(), " ");
    assert_eq!(period.relation_name(), "\t ");
    assert_eq!(period.constraint_name(), "  ");
}

#[test]
fn catalog_family_coordinates_reject_code_zero() {
    assert!(
        ColumnGenerationObservation::not_generated(
            "public",
            "bad\0relation",
            RelationKind::Table,
            "column",
        )
        .is_err()
    );
    assert!(
        ConstraintTimingObservation::new(
            "public",
            "relation",
            RelationKind::Table,
            "bad\0constraint",
            ConstraintDeferrability::NotDeferrable,
        )
        .is_err()
    );
    assert!(
        ConstraintPeriodObservation::new(
            "bad\0schema",
            "relation",
            RelationKind::Table,
            "constraint",
            false,
        )
        .is_err()
    );
}
