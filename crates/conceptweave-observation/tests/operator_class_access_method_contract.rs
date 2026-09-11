use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation,
};

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

#[test]
fn unbound_operator_class_fails_closed_before_relation_identity() {
    let operator_class = QualifiedOperatorClassName::new("pg_catalog", "text_ops")
        .expect("schema-qualified operator-class text is structurally valid");
    let semantics = IndexKeySemantics::new(1, None, operator_class, 0)
        .expect("one-based key semantics are structurally valid");
    let index = IndexObservation::new(
        "document_title_ix",
        false,
        None,
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "title")
                .expect("key attribute fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("index layout is valid")
    .with_access_method("btree")
    .with_key_semantics(vec![semantics])
    .expect("semantic position matches the structural key");

    let error = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new("title", 1, "text", catalog_type("text"), false, None)
                .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect_err(
        "a schema/name-only operator class must not enter governed identity without its pg_am binding",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_operator_class_access_method",
        }
    );
}
