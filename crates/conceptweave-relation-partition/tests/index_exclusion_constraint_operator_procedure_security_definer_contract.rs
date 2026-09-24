include!("index_exclusion_constraint_operator_procedure_kind_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureSecurityDefinerObservation,
    IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot,
};

fn procedure_kind_snapshot() -> IndexExclusionConstraintOperatorProcedureKindSnapshot {
    let parallel_safety = parallel_safety_snapshot();
    IndexExclusionConstraintOperatorProcedureKindSnapshot::new(
        &parallel_safety,
        vec![kind_observation(operator("="), procedure("int4eq"))],
    )
    .unwrap()
}

fn security_definer_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    security_definer: bool,
) -> IndexExclusionConstraintOperatorProcedureSecurityDefinerObservation {
    IndexExclusionConstraintOperatorProcedureSecurityDefinerObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        security_definer,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_preserves_invoker_provenance() {
    let procedure_kind = procedure_kind_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
            operator("="),
            procedure("int4eq"),
            false,
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert!(!receipt.location().security_definer());
    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.source_id(), procedure_kind.source_connection_key());
    assert_eq!(
        receipt.connection_policy_binding(),
        procedure_kind.connection_policy_binding()
    );
    assert_eq!(
        receipt.extractor_revision(),
        procedure_kind.extractor_revision()
    );
    assert_eq!(receipt.observed_at_utc(), procedure_kind.observed_at_utc());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure-security-definer")
    );
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_distinguishes_execution_privilege() {
    let procedure_kind = procedure_kind_snapshot();
    let invoker = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
            operator("="),
            procedure("int4eq"),
            false,
        )],
    )
    .unwrap();
    let definer = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
            operator("="),
            procedure("int4eq"),
            true,
        )],
    )
    .unwrap();

    assert_ne!(invoker.snapshot_digest(), definer.snapshot_digest());
    assert!(
        definer
            .source_receipt(coordinate(), 1)
            .unwrap()
            .location()
            .security_definer()
    );
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_rejects_procedure_binding_drift() {
    let procedure_kind = procedure_kind_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
            operator("="),
            procedure("int4ne"),
            false,
        )],
    )
    .expect_err("security context must bind to the exact pg_operator.oprcode routine");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_security_definer_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_rejects_operator_binding_drift() {
    let procedure_kind = procedure_kind_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
            operator("<>"),
            procedure("int4eq"),
            false,
        )],
    )
    .expect_err("security context must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_security_definer_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_rejects_missing_evidence() {
    let procedure_kind = procedure_kind_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![],
    )
    .expect_err("every governed operator procedure needs raw prosecdef evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_security_definer_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_rejects_duplicate_coordinate() {
    let procedure_kind = procedure_kind_snapshot();
    let observation = security_definer_observation(operator("="), procedure("int4eq"), false);
    let error = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate raw prosecdef evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_security_definer_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_security_definer_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureSecurityDefinerObservation::new(
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
fn ordinary_exclude_operator_procedure_security_definer_rejects_unknown_receipt_coordinate() {
    let procedure_kind = procedure_kind_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot::new(
        &procedure_kind,
        vec![security_definer_observation(
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
fn ordinary_exclude_operator_procedure_security_definer_snapshot_is_publicly_composed() {
    assert!(
        std::mem::size_of::<IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot>() > 0
    );
}
