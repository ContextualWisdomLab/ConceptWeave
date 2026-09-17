include!("index_exclusion_constraint_operator_procedure_transform_converter_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity,
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot,
};

fn converter_snapshot() -> IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot {
    let transform_types = selected_transform_types_predecessor();
    let definition = definition_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot::new(
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
                Some(converter_function(
                    "payload_to_sql",
                    custom_payload_type(),
                    "payload_to_sql_v1",
                )),
            )],
        )],
    )
    .unwrap()
}

fn converter_owner_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    owner_oid: u32,
    owner_role_name: &str,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation {
    let owner = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity::new(
        owner_oid,
        owner_role_name,
    )
    .unwrap();
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        owner,
    )
    .unwrap()
}

fn complete_owner_observations() -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation> {
    vec![
        converter_owner_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            16_384,
            "transform_runtime",
        ),
        converter_owner_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            16_384,
            "transform_runtime",
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_owner_preserves_exact_pg_proc_owner() {
    let predecessor = converter_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        complete_owner_observations(),
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

    assert_eq!(receipt.location().converter_schema_name(), "public");
    assert_eq!(receipt.location().converter_function_name(), "payload_from_sql");
    assert_eq!(receipt.location().owner_oid(), 16_384);
    assert_eq!(receipt.location().owner_role_name(), "transform_runtime");
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .contains("procedure-transform-converters"));
}

#[test]
fn ordinary_exclude_transform_converter_owner_distinguishes_owner_reassignment() {
    let predecessor = converter_snapshot();
    let left = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        complete_owner_observations(),
    )
    .unwrap();
    let mut changed = complete_owner_observations();
    changed[0] = converter_owner_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        32_768,
        "transform_security",
    );
    let right = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        changed,
    )
    .unwrap();

    assert_ne!(left.snapshot_digest(), right.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_owner_rejects_missing_converter_owner() {
    let predecessor = converter_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        vec![converter_owner_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            16_384,
            "transform_runtime",
        )],
    )
    .expect_err("every nonzero pg_transform converter must have one exact owner observation");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_owner_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_owner_rejects_converter_binding_drift() {
    let predecessor = converter_snapshot();
    let mut observations = complete_owner_observations();
    observations[0] = converter_owner_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        16_384,
        "transform_runtime",
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("owner evidence must remain bound to the exact nonzero converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_owner_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_owner_rejects_zero_owner_oid() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity::new(
        0,
        "transform_runtime",
    )
    .expect_err("pg_proc.proowner is a nonzero role OID");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_owner_oid",
    );
}

#[test]
fn ordinary_exclude_transform_converter_owner_rejects_blank_owner_role_name() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity::new(
        16_384,
        "   ",
    )
    .expect_err("owner OID resolution must retain a nonblank same-generation role name");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_owner_role_name",
    );
}

#[test]
fn ordinary_exclude_transform_converter_owner_rejects_duplicate_coordinate() {
    let predecessor = converter_snapshot();
    let mut observations = complete_owner_observations();
    observations.push(observations[0].clone());
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("converter owner coordinates must be unique");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_owner_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_owner_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        complete_owner_observations(),
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
fn ordinary_exclude_transform_converter_owner_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot>() > 0);
}
