use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexPartitionCoordinate, IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    PartitionParentRelationCoordinate, RelationPartitionObservation, RelationPartitionSnapshot,
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
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
}

fn relation(name: &str, kind: RelationKind, index_name: &str) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![
            ColumnObservationV3::new(
                "id",
                1,
                "bigint",
                QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
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

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-partition-v1",
        "2026-09-14T14:45:00Z",
        vec![
            relation("events", RelationKind::PartitionedTable, "events_id_idx"),
            relation("events_2026", RelationKind::Table, "events_2026_id_idx"),
        ],
        vec![],
        vec![],
    )
    .unwrap()
}

fn relation_partition_snapshot(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
    RelationPartitionSnapshot::new(
        base,
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
    .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events",
        RelationKind::PartitionedTable,
        "events_id_idx",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "events_2026_id_idx",
    )
    .unwrap()
}

fn assert_field(error: ObservationError, expected: &'static str) {
    assert_eq!(
        error,
        ObservationError::InvalidObservationField { field: expected }
    );
}

#[test]
fn attached_child_index_topology_changes_governed_identity() {
    let base = base_snapshot();
    let relations = relation_partition_snapshot(&base);

    let local = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(
                parent_index(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
            IndexPartitionObservation::non_partition(child_index(), IndexRelationKind::Index)
                .unwrap(),
        ],
    )
    .unwrap();

    let attached = IndexPartitionSnapshot::new(
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

    assert_ne!(local.snapshot_digest(), attached.snapshot_digest());
}

#[test]
fn index_partition_parent_must_match_relation_partition_parent() {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-partition-v1",
        "2026-09-14T14:45:00Z",
        vec![
            relation(
                "events_a",
                RelationKind::PartitionedTable,
                "events_a_id_idx",
            ),
            relation(
                "events_b",
                RelationKind::PartitionedTable,
                "events_b_id_idx",
            ),
            relation("events_2026", RelationKind::Table, "events_2026_id_idx"),
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
                "events_a",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::non_partition(
                "public",
                "events_b",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "events_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "events_a").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();

    let error = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(
                IndexPartitionCoordinate::new(
                    "public",
                    "events_a",
                    RelationKind::PartitionedTable,
                    "events_a_id_idx",
                )
                .unwrap(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
            IndexPartitionObservation::non_partition(
                IndexPartitionCoordinate::new(
                    "public",
                    "events_b",
                    RelationKind::PartitionedTable,
                    "events_b_id_idx",
                )
                .unwrap(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
            IndexPartitionObservation::partition(
                IndexPartitionCoordinate::new(
                    "public",
                    "events_2026",
                    RelationKind::Table,
                    "events_2026_id_idx",
                )
                .unwrap(),
                IndexRelationKind::Index,
                IndexPartitionCoordinate::new(
                    "public",
                    "events_b",
                    RelationKind::PartitionedTable,
                    "events_b_id_idx",
                )
                .unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .expect_err("index and table partition parents must agree");
    assert_field(error, "index_partition_relation_parent");
}

#[test]
fn complete_index_family_and_stable_attachment_are_required() {
    let base = base_snapshot();
    let relations = relation_partition_snapshot(&base);

    let incomplete = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(
                parent_index(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
        ],
    )
    .expect_err("every observed index needs relkind/relispartition evidence");
    assert_field(incomplete, "index_partition_completeness");

    let detach = IndexPartitionObservation::partition(
        child_index(),
        IndexRelationKind::Index,
        parent_index(),
        true,
    )
    .expect_err("detach-pending index topology must fail closed");
    assert_field(detach, "index_partition_detach_pending");
}

#[test]
fn receipt_is_bound_to_exact_index_coordinate() {
    let base = base_snapshot();
    let relations = relation_partition_snapshot(&base);
    let governed = IndexPartitionSnapshot::new(
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

    let receipt = governed.source_receipt(child_index()).unwrap();
    assert_eq!(receipt.source_id(), "warehouse_primary");
    assert_eq!(receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(receipt.source_digest(), governed.snapshot_digest());
    assert_eq!(receipt.extractor_revision(), "extractor-index-partition-v1");
    assert_eq!(receipt.observed_at_utc(), "2026-09-14T14:45:00Z");
}
