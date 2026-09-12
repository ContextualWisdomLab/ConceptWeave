use conceptweave_observation::{
    ColumnObservationV3, ConstraintPeriodObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn column(
    name: &str,
    position: u32,
    data_type: &str,
    type_name: &str,
) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        position,
        data_type,
        catalog_type(type_name),
        false,
        None,
    )
    .expect("column fixture is valid")
}

fn temporal_primary_key_with_scalar_period() -> RelationObservation {
    let attributes = ["document_id", "valid_during"]
        .iter()
        .enumerate()
        .map(|(index, column_name)| {
            IndexAttributeObservation::new(
                u32::try_from(index + 1).expect("fixture position fits u32"),
                IndexAttributeKind::Key,
                *column_name,
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
    .expect("backing index fixture is structurally valid")
    .with_access_method("gist")
    .with_catalog_flags(IndexCatalogFlags::new(
        true, true, true, false, false, false,
    ))
    .expect("catalog flags fixture is coherent");

    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            column("document_id", 1, "bigint", "int8"),
            column("valid_during", 2, "text", "text"),
        ],
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("primary-key fixture is valid"),
    )])
    .expect("constraint fixture is valid")
    .with_indexes(vec![backing_index])
    .expect("index fixture is valid")
}

#[test]
fn without_overlaps_rejects_scalar_final_key_column() {
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T02:50:00Z",
        vec![temporal_primary_key_with_scalar_period()],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot can preserve non-temporal scalar schema evidence");

    let period = ConstraintPeriodObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        true,
    )
    .expect("explicit conperiod fixture is structurally valid");

    let error = snapshot
        .with_observed_constraint_periods(vec![period])
        .expect_err(
            "PostgreSQL 18 WITHOUT OVERLAPS requires its final column to be range or multirange",
        );
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        }
    );
}
