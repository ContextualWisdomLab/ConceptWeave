use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, ForeignKeyObservation, ObservationError,
    PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation, UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn check_constraint() -> TableConstraintObservation {
    TableConstraintObservation::Check(
        CheckConstraintObservation::new("document_id_positive", "document_id > 0", true, true, false)
            .expect("CHECK fixture is valid"),
    )
}

fn primary_key_constraint() -> TableConstraintObservation {
    TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
            .expect("primary-key fixture is valid"),
    )
}

fn unique_constraint() -> TableConstraintObservation {
    TableConstraintObservation::Unique(
        UniqueConstraintObservation::new("document_id_key", vec!["document_id".to_owned()])
            .expect("unique fixture is valid"),
    )
}

fn foreign_key_constraint() -> TableConstraintObservation {
    TableConstraintObservation::ForeignKey(
        ForeignKeyObservation::new(
            "document_parent_fkey",
            vec!["document_id".to_owned()],
            "public",
            "parent_document",
            vec!["document_id".to_owned()],
        )
        .expect("foreign-key fixture is valid"),
    )
}

fn relation_with_constraints(
    kind: RelationKind,
    constraints: Vec<TableConstraintObservation>,
) -> RelationObservation {
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
    .with_constraints(constraints)
    .expect("relation-local constraint structure is valid before snapshot admission")
}

fn snapshot(
    kind: RelationKind,
    constraints: Vec<TableConstraintObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T13:44:00Z",
        vec![relation_with_constraints(kind, constraints)],
        Vec::new(),
        Vec::new(),
    )
}

fn assert_constraint_kind_error(
    kind: RelationKind,
    constraints: Vec<TableConstraintObservation>,
) {
    let error = snapshot(kind, constraints)
        .expect_err("impossible PostgreSQL relation/constraint combinations must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_constraint_kind",
        },
        "unexpected admission result for {kind:?}"
    );
}

#[test]
fn represented_constraints_follow_postgresql_relation_kind_rules() {
    for kind in [RelationKind::Table, RelationKind::PartitionedTable] {
        assert!(
            snapshot(kind, vec![primary_key_constraint(), check_constraint()]).is_ok(),
            "ordinary and partitioned tables admit represented table constraints"
        );
    }

    assert!(
        snapshot(RelationKind::ForeignTable, vec![check_constraint()]).is_ok(),
        "foreign tables admit PostgreSQL CHECK constraint evidence"
    );
    assert_constraint_kind_error(RelationKind::ForeignTable, vec![primary_key_constraint()]);
    assert_constraint_kind_error(RelationKind::ForeignTable, vec![unique_constraint()]);
    assert_constraint_kind_error(RelationKind::ForeignTable, vec![foreign_key_constraint()]);

    for kind in [
        RelationKind::View,
        RelationKind::MaterializedView,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        assert_constraint_kind_error(kind, vec![check_constraint()]);
    }
}
