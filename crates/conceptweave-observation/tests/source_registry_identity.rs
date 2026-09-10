use conceptweave_observation::{ObservationError, PostgresSchemaSnapshot};

mod support;

fn snapshot_with_source() -> Result<PostgresSchemaSnapshot, ObservationError> {
    PostgresSchemaSnapshot::new(
        &support::resolved_source("grc_readonly_connection"),
        "postgres_introspector_v1",
        "2026-09-03T13:00:00Z",
        Vec::new(),
    )
}

#[test]
fn snapshot_source_connection_key_must_match_the_source_port_registry_identity() {
    assert_eq!(
        snapshot_with_source().unwrap().source_connection_key(),
        "grc_readonly_connection"
    );
}

#[test]
fn snapshot_accepts_a_bounded_multiword_snake_case_registry_identity() {
    let snapshot = snapshot_with_source().expect(
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
