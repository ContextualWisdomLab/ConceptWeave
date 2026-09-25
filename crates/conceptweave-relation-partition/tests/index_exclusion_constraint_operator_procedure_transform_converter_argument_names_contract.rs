include!(
    "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_contract.rs"
);

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot,
};

fn converter_argument_modes_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot {
    let predecessor = converter_argument_count_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
        &predecessor,
        complete_converter_argument_modes_observations(),
    )
    .unwrap()
}

fn converter_argument_names_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    argument_names: Option<Vec<String>>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        argument_names,
    )
    .unwrap()
}

fn complete_converter_argument_names_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation> {
    vec![
        converter_argument_names_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        ),
        converter_argument_names_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_argument_names_preserve_null_named_and_empty_position_distinctly()
 {
    use IndexExclusionConstraintOperatorProcedureTransformConverterArgumentMode::{In, Out};

    let predecessor = converter_argument_modes_snapshot();
    let unnamed =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            complete_converter_argument_names_observations(),
        )
        .unwrap();
    assert_eq!(unnamed.observations()[0].argument_names(), None);

    let mut named_observations = complete_converter_argument_names_observations();
    named_observations[0] = converter_argument_names_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec!["value".to_owned()]),
    );
    let named =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            named_observations,
        )
        .unwrap();
    assert_eq!(
        named.observations()[0].argument_names(),
        Some(&["value".to_owned()][..])
    );
    assert_ne!(unnamed.snapshot_digest(), named.snapshot_digest());

    let argument_count = converter_argument_count_snapshot();
    let mut output_modes = complete_converter_argument_modes_observations();
    output_modes[1] = converter_argument_modes_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
        "payload_to_sql",
        Some(vec![In, Out]),
    );
    let output_mode_snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &argument_count,
            output_modes,
        )
        .unwrap();
    let mut mixed_names = complete_converter_argument_names_observations();
    mixed_names[1] = converter_argument_names_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
        "payload_to_sql",
        Some(vec![String::new(), "result".to_owned()]),
    );
    let mixed =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &output_mode_snapshot,
            mixed_names,
        )
        .unwrap();
    assert_eq!(
        mixed.observations()[1].argument_names(),
        Some(&[String::new(), "result".to_owned()][..])
    );
    assert_ne!(unnamed.snapshot_digest(), mixed.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_argument_names_reject_impossible_raw_shapes() {
    for names in [Some(vec![]), Some(vec![String::new()])] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            names,
        )
        .expect_err("PostgreSQL stores NULL rather than an empty or wholly unnamed proargnames array");
        assert_field(
            error,
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_names",
        );
    }
}

#[test]
fn ordinary_exclude_transform_converter_argument_names_reject_vector_length_drift() {
    let predecessor = converter_argument_modes_snapshot();
    let mut too_many = complete_converter_argument_names_observations();
    too_many[0] = converter_argument_names_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec!["value".to_owned(), "extra".to_owned()]),
    );
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            too_many,
        )
        .expect_err("proargnames positions must match the same-row all-argument shape");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_shape",
    );
}

#[test]
fn ordinary_exclude_transform_converter_argument_names_reject_completeness_binding_and_duplicate_failures()
 {
    let predecessor = converter_argument_modes_snapshot();
    let missing =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            vec![converter_argument_names_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                None,
            )],
        )
        .expect_err(
            "every argument-mode predecessor direction needs explicit proargnames evidence",
        );
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_completeness",
    );

    let mut extra = complete_converter_argument_names_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
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
    let extra_error =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            extra,
        )
        .expect_err("argument-name evidence cannot introduce an absent converter coordinate");
    assert_field(
        extra_error,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_completeness",
    );

    let observation = converter_argument_names_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate argument-name evidence must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_coordinate",
    );

    let mut drift = complete_converter_argument_names_observations();
    drift[0] = converter_argument_names_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        None,
    );
    let binding =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            drift,
        )
        .expect_err("argument-name evidence must remain bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_argument_names_preserve_location_receipt_and_input_edges() {
    let predecessor = converter_argument_modes_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
            &predecessor,
            complete_converter_argument_names_observations(),
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
    assert_eq!(receipt.location().argument_names(), None);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
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
    let type_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
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
        (
            " ",
            "payload_from_sql",
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_function_schema",
        ),
        (
            "public",
            "\t",
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_function_name",
        ),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
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

    let zero =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation::new(
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
    assert!(matches!(
        missing,
        ObservationError::UnknownObservationLocation { .. }
    ));
}
