include!("index_exclusion_constraint_operator_procedure_transform_converter_strictness_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot,
};

fn converter_strictness_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot {
    let predecessor = converter_leakproof_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot::new(
        &predecessor,
        complete_converter_strictness_observations(),
    )
    .unwrap()
}

fn converter_volatility_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    volatility: char,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        volatility,
    )
    .unwrap()
}

fn complete_converter_volatility_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation> {
    vec![
        converter_volatility_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            'i',
        ),
        converter_volatility_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            's',
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_volatility_preserves_raw_provolatile() {
    let predecessor = converter_strictness_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        complete_converter_volatility_observations(),
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

    assert_eq!(receipt.location().volatility(), 'i');
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
fn ordinary_exclude_transform_converter_volatility_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'i',
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'i',
    )
    .unwrap()
    .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_volatility_distinguishes_catalog_states() {
    let predecessor = converter_strictness_snapshot();
    let immutable = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        complete_converter_volatility_observations(),
    )
    .unwrap();
    let mut changed = complete_converter_volatility_observations();
    changed[0] = converter_volatility_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        'v',
    );
    let volatile = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        changed,
    )
    .unwrap();

    assert_ne!(immutable.snapshot_digest(), volatile.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_volatility_preserves_post_creation_volatile_drift() {
    let observation = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'v',
    )
    .expect("Source Observation must preserve live VOLATILE drift even though fresh CREATE TRANSFORM would reject it");

    assert_eq!(observation.volatility(), 'v');
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_unknown_catalog_state() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'x',
    )
    .expect_err("PostgreSQL provolatile must be one of i, s, or v");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility",
    );
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_missing_direction() {
    let predecessor = converter_strictness_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        vec![converter_volatility_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            'i',
        )],
    )
    .expect_err("every strictness predecessor converter direction must carry provolatile evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_extra_coordinate() {
    let predecessor = converter_strictness_snapshot();
    let mut observations = complete_converter_volatility_observations();
    observations.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            'i',
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("volatility evidence cannot introduce a converter coordinate absent from predecessor");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_binding_drift() {
    let predecessor = converter_strictness_snapshot();
    let mut observations = complete_converter_volatility_observations();
    observations[0] = converter_volatility_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        'i',
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("volatility evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_duplicate_coordinate() {
    let predecessor = converter_strictness_snapshot();
    let observation = converter_volatility_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        'i',
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter volatility evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_blank_converter_identifiers() {
    let blank_schema = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        " ",
        "payload_from_sql",
        'i',
    )
    .expect_err("converter schema is part of the exact function binding");
    assert_field(
        blank_schema,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_function_schema",
    );

    let blank_name = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "\t",
        'i',
    )
    .expect_err("converter function name is part of the exact function binding");
    assert_field(
        blank_name,
        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_function_name",
    );
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'i',
    )
    .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_volatility_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_strictness_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot::new(
        &predecessor,
        complete_converter_volatility_observations(),
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
fn ordinary_exclude_transform_converter_volatility_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot,
    >() > 0);
}
