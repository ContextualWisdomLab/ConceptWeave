include!("index_exclusion_constraint_operator_procedure_cost_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformTypesObservation,
    IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot,
};

fn cost_predecessor() -> IndexExclusionConstraintOperatorProcedureCostSnapshot {
    let planner_support = planner_support_predecessor();
    IndexExclusionConstraintOperatorProcedureCostSnapshot::new(
        &planner_support,
        vec![cost_observation(operator("="), procedure("int4eq"), 1.0)],
    )
    .unwrap()
}

fn transform_types_observation(
    observed_operator: QualifiedOperatorSignature,
    observed_procedure: QualifiedProcedureSignature,
    transform_types: Option<Vec<QualifiedTypeName>>,
) -> IndexExclusionConstraintOperatorProcedureTransformTypesObservation {
    IndexExclusionConstraintOperatorProcedureTransformTypesObservation::new(
        coordinate(),
        1,
        observed_operator,
        observed_procedure,
        transform_types,
    )
    .unwrap()
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_preserve_exact_pg_proc_protrftypes() {
    let predecessor = cost_predecessor();
    let transformed = QualifiedTypeName::new("public", "custom_payload").unwrap();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec![transformed.clone()]),
        )],
    )
    .unwrap();
    let receipt = snapshot.source_receipt(coordinate(), 1).unwrap();

    assert_eq!(receipt.location().operator(), &operator("="));
    assert_eq!(receipt.location().procedure(), &procedure("int4eq"));
    assert_eq!(
        receipt.location().transform_types(),
        Some(&[transformed][..])
    );
    assert_eq!(receipt.source_id(), predecessor.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/1/procedure-transform-types")
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_distinguish_null_and_transform_selection() {
    let predecessor = cost_predecessor();
    let none = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            None,
        )],
    )
    .unwrap();
    let one = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec![
                QualifiedTypeName::new("public", "custom_payload").unwrap(),
            ]),
        )],
    )
    .unwrap();

    assert_ne!(none.snapshot_digest(), one.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_are_set_canonical() {
    let predecessor = cost_predecessor();
    let left = QualifiedTypeName::new("public", "alpha_payload").unwrap();
    let right = QualifiedTypeName::new("public", "beta_payload").unwrap();
    let forward = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec![left.clone(), right.clone()]),
        )],
    )
    .unwrap();
    let reverse = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4eq"),
            Some(vec![right, left]),
        )],
    )
    .unwrap();

    assert_eq!(forward.snapshot_digest(), reverse.snapshot_digest());
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_explicit_empty_array() {
    let error = IndexExclusionConstraintOperatorProcedureTransformTypesObservation::new(
        coordinate(),
        1,
        operator("="),
        procedure("int4eq"),
        Some(vec![]),
    )
    .expect_err("PostgreSQL documents NULL rather than an empty protrftypes array when none apply");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_types_empty",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_duplicate_type() {
    let duplicated = QualifiedTypeName::new("public", "custom_payload").unwrap();
    let error = IndexExclusionConstraintOperatorProcedureTransformTypesObservation::new(
        coordinate(),
        1,
        operator("="),
        procedure("int4eq"),
        Some(vec![duplicated.clone(), duplicated]),
    )
    .expect_err("duplicate protrftypes entries must not be silently collapsed");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_types_duplicate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_procedure_binding_drift() {
    let predecessor = cost_predecessor();
    let error = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("="),
            procedure("int4ne"),
            None,
        )],
    )
    .expect_err("transform evidence must bind to the exact pg_operator.oprcode function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_types_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_operator_binding_drift() {
    let predecessor = cost_predecessor();
    let error = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
            operator("<>"),
            procedure("int4eq"),
            None,
        )],
    )
    .expect_err("transform evidence must stay on the exact governed conexclop position");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_types_binding",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_missing_evidence() {
    let predecessor = cost_predecessor();
    let error =
        IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(&predecessor, vec![])
            .expect_err("every governed operator function needs explicit protrftypes evidence");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_types_completeness",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_duplicate_coordinate() {
    let predecessor = cost_predecessor();
    let observation = transform_types_observation(operator("="), procedure("int4eq"), None);
    let error = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate transform observations must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_types_coordinate",
    );
}

#[test]
fn ordinary_exclude_operator_procedure_transform_types_reject_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformTypesObservation::new(
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
fn ordinary_exclude_operator_procedure_transform_types_reject_unknown_receipt_coordinate() {
    let predecessor = cost_predecessor();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot::new(
        &predecessor,
        vec![transform_types_observation(
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
fn ordinary_exclude_operator_procedure_transform_types_snapshot_is_publicly_composed() {
    assert!(
        std::mem::size_of::<IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot>() > 0
    );
}
