include!("index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot,
};

fn converter_parallel_safety_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot {
    let predecessor = converter_volatility_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        complete_converter_parallel_safety_observations(),
    )
    .unwrap()
}

fn converter_planner_support(name: &str) -> QualifiedProcedureSignature {
    QualifiedProcedureSignature::new(
        "pg_catalog",
        name,
        vec![QualifiedTypeName::new("pg_catalog", "internal").unwrap()],
    )
    .unwrap()
}

fn converter_planner_support_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    planner_support: Option<QualifiedProcedureSignature>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        planner_support,
    )
    .unwrap()
}

fn complete_converter_planner_support_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation> {
    vec![
        converter_planner_support_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            Some(converter_planner_support("payload_from_sql_support")),
        ),
        converter_planner_support_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_preserves_exact_prosupport_state() {
    let predecessor = converter_parallel_safety_snapshot();
    let support = converter_planner_support("payload_from_sql_support");
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        complete_converter_planner_support_observations(),
    )
    .unwrap();
    let receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .unwrap();

    assert_eq!(receipt.location().planner_support(), Some(&support));
    assert_eq!(receipt.source_id(), predecessor.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(
        receipt.connection_policy_binding(),
        predecessor.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), predecessor.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), predecessor.observed_at_utc());
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_distinguishes_absence_and_identity() {
    let predecessor = converter_parallel_safety_snapshot();
    let mut absent_observations = complete_converter_planner_support_observations();
    absent_observations[0] = converter_planner_support_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let absent = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        absent_observations,
    )
    .unwrap();
    let first = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        complete_converter_planner_support_observations(),
    )
    .unwrap();
    let mut second_observations = complete_converter_planner_support_observations();
    second_observations[0] = converter_planner_support_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(converter_planner_support("payload_from_sql_support_v2")),
    );
    let second = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        second_observations,
    )
    .unwrap();

    assert_ne!(absent.snapshot_digest(), first.snapshot_digest());
    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
    assert_ne!(absent.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        None,
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        None,
    )
    .unwrap()
    .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_missing_direction() {
    let predecessor = converter_parallel_safety_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        vec![converter_planner_support_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            Some(converter_planner_support("payload_from_sql_support")),
        )],
    )
    .expect_err("every parallel-safety predecessor converter direction needs explicit prosupport evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_extra_coordinate() {
    let predecessor = converter_parallel_safety_snapshot();
    let mut observations = complete_converter_planner_support_observations();
    observations.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            None,
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("planner-support evidence cannot introduce a converter coordinate absent from predecessor");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_binding_drift() {
    let predecessor = converter_parallel_safety_snapshot();
    let mut observations = complete_converter_planner_support_observations();
    observations[0] = converter_planner_support_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        Some(converter_planner_support("payload_from_sql_support")),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("planner-support evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_duplicate_coordinate() {
    let predecessor = converter_parallel_safety_snapshot();
    let observation = converter_planner_support_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter planner-support evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_blank_converter_identifiers() {
    let blank_schema = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        " ",
        "payload_from_sql",
        None,
    )
    .expect_err("converter schema is part of the exact function binding");
    assert_field(
        blank_schema,
        "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_function_schema",
    );

    let blank_name = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "\t",
        None,
    )
    .expect_err("converter function name is part of the exact function binding");
    assert_field(
        blank_name,
        "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_function_name",
    );
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        None,
    )
    .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_parallel_safety_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        complete_converter_planner_support_observations(),
    )
    .unwrap();
    let error = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err("receipt lookup must remain exact-coordinate, position, type, and direction bound");
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn ordinary_exclude_transform_converter_planner_support_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot,
    >() > 0);
}
