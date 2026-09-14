use conceptweave_observation::{NotNullConstraintObservation, ObservationError, RelationKind};

#[test]
fn not_null_constraint_rejects_not_enforced_source_state() {
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
    .expect_err("PostgreSQL 18 supports NOT ENFORCED only for CHECK and foreign-key constraints");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_enforcement",
        }
    );
}
