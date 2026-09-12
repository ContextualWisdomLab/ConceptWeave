use conceptweave_observation::{
    ColumnObservationV3, ConstraintPeriodObservation, ForeignKeyAction, ForeignKeyDeferrability,
    ForeignKeyMatchType, ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
    TableConstraintObservation, TypeKindObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn temporal_type_kinds() -> Vec<TypeKindObservation> {
    vec![
        TypeKindObservation::range(catalog_type("tstzrange"), catalog_type("tstzmultirange")),
        TypeKindObservation::multirange(
            catalog_type("tstzmultirange"),
            catalog_type("tstzrange"),
        ),
    ]
}

fn column(name: &str, position: u32, type_name: &str) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        position,
        type_name,
        catalog_type(type_name),
        false,
        None,
    )
    .expect("column fixture is valid")
}

fn outbound_period_relation() -> RelationObservation {
    let behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::NoAction,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    );
    RelationObservation::new(
        "public",
        "document_version",
        RelationKind::Table,
        vec![
            column("document_id", 1, "int8"),
            column("valid_during", 2, "tstzrange"),
        ],
    )
    .expect("child relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::ForeignKey(
        ForeignKeyObservation::with_reference_behavior(
            "document_version_period_fk",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
            "public",
            "document",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
            behavior,
        )
        .expect("PERIOD foreign-key fixture is structurally valid"),
    )])
    .expect("child constraint fixture is valid")
}

#[test]
fn period_foreign_key_rejects_missing_referenced_temporal_key_evidence() {
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T11:45:00Z",
        vec![outbound_period_relation()],
        Vec::new(),
        Vec::new(),
        temporal_type_kinds(),
    )
    .expect("local PERIOD foreign-key evidence is structurally valid");

    let period = ConstraintPeriodObservation::new(
        "public",
        "document_version",
        RelationKind::Table,
        "document_version_period_fk",
        true,
    )
    .expect("constraint-period fixture is valid");

    let error = snapshot
        .with_observed_constraint_periods(vec![period])
        .expect_err("PERIOD authority requires captured referenced WITHOUT OVERLAPS key evidence");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_reference",
        }
    );
}
