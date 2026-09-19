//! Recovery-validation contract for transform-converter initial privileges.

use conceptweave_relation_partition::{
    validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType,
};

fn material(
    grants: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial {
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::new(
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType::Extension,
        grants,
    )
    .expect("valid initial privilege material")
}

#[test]
fn initial_privilege_recovery_validation_admits_absence_and_resolved_material() {
    assert_eq!(
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(None),
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation::Ready,
    );

    let resolved = material(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            "application_reader",
            "extension_owner",
            false,
        )
        .expect("resolved grant"),
    ]);
    assert_eq!(
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(Some(&resolved)),
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation::Ready,
    );
}

#[test]
fn initial_privilege_recovery_validation_blocks_unresolved_grantee_and_grantor() {
    let damaged = material(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
            16_424,
            16_425,
            false,
        )
        .expect("dangling grant"),
    ]);

    let validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            Some(&damaged),
        );
    assert_eq!(
        validation,
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeRecoveryValidation::Blocked {
            unresolved_grantee_count: 1,
            unresolved_grantor_count: 1,
        },
    );
    assert!(!validation.is_ready());
    assert_eq!(validation.unresolved_grantee_count(), 1);
    assert_eq!(validation.unresolved_grantor_count(), 1);

    let diagnostic = format!("{validation:?}");
    assert!(!diagnostic.contains("16424"));
    assert!(!diagnostic.contains("16425"));
}

#[test]
fn initial_privilege_recovery_validation_counts_each_unresolved_dimension_independently() {
    let dangling_grantee = material(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_grantee_oid(
            42,
            "extension_owner",
            false,
        )
        .expect("dangling grantee"),
    ]);
    let dangling_grantor = material(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role_with_unresolved_grantor_oid(
            "application_reader",
            43,
            false,
        )
        .expect("dangling grantor"),
    ]);

    let grantee_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            Some(&dangling_grantee),
        );
    assert_eq!(grantee_validation.unresolved_grantee_count(), 1);
    assert_eq!(grantee_validation.unresolved_grantor_count(), 0);

    let grantor_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            Some(&dangling_grantor),
        );
    assert_eq!(grantor_validation.unresolved_grantee_count(), 0);
    assert_eq!(grantor_validation.unresolved_grantor_count(), 1);
}
