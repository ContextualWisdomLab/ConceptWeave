use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintTimingObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics, IndexObservation,
    ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
};
use conceptweave_relation_partition::{
    IndexConstraintParentageCoordinate, IndexConstraintParentageObservation,
    IndexConstraintParentageSnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
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

fn key_index(name: &str, primary: bool) -> IndexObservation {
    IndexObservation::new(
        name,
        true,
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
    .with_catalog_flags(IndexCatalogFlags::new(
        primary, false, true, false, false, false,
    ))
    .unwrap()
    .with_ready(true)
    .with_valid(true)
    .with_live(true)
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    primary_constraint: bool,
) -> RelationObservation {
    let relation = RelationObservation::new(
        "public",
        name,
        kind,
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
    .unwrap();

    let relation = if primary_constraint {
        relation
            .with_constraints(vec![TableConstraintObservation::PrimaryKey(
                PrimaryKeyObservation::new(index_name, vec!["id".to_owned()]).unwrap(),
            )])
            .unwrap()
    } else {
        relation
    };

    relation
        .with_indexes(vec![key_index(index_name, primary_constraint)])
        .unwrap()
}

fn base_snapshot(parent_constraint: bool) -> PostgresSchemaSnapshotV3 {
    let mut timings = Vec::new();
    if parent_constraint {
        timings.push(
            ConstraintTimingObservation::new(
                "public",
                "events",
                RelationKind::PartitionedTable,
                "events_pkey",
                ConstraintDeferrability::NotDeferrable,
            )
            .unwrap(),
        );
    }
    timings.push(
        ConstraintTimingObservation::new(
            "public",
            "events_2026",
            RelationKind::Table,
            "events_2026_pkey",
            ConstraintDeferrability::NotDeferrable,
        )
        .unwrap(),
    );

    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &authorized_source(),
        "extractor-index-constraint-parentage-v1",
        "2026-09-16T09:41:00Z",
        vec![
            relation(
                "events",
                RelationKind::PartitionedTable,
                "events_pkey",
                parent_constraint,
            ),
            relation(
                "events_2026",
                RelationKind::Table,
                "events_2026_pkey",
                true,
            ),
        ],
        vec![],
        vec![],
        timings,
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

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events",
        RelationKind::PartitionedTable,
        "events_pkey",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "events_2026_pkey",
    )
    .unwrap()
}

fn index_partitions(
    base: &PostgresSchemaSnapshotV3,
    relations: &RelationPartitionSnapshot,
) -> IndexPartitionSnapshot {
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
    .unwrap()
}

fn parent_constraint() -> IndexConstraintParentageCoordinate {
    IndexConstraintParentageCoordinate::new(
        "public",
        "events",
        RelationKind::PartitionedTable,
        "events_pkey",
    )
    .unwrap()
}

fn child_constraint() -> IndexConstraintParentageCoordinate {
    IndexConstraintParentageCoordinate::new(
        "public",
        "events_2026",
        RelationKind::Table,
        "events_2026_pkey",
    )
    .unwrap()
}

#[test]
fn attached_constraint_backed_child_requires_exact_parent_constraint() {
    let base = base_snapshot(true);
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);

    let error = IndexConstraintParentageSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexConstraintParentageObservation::root(parent_constraint()).unwrap(),
            IndexConstraintParentageObservation::root(child_constraint()).unwrap(),
        ],
    )
    .expect_err("an attached child constraint cannot erase pg_constraint.conparentid");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_constraint_parentage",
        }
    );
}

#[test]
fn exact_constraint_parentage_is_admitted_and_receipted() {
    let base = base_snapshot(true);
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);

    let snapshot = IndexConstraintParentageSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![
            IndexConstraintParentageObservation::root(parent_constraint()).unwrap(),
            IndexConstraintParentageObservation::partition(
                child_constraint(),
                parent_constraint(),
            )
            .unwrap(),
        ],
    )
    .expect("exact pg_constraint.conparentid parentage must be admitted");

    let receipt = snapshot
        .source_receipt(child_constraint())
        .expect("observed child constraint parentage must issue provenance");
    assert_eq!(receipt.location().parent_constraint(), Some(&parent_constraint()));
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn child_constraint_below_nonconstraint_parent_index_remains_local() {
    let base = base_snapshot(false);
    let relations = relation_partitions(&base);
    let indexes = index_partitions(&base, &relations);

    let snapshot = IndexConstraintParentageSnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![IndexConstraintParentageObservation::root(child_constraint()).unwrap()],
    )
    .expect(
        "PostgreSQL permits a constraint-backed child index below a non-constraint parent index without conparentid",
    );

    assert_eq!(snapshot.observations().len(), 1);
    assert!(snapshot.observations()[0].parent_constraint().is_none());
}
