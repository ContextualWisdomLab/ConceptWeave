include!("index_exclusion_constraint_operator_procedure_transform_converter_initial_privileges_contract.rs");

#[test]
fn ordinary_exclude_converter_initial_privileges_preserve_dangling_role_oid_identity() {
    let resolved_numeric_name =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            26_424,
            "16424",
            10,
            "postgres",
            false,
        )
        .unwrap();
    let dangling_grantee =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_grantee_oid(
            16_424,
            10,
            "postgres",
            false,
        )
        .unwrap();
    let dangling_grantor =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role_with_unresolved_grantor_oid(
            20,
            "analytics",
            16_425,
            true,
        )
        .unwrap();
    let both_dangling =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
            16_424,
            16_425,
            true,
        )
        .unwrap();
    let public_dangling_grantor =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::public_with_unresolved_grantor_oid(
            16_425,
            false,
        )
        .unwrap();

    let resolved = extension_initial_privileges(vec![resolved_numeric_name]);
    let dangling = extension_initial_privileges(vec![dangling_grantee]);
    assert_ne!(
        resolved.digest(),
        dangling.digest(),
        "a numeric role name must not alias an unresolved raw role OID"
    );

    let left = extension_initial_privileges(vec![dangling_grantor, public_dangling_grantor]);
    let right = extension_initial_privileges(vec![both_dangling]);
    assert_ne!(left.digest(), right.digest());
}

#[test]
fn ordinary_exclude_converter_initial_privileges_expose_dangling_role_diagnostics() {
    let resolved = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            20,
            "analytics",
            10,
            "postgres",
            false,
        )
        .unwrap(),
    ]);
    assert_eq!(resolved.unresolved_grantee_count(), 0);
    assert_eq!(resolved.unresolved_grantor_count(), 0);

    let dangling_grantee = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_grantee_oid(
            16_424,
            10,
            "postgres",
            false,
        )
        .unwrap(),
    ]);
    assert_eq!(dangling_grantee.unresolved_grantee_count(), 1);
    assert_eq!(dangling_grantee.unresolved_grantor_count(), 0);

    let dangling_grantor = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role_with_unresolved_grantor_oid(
            20,
            "analytics",
            16_425,
            true,
        )
        .unwrap(),
    ]);
    assert_eq!(dangling_grantor.unresolved_grantee_count(), 0);
    assert_eq!(dangling_grantor.unresolved_grantor_count(), 1);

    let both_dangling = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
            16_424,
            16_425,
            true,
        )
        .unwrap(),
    ]);
    assert_eq!(both_dangling.unresolved_grantee_count(), 1);
    assert_eq!(both_dangling.unresolved_grantor_count(), 1);

    let public_with_dangling_grantor = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::public_with_unresolved_grantor_oid(
            16_425,
            false,
        )
        .unwrap(),
    ]);
    assert_eq!(public_with_dangling_grantor.unresolved_grantee_count(), 0);
    assert_eq!(public_with_dangling_grantor.unresolved_grantor_count(), 1);
}

#[test]
fn ordinary_exclude_converter_initial_privileges_retain_readable_source_order_acl_semantics() {
    let material = extension_initial_privileges(vec![
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_grantee_oid(
            16_424,
            10,
            "postgres",
            true,
        )
        .unwrap(),
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            20,
            "analytics",
            10,
            "postgres",
            false,
        )
        .unwrap(),
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::public(
            10,
            "postgres",
            false,
        )
        .unwrap(),
    ]);

    let grants = material.grants();
    assert_eq!(grants.len(), 3);

    assert!(!grants[0].is_public_grantee());
    assert_eq!(grants[0].resolved_grantee_role_name(), None);
    assert!(grants[0].has_unresolved_grantee());
    assert_eq!(grants[0].resolved_grantor_role_name(), Some("postgres"));
    assert!(!grants[0].has_unresolved_grantor());
    assert!(grants[0].grant_option());

    assert!(!grants[1].is_public_grantee());
    assert_eq!(grants[1].resolved_grantee_role_name(), Some("analytics"));
    assert!(!grants[1].has_unresolved_grantee());
    assert_eq!(grants[1].resolved_grantor_role_name(), Some("postgres"));
    assert!(!grants[1].has_unresolved_grantor());
    assert!(!grants[1].grant_option());

    assert!(grants[2].is_public_grantee());
    assert_eq!(grants[2].resolved_grantee_role_name(), None);
    assert!(!grants[2].has_unresolved_grantee());
    assert_eq!(grants[2].resolved_grantor_role_name(), Some("postgres"));
    assert!(!grants[2].has_unresolved_grantor());
    assert!(!grants[2].grant_option());

    let diagnostic = format!("{material:?}");
    assert!(!diagnostic.contains("16424"));
}

#[test]
fn ordinary_exclude_converter_initial_privileges_reject_public_oid_as_unresolved_role() {
    let dangling_grantee =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_grantee_oid(
            0,
            10,
            "postgres",
            false,
        )
        .expect_err("OID zero is PUBLIC, not an unresolved role identity");
    assert_field(
        dangling_grantee,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_oid",
    );

    let dangling_grantor =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::public_with_unresolved_grantor_oid(
            0,
            false,
        )
        .expect_err("grantor OID zero is not a role identity");
    assert_field(
        dangling_grantor,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_oid",
    );

    let resolved_grantee =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            0,
            "analytics",
            10,
            "postgres",
            false,
        )
        .expect_err("resolved grantee OID zero is PUBLIC, not a role identity");
    assert_field(
        resolved_grantee,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_oid",
    );

    let resolved_grantor =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::public(
            0,
            "postgres",
            false,
        )
        .expect_err("resolved grantor OID zero is not a role identity");
    assert_field(
        resolved_grantor,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantor_role_oid",
    );
}

#[test]
fn ordinary_exclude_converter_initial_privilege_grant_debug_redacts_dangling_role_oids() {
    let unresolved =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::unresolved_role_oids(
            16_424,
            16_425,
            true,
        )
        .expect("valid dangling grant");
    let diagnostic = format!("{unresolved:?}");

    assert!(!diagnostic.contains("16424"));
    assert!(!diagnostic.contains("16425"));
    assert!(diagnostic.contains("UnresolvedRoleOid"));
    assert!(diagnostic.contains("grant_option: true"));
}
