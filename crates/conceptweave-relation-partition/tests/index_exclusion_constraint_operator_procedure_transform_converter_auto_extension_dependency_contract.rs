include!(
    "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_contract.rs"
);

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot,
};

fn converter_extension_membership_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot {
    let predecessor = converter_transform_types_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot::new(
        &predecessor,
        complete_converter_extension_membership_observations(),
    )
    .unwrap()
}

fn converter_auto_extension_dependency_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    extension_names: &[&str],
) -> IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        extension_names.iter().map(|name| (*name).to_owned()).collect(),
    )
    .unwrap()
}

fn complete_converter_auto_extension_dependency_observations() -> Vec<
    IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation,
> {
    vec![
        converter_auto_extension_dependency_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            &[],
        ),
        converter_auto_extension_dependency_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            &[],
        ),
    ]
}

#[test]
fn ordinary_exclude_converter_auto_extension_dependency_preserves_zero_one_and_multiple_edges() {
    let predecessor = converter_extension_membership_snapshot();
    let none = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        complete_converter_auto_extension_dependency_observations(),
    )
    .unwrap();
    assert!(none.observations()[0].extension_names().is_empty());

    let mut one_observations = complete_converter_auto_extension_dependency_observations();
    one_observations[0] = converter_auto_extension_dependency_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        &["payload_runtime"],
    );
    let one = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        one_observations,
    )
    .unwrap();
    assert_eq!(
        one.observations()[0].extension_names(),
        &["payload_runtime"]
    );
    assert_ne!(none.snapshot_digest(), one.snapshot_digest());

    let mut multiple_observations = complete_converter_auto_extension_dependency_observations();
    multiple_observations[0] = converter_auto_extension_dependency_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        &["payload_runtime", "shared_codec"],
    );
    let multiple = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        multiple_observations,
    )
    .unwrap();
    assert_eq!(
        multiple.observations()[0].extension_names(),
        &["payload_runtime", "shared_codec"]
    );
    assert_ne!(one.snapshot_digest(), multiple.snapshot_digest());
}

#[test]
fn ordinary_exclude_converter_auto_extension_dependency_canonicalizes_set_order() {
    let predecessor = converter_extension_membership_snapshot();
    let mut left_observations = complete_converter_auto_extension_dependency_observations();
    left_observations[0] = converter_auto_extension_dependency_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        &["zeta_runtime", "alpha_runtime"],
    );
    let left = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        left_observations,
    )
    .unwrap();

    let mut right_observations = complete_converter_auto_extension_dependency_observations();
    right_observations[0] = converter_auto_extension_dependency_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        &["alpha_runtime", "zeta_runtime"],
    );
    let right = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        right_observations,
    )
    .unwrap();

    assert_eq!(left.snapshot_digest(), right.snapshot_digest());
    assert_eq!(
        left.observations()[0].extension_names(),
        &["alpha_runtime", "zeta_runtime"]
    );
}

#[test]
fn ordinary_exclude_converter_auto_extension_dependency_rejects_blank_and_duplicate_extension_names()
 {
    let blank = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        vec![" \t".to_owned()],
    )
    .expect_err("auto-extension dependency names are exact nonblank pg_extension names");
    assert_field(
        blank,
        "index_exclusion_constraint_operator_procedure_transform_converter_auto_extension_dependency_extension_name",
    );

    let duplicate = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        vec!["payload_runtime".to_owned(), "payload_runtime".to_owned()],
    )
    .expect_err("duplicate pg_depend.deptype='x' edges must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_auto_extension_dependency_extension_name",
    );
}

#[test]
fn ordinary_exclude_converter_auto_extension_dependency_rejects_completeness_binding_and_duplicate_coordinates()
 {
    let predecessor = converter_extension_membership_snapshot();
    let missing = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        vec![converter_auto_extension_dependency_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            &[],
        )],
    )
    .expect_err("every converter direction needs explicit auto-extension dependency evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_auto_extension_dependency_completeness",
    );

    let observation = converter_auto_extension_dependency_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        &[],
    );
    let duplicate = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter coordinates must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_auto_extension_dependency_coordinate",
    );

    let mut drift = complete_converter_auto_extension_dependency_observations();
    drift[0] = converter_auto_extension_dependency_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        &[],
    );
    let binding = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        drift,
    )
    .expect_err("auto-extension dependency evidence must stay bound to the exact converter function");
    assert_field(
        binding,
        "index_exclusion_constraint_operator_procedure_transform_converter_auto_extension_dependency_binding",
    );
}

#[test]
fn ordinary_exclude_converter_auto_extension_dependency_preserves_root_location_and_receipt() {
    let predecessor = converter_extension_membership_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencySnapshot::new(
        &predecessor,
        complete_converter_auto_extension_dependency_observations(),
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
    assert!(receipt.location().extension_names().is_empty());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());

    let left = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation::new(
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
    let right = IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation::new(
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
