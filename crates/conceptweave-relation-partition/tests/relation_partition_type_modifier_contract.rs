use conceptweave_observation::{
    ColumnObservationV3, ObservationError, PostgresSchemaSnapshotV3, QualifiedTypeName,
    RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    ColumnTypeModifierLocation, ColumnTypeModifierObservation, PartitionParentRelationCoordinate,
    RelationPartitionObservation, RelationPartitionSnapshot, RelationPartitionTypeModifierSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry<'a> {
    source: &'a str,
    binding: &'a str,
    schemas: &'a [&'a str],
}

impl SourceConnectionRegistry for Registry<'_> {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == self.source
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == self.source).then(|| self.binding.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == self.source
            && source_connection.connection_policy_binding() == self.binding
            && allowed_schema_names
                .iter()
                .map(String::as_str)
                .eq(self.schemas.iter().copied())
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == self.source
            && source_connection.connection_policy_binding() == self.binding
            && resource_envelope.request_budget().max_schema_count() <= self.schemas.len()
            && resource_envelope.request_budget().max_schema_bytes() <= 256
            && resource_envelope.limits().operation_timeout_ms() <= 1_000
            && resource_envelope.limits().statement_timeout_ms() <= 1_000
            && resource_envelope.limits().max_rows() <= 10
            && resource_envelope.limits().max_bytes() <= 1_024
            && resource_envelope.limits().max_concurrent_queries() <= 1
    }
}

fn authorized_source() -> AuthorizedObservationRequest {
    authorized_source_with_context("warehouse_primary", POLICY_BINDING)
}

fn authorized_source_with_context(source: &str, binding: &str) -> AuthorizedObservationRequest {
    authorized_source_with_scope(source, binding, &["public"])
}

fn authorized_source_with_scope(
    source: &str,
    binding: &str,
    schemas: &[&str],
) -> AuthorizedObservationRequest {
    ObservationRequest::new(
        source,
        schemas.iter().map(|schema| (*schema).to_owned()).collect(),
        ObservationRequestBudget::new(schemas.len(), 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry {
        source,
        binding,
        schemas,
    })
    .unwrap()
}

fn column(name: &str, ordinal: u32, data_type: &str, type_name: &str) -> ColumnObservationV3 {
    ColumnObservationV3::new(
        name,
        ordinal,
        data_type,
        QualifiedTypeName::new("pg_catalog", type_name).unwrap(),
        true,
        None,
    )
    .unwrap()
}

fn relation(
    name: &str,
    kind: RelationKind,
    columns: Vec<ColumnObservationV3>,
) -> RelationObservation {
    RelationObservation::new("public", name, kind, columns).unwrap()
}

fn base_snapshot() -> PostgresSchemaSnapshotV3 {
    PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-partition-atttypmod-v1",
        "2026-09-15T04:44:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                vec![
                    column("account_code", 1, "character varying", "varchar"),
                    column("account_email", 2, "text", "text"),
                ],
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                vec![
                    column("account_email", 1, "text", "text"),
                    column("account_code", 2, "character varying", "varchar"),
                ],
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap()
}

fn relation_partition_snapshot(base: &PostgresSchemaSnapshotV3) -> RelationPartitionSnapshot {
    RelationPartitionSnapshot::new(
        base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "public",
                "accounts_2026",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "accounts").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap()
}

fn modifier(
    relation_name: &str,
    relation_kind: RelationKind,
    column_name: &str,
    atttypmod: i32,
) -> ColumnTypeModifierObservation {
    ColumnTypeModifierObservation::new(
        "public",
        relation_name,
        relation_kind,
        column_name,
        atttypmod,
    )
    .unwrap()
}

fn complete_modifiers(
    parent_varchar: i32,
    child_varchar: i32,
) -> Vec<ColumnTypeModifierObservation> {
    vec![
        modifier(
            "accounts",
            RelationKind::PartitionedTable,
            "account_code",
            parent_varchar,
        ),
        modifier(
            "accounts",
            RelationKind::PartitionedTable,
            "account_email",
            -1,
        ),
        modifier("accounts_2026", RelationKind::Table, "account_email", -1),
        modifier(
            "accounts_2026",
            RelationKind::Table,
            "account_code",
            child_varchar,
        ),
    ]
}

#[test]
fn raw_type_modifier_mismatch_fails_even_when_rendered_type_text_matches() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);

    let error = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 68),
    )
    .expect_err(
        "PostgreSQL build_attrmap_by_name rejects equal type OIDs with different raw atttypmod",
    );

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_type_modifier",
        }
    );
}

#[test]
fn complete_equal_raw_type_modifiers_preserve_name_mapping_across_physical_order() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);

    let snapshot = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 36),
    )
    .expect("same-name parent and child columns with equal raw atttypmod are admissible");

    assert_eq!(snapshot.observations().len(), 4);
    assert!(snapshot.snapshot_digest().starts_with("sha256:"));
}

#[test]
fn raw_negative_one_type_modifier_is_preserved_as_catalog_evidence() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let snapshot = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(-1, -1),
    )
    .expect("atttypmod=-1 is a valid exact PostgreSQL catalog value");

    let observation = snapshot
        .observations()
        .iter()
        .find(|observation| {
            observation.relation_name() == "accounts" && observation.column_name() == "account_code"
        })
        .unwrap();
    assert_eq!(observation.type_modifier(), -1);
}

#[test]
fn structured_type_modifier_evidence_must_cover_every_bounded_column_once() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let mut observations = complete_modifiers(36, 36);
    observations.pop();

    let error =
        RelationPartitionTypeModifierSnapshot::new(&base, &relation_partition, observations)
            .expect_err("absence is not equivalent to atttypmod=-1");
    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_type_modifier_completeness",
        }
    );
}

#[test]
fn structured_type_modifier_changes_successor_identity_without_rewriting_predecessor() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let varchar_32 = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 36),
    )
    .unwrap();
    let unconstrained = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(-1, -1),
    )
    .unwrap();

    assert_ne!(
        varchar_32.snapshot_digest(),
        unconstrained.snapshot_digest()
    );
    assert_eq!(
        relation_partition.snapshot_digest(),
        relation_partition_snapshot(&base).snapshot_digest()
    );
}

#[test]
fn exact_column_receipt_uses_structured_modifier_successor_digest() {
    let base = base_snapshot();
    let relation_partition = relation_partition_snapshot(&base);
    let snapshot = RelationPartitionTypeModifierSnapshot::new(
        &base,
        &relation_partition,
        complete_modifiers(36, 36),
    )
    .unwrap();

    let location = ColumnTypeModifierLocation::new(
        "public",
        "accounts_2026",
        RelationKind::Table,
        "account_code",
    )
    .unwrap();
    let receipt = snapshot.source_receipt(location).unwrap();

    assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
    assert_eq!(receipt.location().column_name(), "account_code");
    assert!(
        receipt
            .location()
            .canonical_location()
            .ends_with("/columns/account_code/type-modifier")
    );
}

#[test]
fn modifier_receipts_require_every_exact_coordinate_component() {
    let base = base_snapshot();
    let predecessor = relation_partition_snapshot(&base);
    let snapshot =
        RelationPartitionTypeModifierSnapshot::new(&base, &predecessor, complete_modifiers(36, 36))
            .unwrap();
    for (schema, relation, kind, column) in [
        (
            "other",
            "accounts_2026",
            RelationKind::Table,
            "account_code",
        ),
        ("public", "other", RelationKind::Table, "account_code"),
        (
            "public",
            "accounts_2026",
            RelationKind::PartitionedTable,
            "account_code",
        ),
        ("public", "accounts_2026", RelationKind::Table, "other"),
    ] {
        let location = ColumnTypeModifierLocation::new(schema, relation, kind, column).unwrap();
        assert_eq!(
            snapshot.source_receipt(location.clone()),
            Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location()
            })
        );
    }
    let location = ColumnTypeModifierLocation::new(
        "public",
        "accounts_2026",
        RelationKind::Table,
        "account_code",
    )
    .unwrap();
    let receipt = snapshot.source_receipt(location).unwrap();
    assert_eq!(receipt.source_id(), base.source_connection_key());
    assert_eq!(receipt.connection_policy_binding(), POLICY_BINDING);
    assert_eq!(receipt.extractor_revision(), base.extractor_revision());
    assert_eq!(receipt.observed_at_utc(), base.observed_at_utc());
}

#[test]
fn duplicate_modifier_inventory_is_rejected_and_order_is_not_identity() {
    let base = base_snapshot();
    let predecessor = relation_partition_snapshot(&base);
    let observations = complete_modifiers(36, 36);
    let snapshot =
        RelationPartitionTypeModifierSnapshot::new(&base, &predecessor, observations.clone())
            .unwrap();
    let mut reversed = observations.clone();
    reversed.reverse();
    assert_eq!(
        snapshot.snapshot_digest(),
        RelationPartitionTypeModifierSnapshot::new(&base, &predecessor, reversed,)
            .unwrap()
            .snapshot_digest()
    );
    let mut duplicate = observations;
    duplicate.push(duplicate[0].clone());
    assert_eq!(
        RelationPartitionTypeModifierSnapshot::new(&base, &predecessor, duplicate),
        Err(ObservationError::InvalidObservationField {
            field: "relation_partition_column_type_modifier_coordinate",
        })
    );
}

#[test]
fn modifier_predecessor_cannot_cross_revision_or_observation_time() {
    let base = base_snapshot();
    let predecessor = relation_partition_snapshot(&base);
    for (revision, time) in [
        ("different-extractor", base.observed_at_utc()),
        (base.extractor_revision(), "2026-09-15T04:44:01Z"),
    ] {
        let changed = PostgresSchemaSnapshotV3::new(
            &authorized_source(),
            revision,
            time,
            base.relations().to_vec(),
            vec![],
            vec![],
        )
        .unwrap();
        assert_eq!(changed.snapshot_digest(), base.snapshot_digest());
        assert_eq!(
            RelationPartitionTypeModifierSnapshot::new(
                &changed,
                &predecessor,
                complete_modifiers(36, 36),
            ),
            Err(ObservationError::InvalidObservationField {
                field: "relation_partition_type_modifier_predecessor",
            })
        );
    }
}

#[test]
fn modifier_predecessor_cannot_cross_authorized_source_or_policy() {
    let base = base_snapshot();
    let predecessor = relation_partition_snapshot(&base);
    for (source, binding) in [
        ("warehouse_secondary", POLICY_BINDING),
        ("warehouse_primary", "fixture_policy_revision_b"),
    ] {
        let changed = PostgresSchemaSnapshotV3::new(
            &authorized_source_with_context(source, binding),
            base.extractor_revision(),
            base.observed_at_utc(),
            base.relations().to_vec(),
            vec![],
            vec![],
        )
        .unwrap();
        assert_eq!(changed.snapshot_digest(), base.snapshot_digest());
        assert_eq!(
            RelationPartitionTypeModifierSnapshot::new(
                &changed,
                &predecessor,
                complete_modifiers(36, 36),
            ),
            Err(ObservationError::InvalidObservationField {
                field: "relation_partition_type_modifier_predecessor",
            })
        );
    }
}

#[test]
fn modifier_and_receipt_coordinates_reject_empty_and_nul_identifiers() {
    for invalid in ["", "bad\0name"] {
        for (schema, relation, column, field) in [
            (invalid, "accounts", "account_code", "schema_name"),
            ("public", invalid, "account_code", "relation_name"),
            ("public", "accounts", invalid, "column_name"),
        ] {
            let observation = ColumnTypeModifierObservation::new(
                schema,
                relation,
                RelationKind::PartitionedTable,
                column,
                -1,
            );
            let location = ColumnTypeModifierLocation::new(
                schema,
                relation,
                RelationKind::PartitionedTable,
                column,
            );
            match observation.unwrap_err() {
                ObservationError::InvalidObservationField { field: actual } => {
                    assert_eq!(actual, format!("relation_partition_type_modifier_{field}"))
                }
                other => panic!("unexpected observation error: {other:?}"),
            }
            match location.unwrap_err() {
                ObservationError::InvalidObservationField { field: actual } => assert_eq!(
                    actual,
                    format!("relation_partition_type_modifier_location_{field}")
                ),
                other => panic!("unexpected receipt error: {other:?}"),
            }
        }
    }
}

#[test]
fn modifier_predecessor_cannot_reuse_changed_source_content() {
    let base = base_snapshot();
    let predecessor = relation_partition_snapshot(&base);
    let mut relations = base.relations().to_vec();
    relations[0] = relations[0]
        .clone()
        .with_source_comment("changed observed relation comment");
    let changed = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        base.extractor_revision(),
        base.observed_at_utc(),
        relations,
        vec![],
        vec![],
    )
    .unwrap();
    assert_ne!(changed.snapshot_digest(), base.snapshot_digest());
    assert_eq!(
        RelationPartitionTypeModifierSnapshot::new(
            &changed,
            &predecessor,
            complete_modifiers(36, 36),
        ),
        Err(ObservationError::InvalidObservationField {
            field: "relation_partition_type_modifier_predecessor",
        })
    );
    let changed_predecessor = relation_partition_snapshot(&changed);
    assert!(
        RelationPartitionTypeModifierSnapshot::new(
            &changed,
            &changed_predecessor,
            complete_modifiers(36, 36),
        )
        .is_ok()
    );
}

#[test]
fn cross_schema_same_name_partitions_bind_modifiers_to_exact_parent_coordinates() {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source_with_scope("warehouse_primary", POLICY_BINDING, &["archive", "public"]),
        "extractor-relation-partition-atttypmod-v1",
        "2026-09-15T04:44:00Z",
        vec![
            RelationObservation::new(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
                vec![column("account_code", 1, "character varying", "varchar")],
            )
            .unwrap(),
            RelationObservation::new(
                "archive",
                "accounts",
                RelationKind::Table,
                vec![column("account_code", 1, "character varying", "varchar")],
            )
            .unwrap(),
        ],
        vec![],
        vec![],
    )
    .unwrap();
    let predecessor = RelationPartitionSnapshot::new(
        &base,
        vec![
            RelationPartitionObservation::non_partition(
                "public",
                "accounts",
                RelationKind::PartitionedTable,
            )
            .unwrap(),
            RelationPartitionObservation::partition(
                "archive",
                "accounts",
                RelationKind::Table,
                PartitionParentRelationCoordinate::new("public", "accounts").unwrap(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    for child_modifier in [36, 68] {
        let result = RelationPartitionTypeModifierSnapshot::new(
            &base,
            &predecessor,
            vec![
                ColumnTypeModifierObservation::new(
                    "public",
                    "accounts",
                    RelationKind::PartitionedTable,
                    "account_code",
                    36,
                )
                .unwrap(),
                ColumnTypeModifierObservation::new(
                    "archive",
                    "accounts",
                    RelationKind::Table,
                    "account_code",
                    child_modifier,
                )
                .unwrap(),
            ],
        );
        if child_modifier == 68 {
            assert_eq!(
                result,
                Err(ObservationError::InvalidObservationField {
                    field: "relation_partition_column_type_modifier",
                })
            );
        } else {
            let snapshot = result.unwrap();
            assert_eq!(snapshot.observations().len(), 2);
            for (schema, kind) in [
                ("archive", RelationKind::Table),
                ("public", RelationKind::PartitionedTable),
            ] {
                let location =
                    ColumnTypeModifierLocation::new(schema, "accounts", kind, "account_code")
                        .unwrap();
                let receipt = snapshot.source_receipt(location.clone()).unwrap();
                assert_eq!(receipt.location(), &location);
                assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
            }
        }
    }
}
