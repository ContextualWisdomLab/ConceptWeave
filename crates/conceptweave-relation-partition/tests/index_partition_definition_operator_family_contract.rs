use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, QualifiedCollationName,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexKeyOperatorFamilyObservation, IndexOperatorFamilySnapshot, IndexPartitionCoordinate,
    IndexPartitionObservation, IndexPartitionSnapshot, IndexRelationKind,
    PartitionParentRelationCoordinate, QualifiedOperatorFamilyName, RelationPartitionObservation,
    RelationPartitionSnapshot,
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

fn index(name: &str, operator_class_name: &str) -> IndexObservation {
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
            QualifiedOperatorClassName::new("pg_catalog", operator_class_name).unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_valid(true)
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    operator_class_name: &str,
) -> RelationObservation {
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
    .with_indexes(vec![index(index_name, operator_class_name)])
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

fn snapshots(
    parent_operator_class: &str,
    child_operator_class: &str,
) -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
) {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-operator-family-equivalence-v1",
        "2026-09-14T16:40:00Z",
        vec![
            relation(
                "events",
                RelationKind::PartitionedTable,
                "events_label_idx",
                parent_operator_class,
            ),
            relation(
                "events_2026",
                RelationKind::Table,
                "events_2026_label_idx",
                child_operator_class,
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

fn family(
    index: IndexPartitionCoordinate,
    operator_class_name: &str,
    operator_family_name: &str,
) -> IndexKeyOperatorFamilyObservation {
    IndexKeyOperatorFamilyObservation::new(
        index,
        1,
        QualifiedOperatorClassName::new("pg_catalog", operator_class_name).unwrap(),
        QualifiedOperatorFamilyName::new("btree", "pg_catalog", operator_family_name).unwrap(),
    )
    .unwrap()
}

#[test]
fn attached_child_must_preserve_operator_family() {
    let (base, relations, indexes) = snapshots("text_ops", "text_pattern_ops");

    let error = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            family(parent_index(), "text_ops", "text_ops"),
            family(child_index(), "text_pattern_ops", "text_pattern_ops"),
        ],
    )
    .expect_err("PostgreSQL CompareIndexInfo rejects an operator-family mismatch");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_operator_family",
        }
    );
}

#[test]
fn different_operator_classes_in_the_same_family_remain_admissible() {
    let (base, relations, indexes) = snapshots("text_ops", "varchar_ops");

    IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            family(parent_index(), "text_ops", "text_ops"),
            family(child_index(), "varchar_ops", "text_ops"),
        ],
    )
    .expect("PostgreSQL compares operator families, not operator-class names");
}

#[test]
fn operator_family_evidence_must_be_complete_and_bound_to_the_observed_class() {
    let (base, relations, indexes) = snapshots("text_ops", "text_ops");

    let missing = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![family(parent_index(), "text_ops", "text_ops")],
    )
    .expect_err("every bounded key position requires family evidence");
    assert_eq!(
        missing,
        ObservationError::InvalidObservationField {
            field: "index_operator_family_completeness",
        }
    );

    let wrong_class = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            family(parent_index(), "text_pattern_ops", "text_ops"),
            family(child_index(), "text_ops", "text_ops"),
        ],
    )
    .expect_err("family evidence must bind the exact observed operator class");
    assert_eq!(
        wrong_class,
        ObservationError::InvalidObservationField {
            field: "index_operator_family_class_binding",
        }
    );
}
