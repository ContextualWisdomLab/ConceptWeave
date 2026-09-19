use conceptweave_observation::{
    ColumnObservationV3, QualifiedCollationName, QualifiedOperatorClassName, QualifiedTypeName,
};

#[test]
fn quoted_whitespace_identifiers_are_preserved_byte_for_byte() {
    let qualified_type = QualifiedTypeName::new("  ", "\t")
        .expect("PostgreSQL quoted identifiers may contain whitespace");
    assert_eq!(qualified_type.schema_name(), "  ");
    assert_eq!(qualified_type.type_name(), "\t");

    let collation = QualifiedCollationName::new(" \t", "\t ")
        .expect("quoted collation identifiers preserve exact whitespace");
    assert_eq!(collation.schema_name(), " \t");
    assert_eq!(collation.collation_name(), "\t ");

    let operator_class = QualifiedOperatorClassName::new("\t", "  ")
        .expect("quoted operator-class identifiers preserve exact whitespace");
    assert_eq!(operator_class.schema_name(), "\t");
    assert_eq!(operator_class.operator_class_name(), "  ");

    let column = ColumnObservationV3::new(
        "  ",
        1,
        "text",
        QualifiedTypeName::new("pg_catalog", "text").expect("fixture type is valid"),
        false,
        None,
    )
    .expect("quoted column identifiers may contain whitespace");
    assert_eq!(column.column_name(), "  ");
}

#[test]
fn empty_and_code_zero_identifiers_fail_closed() {
    assert!(QualifiedTypeName::new("", "text").is_err());
    assert!(QualifiedTypeName::new("public", "").is_err());
    assert!(QualifiedTypeName::new("bad\0schema", "text").is_err());
    assert!(QualifiedTypeName::new("public", "bad\0type").is_err());

    assert!(QualifiedCollationName::new("bad\0schema", "default").is_err());
    assert!(QualifiedOperatorClassName::new("pg_catalog", "bad\0ops").is_err());

    assert!(
        ColumnObservationV3::new(
            "bad\0column",
            1,
            "text",
            QualifiedTypeName::new("pg_catalog", "text").expect("fixture type is valid"),
            false,
            None,
        )
        .is_err()
    );
}
