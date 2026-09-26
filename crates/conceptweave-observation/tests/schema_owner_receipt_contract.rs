use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, SchemaOwnerLocation, SchemaOwnerObservation,
};

mod support;

#[test]
fn empty_schema_owner_has_an_exact_receipt_only_after_observation() {
    let schema = "Sales/~North";
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &[schema]),
        "postgres_introspector_v3",
        "2026-09-26T00:00:00Z",
        vec![],
        vec![],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_type_owners(vec![])
    .unwrap();
    let location = SchemaOwnerLocation::new(schema).unwrap();
    assert_eq!(location.canonical_location(), "/schemas/Sales~1~0North");
    assert!(matches!(
        snapshot.schema_owner_source_receipt(location.clone()),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));

    let snapshot = snapshot
        .with_observed_schema_owners(vec![
            SchemaOwnerObservation::new(schema, 42, "fixture_owner").unwrap(),
        ])
        .unwrap();
    let receipt = snapshot
        .schema_owner_source_receipt(location.clone())
        .unwrap();
    assert_eq!(receipt.location(), &location);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(
        receipt.connection_policy_binding(),
        snapshot.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), snapshot.observed_at_utc());
    assert_eq!(
        snapshot.schema_owner_source_receipt(SchemaOwnerLocation::new("other/~schema").unwrap()),
        Err(ObservationError::UnknownObservationLocation {
            location: "/schemas/other~1~0schema".to_owned()
        })
    );
    assert!(SchemaOwnerLocation::new("").is_err());
    assert!(SchemaOwnerLocation::new("bad\0schema").is_err());
}
