use conceptweave_observation::{ObservationError, PostgresSchemaSnapshot};

const SNAPSHOT_DIGEST: &str =
    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn snapshot_with_source(
    source_connection_key: &str,
) -> Result<PostgresSchemaSnapshot, ObservationError> {
    PostgresSchemaSnapshot::new(
        source_connection_key,
        SNAPSHOT_DIGEST,
        "postgres_introspector_v1",
        "2026-09-03T13:00:00Z",
        Vec::new(),
    )
}

#[test]
fn snapshot_source_connection_key_must_match_the_source_port_registry_identity() {
    for source_connection_key in [
        "postgres://reader:secret@example.invalid/database",
        "host=example.invalid password=secret",
        "warehouse",
        "Warehouse_primary",
        "warehouse-primary",
        "warehouse__primary",
        "_warehouse_primary",
        "warehouse_primary_",
    ] {
        assert_eq!(
            snapshot_with_source(source_connection_key),
            Err(ObservationError::InvalidObservationField {
                field: "source_connection_key",
            }),
            "immutable observation provenance must not bypass the source-port registry-key boundary: {source_connection_key}"
        );
    }

    let oversized_key = format!("source_{}", "a".repeat(122));
    assert_eq!(oversized_key.len(), 129);
    assert_eq!(
        snapshot_with_source(&oversized_key),
        Err(ObservationError::InvalidObservationField {
            field: "source_connection_key",
        })
    );
}

#[test]
fn snapshot_accepts_a_bounded_multiword_snake_case_registry_identity() {
    let snapshot = snapshot_with_source("grc_readonly_connection").expect(
        "the immutable snapshot accepts the same opaque registry identity as the source port",
    );

    assert_eq!(snapshot.source_connection_key(), "grc_readonly_connection");
    let table_location =
        conceptweave_observation::ObservationLocation::table("public", "event_record")
            .expect("location shape is valid");
    assert_eq!(
        snapshot.source_receipt(table_location),
        Err(ObservationError::UnknownObservationLocation {
            location: "/schemas/public/tables/event_record".to_owned(),
        })
    );
}
