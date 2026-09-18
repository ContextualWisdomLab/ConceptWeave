include!("index_exclusion_constraint_operator_procedure_transform_converter_transform_types_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot,
};

fn converter_transform_types_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot {
    let predecessor = converter_argument_names_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
        &predecessor,
        complete_converter_transform_types_observations(),
    )
    .unwrap()
}

fn converter_extension_membership_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    extension_name: Option<&str>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        extension_name.map(str::to_owned),
    )
    .unwrap()
}

fn complete_converter_extension_membership_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation> {
    vec![
        converter_extension_membership_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        ),
        converter_extension_membership_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_converter_extension_membership_preserves_absence_and_exact_member_extension() {
    let predecessor = converter_transform_types_snapshot();
    let absent = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        complete_converter_extension_membership_observations(),
    )
    .unwrap();
    assert_eq!(absent.observations()[0].extension_name(), None);

    let mut member_observations = complete_converter_extension_membership_observations();
    member_observations[0] = converter_extension_membership_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some("payload_codec"),
    );
    let member = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        member_observations,
    )
    .unwrap();
    assert_eq!(member.observations()[0].extension_name(), Some("payload_codec"));
    assert_ne!(absent.snapshot_digest(), member.snapshot_digest());

    let mut other_observations = complete_converter_extension_membership_observations();
    other_observations[0] = converter_extension_membership_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some("other_codec"),
    );
    let other = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        other_observations,
    )
    .unwrap();
    assert_ne!(member.snapshot_digest(), other.snapshot_digest());
}

#[test]
fn ordinary_exclude_converter_extension_membership_rejects_blank_member_name() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        Some(" \t".to_owned()),
    )
    .expect_err("an extension membership edge needs an exact nonblank pg_extension name");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_extension_name",
    );
}

#[test]
fn ordinary_exclude_converter_extension_membership_rejects_completeness_binding_and_duplicate_coordinates() {
    let predecessor = converter_transform_types_snapshot();
    let missing = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        vec![converter_extension_membership_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        )],
    )
    .expect_err("every converter direction needs explicit extension-membership evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_completeness",
    );

    let mut extra = complete_converter_extension_membership_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
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
    let extra_error = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        extra,
    )
    .expect_err("extension membership cannot introduce an absent converter coordinate");
    assert_field(
        extra_error,
        "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_completeness",
    );

    let observation = converter_extension_membership_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let duplicate = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate extension-membership coordinates must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_coordinate",
    );

    let mut drift = complete_converter_extension_membership_observations();
    drift[0] = converter_extension_membership_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        None,
    );
    let binding = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        drift,
    )
    .expect_err("extension membership must stay bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_binding",
    );
}

#[test]
fn ordinary_exclude_converter_extension_membership_preserves_location_receipt_and_input_edges() {
    let predecessor = converter_transform_types_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        complete_converter_extension_membership_observations(),
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
    assert_eq!(receipt.location().extension_name(), None);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let dotted_schema = QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type = QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
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
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
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

    for (schema, function, field) in [
        (" ", "payload_from_sql", "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_function_schema"),
        ("public", "\t", "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_function_name"),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            schema,
            function,
            None,
        )
        .expect_err("converter identifiers are exact binding inputs");
        assert_field(error, field);
    }

    let zero = IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        None,
    )
    .expect_err("converter positions are one-based");
    assert_eq!(zero, ObservationError::InvalidOrdinalPosition);

    let missing = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err("receipt lookup remains exact-coordinate bound");
    assert!(matches!(missing, ObservationError::UnknownObservationLocation { .. }));
}
