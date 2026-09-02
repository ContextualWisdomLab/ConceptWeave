use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservation, ObservationError, TableConstraintObservation,
    TableObservation,
};

fn quantity_column() -> ColumnObservation {
    ColumnObservation::new("quantity_count", 1, "integer", false, None)
        .expect("column metadata is valid")
}

#[test]
fn check_constraint_preserves_exact_definition_and_postgresql_18_status_flags() {
    let check = CheckConstraintObservation::new(
        "order_quantity_positive",
        "CHECK ((quantity_count > 0))",
        true,
        false,
        true,
    )
    .expect("check metadata is valid");

    assert_eq!(check.constraint_name(), "order_quantity_positive");
    assert_eq!(check.definition(), "CHECK ((quantity_count > 0))");
    assert!(check.validated());
    assert!(!check.enforced());
    assert!(check.no_inherit());
}

#[test]
fn check_constraint_definition_must_be_observed_not_blank() {
    let error =
        CheckConstraintObservation::new("order_quantity_positive", " \t\n ", true, true, false)
            .expect_err("blank source definition must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "check_definition"
        }
    );
}

#[test]
fn table_retains_check_constraint_without_inventing_expression_column_coordinates() {
    let check = CheckConstraintObservation::new(
        "order_quantity_positive",
        "CHECK ((quantity_count > 0))",
        true,
        true,
        false,
    )
    .expect("check metadata is valid");
    let table = TableObservation::with_constraints(
        "sales_data",
        "order_record",
        vec![quantity_column()],
        vec![TableConstraintObservation::Check(check)],
    )
    .expect("table observation accepts exact check evidence");

    let observed = match &table.constraints()[0] {
        TableConstraintObservation::Check(check) => check,
        other => panic!("expected check constraint, observed {other:?}"),
    };
    assert_eq!(observed.definition(), "CHECK ((quantity_count > 0))");
}
