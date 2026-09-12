use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintPeriodObservation,
    ConstraintTimingObservation, ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType,
    ForeignKeyObservation, ForeignKeyReferenceBehavior, IndexAttributeKind,
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

fn temporal_key_operator_signatures() -> Vec<(u32, String, String, QualifiedTypeName, QualifiedTypeName)> {
    vec![
        (
            1,
            "pg_catalog".to_owned(),
            "=".to_owned(),
            catalog_type("int8"),
            catalog_type("int8"),
        ),
        (
            2,
            "pg_catalog".to_owned(),
            "&&".to_owned(),
            catalog_type("tstzrange"),
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

fn temporal_parent_relation(immediate: bool) -> RelationObservation {
    let attributes = ["document_id", "valid_during"]
        .into_iter()
        .enumerate()
        .map(|(index, column_name)| {
            IndexAttributeObservation::new(
                u32::try_from(index + 1).expect("fixture position fits u32"),
                IndexAttributeKind::Key,
                column_name,
            )
            .expect("index attribute fixture is valid")
        })
        .collect();
    let backing_index = IndexObservation::new(
        "document_temporal_key",
        true,
        Some(false),
        attributes,
        Vec::new(),
    )
    .expect("backing index fixture is valid")
    .with_access_method("gist")
    .with_catalog_flags(IndexCatalogFlags::new(
        true, true, immediate, false, false, false,
    ))
    .expect("catalog flags are coherent");

    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            column("document_id", 1, "int8"),
            column("valid_during", 2, "tstzrange"),
        ],
    )
    .expect("parent relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("temporal primary key fixture is valid"),
    )])
    .expect("parent constraint fixture is valid")
    .with_indexes(vec![backing_index])
    .expect("parent index fixture is valid")
}

fn period_child_relation() -> RelationObservation {
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
        .expect("period foreign key fixture is valid"),
    )])
    .expect("child constraint fixture is valid")
}

fn timing(deferrability: ConstraintDeferrability) -> ConstraintTimingObservation {
    ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        deferrability,
    )
    .expect("constraint timing fixture is valid")
}

fn period(relation_name: &str, constraint_name: &str) -> ConstraintPeriodObservation {
    let observation = ConstraintPeriodObservation::new(
        "public",
        relation_name,
        RelationKind::Table,
        constraint_name,
        true,
    )
    .expect("constraint period fixture is valid");
    if relation_name == "document" && constraint_name == "document_temporal_key" {
        observation
            .with_exclusion_operator_signatures(temporal_key_operator_signatures())
            .expect("temporal key exclusion operators are valid")
    } else {
        observation
    }
}

fn periods() -> Vec<ConstraintPeriodObservation> {
    vec![
        period("document", "document_temporal_key"),
        period("document_version", "document_version_period_fk"),
    ]
}

fn base_snapshot(immediate: bool) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:00:00Z",
        vec![temporal_parent_relation(immediate), period_child_relation()],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot fixture is valid")
    .with_observed_type_kinds(temporal_type_kinds())
    .expect("temporal type-kind evidence fixture is coherent")
}

fn timed_snapshot(
    immediate: bool,
    deferrability: ConstraintDeferrability,
) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_type_kinds_and_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T00:00:00Z",
        vec![temporal_parent_relation(immediate), period_child_relation()],
        Vec::new(),
        Vec::new(),
        temporal_type_kinds(),
        vec![timing(deferrability)],
    )
    .expect("timed snapshot fixture is internally coherent")
}

#[test]
fn period_foreign_key_requires_observed_referenced_key_timing() {
    let error = base_snapshot(true)
        .with_observed_constraint_periods(periods())
        .expect_err("PERIOD references must not infer non-deferrability from index shape");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_reference_timing",
        }
    );
}

#[test]
fn period_foreign_key_rejects_deferrable_referenced_temporal_key() {
    let error = timed_snapshot(false, ConstraintDeferrability::InitiallyImmediate)
        .with_observed_constraint_periods(periods())
        .expect_err("PostgreSQL PERIOD references require a non-deferrable referenced key");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_reference_timing",
        }
    );
}

#[test]
fn period_foreign_key_accepts_observed_nondeferrable_referenced_temporal_key() {
    timed_snapshot(true, ConstraintDeferrability::NotDeferrable)
        .with_observed_constraint_periods(periods())
        .expect("PERIOD reference is backed by explicit non-deferrable temporal-key evidence");
}
