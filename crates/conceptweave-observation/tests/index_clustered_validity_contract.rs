use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn index(clustered: bool, valid: Option<bool>) -> IndexObservation {
    let mut index = IndexObservation::new(
        if clustered {
            "document_clustered_ix"
        } else {
            "document_lookup_ix"
        },
        false,
        None,
        vec![IndexAttributeObservation::column(
            1,
            IndexAttributeKind::Key,
            "document_id",
        )
        .expect("column key fixture is valid")],
        Vec::new(),
    )
    .expect("index fixture is structurally valid")
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
        false, false, true, clustered, false, false,
    ))
    .expect("catalog flag fixture is structurally constructible");

    if let Some(valid) = valid {
        index = index.with_valid(valid);
    }
    index
}

fn relation(index: IndexObservation) -> RelationObservation {
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
    .with_indexes(vec![index])
    .expect("index fixture is valid before aggregate clustered-index validation")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-22T02:38:00+09:00",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn clustered_index_rejects_explicit_invalid_lifecycle_state() {
    let error = snapshot(relation(index(true, Some(false))))
        .expect_err("PostgreSQL cannot persist clustered ownership on an invalid index");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_clustered",
        }
    );
}

#[test]
fn clustered_index_accepts_explicit_valid_lifecycle_state() {
    snapshot(relation(index(true, Some(true))))
        .expect("an explicitly valid clustered index remains admissible");
}

#[test]
fn clustered_index_preserves_unobserved_validity_state() {
    snapshot(relation(index(true, None)))
        .expect("missing optional validity evidence must not be fabricated as invalid");
}

#[test]
fn ordinary_invalid_index_remains_admissible() {
    snapshot(relation(index(false, Some(false))))
        .expect("an observed invalid index is not contradictory unless it claims clustered ownership");
}
