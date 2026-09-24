include!("index_exclusion_constraint_operator_procedure_access_control_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedurePlannerSupportObservation,
    IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot,
};

fn access_control_snapshot() -> IndexExclusionConstraintOperatorProcedureAccessControlSnapshot {
    let configuration = configuration_snapshot();
    IndexExclusionConstraintOperatorProcedureAccessControlSnapshot::new(
        &configuration,
        vec![access_observation(
            operator("="),
            procedure("int4eq"),
            true,
            default_execute_grants(),
        )],
    )
    .unwrap()
}

fn planner_support(name: &str) -> QualifiedProcedureSignature {
    QualifiedProcedureSignature::new(
        "pg_catalog",
        name,
        vec![QualifiedTypeName::new("pg_catalog", "internal").unwrap()],
    )
    .unwrap()
}

fn planner_support_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    planner_support: Option<QualifiedProcedureSignature>,
) -> IndexExclusionConstraintOperatorProcedurePlannerSupportObservation {
    IndexExclusionConstraintOperatorProcedurePlannerSupportObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        planner_support,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_preserves_exact_catalog_state() {
    let access_control = access_control_snapshot();
    let support = planner_support("int4eq_support");
    let snapshot = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("="),
            procedure("int4eq"),
            Some(support.clone()),
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(receipt.location().planner_support(), Some(&support));
    assert_eq!(receipt.source_id(), access_control.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure-planner-support")
    );
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_distinguishes_absence_and_identity() {
    let access_control = access_control_snapshot();
    let absent = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("="),
            procedure("int4eq"),
            None,
        )],
    )
    .unwrap();
    let first = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("="),
            procedure("int4eq"),
            Some(planner_support("int4eq_support")),
        )],
    )
    .unwrap();
    let second = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("="),
            procedure("int4eq"),
            Some(planner_support("int4eq_support_v2")),
        )],
    )
    .unwrap();

    assert_ne!(absent.snapshot_digest(), first.snapshot_digest());
    assert_ne!(first.snapshot_digest(), second.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_rejects_procedure_binding_drift() {
    let access_control = access_control_snapshot();
    let error = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("="),
            procedure("int4ne"),
            Some(planner_support("int4eq_support")),
        )],
    )
    .expect_err("planner support evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_planner_support_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_rejects_operator_binding_drift() {
    let access_control = access_control_snapshot();
    let error = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("<>"),
            procedure("int4eq"),
            Some(planner_support("int4eq_support")),
        )],
    )
    .expect_err("planner support evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_planner_support_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_rejects_missing_evidence() {
    let access_control = access_control_snapshot();
    let error = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![],
    )
    .expect_err("every governed operator function needs explicit planner-support evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_planner_support_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_rejects_duplicate_coordinate() {
    let access_control = access_control_snapshot();
    let observation = planner_support_observation(
        operator("="),
        procedure("int4eq"),
        Some(planner_support("int4eq_support")),
    );
    let error = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate planner-support observations must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_planner_support_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedurePlannerSupportObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        None,
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_planner_support_rejects_unknown_receipt_coordinate() {
    let access_control = access_control_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
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
fn ordinary_exclude_operator_procedure_planner_support_snapshot_is_publicly_composed() {
    assert!(
        std::mem::size_of::<IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot>() > 0
    );
}
