use conceptweave_observation::{NotNullConstraintObservation, ObservationError, RelationKind};

#[test]
fn no_inherit_not_null_requires_a_purely_local_origin() {
    for (is_local, inheritance_ancestor_count) in [(false, 1), (true, 1)] {
        let error = NotNullConstraintObservation::new(
            "public",
            "metric",
            RelationKind::Table,
            "metric_value_not_null",
            "value",
            true,
            true,
            is_local,
            inheritance_ancestor_count,
            true,
        )
        .expect_err("NO INHERIT cannot coexist with inherited ancestry");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "not_null_constraint_no_inherit_origin",
            }
        );
    }
}

#[test]
fn purely_local_no_inherit_not_null_remains_representable() {
    let observation = NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_value_not_null",
        "value",
        true,
        true,
        true,
        0,
        true,
    )
    .expect("a local ordinary-table NO INHERIT constraint is valid PostgreSQL 18 state");

    assert!(observation.no_inherit());
    assert!(observation.is_local());
    assert_eq!(observation.inheritance_ancestor_count(), 0);
}

#[test]
fn purely_local_foreign_table_no_inherit_not_null_remains_representable() {
    let observation = NotNullConstraintObservation::new(
        "public",
        "remote_metric",
        RelationKind::ForeignTable,
        "remote_metric_value_not_null",
        "value",
        true,
        true,
        true,
        0,
        true,
    )
    .expect("CREATE FOREIGN TABLE supports local NOT NULL NO INHERIT state");

    assert!(observation.no_inherit());
    assert_eq!(observation.relation_kind(), RelationKind::ForeignTable);
}
