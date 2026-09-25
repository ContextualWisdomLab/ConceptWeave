use conceptweave_observation::{NotNullConstraintObservation, ObservationError, RelationKind};

#[test]
fn ordinary_table_rejects_not_enforced_not_null() {
    let error = NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        false,
        false,
        true,
        0,
        false,
    )
    .expect_err("PostgreSQL 18 NOT NULL constraints are always enforced");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_enforcement",
        }
    );
}

#[test]
fn foreign_table_rejects_not_enforced_not_null() {
    let error = NotNullConstraintObservation::new(
        "remote",
        "metric",
        RelationKind::ForeignTable,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        false,
        true,
        0,
        false,
    )
    .expect_err("PostgreSQL 18 stores foreign-table NOT NULL constraints as enforced");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_enforcement",
        }
    );
}

#[test]
fn foreign_table_rejects_not_valid_not_null() {
    let error = NotNullConstraintObservation::new(
        "remote",
        "metric",
        RelationKind::ForeignTable,
        "metric_raw_value_not_null",
        "raw_value",
        false,
        true,
        true,
        0,
        false,
    )
    .expect_err("PostgreSQL 18 permits NOT VALID on foreign-table CHECK constraints, not NOT NULL");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_foreign_validation",
        }
    );
}
