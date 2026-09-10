use conceptweave_observation::{
    ColumnObservation, ForeignKeyObservation, ObservationError, PrimaryKeyObservation,
    TableConstraintObservation, TableObservation, UniqueConstraintObservation,
};

fn column(name: &str, ordinal_position: u32, nullable: bool) -> ColumnObservation {
    ColumnObservation::new(name, ordinal_position, "uuid", nullable, None)
        .expect("fixture column is valid")
}

#[test]
fn table_preserves_composite_primary_unique_and_foreign_key_evidence() {
    let primary_key = PrimaryKeyObservation::new(
        "event_identity_pk",
        vec!["tenant_key".to_owned(), "event_key".to_owned()],
    )
    .expect("composite primary key is valid");
    let unique_key = UniqueConstraintObservation::new(
        "event_external_ref_uq",
        vec!["tenant_key".to_owned(), "external_ref".to_owned()],
    )
    .expect("composite unique constraint is valid");
    let foreign_key = ForeignKeyObservation::new(
        "event_account_fk",
        vec!["tenant_key".to_owned(), "account_key".to_owned()],
        "identity",
        "account_record",
        vec!["tenant_key".to_owned(), "account_key".to_owned()],
    )
    .expect("composite foreign key is valid");

    let table = TableObservation::with_constraints(
        "public",
        "event_record",
        vec![
            column("external_ref", 3, false),
            column("tenant_key", 1, false),
            column("account_key", 4, true),
            column("event_key", 2, false),
        ],
        vec![
            TableConstraintObservation::ForeignKey(foreign_key),
            TableConstraintObservation::Unique(unique_key),
            TableConstraintObservation::PrimaryKey(primary_key),
        ],
    )
    .expect("table and constraints are valid");

    let constraint_names: Vec<_> = table
        .constraints()
        .iter()
        .map(TableConstraintObservation::constraint_name)
        .collect();
    assert_eq!(
        constraint_names,
        vec![
            "event_account_fk",
            "event_external_ref_uq",
            "event_identity_pk"
        ]
    );

    let TableConstraintObservation::ForeignKey(observed_fk) = &table.constraints()[0] else {
        panic!("foreign key should sort first by exact constraint name");
    };
    assert_eq!(observed_fk.column_names(), &["tenant_key", "account_key"]);
    assert_eq!(observed_fk.referenced_schema_name(), "identity");
    assert_eq!(observed_fk.referenced_table_name(), "account_record");
    assert_eq!(
        observed_fk.referenced_column_names(),
        &["tenant_key", "account_key"]
    );
    assert!(table.columns()[3].nullable());
}

#[test]
fn table_rejects_constraints_that_reference_unknown_local_columns() {
    let primary_key = PrimaryKeyObservation::new(
        "event_identity_pk",
        vec!["tenant_key".to_owned(), "missing_event_key".to_owned()],
    )
    .expect("constraint shape is valid before table binding");

    let error = TableObservation::with_constraints(
        "public",
        "event_record",
        vec![column("tenant_key", 1, false)],
        vec![TableConstraintObservation::PrimaryKey(primary_key)],
    )
    .expect_err("constraints must bind only observed local columns");

    assert_eq!(
        error,
        ObservationError::UnknownConstraintColumn {
            schema_name: "public".to_owned(),
            table_name: "event_record".to_owned(),
            constraint_name: "event_identity_pk".to_owned(),
            column_name: "missing_event_key".to_owned(),
        }
    );
}

#[test]
fn table_rejects_duplicate_constraint_names() {
    let primary_key =
        PrimaryKeyObservation::new("event_identity_key", vec!["event_key".to_owned()])
            .expect("primary key is valid");
    let unique_key =
        UniqueConstraintObservation::new("event_identity_key", vec!["event_key".to_owned()])
            .expect("unique key is valid");

    let error = TableObservation::with_constraints(
        "public",
        "event_record",
        vec![column("event_key", 1, false)],
        vec![
            TableConstraintObservation::PrimaryKey(primary_key),
            TableConstraintObservation::Unique(unique_key),
        ],
    )
    .expect_err("exact duplicate source constraint names must fail closed");

    assert_eq!(
        error,
        ObservationError::DuplicateConstraintName {
            schema_name: "public".to_owned(),
            table_name: "event_record".to_owned(),
            constraint_name: "event_identity_key".to_owned(),
        }
    );
}

#[test]
fn constraint_constructors_reject_empty_duplicate_and_mismatched_column_sets() {
    let empty = PrimaryKeyObservation::new("event_identity_pk", Vec::new())
        .expect_err("primary keys need at least one source column");
    assert_eq!(
        empty,
        ObservationError::EmptyConstraintColumns {
            constraint_name: "event_identity_pk".to_owned(),
        }
    );

    let duplicate = UniqueConstraintObservation::new(
        "event_identity_uq",
        vec!["event_key".to_owned(), "event_key".to_owned()],
    )
    .expect_err("constraint column coordinates must be unique");
    assert_eq!(
        duplicate,
        ObservationError::DuplicateConstraintColumn {
            constraint_name: "event_identity_uq".to_owned(),
            column_name: "event_key".to_owned(),
        }
    );

    let mismatch = ForeignKeyObservation::new(
        "event_account_fk",
        vec!["tenant_key".to_owned(), "account_key".to_owned()],
        "identity",
        "account_record",
        vec!["account_key".to_owned()],
    )
    .expect_err("foreign key local and referenced arity must match");
    assert_eq!(
        mismatch,
        ObservationError::ForeignKeyArityMismatch {
            constraint_name: "event_account_fk".to_owned(),
            local_column_count: 2,
            referenced_column_count: 1,
        }
    );
}

#[test]
fn constraint_identifiers_reject_blank_source_metadata() {
    let error = ForeignKeyObservation::new(
        "event_account_fk",
        vec!["account_key".to_owned()],
        "\u{2003}",
        "account_record",
        vec!["account_key".to_owned()],
    )
    .expect_err("referenced schema identity must be present");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "referenced_schema_name"
        }
    );

    for result in [
        PrimaryKeyObservation::new(" ", vec!["event_key".to_owned()]).map(|_| ()),
        UniqueConstraintObservation::new("\n", vec!["event_key".to_owned()]).map(|_| ()),
        ForeignKeyObservation::new(
            "\t",
            vec!["event_key".to_owned()],
            "public",
            "event_record",
            vec!["event_key".to_owned()],
        )
        .map(|_| ()),
        ForeignKeyObservation::new(
            "event_parent_fk",
            vec!["event_key".to_owned()],
            "public",
            " ",
            vec!["event_key".to_owned()],
        )
        .map(|_| ()),
        ForeignKeyObservation::new(
            "event_parent_fk",
            vec![" ".to_owned()],
            "public",
            "event_record",
            vec!["event_key".to_owned()],
        )
        .map(|_| ()),
        ForeignKeyObservation::new(
            "event_parent_fk",
            vec!["event_key".to_owned()],
            "public",
            "event_record",
            vec!["event_key".to_owned(), "event_key".to_owned()],
        )
        .map(|_| ()),
    ] {
        assert!(result.is_err());
    }
}
