use conceptweave_observation::{QualifiedTypeName, RelationKind};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial,
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial,
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant,
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity,
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation,
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation,
};

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn quoted_identifier_collision_pair() -> (QualifiedTypeName, QualifiedTypeName) {
    (
        QualifiedTypeName::new("payload.domain", "json").unwrap(),
        QualifiedTypeName::new("payload", "domain.json").unwrap(),
    )
}

fn owner_location(transform_type: QualifiedTypeName) -> String {
    let owner = IndexExclusionConstraintOperatorProcedureTransformConverterOwnerIdentity::new(
        16_384,
        "transform_runtime",
    )
    .unwrap();
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation::new(
        coordinate(),
        1,
        transform_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        owner,
    )
    .unwrap()
    .canonical_location()
}

fn access_control_location(transform_type: QualifiedTypeName) -> String {
    let access_control =
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial::new(
            false,
            vec![
                IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant::public(
                    "transform_runtime",
                    false,
                )
                .unwrap(),
            ],
        )
        .unwrap();
    IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation::new(
        coordinate(),
        1,
        transform_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        access_control,
    )
    .unwrap()
    .canonical_location()
}

fn configuration_location(transform_type: QualifiedTypeName) -> String {
    let configuration =
        IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationMaterial::from_proconfig(
            None,
        );
    IndexExclusionConstraintOperatorProcedureTransformConverterConfigurationObservation::new(
        coordinate(),
        1,
        transform_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        configuration,
    )
    .unwrap()
    .canonical_location()
}

fn security_definer_location(transform_type: QualifiedTypeName) -> String {
    IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerObservation::new(
        coordinate(),
        1,
        transform_type,
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
        "public",
        "payload_from_sql",
        false,
    )
    .unwrap()
    .canonical_location()
}

#[test]
fn transform_converter_predecessor_locations_are_collision_safe_for_quoted_type_names() {
    let (dotted_schema, dotted_type) = quoted_identifier_collision_pair();
    assert_ne!(
        owner_location(dotted_schema.clone()),
        owner_location(dotted_type.clone()),
        "owner provenance must preserve the quoted schema/type boundary",
    );
    assert_ne!(
        access_control_location(dotted_schema.clone()),
        access_control_location(dotted_type.clone()),
        "access-control provenance must preserve the quoted schema/type boundary",
    );
    assert_ne!(
        configuration_location(dotted_schema.clone()),
        configuration_location(dotted_type.clone()),
        "configuration provenance must preserve the quoted schema/type boundary",
    );
    assert_ne!(
        security_definer_location(dotted_schema),
        security_definer_location(dotted_type),
        "security-definer provenance must preserve the quoted schema/type boundary",
    );
}
