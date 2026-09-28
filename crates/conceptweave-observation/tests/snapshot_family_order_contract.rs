use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};

mod support;

type Attach = fn(PostgresSchemaSnapshotV3) -> Result<PostgresSchemaSnapshotV3, ObservationError>;

#[test]
fn successor_families_reject_replay_and_reverse_order() {
    let base = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T05:12:00Z",
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let stages: [(&'static str, Attach, &'static str, &'static str); 6] = [
        (
            "column identity",
            |snapshot| snapshot.with_observed_column_identities(Vec::new()),
            "column_identity_observation_order",
            "column_identity_already_observed",
        ),
        (
            "NOT NULL constraint",
            |snapshot| snapshot.with_observed_not_null_constraints(Vec::new()),
            "not_null_constraint_observation_order",
            "not_null_constraint_already_observed",
        ),
        (
            "constraint timing",
            |snapshot| snapshot.with_observed_constraint_timings(Vec::new()),
            "constraint_timing_observation_order",
            "constraint_timing_observation_order",
        ),
        (
            "constraint period",
            |snapshot| snapshot.with_observed_constraint_periods(Vec::new()),
            "constraint_period_already_observed",
            "constraint_period_already_observed",
        ),
        (
            "foreign-key catalog",
            |snapshot| snapshot.with_observed_foreign_key_catalog(Vec::new()),
            "foreign_key_catalog_already_observed",
            "foreign_key_catalog_already_observed",
        ),
        (
            "collation definition",
            |snapshot| snapshot.with_observed_collation_definitions(Vec::new()),
            "collation_definition_observation_order",
            "collation_definition_observation_order",
        ),
    ];

    for (position, (family, attach, order_error, repeat_error)) in stages.iter().enumerate() {
        let observed = attach(base.clone()).unwrap_or_else(|error| panic!("{family}: {error}"));
        assert_eq!(
            attach(observed),
            Err(ObservationError::InvalidObservationField {
                field: repeat_error,
            }),
            "{family} cannot be attached twice"
        );
        for (later_family, later, _, _) in stages.iter().skip(position + 1) {
            let observed_later =
                later(base.clone()).unwrap_or_else(|error| panic!("{later_family}: {error}"));
            assert_eq!(
                attach(observed_later),
                Err(ObservationError::InvalidObservationField { field: order_error }),
                "{family} cannot follow {later_family}"
            );
        }
    }
}
