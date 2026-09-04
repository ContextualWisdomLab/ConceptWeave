use conceptweave_observation::{
    ColumnObservation, ForeignKeyObservation, ObservationError, ObservationLocation,
    ObservationLocationKind, PostgresSchemaSnapshot, TableConstraintObservation, TableObservation,
};

mod support;

const SNAPSHOT_DIGEST: &str =
    "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn snapshot() -> PostgresSchemaSnapshot {
    let foreign_key = ForeignKeyObservation::new(
        "Order/Account~FK",
        vec!["Account/Key".to_owned()],
        "Identity",
        "Account~Record",
        vec!["Account/Key".to_owned()],
    )
    .expect("foreign key fixture is valid");
    let table = TableObservation::with_constraints(
        "Sales/~North",
        "Order/Line",
        vec![
            ColumnObservation::new("Order~Key", 1, "uuid", false, None)
                .expect("primary column fixture is valid"),
            ColumnObservation::new("Account/Key", 2, "uuid", true, None)
                .expect("foreign-key column fixture is valid"),
        ],
        vec![TableConstraintObservation::ForeignKey(foreign_key)],
    )
    .expect("table fixture is valid");

    PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_source"),
        SNAPSHOT_DIGEST,
        "catalog-v1",
        "2026-09-02T06:00:00Z",
        vec![table],
    )
    .expect("snapshot fixture is valid")
}

#[test]
fn snapshot_issues_exact_evidence_receipt_for_observed_column() {
    let location = ObservationLocation::column("Sales/~North", "Order/Line", "Account/Key")
        .expect("location fixture is valid");

    let receipt = snapshot()
        .source_receipt(location)
        .expect("observed location can be receipted");

    assert_eq!(receipt.source_id(), "warehouse_source");
    assert_eq!(receipt.source_digest(), SNAPSHOT_DIGEST);
    assert_eq!(receipt.extractor_revision(), "catalog-v1");
    assert_eq!(receipt.observed_at_utc(), "2026-09-02T06:00:00Z");
    assert_eq!(receipt.location().kind(), ObservationLocationKind::Column);
    assert_eq!(receipt.location().schema_name(), "Sales/~North");
    assert_eq!(receipt.location().table_name(), "Order/Line");
    assert_eq!(receipt.location().column_name(), Some("Account/Key"));
    assert_eq!(receipt.location().constraint_name(), None);
    assert_eq!(
        receipt.location().canonical_location(),
        "/schemas/Sales~1~0North/tables/Order~1Line/columns/Account~1Key"
    );
}

#[test]
fn snapshot_issues_exact_evidence_receipt_for_observed_table() {
    let location = ObservationLocation::table("Sales/~North", "Order/Line").unwrap();
    assert!(snapshot().source_receipt(location).is_ok());
}

#[test]
fn canonical_locations_are_typed_and_collision_safe() {
    let table = ObservationLocation::table("public", "event_record").expect("valid table");
    let column =
        ObservationLocation::column("public", "event_record", "event_key").expect("valid column");
    let constraint = ObservationLocation::constraint("public", "event_record", "event_identity_pk")
        .expect("valid constraint");

    assert_eq!(table.kind(), ObservationLocationKind::Table);
    assert_eq!(column.kind(), ObservationLocationKind::Column);
    assert_eq!(constraint.kind(), ObservationLocationKind::Constraint);
    assert_eq!(
        table.canonical_location(),
        "/schemas/public/tables/event_record"
    );
    assert_eq!(
        column.canonical_location(),
        "/schemas/public/tables/event_record/columns/event_key"
    );
    assert_eq!(
        constraint.canonical_location(),
        "/schemas/public/tables/event_record/constraints/event_identity_pk"
    );
}

#[test]
fn snapshot_rejects_receipt_for_unobserved_location() {
    let missing = ObservationLocation::column("Sales/~North", "Order/Line", "missing_column")
        .expect("location shape is valid before snapshot binding");
    let expected_location = missing.canonical_location();

    let error = snapshot()
        .source_receipt(missing)
        .expect_err("a receipt cannot invent an unobserved source coordinate");

    assert_eq!(
        error,
        ObservationError::UnknownObservationLocation {
            location: expected_location,
        }
    );
}

#[test]
fn snapshot_rejects_a_location_with_only_the_schema_in_common() {
    let missing = ObservationLocation::table("Sales/~North", "Other/Line")
        .expect("location shape is valid before snapshot binding");
    assert!(snapshot().source_receipt(missing).is_err());
}

#[test]
fn snapshot_rejects_a_location_with_only_the_table_name_in_common() {
    let missing = ObservationLocation::table("Other/South", "Order/Line")
        .expect("location shape is valid before snapshot binding");
    assert!(snapshot().source_receipt(missing).is_err());
}

#[test]
fn snapshot_receipts_existing_constraint_coordinates() {
    let location =
        ObservationLocation::constraint("Sales/~North", "Order/Line", "Order/Account~FK")
            .expect("constraint location is valid");

    let receipt = snapshot()
        .source_receipt(location)
        .expect("observed constraint can be receipted");

    assert_eq!(
        receipt.location().kind(),
        ObservationLocationKind::Constraint
    );
    assert_eq!(receipt.location().column_name(), None);
    assert_eq!(
        receipt.location().constraint_name(),
        Some("Order/Account~FK")
    );
    assert_eq!(
        receipt.location().canonical_location(),
        "/schemas/Sales~1~0North/tables/Order~1Line/constraints/Order~1Account~0FK"
    );
}

#[test]
fn evidence_location_rejects_blank_exact_identifiers() {
    assert_eq!(
        ObservationLocation::table("\u{2003}", "event_record"),
        Err(ObservationError::InvalidObservationField {
            field: "schema_name"
        })
    );
    assert_eq!(
        ObservationLocation::column("public", "event_record", " "),
        Err(ObservationError::InvalidObservationField {
            field: "column_name"
        })
    );
    assert_eq!(
        ObservationLocation::constraint("public", "event_record", "\n\t"),
        Err(ObservationError::InvalidObservationField {
            field: "constraint_name"
        })
    );
    assert!(ObservationLocation::table("public", " ").is_err());
}
