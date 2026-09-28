include!(
    "index_exclusion_constraint_operator_procedure_transform_converter_planner_support_contract.rs"
);

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot,
};

fn converter_planner_support_snapshot()
-> IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot {
    let predecessor = converter_parallel_safety_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot::new(
        &predecessor,
        complete_converter_planner_support_observations(),
    )
    .unwrap()
}

fn converter_cost_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    execution_cost: f32,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        execution_cost,
    )
    .unwrap()
}

fn complete_converter_cost_observations()
-> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation> {
    vec![
        converter_cost_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            1.0,
        ),
        converter_cost_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            100.0,
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_cost_preserves_exact_procost_float4() {
    let predecessor = converter_planner_support_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        complete_converter_cost_observations(),
    )
    .unwrap();
    let receipt = snapshot
        .source_receipt(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .unwrap();

    assert_eq!(
        receipt.location().execution_cost().to_bits(),
        1.0_f32.to_bits()
    );
    assert_eq!(receipt.source_id(), predecessor.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(
        receipt.connection_policy_binding(),
        predecessor.connection_policy_binding()
    );
    assert_eq!(
        receipt.extractor_revision(),
        predecessor.extractor_revision()
    );
    assert_eq!(receipt.observed_at_utc(), predecessor.observed_at_utc());
}

#[test]
fn ordinary_exclude_transform_converter_cost_distinguishes_catalog_values() {
    let predecessor = converter_planner_support_snapshot();
    let baseline = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        complete_converter_cost_observations(),
    )
    .unwrap();
    let mut changed_observations = complete_converter_cost_observations();
    changed_observations[0] = converter_cost_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        1.5,
    );
    let changed = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        changed_observations,
    )
    .unwrap();

    assert_ne!(baseline.snapshot_digest(), changed.snapshot_digest());
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_nonpositive_or_nonfinite_values() {
    for invalid_cost in [
        0.0_f32,
        -0.0_f32,
        -1.0_f32,
        f32::NAN,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ] {
        let error =
            IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
                coordinate(),
                1,
                custom_payload_type(),
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                "public",
                "payload_from_sql",
                invalid_cost,
            )
            .expect_err("PostgreSQL procost must be positive and finite");
        assert_field(
            error,
            "index_exclusion_constraint_operator_procedure_transform_converter_cost",
        );
    }
}

#[test]
fn ordinary_exclude_transform_converter_cost_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
            coordinate(),
            1,
            dotted_schema,
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            1.0,
        )
        .unwrap()
        .canonical_location();
    let type_location =
        IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
            coordinate(),
            1,
            dotted_type,
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            1.0,
        )
        .unwrap()
        .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_missing_direction() {
    let predecessor = converter_planner_support_snapshot();
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        vec![converter_cost_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            1.0,
        )],
    )
    .expect_err(
        "every planner-support predecessor converter direction needs explicit procost evidence",
    );
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_cost_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_extra_coordinate() {
    let predecessor = converter_planner_support_snapshot();
    let mut observations = complete_converter_cost_observations();
    observations.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            1.0,
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("cost evidence cannot introduce a converter coordinate absent from predecessor");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_cost_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_binding_drift() {
    let predecessor = converter_planner_support_snapshot();
    let mut observations = complete_converter_cost_observations();
    observations[0] = converter_cost_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        1.0,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        observations,
    )
    .expect_err("cost evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_cost_binding",
    );
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_duplicate_coordinate() {
    let predecessor = converter_planner_support_snapshot();
    let observation = converter_cost_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        1.0,
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter cost evidence must not collapse");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_cost_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_blank_converter_identifiers() {
    let blank_schema =
        IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "",
            "payload_from_sql",
            1.0,
        )
        .expect_err("converter schema is part of the exact function binding");
    assert_field(
        blank_schema,
        "index_exclusion_constraint_operator_procedure_transform_converter_cost_function_schema",
    );

    let blank_name =
        IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "",
            1.0,
        )
        .expect_err("converter function name is part of the exact function binding");
    assert_field(
        blank_name,
        "index_exclusion_constraint_operator_procedure_transform_converter_cost_function_name",
    );
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_zero_position() {
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        1.0,
    )
    .expect_err("converter positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn ordinary_exclude_transform_converter_cost_rejects_unknown_receipt_coordinate() {
    let predecessor = converter_planner_support_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        complete_converter_cost_observations(),
    )
    .unwrap();
    let error = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err(
            "receipt lookup must remain exact-coordinate, position, type, and direction bound",
        );
    assert!(matches!(
        error,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn ordinary_exclude_transform_converter_cost_snapshot_is_publicly_composed() {
    assert!(
        std::mem::size_of::<IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot>(
        ) > 0
    );
}
