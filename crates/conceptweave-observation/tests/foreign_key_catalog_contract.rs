use conceptweave_observation::{
    ColumnObservationV3, ForeignKeyCatalogObservation, ForeignKeyObservation,
    ForeignKeyOperatorObservation, IndexAttributeKind, IndexAttributeObservation,
    IndexCatalogFlags, IndexKeySemantics, IndexObservation, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
    TableConstraintObservation,
};

mod support;

#[test]
fn observed_empty_foreign_key_catalog_has_distinct_identity_and_single_use() {
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-25T00:00:00Z",
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let previous_digest = snapshot.snapshot_digest().to_owned();
    let observed = snapshot
        .with_observed_foreign_key_catalog(Vec::new())
        .unwrap();
    assert_eq!(observed.foreign_key_catalog(), Some([].as_slice()));
    assert_ne!(observed.snapshot_digest(), previous_digest);
    assert!(
        observed
            .with_observed_foreign_key_catalog(Vec::new())
            .is_err()
    );
}

#[test]
fn captured_foreign_key_requires_matching_referenced_index_and_changes_digest() {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let column = |name: &str| {
        ColumnObservationV3::new(name, 1, "integer", int4.clone(), true, None).unwrap()
    };
    let parent_index = IndexObservation::new(
        "parent_key",
        true,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()],
        Vec::new(),
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, false,
    ))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true);
    let parent =
        RelationObservation::new("public", "parent", RelationKind::Table, vec![column("id")])
            .unwrap()
            .with_indexes(vec![parent_index])
            .unwrap();
    let child = RelationObservation::new(
        "public",
        "child",
        RelationKind::Table,
        vec![column("parent_id")],
    )
    .unwrap()
    .with_constraints(vec![TableConstraintObservation::ForeignKey(
        ForeignKeyObservation::new(
            "child_parent_fk",
            vec!["parent_id".to_owned()],
            "public",
            "parent",
            vec!["id".to_owned()],
        )
        .unwrap(),
    )])
    .unwrap();
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-25T00:00:00Z",
        vec![parent, child],
        Vec::new(),
        Vec::new(),
    )
    .unwrap();
    let operator =
        ForeignKeyOperatorObservation::new("pg_catalog", "=", int4.clone(), int4).unwrap();
    let catalog = |index_name: &str| {
        ForeignKeyCatalogObservation::new(
            "public",
            "child",
            "child_parent_fk",
            index_name,
            vec![operator.clone()],
            vec![operator.clone()],
            vec![operator.clone()],
        )
        .unwrap()
    };
    assert!(
        snapshot
            .clone()
            .with_observed_foreign_key_catalog(vec![catalog("missing")])
            .is_err()
    );
    let base_digest = snapshot.snapshot_digest().to_owned();
    let digest = snapshot
        .with_observed_foreign_key_catalog(vec![catalog("parent_key")])
        .unwrap()
        .snapshot_digest()
        .to_owned();
    assert_ne!(digest, base_digest);
}
