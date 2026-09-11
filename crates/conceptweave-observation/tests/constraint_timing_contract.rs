use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, ConstraintDeferrability,
    ConstraintTimingObservation, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    PrimaryKeyObservation, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation, UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn base_relation() -> RelationObservation {
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
}

fn relation(constraint: TableConstraintObservation) -> RelationObservation {
    base_relation()
        .with_constraints(vec![constraint])
        .expect("constraint fixture is valid")
}

fn backing_index(
    constraint_name: &str,
    primary: bool,
    deferrability: ConstraintDeferrability,
) -> IndexObservation {
    let immediate = matches!(deferrability, ConstraintDeferrability::NotDeferrable);
    IndexObservation::new(
        constraint_name,
        true,
        Some(false),
        vec![IndexAttributeObservation::new(
            1,
            IndexAttributeKind::Key,
            "document_id",
        )
        .expect("key fixture is valid")],
        Vec::new(),
    )
    .expect("backing-index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
            .expect("operator-class fixture is valid"),
        0,
    )
    .expect("key semantics fixture is valid")])
    .expect("one semantic record matches the single key position")
    .with_catalog_flags(IndexCatalogFlags::new(
        primary,
        false,
        immediate,
        false,
        false,
        false,
    ))
    .expect("catalog-flag fixture is coherent")
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
    let key_shape = timings.first().and_then(|timing| match &constraint {
        TableConstraintObservation::PrimaryKey(primary_key) => Some((
            primary_key.constraint_name().to_owned(),
            true,
            timing.deferrability(),
        )),
        TableConstraintObservation::Unique(unique) => Some((
            unique.constraint_name().to_owned(),
            false,
            timing.deferrability(),
        )),
        TableConstraintObservation::ForeignKey(_) | TableConstraintObservation::Check(_) => None,
    });
    let observed_relation = match key_shape {
        Some((constraint_name, primary, deferrability)) => relation(constraint)
            .with_indexes(vec![backing_index(
                &constraint_name,
                primary,
                deferrability,
            )])?,
        None => relation(constraint),
    };

    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T16:48:00Z",
        vec![observed_relation],
        Vec::new(),
        Vec::new(),
        timings,
    )
}

#[test]
fn coherent_primary_key_deferrability_states_change_governed_identity() {
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
    let relations = vec![base_relation()];
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
    .expect("explicitly observed-empty timing family is valid when no key constraints exist");

    assert_ne!(unobserved.snapshot_digest(), observed_empty.snapshot_digest());
    assert_eq!(unobserved.constraint_timings(), None);
    assert_eq!(observed_empty.constraint_timings(), Some(&[][..]));
}

#[test]
fn observed_timing_inventory_must_cover_every_key_constraint() {
    let error = snapshot(primary_key(), Vec::new())
        .expect_err("observed timing family cannot silently omit a primary key");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_timing_completeness",
        }
    );
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
