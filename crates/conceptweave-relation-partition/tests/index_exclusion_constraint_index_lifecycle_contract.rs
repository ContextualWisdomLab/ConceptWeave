use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionConstraintCatalogShapeObservation, IndexExclusionConstraintCatalogShapeSnapshot,
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintForeignActionCodes,
    IndexExclusionConstraintForeignPayloadPresence, IndexExclusionConstraintIndexLifecycleSnapshot,
    IndexExclusionConstraintIndexNameSnapshot, IndexExclusionConstraintIndexNamespaceObservation,
    IndexExclusionConstraintIndexNamespaceSnapshot, IndexExclusionConstraintObservation,
    IndexExclusionConstraintSnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, RelationPartitionObservation,
    RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;
impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, key: &str) -> bool {
        key == "warehouse"
    }

    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "warehouse").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(&self, source: &ResolvedSourceConnection, schemas: &[String]) -> bool {
        source.source_connection_key() == "warehouse"
            && source.connection_policy_binding() == POLICY_BINDING
            && schemas == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source: &ResolvedSourceConnection,
        envelope: ObservationResourceEnvelope,
    ) -> bool {
        source.source_connection_key() == "warehouse"
            && source.connection_policy_binding() == POLICY_BINDING
            && envelope.request_budget().max_schema_count() <= 1
            && envelope.request_budget().max_schema_bytes() <= 256
            && envelope.limits().operation_timeout_ms() <= 1_000
            && envelope.limits().statement_timeout_ms() <= 1_000
            && envelope.limits().max_rows() <= 10
            && envelope.limits().max_bytes() <= 1_024
            && envelope.limits().max_concurrent_queries() <= 1
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

fn coordinate() -> IndexExclusionConstraintCoordinate {
    IndexExclusionConstraintCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn index_coordinate() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "bookings",
        RelationKind::Table,
        "bookings_no_overlap",
    )
    .unwrap()
}

fn index_with_lifecycle(
    ready: Option<bool>,
    valid: Option<bool>,
    live: Option<bool>,
) -> IndexObservation {
    let mut index = IndexObservation::new(
        "bookings_no_overlap",
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops").unwrap(),
        0,
    )
    .unwrap()])
    .unwrap()
    .with_catalog_flags(IndexCatalogFlags::new(false, true, true, false, false, false))
    .unwrap();

    if let Some(value) = ready {
        index = index.with_ready(value);
    }
    if let Some(value) = valid {
        index = index.with_valid(value);
    }
    if let Some(value) = live {
        index = index.with_live(value);
    }
    index
}

fn base_with_index(index: IndexObservation) -> PostgresSchemaSnapshotV3 {
    let relation = RelationObservation::new(
        "public",
        "bookings",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "id",
            1,
            "bigint",
            QualifiedTypeName::new("pg_catalog", "int8").unwrap(),
            false,
            None,
        )
        .unwrap()],
    )
    .unwrap()
    .with_indexes(vec![index])
    .unwrap();

    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-index-lifecycle-v1",
        "2026-09-17T01:30:00Z",
        vec![relation],
        vec![],
        vec![],
    )
    .unwrap()
}

struct Stack {
    base: PostgresSchemaSnapshotV3,
    namespaces: IndexExclusionConstraintIndexNamespaceSnapshot,
}

fn stack(ready: Option<bool>, valid: Option<bool>, live: Option<bool>) -> Stack {
    let base = base_with_index(index_with_lifecycle(ready, valid, live));
    let relations = RelationPartitionSnapshot::new(
        &base,
        vec![RelationPartitionObservation::non_partition(
            "public",
            "bookings",
            RelationKind::Table,
        )
        .unwrap()],
    )
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![IndexPartitionObservation::non_partition(
            index_coordinate(),
            IndexRelationKind::Index,
        )
        .unwrap()],
    )
    .unwrap();
    let constraints = IndexExclusionConstraintSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![IndexExclusionConstraintObservation::root(
            coordinate(),
            index_coordinate(),
        )
        .unwrap()],
    )
    .unwrap();
    let shapes = IndexExclusionConstraintCatalogShapeSnapshot::new(
        &constraints,
        vec![IndexExclusionConstraintCatalogShapeObservation::new(
            coordinate(),
            'x',
            true,
            false,
            IndexExclusionConstraintForeignActionCodes::new(' ', ' ', ' '),
            IndexExclusionConstraintForeignPayloadPresence::new(false, false, [false; 3], false),
            false,
        )
        .unwrap()],
    )
    .unwrap();
    let names = IndexExclusionConstraintIndexNameSnapshot::new(
        &base,
        &relations,
        &indexes,
        &constraints,
        &shapes,
    )
    .unwrap();
    let namespaces = IndexExclusionConstraintIndexNamespaceSnapshot::new(
        &names,
        vec![IndexExclusionConstraintIndexNamespaceObservation::new(
            coordinate(),
            index_coordinate(),
            "public",
        )
        .unwrap()],
    )
    .unwrap();

    Stack { base, namespaces }
}

#[test]
fn exact_backing_index_lifecycle_is_retained_and_receipted() {
    let stack = stack(Some(true), Some(true), Some(true));
    let snapshot = IndexExclusionConstraintIndexLifecycleSnapshot::new(
        &stack.base,
        &stack.namespaces,
    )
    .expect("an ordinary EXCLUDE backing index must be ready, valid, and live");

    let receipt = snapshot.source_receipt(coordinate()).unwrap();
    assert_eq!(receipt.location().backing_index(), &index_coordinate());
    assert!(receipt.location().index_ready());
    assert!(receipt.location().index_valid());
    assert!(receipt.location().index_live());
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/backing-index-lifecycle")
    );
}

#[test]
fn missing_backing_index_lifecycle_evidence_fails_closed() {
    for stack in [
        stack(None, Some(true), Some(true)),
        stack(Some(true), None, Some(true)),
        stack(Some(true), Some(true), None),
    ] {
        let error = IndexExclusionConstraintIndexLifecycleSnapshot::new(
            &stack.base,
            &stack.namespaces,
        )
        .expect_err("missing pg_index lifecycle bits must not be synthesized from constraint state");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "index_exclusion_constraint_index_lifecycle_presence",
            }
        );
    }
}

#[test]
fn non_enforceable_backing_index_lifecycle_fails_closed() {
    for stack in [
        stack(Some(false), Some(true), Some(true)),
        stack(Some(true), Some(false), Some(true)),
        stack(Some(true), Some(true), Some(false)),
    ] {
        let error = IndexExclusionConstraintIndexLifecycleSnapshot::new(
            &stack.base,
            &stack.namespaces,
        )
        .expect_err("ordinary EXCLUDE must not publish against a non-ready, invalid, or dead index");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "index_exclusion_constraint_index_lifecycle_state",
            }
        );
    }
}

#[test]
fn lifecycle_must_bind_the_exact_conindid_backing_index() {
    let stack = stack(Some(true), Some(true), Some(true));
    let unrelated = IndexObservation::new(
        "other_index",
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true);
    let unrelated_base = base_with_index(unrelated);

    let error = IndexExclusionConstraintIndexLifecycleSnapshot::new(
        &unrelated_base,
        &stack.namespaces,
    )
    .expect_err("lifecycle evidence must come from the exact conindid backing index");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_exclusion_constraint_index_lifecycle_backing_index",
        }
    );
}

#[test]
fn unknown_backing_index_lifecycle_receipt_fails_closed() {
    let stack = stack(Some(true), Some(true), Some(true));
    let snapshot = IndexExclusionConstraintIndexLifecycleSnapshot::new(
        &stack.base,
        &stack.namespaces,
    )
    .unwrap();
    let unknown = IndexExclusionConstraintCoordinate::new(
        "public",
        "other",
        RelationKind::Table,
        "other_no_overlap",
    )
    .unwrap();
    assert!(matches!(
        snapshot.source_receipt(unknown),
        Err(ObservationError::UnknownObservationLocation { .. })
    ));
}
