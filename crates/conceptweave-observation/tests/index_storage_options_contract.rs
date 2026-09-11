use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, IndexStorageOption, ObservationError, PostgresSchemaSnapshotV3,
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

fn option(name: &str, value: &str) -> IndexStorageOption {
    IndexStorageOption::new(name, value).expect("storage-option fixture is valid")
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
        "2026-09-11T09:05:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("snapshot fixture is valid")
    .snapshot_digest()
    .to_owned()
}

#[test]
fn storage_option_value_is_material_v3_identity() {
    let lower = digest(
        complete_index()
            .with_storage_options(vec![option("fillfactor", "70")])
            .expect("storage option is valid"),
    );
    let higher = digest(
        complete_index()
            .with_storage_options(vec![option("fillfactor", "90")])
            .expect("storage option is valid"),
    );

    assert_ne!(lower, higher);
}

#[test]
fn storage_option_order_is_not_a_second_identity() {
    let left = digest(
        complete_index()
            .with_storage_options(vec![
                option("fillfactor", "70"),
                option("deduplicate_items", "off"),
            ])
            .expect("storage options are valid"),
    );
    let right = digest(
        complete_index()
            .with_storage_options(vec![
                option("deduplicate_items", "off"),
                option("fillfactor", "70"),
            ])
            .expect("storage options are valid"),
    );

    assert_eq!(left, right);
}

#[test]
fn unobserved_reloptions_do_not_collapse_into_observed_empty_set() {
    let unobserved = digest(complete_index());
    let observed_empty = digest(
        complete_index()
            .with_storage_options(Vec::new())
            .expect("an explicitly observed empty reloptions set is valid"),
    );

    assert_ne!(unobserved, observed_empty);
}

#[test]
fn duplicate_storage_option_names_fail_closed() {
    let error = complete_index()
        .with_storage_options(vec![
            option("fillfactor", "70"),
            option("fillfactor", "90"),
        ])
        .expect_err("one exact option name cannot carry two observed values");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_storage_options"
        }
    );
}

#[test]
fn blank_storage_option_name_fails_closed_while_value_is_exact_text() {
    let error = IndexStorageOption::new(" ", "70")
        .expect_err("blank storage-option names are not valid catalog evidence");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_storage_option_name"
        }
    );

    let exact = IndexStorageOption::new("extension_option", "")
        .expect("generic representation preserves an exact empty value without interpretation");
    assert_eq!(exact.value(), "");
}
