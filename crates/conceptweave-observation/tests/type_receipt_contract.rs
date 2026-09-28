use conceptweave_observation::{
    ObservationError, PostgresSchemaSnapshotV3, PostgresTypeKind, QualifiedTypeName,
    TypeKindObservation, TypeOwnerObservation,
};

mod support;

#[test]
fn exact_type_receipt_requires_observed_kind_and_final_owner_identity() {
    let schema = "Sales/~North";
    let type_name = QualifiedTypeName::new(schema, "price/v2").unwrap();
    let authorized = support::authorized_source("warehouse_primary", &[schema]);
    let empty = PostgresSchemaSnapshotV3::new(
        &authorized,
        "extractor-v3",
        "2026-09-26T00:00:00Z",
        vec![],
        vec![],
        vec![],
    )
    .unwrap();
    assert_eq!(
        empty.type_source_receipt(type_name.clone()),
        Err(ObservationError::UnknownObservationLocation {
            location: "/schemas/Sales~1~0North/types/price~1v2".to_owned()
        })
    );

    let observed = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &authorized,
        "extractor-v3",
        "2026-09-26T00:00:00Z",
        vec![],
        vec![],
        vec![],
        vec![TypeKindObservation::plain(type_name.clone(), PostgresTypeKind::Base).unwrap()],
    )
    .unwrap();
    let before_owner = observed.type_source_receipt(type_name.clone()).unwrap();
    assert_eq!(
        before_owner.canonical_location(),
        "/schemas/Sales~1~0North/types/price~1v2"
    );
    assert_eq!(before_owner.source_digest(), observed.snapshot_digest());

    let owned = observed
        .with_observed_type_owners(vec![
            TypeOwnerObservation::new(type_name.clone(), 42, "owner").unwrap(),
        ])
        .unwrap();
    let receipt = owned.type_source_receipt(type_name.clone()).unwrap();
    assert_ne!(before_owner.source_digest(), receipt.source_digest());
    assert_eq!(receipt.source_digest(), owned.snapshot_digest());
    assert_eq!(receipt.type_name(), &type_name);
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(
        receipt.connection_policy_binding(),
        owned.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), owned.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), owned.observed_at_utc());
    assert!(matches!(
        owned.type_source_receipt(QualifiedTypeName::new(schema, "other").unwrap()),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
    assert!(matches!(
        owned.type_source_receipt(QualifiedTypeName::new("other", "price/v2").unwrap()),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
}
