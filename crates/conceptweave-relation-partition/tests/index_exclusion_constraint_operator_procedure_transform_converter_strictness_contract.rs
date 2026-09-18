include!("index_exclusion_constraint_operator_procedure_transform_converter_leakproof_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot,
};

fn leakproof_snapshot() -> IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot {
    let predecessor = security_definer_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        complete_leakproof_observations(),
    )
    .unwrap()
}

fn converter_strictness_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    is_strict: bool,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        is_strict,
    )
    .unwrap()
}

fn complete_converter_strictness_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation> {
    vec![
        converter_strictness_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            true,
        ),
        converter_strictness_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            true,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_strictness_preserves_raw_proisstrict() {
    let predecessor = leakproof_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
            &predecessor,
            complete_converter_strictness_observations(),
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

    assert!(receipt.location().is_strict());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.source_id(), predecessor.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        predecessor.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), predecessor.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), predecessor.observed_at_utc());
}

#[test]
fn ordinary_exclude_transform_converter_strictness_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        true,
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        true,
    )
    .unwrap()
    .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_strictness_distinguishes_strict_from_non_strict() {
    let predecessor = leakproof_snapshot();
    let strict = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
        &predecessor,
        complete_converter_strictness_observations(),
    )
    .unwrap();
    let mut changed = complete_converter_strictness_observations();
    changed[0] = converter_strictness_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        false,
    );
    let non_strict =
        IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
            &predecessor,
            changed,
        )
        .unwrap();

    assert_ne!(strict.snapshot_digest(), non_strict.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_missing_direction() {
    let predecessor = leakproof_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
        &predecessor,
        vec![converter_strictness_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            true,
        )],
    )
    .expect_err("every nonzero converter direction must carry raw proisstrict evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_strictness_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_extra_coordinate() {
    let predecessor = leakproof_snapshot();
    let mut observations = complete_converter_strictness_observations();
    observations.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            true,
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("strictness evidence cannot introduce a converter coordinate absent from predecessor");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_strictness_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_binding_drift() {
    let predecessor = leakproof_snapshot();
    let mut observations = complete_converter_strictness_observations();
    observations[0] = converter_strictness_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        true,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("strictness evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_strictness_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_duplicate_coordinate() {
    let predecessor = leakproof_snapshot();
    let observation = converter_strictness_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        true,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter strictness evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_strictness_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_blank_converter_schema() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "  ",
        "payload_from_sql",
        true,
    )
    .expect_err("converter schema is part of the exact function binding");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_strictness_function_schema",
    );
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_blank_converter_function_name() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "\t",
        true,
    )
    .expect_err("converter function name is part of the exact function binding");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_strictness_function_name",
    );
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        true,
    )
    .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_strictness_rejects_unknown_receipt_coordinate() {
    let predecessor = leakproof_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
            &predecessor,
            complete_converter_strictness_observations(),
        )
        .unwrap();
    let error = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err("receipt lookup must remain exact-coordinate and exact-position bound");
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn ordinary_exclude_transform_converter_strictness_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot,
    >() > 0);
}
