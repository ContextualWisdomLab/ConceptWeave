include!(
    "index_exclusion_constraint_operator_procedure_transform_converter_argument_names_contract.rs"
);

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot,
};

fn converter_argument_names_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot {
    let predecessor = converter_argument_modes_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot::new(
        &predecessor,
        complete_converter_argument_names_observations(),
    )
    .unwrap()
}

fn converter_transform_types_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    transform_types: Option<Vec<QualifiedTypeName>>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        transform_types,
    )
    .unwrap()
}

fn complete_converter_transform_types_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation> {
    vec![
        converter_transform_types_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        ),
        converter_transform_types_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_converter_transform_types_preserve_null_and_selected_set_distinctly() {
    let predecessor = converter_argument_names_snapshot();
    let absent =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            complete_converter_transform_types_observations(),
        )
        .unwrap();
    assert_eq!(absent.observations()[0].transform_types(), None);

    let mut selected_observations = complete_converter_transform_types_observations();
    selected_observations[0] = converter_transform_types_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(vec![
            QualifiedTypeName::new("ext", "zeta").unwrap(),
            QualifiedTypeName::new("ext", "alpha").unwrap(),
        ]),
    );
    let selected =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            selected_observations,
        )
        .unwrap();
    assert_eq!(
        selected.observations()[0].transform_types(),
        Some(
            &[
                QualifiedTypeName::new("ext", "alpha").unwrap(),
                QualifiedTypeName::new("ext", "zeta").unwrap(),
            ][..]
        )
    );
    assert_ne!(absent.snapshot_digest(), selected.snapshot_digest());
}

#[test]
fn ordinary_exclude_converter_transform_types_reject_empty_and_duplicate_sets() {
    let empty =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            Some(vec![]),
        )
        .expect_err("PostgreSQL stores NULL rather than an empty protrftypes array");
    assert_field(
        empty,
        "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_empty",
    );

    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            Some(vec![custom_payload_type(), custom_payload_type()]),
        )
        .expect_err("duplicate selected transform types must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_duplicate",
    );
}

#[test]
fn ordinary_exclude_converter_transform_types_reject_completeness_binding_and_duplicate_coordinates()
 {
    let predecessor = converter_argument_names_snapshot();
    let missing =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            vec![converter_transform_types_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                None,
            )],
        )
        .expect_err("every converter direction needs explicit protrftypes evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_completeness",
    );

    let mut extra = complete_converter_transform_types_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
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
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            extra,
        )
        .expect_err("converter protrftypes cannot introduce an absent coordinate");
    assert_field(
        extra_error,
        "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_completeness",
    );

    let observation = converter_transform_types_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate converter protrftypes coordinates must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_coordinate",
    );

    let mut drift = complete_converter_transform_types_observations();
    drift[0] = converter_transform_types_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        None,
    );
    let binding =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            drift,
        )
        .expect_err("converter protrftypes must remain bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_binding",
    );
}

#[test]
fn ordinary_exclude_converter_transform_types_preserve_location_receipt_and_input_edges() {
    let predecessor = converter_argument_names_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot::new(
            &predecessor,
            complete_converter_transform_types_observations(),
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
    assert_eq!(receipt.location().transform_types(), None);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let dotted_schema = QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type = QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
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
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
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
            "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_function_schema",
        ),
        (
            "public",
            "\t",
            "index_exclusion_constraint_operator_procedure_transform_converter_transform_types_function_name",
        ),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
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
        IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation::new(
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
