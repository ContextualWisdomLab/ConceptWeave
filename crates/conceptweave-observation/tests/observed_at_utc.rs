use conceptweave_observation::{ObservationError, PostgresSchemaSnapshot};

const SNAPSHOT_DIGEST: &str =
    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn snapshot_requires_an_explicit_utc_observation_timestamp() {
    for observed_at_utc in [
        "time",
        "2026-09-02",
        "2026-09-02T12:00:00",
        "2026-09-02T21:00:00+09:00",
    ] {
        let error = PostgresSchemaSnapshot::new(
            "warehouse-primary",
            SNAPSHOT_DIGEST,
            "postgres-introspector/1",
            observed_at_utc,
            Vec::new(),
        )
        .expect_err("non-UTC or malformed observation timestamps must fail closed");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "observed_at_utc"
            }
        );
    }
}

#[test]
fn snapshot_accepts_an_explicit_utc_observation_timestamp() {
    let snapshot = PostgresSchemaSnapshot::new(
        "warehouse-primary",
        SNAPSHOT_DIGEST,
        "postgres-introspector/1",
        "2026-09-02T12:00:00Z",
        Vec::new(),
    )
    .expect("an explicit UTC observation timestamp is valid evidence");

    assert_eq!(snapshot.observed_at_utc(), "2026-09-02T12:00:00Z");
}
