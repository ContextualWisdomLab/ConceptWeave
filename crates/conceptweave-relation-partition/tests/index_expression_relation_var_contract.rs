use conceptweave_observation::{
    ObservationError, QualifiedCollationName, QualifiedTypeName, RelationKind,
};
use conceptweave_relation_partition::{
    IndexExpressionRelationVarLocation, IndexExpressionRelationVarObservation,
    IndexPartitionCoordinate, RelationVarRelationRole, RelationVarReturningType,
};

fn index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "analytics",
        "accounts_2026",
        RelationKind::Table,
        "accounts_email_idx",
    )
    .unwrap()
}

fn location() -> IndexExpressionRelationVarLocation {
    IndexExpressionRelationVarLocation::Expression {
        index: index(),
        key_position: 1,
        leaf_position: 1,
    }
}

fn text_type() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "text").unwrap()
}

fn c_collation() -> QualifiedCollationName {
    QualifiedCollationName::new("pg_catalog", "C").unwrap()
}

#[test]
fn relation_var_observation_preserves_postgresql_equal_state_without_raw_rte_or_attnum() {
    let observation = IndexExpressionRelationVarObservation::new(
        location(),
        "account_email",
        text_type(),
        -1,
        Some(c_collation()),
        RelationVarRelationRole::IndexRelation,
        true,
        0,
        RelationVarReturningType::Default,
    )
    .unwrap();

    assert_eq!(observation.location(), &location());
    assert_eq!(observation.column_name(), "account_email");
    assert_eq!(observation.value_type(), &text_type());
    assert_eq!(observation.type_modifier(), -1);
    assert_eq!(observation.collation(), Some(&c_collation()));
    assert_eq!(observation.relation_role(), RelationVarRelationRole::IndexRelation);
    assert!(observation.nulling_relations_empty());
    assert_eq!(observation.levels_up(), 0);
    assert_eq!(observation.returning_type(), RelationVarReturningType::Default);
}

#[test]
fn relation_var_observation_fails_closed_on_non_index_var_context() {
    let other_role = IndexExpressionRelationVarObservation::new(
        location(),
        "account_email",
        text_type(),
        -1,
        Some(c_collation()),
        RelationVarRelationRole::Other,
        true,
        0,
        RelationVarReturningType::Default,
    )
    .expect_err("an index expression Var must resolve to the indexed relation role");
    assert_eq!(
        other_role,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_role",
        }
    );

    let nulling = IndexExpressionRelationVarObservation::new(
        location(),
        "account_email",
        text_type(),
        -1,
        Some(c_collation()),
        RelationVarRelationRole::IndexRelation,
        false,
        0,
        RelationVarReturningType::Default,
    )
    .expect_err("stored index expressions cannot discard non-empty varnullingrels");
    assert_eq!(
        nulling,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_nulling_relations",
        }
    );

    let outer_level = IndexExpressionRelationVarObservation::new(
        location(),
        "account_email",
        text_type(),
        -1,
        Some(c_collation()),
        RelationVarRelationRole::IndexRelation,
        true,
        1,
        RelationVarReturningType::Default,
    )
    .expect_err("stored index expressions cannot contain outer-query Vars");
    assert_eq!(
        outer_level,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_levels_up",
        }
    );

    let returning = IndexExpressionRelationVarObservation::new(
        location(),
        "account_email",
        text_type(),
        -1,
        Some(c_collation()),
        RelationVarRelationRole::IndexRelation,
        true,
        0,
        RelationVarReturningType::Old,
    )
    .expect_err("stored index expressions cannot contain RETURNING OLD/NEW Vars");
    assert_eq!(
        returning,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_returning_type",
        }
    );
}

#[test]
fn relation_var_location_requires_real_one_based_expression_coordinates() {
    let zero_key = IndexExpressionRelationVarLocation::expression(index(), 0, 1)
        .expect_err("expression key positions are one-based");
    assert_eq!(zero_key, ObservationError::InvalidOrdinalPosition);

    let zero_leaf = IndexExpressionRelationVarLocation::predicate(index(), 0)
        .expect_err("relation-Var leaf positions are one-based");
    assert_eq!(zero_leaf, ObservationError::InvalidOrdinalPosition);
}
