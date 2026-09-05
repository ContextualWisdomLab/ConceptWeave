use conceptweave_observation::{ColumnObservation, PostgresSchemaSnapshot, TableObservation};

mod support;

const ASSERTED_DIGEST: &str =
    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn table(comment: &str) -> TableObservation {
    TableObservation::new(
        "public",
        "event_record",
        vec![ColumnObservation::new(
            "event_key",
            1,
            "uuid",
            false,
            Some(comment.to_owned()),
        )
        .expect("fixture column is valid")],
    )
    .expect("fixture table is valid")
}

#[test]
fn snapshot_digest_changes_when_observed_metadata_changes_even_if_caller_assertion_is_reused() {
    let first = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        ASSERTED_DIGEST,
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![table("first source comment")],
    )
    .expect("first snapshot is structurally valid");
    let changed = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        ASSERTED_DIGEST,
        "postgres_introspector_v1",
        "2026-09-05T03:31:00Z",
        vec![table("changed source comment")],
    )
    .expect("changed snapshot is structurally valid");

    assert_ne!(
        first.snapshot_digest(),
        changed.snapshot_digest(),
        "immutable source identity must be derived from observed metadata, not a reusable caller assertion"
    );
}

#[test]
fn snapshot_digest_is_stable_across_table_input_order_and_provenance_coordinates() {
    let alpha = TableObservation::new("audit", "event_record", Vec::new()).unwrap();
    let beta = TableObservation::new("public", "event_record", Vec::new()).unwrap();

    let first = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        ASSERTED_DIGEST,
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![beta.clone(), alpha.clone()],
    )
    .unwrap();
    let reordered = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_secondary"),
        ASSERTED_DIGEST,
        "postgres_introspector_v2",
        "2026-09-05T04:30:00Z",
        vec![alpha, beta],
    )
    .unwrap();

    assert_eq!(first.snapshot_digest(), reordered.snapshot_digest());
}

#[test]
fn source_receipt_exposes_the_snapshot_verified_digest() {
    let snapshot = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        ASSERTED_DIGEST,
        "postgres_introspector_v1",
        "2026-09-05T03:30:00Z",
        vec![table("source comment")],
    )
    .unwrap();
    let receipt = snapshot
        .source_receipt(
            conceptweave_observation::ObservationLocation::table("public", "event_record").unwrap(),
        )
        .unwrap();

    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}
