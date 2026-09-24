include!("index_exclusion_constraint_operator_procedure_transform_converter_initial_privileges_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation,
    IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot,
};

fn converter_function_initial_privilege_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot {
    let predecessor = converter_security_label_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        complete_initial_privilege_observations(),
    )
    .unwrap()
}

fn transform_extension_membership_observation(
    transform_type: QualifiedTypeName,
    target_language_name: &str,
    extension_name: Option<&str>,
) -> IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation {
    IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation::new(
        coordinate(),
        1,
        transform_type,
        target_language_name,
        extension_name.map(str::to_owned),
    )
    .unwrap()
}

fn complete_transform_extension_membership_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation> {
    vec![transform_extension_membership_observation(
        custom_payload_type(),
        "internal",
        None,
    )]
}

#[test]
fn ordinary_exclude_transform_extension_membership_descends_from_converter_initial_privileges() {
    let transforms = converter_snapshot();
    let security_labels = converter_security_label_snapshot();
    let absent = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &security_labels,
        complete_initial_privilege_observations(),
    )
    .unwrap();
    let standalone = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &absent,
        &transforms,
        complete_transform_extension_membership_observations(),
    )
    .unwrap();

    let mut initial_observations = complete_initial_privilege_observations();
    initial_observations[0] = initial_privilege_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(extension_initial_privileges(vec![initial_public_execute(
            "postgres",
            false,
        )])),
    );
    let with_initial_privileges =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
            &security_labels,
            initial_observations,
        )
        .unwrap();
    let recovery_distinct = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &with_initial_privileges,
        &transforms,
        complete_transform_extension_membership_observations(),
    )
    .unwrap();

    assert_ne!(standalone.snapshot_digest(), recovery_distinct.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_extension_membership_preserves_standalone_and_exact_member_extension() {
    let function_lifecycle = converter_function_initial_privilege_snapshot();
    let transforms = converter_snapshot();
    let standalone = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        complete_transform_extension_membership_observations(),
    )
    .unwrap();
    assert_eq!(standalone.observations()[0].extension_name(), None);

    let member = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        vec![transform_extension_membership_observation(
            custom_payload_type(),
            "internal",
            Some("payload_codec"),
        )],
    )
    .unwrap();
    assert_eq!(member.observations()[0].extension_name(), Some("payload_codec"));
    assert_ne!(standalone.snapshot_digest(), member.snapshot_digest());

    let other = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        vec![transform_extension_membership_observation(
            custom_payload_type(),
            "internal",
            Some("other_codec"),
        )],
    )
    .unwrap();
    assert_ne!(member.snapshot_digest(), other.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_extension_membership_is_one_fact_per_transform_row_not_converter_direction() {
    let function_lifecycle = converter_function_initial_privilege_snapshot();
    let transforms = converter_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        complete_transform_extension_membership_observations(),
    )
    .unwrap();

    assert_eq!(snapshot.observations().len(), 1);
    assert_eq!(snapshot.observations()[0].transform_type(), &custom_payload_type());
    assert_eq!(snapshot.observations()[0].target_language_name(), "internal");
}

#[test]
fn ordinary_exclude_transform_extension_membership_rejects_completeness_binding_and_duplicates() {
    let function_lifecycle = converter_function_initial_privilege_snapshot();
    let transforms = converter_snapshot();

    let missing = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        vec![],
    )
    .expect_err("every same-generation pg_transform row needs explicit extension-membership evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_completeness",
    );

    let observation = transform_extension_membership_observation(custom_payload_type(), "internal", None);
    let duplicate = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate transform-object membership coordinates must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_coordinate",
    );

    let drift = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        vec![transform_extension_membership_observation(
            custom_payload_type(),
            "plpython3u",
            None,
        )],
    )
    .expect_err("transform membership must remain bound to the exact pg_transform language");
    assert_field(
        drift,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_completeness",
    );

    let extra = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        vec![
            transform_extension_membership_observation(
                custom_payload_type(),
                "internal",
                None,
            ),
            transform_extension_membership_observation(
                QualifiedTypeName::new("public", "other_payload").unwrap(),
                "internal",
                None,
            ),
        ],
    )
    .expect_err("transform membership cannot introduce an absent pg_transform coordinate");
    assert_field(
        extra,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_extension_membership_rejects_invalid_identifiers_and_zero_position() {
    for (language, extension, field) in [
        (
            "",
            None,
            "index_exclusion_constraint_operator_procedure_transform_extension_membership_language",
        ),
        (
            "internal",
            Some(""),
            "index_exclusion_constraint_operator_procedure_transform_extension_membership_extension_name",
        ),
        (
            "internal\0",
            None,
            "index_exclusion_constraint_operator_procedure_transform_extension_membership_language",
        ),
        (
            "internal",
            Some("extension\0"),
            "index_exclusion_constraint_operator_procedure_transform_extension_membership_extension_name",
        ),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            language,
            extension.map(str::to_owned),
        )
        .expect_err("PostgreSQL catalog identifiers cannot be empty or contain NUL");
        assert_field(error, field);
    }

    let zero = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        "internal",
        None,
    )
    .expect_err("transform positions are one-based");
    assert_eq!(zero, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_extension_membership_preserves_collision_safe_location_and_exact_receipt() {
    let left = transform_extension_membership_observation(
        QualifiedTypeName::new("payload.domain", "json").unwrap(),
        "internal.lang",
        None,
    )
    .canonical_location();
    let right = transform_extension_membership_observation(
        QualifiedTypeName::new("payload", "domain.json").unwrap(),
        "internal.lang",
        None,
    )
    .canonical_location();
    assert_ne!(left, right);

    let function_lifecycle = converter_function_initial_privilege_snapshot();
    let transforms = converter_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &transforms,
        complete_transform_extension_membership_observations(),
    )
    .unwrap();
    let receipt = snapshot
        .source_receipt(coordinate(), 1, custom_payload_type(), "internal")
        .unwrap();
    assert_eq!(receipt.location().extension_name(), None);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let missing = snapshot
        .source_receipt(coordinate(), 2, custom_payload_type(), "internal")
        .expect_err("receipt lookup remains exact transform-object coordinate bound");
    assert!(matches!(missing, ObservationError::UnknownObservationLocation { .. }));
}
