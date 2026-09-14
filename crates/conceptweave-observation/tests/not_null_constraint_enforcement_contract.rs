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
    .expect_err("ordinary PostgreSQL 18 tables do not support NOT ENFORCED NOT NULL constraints");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_enforcement",
        }
    );
}

#[test]
fn foreign_table_preserves_not_enforced_not_null() {
    let observation = NotNullConstraintObservation::new(
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
    .expect("PostgreSQL 18 CREATE FOREIGN TABLE permits NOT NULL NOT ENFORCED");

    assert_eq!(observation.relation_kind(), RelationKind::ForeignTable);
    assert!(!observation.enforced());
}
