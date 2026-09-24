include!("index_exclusion_constraint_operator_procedure_transform_converter_access_control_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial,
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot,
};

fn converter_access_control_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot {
    let predecessor = converter_owner_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
        &predecessor,
        complete_access_control_observations(),
    )
    .unwrap()
}

fn converter_configuration_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    proconfig: Option<Vec<&str>>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation {
    let material =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial::from_proconfig(
            proconfig.map(|entries| entries.into_iter().map(str::to_owned).collect()),
        );
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        material,
    )
    .unwrap()
}

fn complete_configuration_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation> {
    vec![
        converter_configuration_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            Some(vec!["search_path=pg_catalog, pg_temp"]),
        ),
        converter_configuration_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_configuration_preserves_exact_proconfig_identity() {
    let predecessor = converter_access_control_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            complete_configuration_observations(),
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

    assert!(receipt.location().configuration().is_configured());
    assert_eq!(receipt.location().configuration().entry_count(), 1);
    assert!(receipt.location().configuration().digest().starts_with("sha256:"));
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
fn ordinary_exclude_transform_converter_configuration_distinguishes_null_from_empty_array() {
    let predecessor = converter_access_control_snapshot();
    let mut absent = complete_configuration_observations();
    absent[0] = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let mut empty = complete_configuration_observations();
    empty[0] = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec![]),
    );
    let absent =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            absent,
        )
        .unwrap();
    let empty =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            empty,
        )
        .unwrap();

    assert_ne!(absent.snapshot_digest(), empty.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_configuration_distinguishes_setting_changes() {
    let predecessor = converter_access_control_snapshot();
    let first =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            complete_configuration_observations(),
        )
        .unwrap();
    let mut changed = complete_configuration_observations();
    changed[0] = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec!["search_path=public"]),
    );
    let second =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            changed,
        )
        .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_configuration_preserves_catalog_array_order() {
    let predecessor = converter_access_control_snapshot();
    let mut first_observations = complete_configuration_observations();
    first_observations[0] = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec!["search_path=pg_catalog, pg_temp", "work_mem=4MB"]),
    );
    let mut second_observations = complete_configuration_observations();
    second_observations[0] = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec!["work_mem=4MB", "search_path=pg_catalog, pg_temp"]),
    );
    let first =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            first_observations,
        )
        .unwrap();
    let second =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            second_observations,
        )
        .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_configuration_rejects_missing_direction() {
    let predecessor = converter_access_control_snapshot();
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            vec![converter_configuration_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                None,
            )],
        )
        .expect_err("every nonzero converter direction must carry exact proconfig evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_configuration_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_configuration_rejects_binding_drift() {
    let predecessor = converter_access_control_snapshot();
    let mut observations = complete_configuration_observations();
    observations[0] = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        None,
    );
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            observations,
        )
        .expect_err("configuration evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_configuration_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_configuration_rejects_duplicate_coordinate() {
    let predecessor = converter_access_control_snapshot();
    let observation = converter_configuration_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate configuration evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_configuration_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_configuration_rejects_zero_position() {
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation::new(
            coordinate(),
            0,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial::from_proconfig(None),
        )
        .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_configuration_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_access_control_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot::new(
            &predecessor,
            complete_configuration_observations(),
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
fn ordinary_exclude_transform_converter_configuration_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationSnapshot,
    >() > 0);
}
