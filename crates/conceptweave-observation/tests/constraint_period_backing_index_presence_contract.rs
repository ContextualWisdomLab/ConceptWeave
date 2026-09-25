use conceptweave_observation::{
    ColumnObservationV3, ConstraintPeriodObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexKeySemantics, IndexObservation,
    ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation, QualifiedOperatorClassName,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
    TypeKindObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn operator_class(schema_name: &str, operator_class_name: &str) -> QualifiedOperatorClassName {
    QualifiedOperatorClassName::new(schema_name, operator_class_name)
        .expect("operator-class coordinate is valid")
}

fn key_semantics(key_columns: &[&str]) -> Vec<IndexKeySemantics> {
    key_columns
        .iter()
        .enumerate()
        .map(|(index, column_name)| {
            let operator_class = match *column_name {
                "document_id" => operator_class("public", "gist_int8_ops"),
                "valid_during" => operator_class("pg_catalog", "range_ops"),
                _ => panic!("unsupported fixture key column {column_name}"),
            };
            IndexKeySemantics::new(
                u32::try_from(index + 1).expect("fixture position fits u32"),
                None,
                operator_class,
                0,
            )
            .expect("key-semantics fixture is valid")
        })
        .collect()
}

fn column(name: &str, position: u32, data_type: &str, type_name: &str) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        position,
        data_type,
        catalog_type(type_name),
        false,
        None,
    )
    .expect("column fixture is valid")
}

fn temporal_type_kinds() -> Vec<TypeKindObservation> {
    vec![
        TypeKindObservation::range(catalog_type("tstzrange"), catalog_type("tstzmultirange")),
        TypeKindObservation::multirange(catalog_type("tstzmultirange"), catalog_type("tstzrange")),
    ]
}

fn temporal_key_relation(index: Option<IndexObservation>) -> RelationObservation {
    let relation = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            column("document_id", 1, "bigint", "int8"),
            column("valid_during", 2, "tstzrange", "tstzrange"),
        ],
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("temporal primary-key fixture is valid"),
    )])
    .expect("constraint fixture is valid");

    match index {
        None => relation,
        Some(index) => relation
            .with_indexes(vec![index])
            .expect("index fixture is valid"),
    }
}

fn temporal_backing_index_for_columns(
    with_catalog_flags: bool,
    key_columns: &[&str],
) -> IndexObservation {
    let attributes = key_columns
        .iter()
        .enumerate()
        .map(|(index, column_name)| {
            IndexAttributeObservation::new(
                u32::try_from(index + 1).expect("fixture position fits u32"),
                IndexAttributeKind::Key,
                *column_name,
            )
            .expect("index attribute fixture is valid")
        })
        .collect();
    let index = IndexObservation::new(
        "document_temporal_key",
        true,
        Some(false),
        attributes,
        Vec::new(),
    )
    .expect("backing-index fixture is structurally valid")
    .with_access_method("gist")
    .with_key_semantics(key_semantics(key_columns))
    .expect("one semantic record matches each temporal key position");

    if with_catalog_flags {
        index
            .with_catalog_flags(IndexCatalogFlags::new(
                true, true, true, false, false, false,
            ))
            .expect("catalog-flag fixture is coherent")
    } else {
        index
    }
}

fn temporal_backing_index(with_catalog_flags: bool) -> IndexObservation {
    temporal_backing_index_for_columns(with_catalog_flags, &["document_id", "valid_during"])
}

fn temporal_backing_index_with_lifecycle(ready: bool, valid: bool, live: bool) -> IndexObservation {
    temporal_backing_index(true)
        .with_ready(ready)
        .with_valid(valid)
        .with_live(live)
}

fn temporal_period() -> ConstraintPeriodObservation {
    ConstraintPeriodObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        true,
    )
    .expect("constraint-period fixture is valid")
    .with_exclusion_operator_signatures(vec![
        (
            1,
            "pg_catalog".to_owned(),
            "=".to_owned(),
            catalog_type("int8"),
            catalog_type("int8"),
        ),
        (
            2,
            "pg_catalog".to_owned(),
            "&&".to_owned(),
            catalog_type("tstzrange"),
            catalog_type("tstzrange"),
        ),
    ])
    .expect("temporal exclusion operator fixture is valid")
}

fn snapshot(relation: RelationObservation) -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T07:35:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot fixture is valid")
    .with_observed_type_kinds(temporal_type_kinds())
    .expect("temporal type-kind evidence fixture is coherent")
}

#[test]
fn temporal_key_rejects_missing_same_name_backing_index_evidence() {
    let error = snapshot(temporal_key_relation(None))
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect_err("WITHOUT OVERLAPS cannot be governed without its same-name backing index");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_backing_index",
        }
    );
}

#[test]
fn temporal_key_rejects_backing_index_without_material_catalog_flags() {
    let index = temporal_backing_index(false)
        .with_ready(true)
        .with_valid(true)
        .with_live(true);
    let error = snapshot(temporal_key_relation(Some(index)))
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect_err("WITHOUT OVERLAPS cannot be governed without material pg_index flags");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_backing_index",
        }
    );
}

#[test]
fn temporal_key_rejects_same_name_gist_exclusion_with_mismatched_key_shape() {
    let mismatched = temporal_backing_index_for_columns(true, &["valid_during", "document_id"])
        .with_ready(true)
        .with_valid(true)
        .with_live(true);
    let error = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T07:35:00Z",
        vec![temporal_key_relation(Some(mismatched))],
        Vec::new(),
        Vec::new(),
    )
    .expect_err("a primary key's backing index must preserve constrained-column order");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn temporal_key_rejects_explicitly_unusable_backing_index_lifecycle() {
    for (state_name, index) in [
        (
            "not ready",
            temporal_backing_index_with_lifecycle(false, true, true),
        ),
        (
            "not valid",
            temporal_backing_index_with_lifecycle(true, false, true),
        ),
        (
            "not live",
            temporal_backing_index_with_lifecycle(true, true, false),
        ),
    ] {
        let error = snapshot(temporal_key_relation(Some(index)))
            .with_observed_constraint_periods(vec![temporal_period()])
            .expect_err(state_name);

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "constraint_period_backing_index",
            }
        );
    }
}

#[test]
fn temporal_key_rejects_unobserved_backing_index_lifecycle() {
    let error = snapshot(temporal_key_relation(Some(temporal_backing_index(true))))
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect_err(
            "WITHOUT OVERLAPS backing evidence must prove ready, valid, and live lifecycle state",
        );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_backing_index",
        }
    );
}

#[test]
fn temporal_key_accepts_explicitly_ready_valid_live_backing_index() {
    let accepted = snapshot(temporal_key_relation(Some(
        temporal_backing_index_with_lifecycle(true, true, true),
    )))
    .with_observed_constraint_periods(vec![temporal_period()])
    .expect("usable WITHOUT OVERLAPS backing-index evidence is admissible");

    assert!(
        accepted
            .constraint_periods()
            .expect("period family was explicitly observed")[0]
            .has_period_semantics()
    );
}
