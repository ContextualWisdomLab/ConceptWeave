use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, ForeignKeyAction, ForeignKeyDeferrability,
    ForeignKeyMatchType, ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationError,
    PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedCollationName, QualifiedTypeName,
    RelationKind, RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn collation(name: &str) -> QualifiedCollationName {
    QualifiedCollationName::new("public", name).expect("collation coordinate is valid")
}

fn parent_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "parent",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new("id", 1, "text", catalog_type("text"), false, None)
                .expect("parent column fixture is valid"),
        ],
    )
    .expect("parent relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("parent_pkey", vec!["id".to_owned()])
            .expect("primary-key fixture is valid"),
    )])
    .expect("parent constraints are valid")
}

fn child_relation() -> RelationObservation {
    let foreign_key = ForeignKeyObservation::with_reference_behavior(
        "child_parent_fk",
        vec!["parent_id".to_owned()],
        "public",
        "parent",
        vec!["id".to_owned()],
        ForeignKeyReferenceBehavior::new(
            ForeignKeyAction::NoAction,
            ForeignKeyAction::NoAction,
            ForeignKeyMatchType::Simple,
            ForeignKeyDeferrability::NotDeferrable,
        ),
    )
    .expect("foreign-key fixture is valid");

    RelationObservation::new(
        "public",
        "child",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new("parent_id", 1, "text", catalog_type("text"), false, None)
                .expect("child column fixture is valid"),
        ],
    )
    .expect("child relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::ForeignKey(foreign_key)])
    .expect("child constraints are valid")
}

fn collatable(
    relation_name: &str,
    column_name: &str,
    collation_name: &str,
    deterministic: bool,
) -> ColumnCollationObservation {
    ColumnCollationObservation::collatable(
        "public",
        relation_name,
        RelationKind::Table,
        column_name,
        collation(collation_name),
        deterministic,
    )
    .expect("column-collation fixture is valid")
}

fn snapshot(
    relations: Vec<RelationObservation>,
    column_collations: Vec<ColumnCollationObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_column_collations(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T05:12:00Z",
        relations,
        Vec::new(),
        Vec::new(),
        column_collations,
    )
}

#[test]
fn exact_column_collation_changes_governed_identity() {
    let first = snapshot(
        vec![parent_relation()],
        vec![collatable("parent", "id", "en_deterministic", true)],
    )
    .expect("first collated snapshot is valid");
    let second = snapshot(
        vec![parent_relation()],
        vec![collatable("parent", "id", "de_deterministic", true)],
    )
    .expect("second collated snapshot is valid");

    assert_ne!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "pg_attribute.attcollation is material source identity even when both collations are deterministic"
    );
}

#[test]
fn observed_uncollatable_is_distinct_from_unobserved_collation_family() {
    let relations = vec![
        RelationObservation::new(
            "public",
            "metric",
            RelationKind::Table,
            vec![
                ColumnObservationV3::new("value", 1, "int8", catalog_type("int8"), false, None)
                    .expect("metric column fixture is valid"),
            ],
        )
        .expect("metric relation fixture is valid"),
    ];

    let unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T05:12:00Z",
        relations.clone(),
        Vec::new(),
        Vec::new(),
    )
    .expect("legacy-compatible unobserved snapshot is valid");
    let observed = snapshot(
        relations,
        vec![
            ColumnCollationObservation::uncollatable(
                "public",
                "metric",
                RelationKind::Table,
                "value",
            )
            .expect("explicit uncollatable evidence is valid"),
        ],
    )
    .expect("observed-uncollatable snapshot is valid");

    assert_ne!(
        unobserved.snapshot_digest(),
        observed.snapshot_digest(),
        "an explicitly observed attcollation=0 family must not collapse into unobserved evidence"
    );
    let evidence = observed
        .column_collations()
        .expect("observed family remains queryable");
    assert_eq!(evidence.len(), 1);
    assert_eq!(evidence[0].schema_name(), "public");
    assert_eq!(evidence[0].relation_name(), "metric");
    assert_eq!(evidence[0].column_name(), "value");
    assert_eq!(evidence[0].collation(), None);
    assert_eq!(evidence[0].deterministic(), None);
}

#[test]
fn observed_family_must_cover_every_bounded_column() {
    let error = snapshot(
        vec![parent_relation(), child_relation()],
        vec![collatable("parent", "id", "shared", true)],
    )
    .expect_err("an observed family with a missing bounded column must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_collation_completeness",
        }
    );
}

#[test]
fn duplicate_column_collation_coordinate_fails_closed() {
    let error = snapshot(
        vec![parent_relation()],
        vec![
            collatable("parent", "id", "first", true),
            collatable("parent", "id", "second", true),
        ],
    )
    .expect_err("two observations for one exact column coordinate are contradictory");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_collation_coordinate",
        }
    );
}

#[test]
fn one_qualified_collation_cannot_claim_conflicting_determinism() {
    let error = snapshot(
        vec![parent_relation(), child_relation()],
        vec![
            collatable("parent", "id", "shared", true),
            collatable("child", "parent_id", "shared", false),
        ],
    )
    .expect_err("one pg_collation coordinate cannot carry two deterministic states");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_collation_determinism",
        }
    );
}

#[test]
fn column_collation_input_order_does_not_change_identity() {
    let first = snapshot(
        vec![parent_relation(), child_relation()],
        vec![
            collatable("parent", "id", "parent_deterministic", true),
            collatable("child", "parent_id", "child_deterministic", true),
        ],
    )
    .expect("first input ordering is valid");
    let second = snapshot(
        vec![child_relation(), parent_relation()],
        vec![
            collatable("child", "parent_id", "child_deterministic", true),
            collatable("parent", "id", "parent_deterministic", true),
        ],
    )
    .expect("second input ordering is valid");

    assert_eq!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "input order must not create a second governed source identity"
    );
}

#[test]
fn foreign_key_rejects_different_collations_when_either_side_is_nondeterministic() {
    let error = snapshot(
        vec![parent_relation(), child_relation()],
        vec![
            collatable("parent", "id", "parent_nd", false),
            collatable("child", "parent_id", "child_nd", false),
        ],
    )
    .expect_err("different nondeterministic FK collations must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "foreign_key_collation",
        }
    );
}

#[test]
fn foreign_key_rejects_different_collations_when_only_one_side_is_nondeterministic() {
    let error = snapshot(
        vec![parent_relation(), child_relation()],
        vec![
            collatable("parent", "id", "parent_deterministic", true),
            collatable("child", "parent_id", "child_nd", false),
        ],
    )
    .expect_err("different FK collations with one nondeterministic side must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "foreign_key_collation",
        }
    );
}

#[test]
fn foreign_key_allows_different_deterministic_collations() {
    snapshot(
        vec![parent_relation(), child_relation()],
        vec![
            collatable("parent", "id", "parent_deterministic", true),
            collatable("child", "parent_id", "child_deterministic", true),
        ],
    )
    .expect("PostgreSQL permits a collatable FK pair when both collations are deterministic");
}

#[test]
fn foreign_key_allows_the_same_nondeterministic_collation() {
    snapshot(
        vec![parent_relation(), child_relation()],
        vec![
            collatable("parent", "id", "shared_nd", false),
            collatable("child", "parent_id", "shared_nd", false),
        ],
    )
    .expect("PostgreSQL permits the same nondeterministic collation on both FK sides");
}
