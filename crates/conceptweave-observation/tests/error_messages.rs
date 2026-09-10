use conceptweave_observation::ObservationError;

#[test]
fn every_observation_error_has_a_stable_operator_message() {
    let cases = [
        (
            ObservationError::InvalidObservationField { field: "field" },
            "invalid observation field: field",
        ),
        (
            ObservationError::InvalidOrdinalPosition,
            "column ordinal position must be positive",
        ),
        (
            ObservationError::DuplicateColumnName {
                schema_name: "public".into(),
                table_name: "events".into(),
                column_name: "event_key".into(),
            },
            "duplicate column observation: public.events.event_key",
        ),
        (
            ObservationError::DuplicateColumnOrdinal {
                schema_name: "public".into(),
                table_name: "events".into(),
                ordinal_position: 1,
            },
            "duplicate column ordinal in public.events: 1",
        ),
        (
            ObservationError::EmptyConstraintColumns {
                constraint_name: "events_pk".into(),
            },
            "constraint has no columns: events_pk",
        ),
        (
            ObservationError::DuplicateConstraintColumn {
                constraint_name: "events_pk".into(),
                column_name: "event_key".into(),
            },
            "duplicate constraint column in events_pk: event_key",
        ),
        (
            ObservationError::DuplicateConstraintName {
                schema_name: "public".into(),
                table_name: "events".into(),
                constraint_name: "events_pk".into(),
            },
            "duplicate constraint observation on public.events: events_pk",
        ),
        (
            ObservationError::UnknownConstraintColumn {
                schema_name: "public".into(),
                table_name: "events".into(),
                constraint_name: "events_pk".into(),
                column_name: "missing_key".into(),
            },
            "constraint events_pk on public.events references unknown local column missing_key",
        ),
        (
            ObservationError::ForeignKeyArityMismatch {
                constraint_name: "events_parent_fk".into(),
                local_column_count: 2,
                referenced_column_count: 1,
            },
            "foreign key events_parent_fk has 2 local columns but 1 referenced columns",
        ),
        (
            ObservationError::DuplicateTableObservation {
                schema_name: "public".into(),
                table_name: "events".into(),
            },
            "duplicate table observation: public.events",
        ),
        (
            ObservationError::UnknownObservationLocation {
                location: "public.events.missing".into(),
            },
            "unobserved source location: public.events.missing",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
