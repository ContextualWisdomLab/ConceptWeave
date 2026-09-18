include!("index_exclusion_constraint_operator_procedure_transform_converter_return_set_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot,
};

fn converter_return_set_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot {
    let predecessor = converter_kind_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
        &predecessor,
        complete_converter_return_set_observations(),
    )
    .unwrap()
}

fn converter_argument_count_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    argument_count: i16,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        argument_count,
    )
    .unwrap()
}

fn complete_converter_argument_count_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation> {
    vec![
        converter_argument_count_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            1,
        ),
        converter_argument_count_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            1,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_argument_count_preserves_required_pronargs_one() {
    let predecessor = converter_return_set_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        complete_converter_argument_count_observations(),
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

    assert_eq!(receipt.location().argument_count(), 1);
    assert_ne!(snapshot.snapshot_digest(), predecessor.snapshot_digest());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_argument_count_rejects_non_one_pronargs() {
    for invalid_count in [i16::MIN, -1, 0, 2, i16::MAX] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            invalid_count,
        )
        .expect_err("PostgreSQL transform converters must have pronargs=1");
        assert_field(
            error,
            "index_exclusion_constraint_operator_procedure_transform_converter_argument_count",
        );
    }
}

#[test]
fn ordinary_exclude_transform_converter_argument_count_rejects_completeness_binding_and_duplicate_failures() {
    let predecessor = converter_return_set_snapshot();
    let missing = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        vec![converter_argument_count_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            1,
        )],
    )
    .expect_err("every return-set predecessor direction needs explicit pronargs evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_completeness",
    );

    let mut extra = complete_converter_argument_count_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            1,
        )
        .unwrap(),
    );
    let extra_error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        extra,
    )
    .expect_err("argument-count evidence cannot introduce an absent converter coordinate");
    assert_field(
        extra_error,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_completeness",
    );

    let observation = converter_argument_count_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        1,
    );
    let duplicate = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate argument-count evidence must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_coordinate",
    );

    let mut drift = complete_converter_argument_count_observations();
    drift[0] = converter_argument_count_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        1,
    );
    let binding = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        drift,
    )
    .expect_err("argument-count evidence must remain bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_argument_count_preserves_location_and_input_edges() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        1,
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        1,
    )
    .unwrap()
    .canonical_location();
    assert_ne!(schema_location, type_location);

    for (schema, function, field) in [
        (" ", "payload_from_sql", "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_function_schema"),
        ("public", "\t", "index_exclusion_constraint_operator_procedure_transform_converter_argument_count_function_name"),
    ] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            schema,
            function,
            1,
        )
        .expect_err("converter identifiers are exact binding inputs");
        assert_field(error, field);
    }

    let zero = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        1,
    )
    .expect_err("converter positions are one-based");
    assert_eq!(zero, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_argument_count_receipt_is_exact_and_publicly_composed() {
    let predecessor = converter_return_set_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot::new(
        &predecessor,
        complete_converter_argument_count_observations(),
    )
    .unwrap();
    let error = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err("receipt lookup remains exact-coordinate bound");
    assert!(matches!(error, ObservationError::UnknownObservationLocation { .. }));
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterArgumentCountSnapshot,
    >() > 0);
}
