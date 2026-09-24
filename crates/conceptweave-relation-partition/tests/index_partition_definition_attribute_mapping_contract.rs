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

fn columns() -> Vec<ColumnObservationV3> {
    ["account_id", "tenant_id", "payload"]
        .into_iter()
        .enumerate()
        .map(|(offset, name)| {
            ColumnObservationV3::new(
                name,
                u32::try_from(offset + 1).unwrap(),
                "bigint",
                QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
                true,
                None,
            )
            .unwrap()
        })
        .collect()
}

fn index(name: &str, keys: &[&str], includes: &[&str]) -> IndexObservation {
    let key_attributes = keys
        .iter()
        .enumerate()
        .map(|(offset, attribute)| {
            IndexAttributeObservation::new(
                u32::try_from(offset + 1).unwrap(),
                IndexAttributeKind::Key,
                *attribute,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let include_attributes = includes
        .iter()
        .enumerate()
        .map(|(offset, attribute)| {
            IndexAttributeObservation::new(
                u32::try_from(keys.len() + offset + 1).unwrap(),
                IndexAttributeKind::Include,
                *attribute,
            )
            .unwrap()
        })
        .collect::<Vec<_>>();
    let semantics = keys
        .iter()
        .enumerate()
        .map(|(offset, _)| {
            IndexKeySemantics::new(
                u32::try_from(offset + 1).unwrap(),
                None,
                QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(),
                0,
            )
            .unwrap()
        })
        .collect();

    IndexObservation::new(name, false, Some(false), key_attributes, include_attributes)
        .unwrap()
        .with_access_method("btree")
        .with_key_semantics(semantics)
        .unwrap()
        .with_valid(true)
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    keys: &[&str],
    includes: &[&str],
) -> RelationObservation {
    RelationObservation::new("public", name, kind, columns())
        .unwrap()
        .with_indexes(vec![index(index_name, keys, includes)])
        .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events",
        RelationKind::PartitionedTable,
        "events_idx",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "events_2026_idx",
    )
    .unwrap()
}

fn snapshots(
    parent_keys: &[&str],
    parent_includes: &[&str],
    child_keys: &[&str],
    child_includes: &[&str],
) -> (PostgresSchemaSnapshotV3, RelationPartitionSnapshot) {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-attribute-mapping-v1",
        "2026-09-14T15:46:00Z",
        vec![
            relation(
                "events",
                RelationKind::PartitionedTable,
                "events_idx",
                parent_keys,
                parent_includes,
            ),
            relation(
                "events_2026",
                RelationKind::Table,
                "events_2026_idx",
                child_keys,
                child_includes,
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
    (base, relations)
}

fn attached_snapshot(
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

#[test]
fn attached_child_must_map_simple_columns_to_the_same_partition_column() {
    let (base, relations) = snapshots(&["account_id"], &[], &["tenant_id"], &[]);

    let error = attached_snapshot(&base, &relations)
        .expect_err("PostgreSQL CompareIndexInfo rejects differently mapped simple-column slots");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_attribute_mapping",
        }
    );
}

#[test]
fn attached_child_must_preserve_key_and_include_cardinality() {
    let (base, relations) = snapshots(
        &["account_id", "payload"],
        &[],
        &["account_id"],
        &["payload"],
    );

    let error = attached_snapshot(&base, &relations)
        .expect_err("PostgreSQL CompareIndexInfo rejects different key attribute counts");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_attribute_mapping",
        }
    );
}

#[test]
fn matching_modeled_attribute_mapping_remains_admissible() {
    let (base, relations) = snapshots(&["account_id"], &["payload"], &["account_id"], &["payload"]);

    attached_snapshot(&base, &relations)
        .expect("matching key and INCLUDE column mapping must remain admissible");
}
