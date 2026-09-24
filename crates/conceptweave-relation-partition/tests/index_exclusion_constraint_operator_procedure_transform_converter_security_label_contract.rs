include!(
    "index_exclusion_constraint_operator_procedure_transform_converter_auto_extension_dependency_contract.rs"
);

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel,
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot,
};

fn converter_auto_extension_dependency_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot {
    let predecessor = converter_extension_membership_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        complete_converter_auto_extension_dependency_observations(),
    )
    .unwrap()
}

fn converter_security_label(
    provider: &str,
    label: &str,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel {
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel::new(provider, label)
        .unwrap()
}

fn converter_security_label_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    labels: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel>,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        labels,
    )
    .unwrap()
}

fn complete_converter_security_label_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation> {
    vec![
        converter_security_label_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            vec![],
        ),
        converter_security_label_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            vec![],
        ),
    ]
}

#[test]
fn ordinary_exclude_converter_security_labels_distinguish_unlabeled_and_mac_labeled_functions() {
    let predecessor = converter_auto_extension_dependency_snapshot();
    let unlabeled =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            complete_converter_security_label_observations(),
        )
        .unwrap();

    let mut labeled_observations = complete_converter_security_label_observations();
    labeled_observations[0] = converter_security_label_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        vec![converter_security_label(
            "selinux",
            "system_u:object_r:sepgsql_trusted_proc_exec_t:s0",
        )],
    );
    let labeled =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            labeled_observations,
        )
        .unwrap();

    assert!(unlabeled.observations()[0].security_labels().is_empty());
    assert_eq!(
        labeled.observations()[0].security_labels()[0].provider(),
        "selinux"
    );
    assert_eq!(
        labeled.observations()[0].security_labels()[0].label(),
        "system_u:object_r:sepgsql_trusted_proc_exec_t:s0"
    );
    assert_ne!(unlabeled.snapshot_digest(), labeled.snapshot_digest());
}

#[test]
fn ordinary_exclude_converter_security_labels_preserve_provider_order_independently_and_raw_label_text()
 {
    let predecessor = converter_auto_extension_dependency_snapshot();
    let left =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            vec![
                converter_security_label_observation(
                    IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                    "payload_from_sql",
                    vec![
                        converter_security_label("z_provider", ""),
                        converter_security_label("a_provider", " raw label \t"),
                    ],
                ),
                converter_security_label_observation(
                    IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
                    "payload_to_sql",
                    vec![],
                ),
            ],
        )
        .unwrap();
    let right =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            vec![
                converter_security_label_observation(
                    IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                    "payload_from_sql",
                    vec![
                        converter_security_label("a_provider", " raw label \t"),
                        converter_security_label("z_provider", ""),
                    ],
                ),
                converter_security_label_observation(
                    IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
                    "payload_to_sql",
                    vec![],
                ),
            ],
        )
        .unwrap();

    assert_eq!(left.snapshot_digest(), right.snapshot_digest());
    assert_eq!(
        left.observations()[0].security_labels()[0].provider(),
        "a_provider"
    );
    assert_eq!(
        left.observations()[0].security_labels()[0].label(),
        " raw label \t"
    );
    assert_eq!(
        left.observations()[0].security_labels()[1].provider(),
        "z_provider"
    );
    assert_eq!(left.observations()[0].security_labels()[1].label(), "");
}

#[test]
fn ordinary_exclude_converter_security_labels_reject_blank_or_duplicate_providers_without_normalizing_labels()
 {
    let blank = IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabel::new(
        " \t",
        "provider-owned-value",
    )
    .expect_err("provider identity must be nonblank");
    assert_field(
        blank,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_label_provider",
    );

    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            vec![
                converter_security_label("selinux", "first"),
                converter_security_label("selinux", "second"),
            ],
        )
        .expect_err("PostgreSQL permits at most one security label per provider per object");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_label_provider",
    );
}

#[test]
fn ordinary_exclude_converter_security_labels_reject_completeness_binding_and_duplicate_coordinates()
 {
    let predecessor = converter_auto_extension_dependency_snapshot();
    let missing =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            vec![converter_security_label_observation(
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "payload_from_sql",
                vec![],
            )],
        )
        .expect_err("every converter direction needs explicit security-label evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_label_completeness",
    );

    let observation = converter_security_label_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        vec![],
    );
    let duplicate =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            vec![observation.clone(), observation],
        )
        .expect_err("duplicate converter coordinates must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_label_coordinate",
    );

    let mut drift = complete_converter_security_label_observations();
    drift[0] = converter_security_label_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        vec![],
    );
    let binding =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            drift,
        )
        .expect_err("security labels must stay bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_security_label_binding",
    );
}

#[test]
fn ordinary_exclude_converter_security_labels_preserve_root_location_and_exact_receipt() {
    let predecessor = converter_auto_extension_dependency_snapshot();
    let snapshot =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelSnapshot::new(
            &predecessor,
            complete_converter_security_label_observations(),
        )
        .unwrap();
    assert_eq!(
        snapshot.converter_snapshot_digest(),
        predecessor.converter_snapshot_digest()
    );

    let receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .unwrap();
    assert!(receipt.location().security_labels().is_empty());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let left =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation::new(
            coordinate(),
            1,
            QualifiedTypeName::new("payload.domain", "json").unwrap(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            vec![],
        )
        .unwrap()
        .canonical_location();
    let right =
        IndexExclusionConstraintOperatorProcedureTransformConverterSecurityLabelObservation::new(
            coordinate(),
            1,
            QualifiedTypeName::new("payload", "domain.json").unwrap(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            vec![],
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
    assert!(matches!(
        missing,
        ObservationError::UnknownObservationLocation { .. }
    ));
}
