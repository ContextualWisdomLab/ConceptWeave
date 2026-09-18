include!("index_exclusion_constraint_operator_procedure_transform_converter_cost_contract.rs");

use conceptweave_relation_partition::{
    IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot,
};

fn converter_cost_snapshot(
) -> IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot {
    let predecessor = converter_planner_support_snapshot();
    IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot::new(
        &predecessor,
        complete_converter_cost_observations(),
    )
    .unwrap()
}

fn converter_kind_observation(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    function_name: &str,
    procedure_kind: char,
) -> IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation {
    IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        direction,
        "public",
        function_name,
        procedure_kind,
    )
    .unwrap()
}

fn complete_converter_kind_observations(
) -> Vec<IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation> {
    vec![
        converter_kind_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            'f',
        ),
        converter_kind_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
            "payload_to_sql",
            'f',
        ),
    ]
}

#[test]
fn ordinary_exclude_transform_converter_kind_preserves_required_normal_function_kind() {
    let predecessor = converter_cost_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        complete_converter_kind_observations(),
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

    assert_eq!(receipt.location().procedure_kind(), 'f');
    assert_eq!(receipt.source_id(), predecessor.source_connection_key());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_ne!(snapshot.snapshot_digest(), predecessor.snapshot_digest());
    assert_eq!(
        receipt.connection_policy_binding(),
        predecessor.connection_policy_binding()
    );
    assert_eq!(receipt.extractor_revision(), predecessor.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), predecessor.observed_at_utc());
}

#[test]
fn ordinary_exclude_transform_converter_kind_rejects_non_function_prokind() {
    for invalid_kind in ['p', 'a', 'w', 'x'] {
        let error = IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
            coordinate(),
            1,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            invalid_kind,
        )
        .expect_err("PostgreSQL transform converters must resolve to prokind='f'");
        assert_field(
            error,
            "index_exclusion_constraint_operator_procedure_transform_converter_kind",
        );
    }
}

#[test]
fn ordinary_exclude_transform_converter_kind_rejects_missing_or_extra_direction() {
    let predecessor = converter_cost_snapshot();
    let missing = IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        vec![converter_kind_observation(
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "payload_from_sql",
            'f',
        )],
    )
    .expect_err("every cost predecessor converter direction needs explicit prokind evidence");
    assert_field(
        missing,
        "index_exclusion_constraint_operator_procedure_transform_converter_kind_completeness",
    );

    let mut extra = complete_converter_kind_observations();
    extra.push(
        IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
            "public",
            "payload_from_sql",
            'f',
        )
        .unwrap(),
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        extra,
    )
    .expect_err("kind evidence cannot introduce a converter coordinate absent from predecessor");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_kind_completeness",
    );
}

#[test]
fn ordinary_exclude_transform_converter_kind_rejects_binding_drift_and_duplicates() {
    let predecessor = converter_cost_snapshot();
    let mut drift = complete_converter_kind_observations();
    drift[0] = converter_kind_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "different_from_sql",
        'f',
    );
    let error = IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        drift,
    )
    .expect_err("kind evidence must remain bound to the exact converter function");
    assert_field(
        error,
        "index_exclusion_constraint_operator_procedure_transform_converter_kind_binding",
    );

    let observation = converter_kind_observation(
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "payload_from_sql",
        'f',
    );
    let duplicate = IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        vec![observation.clone(), observation],
    )
    .expect_err("duplicate converter kind evidence must not collapse");
    assert_field(
        duplicate,
        "index_exclusion_constraint_operator_procedure_transform_converter_kind_coordinate",
    );
}

#[test]
fn ordinary_exclude_transform_converter_kind_location_is_collision_safe_for_quoted_type_names() {
    let dotted_schema =
        conceptweave_observation::QualifiedTypeName::new("payload.domain", "json").unwrap();
    let dotted_type =
        conceptweave_observation::QualifiedTypeName::new("payload", "domain.json").unwrap();
    let schema_location = IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
        coordinate(),
        1,
        dotted_schema,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'f',
    )
    .unwrap()
    .canonical_location();
    let type_location = IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
        coordinate(),
        1,
        dotted_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'f',
    )
    .unwrap()
    .canonical_location();

    assert_ne!(schema_location, type_location);
}

#[test]
fn ordinary_exclude_transform_converter_kind_rejects_blank_identifiers_zero_position_and_unknown_receipt() {
    let blank_schema = IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        " ",
        "payload_from_sql",
        'f',
    )
    .expect_err("converter schema is part of the exact function binding");
    assert_field(
        blank_schema,
        "index_exclusion_constraint_operator_procedure_transform_converter_kind_function_schema",
    );

    let blank_name = IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
        coordinate(),
        1,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "\t",
        'f',
    )
    .expect_err("converter function name is part of the exact function binding");
    assert_field(
        blank_name,
        "index_exclusion_constraint_operator_procedure_transform_converter_kind_function_name",
    );

    let zero_position = IndexExclusionConstraintOperatorProcedureTransformConverterKindObservation::new(
        coordinate(),
        0,
        custom_payload_type(),
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        'f',
    )
    .expect_err("converter positions are one-based");
    assert_eq!(zero_position, ObservationError::InvalidOrdinalPosition);

    let predecessor = converter_cost_snapshot();
    let snapshot = IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot::new(
        &predecessor,
        complete_converter_kind_observations(),
    )
    .unwrap();
    let unknown = snapshot
        .source_receipt(
            coordinate(),
            2,
            custom_payload_type(),
            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        )
        .expect_err("receipt lookup must remain exact-coordinate, position, type, and direction bound");
    assert!(matches!(
        unknown,
        ObservationError::UnknownObservationLocation { .. }
    ));
}

#[test]
fn ordinary_exclude_transform_converter_kind_snapshot_is_publicly_composed() {
    assert!(std::mem::size_of::<
        IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot,
    >() > 0);
}
