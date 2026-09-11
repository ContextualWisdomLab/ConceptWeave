use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintPeriodObservation,
    ConstraintTimingObservation, ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType,
    ForeignKeyObservation, ForeignKeyReferenceBehavior, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedTypeName, RelationKind,
    RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn column(name: &str, position: u32, type_name: &str) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        position,
        type_name,
        catalog_type(type_name),
        false,
        None,
    )
    .expect("column fixture is valid")
}

fn temporal_parent_relation() -> RelationObservation {
    let attributes = ["document_id", "valid_during"]
        .into_iter()
        .enumerate()
        .map(|(index, column_name)| {
            IndexAttributeObservation::new(
                u32::try_from(index + 1).expect("fixture position fits u32"),
                IndexAttributeKind::Key,
                column_name,
            )
            .expect("index attribute fixture is valid")
        })
        .collect();
    let backing_index = IndexObservation::new(
        "document_temporal_key",
        true,
        Some(false),
        attributes,
        Vec::new(),
    )
    .expect("backing index fixture is valid")
    .with_access_method("gist")
    .with_catalog_flags(IndexCatalogFlags::new(
        true, true, true, false, false, false,
    ))
    .expect("catalog flags are coherent");

    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            column("document_id", 1, "int8"),
            column("valid_during", 2, "tstzrange"),
        ],
    )
    .expect("parent relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("temporal primary key fixture is valid"),
    )])
    .expect("parent constraint fixture is valid")
    .with_indexes(vec![backing_index])
    .expect("parent index fixture is valid")
}

fn period_child_relation(
    reference_behavior: Option<ForeignKeyReferenceBehavior>,
) -> RelationObservation {
    let column_names = vec!["document_id".to_owned(), "valid_during".to_owned()];
    let referenced_column_names = column_names.clone();
    let foreign_key = match reference_behavior {
        Some(behavior) => ForeignKeyObservation::with_reference_behavior(
            "document_version_period_fk",
            column_names,
            "public",
            "document",
            referenced_column_names,
            behavior,
        ),
        None => ForeignKeyObservation::new(
            "document_version_period_fk",
            column_names,
            "public",
            "document",
            referenced_column_names,
        ),
    }
    .expect("foreign key fixture is structurally valid");

    RelationObservation::new(
        "public",
        "document_version",
        RelationKind::Table,
        vec![
            column("document_id", 1, "int8"),
            column("valid_during", 2, "tstzrange"),
        ],
    )
    .expect("child relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::ForeignKey(foreign_key)])
    .expect("child constraint fixture is valid")
}

fn reference_behavior(
    update_action: ForeignKeyAction,
    delete_action: ForeignKeyAction,
) -> ForeignKeyReferenceBehavior {
    ForeignKeyReferenceBehavior::new(
        update_action,
        delete_action,
        ForeignKeyMatchType::Simple,
        ForeignKeyDeferrability::NotDeferrable,
    )
}

fn timing() -> ConstraintTimingObservation {
    ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        ConstraintDeferrability::NotDeferrable,
    )
    .expect("referenced temporal-key timing fixture is valid")
}

fn period(relation_name: &str, constraint_name: &str) -> ConstraintPeriodObservation {
    ConstraintPeriodObservation::new(
        "public",
        relation_name,
        RelationKind::Table,
        constraint_name,
        true,
    )
    .expect("period observation fixture is valid")
}

fn snapshot_with_temporal_fk(
    reference_behavior: Option<ForeignKeyReferenceBehavior>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T23:00:00Z",
        vec![
            temporal_parent_relation(),
            period_child_relation(reference_behavior),
        ],
        Vec::new(),
        Vec::new(),
    )?
    .with_observed_constraint_timings(vec![timing()])?
    .with_observed_constraint_periods(vec![
        period("document", "document_temporal_key"),
        period("document_version", "document_version_period_fk"),
    ])
}

#[test]
fn temporal_foreign_key_accepts_no_action_reference_behavior() {
    snapshot_with_temporal_fk(Some(reference_behavior(
        ForeignKeyAction::NoAction,
        ForeignKeyAction::NoAction,
    )))
    .expect("PostgreSQL temporal foreign keys support NO ACTION");
}

#[test]
fn temporal_foreign_key_requires_observed_reference_behavior() {
    let error = snapshot_with_temporal_fk(None)
        .expect_err("a governed PERIOD foreign key needs observed referential actions");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_action",
        }
    );
}

#[test]
fn temporal_foreign_key_rejects_unsupported_update_actions() {
    for action in [
        ForeignKeyAction::Restrict,
        ForeignKeyAction::Cascade,
        ForeignKeyAction::SetNull,
        ForeignKeyAction::SetDefault,
    ] {
        let error = snapshot_with_temporal_fk(Some(reference_behavior(
            action,
            ForeignKeyAction::NoAction,
        )))
        .expect_err("PostgreSQL temporal foreign keys reject this ON UPDATE action");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "constraint_period_action",
            }
        );
    }
}

#[test]
fn temporal_foreign_key_rejects_unsupported_delete_actions() {
    for action in [
        ForeignKeyAction::Restrict,
        ForeignKeyAction::Cascade,
        ForeignKeyAction::SetNull,
        ForeignKeyAction::SetDefault,
    ] {
        let error = snapshot_with_temporal_fk(Some(reference_behavior(
            ForeignKeyAction::NoAction,
            action,
        )))
        .expect_err("PostgreSQL temporal foreign keys reject this ON DELETE action");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "constraint_period_action",
            }
        );
    }
}
