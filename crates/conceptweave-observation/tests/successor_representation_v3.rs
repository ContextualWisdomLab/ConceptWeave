use conceptweave_observation::{
    CheckConstraintObservation, ColumnObservationV3, DomainCheckConstraintObservation,
    DomainObservation, EnumObservation, IndexAttributeKind, IndexAttributeObservation,
    IndexObservation, ObservationError, PostgresSchemaSnapshot, PostgresSchemaSnapshotV3,
    QualifiedCollationName, QualifiedTypeName, RelationKind, RelationObservation,
    SchemaObjectLocation, SchemaObjectLocationKind, TableConstraintObservation,
    UniqueConstraintObservation,
};

mod support;

fn catalog_type(type_name: &str) -> QualifiedTypeName {
    QualifiedTypeName::new("pg_catalog", type_name).expect("catalog type coordinate is valid")
}

fn bound_column(
    column_name: &str,
    ordinal_position: u32,
    data_type: &str,
    type_binding: QualifiedTypeName,
    nullable: bool,
) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        column_name,
        ordinal_position,
        data_type,
        type_binding,
        nullable,
        None,
    )
    .expect("bound column fixture is valid")
}

fn event_index() -> IndexObservation {
    IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
    )
    .expect("index fixture is valid")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("observed index comment")
}

fn index_attribute(
    position: u32,
    kind: IndexAttributeKind,
    attribute_name: &str,
) -> IndexAttributeObservation {
    IndexAttributeObservation::new(position, kind, attribute_name)
        .expect("index attribute fixture is valid")
}

fn indexed_relation() -> RelationObservation {
    event_relation(RelationKind::Table)
        .with_indexes(vec![event_index()])
        .expect("index fixture references observed columns")
}

fn expression_index() -> IndexObservation {
    IndexObservation::new(
        "event_lower_email_ix",
        true,
        Some(false),
        vec![IndexAttributeObservation::expression(
            1,
            IndexAttributeKind::Key,
            "lower(parent_key)",
        )
        .expect("expression attribute fixture is valid")],
        Vec::new(),
    )
    .expect("expression index fixture is valid")
    .with_access_method("btree")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_ready(true)
    .with_valid(true)
    .with_live(false)
    .with_index_definition(
        "CREATE UNIQUE INDEX event_lower_email_ix ON public.event_record USING btree (lower(parent_key)) WHERE (parent_key IS NOT NULL)",
    )
    .with_source_comment("observed expression index comment")
}

fn event_constraints(reversed: bool) -> Vec<TableConstraintObservation> {
    let unique = TableConstraintObservation::Unique(
        UniqueConstraintObservation::new("event_parent_uq", vec!["parent_key".to_owned()])
            .expect("unique constraint fixture is valid"),
    );
    let check = TableConstraintObservation::Check(
        CheckConstraintObservation::new(
            "event_parent_present",
            "CHECK ((parent_key IS NOT NULL))",
            true,
            true,
            false,
        )
        .expect("check constraint fixture is valid"),
    );
    if reversed {
        vec![check, unique]
    } else {
        vec![unique, check]
    }
}

fn base_relation(
    kind: RelationKind,
    reversed_columns: bool,
    reversed_constraints: bool,
) -> RelationObservation {
    let mut columns = vec![
        bound_column("event_key", 1, "uuid", catalog_type("uuid"), false),
        bound_column("parent_key", 2, "uuid", catalog_type("uuid"), true),
    ];
    if reversed_columns {
        columns.reverse();
    }
    RelationObservation::new("public", "event_record", kind, columns)
        .expect("relation fixture is valid")
        .with_constraints(event_constraints(reversed_constraints))
        .expect("relation constraints are valid")
}

fn event_relation(kind: RelationKind) -> RelationObservation {
    base_relation(kind, false, false).with_source_comment("observed relation comment")
}

fn audit_view() -> RelationObservation {
    RelationObservation::new("audit", "event_record", RelationKind::View, Vec::new())
        .expect("audit view fixture is valid")
}

fn status_enum() -> EnumObservation {
    EnumObservation::new(
        "public",
        "event_status",
        vec!["pending".to_owned(), "done".to_owned()],
    )
    .expect("enum fixture is valid")
    .with_source_comment("observed enum comment")
}

fn status_domain() -> DomainObservation {
    DomainObservation::new("public", "event_status_kind", catalog_type("text"))
        .expect("domain fixture is valid")
        .with_type_modifier(64)
        .with_array_dimensions(0)
        .with_collation(
            QualifiedCollationName::new("pg_catalog", "C").expect("collation fixture is valid"),
        )
        .with_not_null(true)
        .with_default_expression("'pending'::text")
        .with_check_constraints(vec![
            DomainCheckConstraintObservation::new(
                "event_status_kind_allowed",
                "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                true,
                true,
            )
            .expect("domain check fixture is valid"),
        ])
        .expect("domain check coordinates are unique")
        .with_source_comment("observed domain comment")
}

fn audit_status_domain() -> DomainObservation {
    DomainObservation::new("audit", "event_status_kind", catalog_type("text"))
        .expect("audit domain fixture is valid")
}

fn snapshot_v3(
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
) -> Result<PostgresSchemaSnapshotV3, ObservationError> {
    PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["audit", "public", "types_only"]),
        "postgres_introspector_v3",
        "2026-09-11T00:00:00Z",
        relations,
        domains,
        enums,
    )
}

fn digest_of(
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
) -> String {
    snapshot_v3(relations, domains, enums)
        .expect("fixture snapshot is valid")
        .snapshot_digest()
        .to_owned()
}

fn complete_snapshot() -> PostgresSchemaSnapshotV3 {
    snapshot_v3(
        vec![event_relation(RelationKind::Table), audit_view()],
        vec![status_domain(), audit_status_domain()],
        vec![status_enum()],
    )
    .expect("complete fixture snapshot is valid")
}

#[test]
fn empty_v3_snapshot_matches_the_independent_successor_vector() {
    let snapshot = snapshot_v3(Vec::new(), Vec::new(), Vec::new())
        .expect("an empty bounded observation still has deterministic identity");
    // Independent SHA-256 vector: big-endian u64 domain length, UTF-8 v3 domain,
    // then big-endian u64 zero relation, domain and enum counts.
    assert_eq!(
        snapshot.snapshot_digest(),
        "sha256:07653b7353c24b0e43b0266853eae9d41f8e96d9ab334546882d9a8647af7d5a"
    );
}

#[test]
fn v3_identity_is_domain_separated_from_v2() {
    let v3_empty = snapshot_v3(Vec::new(), Vec::new(), Vec::new())
        .expect("empty v3 fixture is valid")
        .snapshot_digest()
        .to_owned();
    let v2_empty = PostgresSchemaSnapshot::new(
        &support::resolved_source("warehouse_primary"),
        "postgres_introspector_v3",
        "2026-09-11T00:00:00Z",
        Vec::new(),
    )
    .expect("empty v2 fixture is valid")
    .snapshot_digest()
    .to_owned();

    assert_ne!(
        v3_empty, v2_empty,
        "successor framing must never reinterpret the frozen v2 domain"
    );
}

#[test]
fn relation_kind_is_material_successor_identity() {
    let kinds = [
        RelationKind::Table,
        RelationKind::PartitionedTable,
        RelationKind::View,
        RelationKind::MaterializedView,
        RelationKind::ForeignTable,
        RelationKind::Sequence,
        RelationKind::CompositeType,
    ];
    let digests: Vec<String> = kinds
        .iter()
        .map(|kind| digest_of(vec![event_relation(*kind)], Vec::new(), Vec::new()))
        .collect();

    for left in 0..digests.len() {
        for right in left + 1..digests.len() {
            assert_ne!(
                digests[left], digests[right],
                "relation kinds {left} and {right} must not share source identity"
            );
        }
    }
}

#[test]
fn relation_and_column_metadata_are_material_successor_identity() {
    let base = digest_of(
        vec![event_relation(RelationKind::Table)],
        Vec::new(),
        Vec::new(),
    );
    let variants = [
        digest_of(
            vec![base_relation(RelationKind::Table, false, false)],
            Vec::new(),
            Vec::new(),
        ),
        digest_of(
            vec![event_relation(RelationKind::Table).with_source_comment("changed comment")],
            Vec::new(),
            Vec::new(),
        ),
        digest_of(
            vec![
                base_relation(RelationKind::Table, false, false)
                    .with_constraints(vec![TableConstraintObservation::Unique(
                        UniqueConstraintObservation::new(
                            "event_parent_uq",
                            vec!["parent_key".to_owned()],
                        )
                        .expect("unique fixture is valid"),
                    )])
                    .expect("constraint replacement is valid"),
            ],
            Vec::new(),
            Vec::new(),
        ),
    ];

    for (index, variant) in variants.iter().enumerate() {
        assert_ne!(
            base, *variant,
            "material relation variant {index} must change successor identity"
        );
    }

    let column_comment = bound_column("event_key", 1, "uuid", catalog_type("uuid"), false);
    let with_comment = ColumnObservationV3::new(
        "event_key",
        1,
        "uuid",
        catalog_type("uuid"),
        false,
        Some("column comment".to_owned()),
    )
    .expect("commented column fixture is valid");
    assert_ne!(column_comment, with_comment);

    let nullable = bound_column("event_key", 1, "uuid", catalog_type("uuid"), true);
    assert_ne!(column_comment, nullable);

    let renamed_display = bound_column("event_key", 1, "uuid ", catalog_type("uuid"), false);
    assert_ne!(column_comment, renamed_display);
}

#[test]
fn enum_label_membership_and_order_are_material_successor_identity() {
    let base = digest_of(Vec::new(), Vec::new(), vec![status_enum()]);

    let reordered = EnumObservation::new(
        "public",
        "event_status",
        vec!["done".to_owned(), "pending".to_owned()],
    )
    .expect("reordered enum fixture is valid")
    .with_source_comment("observed enum comment");
    let renamed = EnumObservation::new(
        "public",
        "event_status",
        vec!["pending".to_owned(), "closed".to_owned()],
    )
    .expect("renamed enum fixture is valid")
    .with_source_comment("observed enum comment");
    let extended = EnumObservation::new(
        "public",
        "event_status",
        vec!["pending".to_owned(), "done".to_owned(), "closed".to_owned()],
    )
    .expect("extended enum fixture is valid")
    .with_source_comment("observed enum comment");
    let comment_only = status_enum().with_source_comment("changed enum comment");

    for (index, variant) in [reordered, renamed, extended, comment_only]
        .into_iter()
        .enumerate()
    {
        assert_ne!(
            base,
            digest_of(Vec::new(), Vec::new(), vec![variant]),
            "material enum variant {index} must change successor identity"
        );
    }
}

#[test]
fn domain_material_semantics_are_successor_identity() {
    let base = digest_of(Vec::new(), vec![status_domain()], Vec::new());

    let variants = [
        DomainObservation::new("public", "event_status_kind", catalog_type("varchar"))
            .expect("base-type variant is valid")
            .with_type_modifier(64)
            .with_array_dimensions(0)
            .with_collation(QualifiedCollationName::new("pg_catalog", "C").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    true,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("modifier variant is valid")
            .with_type_modifier(32)
            .with_array_dimensions(0)
            .with_collation(QualifiedCollationName::new("pg_catalog", "C").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    true,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("array variant is valid")
            .with_type_modifier(64)
            .with_array_dimensions(1)
            .with_collation(QualifiedCollationName::new("pg_catalog", "C").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    true,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("collation variant is valid")
            .with_type_modifier(64)
            .with_array_dimensions(0)
            .with_collation(QualifiedCollationName::new("public", "shift_jis").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    true,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        status_domain().with_not_null(false),
        status_domain().with_default_expression("'done'::text"),
        status_domain().with_source_comment("changed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("check variant is valid")
            .with_type_modifier(64)
            .with_array_dimensions(0)
            .with_collation(QualifiedCollationName::new("pg_catalog", "C").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text])))",
                    true,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("validated variant is valid")
            .with_type_modifier(64)
            .with_array_dimensions(0)
            .with_collation(QualifiedCollationName::new("pg_catalog", "C").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    false,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("enforced variant is valid")
            .with_type_modifier(64)
            .with_array_dimensions(0)
            .with_collation(QualifiedCollationName::new("pg_catalog", "C").expect("collation"))
            .with_not_null(true)
            .with_default_expression("'pending'::text")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    true,
                    false,
                )
                .expect("check"),
            ])
            .expect("checks")
            .with_source_comment("observed domain comment"),
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("bare domain variant is valid")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE = ANY (ARRAY['pending'::text, 'done'::text])))",
                    true,
                    true,
                )
                .expect("check"),
            ])
            .expect("checks"),
    ];

    for (index, variant) in variants.into_iter().enumerate() {
        assert_ne!(
            base,
            digest_of(Vec::new(), vec![variant], Vec::new()),
            "material domain variant {index} must change successor identity"
        );
    }
}

#[test]
fn v3_digest_excludes_provenance_and_input_order() {
    let first = complete_snapshot();
    let permuted = snapshot_v3(
        vec![
            audit_view(),
            base_relation(RelationKind::Table, true, true)
                .with_source_comment("observed relation comment"),
        ],
        vec![audit_status_domain(), status_domain()],
        vec![status_enum()],
    )
    .expect("permuted fixture is valid");
    assert_eq!(
        first.snapshot_digest(),
        permuted.snapshot_digest(),
        "input order must not change successor identity"
    );

    let reattributed = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_secondary", &["audit", "public", "types_only"]),
        "postgres_introspector_v9",
        "2030-01-01T00:00:00Z",
        vec![event_relation(RelationKind::Table), audit_view()],
        vec![status_domain(), audit_status_domain()],
        vec![status_enum()],
    )
    .expect("reattributed fixture is valid");
    assert_eq!(
        first.snapshot_digest(),
        reattributed.snapshot_digest(),
        "source key, policy binding, extractor revision and observation time are provenance"
    );
}

#[test]
fn every_v3_evidence_kind_has_a_verified_receipt_coordinate() {
    let snapshot = complete_snapshot();
    let cases = [
        (
            SchemaObjectLocation::table("public", "event_record").expect("table location"),
            SchemaObjectLocationKind::Table,
            "/schemas/public/tables/event_record",
        ),
        (
            SchemaObjectLocation::column("public", "event_record", "parent_key")
                .expect("column location"),
            SchemaObjectLocationKind::Column,
            "/schemas/public/tables/event_record/columns/parent_key",
        ),
        (
            SchemaObjectLocation::constraint("public", "event_record", "event_parent_uq")
                .expect("constraint location"),
            SchemaObjectLocationKind::Constraint,
            "/schemas/public/tables/event_record/constraints/event_parent_uq",
        ),
        (
            SchemaObjectLocation::domain("public", "event_status_kind").expect("domain location"),
            SchemaObjectLocationKind::Domain,
            "/schemas/public/domains/event_status_kind",
        ),
        (
            SchemaObjectLocation::enum_("public", "event_status").expect("enum location"),
            SchemaObjectLocationKind::Enum,
            "/schemas/public/enums/event_status",
        ),
    ];

    for (location, kind, canonical) in cases {
        assert_eq!(location.kind(), kind);
        assert_eq!(location.canonical_location(), canonical);
        let receipt = snapshot
            .source_receipt(location)
            .expect("observed successor coordinate can be receipted");
        assert_eq!(receipt.source_id(), "warehouse_primary");
        assert_eq!(
            receipt.connection_policy_binding(),
            "fixture_policy_revision_a"
        );
        assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
        assert_eq!(receipt.extractor_revision(), "postgres_introspector_v3");
        assert_eq!(receipt.observed_at_utc(), "2026-09-11T00:00:00Z");
        assert_eq!(receipt.location().canonical_location(), canonical);
    }
}

#[test]
fn v3_schema_scoped_canonical_locations_are_collision_safe() {
    let domain =
        SchemaObjectLocation::domain("Sales/~North", "Status/~Kind").expect("domain location");
    assert_eq!(
        domain.canonical_location(),
        "/schemas/Sales~1~0North/domains/Status~1~0Kind"
    );
    let expected = SchemaObjectLocation::enum_("audit", "Mood/State").expect("enum location");
    assert_eq!(
        expected.canonical_location(),
        "/schemas/audit/enums/Mood~1State"
    );
}

#[test]
fn v3_location_accessors_expose_only_the_declared_coordinate() {
    let locations = [
        SchemaObjectLocation::table("public", "event_record").expect("table"),
        SchemaObjectLocation::column("public", "event_record", "event_key").expect("column"),
        SchemaObjectLocation::constraint("public", "event_record", "event_parent_uq")
            .expect("constraint"),
        SchemaObjectLocation::domain("public", "event_status_kind").expect("domain"),
        SchemaObjectLocation::enum_("public", "event_status").expect("enum"),
    ];

    let tables: Vec<Option<&str>> = locations
        .iter()
        .map(SchemaObjectLocation::table_name)
        .collect();
    assert_eq!(
        tables,
        vec![
            Some("event_record"),
            Some("event_record"),
            Some("event_record"),
            None,
            None
        ]
    );

    let columns: Vec<Option<&str>> = locations
        .iter()
        .map(SchemaObjectLocation::column_name)
        .collect();
    assert_eq!(columns, vec![None, Some("event_key"), None, None, None]);

    let constraints: Vec<Option<&str>> = locations
        .iter()
        .map(SchemaObjectLocation::constraint_name)
        .collect();
    assert_eq!(
        constraints,
        vec![None, None, Some("event_parent_uq"), None, None]
    );

    let domains: Vec<Option<&str>> = locations
        .iter()
        .map(SchemaObjectLocation::domain_name)
        .collect();
    assert_eq!(
        domains,
        vec![None, None, None, Some("event_status_kind"), None]
    );

    let enums: Vec<Option<&str>> = locations
        .iter()
        .map(SchemaObjectLocation::enum_name)
        .collect();
    assert_eq!(enums, vec![None, None, None, None, Some("event_status")]);

    for location in &locations {
        assert_eq!(location.schema_name(), "public");
    }
}

#[test]
fn schema_scoped_receipts_cannot_be_satisfied_by_a_different_kind() {
    let snapshot = complete_snapshot();
    let unobserved = [
        SchemaObjectLocation::table("public", "event_status").expect("table shape"),
        SchemaObjectLocation::table("public", "missing_relation").expect("table shape"),
        SchemaObjectLocation::column("public", "event_record", "missing_key")
            .expect("column shape"),
        SchemaObjectLocation::constraint("public", "event_record", "missing_constraint")
            .expect("constraint shape"),
        SchemaObjectLocation::domain("public", "event_status").expect("domain shape"),
        SchemaObjectLocation::domain("public", "missing_domain").expect("domain shape"),
        SchemaObjectLocation::enum_("public", "event_record").expect("enum shape"),
        SchemaObjectLocation::enum_("audit", "event_status").expect("enum shape"),
    ];

    for location in unobserved {
        let expected = location.canonical_location();
        let error = snapshot
            .source_receipt(location)
            .expect_err("an unrelated coordinate in the same schema must not be receipted");
        assert_eq!(
            error,
            ObservationError::UnknownObservationLocation { location: expected }
        );
    }
}

#[test]
fn columns_resolve_to_their_exact_qualified_type_coordinate() {
    let public_kind = QualifiedTypeName::new("public", "event_status_kind").expect("type");
    let audit_kind = QualifiedTypeName::new("audit", "event_status_kind").expect("type");

    let bound_relation = |binding: QualifiedTypeName| {
        RelationObservation::new(
            "public",
            "event_record",
            RelationKind::Table,
            vec![
                ColumnObservationV3::new(
                    "event_status",
                    1,
                    "event_status_kind",
                    binding,
                    false,
                    None,
                )
                .expect("bound column"),
            ],
        )
        .expect("relation fixture")
    };
    let domains = || {
        vec![
            DomainObservation::new("public", "event_status_kind", catalog_type("text"))
                .expect("domain"),
            DomainObservation::new("audit", "event_status_kind", catalog_type("text"))
                .expect("domain"),
        ]
    };

    let public_snapshot = snapshot_v3(
        vec![bound_relation(public_kind.clone())],
        domains(),
        Vec::new(),
    )
    .expect("public binding must resolve");
    let audit_snapshot = snapshot_v3(
        vec![bound_relation(audit_kind.clone())],
        domains(),
        Vec::new(),
    )
    .expect("audit binding must resolve");

    assert_eq!(
        public_snapshot.relations()[0].columns()[0].type_binding(),
        &public_kind
    );
    assert_eq!(
        audit_snapshot.relations()[0].columns()[0].type_binding(),
        &audit_kind
    );
    assert_eq!(
        public_snapshot.relations()[0].columns()[0].data_type(),
        "event_status_kind",
        "display text remains separate from the qualified identity"
    );
    assert_ne!(
        public_snapshot.snapshot_digest(),
        audit_snapshot.snapshot_digest(),
        "same-named types in different schemas must stay distinguishable"
    );
}

#[test]
fn enum_bindings_resolve_without_manufacturing_a_domain() {
    let enum_binding = QualifiedTypeName::new("public", "event_status").expect("type");
    let relation = RelationObservation::new(
        "public",
        "event_record",
        RelationKind::Table,
        vec![
            ColumnObservationV3::new("event_status", 1, "event_status", enum_binding, false, None)
                .expect("bound column"),
        ],
    )
    .expect("relation fixture");
    let snapshot = snapshot_v3(vec![relation], Vec::new(), vec![status_enum()])
        .expect("an enum coordinate is a resolvable type binding");
    assert!(snapshot.domains().is_empty());
}

#[test]
fn fake_type_coordinates_fail_closed() {
    let missing_binding = QualifiedTypeName::new("public", "missing_kind").expect("type");
    let error = snapshot_v3(
        vec![
            RelationObservation::new(
                "public",
                "event_record",
                RelationKind::Table,
                vec![
                    ColumnObservationV3::new(
                        "event_status",
                        1,
                        "event_status",
                        missing_binding,
                        false,
                        None,
                    )
                    .expect("bound column"),
                ],
            )
            .expect("relation fixture"),
        ],
        Vec::new(),
        Vec::new(),
    )
    .expect_err("an unresolvable qualified type coordinate must fail closed");
    assert_eq!(
        error,
        ObservationError::UnknownTypeBinding {
            schema_name: "public".to_owned(),
            type_name: "missing_kind".to_owned(),
        }
    );

    let table_binding = QualifiedTypeName::new("public", "event_record").expect("type");
    let error = snapshot_v3(
        vec![
            RelationObservation::new(
                "public",
                "event_record",
                RelationKind::Table,
                vec![
                    ColumnObservationV3::new(
                        "event_status",
                        1,
                        "event_record",
                        table_binding,
                        false,
                        None,
                    )
                    .expect("bound column"),
                ],
            )
            .expect("relation fixture"),
        ],
        Vec::new(),
        Vec::new(),
    )
    .expect_err("a relation coordinate must not satisfy a type binding");
    assert_eq!(
        error,
        ObservationError::UnknownTypeBinding {
            schema_name: "public".to_owned(),
            type_name: "event_record".to_owned(),
        }
    );
}

#[test]
fn unauthorized_schema_objects_fail_before_a_snapshot_exists() {
    let unauthorized_domain =
        DomainObservation::new("audit", "money_kind", catalog_type("numeric"))
            .expect("domain fixture");
    let error = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T00:00:00Z",
        Vec::new(),
        vec![unauthorized_domain],
        Vec::new(),
    )
    .expect_err("an unauthorized type-only schema must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "unauthorized_schema_name"
        }
    );

    let unauthorized_enum =
        EnumObservation::new("audit", "money_kind", vec!["KRW".to_owned()]).expect("enum fixture");
    let error = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public"]),
        "postgres_introspector_v3",
        "2026-09-11T00:00:00Z",
        Vec::new(),
        Vec::new(),
        vec![unauthorized_enum],
    )
    .expect_err("an unauthorized enum schema must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "unauthorized_schema_name"
        }
    );

    let error = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["audit"]),
        "postgres_introspector_v3",
        "2026-09-11T00:00:00Z",
        vec![event_relation(RelationKind::Table)],
        Vec::new(),
        Vec::new(),
    )
    .expect_err("an unauthorized relation schema must fail closed");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "unauthorized_schema_name"
        }
    );
}

#[test]
fn type_only_authorized_schema_succeeds_without_relations() {
    let domain = DomainObservation::new("types_only", "money_kind", catalog_type("numeric"))
        .expect("domain fixture");
    let observed_enum = EnumObservation::new(
        "types_only",
        "currency_code",
        vec!["KRW".to_owned(), "USD".to_owned()],
    )
    .expect("enum fixture");

    let snapshot = PostgresSchemaSnapshotV3::new(
        &support::authorized_source("warehouse_primary", &["public", "types_only"]),
        "postgres_introspector_v3",
        "2026-09-11T00:00:00Z",
        Vec::new(),
        vec![domain],
        vec![observed_enum],
    )
    .expect("a type-only authorized schema must be observable without a table");
    assert!(snapshot.relations().is_empty());
    assert_eq!(snapshot.domains().len(), 1);
    assert_eq!(snapshot.enums().len(), 1);
    assert!(
        snapshot
            .source_receipt(
                SchemaObjectLocation::domain("types_only", "money_kind").expect("domain location")
            )
            .is_ok()
    );
    assert!(
        snapshot
            .source_receipt(
                SchemaObjectLocation::enum_("types_only", "currency_code").expect("enum location")
            )
            .is_ok()
    );
}

#[test]
fn successor_snapshot_rejects_duplicate_schema_object_coordinates() {
    let duplicate_relation = snapshot_v3(
        vec![
            event_relation(RelationKind::Table),
            base_relation(RelationKind::Table, false, false).with_source_comment("comment"),
        ],
        Vec::new(),
        Vec::new(),
    )
    .expect_err("duplicate relations must fail closed");
    assert_eq!(
        duplicate_relation,
        ObservationError::DuplicateRelationObservation {
            schema_name: "public".to_owned(),
            relation_name: "event_record".to_owned(),
        }
    );

    let duplicate_domain = snapshot_v3(
        Vec::new(),
        vec![status_domain(), status_domain()],
        Vec::new(),
    )
    .expect_err("duplicate domains must fail closed");
    assert_eq!(
        duplicate_domain,
        ObservationError::DuplicateDomainObservation {
            schema_name: "public".to_owned(),
            domain_name: "event_status_kind".to_owned(),
        }
    );

    let duplicate_enum = snapshot_v3(Vec::new(), Vec::new(), vec![status_enum(), status_enum()])
        .expect_err("duplicate enums must fail closed");
    assert_eq!(
        duplicate_enum,
        ObservationError::DuplicateEnumObservation {
            schema_name: "public".to_owned(),
            enum_name: "event_status".to_owned(),
        }
    );

    let collapsed_type_names = snapshot_v3(
        Vec::new(),
        vec![
            DomainObservation::new("public", "event_status", catalog_type("text")).expect("domain"),
        ],
        vec![status_enum()],
    )
    .expect_err("a domain and an enum must not share one type coordinate");
    assert_eq!(
        collapsed_type_names,
        ObservationError::DuplicateSchemaTypeName {
            schema_name: "public".to_owned(),
            type_name: "event_status".to_owned(),
        }
    );
}

#[test]
fn v3_value_objects_reject_ambiguous_or_blank_evidence() {
    assert_eq!(
        QualifiedTypeName::new(" ", "uuid"),
        Err(ObservationError::InvalidObservationField {
            field: "schema_name"
        })
    );
    assert_eq!(
        QualifiedTypeName::new("pg_catalog", "\t"),
        Err(ObservationError::InvalidObservationField { field: "type_name" })
    );
    assert_eq!(
        QualifiedCollationName::new("pg_catalog", "\u{2003}"),
        Err(ObservationError::InvalidObservationField {
            field: "collation_name"
        })
    );
    assert_eq!(
        ColumnObservationV3::new(" ", 1, "uuid", catalog_type("uuid"), false, None),
        Err(ObservationError::InvalidObservationField {
            field: "column_name"
        })
    );
    assert_eq!(
        ColumnObservationV3::new("event_key", 0, "uuid", catalog_type("uuid"), false, None),
        Err(ObservationError::InvalidOrdinalPosition)
    );
    assert_eq!(
        ColumnObservationV3::new("event_key", 1, " ", catalog_type("uuid"), false, None),
        Err(ObservationError::InvalidObservationField { field: "data_type" })
    );
    assert_eq!(
        RelationObservation::new(" ", "event_record", RelationKind::Table, Vec::new()),
        Err(ObservationError::InvalidObservationField {
            field: "schema_name"
        })
    );
    assert_eq!(
        RelationObservation::new("public", " ", RelationKind::Table, Vec::new()),
        Err(ObservationError::InvalidObservationField {
            field: "relation_name"
        })
    );
    assert_eq!(
        DomainObservation::new("public", " ", catalog_type("text")),
        Err(ObservationError::InvalidObservationField {
            field: "domain_name"
        })
    );
    assert_eq!(
        EnumObservation::new("public", " ", Vec::new()),
        Err(ObservationError::InvalidObservationField { field: "enum_name" })
    );
    assert_eq!(
        DomainCheckConstraintObservation::new(" ", "CHECK (true)", true, true),
        Err(ObservationError::InvalidObservationField {
            field: "constraint_name"
        })
    );
    assert_eq!(
        DomainCheckConstraintObservation::new("event_status_kind_allowed", " ", true, true),
        Err(ObservationError::InvalidObservationField {
            field: "check_definition"
        })
    );

    assert_eq!(
        SchemaObjectLocation::table(" ", "event_record"),
        Err(ObservationError::InvalidObservationField {
            field: "schema_name"
        })
    );
    assert_eq!(
        SchemaObjectLocation::table("public", " "),
        Err(ObservationError::InvalidObservationField {
            field: "table_name"
        })
    );
    assert_eq!(
        SchemaObjectLocation::column("public", "event_record", " "),
        Err(ObservationError::InvalidObservationField {
            field: "column_name"
        })
    );
    assert_eq!(
        SchemaObjectLocation::constraint("public", "event_record", "\n"),
        Err(ObservationError::InvalidObservationField {
            field: "constraint_name"
        })
    );
    assert_eq!(
        SchemaObjectLocation::domain("public", " "),
        Err(ObservationError::InvalidObservationField {
            field: "domain_name"
        })
    );
    assert_eq!(
        SchemaObjectLocation::enum_("public", " "),
        Err(ObservationError::InvalidObservationField { field: "enum_name" })
    );
}

#[test]
fn v3_relations_reject_duplicate_or_unknown_inner_coordinates() {
    let duplicate_column = RelationObservation::new(
        "public",
        "event_record",
        RelationKind::Table,
        vec![
            bound_column("event_key", 1, "uuid", catalog_type("uuid"), false),
            bound_column("event_key", 2, "uuid", catalog_type("uuid"), false),
        ],
    )
    .expect_err("duplicate bound columns must fail closed");
    assert_eq!(
        duplicate_column,
        ObservationError::DuplicateColumnName {
            schema_name: "public".to_owned(),
            table_name: "event_record".to_owned(),
            column_name: "event_key".to_owned(),
        }
    );

    let duplicate_ordinal = RelationObservation::new(
        "public",
        "event_record",
        RelationKind::Table,
        vec![
            bound_column("event_key", 1, "uuid", catalog_type("uuid"), false),
            bound_column("parent_key", 1, "uuid", catalog_type("uuid"), false),
        ],
    )
    .expect_err("duplicate ordinals must fail closed");
    assert_eq!(
        duplicate_ordinal,
        ObservationError::DuplicateColumnOrdinal {
            schema_name: "public".to_owned(),
            table_name: "event_record".to_owned(),
            ordinal_position: 1,
        }
    );

    let duplicate_constraint = base_relation(RelationKind::Table, false, false)
        .with_constraints(vec![
            TableConstraintObservation::Unique(
                UniqueConstraintObservation::new("event_parent_uq", vec!["parent_key".to_owned()])
                    .expect("unique fixture"),
            ),
            TableConstraintObservation::Check(
                CheckConstraintObservation::new(
                    "event_parent_uq",
                    "CHECK ((parent_key IS NOT NULL))",
                    true,
                    true,
                    false,
                )
                .expect("check fixture"),
            ),
        ])
        .expect_err("duplicate constraint names must fail closed");
    assert_eq!(
        duplicate_constraint,
        ObservationError::DuplicateConstraintName {
            schema_name: "public".to_owned(),
            table_name: "event_record".to_owned(),
            constraint_name: "event_parent_uq".to_owned(),
        }
    );

    let unknown_constraint_column = base_relation(RelationKind::Table, false, false)
        .with_constraints(vec![TableConstraintObservation::Unique(
            UniqueConstraintObservation::new("event_missing_uq", vec!["missing_key".to_owned()])
                .expect("unique fixture"),
        )])
        .expect_err("constraints must reference observed boundary columns");
    assert_eq!(
        unknown_constraint_column,
        ObservationError::UnknownConstraintColumn {
            schema_name: "public".to_owned(),
            table_name: "event_record".to_owned(),
            constraint_name: "event_missing_uq".to_owned(),
            column_name: "missing_key".to_owned(),
        }
    );

    let duplicate_domain_check =
        DomainObservation::new("public", "event_status_kind", catalog_type("text"))
            .expect("domain fixture")
            .with_check_constraints(vec![
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE <> ''::text))",
                    true,
                    true,
                )
                .expect("check fixture"),
                DomainCheckConstraintObservation::new(
                    "event_status_kind_allowed",
                    "CHECK ((VALUE IS NOT NULL))",
                    true,
                    true,
                )
                .expect("check fixture"),
            ])
            .expect_err("duplicate domain check names must fail closed");
    assert_eq!(
        duplicate_domain_check,
        ObservationError::DuplicateDomainCheckConstraint {
            schema_name: "public".to_owned(),
            domain_name: "event_status_kind".to_owned(),
            constraint_name: "event_status_kind_allowed".to_owned(),
        }
    );

    let duplicate_enum_label = EnumObservation::new(
        "public",
        "event_status",
        vec!["pending".to_owned(), "pending".to_owned()],
    )
    .expect_err("duplicate enum labels must fail closed");
    assert_eq!(
        duplicate_enum_label,
        ObservationError::DuplicateEnumLabel {
            schema_name: "public".to_owned(),
            enum_name: "event_status".to_owned(),
            label: "pending".to_owned(),
        }
    );
}

#[test]
fn enum_labels_preserve_exact_source_text_including_empty_labels() {
    let observed_enum = EnumObservation::new(
        "public",
        "event_status",
        vec![String::new(), "pending".to_owned()],
    )
    .expect("an empty enum label is exact source text, not a missing value");
    assert_eq!(observed_enum.labels(), ["", "pending"]);
}

#[test]
fn index_evidence_is_material_successor_identity() {
    let base = digest_of(vec![indexed_relation()], Vec::new(), Vec::new());

    let no_predicate = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
    )
    .expect("index fixture")
    .with_source_comment("observed index comment");
    let non_unique = IndexObservation::new(
        "event_parent_ix",
        false,
        None,
        vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
    )
    .expect("index fixture")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("observed index comment");
    let nulls_not_distinct = IndexObservation::new(
        "event_parent_ix",
        true,
        Some(true),
        vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
    )
    .expect("index fixture")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("observed index comment");
    let role_swapped = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![index_attribute(
            1,
            IndexAttributeKind::Include,
            "parent_key",
        )],
        vec![index_attribute(2, IndexAttributeKind::Key, "event_key")],
    )
    .expect("index fixture")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("observed index comment");
    let reordered_attributes = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![index_attribute(1, IndexAttributeKind::Key, "event_key")],
        vec![index_attribute(
            2,
            IndexAttributeKind::Include,
            "parent_key",
        )],
    )
    .expect("index fixture")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("observed index comment");
    let renamed = IndexObservation::new(
        "event_parent_ix_v2",
        true,
        None,
        vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
    )
    .expect("index fixture")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("observed index comment");
    let comment_only = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
        vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
    )
    .expect("index fixture")
    .with_predicate("(parent_key IS NOT NULL)")
    .with_source_comment("changed index comment");

    let variants = [
        IndexObservation::new(
            "event_parent_ix",
            true,
            None,
            vec![index_attribute(1, IndexAttributeKind::Key, "parent_key")],
            vec![index_attribute(2, IndexAttributeKind::Include, "event_key")],
        )
        .expect("index fixture"),
        no_predicate,
        non_unique,
        nulls_not_distinct,
        role_swapped,
        reordered_attributes,
        renamed,
        comment_only,
    ];

    assert_ne!(
        digest_of(
            vec![event_relation(RelationKind::Table)],
            Vec::new(),
            Vec::new()
        ),
        base,
        "adding index evidence must change successor identity"
    );
    for (index_number, variant) in variants.into_iter().enumerate() {
        let relation = event_relation(RelationKind::Table)
            .with_indexes(vec![variant])
            .expect("variant index references observed columns");
        assert_ne!(
            base,
            digest_of(vec![relation], Vec::new(), Vec::new()),
            "material index variant {index_number} must change successor identity"
        );
    }
}

#[test]
fn index_coordinates_receive_verified_receipts() {
    let snapshot = snapshot_v3(vec![indexed_relation()], Vec::new(), Vec::new())
        .expect("indexed fixture snapshot is valid");

    let location = SchemaObjectLocation::index("public", "event_record", "event_parent_ix")
        .expect("index location");
    assert_eq!(location.kind(), SchemaObjectLocationKind::Index);
    assert_eq!(
        location.canonical_location(),
        "/schemas/public/tables/event_record/indexes/event_parent_ix"
    );
    assert_eq!(location.table_name(), Some("event_record"));
    assert_eq!(location.index_name(), Some("event_parent_ix"));
    assert_eq!(location.column_name(), None);
    assert_eq!(location.constraint_name(), None);
    assert_eq!(location.domain_name(), None);
    assert_eq!(location.enum_name(), None);

    let receipt = snapshot
        .source_receipt(location)
        .expect("observed index coordinate can be receipted");
    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
}

#[test]
fn index_locational_coordinates_are_collision_safe() {
    let location = SchemaObjectLocation::index("Sales/~North", "Event/Record", "Parent/~Ix")
        .expect("index location");
    assert_eq!(
        location.canonical_location(),
        "/schemas/Sales~1~0North/tables/Event~1Record/indexes/Parent~1~0Ix"
    );
}

#[test]
fn index_attributes_are_canonicalized_in_position_order() {
    let index = IndexObservation::new(
        "event_parent_ix",
        true,
        None,
        vec![
            index_attribute(2, IndexAttributeKind::Key, "event_key"),
            index_attribute(1, IndexAttributeKind::Key, "parent_key"),
        ],
        vec![
            index_attribute(4, IndexAttributeKind::Include, "event_key"),
            index_attribute(3, IndexAttributeKind::Include, "parent_key"),
        ],
    )
    .expect("out-of-order attributes are canonicalized, not rejected");

    let key_positions: Vec<u32> = index
        .key_attributes()
        .iter()
        .map(IndexAttributeObservation::position)
        .collect();
    let include_positions: Vec<u32> = index
        .include_attributes()
        .iter()
        .map(IndexAttributeObservation::position)
        .collect();
    assert_eq!(key_positions, vec![1, 2]);
    assert_eq!(include_positions, vec![3, 4]);
    assert_eq!(
        index.key_attributes()[0].attribute_name(),
        Some("parent_key")
    );
    assert_eq!(index.key_attributes()[0].kind(), IndexAttributeKind::Key);
}

#[test]
fn relation_indexes_reject_duplicate_or_unknown_coordinates() {
    let duplicate_index = event_relation(RelationKind::Table)
        .with_indexes(vec![event_index(), event_index()])
        .expect_err("duplicate index names must fail closed");
    assert_eq!(
        duplicate_index,
        ObservationError::DuplicateIndexObservation {
            schema_name: "public".to_owned(),
            relation_name: "event_record".to_owned(),
            index_name: "event_parent_ix".to_owned(),
        }
    );

    let duplicate_position = event_relation(RelationKind::Table)
        .with_indexes(vec![
            IndexObservation::new(
                "event_parent_ix",
                true,
                None,
                vec![
                    index_attribute(1, IndexAttributeKind::Key, "parent_key"),
                    index_attribute(1, IndexAttributeKind::Key, "event_key"),
                ],
                Vec::new(),
            )
            .expect("index fixture"),
        ])
        .expect_err("duplicate attribute positions must fail closed");
    assert_eq!(
        duplicate_position,
        ObservationError::DuplicateIndexAttribute {
            schema_name: "public".to_owned(),
            relation_name: "event_record".to_owned(),
            index_name: "event_parent_ix".to_owned(),
            position: 1,
        }
    );

    let unknown_attribute = event_relation(RelationKind::Table)
        .with_indexes(vec![
            IndexObservation::new(
                "event_parent_ix",
                true,
                None,
                vec![index_attribute(1, IndexAttributeKind::Key, "missing_key")],
                Vec::new(),
            )
            .expect("index fixture"),
        ])
        .expect_err("index attributes must resolve to observed relation columns");
    assert_eq!(
        unknown_attribute,
        ObservationError::UnknownIndexAttribute {
            schema_name: "public".to_owned(),
            relation_name: "event_record".to_owned(),
            index_name: "event_parent_ix".to_owned(),
            attribute_name: "missing_key".to_owned(),
        }
    );
}

#[test]
fn index_value_objects_reject_blank_or_zero_evidence() {
    assert_eq!(
        IndexObservation::new(" ", true, None, Vec::new(), Vec::new()),
        Err(ObservationError::InvalidObservationField {
            field: "index_name"
        })
    );
    assert_eq!(
        IndexAttributeObservation::new(1, IndexAttributeKind::Key, " "),
        Err(ObservationError::InvalidObservationField {
            field: "attribute_name"
        })
    );
    assert_eq!(
        IndexAttributeObservation::new(0, IndexAttributeKind::Key, "parent_key"),
        Err(ObservationError::InvalidOrdinalPosition)
    );
    assert_eq!(
        SchemaObjectLocation::index("public", " ", "event_parent_ix"),
        Err(ObservationError::InvalidObservationField {
            field: "table_name"
        })
    );
    assert_eq!(
        SchemaObjectLocation::index("public", "event_record", " "),
        Err(ObservationError::InvalidObservationField {
            field: "index_name"
        })
    );
}

#[test]
fn index_readiness_validity_liveness_and_definition_are_material() {
    let base = digest_of(
        vec![
            event_relation(RelationKind::Table)
                .with_indexes(vec![
                    event_index()
                        .with_ready(true)
                        .with_valid(true)
                        .with_live(true),
                ])
                .expect("observed index"),
        ],
        Vec::new(),
        Vec::new(),
    );
    let variants = [
        digest_of(
            vec![event_relation(RelationKind::Table)
                .with_indexes(vec![event_index()
                    .with_ready(false)
                    .with_valid(true)
                    .with_live(true)])
                .expect("observed index")],
            Vec::new(),
            Vec::new(),
        ),
        digest_of(
            vec![event_relation(RelationKind::Table)
                .with_indexes(vec![event_index()
                    .with_ready(true)
                    .with_valid(false)
                    .with_live(true)])
                .expect("observed index")],
            Vec::new(),
            Vec::new(),
        ),
        digest_of(
            vec![event_relation(RelationKind::Table)
                .with_indexes(vec![event_index()
                    .with_ready(true)
                    .with_valid(true)
                    .with_live(false)])
                .expect("observed index")],
            Vec::new(),
            Vec::new(),
        ),
        digest_of(
            vec![event_relation(RelationKind::Table)
                .with_indexes(vec![event_index()
                    .with_ready(true)
                    .with_valid(true)
                    .with_live(true)
                    .with_access_method("hash")])
                .expect("observed index")],
            Vec::new(),
            Vec::new(),
        ),
        digest_of(
            vec![event_relation(RelationKind::Table)
                .with_indexes(vec![event_index()
                    .with_ready(true)
                    .with_valid(true)
                    .with_live(true)
                    .with_index_definition(
                        "CREATE UNIQUE INDEX event_parent_ix ON public.event_record USING btree (parent_key)",
                    )])
                .expect("observed index")],
            Vec::new(),
            Vec::new(),
        ),
    ];

    for (index_number, variant) in variants.into_iter().enumerate() {
        assert_ne!(
            base, variant,
            "index readiness/validity/liveness/definition variant {index_number} must change successor identity"
        );
    }
}

#[test]
fn expression_indexes_are_structurally_distinct_from_column_indexes() {
    let expression_snapshot = snapshot_v3(
        vec![
            event_relation(RelationKind::Table)
                .with_indexes(vec![expression_index()])
                .expect("expression index fixture is valid"),
        ],
        Vec::new(),
        Vec::new(),
    )
    .expect("expression index snapshot is valid");
    let observed = &expression_snapshot.relations()[0].indexes()[0];
    let attribute = &observed.key_attributes()[0];
    assert_eq!(attribute.attribute_name(), None);
    assert_eq!(attribute.expression_text(), Some("lower(parent_key)"));
    assert_eq!(observed.access_method(), Some("btree"));
    assert_eq!(observed.ready(), Some(true));
    assert_eq!(observed.valid(), Some(true));
    assert_eq!(observed.live(), Some(false));
    assert!(observed.index_definition().is_some());

    let column_snapshot = snapshot_v3(vec![indexed_relation()], Vec::new(), Vec::new())
        .expect("column index snapshot is valid");
    assert_ne!(
        expression_snapshot.snapshot_digest(),
        column_snapshot.snapshot_digest(),
        "the same position as an expression and as a column must not collapse"
    );
}

#[test]
fn index_attribute_rejects_a_blank_expression() {
    assert_eq!(
        IndexAttributeObservation::expression(1, IndexAttributeKind::Key, " "),
        Err(ObservationError::InvalidObservationField {
            field: "expression"
        })
    );
    assert_eq!(
        IndexAttributeObservation::expression(0, IndexAttributeKind::Key, "lower(x)"),
        Err(ObservationError::InvalidOrdinalPosition)
    );
}

#[test]
fn index_receipts_cannot_be_satisfied_by_a_different_kind_or_relation() {
    let snapshot = snapshot_v3(vec![indexed_relation()], Vec::new(), Vec::new())
        .expect("indexed fixture snapshot is valid");
    let unobserved = [
        SchemaObjectLocation::index("public", "event_record", "missing_ix").expect("index shape"),
        SchemaObjectLocation::index("public", "missing_relation", "event_parent_ix")
            .expect("index shape"),
        SchemaObjectLocation::constraint("public", "event_record", "event_parent_ix")
            .expect("constraint shape"),
    ];

    for location in unobserved {
        let expected = location.canonical_location();
        let error = snapshot
            .source_receipt(location)
            .expect_err("an unrelated coordinate must not be receipted as an index");
        assert_eq!(
            error,
            ObservationError::UnknownObservationLocation { location: expected }
        );
    }
}
