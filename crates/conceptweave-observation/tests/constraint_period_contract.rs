use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, ConstraintDeferrability,
    ConstraintPeriodObservation, ConstraintTimingObservation, ForeignKeyAction,
    ForeignKeyDeferrability, ForeignKeyMatchType, ForeignKeyObservation,
    ForeignKeyReferenceBehavior, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
    UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn column(
    name: &str,
    position: u32,
    data_type: &str,
    type_name: &str,
    nullable: bool,
) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        position,
        data_type,
        catalog_type(type_name),
        nullable,
        None,
    )
    .expect("column fixture is valid")
}

fn backing_index(
    name: &str,
    columns: &[&str],
    primary: bool,
    exclusion: bool,
    access_method: &str,
) -> IndexObservation {
    let attributes = columns
        .iter()
        .enumerate()
        .map(|(index, column_name)| {
            IndexAttributeObservation::new(
                u32::try_from(index + 1).expect("fixture position fits u32"),
                IndexAttributeKind::Key,
                *column_name,
            )
            .expect("index attribute fixture is valid")
        })
        .collect();
    IndexObservation::new(name, true, Some(false), attributes, Vec::new())
        .expect("backing-index fixture is structurally valid")
        .with_access_method(access_method)
        .with_catalog_flags(IndexCatalogFlags::new(
            primary,
            exclusion,
            true,
            false,
            false,
            false,
        ))
        .expect("catalog-flag fixture is coherent")
}

fn ordinary_unique_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![column("document_id", 1, "bigint", "int8", false)],
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::Unique(
        UniqueConstraintObservation::new("document_id_key", vec!["document_id".to_owned()])
            .expect("unique fixture is valid"),
    )])
    .expect("constraint fixture is valid")
    .with_indexes(vec![backing_index(
        "document_id_key",
        &["document_id"],
        false,
        false,
        "btree",
    )])
    .expect("index fixture is valid")
}

fn temporal_parent_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            column("document_id", 1, "bigint", "int8", false),
            column("valid_during", 2, "tstzrange", "tstzrange", false),
        ],
    )
    .expect("parent relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("temporal primary-key fixture is valid"),
    )])
    .expect("parent constraint fixture is valid")
    .with_indexes(vec![backing_index(
        "document_temporal_key",
        &["document_id", "valid_during"],
        true,
        true,
        "gist",
    )])
    .expect("temporal backing-index fixture is valid")
}

fn period_child_relation() -> RelationObservation {
    let behavior = ForeignKeyReferenceBehavior::new(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::NoAction,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    );
    RelationObservation::new(
        "public",
        "document_version",
        RelationKind::Table,
        vec![
            column("document_id", 1, "bigint", "int8", false),
            column("valid_during", 2, "tstzrange", "tstzrange", false),
        ],
    )
    .expect("child relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::ForeignKey(
        ForeignKeyObservation::with_reference_behavior(
            "document_version_period_fk",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
            "public",
            "document",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
            behavior,
        )
        .expect("period foreign-key fixture is structurally valid"),
    )])
    .expect("child constraint fixture is valid")
}

fn single_column_foreign_key_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "document_version",
        RelationKind::Table,
        vec![column(
            "valid_during",
            1,
            "tstzrange",
            "tstzrange",
            false,
        )],
    )
    .expect("single-column child relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::ForeignKey(
        ForeignKeyObservation::new(
            "document_version_period_fk",
            vec!["valid_during".to_owned()],
            "public",
            "document",
            vec!["valid_during".to_owned()],
        )
        .expect("single-column foreign-key fixture is structurally valid"),
    )])
    .expect("single-column child constraint fixture is valid")
}

fn temporal_key_timing() -> ConstraintTimingObservation {
    ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        ConstraintDeferrability::NotDeferrable,
    )
    .expect("referenced temporal-key timing fixture is valid")
}

fn period(
    relation_name: &str,
    constraint_name: &str,
    has_period_semantics: bool,
) -> ConstraintPeriodObservation {
    ConstraintPeriodObservation::new(
        "public",
        relation_name,
        RelationKind::Table,
        constraint_name,
        has_period_semantics,
    )
    .expect("constraint-period fixture is valid")
}

fn base_snapshot(relations: Vec<RelationObservation>) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T22:00:00Z",
        relations,
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot fixture is valid")
}

fn timed_base_snapshot(relations: Vec<RelationObservation>) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T22:00:00Z",
        relations,
        Vec::new(),
        Vec::new(),
        vec![temporal_key_timing()],
    )
    .expect("timed base snapshot fixture is valid")
}

#[test]
fn observed_false_period_state_is_distinct_from_unobserved() {
    let relation = ordinary_unique_relation();
    let unobserved = base_snapshot(vec![relation.clone()]);
    let observed_false = base_snapshot(vec![relation])
        .with_observed_constraint_periods(vec![period("document", "document_id_key", false)])
        .expect("ordinary UNIQUE has explicit conperiod=false");

    assert_ne!(unobserved.snapshot_digest(), observed_false.snapshot_digest());
    assert_eq!(unobserved.constraint_periods(), None);
    assert!(
        !observed_false
            .constraint_periods()
            .expect("period family was observed")[0]
            .has_period_semantics()
    );
}

#[test]
fn temporal_primary_key_requires_matching_exclusion_backing_index_evidence() {
    let accepted = base_snapshot(vec![temporal_parent_relation()])
        .with_observed_constraint_periods(vec![period(
            "document",
            "document_temporal_key",
            true,
        )])
        .expect("WITHOUT OVERLAPS primary key has coherent explicit conperiod=true evidence");
    assert!(accepted.constraint_periods().is_some());

    let error = base_snapshot(vec![temporal_parent_relation()])
        .with_observed_constraint_periods(vec![period(
            "document",
            "document_temporal_key",
            false,
        )])
        .expect_err("explicit conperiod=false cannot contradict an observed exclusion backing index");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_backing_index",
        }
    );
}

#[test]
fn period_foreign_key_and_referenced_temporal_key_are_preserved_together() {
    let snapshot = timed_base_snapshot(vec![temporal_parent_relation(), period_child_relation()])
        .with_observed_constraint_periods(vec![
            period("document_version", "document_version_period_fk", true),
            period("document", "document_temporal_key", true),
        ])
        .expect("PERIOD foreign key targets an observed non-deferrable WITHOUT OVERLAPS primary key");

    let periods = snapshot
        .constraint_periods()
        .expect("period family was explicitly observed");
    assert_eq!(periods.len(), 2);
    assert!(periods.iter().all(ConstraintPeriodObservation::has_period_semantics));
}

#[test]
fn observed_period_family_must_cover_keys_and_foreign_keys() {
    let error = base_snapshot(vec![temporal_parent_relation(), period_child_relation()])
        .with_observed_constraint_periods(vec![period(
            "document",
            "document_temporal_key",
            true,
        )])
        .expect_err("observed conperiod family cannot silently omit a foreign key");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_completeness",
        }
    );
}

#[test]
fn period_foreign_key_requires_a_nonperiod_key_prefix() {
    let error = base_snapshot(vec![single_column_foreign_key_relation()])
        .with_observed_constraint_periods(vec![period(
            "document_version",
            "document_version_period_fk",
            true,
        )])
        .expect_err("PostgreSQL PERIOD foreign keys require at least one equality-key column");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_shape",
        }
    );
}

#[test]
fn period_observation_cannot_target_check_constraint() {
    let relation = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![column("document_id", 1, "bigint", "int8", false)],
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::Check(
        CheckConstraintObservation::new(
            "document_positive",
            "document_id > 0",
            true,
            true,
            false,
        )
        .expect("check fixture is valid"),
    )])
    .expect("check constraint fixture is valid");

    let error = base_snapshot(vec![relation])
        .with_observed_constraint_periods(vec![period(
            "document",
            "document_positive",
            false,
        )])
        .expect_err("conperiod semantics apply only to PRIMARY KEY, UNIQUE, and FOREIGN KEY rows");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_kind",
        }
    );
}

#[test]
fn period_family_input_order_does_not_change_identity() {
    let relations = vec![temporal_parent_relation(), period_child_relation()];
    let forward = timed_base_snapshot(relations.clone())
        .with_observed_constraint_periods(vec![
            period("document", "document_temporal_key", true),
            period("document_version", "document_version_period_fk", true),
        ])
        .expect("forward period inventory is valid");
    let reverse = timed_base_snapshot(relations)
        .with_observed_constraint_periods(vec![
            period("document_version", "document_version_period_fk", true),
            period("document", "document_temporal_key", true),
        ])
        .expect("reverse period inventory is valid");

    assert_eq!(forward.snapshot_digest(), reverse.snapshot_digest());
}
