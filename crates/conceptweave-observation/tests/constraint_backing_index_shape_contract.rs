use conceptweave_observation::{
    ColumnObservationV3, ConstraintDeferrability, ConstraintTimingObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
    TableConstraintObservation, UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn columns() -> Vec<ColumnObservationV3> {
    [
        ("document_id", 1, "bigint", "int8"),
        ("tenant_id", 2, "bigint", "int8"),
        ("payload", 3, "text", "text"),
    ]
    .into_iter()
    .map(|(name, ordinal, data_type, type_name)| {
        ColumnObservationV3::new(
            name,
            ordinal,
            data_type,
            catalog_type(type_name),
            false,
            None,
        )
        .expect("column fixture is valid")
    })
    .collect()
}

fn unique_constraint(
    columns: &[&str],
    nulls_not_distinct: Option<bool>,
) -> TableConstraintObservation {
    let constraint = UniqueConstraintObservation::new(
        "document_key",
        columns.iter().map(|column| (*column).to_owned()).collect(),
    )
    .expect("unique fixture is valid");
    TableConstraintObservation::Unique(match nulls_not_distinct {
        Some(value) => constraint.with_nulls_not_distinct(value),
        None => constraint,
    })
}

fn simple_key(position: u32, column: &str) -> IndexAttributeObservation {
    IndexAttributeObservation::column(position, IndexAttributeKind::Key, column)
        .expect("simple key fixture is valid")
}

fn expression_key(position: u32, expression: &str) -> IndexAttributeObservation {
    IndexAttributeObservation::expression(position, IndexAttributeKind::Key, expression)
        .expect("expression key fixture is valid")
}

fn include(position: u32, column: &str) -> IndexAttributeObservation {
    IndexAttributeObservation::column(position, IndexAttributeKind::Include, column)
        .expect("include fixture is valid")
}

fn backing_index(
    key_attributes: Vec<IndexAttributeObservation>,
    include_attributes: Vec<IndexAttributeObservation>,
    nulls_not_distinct: Option<bool>,
) -> IndexObservation {
    IndexObservation::new(
        "document_key",
        true,
        nulls_not_distinct,
        key_attributes,
        include_attributes,
    )
    .expect("backing-index fixture is structurally valid")
    .with_access_method("btree")
    .with_catalog_flags(IndexCatalogFlags::new(false, false, true, false, false, false))
    .expect("unique non-primary catalog flags are valid")
}

fn snapshot(
    constraint: TableConstraintObservation,
    index: IndexObservation,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    let relation = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        columns(),
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![constraint])
    .expect("constraint fixture is valid")
    .with_indexes(vec![index])
    .expect("index fixture is valid");

    let timing = ConstraintTimingObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_key",
        ConstraintDeferrability::NotDeferrable,
    )
    .expect("timing fixture is valid");

    PostgresSchemaSnapshotV3::new_with_constraint_timings(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T18:42:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        vec![timing],
    )
}

fn assert_backing_index_error(result: Result<PostgresSchemaSnapshotV3, ObservationError>) {
    assert_eq!(
        result.expect_err("impossible key-constraint/index shape must fail closed"),
        ObservationError::InvalidObservationField {
            field: "constraint_backing_index",
        }
    );
}

#[test]
fn key_constraint_requires_exact_backing_index_key_columns_and_order() {
    assert_backing_index_error(snapshot(
        unique_constraint(&["document_id", "tenant_id"], Some(false)),
        backing_index(
            vec![simple_key(1, "tenant_id"), simple_key(2, "document_id")],
            Vec::new(),
            Some(false),
        ),
    ));
}

#[test]
fn key_constraint_rejects_expression_or_partial_backing_index() {
    assert_backing_index_error(snapshot(
        unique_constraint(&["document_id"], Some(false)),
        backing_index(
            vec![expression_key(1, "(document_id + 0)")],
            Vec::new(),
            Some(false),
        ),
    ));

    assert_backing_index_error(snapshot(
        unique_constraint(&["document_id"], Some(false)),
        backing_index(
            vec![simple_key(1, "document_id")],
            Vec::new(),
            Some(false),
        )
        .with_predicate("tenant_id > 0"),
    ));
}

#[test]
fn observed_unique_null_treatment_must_match_backing_index() {
    assert_backing_index_error(snapshot(
        unique_constraint(&["document_id"], Some(true)),
        backing_index(
            vec![simple_key(1, "document_id")],
            Vec::new(),
            Some(false),
        ),
    ));

    assert_backing_index_error(snapshot(
        unique_constraint(&["document_id"], Some(false)),
        backing_index(
            vec![simple_key(1, "document_id")],
            Vec::new(),
            Some(true),
        ),
    ));
}

#[test]
fn coherent_constraint_key_with_include_payload_is_admitted() {
    snapshot(
        unique_constraint(&["document_id"], Some(true)),
        backing_index(
            vec![simple_key(1, "document_id")],
            vec![include(2, "payload")],
            Some(true),
        ),
    )
    .expect("INCLUDE payload is not part of the unique constraint key");
}
