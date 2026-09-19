use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservation, ForeignKeyObservation, ObservationLocation,
    PrimaryKeyObservation, TableObservation, UniqueConstraintObservation,
};

#[test]
fn legacy_model_preserves_quoted_whitespace_identifiers() {
    let column = ColumnObservation::new("  ", 1, "text", false, None)
        .expect("quoted column identifiers may contain whitespace");
    assert_eq!(column.column_name(), "  ");

    let primary_key = PrimaryKeyObservation::new("\t", vec!["  ".to_owned()])
        .expect("quoted constraint and column identifiers preserve whitespace");
    assert_eq!(primary_key.constraint_name(), "\t");
    assert_eq!(primary_key.column_names(), &["  ".to_owned()]);

    let unique = UniqueConstraintObservation::new("  ", vec!["\t".to_owned()])
        .expect("quoted unique-constraint identifiers preserve whitespace");
    assert_eq!(unique.constraint_name(), "  ");
    assert_eq!(unique.column_names(), &["\t".to_owned()]);

    let check = CheckConstraintObservation::new(" \t", "VALUE IS NOT NULL", true, true, false)
        .expect("quoted check-constraint identifiers preserve whitespace");
    assert_eq!(check.constraint_name(), " \t");

    let foreign_key = ForeignKeyObservation::new(
        "\t ",
        vec!["  ".to_owned()],
        " \t",
        "\t ",
        vec!["\t".to_owned()],
    )
    .expect("quoted foreign-key coordinates preserve whitespace");
    assert_eq!(foreign_key.constraint_name(), "\t ");
    assert_eq!(foreign_key.referenced_schema_name(), " \t");
    assert_eq!(foreign_key.referenced_table_name(), "\t ");
    assert_eq!(foreign_key.column_names(), &["  ".to_owned()]);
    assert_eq!(foreign_key.referenced_column_names(), &["\t".to_owned()]);

    let table = TableObservation::new(
        "  ",
        "\t",
        vec![ColumnObservation::new(" \t", 1, "text", false, None)
            .expect("fixture column is valid")],
    )
    .expect("quoted schema/table identifiers preserve whitespace");
    assert_eq!(table.schema_name(), "  ");
    assert_eq!(table.table_name(), "\t");

    let location = ObservationLocation::column(" \t", "\t ", "  ")
        .expect("quoted evidence coordinates preserve whitespace");
    assert_eq!(location.schema_name(), " \t");
    assert_eq!(location.table_name(), "\t ");
    assert_eq!(location.column_name(), Some("  "));
}

#[test]
fn legacy_model_rejects_empty_and_code_zero_identifiers() {
    assert!(ColumnObservation::new("", 1, "text", false, None).is_err());
    assert!(ColumnObservation::new("bad\0column", 1, "text", false, None).is_err());
    assert!(PrimaryKeyObservation::new("bad\0constraint", vec!["id".to_owned()]).is_err());
    assert!(PrimaryKeyObservation::new("pk", vec!["bad\0column".to_owned()]).is_err());
    assert!(
        ForeignKeyObservation::new(
            "fk",
            vec!["id".to_owned()],
            "bad\0schema",
            "parent",
            vec!["id".to_owned()],
        )
        .is_err()
    );
    assert!(TableObservation::new("public", "bad\0table", vec![]).is_err());
    assert!(ObservationLocation::constraint("public", "table", "bad\0constraint").is_err());
}

#[test]
fn legacy_rendered_text_keeps_nonblank_policy() {
    assert!(ColumnObservation::new("column", 1, "   ", false, None).is_err());
    assert!(CheckConstraintObservation::new("check_name", "\t", true, true, false).is_err());
}
