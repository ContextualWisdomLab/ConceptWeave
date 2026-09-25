use conceptweave_observation::{NotNullConstraintObservation, ObservationError, RelationKind};

fn constraint(
    is_local: bool,
    inheritance_ancestor_count: u16,
) -> Result<NotNullConstraintObservation, ObservationError> {
    NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        is_local,
        inheritance_ancestor_count,
        false,
    )
}

#[test]
fn inherited_not_null_requires_at_least_one_inheritance_ancestor() {
    let error = constraint(false, 0).expect_err(
        "a non-local NOT NULL constraint without an ancestor is not source-representable",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_inheritance_origin",
        }
    );
}

#[test]
fn source_representable_local_and_inherited_origins_remain_accepted() {
    constraint(true, 0).expect("a purely local NOT NULL constraint is source-representable");
    constraint(false, 1).expect("an inherited-only NOT NULL constraint has an ancestor");
    constraint(true, 1).expect("a locally-defined constraint may also be inherited");
}
