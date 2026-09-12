use conceptweave_observation::{
    ColumnObservationV3, ConstraintPeriodObservation, DomainObservation, IndexAttributeKind,
    IndexAttributeObservation, IndexCatalogFlags, IndexObservation, ObservationError,
    PostgresSchemaSnapshotV3, PostgresTypeKind, PrimaryKeyObservation, QualifiedTypeName,
    RelationKind, RelationObservation, TableConstraintObservation, TypeKindObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn user_type(schema_name: &str, type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new(schema_name, type_name).expect("user type coordinate is valid")
}

fn column_with_binding(
    name: &str,
    position: u32,
    data_type: &str,
    type_binding: QualifiedTypeName,
) -> ColumnObservationV3 {
    ColumnObservationV3::new(name, position, data_type, type_binding, false, None)
        .expect("column fixture is valid")
}

fn column(
    name: &str,
    position: u32,
    data_type: &str,
    type_name: &str,
) -> ColumnObservationV3 {
    column_with_binding(name, position, data_type, catalog_type(type_name))
}

fn temporal_backing_index() -> IndexObservation {
    let attributes = ["document_id", "valid_during"]
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
    IndexObservation::new(
        "document_temporal_key",
        true,
        Some(false),
        attributes,
        Vec::new(),
    )
    .expect("backing index fixture is structurally valid")
    .with_access_method("gist")
    .with_catalog_flags(IndexCatalogFlags::new(
        true, true, true, false, false, false,
    ))
    .expect("catalog flags fixture is coherent")
}

fn temporal_primary_key_with_binding(
    type_binding: QualifiedTypeName,
    data_type: &str,
) -> RelationObservation {
    RelationObservation::new(
        "public",
        "document",
        RelationKind::Table,
        vec![
            column("document_id", 1, "bigint", "int8"),
            column_with_binding("valid_during", 2, data_type, type_binding),
        ],
    )
    .expect("relation fixture is valid")
    .with_constraints(vec![TableConstraintObservation::PrimaryKey(
        PrimaryKeyObservation::new(
            "document_temporal_key",
            vec!["document_id".to_owned(), "valid_during".to_owned()],
        )
        .expect("primary-key fixture is valid"),
    )])
    .expect("constraint fixture is valid")
    .with_indexes(vec![temporal_backing_index()])
    .expect("index fixture is valid")
}

fn temporal_primary_key_with_scalar_period() -> RelationObservation {
    temporal_primary_key_with_binding(catalog_type("text"), "text")
}

fn temporal_primary_key_with_domain_period() -> RelationObservation {
    temporal_primary_key_with_binding(user_type("public", "active_period"), "active_period")
}

fn temporal_period() -> ConstraintPeriodObservation {
    ConstraintPeriodObservation::new(
        "public",
        "document",
        RelationKind::Table,
        "document_temporal_key",
        true,
    )
    .expect("explicit conperiod fixture is structurally valid")
}

fn catalog_range_family(range_name: &str, multirange_name: &str) -> Vec<TypeKindObservation> {
    vec![
        TypeKindObservation::range(catalog_type(range_name), catalog_type(multirange_name)),
        TypeKindObservation::multirange(catalog_type(multirange_name), catalog_type(range_name)),
    ]
}

fn user_range_family(range_name: &str, multirange_name: &str) -> Vec<TypeKindObservation> {
    vec![
        TypeKindObservation::range(
            user_type("public", range_name),
            user_type("public", multirange_name),
        ),
        TypeKindObservation::multirange(
            user_type("public", multirange_name),
            user_type("public", range_name),
        ),
    ]
}

#[test]
fn without_overlaps_rejects_missing_type_kind_evidence() {
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T02:45:00Z",
        vec![temporal_primary_key_with_scalar_period()],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot preserves the qualified column binding without claiming type kind");

    let error = snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect_err("temporal semantics cannot be inferred from a type name or GiST index");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        }
    );
}

#[test]
fn without_overlaps_rejects_scalar_final_key_column() {
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T02:50:00Z",
        vec![temporal_primary_key_with_scalar_period()],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot can preserve non-temporal scalar schema evidence")
    .with_observed_type_kinds(vec![
        TypeKindObservation::plain(catalog_type("text"), PostgresTypeKind::Base)
            .expect("scalar type-kind fixture is valid"),
    ])
    .expect("scalar pg_type evidence is valid source evidence");

    let error = snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect_err(
            "PostgreSQL 18 WITHOUT OVERLAPS requires its final column to resolve to range or multirange",
        );
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        }
    );
}

#[test]
fn without_overlaps_preserves_domain_over_range_positive_control() {
    let period_domain = DomainObservation::new(
        "public",
        "active_period",
        catalog_type("tstzrange"),
    )
    .expect("domain-over-range fixture is valid");
    let mut type_kinds = catalog_range_family("tstzrange", "tstzmultirange");
    type_kinds.push(TypeKindObservation::domain(
        user_type("public", "active_period"),
        catalog_type("tstzrange"),
    ));
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:25:00Z",
        vec![temporal_primary_key_with_domain_period()],
        vec![period_domain],
        Vec::new(),
    )
    .expect("domain-over-range schema evidence is representable")
    .with_observed_type_kinds(type_kinds)
    .expect("pg_type and pg_range evidence is coherent");

    let accepted = snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect("PostgreSQL 18.4+ permits WITHOUT OVERLAPS on a domain over a range type");
    assert!(
        accepted
            .constraint_periods()
            .expect("period family was observed")[0]
            .has_period_semantics()
    );
}

#[test]
fn without_overlaps_preserves_domain_over_user_defined_range() {
    let period_domain = DomainObservation::new(
        "public",
        "active_period",
        user_type("public", "business_period"),
    )
    .expect("domain-over-user-range fixture is valid");
    let mut type_kinds = user_range_family("business_period", "business_period_set");
    type_kinds.push(TypeKindObservation::domain(
        user_type("public", "active_period"),
        user_type("public", "business_period"),
    ));
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:27:00Z",
        vec![temporal_primary_key_with_domain_period()],
        vec![period_domain],
        Vec::new(),
        type_kinds,
    )
    .expect("domain over a user-defined range remains representable through the compatibility seam");

    snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect("domain-over-user-range satisfies WITHOUT OVERLAPS type semantics");
}

#[test]
fn without_overlaps_rejects_domain_over_scalar() {
    let period_domain = DomainObservation::new(
        "public",
        "active_period",
        catalog_type("text"),
    )
    .expect("domain-over-scalar fixture is valid");
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:30:00Z",
        vec![temporal_primary_key_with_domain_period()],
        vec![period_domain],
        Vec::new(),
    )
    .expect("domain-over-scalar schema is structurally representable")
    .with_observed_type_kinds(vec![
        TypeKindObservation::domain(
            user_type("public", "active_period"),
            catalog_type("text"),
        ),
        TypeKindObservation::plain(catalog_type("text"), PostgresTypeKind::Base)
            .expect("scalar type-kind fixture is valid"),
    ])
    .expect("domain and base-type catalog evidence is coherent");

    let error = snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect_err("a domain does not make a scalar base type temporal");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        }
    );
}

#[test]
fn direct_user_defined_range_is_preserved_without_name_allowlisting() {
    let relation = temporal_primary_key_with_binding(
        user_type("public", "business_period"),
        "business_period",
    );
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:35:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        user_range_family("business_period", "business_period_set"),
    )
    .expect("direct user-defined range coordinates are governed source evidence");

    assert_eq!(
        snapshot.relations()[0].columns()[1].type_binding(),
        &user_type("public", "business_period")
    );
    assert!(snapshot.type_kinds().is_some());
    snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect("direct user-defined range satisfies WITHOUT OVERLAPS type semantics");
}

#[test]
fn direct_user_defined_multirange_is_preserved_without_name_allowlisting() {
    let relation = temporal_primary_key_with_binding(
        user_type("public", "business_period_set"),
        "business_period_set",
    );
    let snapshot = PostgresSchemaSnapshotV3::new_with_type_kinds(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:40:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
        user_range_family("business_period", "business_period_set"),
    )
    .expect("direct user-defined multirange coordinates are governed source evidence");

    snapshot
        .with_observed_constraint_periods(vec![temporal_period()])
        .expect("direct user-defined multirange satisfies WITHOUT OVERLAPS type semantics");
}

#[test]
fn range_and_multirange_evidence_must_be_reciprocal() {
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:45:00Z",
        vec![temporal_primary_key_with_scalar_period()],
        Vec::new(),
        Vec::new(),
    )
    .expect("base snapshot fixture is valid");

    let error = snapshot
        .with_observed_type_kinds(vec![TypeKindObservation::range(
            catalog_type("tstzrange"),
            catalog_type("tstzmultirange"),
        )])
        .expect_err("one-sided pg_range evidence cannot be governed as a complete pair");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "type_kind_range_reciprocity",
        }
    );
}

#[test]
fn cyclic_domain_type_kind_evidence_fails_closed() {
    let first_domain = DomainObservation::new(
        "public",
        "active_period",
        user_type("public", "active_period_alias"),
    )
    .expect("first cyclic domain fixture is structurally valid");
    let second_domain = DomainObservation::new(
        "public",
        "active_period_alias",
        user_type("public", "active_period"),
    )
    .expect("second cyclic domain fixture is structurally valid");
    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:50:00Z",
        vec![temporal_primary_key_with_domain_period()],
        vec![first_domain, second_domain],
        Vec::new(),
    )
    .expect("base representation preserves the explicit domain coordinates");

    let error = snapshot
        .with_observed_type_kinds(vec![
            TypeKindObservation::domain(
                user_type("public", "active_period"),
                user_type("public", "active_period_alias"),
            ),
            TypeKindObservation::domain(
                user_type("public", "active_period_alias"),
                user_type("public", "active_period"),
            ),
        ])
        .expect_err("domain-base evidence must not contain a cycle");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "type_kind_domain_cycle",
        }
    );
}

#[test]
fn type_kind_identity_distinguishes_unobserved_and_is_input_order_stable() {
    let relation = temporal_primary_key_with_scalar_period();
    let unobserved = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:55:00Z",
        vec![relation.clone()],
        Vec::new(),
        Vec::new(),
    )
    .expect("unobserved type-kind snapshot is valid");

    let forward = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:55:00Z",
        vec![relation.clone()],
        Vec::new(),
        Vec::new(),
    )
    .expect("forward base snapshot is valid")
    .with_observed_type_kinds(catalog_range_family("tstzrange", "tstzmultirange"))
    .expect("forward type-kind family is coherent");

    let mut reverse_types = catalog_range_family("tstzrange", "tstzmultirange");
    reverse_types.reverse();
    let reverse = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-12T03:55:00Z",
        vec![relation],
        Vec::new(),
        Vec::new(),
    )
    .expect("reverse base snapshot is valid")
    .with_observed_type_kinds(reverse_types)
    .expect("reverse type-kind family is coherent");

    assert_ne!(unobserved.snapshot_digest(), forward.snapshot_digest());
    assert_eq!(forward.snapshot_digest(), reverse.snapshot_digest());
    assert_eq!(unobserved.type_kinds(), None);
    assert!(forward.type_kinds().is_some());
}
