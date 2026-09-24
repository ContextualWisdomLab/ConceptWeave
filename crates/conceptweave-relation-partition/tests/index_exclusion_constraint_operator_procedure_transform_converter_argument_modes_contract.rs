include!(
    "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_contract.rs"
);

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentMode,
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot,
};

fn converter_argument_count_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot {
    let predecessor = converter_return_set_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        complete_converter_argument_count_observations(),
    )
    .unwrap()
}

fn converter_argument_modes_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    argument_modes: Option<
        Vec<IndexExclusionConstraintOperatorProcedureTransformConverterArgumentMode>,
    >,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        argument_modes,
    )
    .unwrap()
}

fn complete_converter_argument_modes_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation> {
    vec![
        converter_argument_modes_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        ),
        converter_argument_modes_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_argument_modes_preserve_null_and_mode_vector_distinctly() {
    use IndexExclusionConstraintOperatorProcedureTransformConverterArgumentMode::{In, InOut, Out};

    let predecessor = converter_argument_count_snapshot();
    let plain =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            complete_converter_argument_modes_observations(),
        )
        .unwrap();
    assert_eq!(plain.observations()[0].argument_modes(), None);

    let mut inout_observations = complete_converter_argument_modes_observations();
    inout_observations[0] = converter_argument_modes_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec![InOut]),
    );
    let inout =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            inout_observations,
        )
        .unwrap();
    assert_eq!(inout.observations()[0].argument_modes(), Some(&[InOut][..]));
    assert_ne!(plain.snapshot_digest(), inout.snapshot_digest());

    let mut out_observations = complete_converter_argument_modes_observations();
    out_observations[1] = converter_argument_modes_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
        "payload_to_sql",
        Some(vec![In, Out]),
    );
    let with_out =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            out_observations,
        )
        .unwrap();
    assert_eq!(
        with_out.observations()[1].argument_modes(),
        Some(&[In, Out][..])
    );
    assert_ne!(plain.snapshot_digest(), with_out.snapshot_digest());
    assert_ne!(inout.snapshot_digest(), with_out.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_argument_modes_reject_impossible_raw_shapes() {
    use IndexExclusionConstraintOperatorProcedureTransformConverterArgumentMode::{
        In, Out, Table, Variadic,
    };

    for modes in [
        Some(vec![]),
        Some(vec![In]),
        Some(vec![Out]),
        Some(vec![In, In]),
        Some(vec![Variadic]),
        Some(vec![In, Table]),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            modes,
        )
        .expect_err("argument modes must remain coherent with the already-bound one internal input and non-set result");
        assert_field(
            error,
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes",
        );
    }
}

#[test]
fn ordinary_exclude_transform_converter_argument_modes_reject_to_sql_inout_shape() {
    use IndexExclusionConstraintOperatorProcedureTransformConverterArgumentMode::InOut;

    let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
        "public",
        "payload_to_sql",
        Some(vec![InOut]),
    )
    .expect_err("TO SQL must return the transform type, so an internal INOUT result cannot satisfy the predecessor binding");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes",
    );
}

#[test]
fn ordinary_exclude_transform_converter_argument_modes_reject_completeness_binding_and_duplicate_failures()
 {
    let predecessor = converter_argument_count_snapshot();
    let missing =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            vec![converter_argument_modes_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                None,
            )],
        )
        .expect_err(
            "every argument-count predecessor direction needs explicit proargmodes evidence",
        );
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_completeness",
    );

    let mut extra = complete_converter_argument_modes_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
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
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            extra,
        )
        .expect_err("argument-mode evidence cannot introduce an absent converter coordinate");
    assert_field(
        extra_error,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_completeness",
    );

    let observation = converter_argument_modes_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate argument-mode evidence must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_coordinate",
    );

    let mut drift = complete_converter_argument_modes_observations();
    drift[0] = converter_argument_modes_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        None,
    );
    let binding =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            drift,
        )
        .expect_err("argument-mode evidence must remain bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_argument_modes_preserve_location_receipt_and_input_edges() {
    let predecessor = converter_argument_count_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesSnapshot::new(
            &predecessor,
            complete_converter_argument_modes_observations(),
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
    assert_eq!(receipt.location().argument_modes(), None);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
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
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
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
            "",
            "payload_from_sql",
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_function_schema",
        ),
        (
            "public",
            "",
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_modes_function_name",
        ),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
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
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation::new(
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
