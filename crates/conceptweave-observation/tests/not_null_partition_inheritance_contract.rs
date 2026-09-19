use conceptweave_observation::{
    NotNullConstraintObservation, ObservationError, RelationKind,
};

#[test]
fn partitioned_table_not_null_constraint_cannot_be_no_inherit() {
    let error = NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::PartitionedTable,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        true,
        0,
        true,
    )
    .expect_err("PostgreSQL 18 partitioned-table NOT NULL constraints are always inherited");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_no_inherit",
        }
    );
}
