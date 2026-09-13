use conceptweave_observation::{
    ColumnGenerationObservation, ColumnIdentityObservation, ColumnObservationV3, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn one_column_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "value_normalized",
            1,
            "numeric",
            catalog_type("numeric"),
            true,
            None,
        )
        .expect("generated-column candidate fixture is valid")],
    )
    .expect("metric relation fixture is valid")
}

fn two_column_relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "metric",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "metric_id",
                1,
                "int8",
                catalog_type("int8"),
                false,
                None,
            )
            .expect("identity candidate fixture is valid"),
            ColumnObservationV3::new(
                "value_normalized",
                2,
                "numeric",
                catalog_type("numeric"),
                false,
                None,
            )
            .expect("generated-column candidate fixture is valid"),
        ],
    )
    .expect("metric relation fixture is valid")
}

fn not_generated(column_name: &str) -> ColumnGenerationObservation {
    ColumnGenerationObservation::not_generated(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("explicit ordinary-column generation evidence is valid")
}

fn stored(column_name: &str) -> ColumnGenerationObservation {
    ColumnGenerationObservation::stored(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("stored generated-column evidence is valid")
}

fn virtual_generated(column_name: &str) -> ColumnGenerationObservation {
    ColumnGenerationObservation::virtual_generated(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("virtual generated-column evidence is valid")
}

fn always_identity(column_name: &str) -> ColumnIdentityObservation {
    ColumnIdentityObservation::generated_always(
        "public",
        "metric",
        RelationKind::Table,
        column_name,
    )
    .expect("identity evidence is valid")
}

fn snapshot(
    relations: Vec<RelationObservation>,
    column_generations: Vec<ColumnGenerationObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_column_generations(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T00:47:00Z",
        relations,
        Vec::new(),
        Vec::new(),
        column_generations,
    )
}

#[test]
fn stored_and_virtual_generation_modes_have_distinct_governed_identity() {
    let stored_snapshot = snapshot(vec![one_column_relation()], vec![stored("value_normalized")])
        .expect("stored generated-column snapshot is valid");
    let virtual_snapshot = snapshot(
        vec![one_column_relation()],
        vec![virtual_generated("value_normalized")],
    )
    .expect("virtual generated-column snapshot is valid");

    assert_ne!(
        stored_snapshot.snapshot_digest(),
        virtual_snapshot.snapshot_digest(),
        "attgenerated='s' and 'v' have different storage/read-write semantics and must not collapse"
    );
}

#[test]
fn observed_not_generated_is_distinct_from_unobserved_generation_family() {
    let relations = vec![one_column_relation()];
    let unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T00:47:00Z",
        relations.clone(),
        Vec::new(),
        Vec::new(),
    )
    .expect("legacy-compatible unobserved snapshot is valid");
    let observed = snapshot(relations, vec![not_generated("value_normalized")])
        .expect("explicit not-generated snapshot is valid");

    assert_ne!(
        unobserved.snapshot_digest(),
        observed.snapshot_digest(),
        "observed attgenerated='' must remain distinct from unobserved generation evidence"
    );

    let evidence = observed
        .column_generations()
        .expect("observed generation family remains queryable");
    assert_eq!(evidence.len(), 1);
    assert!(evidence[0].is_not_generated());
}

#[test]
fn observed_generation_family_must_cover_every_bounded_column() {
    let error = snapshot(
        vec![two_column_relation()],
        vec![stored("value_normalized")],
    )
    .expect_err("an observed generation family missing metric_id must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_generation_completeness",
        }
    );
}

#[test]
fn duplicate_column_generation_coordinate_fails_closed() {
    let error = snapshot(
        vec![one_column_relation()],
        vec![stored("value_normalized"), virtual_generated("value_normalized")],
    )
    .expect_err("one exact column cannot have two attgenerated states");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_generation_coordinate",
        }
    );
}

#[test]
fn generation_input_order_does_not_change_identity() {
    let first = snapshot(
        vec![two_column_relation()],
        vec![not_generated("metric_id"), stored("value_normalized")],
    )
    .expect("first generation ordering is valid");
    let second = snapshot(
        vec![two_column_relation()],
        vec![stored("value_normalized"), not_generated("metric_id")],
    )
    .expect("second generation ordering is valid");

    assert_eq!(
        first.snapshot_digest(),
        second.snapshot_digest(),
        "input order must not create a second governed identity for the same attgenerated family"
    );
}

#[test]
fn generated_column_and_identity_evidence_are_mutually_exclusive() {
    let error = snapshot(
        vec![two_column_relation()],
        vec![not_generated("metric_id"), stored("value_normalized")],
    )
    .expect("generation family is internally valid")
    .with_observed_column_identities(vec![
        ColumnIdentityObservation::not_identity(
            "public",
            "metric",
            RelationKind::Table,
            "metric_id",
        )
        .expect("ordinary identity evidence is valid"),
        always_identity("value_normalized"),
    ])
    .expect_err("PostgreSQL generated columns cannot also have identity definitions");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_generation_identity",
        }
    );
}

#[test]
fn identity_first_cannot_bypass_generation_identity_validation() {
    let identity_first = PostgresSchemaSnapshotV3::new_with_column_identities(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-13T00:47:00Z",
        vec![two_column_relation()],
        Vec::new(),
        Vec::new(),
        vec![
            always_identity("metric_id"),
            ColumnIdentityObservation::not_identity(
                "public",
                "metric",
                RelationKind::Table,
                "value_normalized",
            )
            .expect("ordinary identity evidence is valid"),
        ],
    )
    .expect("identity family is internally valid");

    let error = identity_first
        .with_observed_column_generations(vec![
            not_generated("metric_id"),
            stored("value_normalized"),
        ])
        .expect_err("generation evidence cannot be attached after identity evidence");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "column_generation_observation_order",
        }
    );
}
