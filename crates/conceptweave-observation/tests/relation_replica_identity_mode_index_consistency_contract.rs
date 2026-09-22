use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
    ReplicaIdentityMode,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn key_semantics() -> IndexKeySemantics {
    IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
            .expect("operator-class fixture is valid"),
        0,
    )
    .expect("key-semantics fixture is valid")
}

fn index(replica_identity: bool) -> IndexObservation {
    IndexObservation::new(
        "document_replica_identity_ix",
        true,
        Some(false),
        vec![IndexAttributeObservation::column(
            1,
            IndexAttributeKind::Key,
            "document_id",
        )
        .expect("column key fixture is valid")],
        Vec::new(),
    )
    .expect("index fixture is structurally valid")
    .with_access_method("btree")
    .with_key_semantics(vec![key_semantics()])
    .expect("one semantic record matches the structural key")
    .with_catalog_flags(IndexCatalogFlags::new(
        false,
        false,
        true,
        false,
        false,
        replica_identity,
    ))
    .expect("catalog flags are structurally constructible")
}

fn relation(
    replica_identity_mode: Option<ReplicaIdentityMode>,
    replica_identity_index: bool,
) -> RelationObservation {
    let relation = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![ColumnObservationV3::new(
            "document_id",
            1,
            "bigint",
            catalog_type("int8"),
            false,
            None,
        )
        .expect("column fixture is valid")],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index(replica_identity_index)])
    .expect("index fixture is valid before aggregate replica-identity validation");

    replica_identity_mode.map_or(relation.clone(), |mode| {
        relation.with_replica_identity_mode(mode)
    })
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-22T05:55:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

fn assert_replica_identity_error(result: Result<PostgresSchemaSnapshotV3, ObservationError>) {
    assert_eq!(
        result.expect_err("contradictory relreplident/indisreplident evidence must fail closed"),
        ObservationError::InvalidObservationField {
            field: "index_replica_identity",
        }
    );
}

#[test]
fn observed_default_mode_rejects_surviving_replica_identity_index_flag() {
    assert_replica_identity_error(snapshot(relation(
        Some(ReplicaIdentityMode::Default),
        true,
    )));
}

#[test]
fn observed_nothing_mode_rejects_surviving_replica_identity_index_flag() {
    assert_replica_identity_error(snapshot(relation(
        Some(ReplicaIdentityMode::Nothing),
        true,
    )));
}

#[test]
fn observed_full_mode_rejects_surviving_replica_identity_index_flag() {
    assert_replica_identity_error(snapshot(relation(Some(ReplicaIdentityMode::Full), true)));
}

#[test]
fn observed_index_mode_accepts_one_surviving_replica_identity_index() {
    snapshot(relation(Some(ReplicaIdentityMode::Index), true))
        .expect("INDEX mode may own one surviving chosen replica-identity index");
}

#[test]
fn observed_index_mode_accepts_zero_surviving_replica_identity_indexes() {
    snapshot(relation(Some(ReplicaIdentityMode::Index), false)).expect(
        "PostgreSQL may retain relreplident=INDEX after its chosen identity index disappears",
    );
}

#[test]
fn unobserved_relation_mode_does_not_invent_a_cross_catalog_contradiction() {
    snapshot(relation(None, true)).expect(
        "legacy evidence without relreplident observation must not be reinterpreted as non-INDEX",
    );
}

#[test]
fn observed_non_index_modes_without_a_chosen_index_remain_admissible() {
    for mode in [
        ReplicaIdentityMode::Default,
        ReplicaIdentityMode::Nothing,
        ReplicaIdentityMode::Full,
    ] {
        snapshot(relation(Some(mode), false))
            .expect("non-INDEX relation modes remain coherent without indisreplident");
    }
}
