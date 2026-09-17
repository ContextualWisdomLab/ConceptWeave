include!("index_exclusion_constraint_operator_procedure_configuration_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureAccessControlMaterial,
    IndexExclusionConstraintOperatorProcedureAccessControlObservation,
    IndexExclusionConstraintOperatorProcedureAccessControlSnapshot,
    IndexExclusionConstraintOperatorProcedureExecuteGrant,
};

fn configuration_snapshot() -> IndexExclusionConstraintOperatorProcedureConfigurationSnapshot {
    let owner = owner_snapshot();
    IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            None,
        )],
    )
    .unwrap()
}

fn default_execute_grants() -> Vec<IndexExclusionConstraintOperatorProcedureExecuteGrant> {
    vec![
        IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
            "postgres",
            "postgres",
            true,
        )
        .unwrap(),
        IndexExclusionConstraintOperatorProcedureExecuteGrant::public("postgres", false).unwrap(),
    ]
}

fn access_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    proacl_was_null: bool,
    grants: Vec<IndexExclusionConstraintOperatorProcedureExecuteGrant>,
) -> IndexExclusionConstraintOperatorProcedureAccessControlObservation {
    let material = IndexExclusionConstraintOperatorProcedureAccessControlMaterial::new(
        proacl_was_null,
        grants,
    )
    .unwrap();
    IndexExclusionConstraintOperatorProcedureAccessControlObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        material,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_preserves_effective_execute_authority() {
    let configuration = configuration_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            true,
            default_execute_grants(),
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert!(receipt.location().access_control().proacl_was_null());
    assert_eq!(receipt.location().access_control().grant_count(), 2);
    assert!(receipt
        .location()
        .access_control()
        .digest()
        .starts_with("sha256:"));
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_id(), configuration.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .ends_with("/1/procedure-access-control"));
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_distinguishes_default_from_explicit_equivalent_acl() {
    let configuration = configuration_snapshot();
    let implicit = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            true,
            default_execute_grants(),
        )],
    )
    .unwrap();
    let explicit = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            default_execute_grants(),
        )],
    )
    .unwrap();

    assert_ne!(implicit.snapshot_digest(), explicit.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_canonicalizes_acl_set_order() {
    let configuration = configuration_snapshot();
    let first = default_execute_grants();
    let mut reversed = default_execute_grants();
    reversed.reverse();
    let left = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            first,
        )],
    )
    .unwrap();
    let right = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            reversed,
        )],
    )
    .unwrap();

    assert_eq!(left.snapshot_digest(), right.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_distinguishes_grantee_grantor_and_grant_option() {
    let configuration = configuration_snapshot();
    let baseline = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            vec![IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
                "app_role",
                "postgres",
                false,
            )
            .unwrap()],
        )],
    )
    .unwrap();
    let other_grantee = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            vec![IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
                "report_role",
                "postgres",
                false,
            )
            .unwrap()],
        )],
    )
    .unwrap();
    let other_grantor = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            vec![IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
                "app_role",
                "security_admin",
                false,
            )
            .unwrap()],
        )],
    )
    .unwrap();
    let grantable = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            false,
            vec![IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
                "app_role",
                "postgres",
                true,
            )
            .unwrap()],
        )],
    )
    .unwrap();

    assert_ne!(baseline.snapshot_digest(), other_grantee.snapshot_digest());
    assert_ne!(baseline.snapshot_digest(), other_grantor.snapshot_digest());
    assert_ne!(baseline.snapshot_digest(), grantable.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_duplicate_effective_grant() {
    let grant = IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
        "app_role",
        "postgres",
        false,
    )
    .unwrap();
    let error = IndexExclusionConstraintOperatorProcedureAccessControlMaterial::new(
        false,
        vec![grant.clone(), grant],
    )
    .expect_err("duplicate effective ACL evidence must fail closed");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_access_control_grant",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_blank_role_identity() {
    let error = IndexExclusionConstraintOperatorProcedureExecuteGrant::role(
        " ",
        "postgres",
        false,
    )
    .expect_err("grantee role identity must be explicit");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_access_control_grantee_role_name",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_procedure_binding_drift() {
    let configuration = configuration_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4ne"),
            false,
            default_execute_grants(),
        )],
    )
    .expect_err("ACL evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_access_control_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_operator_binding_drift() {
    let configuration = configuration_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("<>"),
            procedure("int4eq"),
            false,
            default_execute_grants(),
        )],
    )
    .expect_err("ACL evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_access_control_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_missing_evidence() {
    let configuration = configuration_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![],
    )
    .expect_err("every governed operator function needs ACL evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_access_control_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_duplicate_coordinate() {
    let configuration = configuration_snapshot();
    let observation = access_observation(
        operator("="),
        procedure("int4eq"),
        false,
        default_execute_grants(),
    );
    let error = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate ACL observations must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_access_control_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureAccessControlObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        IndexExclusionConstraintOperatorProcedureAccessControlMaterial::new(
            true,
            default_execute_grants(),
        )
        .unwrap(),
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_access_control_rejects_unknown_receipt_coordinate() {
    let configuration = configuration_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            true,
            default_execute_grants(),
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
fn ordinary_exclude_operator_procedure_access_control_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureAccessControlSnapshot>() > 0);
}
