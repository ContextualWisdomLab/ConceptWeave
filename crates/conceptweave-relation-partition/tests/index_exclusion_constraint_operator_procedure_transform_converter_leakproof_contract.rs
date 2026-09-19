include!("index_exclusion_constraint_operator_procedure_transform_converter_security_definer_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot,
};

fn security_definer_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot {
    let predecessor = configuration_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
        &predecessor,
        complete_security_definer_observations(),
    )
    .unwrap()
}

fn leakproof_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    leakproof: bool,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        leakproof,
    )
    .unwrap()
}

fn complete_leakproof_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation> {
    vec![
        leakproof_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            false,
        ),
        leakproof_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            false,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_preserves_raw_proleakproof() {
    let predecessor = security_definer_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        complete_leakproof_observations(),
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

    assert!(!receipt.location().leakproof());
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
fn ordinary_exclude_transform_converter_leakproof_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        false,
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        false,
    )
    .unwrap()
    .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_distinguishes_false_from_true() {
    let predecessor = security_definer_snapshot();
    let nonleakproof =
        IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
            &predecessor,
            complete_leakproof_observations(),
        )
        .unwrap();
    let mut changed = complete_leakproof_observations();
    changed[0] = leakproof_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        true,
    );
    let leakproof = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        changed,
    )
    .unwrap();

    assert_ne!(nonleakproof.snapshot_digest(), leakproof.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_rejects_missing_direction() {
    let predecessor = security_definer_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        vec![leakproof_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            false,
        )],
    )
    .expect_err("every nonzero converter direction must carry raw proleakproof evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_leakproof_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_rejects_binding_drift() {
    let predecessor = security_definer_snapshot();
    let mut observations = complete_leakproof_observations();
    observations[0] = leakproof_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        false,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("leakproof evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_leakproof_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_rejects_duplicate_coordinate() {
    let predecessor = security_definer_snapshot();
    let observation = leakproof_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        false,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter leakproof evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_leakproof_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        false,
    )
    .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_leakproof_rejects_unknown_receipt_coordinate() {
    let predecessor = security_definer_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot::new(
        &predecessor,
        complete_leakproof_observations(),
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
fn ordinary_exclude_transform_converter_leakproof_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot,
    >() > 0);
}
