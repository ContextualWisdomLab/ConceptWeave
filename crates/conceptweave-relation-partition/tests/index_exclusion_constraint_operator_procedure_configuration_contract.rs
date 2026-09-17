include!("index_exclusion_constraint_operator_procedure_owner_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureConfigurationMaterial,
    IndexExclusionConstraintOperatorProcedureConfigurationObservation,
    IndexExclusionConstraintOperatorProcedureConfigurationSnapshot,
};

fn owner_snapshot() -> IndexExclusionConstraintOperatorProcedureOwnerSnapshot {
    let definition = definition_snapshot();
    IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            10,
            "postgres",
        )],
    )
    .unwrap()
}

fn configuration_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    proconfig: Option<Vec<&str>>,
) -> IndexExclusionConstraintOperatorProcedureConfigurationObservation {
    let material = IndexExclusionConstraintOperatorProcedureConfigurationMaterial::from_proconfig(
        proconfig.map(|entries| entries.into_iter().map(str::to_owned).collect()),
    );
    IndexExclusionConstraintOperatorProcedureConfigurationObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        material,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_preserves_exact_proconfig_identity() {
    let owner = owner_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec!["search_path=pg_catalog, pg_temp"]),
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert!(receipt.location().configuration().is_configured());
    assert_eq!(receipt.location().configuration().entry_count(), 1);
    assert!(receipt.location().configuration().digest().starts_with("sha256:"));
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_id(), owner.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        owner.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), owner.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), owner.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .ends_with("/1/procedure-configuration"));
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_distinguishes_null_from_empty_array() {
    let owner = owner_snapshot();
    let absent = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            None,
        )],
    )
    .unwrap();
    let empty = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec![]),
        )],
    )
    .unwrap();

    assert_ne!(absent.snapshot_digest(), empty.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_distinguishes_search_path_changes() {
    let owner = owner_snapshot();
    let secure = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec!["search_path=pg_catalog, pg_temp"]),
        )],
    )
    .unwrap();
    let caller_path = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec!["search_path=public"]),
        )],
    )
    .unwrap();

    assert_ne!(secure.snapshot_digest(), caller_path.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_preserves_catalog_array_order() {
    let owner = owner_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec!["search_path=pg_catalog, pg_temp", "work_mem=4MB"]),
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec!["work_mem=4MB", "search_path=pg_catalog, pg_temp"]),
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_rejects_procedure_binding_drift() {
    let owner = owner_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4ne"),
            None,
        )],
    )
    .expect_err("configuration evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_configuration_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_rejects_operator_binding_drift() {
    let owner = owner_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("<>"),
            procedure("int4eq"),
            None,
        )],
    )
    .expect_err("configuration evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_configuration_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_rejects_missing_evidence() {
    let owner = owner_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(&owner, vec![])
        .expect_err("every governed operator function needs proconfig evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_configuration_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_rejects_duplicate_coordinate() {
    let owner = owner_snapshot();
    let observation = configuration_observation(operator("="), procedure("int4eq"), None);
    let error = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate configuration evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_configuration_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureConfigurationObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        IndexExclusionConstraintOperatorProcedureConfigurationMaterial::from_proconfig(None),
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_configuration_rejects_unknown_receipt_coordinate() {
    let owner = owner_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureConfigurationSnapshot::new(
        &owner,
        vec![configuration_observation(
            operator("="),
            procedure("int4eq"),
            None,
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
fn ordinary_exclude_operator_procedure_configuration_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureConfigurationSnapshot>() > 0);
}
