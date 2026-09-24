use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn key_semantics(position: u32) -> IndexKeySemantics {
    IndexKeySemantics::new(
        position,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
            .expect("operator-class fixture is valid"),
        0,
    )
    .expect("key-semantics fixture is valid")
}

fn simple_index(is_unique: bool, immediate: bool, replica_identity: bool) -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        is_unique,
        Some(false),
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "document_id")
                .expect("column key fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![key_semantics(1)])
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

fn multi_key_replica_identity_index() -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        true,
        Some(false),
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "document_id")
                .expect("first key fixture is valid"),
            IndexAttributeObservation::column(2, IndexAttributeKind::Key, "tenant_id")
                .expect("second key fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("multi-key index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![key_semantics(1), key_semantics(2)])
    .expect("semantic records match both structural keys")
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, true,
    ))
    .expect("replica-identity catalog flags are structurally constructible")
}

fn covering_replica_identity_index() -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        true,
        Some(false),
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "document_id")
                .expect("key fixture is valid"),
        ],
        vec![
            IndexAttributeObservation::column(2, IndexAttributeKind::Include, "payload")
                .expect("INCLUDE fixture is valid"),
        ],
    )
    .expect("covering index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![key_semantics(1)])
    .expect("INCLUDE payload does not create key semantics")
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, true,
    ))
    .expect("replica-identity catalog flags are structurally constructible")
}

fn expression_index() -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        true,
        Some(false),
        vec![
            IndexAttributeObservation::expression(1, IndexAttributeKind::Key, "document_id + 0")
                .expect("expression key fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("expression index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![key_semantics(1)])
    .expect("one semantic record matches the expression key")
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, true,
    ))
    .expect("replica-identity catalog flags are structurally constructible")
}

fn relation(kind: RelationKind, nullable: bool, index: IndexObservation) -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        kind,
        vec![
            ColumnObservationV3::new(
                "document_id",
                1,
                "bigint",
                catalog_type("int8"),
                nullable,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect("index fixture is valid before aggregate replica-identity validation")
}

fn multi_column_relation(
    second_key_nullable: bool,
    index: IndexObservation,
) -> RelationObservation {
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
            .expect("first key column fixture is valid"),
            ColumnObservationV3::new(
                "tenant_id",
                2,
                "bigint",
                catalog_type("int8"),
                second_key_nullable,
                None,
            )
            .expect("second key column fixture is valid"),
        ],
    )
    .expect("multi-column relation fixture is valid")
    .with_indexes(vec![index])
    .expect("multi-key index fixture is valid before aggregate validation")
}

fn covering_relation(index: IndexObservation) -> RelationObservation {
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
            .expect("replica key column fixture is valid"),
            ColumnObservationV3::new("payload", 2, "bigint", catalog_type("int8"), true, None)
                .expect("nullable INCLUDE payload fixture is valid"),
        ],
    )
    .expect("covering relation fixture is valid")
    .with_indexes(vec![index])
    .expect("covering index fixture is valid before aggregate validation")
}

fn table(nullable: bool, index: IndexObservation) -> RelationObservation {
    relation(RelationKind::Table, nullable, index)
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-20T21:42:00Z",
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
fn replica_identity_index_must_belong_to_a_table_relation() {
    assert_replica_identity_error(snapshot(relation(
        RelationKind::MaterializedView,
        false,
        simple_index(true, true, true),
    )));
}

#[test]
fn replica_identity_index_must_be_unique() {
    assert_replica_identity_error(snapshot(table(false, simple_index(false, true, true))));
}

#[test]
fn replica_identity_index_must_be_immediate_not_deferred() {
    assert_replica_identity_error(snapshot(table(false, simple_index(true, false, true))));
}

#[test]
fn replica_identity_index_must_not_be_partial() {
    assert_replica_identity_error(snapshot(table(
        false,
        simple_index(true, true, true).with_predicate("document_id > 0"),
    )));
}

#[test]
fn replica_identity_index_must_use_columns_not_expressions() {
    assert_replica_identity_error(snapshot(table(false, expression_index())));
}

#[test]
fn replica_identity_index_columns_must_be_not_null() {
    assert_replica_identity_error(snapshot(table(true, simple_index(true, true, true))));
}

#[test]
fn every_replica_identity_key_column_must_be_not_null() {
    assert_replica_identity_error(snapshot(multi_column_relation(
        true,
        multi_key_replica_identity_index(),
    )));
}

#[test]
fn eligible_replica_identity_index_remains_admissible() {
    snapshot(table(false, simple_index(true, true, true)))
        .expect("unique non-partial immediate column-only NOT NULL replica identity is admissible");
}

#[test]
fn nullable_include_payload_does_not_become_replica_identity_key_material() {
    snapshot(covering_relation(covering_replica_identity_index())).expect(
        "nullable INCLUDE payload remains outside the replica-identity key and its NOT NULL rule",
    );
}

#[test]
fn partitioned_table_replica_identity_does_not_require_lifecycle_evidence() {
    snapshot(relation(
        RelationKind::PartitionedTable,
        false,
        simple_index(true, true, true),
    ))
    .expect(
        "partitioned-table replica identity remains valid before optional lifecycle observation",
    );
}

#[test]
fn non_replica_indexes_do_not_import_replica_identity_restrictions() {
    snapshot(table(
        true,
        simple_index(false, false, false).with_predicate("document_id > 0"),
    ))
    .expect("ordinary non-replica indexes retain their independent catalog semantics");
}

#[test]
fn materialized_view_non_replica_index_remains_admissible() {
    snapshot(relation(
        RelationKind::MaterializedView,
        true,
        simple_index(false, false, false).with_predicate("document_id > 0"),
    ))
    .expect("materialized-view indexes remain admissible when they do not claim replica identity");
}
