use conceptweave_observation::{
    ColumnObservationV3, DomainCheckConstraintObservation, DomainObservation, EnumObservation,
    IndexAttributeKind, IndexAttributeObservation, IndexKeySemantics, IndexObservation,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
    SchemaObjectLocation,
};

fn catalog_text() -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", "text").expect("catalog type coordinate is valid")
}

fn relation_fixture() -> RelationObservation {
    RelationObservation::new(
        "public",
        "items",
        RelationKind::Table,
        vec![ColumnObservationV3::new("id", 1, "text", catalog_text(), false, None)
            .expect("fixture column is valid")],
    )
    .expect("fixture relation is valid")
}

fn index_with_access_method(access_method: &str) -> IndexObservation {
    IndexObservation::new(
        "items_idx",
        false,
        None,
        vec![IndexAttributeObservation::column(1, IndexAttributeKind::Key, "id")
            .expect("fixture attribute is valid")],
        vec![],
    )
    .expect("fixture index is valid")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "text_ops")
                .expect("fixture operator class is valid"),
            0,
        )
        .expect("fixture key semantics are valid"),
    ])
    .expect("fixture key semantics match the index")
    .with_access_method(access_method)
}

#[test]
fn representation_v3_catalog_identifiers_preserve_quoted_whitespace() {
    let domain_check = DomainCheckConstraintObservation::new("  ", "VALUE IS NOT NULL", true, true)
        .expect("quoted domain-constraint identifiers may contain whitespace");
    assert_eq!(domain_check.constraint_name(), "  ");

    let domain = DomainObservation::new(" \t", "\t ", catalog_text())
        .expect("quoted domain coordinates may contain whitespace");
    assert_eq!(domain.schema_name(), " \t");
    assert_eq!(domain.domain_name(), "\t ");

    let observed_enum = EnumObservation::new("\t", "  ", vec!["label".to_owned()])
        .expect("quoted enum coordinates may contain whitespace");
    assert_eq!(observed_enum.schema_name(), "\t");
    assert_eq!(observed_enum.enum_name(), "  ");

    let relation = RelationObservation::new("  ", "\t", RelationKind::Table, vec![])
        .expect("quoted relation coordinates may contain whitespace");
    assert_eq!(relation.schema_name(), "  ");
    assert_eq!(relation.relation_name(), "\t");

    let attribute = IndexAttributeObservation::column(1, IndexAttributeKind::Key, " \t")
        .expect("quoted column identifiers may contain whitespace");
    assert_eq!(attribute.attribute_name(), Some(" \t"));

    let index = IndexObservation::new("\t ", false, None, vec![attribute], vec![])
        .expect("quoted index identifiers may contain whitespace");
    assert_eq!(index.index_name(), "\t ");

    let location = SchemaObjectLocation::constraint(
        " \t",
        "\t ",
        RelationKind::Table,
        "  ",
    )
    .expect("quoted successor coordinates may contain whitespace");
    assert_eq!(location.schema_name(), " \t");
    assert_eq!(location.relation_name(), Some("\t "));
    assert_eq!(location.constraint_name(), Some("  "));
}

#[test]
fn representation_v3_catalog_identifiers_reject_empty_and_code_zero() {
    assert!(DomainCheckConstraintObservation::new("bad\0constraint", "VALUE IS NOT NULL", true, true).is_err());
    assert!(DomainObservation::new("", "domain_name", catalog_text()).is_err());
    assert!(EnumObservation::new("public", "bad\0enum", vec![]).is_err());
    assert!(RelationObservation::new("public", "bad\0relation", RelationKind::Table, vec![]).is_err());
    assert!(IndexAttributeObservation::column(1, IndexAttributeKind::Key, "bad\0column").is_err());
    assert!(
        IndexObservation::new(
            "bad\0index",
            false,
            None,
            vec![IndexAttributeObservation::column(1, IndexAttributeKind::Key, "id")
                .expect("fixture attribute is valid")],
            vec![],
        )
        .is_err()
    );
    assert!(SchemaObjectLocation::relation("bad\0schema", "relation", RelationKind::Table).is_err());
}

#[test]
fn representation_v3_access_method_preserves_postgresql_identifier_semantics() {
    let relation = relation_fixture()
        .with_indexes(vec![index_with_access_method("  ")])
        .expect("quoted access-method identifiers may contain whitespace");
    assert_eq!(relation.indexes()[0].access_method(), Some("  "));

    assert!(
        relation_fixture()
            .with_indexes(vec![index_with_access_method("")])
            .is_err()
    );
    assert!(
        relation_fixture()
            .with_indexes(vec![index_with_access_method("bad\0am")])
            .is_err()
    );
}

#[test]
fn representation_v3_rendered_text_keeps_nonblank_policy() {
    assert!(DomainCheckConstraintObservation::new("constraint_name", "\t", true, true).is_err());
}
