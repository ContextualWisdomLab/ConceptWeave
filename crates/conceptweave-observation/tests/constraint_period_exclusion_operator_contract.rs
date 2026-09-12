use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintExclusionOperatorObservation,
    ConstraintPeriodObservation, ConstraintTimingObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation, TypeKindObservation,
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

fn temporal_relation() -> RelationObservation {
    let columns = vec![
        ColumnObservationV3::new(
            "document_id",
            1,
            "bigint",
            catalog_type("int8"),
            false,
            None,
        )
        .expect("id column fixture is valid"),
        ColumnObservationV3::new(
            "valid_during",
            2,
            "tstzrange",
            catalog_type("tstzrange"),
            false,
            None,
        )
        .expect("period column fixture is valid"),
    ];
    let key = TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("temporal key fixture is valid"),
    );
    let index = IndexObservation::new(
        "document_temporal_key",
        true,
        Some(false),
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "document_id")
                .expect("index attribute fixture is valid"),
            IndexAttributeObservation::new(2, IndexAttributeKind::Key, "valid_during")
                .expect("index attribute fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("backing index fixture is valid")
    .with_access_method("gist")
    .with_catalog_flags(IndexCatalogFlags::new(
        true, true, true, false, false, false,
    ))
    .expect("backing index catalog flags are valid");

    RelationObservation::new("public", "document", RelationKind::Table, columns)
        .expect("relation fixture is valid")
        .with_constraints(vec![key])
        .expect("constraint fixture is valid")
        .with_indexes(vec![index])
        .expect("index fixture is valid")
}

fn timing() -> ConstraintTimingObservation {
    ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        ConstraintDeferrability::NotDeferrable,
    )
    .expect("timing fixture is valid")
}

fn exclusion_operator(
    position: u32,
    operator_name: &str,
    operand_type: &str,
) -> ConstraintExclusionOperatorObservation {
    ConstraintExclusionOperatorObservation::new(
        position,
        "pg_catalog",
        operator_name,
        catalog_type(operand_type),
        catalog_type(operand_type),
    )
    .expect("operator fixture is valid")
}

fn expected_operators(first_operator_name: &str) -> Vec<ConstraintExclusionOperatorObservation> {
    vec![
        exclusion_operator(1, first_operator_name, "int8"),
        exclusion_operator(2, "&&", "tstzrange"),
    ]
}

fn period_without_operators() -> ConstraintPeriodObservation {
    ConstraintPeriodObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        true,
    )
    .expect("period fixture is valid")
}

fn period_with_operators(
    operators: Vec<ConstraintExclusionOperatorObservation>,
) -> ConstraintPeriodObservation {
    period_without_operators()
        .with_exclusion_operators(operators)
        .expect("temporal exclusion operator fixture is valid")
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_type_kinds_and_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T09:42:00Z",
        vec![temporal_relation()],
        Vec::new(),
        Vec::new(),
        temporal_type_kinds(),
        vec![timing()],
    )
    .expect("base temporal snapshot is coherent")
}

#[test]
fn temporal_key_requires_observed_conexclop_operator_vector() {
    let error = base_snapshot()
        .with_observed_constraint_periods(vec![period_without_operators()])
        .expect_err("WITHOUT OVERLAPS cannot omit pg_constraint.conexclop evidence");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_exclusion_operators",
        }
    );
}

#[test]
fn temporal_key_requires_one_exclusion_operator_per_key_column() {
    let error = base_snapshot()
        .with_observed_constraint_periods(vec![period_with_operators(vec![exclusion_operator(
            1, "=", "int8",
        )])])
        .expect_err("conexclop arity must match the temporal key column arity");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_exclusion_operators",
        }
    );
}

#[test]
fn coherent_temporal_key_preserves_exact_exclusion_operator_signatures() {
    let snapshot = base_snapshot()
        .with_observed_constraint_periods(vec![period_with_operators(expected_operators("="))])
        .expect("complete temporal exclusion operator evidence is admissible");

    let operators = snapshot.constraint_periods().expect("period family is observed")[0]
        .exclusion_operators()
        .expect("temporal key operator vector is observed");
    assert_eq!(operators.len(), 2);
    assert_eq!(operators[0].position(), 1);
    assert_eq!(operators[0].operator_schema_name(), "pg_catalog");
    assert_eq!(operators[0].operator_name(), "=");
    assert_eq!(operators[1].position(), 2);
    assert_eq!(operators[1].operator_name(), "&&");
}

#[test]
fn exclusion_operator_identity_changes_the_period_snapshot_digest() {
    let equality = base_snapshot()
        .with_observed_constraint_periods(vec![period_with_operators(expected_operators("="))])
        .expect("equality-operator fixture is coherent");
    let alternate = base_snapshot()
        .with_observed_constraint_periods(vec![period_with_operators(expected_operators("=#"))])
        .expect("alternate resolved operator fixture is coherent");

    assert_ne!(equality.snapshot_digest(), alternate.snapshot_digest());
}
