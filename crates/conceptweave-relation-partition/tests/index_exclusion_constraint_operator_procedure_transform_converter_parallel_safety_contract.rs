include!("index_exclusion_constraint_operator_procedure_transform_converter_volatility_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot,
};

fn converter_volatility_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot {
    let predecessor = converter_strictness_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        complete_converter_volatility_observations(),
    )
    .unwrap()
}

fn converter_parallel_safety_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    parallel_safety: char,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        parallel_safety,
    )
    .unwrap()
}

fn complete_converter_parallel_safety_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation> {
    vec![
        converter_parallel_safety_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            's',
        ),
        converter_parallel_safety_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            'r',
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_preserves_raw_proparallel() {
    let predecessor = converter_volatility_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        complete_converter_parallel_safety_observations(),
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

    assert_eq!(receipt.location().parallel_safety(), 's');
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
fn ordinary_exclude_transform_converter_parallel_safety_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        's',
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        's',
    )
    .unwrap()
    .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_distinguishes_catalog_states() {
    let predecessor = converter_volatility_snapshot();
    let safe = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        complete_converter_parallel_safety_observations(),
    )
    .unwrap();
    let mut restricted_observations = complete_converter_parallel_safety_observations();
    restricted_observations[0] = converter_parallel_safety_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        'r',
    );
    let restricted = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        restricted_observations,
    )
    .unwrap();
    let mut unsafe_observations = complete_converter_parallel_safety_observations();
    unsafe_observations[0] = converter_parallel_safety_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        'u',
    );
    let unsafe_snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        unsafe_observations,
    )
    .unwrap();

    assert_ne!(safe.snapshot_digest(), restricted.snapshot_digest());
    assert_ne!(safe.snapshot_digest(), unsafe_snapshot.snapshot_digest());
    assert_ne!(restricted.snapshot_digest(), unsafe_snapshot.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_unknown_catalog_state() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'x',
    )
    .expect_err("PostgreSQL proparallel must be one of s, r, or u");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety",
    );
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_missing_direction() {
    let predecessor = converter_volatility_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        vec![converter_parallel_safety_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            's',
        )],
    )
    .expect_err("every volatility predecessor converter direction must carry proparallel evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_extra_coordinate() {
    let predecessor = converter_volatility_snapshot();
    let mut observations = complete_converter_parallel_safety_observations();
    observations.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            's',
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("parallel-safety evidence cannot introduce a converter coordinate absent from predecessor");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_binding_drift() {
    let predecessor = converter_volatility_snapshot();
    let mut observations = complete_converter_parallel_safety_observations();
    observations[0] = converter_parallel_safety_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        's',
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("parallel-safety evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_duplicate_coordinate() {
    let predecessor = converter_volatility_snapshot();
    let observation = converter_parallel_safety_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        's',
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter parallel-safety evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_blank_converter_identifiers() {
    let blank_schema = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        " ",
        "payload_from_sql",
        's',
    )
    .expect_err("converter schema is part of the exact function binding");
    assert_field(
        blank_schema,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_function_schema",
    );

    let blank_name = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "\t",
        's',
    )
    .expect_err("converter function name is part of the exact function binding");
    assert_field(
        blank_name,
        "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_function_name",
    );
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        's',
    )
    .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_parallel_safety_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_volatility_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot::new(
        &predecessor,
        complete_converter_parallel_safety_observations(),
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
fn ordinary_exclude_transform_converter_parallel_safety_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot,
    >() > 0);
}
