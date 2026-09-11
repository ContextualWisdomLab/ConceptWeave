use conceptweave_observation::{
    ArrayTypeObservation, ColumnObservationV3, DomainObservation, EnumObservation,
    PostgresSchemaSnapshotV3, QualifiedTypeName, RelationKind, RelationObservation,
};

mod support;

fn type_name(schema: &str, name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema, name).expect("qualified type coordinate is valid")
}

fn catalog_type(name: &str) -> QualifiedTypeName {
    type_name("pg_catalog", name)
}

fn relation(
    relation_name: &str,
    kind: RelationKind,
    column_name: &str,
    binding: QualifiedTypeName,
) -> RelationObservation {
    RelationObservation::new(
        "public",
        relation_name,
        kind,
        vec![
            ColumnObservationV3::new(
                column_name,
                1,
                format!("{}.{}", binding.schema_name(), binding.type_name()),
                binding,
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
    array_types: Vec<ArrayTypeObservation>,
) -> Result<PostgresSchemaSnapshotV3, conceptweave_observation::ObservationError> {
    PostgresSchemaSnapshotV3::new_with_array_types(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T15:12:00Z",
        relations,
        domains,
        enums,
        array_types,
    )
}

#[test]
fn observed_enum_array_type_resolves_by_exact_catalog_coordinate() {
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let status_array = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("array fixture is valid");
    let consumer = relation(
        "ticket",
        RelationKind::Table,
        "statuses",
        type_name("public", "_status"),
    );

    assert!(
        snapshot(vec![consumer], Vec::new(), vec![status], vec![status_array]).is_ok(),
        "the exact pg_type.typarray row for an observed enum must satisfy type resolution"
    );
}

#[test]
fn observed_domain_and_relation_row_type_arrays_resolve() {
    let account_id = DomainObservation::new("public", "account_id", catalog_type("int8"))
        .expect("domain fixture is valid");
    let address = relation(
        "postal_address",
        RelationKind::CompositeType,
        "street",
        catalog_type("text"),
    );
    let domain_array = ArrayTypeObservation::new(
        type_name("public", "_account_id"),
        type_name("public", "account_id"),
    )
    .expect("domain array fixture is valid");
    let composite_array = ArrayTypeObservation::new(
        type_name("public", "_postal_address"),
        type_name("public", "postal_address"),
    )
    .expect("composite array fixture is valid");
    let consumer = RelationObservation::new(
        "public",
        "shipment",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new(
                "account_ids",
                1,
                "public.account_id[]",
                type_name("public", "_account_id"),
                false,
                None,
            )
            .expect("domain-array column is valid"),
            ColumnObservationV3::new(
                "addresses",
                2,
                "public.postal_address[]",
                type_name("public", "_postal_address"),
                true,
                None,
            )
            .expect("composite-array column is valid"),
        ],
    )
    .expect("consumer relation is valid");

    assert!(
        snapshot(
            vec![address, consumer],
            vec![account_id],
            Vec::new(),
            vec![domain_array, composite_array],
        )
        .is_ok(),
        "true arrays of observed domain and composite row types must be first-class type identity"
    );
}

#[test]
fn collision_adjusted_array_name_is_observed_not_inferred_from_underscore_convention() {
    let reserved = EnumObservation::new(
        "public",
        "reserved",
        vec!["reserved".to_owned()],
    )
    .expect("reserved enum fixture is valid");
    let renamed_reserved_array = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "reserved"),
    )
    .expect("renamed prior true-array fixture is valid");
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let collision_adjusted_array = ArrayTypeObservation::new(
        type_name("public", "__status"),
        type_name("public", "status"),
    )
    .expect("collision-adjusted array fixture is valid");
    let consumer = relation(
        "ticket",
        RelationKind::Table,
        "statuses",
        type_name("public", "__status"),
    );

    assert!(
        snapshot(
            vec![consumer],
            Vec::new(),
            vec![reserved, status],
            vec![renamed_reserved_array, collision_adjusted_array],
        )
        .is_ok(),
        "array identity must follow the exact typarray row when a prior true-array name occupies the conventional candidate"
    );
}

#[test]
fn array_element_must_resolve_to_an_observed_or_catalog_type() {
    let invalid_array = ArrayTypeObservation::new(
        type_name("public", "_missing_type"),
        type_name("public", "missing_type"),
    )
    .expect("array coordinate itself is structurally valid");

    assert!(
        snapshot(Vec::new(), Vec::new(), Vec::new(), vec![invalid_array]).is_err(),
        "an array pg_type row cannot promote an unknown element coordinate into governed identity"
    );
}

#[test]
fn array_type_name_cannot_collide_with_an_existing_exact_pg_type_name() {
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let colliding_domain = DomainObservation::new("public", "_status", catalog_type("text"))
        .expect("domain fixture is structurally valid");
    let colliding_array = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("array fixture is structurally valid");

    assert!(
        snapshot(
            Vec::new(),
            vec![colliding_domain],
            vec![status],
            vec![colliding_array],
        )
        .is_err(),
        "array types occupy the same schema-local pg_type namespace as domains and enums"
    );
}

#[test]
fn duplicate_exact_array_type_coordinates_fail_closed() {
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let first = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("first array fixture is valid");
    let duplicate = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("duplicate array fixture is structurally valid");

    assert!(
        snapshot(Vec::new(), Vec::new(), vec![status], vec![first, duplicate]).is_err(),
        "duplicate exact array pg_type rows are contradictory observation evidence"
    );
}

#[test]
fn one_element_type_cannot_claim_two_true_array_rows() {
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let first = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("first true-array fixture is valid");
    let contradictory = ArrayTypeObservation::new(
        type_name("public", "__status"),
        type_name("public", "status"),
    )
    .expect("second coordinate is structurally valid");

    assert!(
        snapshot(
            Vec::new(),
            Vec::new(),
            vec![status],
            vec![first, contradictory],
        )
        .is_err(),
        "pg_type.typarray identifies one true array row for one element type"
    );
}

#[test]
fn observed_true_array_cannot_be_the_element_of_another_true_array() {
    let status = EnumObservation::new(
        "public",
        "status",
        vec!["open".to_owned(), "closed".to_owned()],
    )
    .expect("enum fixture is valid");
    let status_array = ArrayTypeObservation::new(
        type_name("public", "_status"),
        type_name("public", "status"),
    )
    .expect("true-array fixture is valid");
    let array_of_array = ArrayTypeObservation::new(
        type_name("public", "__status"),
        type_name("public", "_status"),
    )
    .expect("nested array coordinate is structurally valid");

    assert!(
        snapshot(
            Vec::new(),
            Vec::new(),
            vec![status],
            vec![status_array, array_of_array],
        )
        .is_err(),
        "PostgreSQL multidimensional values use the same true array type rather than arrays of arrays"
    );
}
