use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation,
    IndexCatalogFlags, IndexKeySemantics, IndexObservation, PostgresSchemaSnapshotV3,
    QualifiedCollationName, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation,
};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    ColumnTypeModifierObservation, IndexExclusionSemanticsSnapshot,
    IndexExpressionNodeSchemaSnapshot, IndexExpressionRelationVarLocation,
    IndexExpressionRelationVarNodeSchemaSnapshot, IndexExpressionRelationVarObservation,
    IndexExpressionRelationVarSnapshot, IndexExpressionSemanticsObservation,
    IndexExpressionSemanticsSnapshot, IndexKeyOperatorFamilyObservation,
    IndexOperatorFamilySnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, QualifiedFunctionSignature,
    QualifiedOperatorFamilyName, RelationPartitionObservation, RelationPartitionSnapshot,
    RelationPartitionTypeModifierSnapshot, RelationVarRelationRole, RelationVarReturningType,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_relation_var_composed_schema";

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
    let index = IndexObservation::new(
        "accounts_email_idx",
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
        &authorized_source(),
        "extractor-relation-var-composed-v1",
        "2026-09-15T00:30:00Z",
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
            IndexExpressionSemanticsObservation::new(
                coordinate(),
                1,
                expression(complete_node_schema),
            )
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
    let stack = stack(true);
    let node_schema = IndexExpressionNodeSchemaSnapshot::new(&stack.expressions)
        .expect("complete FuncExpr fields must satisfy the historical v1 node schema");
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
