use conceptweave_observation::{
    ColumnGenerationObservation, ColumnIdentityObservation, ColumnObservationV3, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_numeric() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "numeric").expect("catalog type coordinate is valid")
}

fn relation(kind: RelationKind) -> RelationObservation {
    RelationObservation::new(
        "public",
        "metric",
        kind,
        vec![ColumnObservationV3::new(
            "metric_id",
            1,
            "numeric",
            catalog_numeric(),
            false,
            None,
        )
        .expect("column fixture is valid")],
    )
    .expect("relation fixture is valid")
}

fn identity_snapshot(
    kind: RelationKind,
    identity: ColumnIdentityObservation,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_column_identities(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-16T03:46:00Z",
        vec![relation(kind)],
        Vec::new(),
        Vec::new(),
        vec![identity],
    )
}

fn generation_snapshot(
    kind: RelationKind,
    generation: ColumnGenerationObservation,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_column_generations(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-16T03:46:00Z",
        vec![relation(kind)],
        Vec::new(),
        Vec::new(),
        vec![generation],
    )
}

fn generated_identity(kind: RelationKind) -> ColumnIdentityObservation {
    ColumnIdentityObservation::generated_always("public", "metric", kind, "metric_id")
        .expect("identity fixture is structurally valid")
}

fn generated_column(kind: RelationKind) -> ColumnGenerationObservation {
    ColumnGenerationObservation::stored("public", "metric", kind, "metric_id")
        .expect("generation fixture is structurally valid")
}

#[test]
fn identity_modes_are_restricted_to_table_and_foreign_table_relation_kinds() {
    for kind in [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::ForeignTable,
    ] {
        identity_snapshot(kind, generated_identity(kind))
            .expect("table and foreign-table DDL can carry identity catalog state");
    }

    for kind in [
        RelationKind::View,
        RelationKind::MaterializedView,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        let error = identity_snapshot(kind, generated_identity(kind))
            .expect_err("unsupported relation kinds cannot carry non-empty attidentity state");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "column_identity_relation_kind",
            }
        );
    }
}

#[test]
fn explicit_not_identity_remains_valid_on_non_table_relations() {
    let observation = ColumnIdentityObservation::not_identity(
        "public",
        "metric",
        RelationKind::View,
        "metric_id",
    )
    .expect("empty attidentity evidence is structurally valid");
    identity_snapshot(RelationKind::View, observation)
        .expect("views still expose the catalog-empty identity state");
}

#[test]
fn generated_modes_are_restricted_to_table_and_foreign_table_relation_kinds() {
    for kind in [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::ForeignTable,
    ] {
        generation_snapshot(kind, generated_column(kind))
            .expect("table and foreign-table DDL can carry generated-column catalog state");
    }

    for kind in [
        RelationKind::View,
        RelationKind::MaterializedView,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ] {
        let error = generation_snapshot(kind, generated_column(kind))
            .expect_err("unsupported relation kinds cannot carry non-empty attgenerated state");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "column_generation_relation_kind",
            }
        );
    }
}

#[test]
fn explicit_not_generated_remains_valid_on_non_table_relations() {
    let observation = ColumnGenerationObservation::not_generated(
        "public",
        "metric",
        RelationKind::View,
        "metric_id",
    )
    .expect("empty attgenerated evidence is structurally valid");
    generation_snapshot(RelationKind::View, observation)
        .expect("views still expose the catalog-empty generation state");
}
