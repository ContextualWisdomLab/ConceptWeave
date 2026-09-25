use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn index(clustered: bool, predicate: Option<&str>) -> IndexObservation {
    let mut index = IndexObservation::new(
        if clustered {
            "document_clustered_ix"
        } else {
            "document_lookup_ix"
        },
        false,
        None,
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "document_id")
                .expect("column key fixture is valid"),
        ],
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

    if let Some(predicate) = predicate {
        index = index.with_predicate(predicate);
    }
    index
}

fn relation(index: IndexObservation) -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "document_id",
                1,
                "bigint",
                catalog_type("int8"),
                false,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect("index fixture is valid before aggregate clustered-index validation")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-21T14:45:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn clustered_index_rejects_a_partial_predicate() {
    let error = snapshot(relation(index(true, Some("(document_id > 0)"))))
        .expect_err("PostgreSQL cannot persist a partial index as the relation's clustered index");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_clustered",
        }
    );
}

#[test]
fn non_partial_clustered_index_remains_admissible() {
    snapshot(relation(index(true, None)))
        .expect("a non-partial clustered index remains valid base catalog evidence");
}

#[test]
fn ordinary_partial_index_remains_admissible() {
    snapshot(relation(index(false, Some("(document_id > 0)"))))
        .expect("partial indexes remain valid when they do not claim clustered-index ownership");
}
