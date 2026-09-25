include!("index_exclusion_constraint_operator_procedure_planner_support_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureCostObservation,
    IndexExclusionConstraintOperatorProcedureCostSnapshot,
};

fn planner_support_predecessor() -> IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot
{
    let access_control = access_control_snapshot();
    IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot::new(
        &access_control,
        vec![planner_support_observation(
            operator("="),
            procedure("int4eq"),
            None,
        )],
    )
    .unwrap()
}

fn cost_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    execution_cost: f32,
) -> IndexExclusionConstraintOperatorProcedureCostObservation {
    IndexExclusionConstraintOperatorProcedureCostObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        execution_cost,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_cost_preserves_exact_pg_proc_procost() {
    let predecessor = planner_support_predecessor();
    let snapshot = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![cost_observation(operator("="), procedure("int4eq"), 1.0)],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(
        receipt.location().execution_cost().to_bits(),
        1.0_f32.to_bits()
    );
    assert_eq!(receipt.source_id(), predecessor.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure-cost")
    );
}

#[test]
fn ordinary_exclude_operator_procedure_cost_distinguishes_planner_cost() {
    let predecessor = planner_support_predecessor();
    let cheap = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![cost_observation(operator("="), procedure("int4eq"), 1.0)],
    )
    .unwrap();
    let expensive = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![cost_observation(operator("="), procedure("int4eq"), 100.0)],
    )
    .unwrap();

    assert_ne!(cheap.snapshot_digest(), expensive.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_non_positive_or_non_finite_values() {
    for execution_cost in [0.0, -1.0, f32::NAN, f32::INFINITY, f32::NEG_INFINITY] {
        let error = IndexExclusionConstraintOperatorProcedureCostObservation::new(
            coordinate(),
            1,
            operator("="),
            procedure("int4eq"),
            execution_cost,
        )
        .expect_err("pg_proc.procost evidence must be a positive finite float4 value");
        assert_field(error, "index_exclusion_constraint_operator_procedure_cost");
    }
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_procedure_binding_drift() {
    let predecessor = planner_support_predecessor();
    let error = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![cost_observation(operator("="), procedure("int4ne"), 1.0)],
    )
    .expect_err("cost evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_cost_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_operator_binding_drift() {
    let predecessor = planner_support_predecessor();
    let error = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![cost_observation(operator("<>"), procedure("int4eq"), 1.0)],
    )
    .expect_err("cost evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_cost_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_missing_evidence() {
    let predecessor = planner_support_predecessor();
    let error = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(&predecessor, vec![])
        .expect_err("every governed operator function needs explicit procost evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_cost_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_duplicate_coordinate() {
    let predecessor = planner_support_predecessor();
    let observation = cost_observation(operator("="), procedure("int4eq"), 1.0);
    let error = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate cost observations must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_cost_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureCostObservation::new(
        coordinate(),
        0,
        operator("="),
        procedure("int4eq"),
        1.0,
    )
    .expect_err("operator procedure positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_operator_procedure_cost_rejects_unknown_receipt_coordinate() {
    let predecessor = planner_support_predecessor();
    let snapshot = IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &predecessor,
        vec![cost_observation(operator("="), procedure("int4eq"), 1.0)],
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
fn ordinary_exclude_operator_procedure_cost_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<IndexExclusionConstraintOperatorProcedureCostSnapshot>() > 0);
}
