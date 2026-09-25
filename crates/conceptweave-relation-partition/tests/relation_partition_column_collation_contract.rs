use conceptweave_observation::{
    ColumnCollationObservation, ColumnObservationV3, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedCollationName, QualifiedTypeName, RelationKind, RelationObservation,
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
}

fn relations() -> Vec<RelationObservation> {
    vec![
        relation("labels", RelationKind::PartitionedTable),
        relation("labels_2026", RelationKind::Table),
    ]
}

fn memberships() -> Vec<RelationPartitionObservation> {
    vec![
        RelationPartitionObservation::non_partition(
            "public",
            "labels",
            RelationKind::PartitionedTable,
        )
        .unwrap(),
        RelationPartitionObservation::partition(
            "public",
            "labels_2026",
            RelationKind::Table,
            PartitionParentRelationCoordinate::new("public", "labels").unwrap(),
            false,
        )
        .unwrap(),
    ]
}

fn collation(
    relation_name: &str,
    relation_kind: RelationKind,
    collation_name: &str,
) -> ColumnCollationObservation {
    ColumnCollationObservation::collatable(
        "public",
        relation_name,
        relation_kind,
        "label",
        QualifiedCollationName::new("pg_catalog", collation_name).unwrap(),
        true,
    )
    .unwrap()
}

fn snapshot(
    parent_collation: &str,
    child_collation: &str,
) -> Result<RelationPartitionSnapshot, ObservationError> {
    let base = PostgresSchemaSnapshotV3::new_with_column_collations(
        &authorized_source(),
        "extractor-partition-column-collation-v1",
        "2026-09-16T04:42:56Z",
        relations(),
        vec![],
        vec![],
        vec![
            collation("labels", RelationKind::PartitionedTable, parent_collation),
            collation("labels_2026", RelationKind::Table, child_collation),
        ],
    )?;
    RelationPartitionSnapshot::new(&base, memberships())
}

#[test]
fn partition_column_collation_must_match_parent() {
    let error = snapshot("C", "POSIX")
        .expect_err("partition pg_attribute.attcollation must match the partitioned-table parent");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_collation",
        }
    );

    snapshot("C", "C").expect("matching partition column collation remains valid");
}
