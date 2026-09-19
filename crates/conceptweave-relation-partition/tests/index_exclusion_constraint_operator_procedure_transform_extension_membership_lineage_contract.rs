include!("index_exclusion_constraint_operator_procedure_transform_extension_membership_contract.rs");

#[test]
fn ordinary_exclude_transform_extension_membership_rejects_same_generation_converter_direction_drift() {
    let function_lifecycle = converter_function_auto_extension_dependency_snapshot();
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let from_sql_only = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            vec![converter_binding(
                Some(converter_function(
                    "payload_from_sql",
                    internal_type(),
                    "payload_from_sql_v1",
                )),
                None,
            )],
        )],
    )
    .unwrap();

    let error = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &from_sql_only,
        complete_transform_extension_membership_observations(),
    )
    .expect_err(
        "transform-object extension membership must not combine a converter-lifecycle predecessor with a different same-generation pg_transform direction set",
    );

    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_binding",
    );
}

#[test]
fn ordinary_exclude_transform_extension_membership_rejects_same_generation_converter_function_drift() {
    let function_lifecycle = converter_function_auto_extension_dependency_snapshot();
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let drifted_converter = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            vec![converter_binding(
                Some(converter_function(
                    "different_from_sql",
                    internal_type(),
                    "payload_from_sql_v1",
                )),
                Some(converter_function(
                    "payload_to_sql",
                    custom_payload_type(),
                    "payload_to_sql_v1",
                )),
            )],
        )],
    )
    .unwrap();

    let error = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &drifted_converter,
        complete_transform_extension_membership_observations(),
    )
    .expect_err(
        "transform-object extension membership must remain bound to the exact same-generation converter functions",
    );

    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_binding",
    );
}

#[test]
fn ordinary_exclude_transform_extension_membership_rejects_same_name_converter_definition_drift() {
    let function_lifecycle = converter_function_auto_extension_dependency_snapshot();
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let drifted_converter = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            vec![converter_binding(
                Some(converter_function(
                    "payload_from_sql",
                    internal_type(),
                    "payload_from_sql_v2",
                )),
                Some(converter_function(
                    "payload_to_sql",
                    custom_payload_type(),
                    "payload_to_sql_v1",
                )),
            )],
        )],
    )
    .unwrap();

    let error = IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new(
        &function_lifecycle,
        &drifted_converter,
        complete_transform_extension_membership_observations(),
    )
    .expect_err(
        "transform-object extension membership must reject a same-name converter snapshot with different immutable function-definition evidence",
    );

    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_extension_membership_lineage",
    );
}
