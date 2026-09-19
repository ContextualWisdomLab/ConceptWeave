use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, IndexTablespace, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn complete_index() -> IndexObservation {
    IndexObservation::new(
        "document_id_ix",
        false,
        Some(false),
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "document_id")
                .expect("key fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("index fixture is valid")
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int8_ops")
                .expect("operator-class fixture is valid"),
            0,
        )
        .expect("key semantics fixture is valid"),
    ])
    .expect("complete key semantics are valid")
}

fn digest(index: IndexObservation) -> String {
    let relation = RelationObservation::new(
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
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect("complete index fixture is valid");

    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T09:15:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("snapshot fixture is valid")
    .snapshot_digest()
    .to_owned()
}

#[test]
fn named_tablespace_is_material_v3_identity() {
    let fast = digest(
        complete_index().with_tablespace(
            IndexTablespace::named("fast_index_space")
                .expect("named tablespace fixture is valid"),
        ),
    );
    let archive = digest(
        complete_index().with_tablespace(
            IndexTablespace::named("archive_index_space")
                .expect("named tablespace fixture is valid"),
        ),
    );

    assert_ne!(fast, archive);
}

#[test]
fn database_default_tablespace_is_distinct_from_unobserved_state() {
    let unobserved = digest(complete_index());
    let observed_default = digest(
        complete_index().with_tablespace(
            IndexTablespace::database_default("pg_default")
                .expect("database-default tablespace fixture is valid"),
        ),
    );

    assert_ne!(unobserved, observed_default);
}

#[test]
fn explicit_named_tablespace_does_not_collapse_into_database_default_marker() {
    let named = digest(
        complete_index().with_tablespace(
            IndexTablespace::named("pg_default").expect("named tablespace fixture is valid"),
        ),
    );
    let database_default = digest(
        complete_index().with_tablespace(
            IndexTablespace::database_default("pg_default")
                .expect("database-default tablespace fixture is valid"),
        ),
    );

    assert_ne!(named, database_default);
}

#[test]
fn blank_tablespace_name_fails_closed() {
    let error = IndexTablespace::named(" ")
        .expect_err("blank tablespace names are not valid catalog evidence");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_tablespace_name"
        }
    );
}
