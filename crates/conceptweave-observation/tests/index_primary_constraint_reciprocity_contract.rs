use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    PrimaryKeyObservation, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn primary_index() -> IndexObservation {
    IndexObservation::new(
        "document_pkey",
        true,
        Some(false),
        vec![IndexAttributeObservation::column(
            1,
            IndexAttributeKind::Key,
            "document_id",
        )
        .expect("primary key attribute fixture is valid")],
        Vec::new(),
    )
    .expect("primary index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
                .expect("operator-class fixture is valid"),
            0,
        )
        .expect("key-semantics fixture is valid"),
    ])
    .expect("one semantic record matches the structural key")
    .with_catalog_flags(IndexCatalogFlags::new(
        true, false, true, false, false, false,
    ))
    .expect("primary catalog flags are structurally valid")
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

fn relation(constraints: Vec<TableConstraintObservation>) -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "document_id",
            1,
            "bigint",
            catalog_type("int8"),
            false,
            None,
        )
        .expect("column fixture is valid")],
    )
    .expect("relation fixture is valid")
    .with_constraints(constraints)
    .expect("constraint fixture is valid")
    .with_indexes(vec![primary_index()])
    .expect("index fixture is valid before snapshot reciprocity validation")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-21T02:44:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn primary_catalog_index_requires_a_matching_primary_key_constraint() {
    let error = snapshot(relation(Vec::new()))
        .expect_err("pg_index.indisprimary cannot exist without its pg_constraint primary key");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn matching_primary_key_constraint_and_primary_catalog_index_remain_admissible() {
    let primary_key = TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
            .expect("primary-key fixture is valid"),
    );

    snapshot(relation(vec![primary_key]))
        .expect("coherent primary-key constraint and primary-index evidence must remain admissible");
}
