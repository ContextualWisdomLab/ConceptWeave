use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, NotNullConstraintObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation,
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

fn index(replica_identity: bool) -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        true,
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
    .with_key_semantics(vec![key_semantics()])
    .expect("one semantic record matches the structural key")
    .with_catalog_flags(IndexCatalogFlags::new(
        false,
        false,
        true,
        false,
        false,
        replica_identity,
    ))
    .expect("catalog flag fixture is structurally constructible")
}

fn relation(replica_identity: bool) -> RelationObservation {
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
    .with_indexes(vec![index(replica_identity)])
    .expect("index fixture is valid before aggregate replica-identity validation")
}

fn not_null(validated: bool) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_id_not_null",
        "document_id",
        validated,
        true,
        true,
        0,
        false,
    )
    .expect("NOT NULL constraint fixture is valid")
}

fn snapshot(
    replica_identity: bool,
    validated: bool,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-22T05:43:43+09:00",
        vec![relation(replica_identity)],
        Vec::new(),
        Vec::new(),
        vec![not_null(validated)],
    )
}

#[test]
fn replica_identity_key_rejects_observed_unvalidated_not_null_constraint() {
    assert_eq!(
        snapshot(true, false)
            .expect_err("PostgreSQL rejects a NOT VALID key constraint for replica identity"),
        ObservationError::InvalidObservationField {
            field: "index_replica_identity",
        }
    );
}

#[test]
fn replica_identity_key_accepts_observed_validated_not_null_constraint() {
    snapshot(true, true).expect("validated NOT NULL evidence is admissible for replica identity");
}

#[test]
fn unvalidated_not_null_constraint_remains_admissible_without_replica_identity() {
    snapshot(false, false)
        .expect("NOT VALID is source-authoritative evidence outside replica-identity eligibility");
}

#[test]
fn replica_identity_does_not_require_unobserved_not_null_family() {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-22T05:43:43+09:00",
        vec![relation(true)],
        Vec::new(),
        Vec::new(),
    )
    .expect("optional first-class NOT NULL evidence is not invented when unobserved");
}
