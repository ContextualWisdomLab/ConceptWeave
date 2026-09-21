use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintTimingObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics, IndexObservation,
    ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
    UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn backing_index(
    name: &str,
    primary: bool,
    access_method: &str,
) -> Result<IndexObservation, ObservationError> {
    IndexObservation::new(
        name,
        true,
        Some(false),
        vec![IndexAttributeObservation::column(
            1,
            IndexAttributeKind::Key,
            "document_id",
        )?],
        Vec::new(),
    )?
    .with_access_method(access_method)
    .with_key_semantics(vec![IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops")?,
        0,
    )?])?
    .with_catalog_flags(IndexCatalogFlags::new(
        primary,
        false,
        true,
        false,
        false,
        false,
    ))
    .map(|index| index.with_ready(true).with_valid(true).with_live(true))
}

fn primary_key() -> TableConstraintObservation {
    TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
            .expect("primary key fixture is valid"),
    )
}

fn unique_key() -> TableConstraintObservation {
    TableConstraintObservation::Unique(
        UniqueConstraintObservation::new("document_id_key", vec!["document_id".to_owned()])
            .expect("unique constraint fixture is valid"),
    )
}

fn snapshot(
    constraint: TableConstraintObservation,
    backing_index: IndexObservation,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    let constraint_name = constraint.constraint_name().to_owned();
    let relation = RelationObservation::new(
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
        )?],
    )?
    .with_constraints(vec![constraint])?
    .with_indexes(vec![backing_index])?;
    let timing = ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        constraint_name,
        ConstraintDeferrability::NotDeferrable,
    )?;

    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-21T12:52:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        vec![timing],
    )
}

fn assert_backing_index_error(result: Result<PostgresSchemaSnapshotV3, ObservationError>) {
    assert_eq!(
        result.expect_err("ordinary key constraints require a B-tree backing index"),
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn primary_key_rejects_non_exclusion_gin_backing_index() {
    assert_backing_index_error(snapshot(
        primary_key(),
        backing_index("document_pkey", true, "gin")
            .expect("the representation can carry contradictory source catalog evidence"),
    ));
}

#[test]
fn unique_constraint_rejects_non_exclusion_gist_backing_index() {
    assert_backing_index_error(snapshot(
        unique_key(),
        backing_index("document_id_key", false, "gist")
            .expect("the representation can carry contradictory source catalog evidence"),
    ));
}

#[test]
fn ordinary_primary_and_unique_constraints_keep_btree_backing_indexes() {
    snapshot(
        primary_key(),
        backing_index("document_pkey", true, "btree").expect("B-tree primary fixture is valid"),
    )
    .expect("ordinary PRIMARY KEY must keep its B-tree backing index");

    snapshot(
        unique_key(),
        backing_index("document_id_key", false, "btree").expect("B-tree unique fixture is valid"),
    )
    .expect("ordinary UNIQUE must keep its B-tree backing index");
}
