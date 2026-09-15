use conceptweave_observation::{
    ColumnObservationV3, DomainObservation, EnumObservation, ObservationError,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn relation(
    relation_name: &str,
    kind: RelationKind,
    column_name: &str,
    type_binding: QualifiedTypeName,
) -> RelationObservation {
    RelationObservation::new(
        "public",
        relation_name,
        kind,
        vec![
            ColumnObservationV3::new(
                column_name,
                1,
                format!("{}.{}", type_binding.schema_name(), type_binding.type_name()),
                type_binding,
                true,
                None,
            )
            .expect("column fixture is valid"),
        ],
    )
    .expect("relation fixture is valid")
}

fn snapshot(
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T14:50:00Z",
        relations,
        domains,
        enums,
    )
}

#[test]
fn relation_backed_composite_row_types_resolve_as_exact_type_bindings() {
    for (suffix, kind) in [
        ("table", RelationKind::Table),
        ("partitioned", RelationKind::PartitionedTable),
        ("view", RelationKind::View),
        ("materialized_view", RelationKind::MaterializedView),
        ("foreign", RelationKind::ForeignTable),
        ("composite", RelationKind::CompositeType),
    ] {
        let source_name = format!("row_source_{suffix}");
        let source = relation(&source_name, kind, "value", catalog_type("text"));
        let consumer = relation(
            &format!("consumer_{suffix}"),
            RelationKind::Table,
            "payload",
            QualifiedTypeName::new("public", &source_name)
                .expect("relation-backed type coordinate is valid"),
        );

        let result = snapshot(vec![source, consumer], Vec::new(), Vec::new());
        assert!(
            result.is_ok(),
            "{kind:?} has a pg_class.reltype-backed composite row type and must resolve by exact schema/name"
        );
    }
}

#[test]
fn sequence_does_not_create_a_relation_backed_row_type() {
    let source = relation(
        "sequence_source",
        RelationKind::Sequence,
        "last_value",
        catalog_type("int8"),
    );
    let consumer = relation(
        "sequence_consumer",
        RelationKind::Table,
        "payload",
        QualifiedTypeName::new("public", "sequence_source")
            .expect("qualified coordinate is structurally valid"),
    );

    assert!(
        snapshot(vec![source, consumer], Vec::new(), Vec::new()).is_err(),
        "pg_class.reltype is zero for sequences, so a sequence name cannot satisfy type resolution"
    );
}

#[test]
fn domain_can_use_an_observed_relation_backed_composite_as_its_base_type() {
    let composite = relation(
        "postal_address",
        RelationKind::CompositeType,
        "street",
        catalog_type("text"),
    );
    let domain = DomainObservation::new(
        "public",
        "validated_address",
        QualifiedTypeName::new("public", "postal_address")
            .expect("composite type coordinate is valid"),
    )
    .expect("domain fixture is valid");

    assert!(
        snapshot(vec![composite], vec![domain], Vec::new()).is_ok(),
        "PostgreSQL domains may use a composite type as their underlying data type"
    );
}

#[test]
fn table_row_type_name_cannot_collide_with_a_domain_in_the_same_schema() {
    let table = relation(
        "account_record",
        RelationKind::Table,
        "account_id",
        catalog_type("int8"),
    );
    let domain = DomainObservation::new("public", "account_record", catalog_type("text"))
        .expect("domain fixture is structurally valid");

    assert!(
        snapshot(vec![table], vec![domain], Vec::new()).is_err(),
        "CREATE TABLE creates a same-named row type, so a same-schema domain cannot share that type name"
    );
}

#[test]
fn table_row_type_name_cannot_collide_with_an_enum_in_the_same_schema() {
    let table = relation(
        "audit_record",
        RelationKind::Table,
        "event_id",
        catalog_type("int8"),
    );
    let observed_enum = EnumObservation::new(
        "public",
        "audit_record",
        vec!["created".to_owned(), "updated".to_owned()],
    )
    .expect("enum fixture is structurally valid");

    assert!(
        snapshot(vec![table], Vec::new(), vec![observed_enum]).is_err(),
        "the table's generated row type and an enum cannot occupy one exact pg_type name"
    );
}
