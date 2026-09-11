use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, OperatorClassOption, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn index_with_options(options: Vec<OperatorClassOption>) -> IndexObservation {
    let semantics = IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("public", "configurable_ops")
            .expect("operator-class coordinate is valid"),
        0,
    )
    .expect("key semantics are valid")
    .with_operator_class_options(options)
    .expect("distinct operator-class option names are valid");

    IndexObservation::new(
        "document_title_ix",
        false,
        None,
        vec![
            IndexAttributeObservation::new(1, IndexAttributeKind::Key, "title")
                .expect("key attribute fixture is valid"),
        ],
        Vec::new(),
    )
    .expect("index layout is valid")
    .with_access_method("gist")
    .with_key_semantics(vec![semantics])
    .expect("one semantic record matches one structural key")
}

fn digest(index: IndexObservation) -> String {
    let relation = RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new("title", 1, "text", catalog_type("text"), false, None)
                .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
    .with_indexes(vec![index])
    .expect("complete index evidence is admissible");

    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T07:00:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("successor snapshot is valid")
    .snapshot_digest()
    .to_owned()
}

#[test]
fn operator_class_options_are_material_successor_identity() {
    let short = digest(index_with_options(vec![
        OperatorClassOption::new("siglen", "16").expect("option fixture is valid"),
    ]));
    let long = digest(index_with_options(vec![
        OperatorClassOption::new("siglen", "32").expect("option fixture is valid"),
    ]));

    assert_ne!(
        short, long,
        "different PostgreSQL operator-class parameters must not collapse to one v3 digest"
    );
}

#[test]
fn operator_class_option_order_is_canonical_but_duplicate_names_fail_closed() {
    let first = digest(index_with_options(vec![
        OperatorClassOption::new("alpha", "1").expect("option fixture is valid"),
        OperatorClassOption::new("beta", "2").expect("option fixture is valid"),
    ]));
    let reversed = digest(index_with_options(vec![
        OperatorClassOption::new("beta", "2").expect("option fixture is valid"),
        OperatorClassOption::new("alpha", "1").expect("option fixture is valid"),
    ]));
    assert_eq!(
        first, reversed,
        "catalog option-array order must not create a second governed identity"
    );

    let error = IndexKeySemantics::new(
        1,
        None,
        QualifiedOperatorClassName::new("public", "configurable_ops")
            .expect("operator-class coordinate is valid"),
        0,
    )
    .expect("key semantics are valid")
    .with_operator_class_options(vec![
        OperatorClassOption::new("siglen", "16").expect("option fixture is valid"),
        OperatorClassOption::new("siglen", "32").expect("option fixture is valid"),
    ])
    .expect_err("one operator-class parameter name cannot carry two observed values");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "operator_class_options",
        }
    );
}

#[test]
fn operator_class_option_name_must_be_nonblank_but_value_is_exact_source_text() {
    assert_eq!(
        OperatorClassOption::new(" ", "16"),
        Err(ObservationError::InvalidObservationField {
            field: "operator_class_option_name",
        })
    );

    let empty_value = OperatorClassOption::new("strategy", "")
        .expect("an empty option value is exact source text, not missing evidence");
    assert_eq!(empty_value.name(), "strategy");
    assert_eq!(empty_value.value(), "");
}
