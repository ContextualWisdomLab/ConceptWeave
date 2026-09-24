include!("index_exclusion_constraint_operator_procedure_transform_converter_kind_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot,
};

fn converter_kind_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot {
    let predecessor = converter_cost_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        complete_converter_kind_observations(),
    )
    .unwrap()
}

fn converter_return_set_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    returns_set: bool,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        returns_set,
    )
    .unwrap()
}

fn complete_converter_return_set_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation> {
    vec![
        converter_return_set_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            false,
        ),
        converter_return_set_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            false,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_return_set_preserves_required_false_proretset() {
    let predecessor = converter_kind_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
            &predecessor,
            complete_converter_return_set_observations(),
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

    assert!(!receipt.location().returns_set());
    assert_ne!(snapshot.snapshot_digest(), predecessor.snapshot_digest());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_return_set_rejects_true_proretset() {
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            true,
        )
        .expect_err("PostgreSQL transform converters must not return a set");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_return_set",
    );
}

#[test]
fn ordinary_exclude_transform_converter_return_set_rejects_missing_extra_duplicate_or_binding_drift()
 {
    let predecessor = converter_kind_snapshot();
    let missing =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
            &predecessor,
            vec![converter_return_set_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                false,
            )],
        )
        .expect_err("every kind predecessor direction needs explicit proretset evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_return_set_completeness",
    );

    let mut extra = complete_converter_return_set_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            false,
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
        &predecessor,
        extra,
    )
    .expect_err(
        "return-set evidence cannot introduce a converter coordinate absent from predecessor",
    );
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_return_set_completeness",
    );

    let observation = converter_return_set_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        false,
    );
    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate return-set evidence must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_return_set_coordinate",
    );

    let mut drift = complete_converter_return_set_observations();
    drift[0] = converter_return_set_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        false,
    );
    let binding =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
            &predecessor,
            drift,
        )
        .expect_err("return-set evidence must remain bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_return_set_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_return_set_keeps_collision_safe_location_and_input_edges() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
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
    let type_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
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

    for (schema, function, field) in [
        (
            "",
            "payload_from_sql",
            "index_exclusion_constraint_operator_procedure_transform_converter_return_set_function_schema",
        ),
        (
            "public",
            "",
            "index_exclusion_constraint_operator_procedure_transform_converter_return_set_function_name",
        ),
    ] {
        let error =
            IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
                coordinate(),
                1,
                custom_payload_type(),
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                schema,
                function,
                false,
            )
            .expect_err("converter identifiers are exact binding inputs");
        assert_field(error, field);
    }

    let zero =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetObservation::new(
            coordinate(),
            0,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            false,
        )
        .expect_err("converter positions are one-based");
    assert_eq!(zero, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_return_set_receipt_is_exact_and_publicly_composed() {
    let predecessor = converter_kind_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot::new(
            &predecessor,
            complete_converter_return_set_observations(),
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
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
    assert!(
        std::mem::size_of::<
            IndexExclusionConstraintOperatorProcedureTransformConverterReturnSetSnapshot,
        >() > 0
    );
}
