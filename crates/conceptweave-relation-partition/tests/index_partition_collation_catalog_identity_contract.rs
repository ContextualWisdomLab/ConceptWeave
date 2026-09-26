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
        vec![
            ColumnObservationV3::new(
                "label",
                1,
                "text",
                QualifiedTypeName::new("pg_catalog", "text").unwrap(),
                true,
                None,
            )
            .unwrap(),
        ],
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
            relation("events", RelationKind::PartitionedTable, "events_label_idx"),
            relation("events_2026", RelationKind::Table, "events_2026_label_idx"),
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

fn observation(
    index: IndexPartitionCoordinate,
    encoding: i32,
) -> IndexKeyCollationIdentityObservation {
    IndexKeyCollationIdentityObservation::new(index, 1, Some(identity(encoding))).unwrap()
}

#[test]
fn catalog_identity_preserves_the_unique_pg_collation_coordinate() {
    let identity = identity(6);
    assert_eq!(identity.schema_name(), "pg_catalog");
    assert_eq!(identity.collation_name(), "C");
    assert_eq!(identity.encoding(), 6);

    let quoted_schema = CollationCatalogIdentity::new(" ", "C", 6).unwrap();
    assert_eq!(quoted_schema.schema_name(), " ");
    let blank_schema = CollationCatalogIdentity::new("", "C", 6)
        .expect_err("collation namespace must be a nonempty catalog coordinate");
    assert_eq!(
        blank_schema,
        ObservationError::InvalidObservationField {
            field: "index_collation_catalog_schema_name",
        }
    );
    let blank_name = CollationCatalogIdentity::new("pg_catalog", "", 6)
        .expect_err("collation name must be a real catalog coordinate");
    assert_eq!(
        blank_name,
        ObservationError::InvalidObservationField {
            field: "index_collation_catalog_name",
        }
    );
}

#[test]
fn key_identity_location_requires_a_real_key_position() {
    let error = IndexKeyCollationIdentityObservation::new(parent_index(), 0, Some(identity(6)))
        .expect_err("index key positions are one-based");
    assert_eq!(error, ObservationError::InvalidOrdinalPosition);
}

#[test]
fn same_qualified_name_with_different_catalog_encoding_is_not_the_same_collation_oid_identity() {
    let (base, relations, indexes) = snapshots();

    let error = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            observation(parent_index(), -1),
            observation(child_index(), 6),
        ],
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
fn catalog_identity_must_bind_to_the_issued_key_collation_name() {
    let (base, relations, indexes) = snapshots();
    let wrong = IndexKeyCollationIdentityObservation::new(
        parent_index(),
        1,
        Some(CollationCatalogIdentity::new("pg_catalog", "POSIX", 6).unwrap()),
    )
    .unwrap();

    let error = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![wrong, observation(child_index(), 6)],
    )
    .expect_err("catalog identity must prove the exact qualified collation already issued in v3");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_collation_catalog_binding",
        }
    );
}

#[test]
fn catalog_identity_family_is_complete_over_bounded_key_semantics() {
    let (base, relations, indexes) = snapshots();

    let error = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![observation(parent_index(), 6)],
    )
    .expect_err("every bounded key-semantic position needs explicit catalog identity evidence");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_collation_catalog_completeness",
        }
    );
}

#[test]
fn matching_catalog_row_identity_remains_admissible_and_receiptable() {
    let (base, relations, indexes) = snapshots();

    let snapshot = IndexPartitionCollationIdentitySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            observation(parent_index(), 6),
            observation(child_index(), 6),
        ],
    )
    .expect("the same resolved pg_collation row must remain admissible");

    assert_eq!(snapshot.source_connection_key(), "warehouse_primary");
    assert_eq!(snapshot.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(snapshot.predecessor_digest(), indexes.snapshot_digest());
    assert!(snapshot.snapshot_digest().starts_with("sha256:"));
    assert_eq!(snapshot.extractor_revision(), indexes.extractor_revision());
    assert_eq!(snapshot.observed_at_utc(), indexes.observed_at_utc());
    assert_eq!(snapshot.observations().len(), 2);

    let receipt = snapshot
        .source_receipt(&child_index(), 1)
        .expect("an observed key must issue exact provenance");
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), snapshot.observed_at_utc());
    assert_eq!(receipt.index(), &child_index());
    assert_eq!(receipt.key_position(), 1);

    let zero = snapshot
        .source_receipt(&child_index(), 0)
        .expect_err("receipt key positions are one-based");
    assert_eq!(zero, ObservationError::InvalidOrdinalPosition);

    let unknown_index = IndexPartitionCoordinate::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "not_observed_idx",
    )
    .unwrap();
    assert!(matches!(
        snapshot.source_receipt(&unknown_index, 1),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
}
