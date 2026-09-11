use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, ConstraintDeferrability,
    ConstraintTimingObservation, ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
    UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn relation(constraint: TableConstraintObservation) -> RelationObservation {
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
    .with_constraints(vec![constraint])
    .expect("constraint fixture is valid")
}

fn primary_key() -> TableConstraintObservation {
    TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
            .expect("primary-key fixture is valid"),
    )
}

fn unique() -> TableConstraintObservation {
    TableConstraintObservation::Unique(
        UniqueConstraintObservation::new("document_id_key", vec!["document_id".to_owned()])
            .expect("unique fixture is valid"),
    )
}

fn timing(
    constraint_name: &str,
    deferrability: ConstraintDeferrability,
) -> ConstraintTimingObservation {
    ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        constraint_name,
        deferrability,
    )
    .expect("constraint timing fixture is valid")
}

fn snapshot(
    constraint: TableConstraintObservation,
    timings: Vec<ConstraintTimingObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T16:48:00Z",
        vec![relation(constraint)],
        Vec::new(),
        Vec::new(),
        timings,
    )
}

#[test]
fn primary_key_deferrability_changes_governed_identity() {
    let not_deferrable = snapshot(
        primary_key(),
        vec![timing(
            "document_pkey",
            ConstraintDeferrability::NotDeferrable,
        )],
    )
    .expect("NOT DEFERRABLE primary key is valid");
    let initially_immediate = snapshot(
        primary_key(),
        vec![timing(
            "document_pkey",
            ConstraintDeferrability::InitiallyImmediate,
        )],
    )
    .expect("DEFERRABLE INITIALLY IMMEDIATE primary key is valid");
    let initially_deferred = snapshot(
        primary_key(),
        vec![timing(
            "document_pkey",
            ConstraintDeferrability::InitiallyDeferred,
        )],
    )
    .expect("DEFERRABLE INITIALLY DEFERRED primary key is valid");

    assert_ne!(not_deferrable.snapshot_digest(), initially_immediate.snapshot_digest());
    assert_ne!(not_deferrable.snapshot_digest(), initially_deferred.snapshot_digest());
    assert_ne!(initially_immediate.snapshot_digest(), initially_deferred.snapshot_digest());
}

#[test]
fn unique_deferrability_changes_governed_identity() {
    let immediate = snapshot(
        unique(),
        vec![timing(
            "document_id_key",
            ConstraintDeferrability::InitiallyImmediate,
        )],
    )
    .expect("immediate UNIQUE timing is valid");
    let deferred = snapshot(
        unique(),
        vec![timing(
            "document_id_key",
            ConstraintDeferrability::InitiallyDeferred,
        )],
    )
    .expect("deferred UNIQUE timing is valid");

    assert_ne!(immediate.snapshot_digest(), deferred.snapshot_digest());
}

#[test]
fn observed_empty_timing_inventory_differs_from_unobserved() {
    let relations = vec![relation(primary_key())];
    let unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T16:48:00Z",
        relations.clone(),
        Vec::new(),
        Vec::new(),
    )
    .expect("legacy v3 constructor remains valid");
    let observed_empty = PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T16:48:00Z",
        relations,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .expect("explicitly observed-empty timing family is valid");

    assert_ne!(unobserved.snapshot_digest(), observed_empty.snapshot_digest());
    assert_eq!(unobserved.constraint_timings(), None);
    assert_eq!(observed_empty.constraint_timings(), Some(&[][..]));
}

#[test]
fn timing_cannot_target_non_key_constraint() {
    let check = TableConstraintObservation::Check(
        CheckConstraintObservation::new(
            "document_positive",
            "document_id > 0",
            true,
            true,
            false,
        )
        .expect("check fixture is valid"),
    );
    let error = snapshot(
        check,
        vec![timing(
            "document_positive",
            ConstraintDeferrability::NotDeferrable,
        )],
    )
    .expect_err("timing family is restricted to PRIMARY KEY and UNIQUE constraints");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_timing_kind",
        }
    );
}
