use conceptweave_observation::{
    ColumnObservationV3, NotNullConstraintObservation, ObservationError, PostgresSchemaSnapshotV3,
    PrimaryKeyObservation, QualifiedTypeName, RelationKind, RelationObservation,
    TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn relation(required_columns: &[&str]) -> RelationObservation {
    RelationObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "raw_value",
                1,
                "numeric",
                catalog_type("numeric"),
                !required_columns.contains(&"raw_value"),
                None,
            )
            .expect("raw_value fixture is valid"),
            ColumnObservationV3::new(
                "note",
                2,
                "text",
                catalog_type("text"),
                !required_columns.contains(&"note"),
                None,
            )
            .expect("note fixture is valid"),
        ],
    )
    .expect("metric relation fixture is valid")
}

fn relation_with_primary_key(required_columns: &[&str], primary_key_columns: &[&str]) -> RelationObservation {
    relation(required_columns)
        .with_constraints(vec![TableConstraintObservation::PrimaryKey(
            PrimaryKeyObservation::new(
                "metric_pkey",
                primary_key_columns
                    .iter()
                    .map(|column_name| (*column_name).to_owned())
                    .collect(),
            )
            .expect("primary-key fixture is valid"),
        )])
        .expect("relation with primary-key fixture is valid")
}

fn not_null(
    constraint_name: &str,
    column_name: &str,
    validated: bool,
    enforced: bool,
    is_local: bool,
    inheritance_ancestor_count: u16,
    no_inherit: bool,
) -> NotNullConstraintObservation {
    NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        constraint_name,
        column_name,
        validated,
        enforced,
        is_local,
        inheritance_ancestor_count,
        no_inherit,
    )
    .expect("NOT NULL constraint evidence is valid")
}

fn snapshot(
    required_columns: &[&str],
    constraints: Vec<NotNullConstraintObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T06:55:00Z",
        vec![relation(required_columns)],
        Vec::new(),
        Vec::new(),
        constraints,
    )
}

#[test]
fn not_null_constraint_name_is_governed_identity() {
    let first = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            true,
            true,
            true,
            0,
            false,
        )],
    )
    .expect("first NOT NULL family is valid");
    let second = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_required",
            "raw_value",
            true,
            true,
            true,
            0,
            false,
        )],
    )
    .expect("second NOT NULL family is valid");

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn not_null_validation_and_enforcement_are_governed_identity() {
    let valid = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            true,
            true,
            true,
            0,
            false,
        )],
    )
    .expect("validated NOT NULL family is valid");
    let not_valid = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            false,
            true,
            true,
            0,
            false,
        )],
    )
    .expect("NOT VALID constraint is still source-authoritative evidence");
    let not_enforced = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            false,
            false,
            true,
            0,
            false,
        )],
    )
    .expect("NOT ENFORCED constraint is still source-authoritative evidence");

    assert_ne!(valid.snapshot_digest(), not_valid.snapshot_digest());
    assert_ne!(not_valid.snapshot_digest(), not_enforced.snapshot_digest());
}

#[test]
fn not_null_inheritance_state_is_governed_identity() {
    let local = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            true,
            true,
            true,
            0,
            false,
        )],
    )
    .expect("local constraint is valid");
    let inherited = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            true,
            true,
            false,
            1,
            false,
        )],
    )
    .expect("inherited constraint is valid");
    let no_inherit = snapshot(
        &["raw_value"],
        vec![not_null(
            "metric_raw_value_not_null",
            "raw_value",
            true,
            true,
            true,
            0,
            true,
        )],
    )
    .expect("NO INHERIT constraint is valid");

    assert_ne!(local.snapshot_digest(), inherited.snapshot_digest());
    assert_ne!(local.snapshot_digest(), no_inherit.snapshot_digest());
}

#[test]
fn not_null_inheritance_ancestor_count_must_fit_postgresql_int2() {
    let error = NotNullConstraintObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        "metric_raw_value_not_null",
        "raw_value",
        true,
        true,
        false,
        32_768,
        false,
    )
    .expect_err("pg_constraint.coninhcount is PostgreSQL int2 and cannot represent 32768");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_inheritance_ancestor_count",
        }
    );
}

#[test]
fn observed_empty_not_null_family_is_distinct_from_unobserved() {
    let unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T06:55:00Z",
        vec![relation(&[])],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot is valid");
    let observed = snapshot(&[], Vec::new()).expect("observed-empty NOT NULL family is valid");

    assert_ne!(unobserved.snapshot_digest(), observed.snapshot_digest());
    assert!(
        observed
            .not_null_constraints()
            .expect("observed NOT NULL family remains queryable")
            .is_empty()
    );
}

#[test]
fn primary_key_can_back_non_nullable_column_without_explicit_not_null_row() {
    let observed = PostgresSchemaSnapshotV3::new_with_not_null_constraints(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T06:55:00Z",
        vec![relation_with_primary_key(&["raw_value"], &["raw_value"])],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    )
    .expect("PRIMARY KEY can be the backing constraint for attnotnull without a contype='n' row");

    assert!(
        observed
            .not_null_constraints()
            .expect("observed NOT NULL inventory remains queryable")
            .is_empty()
    );
}

#[test]
fn not_null_constraint_on_nullable_column_fails_closed() {
    let error = snapshot(
        &["raw_value"],
        vec![
            not_null(
                "metric_raw_value_not_null",
                "raw_value",
                true,
                true,
                true,
                0,
                false,
            ),
            not_null(
                "metric_note_not_null",
                "note",
                true,
                true,
                true,
                0,
                false,
            ),
        ],
    )
    .expect_err("nullable summary and NOT NULL constraint inventory cannot contradict");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_nullability",
        }
    );
}

#[test]
fn duplicate_not_null_constraint_for_one_column_fails_closed() {
    let error = snapshot(
        &["raw_value"],
        vec![
            not_null(
                "metric_raw_value_not_null",
                "raw_value",
                true,
                true,
                true,
                0,
                false,
            ),
            not_null(
                "metric_raw_value_required",
                "raw_value",
                true,
                true,
                true,
                0,
                false,
            ),
        ],
    )
    .expect_err("PostgreSQL 18 permits only one explicit NOT NULL constraint per column");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_column",
        }
    );
}

#[test]
fn not_null_constraint_input_order_does_not_change_identity() {
    let first = snapshot(
        &["raw_value", "note"],
        vec![
            not_null(
                "metric_raw_value_not_null",
                "raw_value",
                true,
                true,
                true,
                0,
                false,
            ),
            not_null(
                "metric_note_not_null",
                "note",
                true,
                true,
                true,
                0,
                false,
            ),
        ],
    )
    .expect("first order is valid");
    let second = snapshot(
        &["raw_value", "note"],
        vec![
            not_null(
                "metric_note_not_null",
                "note",
                true,
                true,
                true,
                0,
                false,
            ),
            not_null(
                "metric_raw_value_not_null",
                "raw_value",
                true,
                true,
                true,
                0,
                false,
            ),
        ],
    )
    .expect("second order is valid");

    assert_eq!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn not_null_family_can_only_be_attached_once() {
    let observed = snapshot(&[], Vec::new()).expect("observed-empty NOT NULL family is valid");
    let error = observed
        .with_observed_not_null_constraints(Vec::new())
        .expect_err("the same first-class NOT NULL family cannot extend identity twice");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "not_null_constraint_already_observed",
        }
    );
}

#[test]
fn earlier_optional_family_cannot_be_attached_after_not_null_evidence() {
    let observed = snapshot(&[], Vec::new()).expect("observed-empty NOT NULL family is valid");
    let error = observed
        .with_observed_type_kinds(Vec::new())
        .expect_err("late type-kind attachment cannot create a second digest ordering");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "type_kind_observation_order",
        }
    );
}
