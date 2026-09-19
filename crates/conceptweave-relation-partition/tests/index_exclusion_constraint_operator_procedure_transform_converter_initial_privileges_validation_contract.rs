//! Recovery-validation contract for transform-converter initial privileges.

use conceptweave_relation_partition::{
    validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial,
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
fn initial_privilege_recovery_validation_admits_absence_and_binds_resolved_material() {
    let absent =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(None);
    assert!(absent.is_ready());
    assert_eq!(absent.material_digest(), None);
    assert_eq!(absent.unresolved_grantee_count(), 0);
    assert_eq!(absent.unresolved_grantor_count(), 0);

    let resolved = material(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            "application_reader",
            "extension_owner",
            false,
        )
        .expect("resolved grant"),
    ]);
    let resolved_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            Some(&resolved),
        );
    assert!(resolved_validation.is_ready());
    assert_eq!(resolved_validation.material_digest(), Some(resolved.digest()));
    assert_eq!(resolved_validation.unresolved_grantee_count(), 0);
    assert_eq!(resolved_validation.unresolved_grantor_count(), 0);
}

#[test]
fn initial_privilege_recovery_validation_blocks_and_binds_unresolved_grantee_and_grantor() {
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
    assert!(!validation.is_ready());
    assert_eq!(validation.material_digest(), Some(damaged.digest()));
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
    assert_eq!(
        grantee_validation.material_digest(),
        Some(dangling_grantee.digest())
    );
    assert_eq!(grantee_validation.unresolved_grantee_count(), 1);
    assert_eq!(grantee_validation.unresolved_grantor_count(), 0);

    let grantor_validation =
        validate_index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_recovery(
            Some(&dangling_grantor),
        );
    assert_eq!(
        grantor_validation.material_digest(),
        Some(dangling_grantor.digest())
    );
    assert_eq!(grantor_validation.unresolved_grantee_count(), 0);
    assert_eq!(grantor_validation.unresolved_grantor_count(), 1);
}
