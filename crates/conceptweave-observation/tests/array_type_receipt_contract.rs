use conceptweave_observation::{
    ArrayTypeLocation, ArrayTypeObservation, EnumObservation, PostgresSchemaSnapshotV3,
    QualifiedTypeName,
};

mod support;

#[test]
fn combined_array_and_timing_constructor_preserves_composition_and_refusal() {
    use conceptweave_observation::{
        ConstraintDeferrability, ConstraintTimingObservation, RelationKind,
    };

    let predecessor = array_aware_snapshot();
    let absent_constraint = ConstraintTimingObservation::new(
        "public",
        "missing",
        RelationKind::Table,
        "missing_key",
        ConstraintDeferrability::NotDeferrable,
    )
    .unwrap();
    for (timings, accepted) in [(vec![], true), (vec![absent_constraint], false)] {
        let combined = PostgresSchemaSnapshotV3::new_with_array_types_and_constraint_timings(
            &support::authorized_source("warehouse_primary", &["public"]),
            predecessor.extractor_revision(),
            predecessor.observed_at_utc(),
            predecessor.relations().to_vec(),
            predecessor.domains().to_vec(),
            predecessor.enums().to_vec(),
            predecessor.array_types().unwrap().to_vec(),
            timings.clone(),
        );
        assert_eq!(combined.is_ok(), accepted);
        assert_eq!(
            combined,
            predecessor
                .clone()
                .with_observed_constraint_timings(timings)
        );
    }
}

fn type_name(schema: &str, name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema, name).expect("qualified type coordinate is valid")
}

fn array_aware_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T15:30:00Z",
        Vec::new(),
        Vec::new(),
        vec![
            EnumObservation::new(
                "public",
                "status",
                vec!["open".to_owned(), "closed".to_owned()],
            )
            .expect("enum fixture is valid"),
        ],
        vec![
            ArrayTypeObservation::new(
                type_name("public", "_status"),
                type_name("public", "status"),
            )
            .expect("array fixture is valid"),
        ],
    )
    .expect("array-aware snapshot is valid")
}

#[test]
fn array_type_receipt_verifies_exact_coordinate_and_public_digest() {
    let snapshot = array_aware_snapshot();
    let location =
        ArrayTypeLocation::new("public", "_status").expect("array-type location is valid");
    let receipt = snapshot
        .array_type_source_receipt(location.clone())
        .expect("observed array type has an exact receipt");

    assert_eq!(receipt.location(), &location);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(
        receipt.connection_policy_binding(),
        snapshot.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), snapshot.observed_at_utc());
}

#[test]
fn unknown_or_unobserved_array_type_coordinate_fails_closed() {
    let snapshot = array_aware_snapshot();
    let unknown = ArrayTypeLocation::new("public", "_missing")
        .expect("unknown location is structurally valid");
    assert!(snapshot.array_type_source_receipt(unknown).is_err());

    let legacy = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T15:30:00Z",
        Vec::new(),
        Vec::new(),
        vec![
            EnumObservation::new(
                "public",
                "status",
                vec!["open".to_owned(), "closed".to_owned()],
            )
            .expect("enum fixture is valid"),
        ],
    )
    .expect("legacy v3 snapshot remains valid");
    let location =
        ArrayTypeLocation::new("public", "_status").expect("array-type location is valid");
    assert!(legacy.array_type_source_receipt(location).is_err());
}

#[test]
fn array_type_location_uses_collision_safe_exact_identifier_escaping() {
    let location = ArrayTypeLocation::new("tenant/a", "_status~v1")
        .expect("location preserves exact identifiers");
    assert_eq!(
        location.canonical_location(),
        "/schemas/tenant~1a/array-types/_status~0v1"
    );
}
