use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, QualifiedCollationName,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    CollationCatalogIdentity, IndexKeyCollationIdentityObservation,
    IndexPartitionCollationIdentitySnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, PartitionParentRelationCoordinate,
    RelationPartitionObservation, RelationPartitionSnapshot,
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

fn index(name: &str) -> IndexObservation {
    IndexObservation::new(
        name,
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "label").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            Some(QualifiedCollationName::new("pg_catalog", "C").unwrap()),
            QualifiedOperatorClassName::new("pg_catalog", "text_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_valid(true)
}

fn relation(name: &str, kind: RelationKind, index_name: &str) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![ColumnObservationV3::new(
            "label",
            1,
            "text",
            QualifiedTypeName::new("pg_catalog", "text").unwrap(),
            true,
            None,
        )
        .unwrap()],
    )
    .unwrap()
    .with_indexes(vec![index(index_name)])
    .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events",
        RelationKind::PartitionedTable,
        "events_label_idx",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "events_2026_label_idx",
    )
    .unwrap()
}

fn snapshots() -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
) {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-collation-catalog-identity-v1",
        "2026-09-15T00:00:00Z",
        vec![
            relation(
                "events",
                RelationKind::PartitionedTable,
                "events_label_idx",
            ),
            relation(
                "events_2026",
                RelationKind::Table,
                "events_2026_label_idx",
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
                "events",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events").unwrap(),
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
    (base, relations, indexes)
}

fn identity(encoding: i32) -> CollationCatalogIdentity {
    CollationCatalogIdentity::new("pg_catalog", "C", encoding).unwrap()
}

fn observation(index: IndexPartitionCoordinate, encoding: i32) -> IndexKeyCollationIdentityObservation {
    IndexKeyCollationIdentityObservation::new(index, 1, Some(identity(encoding))).unwrap()
}

#[test]
fn same_qualified_name_with_different_catalog_encoding_is_not_the_same_collation_oid_identity() {
    let (base, relations, indexes) = snapshots();

    let error = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![observation(parent_index(), -1), observation(child_index(), 6)],
    )
    .expect_err(
        "pg_collation is unique by name+encoding+namespace, so two distinct rows must not collapse",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_collation_catalog_identity",
        }
    );
}

#[test]
fn matching_catalog_row_identity_remains_admissible() {
    let (base, relations, indexes) = snapshots();

    let snapshot = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![observation(parent_index(), 6), observation(child_index(), 6)],
    )
    .expect("the same resolved pg_collation row must remain admissible");

    assert!(snapshot.snapshot_digest().starts_with("sha256:"));
    assert_eq!(snapshot.observations().len(), 2);
}
