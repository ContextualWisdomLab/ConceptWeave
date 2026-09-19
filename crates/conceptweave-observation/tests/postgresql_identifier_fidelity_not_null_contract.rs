use conceptweave_observation::{
    NotNullConstraintObservation, ParentNotNullConstraintCoordinate, RelationKind,
};

#[test]
fn not_null_identifiers_preserve_quoted_whitespace() {
    let parent = ParentNotNullConstraintCoordinate::new(
        "  ",
        "\t",
        RelationKind::PartitionedTable,
        " \t",
    )
    .expect("quoted PostgreSQL identifiers may consist of whitespace");
    assert_eq!(parent.schema_name(), "  ");
    assert_eq!(parent.relation_name(), "\t");
    assert_eq!(parent.constraint_name(), " \t");

    let constraint = NotNullConstraintObservation::new(
        "\t ",
        "  ",
        RelationKind::Table,
        "\t",
        " \t ",
        true,
        true,
        true,
        0,
        false,
    )
    .expect("NOT NULL source coordinates preserve quoted whitespace identifiers");
    assert_eq!(constraint.schema_name(), "\t ");
    assert_eq!(constraint.relation_name(), "  ");
    assert_eq!(constraint.constraint_name(), "\t");
    assert_eq!(constraint.column_name(), " \t ");
}

#[test]
fn not_null_identifiers_reject_empty_and_code_zero() {
    assert!(
        ParentNotNullConstraintCoordinate::new(
            "",
            "parent",
            RelationKind::PartitionedTable,
            "constraint",
        )
        .is_err()
    );
    assert!(
        NotNullConstraintObservation::new(
            "public",
            "bad\0relation",
            RelationKind::Table,
            "constraint",
            "column",
            true,
            true,
            true,
            0,
            false,
        )
        .is_err()
    );
    assert!(
        NotNullConstraintObservation::new(
            "public",
            "relation",
            RelationKind::Table,
            "constraint",
            "bad\0column",
            true,
            true,
            true,
            0,
            false,
        )
        .is_err()
    );
}
