use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn key_semantics() -> IndexKeySemantics {
    IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
            .expect("operator-class fixture is valid"),
        0,
    )
    .expect("key-semantics fixture is valid")
}

fn index(index_name: &str, clustered: bool) -> IndexObservation {
    IndexObservation::new(
        index_name,
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
    .with_key_semantics(vec![key_semantics()])
    .expect("one semantic record matches the structural key")
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, clustered, false, false,
    ))
    .expect("catalog flag fixture is structurally constructible")
}

fn relation(indexes: Vec<IndexObservation>) -> RelationObservation {
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
    .with_indexes(indexes)
    .expect("index fixtures are valid before aggregate clustered-index validation")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-21T08:45:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn relation_rejects_multiple_clustered_indexes() {
    let error = snapshot(relation(vec![
        index("document_clustered_a_ix", true),
        index("document_clustered_b_ix", true),
    ]))
    .expect_err("one relation cannot coherently expose two last-clustered indexes");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_clustered",
        }
    );
}

#[test]
fn one_clustered_index_and_an_ordinary_index_remain_admissible() {
    snapshot(relation(vec![
        index("document_clustered_ix", true),
        index("document_lookup_ix", false),
    ]))
    .expect("an unrelated ordinary index does not become a second clustered index");
}

#[test]
fn zero_clustered_indexes_remain_admissible() {
    snapshot(relation(vec![
        index("document_lookup_a_ix", false),
        index("document_lookup_b_ix", false),
    ]))
    .expect("absence of a previously clustered index is valid catalog state");
}
