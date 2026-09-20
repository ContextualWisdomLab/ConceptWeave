use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation,
};

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn complete_index() -> IndexObservation {
    IndexObservation::new(
        "relation_id_ix",
        false,
        None,
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id")
                .expect("key attribute fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("index layout is valid")
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops")
                .expect("operator class fixture is valid"),
            0,
        )
        .expect("key semantics fixture is valid"),
    ])
    .expect("one semantic record exactly matches one key position")
}

fn relation(kind: RelationKind) -> RelationObservation {
    RelationObservation::new(
        "public",
        "indexed_relation",
        kind,
        vec![
            ColumnObservationV3::new("id", 1, "integer", catalog_type("int4"), false, None)
                .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
}

#[test]
fn non_indexable_relation_kinds_reject_nonempty_index_evidence() {
    for kind in [
        RelationKind::View,
        RelationKind::ForeignTable,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        let error = relation(kind)
            .with_indexes(vec![complete_index()])
            .expect_err("non-indexable PostgreSQL relation kinds must reject local index evidence");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField { field: "indexes" },
            "relation kind {kind:?} must fail at the relation/index ownership boundary"
        );
    }
}

#[test]
fn indexable_relation_kinds_accept_complete_index_evidence() {
    for kind in [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::MaterializedView,
    ] {
        let observed = relation(kind)
            .with_indexes(vec![complete_index()])
            .expect("PostgreSQL index-owning relation kind must admit complete index evidence");
        assert_eq!(observed.indexes().len(), 1);
    }
}

#[test]
fn every_relation_kind_can_carry_an_explicitly_empty_index_collection() {
    for kind in [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::View,
        RelationKind::MaterializedView,
        RelationKind::ForeignTable,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        let observed = relation(kind)
            .with_indexes(Vec::new())
            .expect("absence of local index evidence is valid for every relation kind");
        assert!(observed.indexes().is_empty());
    }
}
