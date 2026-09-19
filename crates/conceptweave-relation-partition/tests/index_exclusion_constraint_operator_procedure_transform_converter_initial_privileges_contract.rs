include!("index_exclusion_constraint_operator_procedure_transform_converter_security_label_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType,
};

fn converter_security_label_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot {
    let predecessor = converter_auto_extension_dependency_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
        &predecessor,
        complete_converter_security_label_observations(),
    )
    .unwrap()
}

fn initial_public_execute(
    grantor: &str,
    grant_option: bool,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant {
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::public(
        grantor,
        grant_option,
    )
    .unwrap()
}

fn initial_role_execute(
    grantee: &str,
    grantor: &str,
    grant_option: bool,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant {
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
        grantee,
        grantor,
        grant_option,
    )
    .unwrap()
}

fn extension_initial_privileges(
    grants: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial {
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::new(
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType::Extension,
        grants,
    )
    .unwrap()
}

fn initial_privilege_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    initial_privileges: Option<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        initial_privileges,
    )
    .unwrap()
}

fn complete_initial_privilege_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation> {
    vec![
        initial_privilege_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        ),
        initial_privilege_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            None,
        ),
    ]
}

#[test]
fn ordinary_exclude_converter_initial_privileges_distinguish_absent_and_extension_baselines() {
    let predecessor = converter_security_label_snapshot();
    let absent = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        complete_initial_privilege_observations(),
    )
    .unwrap();

    let mut extension_observations = complete_initial_privilege_observations();
    extension_observations[0] = initial_privilege_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(extension_initial_privileges(vec![initial_public_execute(
            "postgres",
            false,
        )])),
    );
    let extension = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        extension_observations,
    )
    .unwrap();

    assert!(absent.observations()[0].initial_privileges().is_none());
    let material = extension.observations()[0].initial_privileges().unwrap();
    assert_eq!(
        material.privilege_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType::Extension
    );
    assert_eq!(material.grant_count(), 1);
    assert_ne!(absent.snapshot_digest(), extension.snapshot_digest());
}

#[test]
fn ordinary_exclude_converter_initial_privileges_preserve_privtype_and_canonical_acl_identity() {
    let predecessor = converter_security_label_snapshot();
    let grants_left = vec![
        initial_role_execute("analytics", "postgres", true),
        initial_public_execute("postgres", false),
    ];
    let grants_right = vec![
        initial_public_execute("postgres", false),
        initial_role_execute("analytics", "postgres", true),
    ];

    let left_material = extension_initial_privileges(grants_left);
    let right_material = extension_initial_privileges(grants_right);
    assert_eq!(left_material.digest(), right_material.digest());

    let initdb_material =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::new(
            IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType::Initdb,
            vec![
                initial_public_execute("postgres", false),
                initial_role_execute("analytics", "postgres", true),
            ],
        )
        .unwrap();
    assert_ne!(left_material.digest(), initdb_material.digest());

    let mut extension_observations = complete_initial_privilege_observations();
    extension_observations[0] = initial_privilege_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(left_material),
    );
    let extension = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        extension_observations,
    )
    .unwrap();

    let mut initdb_observations = complete_initial_privilege_observations();
    initdb_observations[0] = initial_privilege_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        Some(initdb_material),
    );
    let initdb = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        initdb_observations,
    )
    .unwrap();
    assert_ne!(extension.snapshot_digest(), initdb.snapshot_digest());
}

#[test]
fn ordinary_exclude_converter_initial_privileges_reject_duplicate_acl_entries_and_blank_roles() {
    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeMaterial::new(
            IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeType::Extension,
            vec![
                initial_public_execute("postgres", false),
                initial_public_execute("postgres", false),
            ],
        )
        .expect_err("duplicate aclitems must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grant",
    );

    let blank =
        IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant::role(
            "  ",
            "postgres",
            false,
        )
        .expect_err("resolved grantee role must be nonblank");
    assert_field(
        blank,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_grantee_role_name",
    );
}

#[test]
fn ordinary_exclude_converter_initial_privileges_reject_completeness_binding_and_duplicate_coordinates() {
    let predecessor = converter_security_label_snapshot();
    let missing = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        vec![initial_privilege_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            None,
        )],
    )
    .expect_err("every converter direction needs explicit pg_init_privs evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_completeness",
    );

    let observation = initial_privilege_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        None,
    );
    let duplicate = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter coordinates must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_coordinate",
    );

    let mut drift = complete_initial_privilege_observations();
    drift[0] = initial_privilege_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        None,
    );
    let binding = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        drift,
    )
    .expect_err("initial privileges must remain bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_initial_privilege_binding",
    );
}

#[test]
fn ordinary_exclude_converter_initial_privileges_preserve_raw_root_location_and_exact_receipt() {
    let predecessor = converter_security_label_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot::new(
        &predecessor,
        complete_initial_privilege_observations(),
    )
    .unwrap();
    assert_eq!(snapshot.converter_snapshot_digest(), predecessor.converter_snapshot_digest());

    let receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .unwrap();
    assert!(receipt.location().initial_privileges().is_none());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let left = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation::new(
        coordinate(),
        1,
        QualifiedTypeName::new("payload.domain", "json").unwrap(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        None,
    )
    .unwrap()
    .canonical_location();
    let right = IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeObservation::new(
        coordinate(),
        1,
        QualifiedTypeName::new("payload", "domain.json").unwrap(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        None,
    )
    .unwrap()
    .canonical_location();
    assert_ne!(left, right);

    let missing = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err("receipt lookup remains exact-coordinate bound");
    assert!(matches!(missing, ObservationError::UnknownObservationLocation { .. }));
}
