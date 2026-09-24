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

fn index(primary: bool, predicate: Option<&str>) -> IndexObservation {
    let mut index = IndexObservation::new(
        if primary {
            "document_pkey"
        } else {
            "document_id_uix"
        },
        true,
        Some(false),
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "document_id")
                .expect("index key fixture is valid"),
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
        primary, false, true, false, false, false,
    ))
    .expect("catalog flags are structurally valid");

    if let Some(predicate) = predicate {
        index = index.with_predicate(predicate);
    }
    index
}

fn relation(index: IndexObservation, include_primary_key: bool) -> RelationObservation {
    let constraints = if include_primary_key {
        vec![TableConstraintObservation::PrimaryKey(
            PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
                .expect("primary-key fixture is valid"),
        )]
    } else {
        Vec::new()
    };

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
    .with_constraints(constraints)
    .expect("constraint fixture is locally valid")
    .with_indexes(vec![index])
    .expect("index fixture is locally valid")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-21T10:52:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn primary_catalog_index_rejects_a_partial_predicate_in_the_base_snapshot() {
    let error = snapshot(relation(index(true, Some("(document_id > 0)")), true))
        .expect_err("a PostgreSQL PRIMARY KEY cannot be backed by a partial index");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn matching_non_partial_primary_index_remains_admissible_without_lifecycle_evidence() {
    snapshot(relation(index(true, None), true))
        .expect("base PRIMARY KEY reciprocity must not require lifecycle evidence");
}

#[test]
fn standalone_partial_unique_index_remains_admissible() {
    snapshot(relation(index(false, Some("(document_id > 0)")), false))
        .expect("partial indexes remain valid when they do not claim PRIMARY KEY ownership");
}
