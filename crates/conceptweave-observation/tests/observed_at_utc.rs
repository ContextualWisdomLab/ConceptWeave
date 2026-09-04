use conceptweave_observation::{ObservationError, PostgresSchemaSnapshot};

mod support;

const SNAPSHOT_DIGEST: &str =
    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn assert_invalid_timestamp(observed_at_utc: &str) {
    let error = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
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

#[test]
fn snapshot_requires_an_explicit_utc_observation_timestamp() {
    for observed_at_utc in [
        "time",
        "2026-09-02",
        "2026-09-02T12:00:00",
        "2026-09-02T21:00:00+09:00",
        "2026-09-02T12:00:00z",
        "2026/09/02T12:00:00Z",
        "2026-09-02T12:00:00.Z",
        "2026-09-02T12:00:00.1xZ",
    ] {
        assert_invalid_timestamp(observed_at_utc);
    }
}

#[test]
fn snapshot_rejects_impossible_calendar_dates_and_clock_values() {
    for observed_at_utc in [
        "2026-00-02T12:00:00Z",
        "2026-13-02T12:00:00Z",
        "2026-09-00T12:00:00Z",
        "2026-04-31T12:00:00Z",
        "2025-02-29T12:00:00Z",
        "2100-02-29T12:00:00Z",
        "2024-02-30T12:00:00Z",
        "2026-09-02T24:00:00Z",
        "2026-09-02T23:60:00Z",
        "2026-09-02T23:59:61Z",
        "2026-09-02T12:00:60Z",
    ] {
        assert_invalid_timestamp(observed_at_utc);
    }
}

#[test]
fn snapshot_accepts_canonical_utc_observation_timestamps() {
    for observed_at_utc in [
        "2026-09-02T12:00:00Z",
        "2026-01-31T23:59:59.123456Z",
        "2026-04-30T00:00:00Z",
        "2025-02-28T00:00:00Z",
        "2024-02-29T00:00:00Z",
        "2000-02-29T00:00:00Z",
        "2024-06-30T23:59:60Z",
    ] {
        let snapshot = PostgresSchemaSnapshot::new(
            &support::resolved_source("warehouse_primary"),
            SNAPSHOT_DIGEST,
            "postgres-introspector/1",
            observed_at_utc,
            Vec::new(),
        )
        .expect("an explicit canonical UTC observation timestamp is valid evidence");

        assert_eq!(snapshot.observed_at_utc(), observed_at_utc);
    }
}
