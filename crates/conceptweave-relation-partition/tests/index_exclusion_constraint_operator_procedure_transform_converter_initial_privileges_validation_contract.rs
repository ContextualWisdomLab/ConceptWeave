//! Recovery-validation contract for transform-converter initial privileges.
//!
//! Reuse the full Source Observation fixture chain so validation is exercised against owner-issued
//! provenance rather than detached ACL material.

include!("index_exclusion_constraint_operator_procedure_transform_converter_initial_privileges_contract.rs");

use conceptweave_relation_partition::{
    validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
};

fn initial_privilege_snapshot_with_materials(
    from_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>,
    to_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot {
    let predecessor = converter_security_label_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        vec![
            initial_privilege_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                from_sql,
            ),
            initial_privilege_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
                "payload_to_sql",
                to_sql,
            ),
        ],
    )
    .expect("valid initial privilege snapshot")
}

fn assert_exact_receipt_binding(
    validation: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation,
    receipt: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSourceReceipt,
) {
    assert_eq!(validation.source_id(), receipt.source_id());
    assert_eq!(
        validation.connection_policy_binding(),
        receipt.connection_policy_binding()
    );
    assert_eq!(validation.source_digest(), receipt.source_digest());
    assert_eq!(validation.extractor_revision(), receipt.extractor_revision());
    assert_eq!(validation.observed_at_utc(), receipt.observed_at_utc());
    assert_eq!(
        validation.canonical_location(),
        receipt.location().canonical_location()
    );
    assert!(validation.matches_source_receipt(receipt));
}

#[test]
fn initial_privilege_recovery_validation_binds_identical_material_to_exact_observation_location() {
    let shared_material = extension_initial_privileges(vec![initial_public_execute(
        "postgres",
        false,
    )]);
    let snapshot = initial_privilege_snapshot_with_materials(
        Some(shared_material.clone()),
        Some(shared_material),
    );
    let from_receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("FROM SQL receipt");
    let to_receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
        )
        .expect("TO SQL receipt");

    let from_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &from_receipt,
        );
    let to_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &to_receipt,
        );

    assert!(from_validation.is_ready());
    assert!(to_validation.is_ready());
    assert_eq!(from_validation.material_digest(), to_validation.material_digest());
    assert_exact_receipt_binding(&from_validation, &from_receipt);
    assert_exact_receipt_binding(&to_validation, &to_receipt);
    assert_ne!(from_validation.canonical_location(), to_validation.canonical_location());
    assert!(!from_validation.matches_source_receipt(&to_receipt));
    assert!(!to_validation.matches_source_receipt(&from_receipt));
}

#[test]
fn initial_privilege_recovery_validation_binds_same_material_to_exact_snapshot_generation() {
    let shared_from = extension_initial_privileges(vec![initial_public_execute(
        "postgres",
        false,
    )]);
    let generation_a =
        initial_privilege_snapshot_with_materials(Some(shared_from.clone()), None);
    let generation_b = initial_privilege_snapshot_with_materials(
        Some(shared_from),
        Some(extension_initial_privileges(vec![initial_role_execute(
            "analytics",
            "postgres",
            false,
        )])),
    );
    assert_ne!(generation_a.snapshot_digest(), generation_b.snapshot_digest());

    let receipt_a = generation_a
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("generation A receipt");
    let receipt_b = generation_b
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("generation B receipt");
    let validation_a =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &receipt_a,
        );
    let validation_b =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &receipt_b,
        );

    assert_eq!(validation_a.material_digest(), validation_b.material_digest());
    assert_exact_receipt_binding(&validation_a, &receipt_a);
    assert_exact_receipt_binding(&validation_b, &receipt_b);
    assert_ne!(validation_a.source_digest(), validation_b.source_digest());
    assert!(!validation_a.matches_source_receipt(&receipt_b));
    assert!(!validation_b.matches_source_receipt(&receipt_a));
}

#[test]
fn initial_privilege_recovery_validation_binds_absence_to_exact_owner_receipt() {
    let snapshot = initial_privilege_snapshot_with_materials(None, None);
    let from_receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("FROM SQL receipt");
    let to_receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
        )
        .expect("TO SQL receipt");
    let from_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &from_receipt,
        );
    let to_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &to_receipt,
        );

    assert!(from_validation.is_ready());
    assert!(to_validation.is_ready());
    assert_eq!(from_validation.material_digest(), None);
    assert_eq!(to_validation.material_digest(), None);
    assert_exact_receipt_binding(&from_validation, &from_receipt);
    assert_exact_receipt_binding(&to_validation, &to_receipt);
    assert!(!from_validation.matches_source_receipt(&to_receipt));
    assert!(!to_validation.matches_source_receipt(&from_receipt));
}

#[test]
fn initial_privilege_recovery_validation_blocks_damaged_receipt_without_logging_raw_oids() {
    let damaged = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
            16_424,
            16_425,
            false,
        )
        .expect("dangling grant"),
    ]);
    let snapshot = initial_privilege_snapshot_with_materials(Some(damaged), None);
    let receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("damaged receipt");
    let expected_material_digest = receipt
        .location()
        .initial_privileges()
        .expect("present material")
        .digest()
        .to_owned();

    let validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &receipt,
        );
    assert!(!validation.is_ready());
    assert_eq!(validation.material_digest(), Some(expected_material_digest.as_str()));
    assert_exact_receipt_binding(&validation, &receipt);
    assert_eq!(validation.unresolved_grantee_count(), 1);
    assert_eq!(validation.unresolved_grantor_count(), 1);
    assert_eq!(validation.unresolved_grantee_oids(), &[16_424]);
    assert_eq!(validation.unresolved_grantor_oids(), &[16_425]);

    let diagnostic = format!("{validation:?}");
    assert!(!diagnostic.contains("16424"));
    assert!(!diagnostic.contains("16425"));
}

#[test]
fn initial_privilege_recovery_validation_distinguishes_equal_count_dangling_role_identities() {
    let snapshot_a = initial_privilege_snapshot_with_materials(
        Some(extension_initial_privileges(vec![
            IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
                16_424,
                16_425,
                false,
            )
            .expect("generation A dangling grant"),
        ])),
        None,
    );
    let snapshot_b = initial_privilege_snapshot_with_materials(
        Some(extension_initial_privileges(vec![
            IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
                26_424,
                26_425,
                false,
            )
            .expect("generation B dangling grant"),
        ])),
        None,
    );
    let receipt_a = snapshot_a
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("generation A receipt");
    let receipt_b = snapshot_b
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect("generation B receipt");

    let validation_a =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &receipt_a,
        );
    let validation_b =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            &receipt_b,
        );

    assert_eq!(validation_a.unresolved_grantee_count(), 1);
    assert_eq!(validation_b.unresolved_grantee_count(), 1);
    assert_eq!(validation_a.unresolved_grantor_count(), 1);
    assert_eq!(validation_b.unresolved_grantor_count(), 1);
    assert_eq!(validation_a.unresolved_grantee_oids(), &[16_424]);
    assert_eq!(validation_b.unresolved_grantee_oids(), &[26_424]);
    assert_eq!(validation_a.unresolved_grantor_oids(), &[16_425]);
    assert_eq!(validation_b.unresolved_grantor_oids(), &[26_425]);
}
