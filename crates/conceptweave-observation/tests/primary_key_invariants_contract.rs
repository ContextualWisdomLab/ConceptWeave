use conceptweave_observation::{
    ColumnObservationV3, ObservationError, PostgresSchemaSnapshotV3, PrimaryKeyObservation,
    QualifiedTypeName, RelationKind, RelationObservation, TableConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn primary_key(name: &str, column_name: &str) -> TableConstraintObservation {
    TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(name, vec![column_name.to_owned()])
            .expect("primary-key fixture is valid"),
    )
}

fn relation(
    nullable: bool,
    constraints: Vec<TableConstraintObservation>,
) -> RelationObservation {
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
                nullable,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_constraints(constraints)
    .expect("relation-local constraint structure is valid before snapshot admission")
}

fn snapshot(relation: RelationObservation) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T16:12:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
}

#[test]
fn one_primary_key_on_nonnullable_columns_is_admitted() {
    assert!(
        snapshot(relation(
            false,
            vec![primary_key("document_pkey", "document_id")],
        ))
        .is_ok(),
        "one primary key over exact NOT NULL column evidence is valid PostgreSQL state"
    );
}

#[test]
fn a_relation_cannot_claim_multiple_primary_keys() {
    let error = snapshot(relation(
        false,
        vec![
            primary_key("document_pkey", "document_id"),
            primary_key("document_pkey_second", "document_id"),
        ],
    ))
    .expect_err("PostgreSQL permits at most one primary key per table");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "primary_key_cardinality",
        }
    );
}

#[test]
fn primary_key_columns_cannot_be_observed_nullable() {
    let error = snapshot(relation(
        true,
        vec![primary_key("document_pkey", "document_id")],
    ))
    .expect_err("PostgreSQL primary-key columns are necessarily NOT NULL");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "primary_key_nullable_column",
        }
    );
}
