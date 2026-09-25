include!("index_exclusion_constraint_operator_procedure_security_definer_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureLeakproofObservation,
    IndexExclusionConstraintOperatorProcedureLeakproofSnapshot,
};

fn security_definer_snapshot() -> IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot {
    let procedure_kind = procedure_kind_snapshot();
    IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
            operator("="),
            procedure("int4eq"),
            false,
        )],
    )
    .unwrap()
}

fn leakproof_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    leakproof: bool,
) -> IndexExclusionConstraintOperatorProcedureLeakproofObservation {
    IndexExclusionConstraintOperatorProcedureLeakproofObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        leakproof,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_preserves_raw_provenance() {
    let security_definer = security_definer_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("="),
            procedure("int4eq"),
            false,
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert!(!receipt.location().leakproof());
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(
        receipt.source_id(),
        security_definer.source_connection_key()
    );
    assert_eq!(
        receipt.connection_policy_binding(),
        security_definer.connection_policy_binding()
    );
    assert_eq!(
        receipt.extractor_revision(),
        security_definer.extractor_revision()
    );
    assert_eq!(
        receipt.observed_at_utc(),
        security_definer.observed_at_utc()
    );
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure-leakproof")
    );
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_distinguishes_security_planner_state() {
    let security_definer = security_definer_snapshot();
    let not_leakproof = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("="),
            procedure("int4eq"),
            false,
        )],
    )
    .unwrap();
    let leakproof = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("="),
            procedure("int4eq"),
            true,
        )],
    )
    .unwrap();

    assert_ne!(not_leakproof.snapshot_digest(), leakproof.snapshot_digest());
    assert!(
        leakproof
            .source_receipt(coordinate(), 1)
            .unwrap()
            .location()
            .leakproof()
    );
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_rejects_procedure_binding_drift() {
    let security_definer = security_definer_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("="),
            procedure("int4ne"),
            false,
        )],
    )
    .expect_err("leakproofness must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_leakproof_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_rejects_operator_binding_drift() {
    let security_definer = security_definer_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("<>"),
            procedure("int4eq"),
            false,
        )],
    )
    .expect_err("leakproofness must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_leakproof_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_rejects_missing_evidence() {
    let security_definer = security_definer_snapshot();
    let error =
        IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(&security_definer, vec![])
            .expect_err("every governed operator function needs raw proleakproof evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_leakproof_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_rejects_duplicate_coordinate() {
    let security_definer = security_definer_snapshot();
    let observation = leakproof_observation(operator("="), procedure("int4eq"), false);
    let error = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate raw proleakproof evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_leakproof_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureLeakproofObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        false,
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_leakproof_rejects_unknown_receipt_coordinate() {
    let security_definer = security_definer_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureLeakproofSnapshot::new(
        &security_definer,
        vec![leakproof_observation(
            operator("="),
            procedure("int4eq"),
            false,
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
fn ordinary_exclude_operator_procedure_leakproof_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureLeakproofSnapshot>() > 0);
}
