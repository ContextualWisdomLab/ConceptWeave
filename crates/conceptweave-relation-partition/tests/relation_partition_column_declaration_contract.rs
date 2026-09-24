use conceptweave_observation::{
    ColumnGenerationObservation, ColumnIdentityObservation, ColumnObservationV3, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
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

fn relation(name: &str, kind: RelationKind) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![
            ColumnObservationV3::new(
                "metric_id",
                1,
                "bigint",
                QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
                false,
                None,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn relations() -> Vec<RelationObservation> {
    vec![
        relation("metrics", RelationKind::PartitionedTable),
        relation("metrics_2026", RelationKind::Table),
    ]
}

fn memberships() -> Vec<RelationPartitionObservation> {
    vec![
        RelationPartitionObservation::non_partition(
            "public",
            "metrics",
            RelationKind::PartitionedTable,
        )
        .unwrap(),
        RelationPartitionObservation::partition(
            "public",
            "metrics_2026",
            RelationKind::Table,
            PartitionParentRelationCoordinate::new("public", "metrics").unwrap(),
            false,
        )
        .unwrap(),
    ]
}

fn identity_snapshot(
    parent: ColumnIdentityObservation,
    child: ColumnIdentityObservation,
) -> Result<RelationPartitionSnapshot, ObservationError> {
    let base = PostgresSchemaSnapshotV3::new_with_column_identities(
        &authorized_source(),
        "extractor-partition-column-declaration-v1",
        "2026-09-16T04:20:00Z",
        relations(),
        vec![],
        vec![],
        vec![parent, child],
    )?;
    RelationPartitionSnapshot::new(&base, memberships())
}

fn generation_snapshot(
    parent: ColumnGenerationObservation,
    child: ColumnGenerationObservation,
) -> Result<RelationPartitionSnapshot, ObservationError> {
    let base = PostgresSchemaSnapshotV3::new_with_column_generations(
        &authorized_source(),
        "extractor-partition-column-declaration-v1",
        "2026-09-16T04:20:00Z",
        relations(),
        vec![],
        vec![],
        vec![parent, child],
    )?;
    RelationPartitionSnapshot::new(&base, memberships())
}

fn identity_always(relation_name: &str, kind: RelationKind) -> ColumnIdentityObservation {
    ColumnIdentityObservation::generated_always("public", relation_name, kind, "metric_id").unwrap()
}

fn identity_by_default(relation_name: &str, kind: RelationKind) -> ColumnIdentityObservation {
    ColumnIdentityObservation::generated_by_default("public", relation_name, kind, "metric_id")
        .unwrap()
}

fn generation_stored(relation_name: &str, kind: RelationKind) -> ColumnGenerationObservation {
    ColumnGenerationObservation::stored("public", relation_name, kind, "metric_id").unwrap()
}

fn generation_virtual(relation_name: &str, kind: RelationKind) -> ColumnGenerationObservation {
    ColumnGenerationObservation::virtual_generated("public", relation_name, kind, "metric_id")
        .unwrap()
}

#[test]
fn partition_identity_mode_must_match_parent() {
    let error = identity_snapshot(
        identity_always("metrics", RelationKind::PartitionedTable),
        identity_by_default("metrics_2026", RelationKind::Table),
    )
    .expect_err("partition identity properties must match the partitioned-table parent");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_identity",
        }
    );

    identity_snapshot(
        identity_always("metrics", RelationKind::PartitionedTable),
        identity_always("metrics_2026", RelationKind::Table),
    )
    .expect("matching inherited identity mode remains valid");
}

#[test]
fn partition_generation_kind_must_match_parent() {
    let error = generation_snapshot(
        generation_stored("metrics", RelationKind::PartitionedTable),
        generation_virtual("metrics_2026", RelationKind::Table),
    )
    .expect_err("partition generated-column kind must match the partitioned-table parent");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_generation",
        }
    );

    generation_snapshot(
        generation_stored("metrics", RelationKind::PartitionedTable),
        generation_stored("metrics_2026", RelationKind::Table),
    )
    .expect("matching inherited generated-column kind remains valid");
}
