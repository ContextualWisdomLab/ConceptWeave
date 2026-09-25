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

fn index(index_name: &str, primary: bool, nulls_not_distinct: Option<bool>) -> IndexObservation {
    IndexObservation::new(
        index_name,
        true,
        nulls_not_distinct,
        vec![
            IndexAttributeObservation::column(1, IndexAttributeKind::Key, "document_id")
                .expect("index key fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("unique index fixture is structurally valid")
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
    .expect("catalog flags are structurally constructible")
}

fn relation(
    constraints: Vec<TableConstraintObservation>,
    observed_index: IndexObservation,
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
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_constraints(constraints)
    .expect("constraint fixture is valid")
    .with_indexes(vec![observed_index])
    .expect("index fixture is valid before aggregate primary-key validation")
}

fn primary_key() -> TableConstraintObservation {
    TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
            .expect("primary-key fixture is valid"),
    )
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-22T08:44:25Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn primary_catalog_index_rejects_nulls_not_distinct() {
    let error = snapshot(relation(
        vec![primary_key()],
        index("document_pkey", true, Some(true)),
    ))
    .expect_err("PostgreSQL primary keys cannot use NULLS NOT DISTINCT indexes");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn primary_catalog_index_accepts_explicit_nulls_distinct() {
    snapshot(relation(
        vec![primary_key()],
        index("document_pkey", true, Some(false)),
    ))
    .expect("explicit NULLS DISTINCT is compatible with a primary-key index");
}

#[test]
fn primary_catalog_index_accepts_unobserved_null_treatment() {
    snapshot(relation(
        vec![primary_key()],
        index("document_pkey", true, None),
    ))
    .expect(
        "an adapter that did not observe null treatment must not be rejected as NULLS NOT DISTINCT",
    );
}

#[test]
fn standalone_unique_nulls_not_distinct_index_remains_admissible() {
    snapshot(relation(
        Vec::new(),
        index("document_id_uix", false, Some(true)),
    ))
    .expect("NULLS NOT DISTINCT remains valid for a standalone unique index");
}
