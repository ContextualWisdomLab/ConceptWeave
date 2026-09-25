//! Structural RED for PostgreSQL 18 relation-level replica-identity mode.
//!
//! `pg_class.relreplident` is source truth distinct from `pg_index.indisreplident`.
//! In particular, PostgreSQL may retain `relreplident = 'i'` after the selected
//! identity index is dropped, so relation mode must not be reconstructed from
//! the surviving index set.

const REPRESENTATION_V3: &str = include_str!("../src/representation_v3.rs");

#[test]
fn relation_replica_identity_mode_is_first_class_source_evidence() {
    assert!(
        REPRESENTATION_V3.contains("pub enum ReplicaIdentityMode"),
        "relation evidence needs an explicit PostgreSQL relreplident value object"
    );
    for variant in ["Default", "Nothing", "Full", "Index"] {
        assert!(
            REPRESENTATION_V3.contains(variant),
            "ReplicaIdentityMode must preserve PostgreSQL mode {variant}"
        );
    }
    assert!(
        REPRESENTATION_V3.contains("replica_identity_mode: Option<ReplicaIdentityMode>"),
        "RelationObservation must distinguish unobserved relreplident from every observed mode"
    );
    assert!(
        REPRESENTATION_V3.contains("with_replica_identity_mode"),
        "the adapter needs a bounded way to attach exact pg_class.relreplident evidence"
    );
    assert!(
        REPRESENTATION_V3.contains("replica_identity_mode()"),
        "relation-level replica identity must be externally inspectable and digestible"
    );
}

#[test]
fn relation_replica_identity_mode_participates_in_snapshot_digest() {
    let digest_start = REPRESENTATION_V3
        .find("fn compute_snapshot_digest_v3")
        .expect("v3 snapshot digest function exists");
    let digest_source = &REPRESENTATION_V3[digest_start..];
    assert!(
        digest_source.contains("relation.replica_identity_mode()"),
        "relreplident must participate in governed snapshot identity rather than remain presentation metadata"
    );
}

#[test]
fn index_mode_is_not_defined_as_presence_of_an_identity_index() {
    assert!(
        !REPRESENTATION_V3.contains("replica_identity_mode() == Some(ReplicaIdentityMode::Index) && relation.indexes().iter().all"),
        "PostgreSQL permits relreplident='i' to survive index drop; do not require an indisreplident row merely from relation mode"
    );
}
