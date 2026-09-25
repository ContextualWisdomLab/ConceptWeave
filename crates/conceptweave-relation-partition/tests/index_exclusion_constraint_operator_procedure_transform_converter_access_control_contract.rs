include!("index_exclusion_constraint_operator_procedure_transform_converter_owner_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial,
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot,
    IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant,
};

fn converter_owner_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot {
    let predecessor = converter_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot::new(
        &predecessor,
        complete_owner_observations(),
    )
    .unwrap()
}

fn access_control_material(
    proacl_was_null: bool,
    role_name: &str,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial {
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial::new(
        proacl_was_null,
        vec![
            IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant::public(
                "transform_runtime",
                false,
            )
            .unwrap(),
            IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant::role(
                role_name,
                "transform_runtime",
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn access_control_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    proacl_was_null: bool,
    role_name: &str,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        access_control_material(proacl_was_null, role_name),
    )
    .unwrap()
}

fn complete_access_control_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation> {
    vec![
        access_control_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            false,
            "transform_client",
        ),
        access_control_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            false,
            "transform_client",
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_acl_preserves_proacl_state_and_execute_grants() {
    let predecessor = converter_owner_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            complete_access_control_observations(),
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

    assert!(!receipt.location().access_control().proacl_was_null());
    assert_eq!(receipt.location().access_control().grant_count(), 2);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_acl_distinguishes_null_from_explicit_acl() {
    let predecessor = converter_owner_snapshot();
    let explicit =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            complete_access_control_observations(),
        )
        .unwrap();
    let mut null_acl = complete_access_control_observations();
    null_acl[0] = access_control_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        true,
        "transform_client",
    );
    let implicit =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            null_acl,
        )
        .unwrap();
    assert_ne!(explicit.snapshot_digest(), implicit.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_acl_distinguishes_execute_grant_changes() {
    let predecessor = converter_owner_snapshot();
    let left =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            complete_access_control_observations(),
        )
        .unwrap();
    let mut changed = complete_access_control_observations();
    changed[0] = access_control_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        false,
        "transform_auditor",
    );
    let right =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            changed,
        )
        .unwrap();
    assert_ne!(left.snapshot_digest(), right.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_acl_rejects_missing_direction() {
    let predecessor = converter_owner_snapshot();
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            vec![access_control_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                false,
                "transform_client",
            )],
        )
        .expect_err("every nonzero converter direction must carry one ACL observation");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_access_control_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_acl_rejects_binding_drift() {
    let predecessor = converter_owner_snapshot();
    let mut observations = complete_access_control_observations();
    observations[0] = access_control_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        false,
        "transform_client",
    );
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot::new(
            &predecessor,
            observations,
        )
        .expect_err("ACL evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_access_control_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_acl_rejects_duplicate_grant() {
    let grant = IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant::public(
        "transform_runtime",
        false,
    )
    .unwrap();
    let error =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial::new(
            false,
            vec![grant.clone(), grant],
        )
        .expect_err("duplicate effective EXECUTE grant evidence must fail closed");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_access_control_grant",
    );
}

#[test]
fn ordinary_exclude_transform_converter_acl_snapshot_is_publicly_composed() {
    assert!(
        std::mem::size_of::<
            IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot,
        >() > 0
    );
}
