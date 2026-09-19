include!("index_exclusion_constraint_operator_procedure_definition_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureOwnerObservation,
    IndexExclusionConstraintOperatorProcedureOwnerSnapshot,
};

fn definition_snapshot() -> IndexExclusionConstraintOperatorProcedureDefinitionSnapshot {
    let leakproof = leakproof_snapshot();
    IndexExclusionConstraintOperatorProcedureDefinitionSnapshot::new(
        &leakproof,
        vec![definition_observation(
            operator("="),
            procedure("int4eq"),
            "internal",
            "int4eq",
            None,
            None,
        )],
    )
    .unwrap()
}

fn owner_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    owner_oid: u32,
    owner_role_name: &str,
) -> IndexExclusionConstraintOperatorProcedureOwnerObservation {
    IndexExclusionConstraintOperatorProcedureOwnerObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        owner_oid,
        owner_role_name,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_owner_preserves_exact_proowner_and_role_resolution() {
    let definition = definition_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            10,
            "postgres",
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert_eq!(receipt.location().owner_oid(), 10);
    assert_eq!(receipt.location().owner_role_name(), "postgres");
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_id(), definition.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        definition.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), definition.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), definition.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(receipt
        .location()
        .canonical_location()
        .ends_with("/1/procedure-owner"));
}

#[test]
fn ordinary_exclude_operator_procedure_owner_distinguishes_owner_oid_changes() {
    let definition = definition_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            10,
            "app_owner",
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            11,
            "app_owner",
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_owner_distinguishes_resolved_role_name_changes() {
    let definition = definition_snapshot();
    let first = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            10,
            "app_owner",
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            10,
            "renamed_owner",
        )],
    )
    .unwrap();

    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_zero_proowner_oid() {
    let error = IndexExclusionConstraintOperatorProcedureOwnerObservation::new(
        coordinate(),
        1,
        operator("="),
        procedure("int4eq"),
        0,
        "postgres",
    )
    .expect_err("pg_proc.proowner must resolve from a nonzero catalog OID");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_owner_oid",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_blank_resolved_role_name() {
    let error = IndexExclusionConstraintOperatorProcedureOwnerObservation::new(
        coordinate(),
        1,
        operator("="),
        procedure("int4eq"),
        10,
        "   ",
    )
    .expect_err("pg_proc.proowner must resolve to an exact nonblank pg_roles.rolname");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_owner_role_name",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_procedure_binding_drift() {
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4ne"),
            10,
            "postgres",
        )],
    )
    .expect_err("owner evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_owner_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_operator_binding_drift() {
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("<>"),
            procedure("int4eq"),
            10,
            "postgres",
        )],
    )
    .expect_err("owner evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_owner_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_missing_evidence() {
    let definition = definition_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(&definition, vec![])
        .expect_err("every governed operator function needs owner evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_owner_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_duplicate_coordinate() {
    let definition = definition_snapshot();
    let observation = owner_observation(operator("="), procedure("int4eq"), 10, "postgres");
    let error = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate owner evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_owner_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureOwnerObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        10,
        "postgres",
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_owner_rejects_unknown_receipt_coordinate() {
    let definition = definition_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureOwnerSnapshot::new(
        &definition,
        vec![owner_observation(
            operator("="),
            procedure("int4eq"),
            10,
            "postgres",
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
fn ordinary_exclude_operator_procedure_owner_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureOwnerSnapshot>() > 0);
}
