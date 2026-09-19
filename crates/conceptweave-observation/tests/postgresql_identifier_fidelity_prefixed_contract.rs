use conceptweave_observation::{
    ColumnCollationObservation, ColumnExpressionObservation, RelationKind,
};

#[test]
fn prefixed_column_coordinate_identifiers_preserve_quoted_whitespace() {
    let collation = ColumnCollationObservation::uncollatable(
        "  ",
        "\t",
        RelationKind::Table,
        " \t",
    )
    .expect("quoted PostgreSQL identifiers may consist of whitespace");

    assert_eq!(collation.schema_name(), "  ");
    assert_eq!(collation.relation_name(), "\t");
    assert_eq!(collation.column_name(), " \t");

    let expression = ColumnExpressionObservation::no_expression(
        "\t ",
        "  ",
        RelationKind::Table,
        "\t",
    )
    .expect("coordinate identifiers stay lossless outside the core v3 structs");

    assert_eq!(expression.schema_name(), "\t ");
    assert_eq!(expression.relation_name(), "  ");
    assert_eq!(expression.column_name(), "\t");
}

#[test]
fn prefixed_column_coordinate_identifiers_reject_empty_and_code_zero() {
    assert!(
        ColumnCollationObservation::uncollatable("", "relation", RelationKind::Table, "column")
            .is_err()
    );
    assert!(
        ColumnCollationObservation::uncollatable(
            "public",
            "bad\0relation",
            RelationKind::Table,
            "column",
        )
        .is_err()
    );
    assert!(
        ColumnExpressionObservation::no_expression(
            "public",
            "relation",
            RelationKind::Table,
            "bad\0column",
        )
        .is_err()
    );
}

#[test]
fn rendered_expression_text_keeps_nonblank_policy() {
    assert!(
        ColumnExpressionObservation::default_expression(
            "public",
            "relation",
            RelationKind::Table,
            "column",
            "   ",
        )
        .is_err()
    );
}
