use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn simple_index(
    is_unique: bool,
    immediate: bool,
    replica_identity: bool,
) -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        is_unique,
        Some(false),
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
        false,
        false,
        immediate,
        false,
        false,
        replica_identity,
    ))
    .expect("catalog flag fixture is structurally constructible")
}

fn expression_index() -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        true,
        Some(false),
        vec![IndexAttributeObservation::expression(
            1,
            IndexAttributeKind::Key,
            "document_id + 0",
        )
        .expect("expression key fixture is valid")],
        Vec::new(),
    )
    .expect("expression index fixture is structurally valid")
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
    .expect("one semantic record matches the expression key")
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, true,
    ))
    .expect("replica-identity catalog flags are structurally constructible")
}

fn relation(nullable: bool, index: IndexObservation) -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "document_id",
            1,
            "bigint",
            catalog_type("int8"),
            nullable,
            None,
        )
        .expect("column fixture is valid")],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect("index fixture is valid before aggregate replica-identity validation")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-21T06:58:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

fn assert_replica_identity_error(result: Result<PostgresSchemaSnapshotV3, ObservationError>) {
    assert_eq!(
        result.expect_err("impossible pg_index.indisreplident evidence must fail closed"),
        ObservationError::InvalidObservationField {
            field: "index_replica_identity",
        }
    );
}

#[test]
fn replica_identity_index_must_be_unique() {
    assert_replica_identity_error(snapshot(relation(
        false,
        simple_index(false, true, true),
    )));
}

#[test]
fn replica_identity_index_must_be_immediate_not_deferred() {
    assert_replica_identity_error(snapshot(relation(
        false,
        simple_index(true, false, true),
    )));
}

#[test]
fn replica_identity_index_must_not_be_partial() {
    assert_replica_identity_error(snapshot(relation(
        false,
        simple_index(true, true, true).with_predicate("document_id > 0"),
    )));
}

#[test]
fn replica_identity_index_must_use_columns_not_expressions() {
    assert_replica_identity_error(snapshot(relation(false, expression_index())));
}

#[test]
fn replica_identity_index_columns_must_be_not_null() {
    assert_replica_identity_error(snapshot(relation(
        true,
        simple_index(true, true, true),
    )));
}

#[test]
fn eligible_replica_identity_index_remains_admissible() {
    snapshot(relation(false, simple_index(true, true, true)))
        .expect("unique non-partial immediate column-only NOT NULL replica identity is admissible");
}

#[test]
fn non_replica_indexes_do_not_import_replica_identity_restrictions() {
    snapshot(relation(
        true,
        simple_index(false, false, false).with_predicate("document_id > 0"),
    ))
    .expect("ordinary non-replica indexes retain their independent catalog semantics");
}
