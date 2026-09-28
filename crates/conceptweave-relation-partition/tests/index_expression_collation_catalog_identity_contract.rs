use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation,
    IndexCatalogFlags, IndexKeySemantics, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedCollationName, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    CanonicalExpression, CanonicalExpressionField, CanonicalExpressionValue,
    CollationCatalogIdentity, CollationDefinitionObservation, ColumnTypeModifierObservation,
    DatabaseDefaultCollationDefinitionObservation, IndexCollationDatabaseEncodingSnapshot,
    IndexCollationDefinitionSnapshot, IndexEffectiveCollationDefinitionSnapshot,
    IndexExclusionSemanticsSnapshot, IndexExpressionCollationIdentityLocation,
    IndexExpressionCollationIdentityObservation, IndexExpressionCollationIdentitySnapshot,
    IndexExpressionNodeSchemaSnapshot, IndexExpressionRelationVarLocation,
    IndexExpressionRelationVarNodeSchemaSnapshot, IndexExpressionRelationVarObservation,
    IndexExpressionRelationVarSnapshot, IndexExpressionSemanticsObservation,
    IndexExpressionSemanticsSnapshot, IndexKeyCollationIdentityObservation,
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot,
    IndexPartitionCollationIdentitySnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, IndexRelationVarCollationIdentityObservation,
    PartitionParentRelationCoordinate, PostgresCollationProvider,
    PostgresDatabaseEncodingObservation, PostgresDatabaseLocaleProvider,
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

struct Registry<'a> {
    source_key: &'a str,
    policy_binding: &'a str,
}

impl SourceConnectionRegistry for Registry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.source_key
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == self.source_key).then(|| self.policy_binding.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == self.source_key
            && source_connection.connection_policy_binding() == self.policy_binding
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == self.source_key
            && source_connection.connection_policy_binding() == self.policy_binding
            && resource_envelope.request_budget().max_schema_count() <= 1
            && resource_envelope.request_budget().max_schema_bytes() <= 256
            && resource_envelope.limits().operation_timeout_ms() <= 1_000
            && resource_envelope.limits().statement_timeout_ms() <= 1_000
            && resource_envelope.limits().max_rows() <= 10
            && resource_envelope.limits().max_bytes() <= 1_024
            && resource_envelope.limits().max_concurrent_queries() <= 1
    }
}

fn authorized_source_with_context(
    source_key: &str,
    policy_binding: &str,
) -> AuthorizedObservationRequest {
    ObservationRequest::new(
        source_key,
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry {
        source_key,
        policy_binding,
    })
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
    stack_with_collation_encoding(ENCODING_UTF8)
}

fn stack_with_collation_encoding(catalog_encoding: i32) -> Stack {
    stack_with_capture_context(
        catalog_encoding,
        "warehouse_primary",
        POLICY_BINDING,
        "extractor-expression-collation-identity-v1",
        "2026-09-15T01:10:00Z",
    )
}

fn stack_with_capture_context(
    catalog_encoding: i32,
    source_key: &str,
    policy_binding: &str,
    extractor_revision: &str,
    observed_at: &str,
) -> Stack {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source_with_context(source_key, policy_binding),
        extractor_revision,
        observed_at,
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
                Some(catalog_default_collation(catalog_encoding)),
            )
            .unwrap(),
            IndexKeyCollationIdentityObservation::new(
                child_index(),
                1,
                Some(catalog_default_collation(catalog_encoding)),
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

#[test]
fn database_encoding_binds_the_complete_collation_stack_and_receipt() {
    let stack = stack();
    let expressions = compose(
        &stack,
        ENCODING_UTF8,
        ENCODING_UTF8,
        ENCODING_UTF8,
        ENCODING_UTF8,
    )
    .unwrap();
    let encoding = PostgresDatabaseEncodingObservation::new(ENCODING_UTF8).unwrap();
    let snapshot =
        IndexCollationDatabaseEncodingSnapshot::new(encoding, &stack.key_collations, &expressions)
            .expect("every observed collation belongs to this source database");
    let receipt = snapshot.source_receipt();

    assert_eq!(
        snapshot.key_collation_predecessor_digest(),
        stack.key_collations.snapshot_digest()
    );
    assert_eq!(
        snapshot.expression_collation_predecessor_digest(),
        expressions.snapshot_digest()
    );
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.observed_at_utc(), stack.base.observed_at_utc());
    assert_eq!(receipt.database_encoding(), encoding);
    assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());

    assert_eq!(
        IndexCollationDatabaseEncodingSnapshot::new(
            PostgresDatabaseEncodingObservation::new(8).unwrap(),
            &stack.key_collations,
            &expressions,
        ),
        Err(ObservationError::InvalidObservationField {
            field: "index_collation_database_encoding_binding",
        })
    );
}

#[test]
fn encoding_independent_collations_keep_database_encoding_in_snapshot_identity() {
    let stack = stack_with_collation_encoding(-1);
    let expressions = compose(&stack, -1, -1, -1, -1).unwrap();
    let utf8 = IndexCollationDatabaseEncodingSnapshot::new(
        PostgresDatabaseEncodingObservation::new(ENCODING_UTF8).unwrap(),
        &stack.key_collations,
        &expressions,
    )
    .unwrap();
    let latin1 = IndexCollationDatabaseEncodingSnapshot::new(
        PostgresDatabaseEncodingObservation::new(8).unwrap(),
        &stack.key_collations,
        &expressions,
    )
    .unwrap();

    assert_ne!(utf8.snapshot_digest(), latin1.snapshot_digest());
    assert_eq!(
        utf8.key_collation_predecessor_digest(),
        latin1.key_collation_predecessor_digest()
    );
    assert_eq!(
        utf8.expression_collation_predecessor_digest(),
        latin1.expression_collation_predecessor_digest()
    );
}

#[test]
fn database_default_effective_definition_binds_material_evidence_and_receipt() {
    let stack = stack_with_collation_encoding(-1);
    let expressions = compose(&stack, -1, -1, -1, -1).unwrap();
    let encoding = IndexCollationDatabaseEncodingSnapshot::new(
        PostgresDatabaseEncodingObservation::new(ENCODING_UTF8).unwrap(),
        &stack.key_collations,
        &expressions,
    )
    .unwrap();
    let material = IndexCollationDefinitionSnapshot::new(
        &encoding,
        &stack.key_collations,
        &expressions,
        vec![
            CollationDefinitionObservation::new(
                catalog_default_collation(-1),
                PostgresCollationProvider::DatabaseDefault,
                true,
                None,
                None,
                None,
                None,
                None,
                Some("153.80".to_owned()),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let coordinate = catalog_default_collation(-1);
    let material_receipt = material.source_receipt(&coordinate).unwrap();
    assert_eq!(material_receipt.identity(), &coordinate);
    assert_eq!(material_receipt.source_id(), "warehouse_primary");
    assert_eq!(material_receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(material_receipt.source_digest(), material.snapshot_digest());
    assert_eq!(
        material_receipt.extractor_revision(),
        stack.base.extractor_revision()
    );
    assert_eq!(
        material_receipt.observed_at_utc(),
        stack.base.observed_at_utc()
    );
    assert_eq!(material.predecessor_digest(), encoding.snapshot_digest());
    assert!(!material.has_version_mismatch());
    for foreign in [
        CollationCatalogIdentity::new("public", "default", -1).unwrap(),
        CollationCatalogIdentity::new("pg_catalog", "other", -1).unwrap(),
        catalog_default_collation(ENCODING_UTF8),
    ] {
        assert!(matches!(
            material.source_receipt(&foreign),
            Err(ObservationError::UnknownObservationLocation { .. })
        ));
    }
    for incomplete in [vec![], vec![material.definitions()[0].clone(); 2]] {
        assert_eq!(
            IndexCollationDefinitionSnapshot::new(
                &encoding,
                &stack.key_collations,
                &expressions,
                incomplete
            ),
            Err(ObservationError::InvalidObservationField {
                field: "index_collation_definition_completeness",
            })
        );
    }
    let database_default = |actual_version: &str| {
        DatabaseDefaultCollationDefinitionObservation::new(
            PostgresDatabaseLocaleProvider::Icu,
            Some("C.UTF-8".to_owned()),
            Some("C.UTF-8".to_owned()),
            Some("und".to_owned()),
            None,
            Some("153.80".to_owned()),
            Some(actual_version.to_owned()),
        )
        .unwrap()
    };
    let effective = |actual_version| {
        IndexEffectiveCollationDefinitionSnapshot::new(
            &material,
            &encoding,
            &stack.key_collations,
            &expressions,
            Some(database_default(actual_version)),
        )
    };
    let snapshot = effective("153.80").unwrap();
    let receipt = snapshot.database_default_source_receipt().unwrap();
    assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());

    assert_eq!(snapshot.predecessor_digest(), material.snapshot_digest());
    assert_eq!(
        snapshot.source_connection_key(),
        material.source_connection_key()
    );
    assert_eq!(
        snapshot.connection_policy_binding(),
        material.connection_policy_binding()
    );
    assert_eq!(snapshot.extractor_revision(), material.extractor_revision());
    assert_eq!(snapshot.observed_at_utc(), material.observed_at_utc());
    assert_eq!(
        snapshot.database_default_definition(),
        Some(&database_default("153.80"))
    );
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.observed_at_utc(), stack.base.observed_at_utc());
    assert!(!snapshot.has_database_default_version_mismatch());
    assert_eq!(
        effective("154.10"),
        Err(ObservationError::InvalidObservationField {
            field: "database_default_collation_actual_version_coherence",
        })
    );
    assert_eq!(
        IndexEffectiveCollationDefinitionSnapshot::new(
            &material,
            &encoding,
            &stack.key_collations,
            &expressions,
            None,
        ),
        Err(ObservationError::InvalidObservationField {
            field: "database_default_collation_definition_presence",
        })
    );
}

#[test]
fn material_collation_rejects_cross_capture_provenance_even_when_content_matches() {
    let baseline = stack_with_collation_encoding(-1);
    let expressions = compose(&baseline, -1, -1, -1, -1).unwrap();
    let encoding = IndexCollationDatabaseEncodingSnapshot::new(
        PostgresDatabaseEncodingObservation::new(ENCODING_UTF8).unwrap(),
        &baseline.key_collations,
        &expressions,
    )
    .unwrap();
    for (source_key, binding, revision, observed_at) in [
        (
            "warehouse_secondary",
            POLICY_BINDING,
            "extractor-expression-collation-identity-v1",
            "2026-09-15T01:10:00Z",
        ),
        (
            "warehouse_primary",
            "fixture_policy_revision_changed",
            "extractor-expression-collation-identity-v1",
            "2026-09-15T01:10:00Z",
        ),
        (
            "warehouse_primary",
            POLICY_BINDING,
            "extractor-expression-collation-identity-v2",
            "2026-09-15T01:10:00Z",
        ),
        (
            "warehouse_primary",
            POLICY_BINDING,
            "extractor-expression-collation-identity-v1",
            "2026-09-15T01:10:01Z",
        ),
    ] {
        let mut other = stack_with_capture_context(-1, source_key, binding, revision, observed_at);
        let other_expressions = compose(&other, -1, -1, -1, -1).unwrap();
        let current_composed = other.composed.clone();
        let current_keys = other.key_collations.clone();
        other.composed = baseline.composed.clone();
        other.key_collations = baseline.key_collations.clone();
        assert_eq!(
            compose(&other, -1, -1, -1, -1),
            Err(ObservationError::InvalidObservationField {
                field: "index_expression_collation_whole_tree_predecessor"
            })
        );
        other.composed = current_composed;
        assert_eq!(
            compose(&other, -1, -1, -1, -1),
            Err(ObservationError::InvalidObservationField {
                field: "index_expression_collation_key_predecessor"
            })
        );
        other.key_collations = current_keys;
        assert_eq!(other_expressions.source_connection_key(), source_key);
        assert_eq!(other_expressions.connection_policy_binding(), binding);
        assert_eq!(other_expressions.extractor_revision(), revision);
        assert_eq!(other_expressions.observed_at_utc(), observed_at);
        assert_eq!(
            baseline.base.snapshot_digest(),
            other.base.snapshot_digest()
        );
        assert_eq!(
            baseline.key_collations.snapshot_digest(),
            other.key_collations.snapshot_digest()
        );
        assert_eq!(
            expressions.snapshot_digest(),
            other_expressions.snapshot_digest()
        );
        for (keys, expressions, expected_field) in [
            (
                &other.key_collations,
                &expressions,
                "index_collation_database_encoding_provenance",
            ),
            (
                &baseline.key_collations,
                &other_expressions,
                "index_collation_database_encoding_provenance",
            ),
            (
                &other.key_collations,
                &other_expressions,
                "index_collation_definition_provenance",
            ),
        ] {
            assert_eq!(
                IndexCollationDefinitionSnapshot::new(&encoding, keys, expressions, vec![]),
                Err(ObservationError::InvalidObservationField {
                    field: expected_field
                })
            );
        }
    }
    let changed = stack_with_collation_encoding(ENCODING_UTF8);
    let changed_expressions = compose(
        &changed,
        ENCODING_UTF8,
        ENCODING_UTF8,
        ENCODING_UTF8,
        ENCODING_UTF8,
    )
    .unwrap();
    for (keys, expressions) in [
        (&changed.key_collations, &expressions),
        (&baseline.key_collations, &changed_expressions),
    ] {
        assert_eq!(
            IndexCollationDefinitionSnapshot::new(&encoding, keys, expressions, vec![]),
            Err(ObservationError::InvalidObservationField {
                field: "index_collation_definition_predecessor"
            })
        );
    }
}

#[test]
fn collation_locations_keep_expression_keys_distinct_from_predicates() {
    let expression =
        IndexExpressionCollationIdentityLocation::expression(child_index(), 1, 2).unwrap();
    let predicate = IndexExpressionCollationIdentityLocation::predicate(child_index(), 2).unwrap();
    assert_eq!(expression.key_position(), Some(1));
    assert_eq!(predicate.key_position(), None);
    assert_eq!(expression.occurrence_position(), 2);
    assert_eq!(predicate.occurrence_position(), 2);
    assert_ne!(
        expression.canonical_location(),
        predicate.canonical_location()
    );
    for location in [
        IndexExpressionCollationIdentityLocation::expression(child_index(), 0, 2),
        IndexExpressionCollationIdentityLocation::expression(child_index(), 1, 0),
        IndexExpressionCollationIdentityLocation::predicate(child_index(), 0),
    ] {
        assert_eq!(location, Err(ObservationError::InvalidOrdinalPosition));
    }
}
