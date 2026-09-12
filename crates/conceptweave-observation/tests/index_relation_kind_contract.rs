use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn complete_index() -> IndexObservation {
    IndexObservation::new(
        "document_lookup_idx",
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

fn indexed_relation(kind: RelationKind) -> RelationObservation {
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
                false,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![complete_index()])
    .expect("relation-local index structure is valid before snapshot admission")
}

fn snapshot(kind: RelationKind) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T12:46:00Z",
        vec![indexed_relation(kind)],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn index_evidence_is_admitted_only_for_postgresql_indexable_relation_kinds() {
    for kind in [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::MaterializedView,
    ] {
        assert!(
            snapshot(kind).is_ok(),
            "PostgreSQL supports local indexes for {kind:?}"
        );
    }

    for kind in [
        RelationKind::View,
        RelationKind::ForeignTable,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        let error = snapshot(kind).expect_err(
            "non-indexable PostgreSQL relation kinds must fail before governed identity",
        );
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "index_relation_kind",
            },
            "unexpected admission result for {kind:?}"
        );
    }
}
