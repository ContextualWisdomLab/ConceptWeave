use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation,
    IndexCatalogFlags, IndexKeySemantics, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedCollationName, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    ColumnTypeModifierObservation, IndexExclusionSemanticsSnapshot,
    IndexExpressionRelationVarLocation, IndexExpressionRelationVarObservation,
    IndexExpressionRelationVarSnapshot, IndexExpressionSemanticsObservation,
    IndexExpressionSemanticsSnapshot, IndexKeyOperatorFamilyObservation,
    IndexOperatorFamilySnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexPredicateSemanticsObservation, IndexRelationKind,
    PartitionParentRelationCoordinate, QualifiedFunctionSignature, QualifiedOperatorFamilyName,
    QualifiedOperatorSignature, RelationPartitionObservation, RelationPartitionSnapshot,
    RelationPartitionTypeModifierSnapshot, RelationVarRelationRole, RelationVarReturningType,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_relation_var";

struct Registry<'a> {
    key: &'a str,
    policy: &'a str,
}

impl SourceConnectionRegistry for Registry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.key
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == self.key).then(|| self.policy.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == self.key
            && source_connection.connection_policy_binding() == self.policy
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == self.key
            && source_connection.connection_policy_binding() == self.policy
            && resource_envelope.request_budget().max_schema_count() <= 1
            && resource_envelope.request_budget().max_schema_bytes() <= 256
            && resource_envelope.limits().operation_timeout_ms() <= 1_000
            && resource_envelope.limits().statement_timeout_ms() <= 1_000
            && resource_envelope.limits().max_rows() <= 10
            && resource_envelope.limits().max_bytes() <= 1_024
            && resource_envelope.limits().max_concurrent_queries() <= 1
    }
}

fn authorized_source() -> AuthorizedObservationRequest {
    authorized_source_with_binding("warehouse_primary", POLICY_BINDING)
}

fn authorized_source_with_binding(key: &str, policy: &str) -> AuthorizedObservationRequest {
    ObservationRequest::new(
        key,
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry { key, policy })
    .unwrap()
}

fn index(name: &str) -> IndexObservation {
    IndexObservation::new(
        name,
        false,
        Some(false),
        vec![
            IndexAttributeObservation::expression(
                1,
                IndexAttributeKind::Key,
                "lower(account_email)",
            )
            .unwrap(),
        ],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            Some(QualifiedCollationName::new("pg_catalog", "default").unwrap()),
            QualifiedOperatorClassName::new("pg_catalog", "text_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, false,
    ))
    .unwrap()
    .with_predicate("account_id > 0")
    .with_valid(true)
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    reverse: bool,
) -> RelationObservation {
    let account_id = ColumnObservationV3::new(
        "account_id",
        if reverse { 2 } else { 1 },
        "integer",
        QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
        false,
        None,
    )
    .unwrap();
    let account_email = ColumnObservationV3::new(
        "account_email",
        if reverse { 1 } else { 2 },
        "text",
        QualifiedTypeName::new("pg_catalog", "text").unwrap(),
        false,
        None,
    )
    .unwrap();
    let columns = if reverse {
        vec![account_email, account_id]
    } else {
        vec![account_id, account_email]
    };
    RelationObservation::new("public", name, kind, columns)
        .unwrap()
        .with_indexes(vec![index(index_name)])
        .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts",
        RelationKind::PartitionedTable,
        "accounts_email_idx",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts_2026",
        RelationKind::Table,
        "accounts_2026_email_idx",
    )
    .unwrap()
}

fn base() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-var-v1",
        "2026-09-15T00:30:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                "accounts_email_idx",
                false,
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                "accounts_2026_email_idx",
                true,
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_column_collations(vec![
        ColumnCollationObservation::uncollatable(
            "public",
            "accounts",
            RelationKind::PartitionedTable,
            "account_id",
        )
        .unwrap(),
        ColumnCollationObservation::collatable(
            "public",
            "accounts",
            RelationKind::PartitionedTable,
            "account_email",
            QualifiedCollationName::new("pg_catalog", "default").unwrap(),
            true,
        )
        .unwrap(),
        ColumnCollationObservation::uncollatable(
            "public",
            "accounts_2026",
            RelationKind::Table,
            "account_id",
        )
        .unwrap(),
        ColumnCollationObservation::collatable(
            "public",
            "accounts_2026",
            RelationKind::Table,
            "account_email",
            QualifiedCollationName::new("pg_catalog", "default").unwrap(),
            true,
        )
        .unwrap(),
    ])
    .unwrap()
}

fn field(name: &str, value: CanonicalExpressionValue) -> CanonicalExpressionField {
    CanonicalExpressionField::new(name, value).unwrap()
}

fn expression_root() -> CanonicalExpression {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    CanonicalExpression::node(
        "FuncExpr",
        vec![
            field(
                "function",
                CanonicalExpressionValue::Function(
                    QualifiedFunctionSignature::new(
                        "pg_catalog",
                        "lower",
                        vec![text.clone()],
                        text,
                    )
                    .unwrap(),
                ),
            ),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![
                    CanonicalExpression::column("account_email").unwrap(),
                ]),
            ),
        ],
    )
    .unwrap()
}

fn predicate_root() -> CanonicalExpression {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    CanonicalExpression::node(
        "OpExpr",
        vec![
            field(
                "operator",
                CanonicalExpressionValue::Operator(
                    QualifiedOperatorSignature::new("pg_catalog", ">", int4.clone(), int4.clone())
                        .unwrap(),
                ),
            ),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![
                    CanonicalExpression::column("account_id").unwrap(),
                    CanonicalExpression::node(
                        "Const",
                        vec![
                            field("type", CanonicalExpressionValue::Type(int4)),
                            field("value", CanonicalExpressionValue::Text("0".to_owned())),
                        ],
                    )
                    .unwrap(),
                ]),
            ),
        ],
    )
    .unwrap()
}

struct Stack {
    base: PostgresSchemaSnapshotV3,
    relations: RelationPartitionSnapshot,
    indexes: IndexPartitionSnapshot,
    families: IndexOperatorFamilySnapshot,
    exclusions: IndexExclusionSemanticsSnapshot,
    expressions: IndexExpressionSemanticsSnapshot,
    type_modifiers: RelationPartitionTypeModifierSnapshot,
}

fn stack() -> Stack {
    stack_from_base(base())
}

fn stack_from_base(base: PostgresSchemaSnapshotV3) -> Stack {
    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "accounts_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "accounts").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(
                parent_index(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
            IndexPartitionObservation::partition(
                child_index(),
                IndexRelationKind::Index,
                parent_index(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let families = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexKeyOperatorFamilyObservation::new(
                parent_index(),
                1,
                QualifiedOperatorClassName::new("pg_catalog", "text_ops").unwrap(),
                QualifiedOperatorFamilyName::new("btree", "pg_catalog", "text_ops").unwrap(),
            )
            .unwrap(),
            IndexKeyOperatorFamilyObservation::new(
                child_index(),
                1,
                QualifiedOperatorClassName::new("pg_catalog", "text_ops").unwrap(),
                QualifiedOperatorFamilyName::new("btree", "pg_catalog", "text_ops").unwrap(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let exclusions =
        IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, vec![])
            .unwrap();
    let expressions = IndexExpressionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        &exclusions,
        vec![
            IndexExpressionSemanticsObservation::new(parent_index(), 1, expression_root()).unwrap(),
            IndexExpressionSemanticsObservation::new(child_index(), 1, expression_root()).unwrap(),
        ],
        vec![
            IndexPredicateSemanticsObservation::new(parent_index(), predicate_root()).unwrap(),
            IndexPredicateSemanticsObservation::new(child_index(), predicate_root()).unwrap(),
        ],
    )
    .unwrap();
    let type_modifiers = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relations,
        vec![
            ColumnTypeModifierObservation::new(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
                "account_id",
                -1,
            )
            .unwrap(),
            ColumnTypeModifierObservation::new(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
                "account_email",
                -1,
            )
            .unwrap(),
            ColumnTypeModifierObservation::new(
                "public",
                "accounts_2026",
                RelationKind::Table,
                "account_id",
                -1,
            )
            .unwrap(),
            ColumnTypeModifierObservation::new(
                "public",
                "accounts_2026",
                RelationKind::Table,
                "account_email",
                -1,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    Stack {
        base,
        relations,
        indexes,
        families,
        exclusions,
        expressions,
        type_modifiers,
    }
}

fn var(
    location: IndexExpressionRelationVarLocation,
    column_name: &str,
    value_type: &str,
    collation: Option<QualifiedCollationName>,
) -> IndexExpressionRelationVarObservation {
    IndexExpressionRelationVarObservation::new(
        location,
        column_name,
        QualifiedTypeName::new("pg_catalog", value_type).unwrap(),
        -1,
        collation,
        RelationVarRelationRole::IndexRelation,
        true,
        0,
        RelationVarReturningType::Default,
    )
    .unwrap()
}

fn complete_vars() -> Vec<IndexExpressionRelationVarObservation> {
    let default_collation = || QualifiedCollationName::new("pg_catalog", "default").unwrap();
    vec![
        var(
            IndexExpressionRelationVarLocation::expression(parent_index(), 1, 1).unwrap(),
            "account_email",
            "text",
            Some(default_collation()),
        ),
        var(
            IndexExpressionRelationVarLocation::predicate(parent_index(), 1).unwrap(),
            "account_id",
            "int4",
            None,
        ),
        var(
            IndexExpressionRelationVarLocation::expression(child_index(), 1, 1).unwrap(),
            "account_email",
            "text",
            Some(default_collation()),
        ),
        var(
            IndexExpressionRelationVarLocation::predicate(child_index(), 1).unwrap(),
            "account_id",
            "int4",
            None,
        ),
    ]
}

fn relation_var_snapshot(
    observations: Vec<IndexExpressionRelationVarObservation>,
) -> Result<IndexExpressionRelationVarSnapshot, ObservationError> {
    let stack = stack();
    IndexExpressionRelationVarSnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.families,
        &stack.exclusions,
        &stack.expressions,
        &stack.type_modifiers,
        observations,
    )
}

#[test]
fn relation_var_successor_is_reachable_without_a_v2_snapshot() {
    let snapshot = relation_var_snapshot(complete_vars())
        .expect("complete relation-Var evidence must branch from exact expression semantics");

    assert_eq!(snapshot.observations().len(), 4);
    assert!(snapshot.snapshot_digest().starts_with("sha256:"));
}

#[test]
fn every_canonical_column_leaf_requires_exact_relation_var_evidence() {
    let mut observations = complete_vars();
    observations.pop();
    let error = relation_var_snapshot(observations)
        .expect_err("omitting one canonical relation-Var leaf must fail closed");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_completeness",
        }
    );
}

#[test]
fn relation_var_type_and_collation_must_match_source_authoritative_column_families() {
    let mut wrong_type = complete_vars();
    wrong_type[0] = var(
        IndexExpressionRelationVarLocation::expression(parent_index(), 1, 1).unwrap(),
        "account_email",
        "varchar",
        Some(QualifiedCollationName::new("pg_catalog", "default").unwrap()),
    );
    let type_error = relation_var_snapshot(wrong_type)
        .expect_err("vartype cannot diverge from the bounded column type binding");
    assert_eq!(
        type_error,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_value_type",
        }
    );

    let mut wrong_collation = complete_vars();
    wrong_collation[0] = var(
        IndexExpressionRelationVarLocation::expression(parent_index(), 1, 1).unwrap(),
        "account_email",
        "text",
        Some(QualifiedCollationName::new("pg_catalog", "C").unwrap()),
    );
    let collation_error = relation_var_snapshot(wrong_collation)
        .expect_err("varcollid cannot diverge from source-authoritative attcollation");
    assert_eq!(
        collation_error,
        ObservationError::InvalidObservationField {
            field: "index_expression_relation_var_collation",
        }
    );
}

#[test]
fn exact_relation_var_receipt_is_bound_to_successor_digest() {
    let snapshot = relation_var_snapshot(complete_vars()).unwrap();
    let location = IndexExpressionRelationVarLocation::predicate(child_index(), 1).unwrap();
    let receipt = snapshot.source_receipt(location.clone()).unwrap();

    assert_eq!(receipt.location(), &location);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.source_id(), snapshot.source_connection_key());
    assert_eq!(receipt.location().leaf_position(), 1);
    assert_eq!(receipt.location().key_position(), None);
    let expression_location =
        IndexExpressionRelationVarLocation::expression(child_index(), 1, 1).unwrap();
    let expression_receipt = snapshot
        .source_receipt(expression_location.clone())
        .unwrap();
    assert_eq!(expression_receipt.location(), &expression_location);
    assert_eq!(expression_receipt.location().leaf_position(), 1);
    assert_eq!(expression_receipt.location().key_position(), Some(1));
    assert_ne!(
        expression_receipt.location().canonical_location(),
        receipt.location().canonical_location()
    );
}

#[test]
fn relation_var_coordinates_reject_duplicate_extra_and_mismatched_column_evidence() {
    let complete = complete_vars();
    for (position, original) in complete.iter().enumerate() {
        let mut duplicate = complete.clone();
        duplicate.push(original.clone());
        let mut extra = complete.clone();
        let absent = match original.location() {
            IndexExpressionRelationVarLocation::Expression {
                index,
                key_position,
                ..
            } => IndexExpressionRelationVarLocation::expression(index.clone(), *key_position, 2)
                .unwrap(),
            IndexExpressionRelationVarLocation::Predicate { index, .. } => {
                IndexExpressionRelationVarLocation::predicate(index.clone(), 2).unwrap()
            }
        };
        extra.push(var(
            absent.clone(),
            original.column_name(),
            original.value_type().type_name(),
            original.collation().cloned(),
        ));
        let mut wrong_column = complete.clone();
        wrong_column[position] = var(
            original.location().clone(),
            "missing_column",
            original.value_type().type_name(),
            original.collation().cloned(),
        );
        let mut wrong_modifier = complete.clone();
        wrong_modifier[position] = IndexExpressionRelationVarObservation::new(
            original.location().clone(),
            original.column_name(),
            original.value_type().clone(),
            4,
            original.collation().cloned(),
            RelationVarRelationRole::IndexRelation,
            true,
            0,
            RelationVarReturningType::Default,
        )
        .unwrap();
        for (observations, field) in [
            (duplicate, "index_expression_relation_var_coordinate"),
            (extra, "index_expression_relation_var_completeness"),
            (wrong_column, "index_expression_relation_var_column"),
            (
                wrong_modifier,
                "index_expression_relation_var_type_modifier",
            ),
        ] {
            assert_eq!(
                relation_var_snapshot(observations).unwrap_err(),
                ObservationError::InvalidObservationField { field }
            );
        }
        let snapshot = relation_var_snapshot(complete.clone()).unwrap();
        assert_eq!(
            snapshot.source_receipt(absent.clone()).unwrap_err(),
            ObservationError::UnknownObservationLocation {
                location: absent.canonical_location()
            }
        );
    }
    let snapshot = relation_var_snapshot(complete.clone()).unwrap();
    let mut reversed = complete;
    reversed.reverse();
    assert_eq!(snapshot, relation_var_snapshot(reversed).unwrap());
    for observation in snapshot.observations() {
        let receipt = snapshot
            .source_receipt(observation.location().clone())
            .unwrap();
        assert_eq!(
            receipt.connection_policy_binding(),
            snapshot.connection_policy_binding()
        );
        assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());
        assert_eq!(receipt.observed_at_utc(), snapshot.observed_at_utc());
    }
}

#[test]
fn relation_var_predecessors_reject_each_mixed_provenance_dimension() {
    let original = stack();
    let baseline = relation_var_snapshot(complete_vars()).unwrap();
    for (key, policy, revision, time) in [
        (
            "warehouse_secondary",
            POLICY_BINDING,
            "extractor-relation-var-v1",
            "2026-09-15T00:30:00Z",
        ),
        (
            "warehouse_primary",
            "fixture_policy_revision_relation_var_v2",
            "extractor-relation-var-v1",
            "2026-09-15T00:30:00Z",
        ),
        (
            "warehouse_primary",
            POLICY_BINDING,
            "extractor-relation-var-v2",
            "2026-09-15T00:30:00Z",
        ),
        (
            "warehouse_primary",
            POLICY_BINDING,
            "extractor-relation-var-v1",
            "2026-09-15T00:30:01Z",
        ),
    ] {
        let changed_base = PostgresSchemaSnapshotV3::new(
            &authorized_source_with_binding(key, policy),
            revision,
            time,
            original.base.relations().to_vec(),
            vec![],
            vec![],
        )
        .unwrap()
        .with_observed_column_collations(original.base.column_collations().unwrap().to_vec())
        .unwrap();
        let changed = stack_from_base(changed_base);
        assert_eq!(
            IndexPartitionSnapshot::new(
                &changed.base,
                &original.relations,
                changed.indexes.observations().to_vec(),
            )
            .unwrap_err(),
            ObservationError::InvalidObservationField {
                field: "index_partition_relation_snapshot_binding",
            }
        );
        assert_eq!(
            IndexOperatorFamilySnapshot::new(
                &changed.base,
                &changed.relations,
                &original.indexes,
                changed.families.observations().to_vec(),
            )
            .unwrap_err(),
            ObservationError::InvalidObservationField {
                field: "index_operator_family_predecessor_binding",
            }
        );
        assert_eq!(
            IndexExclusionSemanticsSnapshot::new(
                &changed.base,
                &changed.relations,
                &changed.indexes,
                &original.families,
                changed.exclusions.observations().to_vec(),
            )
            .unwrap_err(),
            ObservationError::InvalidObservationField {
                field: "index_exclusion_semantics_predecessor_binding",
            }
        );
        assert_eq!(
            IndexExpressionSemanticsSnapshot::new(
                &changed.base,
                &changed.relations,
                &changed.indexes,
                &changed.families,
                &original.exclusions,
                changed.expressions.expression_observations().to_vec(),
                changed.expressions.predicate_observations().to_vec(),
            )
            .unwrap_err(),
            ObservationError::InvalidObservationField {
                field: "index_expression_semantics_predecessor_binding",
            }
        );
        let fresh = IndexExpressionRelationVarSnapshot::new(
            &changed.base,
            &changed.relations,
            &changed.indexes,
            &changed.families,
            &changed.exclusions,
            &changed.expressions,
            &changed.type_modifiers,
            complete_vars(),
        )
        .unwrap();
        assert_eq!(fresh.snapshot_digest(), baseline.snapshot_digest());
        for observation in fresh.observations() {
            let receipt = fresh
                .source_receipt(observation.location().clone())
                .unwrap();
            assert_eq!(receipt.source_id(), key);
            assert_eq!(receipt.connection_policy_binding(), policy);
            assert_eq!(receipt.extractor_revision(), revision);
            assert_eq!(receipt.observed_at_utc(), time);
        }
        assert_eq!(fresh.source_connection_key(), key);
        assert_eq!(fresh.connection_policy_binding(), policy);
        assert_eq!(fresh.extractor_revision(), revision);
        assert_eq!(fresh.observed_at_utc(), time);
        let stale_pair = IndexExpressionRelationVarSnapshot::new(
            &changed.base,
            &changed.relations,
            &changed.indexes,
            &changed.families,
            &changed.exclusions,
            &original.expressions,
            &original.type_modifiers,
            complete_vars(),
        )
        .unwrap_err();
        assert_eq!(
            stale_pair,
            ObservationError::InvalidObservationField {
                field: "index_expression_relation_var_predecessor_provenance",
            }
        );
        let mixed = IndexExpressionRelationVarSnapshot::new(
            &original.base,
            &original.relations,
            &original.indexes,
            &original.families,
            &original.exclusions,
            &original.expressions,
            &changed.type_modifiers,
            complete_vars(),
        )
        .unwrap_err();
        assert_eq!(
            mixed,
            ObservationError::InvalidObservationField {
                field: "index_expression_relation_var_predecessor_provenance",
            }
        );
    }
}

#[test]
fn relation_var_successor_rejects_whole_rows_and_trees_without_column_leaves() {
    let stack = stack();
    let constant = CanonicalExpression::node(
        "Const",
        vec![
            field(
                "type",
                CanonicalExpressionValue::Type(
                    QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
                ),
            ),
            field("value", CanonicalExpressionValue::Text("1".to_owned())),
        ],
    )
    .unwrap();
    for (root, expected_field) in [
        (
            CanonicalExpression::WholeRow,
            "index_partition_definition_expression_whole_row",
        ),
        (constant, "index_expression_relation_var_empty"),
    ] {
        let expressions = IndexExpressionSemanticsSnapshot::new(
            &stack.base,
            &stack.relations,
            &stack.indexes,
            &stack.families,
            &stack.exclusions,
            vec![
                IndexExpressionSemanticsObservation::new(parent_index(), 1, root.clone()).unwrap(),
                IndexExpressionSemanticsObservation::new(child_index(), 1, root.clone()).unwrap(),
            ],
            vec![
                IndexPredicateSemanticsObservation::new(parent_index(), root.clone()).unwrap(),
                IndexPredicateSemanticsObservation::new(child_index(), root).unwrap(),
            ],
        );
        if expected_field == "index_partition_definition_expression_whole_row" {
            assert_eq!(
                expressions.unwrap_err(),
                ObservationError::InvalidObservationField {
                    field: expected_field
                }
            );
            continue;
        }
        let expressions = expressions.unwrap();
        assert_eq!(
            IndexExpressionRelationVarSnapshot::new(
                &stack.base,
                &stack.relations,
                &stack.indexes,
                &stack.families,
                &stack.exclusions,
                &expressions,
                &stack.type_modifiers,
                vec![],
            )
            .unwrap_err(),
            ObservationError::InvalidObservationField {
                field: expected_field
            }
        );
    }
}
