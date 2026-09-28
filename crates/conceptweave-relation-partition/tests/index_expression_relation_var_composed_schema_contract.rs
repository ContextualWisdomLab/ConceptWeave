use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation,
    IndexCatalogFlags, IndexKeySemantics, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedCollationName, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    ColumnTypeModifierObservation, IndexExclusionSemanticsSnapshot,
    IndexExpressionNodeSchemaSnapshot, IndexExpressionNodeSchemaSnapshotV2,
    IndexExpressionRelationVarLocation, IndexExpressionRelationVarNodeSchemaSnapshot,
    IndexExpressionRelationVarObservation, IndexExpressionRelationVarSnapshot,
    IndexExpressionSemanticsObservation, IndexExpressionSemanticsSnapshot,
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    QualifiedFunctionSignature, QualifiedOperatorFamilyName, RelationPartitionObservation,
    RelationPartitionSnapshot, RelationPartitionTypeModifierSnapshot, RelationVarRelationRole,
    RelationVarReturningType,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_relation_var_composed_schema";
const CAPTURE: (&str, &str, &str, &str) = (
    "warehouse_primary",
    POLICY_BINDING,
    "extractor-relation-var-composed-v1",
    "2026-09-15T00:30:00Z",
);

struct Registry<'a> {
    source: &'a str,
    policy: &'a str,
}

impl SourceConnectionRegistry for Registry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.source
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == self.source).then(|| self.policy.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == self.source
            && source_connection.connection_policy_binding() == self.policy
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == self.source
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

fn authorized_source(source: &str, policy: &str) -> AuthorizedObservationRequest {
    ObservationRequest::new(
        source,
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry { source, policy })
    .unwrap()
}

fn coordinate() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts",
        RelationKind::Table,
        "accounts_email_idx",
    )
    .unwrap()
}

fn field(name: &str, value: CanonicalExpressionValue) -> CanonicalExpressionField {
    CanonicalExpressionField::new(name, value).unwrap()
}

fn expression(complete_node_schema: bool) -> CanonicalExpression {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    let default_collation = QualifiedCollationName::new("pg_catalog", "default").unwrap();
    let mut fields = vec![
        field(
            "function",
            CanonicalExpressionValue::Function(
                QualifiedFunctionSignature::new("pg_catalog", "lower", vec![text.clone()], text)
                    .unwrap(),
            ),
        ),
        field(
            "arguments",
            CanonicalExpressionValue::ExpressionList(vec![
                CanonicalExpression::column("account_email").unwrap(),
            ]),
        ),
    ];
    if complete_node_schema {
        fields.extend([
            field("returns_set", CanonicalExpressionValue::Boolean(false)),
            field("variadic", CanonicalExpressionValue::Boolean(false)),
            field(
                "result_collation",
                CanonicalExpressionValue::Collation(default_collation.clone()),
            ),
            field(
                "input_collation",
                CanonicalExpressionValue::Collation(default_collation),
            ),
        ]);
    }
    CanonicalExpression::node("FuncExpr", fields).unwrap()
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

fn stack(complete_node_schema: bool) -> Stack {
    stack_with_expression(
        expression(complete_node_schema),
        "lower(account_email)",
        "text_ops",
        "text_ops",
        Some(QualifiedCollationName::new("pg_catalog", "default").unwrap()),
        CAPTURE,
    )
}

fn stack_with_expression(
    canonical_expression: CanonicalExpression,
    source_expression: &str,
    operator_class: &str,
    operator_family: &str,
    collation: Option<QualifiedCollationName>,
    capture: (&str, &str, &str, &str),
) -> Stack {
    let index = IndexObservation::new(
        "accounts_email_idx",
        false,
        Some(false),
        vec![
            IndexAttributeObservation::expression(1, IndexAttributeKind::Key, source_expression)
                .unwrap(),
        ],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            collation,
            QualifiedOperatorClassName::new("pg_catalog", operator_class).unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(
        false, false, true, false, false, false,
    ))
    .unwrap()
    .with_valid(true);

    let relation = RelationObservation::new(
        "public",
        "accounts",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "account_email",
                1,
                "text",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                false,
                None,
            )
            .unwrap(),
        ],
    )
    .unwrap()
    .with_indexes(vec![index])
    .unwrap();

    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(capture.0, capture.1),
        capture.2,
        capture.3,
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_column_collations(vec![
        ColumnCollationObservation::collatable(
            "public",
            "accounts",
            RelationKind::Table,
            "account_email",
            QualifiedCollationName::new("pg_catalog", "default").unwrap(),
            true,
        )
        .unwrap(),
    ])
    .unwrap();

    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition("public", "accounts", RelationKind::Table)
                .unwrap(),
        ],
    )
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(coordinate(), IndexRelationKind::Index)
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
                coordinate(),
                1,
                QualifiedOperatorClassName::new("pg_catalog", operator_class).unwrap(),
                QualifiedOperatorFamilyName::new("btree", "pg_catalog", operator_family).unwrap(),
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
            IndexExpressionSemanticsObservation::new(coordinate(), 1, canonical_expression)
                .unwrap(),
        ],
        vec![],
    )
    .unwrap();
    let type_modifiers = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relations,
        vec![
            ColumnTypeModifierObservation::new(
                "public",
                "accounts",
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

fn relation_vars(stack: &Stack) -> IndexExpressionRelationVarSnapshot {
    IndexExpressionRelationVarSnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.families,
        &stack.exclusions,
        &stack.expressions,
        &stack.type_modifiers,
        vec![
            IndexExpressionRelationVarObservation::new(
                IndexExpressionRelationVarLocation::expression(coordinate(), 1, 1).unwrap(),
                "account_email",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                -1,
                Some(QualifiedCollationName::new("pg_catalog", "default").unwrap()),
                RelationVarRelationRole::IndexRelation,
                true,
                0,
                RelationVarReturningType::Default,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

#[test]
fn relation_var_v1_alone_can_exist_when_the_enclosing_node_schema_is_incomplete() {
    let stack = stack(false);
    relation_vars(&stack);
    assert!(IndexExpressionNodeSchemaSnapshot::new(&stack.expressions).is_err());
}

#[test]
fn complete_node_schema_and_relation_var_proofs_compose_into_a_new_successor() {
    assert_eq!(
        IndexExpressionRelationVarNodeSchemaSnapshot::schema_revision(),
        "postgresql-18-equal-node-schema-plus-relation-var-v1"
    );
    let stack = stack(true);
    let node_schema = IndexExpressionNodeSchemaSnapshot::new(&stack.expressions)
        .expect("complete FuncExpr fields must satisfy the historical v1 node schema");
    assert_eq!(
        IndexExpressionNodeSchemaSnapshotV2::new(&stack.expressions, &node_schema).unwrap_err(),
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema_v2",
        }
    );
    let relation_vars = relation_vars(&stack);

    let composed = IndexExpressionRelationVarNodeSchemaSnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.families,
        &stack.exclusions,
        &stack.expressions,
        &stack.type_modifiers,
        &node_schema,
        &relation_vars,
    )
    .expect("exact node-schema v1 plus exact relation-Var v1 must compose");

    assert_eq!(
        composed.node_schema_predecessor_digest(),
        node_schema.snapshot_digest()
    );
    assert_eq!(
        composed.relation_var_predecessor_digest(),
        relation_vars.snapshot_digest()
    );
    assert!(composed.snapshot_digest().starts_with("sha256:"));
}

#[test]
fn composed_schema_rejects_jointly_stale_proofs_even_when_content_is_equal() {
    let original = stack(true);
    let old_schema = IndexExpressionNodeSchemaSnapshot::new(&original.expressions).unwrap();
    let old_vars = relation_vars(&original);
    let compose = |stack: &Stack,
                   schema: &IndexExpressionNodeSchemaSnapshot,
                   vars: &IndexExpressionRelationVarSnapshot| {
        IndexExpressionRelationVarNodeSchemaSnapshot::new(
            &stack.base,
            &stack.relations,
            &stack.indexes,
            &stack.families,
            &stack.exclusions,
            &stack.expressions,
            &stack.type_modifiers,
            schema,
            vars,
        )
    };
    let original_composed = compose(&original, &old_schema, &old_vars).unwrap();
    for capture in [
        ("warehouse_archive", CAPTURE.1, CAPTURE.2, CAPTURE.3),
        (
            CAPTURE.0,
            "fixture_policy_revision_other",
            CAPTURE.2,
            CAPTURE.3,
        ),
        (
            CAPTURE.0,
            CAPTURE.1,
            "extractor-relation-var-composed-v2",
            CAPTURE.3,
        ),
        (CAPTURE.0, CAPTURE.1, CAPTURE.2, "2026-09-15T00:31:00Z"),
    ] {
        let current = stack_with_expression(
            expression(true),
            "lower(account_email)",
            "text_ops",
            "text_ops",
            Some(QualifiedCollationName::new("pg_catalog", "default").unwrap()),
            capture,
        );
        if capture.0 == CAPTURE.0 && capture.1 == CAPTURE.1 {
            assert_eq!(
                current.base.snapshot_digest(),
                original.base.snapshot_digest()
            );
        }
        let current_schema = IndexExpressionNodeSchemaSnapshot::new(&current.expressions).unwrap();
        let current_vars = relation_vars(&current);
        for (schema, vars, field) in [
            (
                &old_schema,
                &old_vars,
                "index_expression_relation_var_node_schema_predecessor",
            ),
            (
                &old_schema,
                &current_vars,
                "index_expression_relation_var_node_schema_predecessor",
            ),
            (
                &current_schema,
                &old_vars,
                "index_expression_relation_var_predecessor",
            ),
        ] {
            let error = compose(&current, schema, vars)
                .expect_err("stale proof cannot authorize a different capture generation");
            assert_eq!(error, ObservationError::InvalidObservationField { field });
        }
        let composed = compose(&current, &current_schema, &current_vars).unwrap();
        assert_eq!(composed.source_connection_key(), capture.0);
        assert_eq!(composed.connection_policy_binding(), capture.1);
        assert_eq!(composed.extractor_revision(), capture.2);
        assert_eq!(composed.observed_at_utc(), capture.3);
        if capture.0 == CAPTURE.0 && capture.1 == CAPTURE.1 {
            assert_eq!(
                composed.snapshot_digest(),
                original_composed.snapshot_digest()
            );
        }
    }
}

#[test]
fn complete_var_free_node_schema_issues_a_distinct_v2_identity() {
    let canonical = CanonicalExpression::node(
        "FuncExpr",
        vec![
            field(
                "function",
                CanonicalExpressionValue::Function(
                    QualifiedFunctionSignature::new(
                        "pg_catalog",
                        "pi",
                        vec![],
                        QualifiedTypeName::new("pg_catalog", "float8").unwrap(),
                    )
                    .unwrap(),
                ),
            ),
            field("returns_set", CanonicalExpressionValue::Boolean(false)),
            field("variadic", CanonicalExpressionValue::Boolean(false)),
            field("result_collation", CanonicalExpressionValue::Null),
            field("input_collation", CanonicalExpressionValue::Null),
            field(
                "arguments",
                CanonicalExpressionValue::ExpressionList(vec![]),
            ),
        ],
    )
    .unwrap();
    let other_capture = stack(true);
    let stack = stack_with_expression(canonical, "pi()", "float8_ops", "float_ops", None, CAPTURE);
    let v1 = IndexExpressionNodeSchemaSnapshot::new(&stack.expressions).unwrap();
    let other_v1 = IndexExpressionNodeSchemaSnapshot::new(&other_capture.expressions).unwrap();
    assert_eq!(
        IndexExpressionNodeSchemaSnapshotV2::new(&stack.expressions, &other_v1).unwrap_err(),
        ObservationError::InvalidObservationField {
            field: "canonical_expression_node_schema_v2",
        }
    );
    let original_v1 = v1.clone();
    let v2 = IndexExpressionNodeSchemaSnapshotV2::new(&stack.expressions, &v1).unwrap();
    assert_eq!(v2.source_connection_key(), v1.source_connection_key());
    assert_eq!(
        v2.connection_policy_binding(),
        v1.connection_policy_binding()
    );
    assert_eq!(v2.extractor_revision(), v1.extractor_revision());
    assert_eq!(v2.observed_at_utc(), v1.observed_at_utc());
    assert_eq!(v2.predecessor_digest(), v1.snapshot_digest());
    assert_ne!(v2.snapshot_digest(), v1.snapshot_digest());
    assert!(v2.snapshot_digest().starts_with("sha256:"));
    assert_eq!(v1, original_v1);
    assert_eq!(
        v2,
        IndexExpressionNodeSchemaSnapshotV2::new(&stack.expressions, &v1).unwrap()
    );
}
