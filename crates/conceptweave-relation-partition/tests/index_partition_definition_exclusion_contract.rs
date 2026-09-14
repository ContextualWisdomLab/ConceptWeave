use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionSemanticsSnapshot, IndexKeyExclusionSemanticsObservation,
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    PartitionParentRelationCoordinate, QualifiedOperatorFamilyName, QualifiedOperatorSignature,
    QualifiedProcedureSignature, RelationPartitionObservation, RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "warehouse"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "warehouse").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "warehouse"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == "warehouse"
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
        "warehouse",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn index(name: &str, exclusion: bool) -> IndexObservation {
    IndexObservation::new(
        name,
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "account_id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(false, exclusion, true, false, false, false))
    .unwrap()
    .with_valid(true)
}

fn relation(name: &str, kind: RelationKind, index_name: &str, exclusion: bool) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![ColumnObservationV3::new(
            "account_id",
            1,
            "integer",
            QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            false,
            None,
        )
        .unwrap()],
    )
    .unwrap()
    .with_indexes(vec![index(index_name, exclusion)])
    .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts",
        RelationKind::PartitionedTable,
        "accounts_excl",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts_2026",
        RelationKind::Table,
        "accounts_2026_excl",
    )
    .unwrap()
}

fn predecessor(
    parent_exclusion: bool,
    child_exclusion: bool,
) -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
    IndexOperatorFamilySnapshot,
) {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-equivalence-v1",
        "2026-09-14T17:08:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                "accounts_excl",
                parent_exclusion,
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                "accounts_2026_excl",
                child_exclusion,
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
        vec![
            family(parent_index()),
            family(child_index()),
        ],
    )
    .unwrap();
    (base, relations, indexes, families)
}

fn family(index: IndexPartitionCoordinate) -> IndexKeyOperatorFamilyObservation {
    IndexKeyOperatorFamilyObservation::new(
        index,
        1,
        QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
        QualifiedOperatorFamilyName::new("btree", "pg_catalog", "integer_ops").unwrap(),
    )
    .unwrap()
}

fn exclusion(
    index: IndexPartitionCoordinate,
    operator_name: &str,
    procedure_name: &str,
    strategy: u16,
) -> IndexKeyExclusionSemanticsObservation {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    IndexKeyExclusionSemanticsObservation::new(
        index,
        1,
        QualifiedOperatorSignature::new(
            "pg_catalog",
            operator_name,
            int4.clone(),
            int4.clone(),
        )
        .unwrap(),
        QualifiedProcedureSignature::new(
            "pg_catalog",
            procedure_name,
            vec![int4.clone(), int4],
        )
        .unwrap(),
        strategy,
    )
    .unwrap()
}

#[test]
fn attached_child_must_preserve_exclusion_presence() {
    let (base, relations, indexes, families) = predecessor(true, false);

    let error = IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, vec![])
        .expect_err("PostgreSQL CompareIndexInfo rejects one-sided exclusion semantics");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_exclusion_presence",
        }
    );
}

#[test]
fn attached_child_must_preserve_exclusion_operator_procedure_and_strategy() {
    let (base, relations, indexes, families) = predecessor(true, true);

    for (child_operator, child_procedure, child_strategy, expected_field) in [
        ("<", "int4lt", 3, "index_partition_definition_exclusion_operator"),
        ("=", "int4lt", 3, "index_partition_definition_exclusion_procedure"),
        ("=", "int4eq", 1, "index_partition_definition_exclusion_strategy"),
    ] {
        let error = IndexExclusionSemanticsSnapshot::new(
            &base,
            &relations,
            &indexes,
            &families,
            vec![
                exclusion(parent_index(), "=", "int4eq", 3),
                exclusion(child_index(), child_operator, child_procedure, child_strategy),
            ],
        )
        .expect_err("every exclusion key semantic must match its attached parent");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: expected_field,
            }
        );
    }
}

#[test]
fn matching_exclusion_semantics_are_admissible_and_complete() {
    let (base, relations, indexes, families) = predecessor(true, true);

    let snapshot = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![
            exclusion(parent_index(), "=", "int4eq", 3),
            exclusion(child_index(), "=", "int4eq", 3),
        ],
    )
    .expect("matching PostgreSQL exclusion arrays should remain admissible");
    assert_eq!(snapshot.observations().len(), 2);

    let missing = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![exclusion(parent_index(), "=", "int4eq", 3)],
    )
    .expect_err("every exclusion-index key requires semantic evidence");
    assert_eq!(
        missing,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_semantics_completeness",
        }
    );
}
