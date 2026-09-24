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

fn index_with_uniqueness(name: &str, valid: bool, unique: bool) -> IndexObservation {
    IndexObservation::new(
        name,
        unique,
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
    .with_valid(valid)
}

fn index(name: &str, valid: bool) -> IndexObservation {
    index_with_uniqueness(name, valid, false)
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    index_valid: bool,
) -> RelationObservation {
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
    .with_indexes(vec![index(index_name, index_valid)])
    .unwrap()
}

fn foreign_relation(name: &str) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        RelationKind::ForeignTable,
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
}

fn base_snapshot(parent_valid: bool, child_valid: bool) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-parent-validity-v1",
        "2026-09-14T15:00:00Z",
        vec![
            relation(
                "events",
                RelationKind::PartitionedTable,
                "events_id_idx",
                parent_valid,
            ),
            relation(
                "events_2026",
                RelationKind::Table,
                "events_2026_id_idx",
                child_valid,
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap()
}

fn foreign_partition_base_snapshot(parent_unique: bool) -> PostgresSchemaSnapshotV3 {
    let parent = RelationObservation::new(
        "public",
        "events",
        RelationKind::PartitionedTable,
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
    .with_indexes(vec![index_with_uniqueness(
        "events_id_idx",
        true,
        parent_unique,
    )])
    .unwrap();

    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-parent-validity-foreign-v1",
        "2026-09-16T00:30:00Z",
        vec![parent, foreign_relation("events_remote")],
        vec![],
        vec![],
    )
    .unwrap()
}

fn relation_partitions(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
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

fn foreign_relation_partitions(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
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
                "events_remote",
                RelationKind::ForeignTable,
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

fn local_index_topology(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> Result<IndexPartitionSnapshot, ObservationError> {
    IndexPartitionSnapshot::new(
        base,
        relations,
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
}

fn attached_local_index_topology(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> Result<IndexPartitionSnapshot, ObservationError> {
    IndexPartitionSnapshot::new(
        base,
        relations,
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
}

fn foreign_partition_index_topology(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> Result<IndexPartitionSnapshot, ObservationError> {
    IndexPartitionSnapshot::new(
        base,
        relations,
        vec![
            IndexPartitionObservation::non_partition(
                parent_index(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
        ],
    )
}

#[test]
fn valid_partitioned_index_requires_attached_child_for_each_direct_partition() {
    let base = base_snapshot(true, true);
    let relations = relation_partitions(&base);

    let error = local_index_topology(&base, &relations)
        .expect_err("a valid partitioned index cannot omit a direct partition child attachment");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_parent_validity",
        }
    );
}

#[test]
fn valid_partitioned_index_requires_attached_child_to_be_valid() {
    let base = base_snapshot(true, false);
    let relations = relation_partitions(&base);

    let error = attached_local_index_topology(&base, &relations)
        .expect_err("a valid partitioned index cannot have an attached invalid child index");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_child_validity",
        }
    );
}

#[test]
fn valid_partitioned_index_accepts_attached_valid_child() {
    let base = base_snapshot(true, true);
    let relations = relation_partitions(&base);

    let topology = attached_local_index_topology(&base, &relations)
        .expect("a valid partitioned index may attach a valid child index");
    assert_eq!(topology.observations().len(), 2);
}

#[test]
fn invalid_partitioned_index_can_stage_local_child_before_attachment() {
    let base = base_snapshot(false, true);
    let relations = relation_partitions(&base);

    let staged = local_index_topology(&base, &relations)
        .expect("CREATE INDEX ON ONLY leaves the parent invalid while child indexes are staged");
    assert_eq!(staged.observations().len(), 2);
}

#[test]
fn valid_non_unique_partitioned_index_skips_foreign_partition_child_index() {
    let base = foreign_partition_base_snapshot(false);
    let relations = foreign_relation_partitions(&base);

    let topology = foreign_partition_index_topology(&base, &relations).expect(
        "PostgreSQL skips foreign-table partitions when building a regular partitioned index",
    );
    assert_eq!(topology.observations().len(), 1);
}

#[test]
fn unique_partitioned_index_rejects_foreign_partition() {
    let base = foreign_partition_base_snapshot(true);
    let relations = foreign_relation_partitions(&base);

    let error = foreign_partition_index_topology(&base, &relations)
        .expect_err("PostgreSQL rejects unique partitioned indexes over foreign-table partitions");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_foreign_partition_unique",
        }
    );
}
