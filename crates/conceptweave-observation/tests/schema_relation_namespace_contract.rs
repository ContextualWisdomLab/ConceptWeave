use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, PostgresSchemaSnapshotV3, QualifiedOperatorClassName, QualifiedTypeName,
    RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn complete_index(index_name: &str) -> IndexObservation {
    IndexObservation::new(
        index_name,
        false,
        Some(false),
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "document_id")
                .expect("key fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("index fixture is valid")
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
                .expect("operator-class fixture is valid"),
            0,
        )
        .expect("key semantics fixture is valid"),
    ])
    .expect("complete key semantics are valid")
}

fn relation(schema_name: &str, relation_name: &str, index_name: Option<&str>) -> RelationObservation {
    let relation = RelationObservation::new(
        schema_name,
        relation_name,
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
    .expect("relation fixture is valid");

    match index_name {
        Some(index_name) => relation
            .with_indexes(vec![complete_index(index_name)])
            .expect("index fixture is valid"),
        None => relation,
    }
}

fn snapshot(relations: Vec<RelationObservation>) -> Result<PostgresSchemaSnapshotV3, conceptweave_observation::ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public", "archive"]),
        "postgres_introspector_v3",
        "2026-09-11T10:58:00Z",
        relations,
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn duplicate_index_names_in_one_schema_fail_closed() {
    let result = snapshot(vec![
        relation("public", "document", Some("shared_idx")),
        relation("public", "audit_log", Some("shared_idx")),
    ]);

    assert!(
        result.is_err(),
        "pg_class requires schema-local relation names, including indexes, to be unique"
    );
}

#[test]
fn index_name_colliding_with_relation_name_in_one_schema_fails_closed() {
    let result = snapshot(vec![
        relation("public", "document", Some("audit_log")),
        relation("public", "audit_log", None),
    ]);

    assert!(
        result.is_err(),
        "an index and another relation cannot share one pg_class name in the same schema"
    );
}

#[test]
fn same_index_name_in_different_schemas_remains_valid() {
    let result = snapshot(vec![
        relation("public", "document", Some("shared_idx")),
        relation("archive", "document", Some("shared_idx")),
    ]);

    assert!(
        result.is_ok(),
        "pg_class relation-name uniqueness is scoped by namespace, not cluster-wide"
    );
}
