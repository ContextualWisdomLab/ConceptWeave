use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedCollationName, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation,
};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    IndexExclusionSemanticsSnapshot, IndexExpressionSemanticsLocation,
    IndexExpressionSemanticsObservation, IndexExpressionSemanticsSnapshot,
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexPredicateSemanticsObservation,
    IndexRelationKind, PartitionParentRelationCoordinate, QualifiedFunctionSignature,
    QualifiedOperatorFamilyName, QualifiedOperatorSignature, RelationPartitionObservation,
    RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "warehouse_primary"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "warehouse_primary").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "warehouse_primary"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == "warehouse_primary"
            && source_connection.connection_policy_binding() == POLICY_BINDING
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
    ObservationRequest::new(
        "warehouse_primary",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
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
    reverse_column_order: bool,
) -> RelationObservation {
    let account_id = ColumnObservationV3::new(
        "account_id",
        if reverse_column_order { 2 } else { 1 },
        "integer",
        QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
        false,
        None,
    )
    .unwrap();
    let account_email = ColumnObservationV3::new(
        "account_email",
        if reverse_column_order { 1 } else { 2 },
        "text",
        QualifiedTypeName::new("pg_catalog", "text").unwrap(),
        false,
        None,
    )
    .unwrap();
    let columns = if reverse_column_order {
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

fn predecessor() -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
    IndexOperatorFamilySnapshot,
    IndexExclusionSemanticsSnapshot,
) {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-expression-equivalence-v1",
        "2026-09-14T17:45:00Z",
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
    .unwrap();

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
        vec![family(parent_index()), family(child_index())],
    )
    .unwrap();

    let exclusions =
        IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, vec![])
            .unwrap();

    (base, relations, indexes, families, exclusions)
}

fn family(index: IndexPartitionCoordinate) -> IndexKeyOperatorFamilyObservation {
    IndexKeyOperatorFamilyObservation::new(
        index,
        1,
        QualifiedOperatorClassName::new("pg_catalog", "text_ops").unwrap(),
        QualifiedOperatorFamilyName::new("btree", "pg_catalog", "text_ops").unwrap(),
    )
    .unwrap()
}

fn field(name: &str, value: CanonicalExpressionValue) -> CanonicalExpressionField {
    CanonicalExpressionField::new(name, value).unwrap()
}

fn function_expression(function_name: &str) -> CanonicalExpression {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    CanonicalExpression::node(
        "FuncExpr",
        vec![
            field(
                "function",
                CanonicalExpressionValue::Function(
                    QualifiedFunctionSignature::new(
                        "pg_catalog",
                        function_name,
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

fn predicate_expression(operator_name: &str) -> CanonicalExpression {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    CanonicalExpression::node(
        "OpExpr",
        vec![
            field(
                "operator",
                CanonicalExpressionValue::Operator(
                    QualifiedOperatorSignature::new(
                        "pg_catalog",
                        operator_name,
                        int4.clone(),
                        int4.clone(),
                    )
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

fn expression(
    index: IndexPartitionCoordinate,
    root: CanonicalExpression,
) -> IndexExpressionSemanticsObservation {
    IndexExpressionSemanticsObservation::new(index, 1, root).unwrap()
}

fn predicate(
    index: IndexPartitionCoordinate,
    root: CanonicalExpression,
) -> IndexPredicateSemanticsObservation {
    IndexPredicateSemanticsObservation::new(index, root).unwrap()
}

fn semantic_snapshot(
    parent_expression: CanonicalExpression,
    child_expression: CanonicalExpression,
    parent_predicate: CanonicalExpression,
    child_predicate: CanonicalExpression,
) -> Result<IndexExpressionSemanticsSnapshot, ObservationError> {
    let (base, relations, indexes, families, exclusions) = predecessor();
    IndexExpressionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        &exclusions,
        vec![
            expression(parent_index(), parent_expression),
            expression(child_index(), child_expression),
        ],
        vec![
            predicate(parent_index(), parent_predicate),
            predicate(child_index(), child_predicate),
        ],
    )
}

#[test]
fn attached_expression_semantics_must_match_after_attribute_mapping() {
    let error = semantic_snapshot(
        function_expression("lower"),
        function_expression("upper"),
        predicate_expression(">"),
        predicate_expression(">"),
    )
    .expect_err("PostgreSQL CompareIndexInfo rejects unequal mapped expression trees");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_expression",
        }
    );
}

#[test]
fn attached_partial_predicate_semantics_must_match_after_attribute_mapping() {
    let error = semantic_snapshot(
        function_expression("lower"),
        function_expression("lower"),
        predicate_expression(">"),
        predicate_expression(">="),
    )
    .expect_err("PostgreSQL CompareIndexInfo rejects unequal mapped predicate trees");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_predicate",
        }
    );
}

#[test]
fn child_whole_row_reference_is_not_attachable() {
    let error = semantic_snapshot(
        CanonicalExpression::whole_row(),
        CanonicalExpression::whole_row(),
        predicate_expression(">"),
        predicate_expression(">"),
    )
    .expect_err("PostgreSQL map_variable_attnos reports whole-row child Vars as non-equivalent");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_expression_whole_row",
        }
    );
}

#[test]
fn stable_column_identity_survives_different_physical_attribute_order() {
    let snapshot = semantic_snapshot(
        function_expression("lower"),
        function_expression("lower"),
        predicate_expression(">"),
        predicate_expression(">"),
    )
    .expect("mapped column identity should not depend on relation-local attnums");

    assert_eq!(snapshot.expression_observations().len(), 2);
    assert_eq!(snapshot.predicate_observations().len(), 2);

    for index in [parent_index(), child_index()] {
        for location in [
            IndexExpressionSemanticsLocation::Expression {
                index: index.clone(),
                key_position: 1,
            },
            IndexExpressionSemanticsLocation::Predicate {
                index: index.clone(),
            },
        ] {
            let receipt = snapshot.source_receipt(location.clone()).unwrap();
            assert_eq!(receipt.location(), &location);
            assert_eq!(receipt.source_id(), snapshot.source_connection_key());
            assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
            assert_eq!(
                receipt.connection_policy_binding(),
                snapshot.connection_policy_binding()
            );
            assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());
            assert_eq!(receipt.observed_at_utc(), snapshot.observed_at_utc());
        }
        for key_position in [0, 2] {
            assert_eq!(
                snapshot
                    .source_receipt(IndexExpressionSemanticsLocation::Expression {
                        index: index.clone(),
                        key_position,
                    })
                    .unwrap_err(),
                ObservationError::UnknownObservationLocation {
                    location: format!(
                        "{}/keys/{key_position}/expression-semantics",
                        index.canonical_location()
                    ),
                }
            );
        }
        for (schema, relation, kind, name) in [
            (
                "archive",
                index.relation_name(),
                index.relation_kind(),
                index.index_name(),
            ),
            (
                index.schema_name(),
                "unobserved_relation",
                index.relation_kind(),
                index.index_name(),
            ),
            (
                index.schema_name(),
                index.relation_name(),
                RelationKind::ForeignTable,
                index.index_name(),
            ),
            (
                index.schema_name(),
                index.relation_name(),
                index.relation_kind(),
                "unobserved_index",
            ),
        ] {
            let absent = IndexPartitionCoordinate::new(schema, relation, kind, name).unwrap();
            for (location, suffix) in [
                (
                    IndexExpressionSemanticsLocation::Expression {
                        index: absent.clone(),
                        key_position: 1,
                    },
                    "keys/1/expression-semantics",
                ),
                (
                    IndexExpressionSemanticsLocation::Predicate {
                        index: absent.clone(),
                    },
                    "predicate-semantics",
                ),
            ] {
                assert_eq!(
                    snapshot.source_receipt(location).unwrap_err(),
                    ObservationError::UnknownObservationLocation {
                        location: format!("{}/{suffix}", absent.canonical_location()),
                    }
                );
            }
        }
    }
}

#[test]
fn every_raw_expression_and_predicate_requires_exact_semantic_evidence() {
    let (base, relations, indexes, families, exclusions) = predecessor();
    let build = |expressions, predicates| {
        IndexExpressionSemanticsSnapshot::new(
            &base,
            &relations,
            &indexes,
            &families,
            &exclusions,
            expressions,
            predicates,
        )
    };
    let expressions = vec![
        expression(parent_index(), function_expression("lower")),
        expression(child_index(), function_expression("lower")),
    ];
    let predicates = vec![
        predicate(parent_index(), predicate_expression(">")),
        predicate(child_index(), predicate_expression(">")),
    ];
    for position in 0..2 {
        let mut missing = expressions.clone();
        missing.remove(position);
        let mut duplicate = expressions.clone();
        duplicate.push(expressions[position].clone());
        let mut extra = expressions.clone();
        extra.push(
            IndexExpressionSemanticsObservation::new(
                expressions[position].index().clone(),
                2,
                function_expression("lower"),
            )
            .unwrap(),
        );
        let mut wrong_column = expressions.clone();
        wrong_column[position] = expression(
            expressions[position].index().clone(),
            CanonicalExpression::column("unobserved_column").unwrap(),
        );
        for (observations, field) in [
            (missing, "index_expression_semantics_completeness"),
            (duplicate, "index_expression_semantics_coordinate"),
            (extra, "index_expression_semantics_completeness"),
            (wrong_column, "index_expression_semantics_column_binding"),
        ] {
            assert_eq!(
                build(observations, predicates.clone()).unwrap_err(),
                ObservationError::InvalidObservationField { field }
            );
        }
        let mut missing = predicates.clone();
        missing.remove(position);
        let mut duplicate = predicates.clone();
        duplicate.push(predicates[position].clone());
        let mut extra = predicates.clone();
        extra.push(predicate(
            IndexPartitionCoordinate::new(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
                "unobserved_index",
            )
            .unwrap(),
            predicate_expression(">"),
        ));
        let mut wrong_column = predicates.clone();
        wrong_column[position] = predicate(
            predicates[position].index().clone(),
            CanonicalExpression::column("unobserved_column").unwrap(),
        );
        for (observations, field) in [
            (missing, "index_predicate_semantics_completeness"),
            (duplicate, "index_predicate_semantics_coordinate"),
            (extra, "index_predicate_semantics_completeness"),
            (wrong_column, "index_expression_semantics_column_binding"),
        ] {
            assert_eq!(
                build(expressions.clone(), observations).unwrap_err(),
                ObservationError::InvalidObservationField { field }
            );
        }
    }
    let snapshot = build(expressions.clone(), predicates.clone()).unwrap();
    let mut reversed_expressions = expressions;
    let mut reversed_predicates = predicates;
    reversed_expressions.reverse();
    reversed_predicates.reverse();
    assert_eq!(
        snapshot,
        build(reversed_expressions, reversed_predicates).unwrap()
    );
}
