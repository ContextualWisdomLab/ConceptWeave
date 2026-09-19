use conceptweave_observation::{
    ColumnObservationV3, ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType,
    ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn relation_with_match_type(match_type: ForeignKeyMatchType) -> RelationObservation {
    let foreign_key = ForeignKeyObservation::with_reference_behavior(
        "document_parent_fk",
        vec!["parent_id".to_owned()],
        "public",
        "parent",
        vec!["id".to_owned()],
        ForeignKeyReferenceBehavior::new(
            ForeignKeyAction::NoAction,
            ForeignKeyAction::NoAction,
            match_type,
            ForeignKeyDeferrability::NotDeferrable,
        ),
    )
    .expect("foreign-key fixture is structurally valid");

    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "parent_id",
            1,
            "int8",
            catalog_type("int8"),
            false,
            None,
        )
        .expect("column fixture is valid")],
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::ForeignKey(foreign_key)])
    .expect("foreign-key relation fixture is valid")
}

fn snapshot(match_type: ForeignKeyMatchType) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T01:45:00Z",
        vec![relation_with_match_type(match_type)],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn governed_postgres_v3_rejects_unimplemented_match_partial() {
    let error = snapshot(ForeignKeyMatchType::Partial)
        .expect_err("PostgreSQL 18 does not implement MATCH PARTIAL");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "foreign_key_match_type",
        }
    );
}

#[test]
fn governed_postgres_v3_accepts_supported_match_types() {
    for match_type in [ForeignKeyMatchType::Simple, ForeignKeyMatchType::Full] {
        snapshot(match_type).expect("PostgreSQL 18 supports MATCH SIMPLE and MATCH FULL");
    }
}
