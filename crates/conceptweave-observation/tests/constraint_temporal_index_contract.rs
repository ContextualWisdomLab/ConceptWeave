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

fn operator_class(schema_name: &str, operator_class_name: &str) -> QualifiedOperatorClassName {
    QualifiedOperatorClassName::new(schema_name, operator_class_name)
        .expect("operator-class coordinate is valid")
}

fn key_semantics(access_method: &str) -> Vec<IndexKeySemantics> {
    let integer_opclass = if access_method == "gist" {
        operator_class("public", "gist_int8_ops")
    } else {
        operator_class("pg_catalog", "int8_ops")
    };
    vec![
        IndexKeySemantics::new(1, None, integer_opclass, 0)
            .expect("identifier key semantics are valid"),
        IndexKeySemantics::new(2, None, operator_class("pg_catalog", "range_ops"), 0)
            .expect("period key semantics are valid"),
    ]
}

fn constraint(primary: bool) -> TableConstraintObservation {
    let columns = vec!["document_id".to_owned(), "valid_during".to_owned()];
    if primary {
        TableConstraintObservation::PrimaryKey(
            PrimaryKeyObservation::new("document_temporal_key", columns)
                .expect("primary-key fixture is valid"),
        )
    } else {
        TableConstraintObservation::Unique(
            UniqueConstraintObservation::new("document_temporal_key", columns)
                .expect("unique fixture is valid"),
        )
    }
}

fn backing_index(
    primary: bool,
    exclusion: bool,
    access_method: &str,
) -> Result<IndexObservation, ObservationError> {
    Ok(IndexObservation::new(
        "document_temporal_key",
        true,
        Some(false),
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "document_id")
                .expect("first key fixture is valid"),
            IndexAttributeObservation::new(2, IndexAttributeKind::Key, "valid_during")
                .expect("period key fixture is valid"),
        ],
        Vec::new(),
    )?
    .with_access_method(access_method)
    .with_key_semantics(key_semantics(access_method))?
    .with_catalog_flags(IndexCatalogFlags::new(
        primary, exclusion, true, false, false, false,
    ))?
    .with_ready(true)
    .with_valid(true)
    .with_live(true))
}

fn relation(
    primary: bool,
    exclusion: bool,
    access_method: &str,
) -> Result<RelationObservation, ObservationError> {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "document_id",
                1,
                "bigint",
                catalog_type("int8"),
                false,
                None,
            )
            .expect("identifier column fixture is valid"),
            ColumnObservationV3::new(
                "valid_during",
                2,
                "tstzrange",
                catalog_type("tstzrange"),
                false,
                None,
            )
            .expect("period column fixture is valid"),
        ],
    )?
    .with_constraints(vec![constraint(primary)])?
    .with_indexes(vec![backing_index(primary, exclusion, access_method)?])
}

fn snapshot(
    primary: bool,
    exclusion: bool,
    access_method: &str,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T19:41:00Z",
        vec![relation(primary, exclusion, access_method)?],
        Vec::new(),
        Vec::new(),
        vec![
            ConstraintTimingObservation::new(
                "public",
                "document",
                RelationKind::Table,
                "document_temporal_key",
                ConstraintDeferrability::NotDeferrable,
            )
            .expect("constraint timing fixture is valid"),
        ],
    )
}

fn assert_backing_index_error(result: Result<PostgresSchemaSnapshotV3, ObservationError>) {
    assert_eq!(
        result.expect_err("a temporal key cannot be backed by a non-GiST exclusion index"),
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn primary_temporal_catalog_flags_require_gist_backing_index() {
    assert_backing_index_error(snapshot(true, true, "btree"));
}

#[test]
fn unique_temporal_catalog_flags_require_gist_backing_index() {
    assert_backing_index_error(snapshot(false, true, "btree"));
}

#[test]
fn temporal_gist_and_ordinary_btree_controls_remain_admissible() {
    snapshot(true, true, "gist")
        .expect("PostgreSQL WITHOUT OVERLAPS primary-key catalog shape is coherent");
    snapshot(false, false, "btree").expect("ordinary unique B-tree catalog shape remains coherent");
}
