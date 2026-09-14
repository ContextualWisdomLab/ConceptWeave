use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

#[test]
fn ordinary_table_rejects_not_enforced_not_null() {
    let error = NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        false,
        false,
        true,
        0,
        false,
    )
    .expect_err("ordinary PostgreSQL 18 tables do not support NOT ENFORCED NOT NULL constraints");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_enforcement",
        }
    );
}

#[test]
fn foreign_table_preserves_not_enforced_not_null() {
    let observation = NotNullConstraintObservation::new(
        "remote",
        "metric",
        RelationKind::ForeignTable,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        false,
        true,
        0,
        false,
    )
    .expect("PostgreSQL 18 CREATE FOREIGN TABLE permits NOT NULL NOT ENFORCED");

    assert_eq!(observation.relation_kind(), RelationKind::ForeignTable);
    assert!(!observation.enforced());
}

#[test]
fn foreign_table_enforcement_state_is_governed_identity() {
    let enforced = foreign_snapshot(true);
    let not_enforced = foreign_snapshot(false);

    assert_ne!(enforced.snapshot_digest(), not_enforced.snapshot_digest());
}

fn foreign_snapshot(enforced: bool) -> PostgresSchemaSnapshotV3 {
    let relation = RelationObservation::new(
        "remote",
        "metric",
        RelationKind::ForeignTable,
        vec![ColumnObservationV3::new(
            "raw_value",
            1,
            "numeric",
            QualifiedTypeName::new("pg_catalog", "numeric").expect("catalog type is valid"),
            false,
            None,
        )
        .expect("foreign-table column fixture is valid")],
    )
    .expect("foreign-table relation fixture is valid");
    let constraint = NotNullConstraintObservation::new(
        "remote",
        "metric",
        RelationKind::ForeignTable,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        enforced,
        true,
        0,
        false,
    )
    .expect("foreign-table NOT NULL enforcement state is source-representable");

    PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["remote"]),
        "postgres_introspector_v3",
        "2026-09-14T09:45:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        vec![constraint],
    )
    .expect("foreign-table NOT NULL snapshot is valid")
}
