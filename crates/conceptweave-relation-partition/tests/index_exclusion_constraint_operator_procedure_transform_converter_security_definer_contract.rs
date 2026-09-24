include!("index_exclusion_constraint_operator_procedure_transform_converter_configuration_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot,
};

fn converter_configuration_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot {
    let predecessor = converter_access_control_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
        &predecessor,
        complete_configuration_observations(),
    )
    .unwrap()
}

fn converter_security_definer_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    security_definer: bool,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        security_definer,
    )
    .unwrap()
}

fn complete_security_definer_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation> {
    vec![
        converter_security_definer_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            false,
        ),
        converter_security_definer_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            false,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_security_context_preserves_raw_prosecdef() {
    let predecessor = converter_configuration_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            complete_security_definer_observations(),
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

    assert!(!receipt.location().security_definer());
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
fn ordinary_exclude_transform_converter_security_context_distinguishes_invoker_from_definer() {
    let predecessor = converter_configuration_snapshot();
    let invoker =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            complete_security_definer_observations(),
        )
        .unwrap();
    let mut changed = complete_security_definer_observations();
    changed[0] = converter_security_definer_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        true,
    );
    let definer =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            changed,
        )
        .unwrap();

    assert_ne!(invoker.snapshot_digest(), definer.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_security_context_rejects_missing_direction() {
    let predecessor = converter_configuration_snapshot();
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            vec![converter_security_definer_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                false,
            )],
        )
        .expect_err("every nonzero converter direction must carry raw prosecdef evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_definer_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_security_context_rejects_binding_drift() {
    let predecessor = converter_configuration_snapshot();
    let mut observations = complete_security_definer_observations();
    observations[0] = converter_security_definer_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        false,
    );
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            observations,
        )
        .expect_err("security-context evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_definer_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_security_context_rejects_duplicate_coordinate() {
    let predecessor = converter_configuration_snapshot();
    let observation = converter_security_definer_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        false,
    );
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate converter security-context evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_definer_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_security_context_rejects_zero_position() {
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation::new(
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
fn ordinary_exclude_transform_converter_security_context_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_configuration_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot::new(
            &predecessor,
            complete_security_definer_observations(),
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
fn ordinary_exclude_transform_converter_security_context_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot,
    >() > 0);
}
