use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintTimingObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics, IndexObservation,
    ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
    UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn backing_index(
    name: &str,
    primary: bool,
    immediate: bool,
) -> Result<IndexObservation, ObservationError> {
    IndexObservation::new(
        name,
        true,
        Some(false),
        vec![IndexAttributeObservation::new(
            1,
            IndexAttributeKind::Key,
            "document_id",
        )
        .expect("key fixture is valid")],
        Vec::new(),
    )?
    .with_access_method("btree")
    .with_key_semantics(vec![IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
            .expect("operator-class fixture is valid"),
        0,
    )
    .expect("key semantics fixture is valid")])?
    .with_catalog_flags(IndexCatalogFlags::new(
        primary,
        false,
        immediate,
        false,
        false,
        false,
    ))
}

fn relation(
    constraint: TableConstraintObservation,
    indexes: Vec<IndexObservation>,
) -> RelationObservation {
    RelationObservation::new(
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
    .with_constraints(vec![constraint])
    .expect("constraint fixture is valid")
    .with_indexes(indexes)
    .expect("index fixture is structurally valid")
}

fn timing(
    constraint_name: &str,
    deferrability: ConstraintDeferrability,
) -> ConstraintTimingObservation {
    ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        constraint_name,
        deferrability,
    )
    .expect("constraint timing fixture is valid")
}

fn snapshot(
    relation: RelationObservation,
    timing: ConstraintTimingObservation,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T16:38:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        vec![timing],
    )
}

fn primary_key() -> TableConstraintObservation {
    TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new("document_pkey", vec!["document_id".to_owned()])
            .expect("primary-key fixture is valid"),
    )
}

fn unique_key() -> TableConstraintObservation {
    TableConstraintObservation::Unique(
        UniqueConstraintObservation::new("document_id_key", vec!["document_id".to_owned()])
            .expect("unique fixture is valid"),
    )
}

fn assert_backing_index_error(result: Result<PostgresSchemaSnapshotV3, ObservationError>) {
    assert_eq!(
        result.expect_err("contradictory key-constraint/index evidence must fail closed"),
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn observed_primary_key_requires_same_name_primary_unique_backing_index() {
    assert_backing_index_error(snapshot(
        relation(primary_key(), Vec::new()),
        timing("document_pkey", ConstraintDeferrability::NotDeferrable),
    ));

    assert_backing_index_error(snapshot(
        relation(
            primary_key(),
            vec![backing_index("document_pkey", false, true)
                .expect("non-primary unique index is structurally valid")],
        ),
        timing("document_pkey", ConstraintDeferrability::NotDeferrable),
    ));
}

#[test]
fn observed_unique_constraint_requires_same_name_unique_nonprimary_backing_index() {
    assert_backing_index_error(snapshot(
        relation(
            unique_key(),
            vec![backing_index("other_unique_index", false, true)
                .expect("differently named unique index is structurally valid")],
        ),
        timing("document_id_key", ConstraintDeferrability::NotDeferrable),
    ));

    assert_backing_index_error(snapshot(
        relation(
            unique_key(),
            vec![backing_index("document_id_key", true, true)
                .expect("primary unique index is structurally valid")],
        ),
        timing("document_id_key", ConstraintDeferrability::NotDeferrable),
    ));
}

#[test]
fn deferrable_key_constraint_requires_nonimmediate_backing_index() {
    assert_backing_index_error(snapshot(
        relation(
            primary_key(),
            vec![backing_index("document_pkey", true, true)
                .expect("immediate primary index is structurally valid")],
        ),
        timing(
            "document_pkey",
            ConstraintDeferrability::InitiallyImmediate,
        ),
    ));

    assert_backing_index_error(snapshot(
        relation(
            unique_key(),
            vec![backing_index("document_id_key", false, true)
                .expect("immediate unique index is structurally valid")],
        ),
        timing(
            "document_id_key",
            ConstraintDeferrability::InitiallyDeferred,
        ),
    ));
}

#[test]
fn observed_key_constraint_rejects_explicitly_unusable_backing_index_lifecycle() {
    for (state_name, index) in [
        (
            "not ready",
            backing_index("document_pkey", true, true)
                .expect("primary index fixture is valid")
                .with_ready(false)
                .with_valid(true)
                .with_live(true),
        ),
        (
            "not valid",
            backing_index("document_pkey", true, true)
                .expect("primary index fixture is valid")
                .with_ready(true)
                .with_valid(false)
                .with_live(true),
        ),
        (
            "not live",
            backing_index("document_pkey", true, true)
                .expect("primary index fixture is valid")
                .with_ready(true)
                .with_valid(true)
                .with_live(false),
        ),
    ] {
        let result = snapshot(
            relation(primary_key(), vec![index]),
            timing("document_pkey", ConstraintDeferrability::NotDeferrable),
        );
        assert_eq!(
            result.expect_err(state_name),
            ObservationError::InvalidObservationField {
                field: "constraint_backing_index",
            }
        );
    }
}

#[test]
fn coherent_key_constraint_and_backing_index_evidence_is_admitted() {
    snapshot(
        relation(
            primary_key(),
            vec![backing_index("document_pkey", true, true)
                .expect("nondeferrable primary index fixture is valid")],
        ),
        timing("document_pkey", ConstraintDeferrability::NotDeferrable),
    )
    .expect("nondeferrable primary key must bind an immediate primary unique index");

    snapshot(
        relation(
            unique_key(),
            vec![backing_index("document_id_key", false, false)
                .expect("deferrable unique index fixture is valid")],
        ),
        timing(
            "document_id_key",
            ConstraintDeferrability::InitiallyDeferred,
        ),
    )
    .expect("deferrable unique constraint must bind a non-immediate unique index");

    snapshot(
        relation(
            primary_key(),
            vec![backing_index("document_pkey", true, true)
                .expect("primary index fixture is valid")
                .with_ready(true)
                .with_valid(true)
                .with_live(true)],
        ),
        timing("document_pkey", ConstraintDeferrability::NotDeferrable),
    )
    .expect("explicitly ready, valid and live backing-index evidence is admissible");
}
