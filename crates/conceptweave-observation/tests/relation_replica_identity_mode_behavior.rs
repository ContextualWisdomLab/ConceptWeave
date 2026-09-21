use conceptweave_observation::{
    ColumnObservationV3, PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind,
    RelationObservation, ReplicaIdentityMode,
};

mod support;

fn relation() -> RelationObservation {
    RelationObservation::new(
        "public",
        "event_record",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "event_key",
                1,
                "uuid",
                QualifiedTypeName::new("pg_catalog", "uuid")
                    .expect("catalog type coordinate is valid"),
                false,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
}

fn snapshot(relation: RelationObservation) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-22T00:00:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("snapshot fixture is valid")
}

#[test]
fn relation_replica_identity_mode_round_trips_without_conflating_unobserved() {
    assert_eq!(relation().replica_identity_mode(), None);

    let cases = [
        ReplicaIdentityMode::Default,
        ReplicaIdentityMode::Nothing,
        ReplicaIdentityMode::Full,
        ReplicaIdentityMode::Index,
    ];
    for mode in cases {
        let observed = relation().with_replica_identity_mode(mode);
        assert_eq!(observed.replica_identity_mode(), Some(mode));
    }
}

#[test]
fn observed_relation_replica_identity_mode_is_governed_digest_material() {
    let mut digests = vec![snapshot(relation()).snapshot_digest().to_owned()];
    for mode in [
        ReplicaIdentityMode::Default,
        ReplicaIdentityMode::Nothing,
        ReplicaIdentityMode::Full,
        ReplicaIdentityMode::Index,
    ] {
        digests.push(
            snapshot(relation().with_replica_identity_mode(mode))
                .snapshot_digest()
                .to_owned(),
        );
    }

    for left in 0..digests.len() {
        for right in left + 1..digests.len() {
            assert_ne!(
                digests[left], digests[right],
                "unobserved and each observed pg_class.relreplident mode must have distinct governed identity"
            );
        }
    }
}

#[test]
fn index_mode_remains_representable_after_identity_index_drop() {
    let relation = relation().with_replica_identity_mode(ReplicaIdentityMode::Index);
    assert!(relation.indexes().is_empty());

    let snapshot = snapshot(relation);
    let observed = &snapshot.relations()[0];
    assert_eq!(
        observed.replica_identity_mode(),
        Some(ReplicaIdentityMode::Index),
        "PostgreSQL preserves relreplident='i' after the selected index is dropped"
    );
    assert!(observed.indexes().is_empty());
}
