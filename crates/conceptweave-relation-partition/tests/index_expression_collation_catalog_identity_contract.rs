use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation,
    IndexCatalogFlags, IndexKeySemantics, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedCollationName, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    CollationCatalogIdentity, ColumnTypeModifierObservation, IndexExclusionSemanticsSnapshot,
    IndexExpressionCollationIdentityLocation, IndexExpressionCollationIdentityObservation,
    IndexExpressionCollationIdentitySnapshot, IndexExpressionNodeSchemaSnapshot,
    IndexExpressionRelationVarLocation, IndexExpressionRelationVarNodeSchemaSnapshot,
    IndexExpressionRelationVarObservation, IndexExpressionRelationVarSnapshot,
    IndexExpressionSemanticsObservation, IndexExpressionSemanticsSnapshot,
    IndexKeyCollationIdentityObservation, IndexKeyOperatorFamilyObservation,
    IndexOperatorFamilySnapshot, IndexPartitionCollationIdentitySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    IndexRelationVarCollationIdentityObservation, PartitionParentRelationCoordinate,
    QualifiedFunctionSignature, QualifiedOperatorFamilyName, RelationPartitionObservation,
    RelationPartitionSnapshot, RelationPartitionTypeModifierSnapshot, RelationVarRelationRole,
    RelationVarReturningType,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_expression_collation_identity";
const ENCODING_UTF8: i32 = 6;

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

fn field(name: &str, value: CanonicalExpressionValue) -> CanonicalExpressionField {
    CanonicalExpressionField::new(name, value).unwrap()
}

fn qualified_default_collation() -> QualifiedCollationName {
    QualifiedCollationName::new("pg_catalog", "default").unwrap()
}

fn catalog_default_collation(encoding: i32) -> CollationCatalogIdentity {
    CollationCatalogIdentity::new("pg_catalog", "default", encoding).unwrap()
}

fn expression() -> CanonicalExpression {
    let text = QualifiedTypeName::new("pg_catalog", "text").unwrap();
    let collation = qualified_default_collation();
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
            field("returns_set", CanonicalExpressionValue::Boolean(false)),
            field("variadic", CanonicalExpressionValue::Boolean(false)),
            field(
                "result_collation",
                CanonicalExpressionValue::Collation(collation.clone()),
            ),
            field(
                "input_collation",
                CanonicalExpressionValue::Collation(collation),
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
            Some(qualified_default_collation()),
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
    .with_valid(true)
}

fn relation(name: &str, kind: RelationKind, index_name: &str) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
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
    .with_indexes(vec![index(index_name)])
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
    node_schema: IndexExpressionNodeSchemaSnapshot,
    relation_vars: IndexExpressionRelationVarSnapshot,
    composed: IndexExpressionRelationVarNodeSchemaSnapshot,
    key_collations: IndexPartitionCollationIdentitySnapshot,
}

fn stack() -> Stack {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-expression-collation-identity-v1",
        "2026-09-15T01:10:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                "accounts_email_idx",
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                "accounts_2026_email_idx",
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap()
    .with_observed_column_collations(vec![
        ColumnCollationObservation::collatable(
            "public",
            "accounts",
            RelationKind::PartitionedTable,
            "account_email",
            qualified_default_collation(),
            true,
        )
        .unwrap(),
        ColumnCollationObservation::collatable(
            "public",
            "accounts_2026",
            RelationKind::Table,
            "account_email",
            qualified_default_collation(),
            true,
        )
        .unwrap(),
    ])
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
            IndexExpressionSemanticsObservation::new(parent_index(), 1, expression()).unwrap(),
            IndexExpressionSemanticsObservation::new(child_index(), 1, expression()).unwrap(),
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
                RelationKind::PartitionedTable,
                "account_email",
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

    let node_schema = IndexExpressionNodeSchemaSnapshot::new(&expressions).unwrap();

    let relation_vars = IndexExpressionRelationVarSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        &exclusions,
        &expressions,
        &type_modifiers,
        vec![
            IndexExpressionRelationVarObservation::new(
                IndexExpressionRelationVarLocation::expression(parent_index(), 1, 1).unwrap(),
                "account_email",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                -1,
                Some(qualified_default_collation()),
                RelationVarRelationRole::IndexRelation,
                true,
                0,
                RelationVarReturningType::Default,
            )
            .unwrap(),
            IndexExpressionRelationVarObservation::new(
                IndexExpressionRelationVarLocation::expression(child_index(), 1, 1).unwrap(),
                "account_email",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                -1,
                Some(qualified_default_collation()),
                RelationVarRelationRole::IndexRelation,
                true,
                0,
                RelationVarReturningType::Default,
            )
            .unwrap(),
        ],
    )
    .unwrap();

    let composed = IndexExpressionRelationVarNodeSchemaSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        &exclusions,
        &expressions,
        &type_modifiers,
        &node_schema,
        &relation_vars,
    )
    .unwrap();

    let key_collations = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexKeyCollationIdentityObservation::new(
                parent_index(),
                1,
                Some(catalog_default_collation(ENCODING_UTF8)),
            )
            .unwrap(),
            IndexKeyCollationIdentityObservation::new(
                child_index(),
                1,
                Some(catalog_default_collation(ENCODING_UTF8)),
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
        node_schema,
        relation_vars,
        composed,
        key_collations,
    }
}

fn expression_collation(
    index: IndexPartitionCoordinate,
    occurrence_position: u32,
    encoding: i32,
) -> IndexExpressionCollationIdentityObservation {
    IndexExpressionCollationIdentityObservation::new(
        IndexExpressionCollationIdentityLocation::expression(index, 1, occurrence_position)
            .unwrap(),
        catalog_default_collation(encoding),
    )
    .unwrap()
}

fn var_collation(
    index: IndexPartitionCoordinate,
    encoding: i32,
) -> IndexRelationVarCollationIdentityObservation {
    IndexRelationVarCollationIdentityObservation::new(
        IndexExpressionRelationVarLocation::expression(index, 1, 1).unwrap(),
        Some(catalog_default_collation(encoding)),
    )
    .unwrap()
}

fn compose(
    stack: &Stack,
    expression_parent_encoding: i32,
    expression_child_encoding: i32,
    var_parent_encoding: i32,
    var_child_encoding: i32,
) -> Result<IndexExpressionCollationIdentitySnapshot, ObservationError> {
    IndexExpressionCollationIdentitySnapshot::new(
        &stack.base,
        &stack.relations,
        &stack.indexes,
        &stack.families,
        &stack.exclusions,
        &stack.expressions,
        &stack.type_modifiers,
        &stack.node_schema,
        &stack.relation_vars,
        &stack.composed,
        &stack.key_collations,
        vec![
            expression_collation(parent_index(), 1, expression_parent_encoding),
            expression_collation(parent_index(), 2, expression_parent_encoding),
            expression_collation(child_index(), 1, expression_child_encoding),
            expression_collation(child_index(), 2, expression_child_encoding),
        ],
        vec![
            var_collation(parent_index(), var_parent_encoding),
            var_collation(child_index(), var_child_encoding),
        ],
    )
}

#[test]
fn expression_collation_occurrences_must_preserve_catalog_row_identity_across_attachment() {
    let stack = stack();
    let error = compose(&stack, -1, ENCODING_UTF8, ENCODING_UTF8, ENCODING_UTF8)
        .expect_err("equal qualified names cannot hide distinct pg_collation rows in node fields");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_expression_collation_catalog_identity",
        }
    );
}

#[test]
fn relation_var_collation_must_preserve_catalog_row_identity_across_attachment() {
    let stack = stack();
    let error = compose(&stack, ENCODING_UTF8, ENCODING_UTF8, -1, ENCODING_UTF8)
        .expect_err("equal qualified varcollid names cannot hide distinct pg_collation rows");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_relation_var_collation_catalog_identity",
        }
    );
}

#[test]
fn matching_catalog_identities_compose_with_the_existing_whole_tree_and_key_proofs() {
    let stack = stack();
    let snapshot = compose(
        &stack,
        ENCODING_UTF8,
        ENCODING_UTF8,
        ENCODING_UTF8,
        ENCODING_UTF8,
    )
    .expect("catalog-exact collation identity must compose when every attached occurrence matches");

    assert_eq!(
        snapshot.whole_tree_predecessor_digest(),
        stack.composed.snapshot_digest()
    );
    assert_eq!(
        snapshot.key_collation_predecessor_digest(),
        stack.key_collations.snapshot_digest()
    );
    assert!(snapshot.snapshot_digest().starts_with("sha256:"));
    assert_eq!(snapshot.expression_observations().len(), 4);
    assert_eq!(snapshot.relation_var_observations().len(), 2);
}
