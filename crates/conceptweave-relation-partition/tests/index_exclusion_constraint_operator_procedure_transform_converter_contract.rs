include!("index_exclusion_constraint_operator_procedure_transform_types_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterBinding,
    IndexExclusionConstraintOperatorProcedureTransformConverterFunction,
    IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition,
    IndexExclusionConstraintOperatorProcedureTransformConverterObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot,
};

fn selected_transform_types_predecessor(
) -> IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot {
    let predecessor = cost_predecessor();
    IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec![QualifiedTypeName::new("public", "custom_payload").unwrap()]),
        )],
    )
    .unwrap()
}

fn null_transform_types_predecessor() -> IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot {
    let predecessor = cost_predecessor();
    IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            None,
        )],
    )
    .unwrap()
}

fn internal_type() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "internal").unwrap()
}

fn custom_payload_type() -> QualifiedTypeName {
    QualifiedTypeName::new("public", "custom_payload").unwrap()
}

fn converter_definition(
    prosrc: &str,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition {
    IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition::new(
        "c",
        prosrc,
        Some("$libdir/custom_transform".to_owned()),
        None,
    )
    .unwrap()
}

fn converter_function(
    function_name: &str,
    return_type: QualifiedTypeName,
    prosrc: &str,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterFunction {
    IndexExclusionConstraintOperatorProcedureTransformConverterFunction::new(
        "public",
        function_name,
        internal_type(),
        return_type,
        converter_definition(prosrc),
    )
    .unwrap()
}

fn converter_binding(
    from_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
    to_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterBinding {
    IndexExclusionConstraintOperatorProcedureTransformConverterBinding::new(
        custom_payload_type(),
        from_sql,
        to_sql,
    )
    .unwrap()
}

fn converter_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    target_language_name: &str,
    converters: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterBinding>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        target_language_name,
        converters,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_preserves_pg_transform_identity() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let binding = converter_binding(
        Some(converter_function(
            "payload_from_sql",
            internal_type(),
            "payload_from_sql_v1",
        )),
        Some(converter_function(
            "payload_to_sql",
            custom_payload_type(),
            "payload_to_sql_v1",
        )),
    );
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            vec![binding],
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert_eq!(receipt.location().target_language_name(), "internal");
    assert_eq!(receipt.location().converters().len(), 1);
    assert_eq!(
        receipt.location().converters()[0].transform_type(),
        &custom_payload_type()
    );
    assert!(receipt.location().converters()[0].from_sql().is_some());
    assert!(receipt.location().converters()[0].to_sql().is_some());
    assert_eq!(receipt.source_id(), transform_types.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .ends_with("/1/procedure-transform-converters"));
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_distinguishes_converter_replacement() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let left = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
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
    let right = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
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
                None,
            )],
        )],
    )
    .unwrap();

    assert_ne!(left.snapshot_digest(), right.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_accepts_one_direction_only() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    for binding in [
        converter_binding(
            Some(converter_function(
                "payload_from_sql",
                internal_type(),
                "payload_from_sql_v1",
            )),
            None,
        ),
        converter_binding(
            None,
            Some(converter_function(
                "payload_to_sql",
                custom_payload_type(),
                "payload_to_sql_v1",
            )),
        ),
    ] {
        IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
            &transform_types,
            &definition,
            vec![converter_observation(
                operator("="),
                procedure("int4eq"),
                "internal",
                vec![binding],
            )],
        )
        .unwrap();
    }
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_missing_selected_transform_row() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            vec![],
        )],
    )
    .expect_err("every protrftypes selection must resolve one exact pg_transform row");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_row_when_protrftypes_is_null() {
    let transform_types = null_transform_types_predecessor();
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
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
    .expect_err("NULL protrftypes must not acquire an unrelated pg_transform row");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_target_language_drift() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("="),
            procedure("int4eq"),
            "plpython3u",
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
    .expect_err("pg_transform.trflang must equal the exact target pg_proc.prolang resolution");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_language_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_empty_transform_row() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterBinding::new(
        custom_payload_type(),
        None,
        None,
    )
    .expect_err("CREATE TRANSFORM must provide at least one conversion direction");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_direction",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_from_sql_return_drift() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterBinding::new(
        custom_payload_type(),
        Some(converter_function(
            "payload_from_sql",
            custom_payload_type(),
            "payload_from_sql_v1",
        )),
        None,
    )
    .expect_err("FROM SQL converter must return pg_catalog.internal");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_from_sql_return_type",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_to_sql_return_drift() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterBinding::new(
        custom_payload_type(),
        None,
        Some(converter_function(
            "payload_to_sql",
            QualifiedTypeName::new("pg_catalog", "text").unwrap(),
            "payload_to_sql_v1",
        )),
    )
    .expect_err("TO SQL converter must return the exact transform type");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_to_sql_return_type",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_non_internal_argument() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterFunction::new(
        "public",
        "payload_from_sql",
        custom_payload_type(),
        internal_type(),
        converter_definition("payload_from_sql_v1"),
    )
    .expect_err("PostgreSQL transform converter functions take one internal argument");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_argument_type",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_binding_drift() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
        &transform_types,
        &definition,
        vec![converter_observation(
            operator("<>"),
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
    .expect_err("transform converter evidence must remain bound to the exact governed operator");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        "internal",
        vec![],
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_rejects_unknown_receipt_coordinate() {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
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
    let error = snapshot
        .source_receipt(coordinate(), 2)
        .expect_err("receipt lookup must remain exact-coordinate and exact-position bound");
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn ordinary_exclude_operator_procedure_transform_converter_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot>() > 0);
}
