use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics,
    IndexObservation, ObservationError, PostgresSchemaSnapshotV3, QualifiedCollationName,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn operator_class(name: &str) -> QualifiedOperatorClassName {
    QualifiedOperatorClassName::new("pg_catalog", name).expect("operator-class coordinate is valid")
}

fn collation(name: &str) -> QualifiedCollationName {
    QualifiedCollationName::new("pg_catalog", name).expect("collation coordinate is valid")
}

fn text_relation(index: IndexObservation) -> RelationObservation {
    RelationObservation::new(
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
    .expect("complete index semantics are admissible")
}

fn key_semantics(
    collation_name: Option<&str>,
    operator_class_name: &str,
    access_method_options: u16,
) -> IndexKeySemantics {
    IndexKeySemantics::new(
        1,
        collation_name.map(collation),
        operator_class(operator_class_name),
        access_method_options,
    )
    .expect("key-semantics fixture is valid")
}

fn text_index(semantics: IndexKeySemantics) -> IndexObservation {
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
    .with_access_method("btree")
    .with_key_semantics(vec![semantics])
    .expect("one semantic record exactly matches one key position")
}

fn digest(index: IndexObservation) -> String {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T05:00:00Z",
        vec![text_relation(index)],
        Vec::new(),
        Vec::new(),
    )
    .expect("complete successor snapshot is valid")
    .snapshot_digest()
    .to_owned()
}

#[test]
fn per_key_collation_operator_class_and_option_bits_are_material_identity() {
    let base = digest(text_index(key_semantics(Some("C"), "text_ops", 0)));
    let variants = [
        digest(text_index(key_semantics(Some("C.utf8"), "text_ops", 0))),
        digest(text_index(key_semantics(Some("C"), "text_pattern_ops", 0))),
        digest(text_index(key_semantics(Some("C"), "text_ops", 1))),
    ];

    for (index, variant) in variants.iter().enumerate() {
        assert_ne!(
            base, *variant,
            "material per-key semantic variant {index} must change structured v3 identity"
        );
    }
}

#[test]
fn missing_per_key_semantics_fail_closed_before_snapshot_identity() {
    let index = IndexObservation::new(
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
    .with_access_method("btree");

    let error = RelationObservation::new(
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
    .expect_err("every index key must carry one semantic record");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_key_semantics"
        }
    );
}

#[test]
fn blank_access_method_fails_closed_when_key_options_need_an_interpretation_context() {
    let index = IndexObservation::new(
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
    .with_access_method("   ")
    .with_key_semantics(vec![key_semantics(Some("C"), "text_ops", 0)])
    .expect("key semantics match the structural key position");

    let error = RelationObservation::new(
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
    .expect_err("per-key option bits require a nonblank observed access method");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "access_method"
        }
    );
}

#[test]
fn key_semantic_positions_must_match_the_exact_key_positions() {
    let index = IndexObservation::new(
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
    .with_access_method("btree");

    let error = index
        .with_key_semantics(vec![
            IndexKeySemantics::new(2, Some(collation("C")), operator_class("text_ops"), 0)
                .expect("the semantic record itself has a valid nonzero position"),
        ])
        .expect_err("semantic position two cannot describe structural key position one");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_key_semantics"
        }
    );
}

#[test]
fn operator_class_coordinate_requires_an_exact_namespace_and_name() {
    for (schema_name, operator_class_name, expected_field) in [
        ("", "text_ops", "schema_name"),
        ("pg_catalog", "", "operator_class_name"),
    ] {
        let error = QualifiedOperatorClassName::new(schema_name, operator_class_name)
            .expect_err("unqualified or blank operator-class coordinates must fail closed");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: expected_field
            }
        );
    }
}
