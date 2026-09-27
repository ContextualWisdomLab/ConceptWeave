use conceptweave_observation::{
    ColumnObservationV3, IndexAttributeKind, IndexAttributeObservation, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, ObservationError, PostgresSchemaSnapshotV3,
    QualifiedOperatorClassName, QualifiedTypeName, RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    IndexExclusionSemanticsSnapshot, IndexExpressionSemanticsSnapshot,
    IndexKeyExclusionSemanticsObservation, IndexKeyOperatorFamilyObservation,
    IndexOperatorFamilySnapshot, IndexPartitionCoordinate, IndexPartitionObservation,
    IndexPartitionSnapshot, IndexRelationKind, PartitionParentRelationCoordinate,
    QualifiedOperatorFamilyName, QualifiedOperatorSignature, QualifiedProcedureSignature,
    RelationPartitionObservation, RelationPartitionSnapshot,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
};

const POLICY_BINDING: &str = "fixture_policy_revision_a";

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, source_connection_key: &str) -> bool {
        source_connection_key == "warehouse_primary"
    }

    fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
        (source_connection_key == "warehouse_primary").then(|| POLICY_BINDING.to_owned())
    }

    fn authorizes_schema_scope(
        &self,
        source_connection: &ResolvedSourceConnection,
        allowed_schema_names: &[String],
    ) -> bool {
        source_connection.source_connection_key() == "warehouse_primary"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && allowed_schema_names == ["public"]
    }

    fn authorizes_resource_envelope(
        &self,
        source_connection: &ResolvedSourceConnection,
        resource_envelope: ObservationResourceEnvelope,
    ) -> bool {
        source_connection.source_connection_key() == "warehouse_primary"
            && source_connection.connection_policy_binding() == POLICY_BINDING
            && resource_envelope.request_budget().max_schema_count() <= 1
            && resource_envelope.request_budget().max_schema_bytes() <= 256
            && resource_envelope.limits().operation_timeout_ms() <= 1_000
            && resource_envelope.limits().statement_timeout_ms() <= 1_000
            && resource_envelope.limits().max_rows() <= 10
            && resource_envelope.limits().max_bytes() <= 1_024
            && resource_envelope.limits().max_concurrent_queries() <= 1
    }
}

fn authorized_source() -> AuthorizedObservationRequest {
    ObservationRequest::new(
        "warehouse_primary",
        vec!["public".to_owned()],
        ObservationRequestBudget::new(1, 256).unwrap(),
        ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn index(name: &str, exclusion: Option<bool>) -> IndexObservation {
    let index = IndexObservation::new(
        name,
        false,
        Some(false),
        vec![IndexAttributeObservation::new(1, IndexAttributeKind::Key, "account_id").unwrap()],
        vec![],
    )
    .unwrap()
    .with_access_method("btree")
    .with_key_semantics(vec![
        IndexKeySemantics::new(
            1,
            None,
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
            0,
        )
        .unwrap(),
    ])
    .unwrap()
    .with_valid(true);
    match exclusion {
        Some(exclusion) => index
            .with_catalog_flags(IndexCatalogFlags::new(
                false, exclusion, true, false, false, false,
            ))
            .unwrap(),
        None => index,
    }
}

fn relation(
    name: &str,
    kind: RelationKind,
    index_name: &str,
    exclusion: Option<bool>,
) -> RelationObservation {
    RelationObservation::new(
        "public",
        name,
        kind,
        vec![
            ColumnObservationV3::new(
                "account_id",
                1,
                "integer",
                QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
                false,
                None,
            )
            .unwrap(),
        ],
    )
    .unwrap()
    .with_indexes(vec![index(index_name, exclusion)])
    .unwrap()
}

fn parent_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts",
        RelationKind::PartitionedTable,
        "accounts_excl",
    )
    .unwrap()
}

fn child_index() -> IndexPartitionCoordinate {
    IndexPartitionCoordinate::new(
        "public",
        "accounts_2026",
        RelationKind::Table,
        "accounts_2026_excl",
    )
    .unwrap()
}

fn predecessor(
    parent_exclusion: Option<bool>,
    child_exclusion: Option<bool>,
) -> (
    PostgresSchemaSnapshotV3,
    RelationPartitionSnapshot,
    IndexPartitionSnapshot,
    IndexOperatorFamilySnapshot,
) {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-index-exclusion-equivalence-v1",
        "2026-09-14T17:08:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                "accounts_excl",
                parent_exclusion,
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                "accounts_2026_excl",
                child_exclusion,
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap();
    let relations = RelationPartitionSnapshot::new(
        &base,
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
    .unwrap();
    let indexes = IndexPartitionSnapshot::new(
        &base,
        &relations,
        vec![
            IndexPartitionObservation::non_partition(
                parent_index(),
                IndexRelationKind::PartitionedIndex,
            )
            .unwrap(),
            IndexPartitionObservation::partition(
                child_index(),
                IndexRelationKind::Index,
                parent_index(),
                false,
            )
            .unwrap(),
        ],
    )
    .unwrap();
    let families = IndexOperatorFamilySnapshot::new(
        &base,
        &relations,
        &indexes,
        vec![family(parent_index()), family(child_index())],
    )
    .unwrap();
    (base, relations, indexes, families)
}

fn family(index: IndexPartitionCoordinate) -> IndexKeyOperatorFamilyObservation {
    IndexKeyOperatorFamilyObservation::new(
        index,
        1,
        QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
        QualifiedOperatorFamilyName::new("btree", "pg_catalog", "integer_ops").unwrap(),
    )
    .unwrap()
}

fn exclusion(
    index: IndexPartitionCoordinate,
    operator_name: &str,
    procedure_name: &str,
    strategy: u16,
) -> IndexKeyExclusionSemanticsObservation {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    IndexKeyExclusionSemanticsObservation::new(
        index,
        1,
        QualifiedOperatorSignature::new("pg_catalog", operator_name, int4.clone(), int4.clone())
            .unwrap(),
        QualifiedProcedureSignature::new("pg_catalog", procedure_name, vec![int4.clone(), int4])
            .unwrap(),
        strategy,
    )
    .unwrap()
}

#[test]
fn exclusion_key_requires_nonzero_coordinates_and_exact_procedure_arguments() {
    let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
    let int8 = QualifiedTypeName::new("pg_catalog", "int8").unwrap();
    let operator =
        QualifiedOperatorSignature::new("pg_catalog", "=", int4.clone(), int8.clone()).unwrap();
    let procedure =
        QualifiedProcedureSignature::new("pg_catalog", "int48eq", vec![int4.clone(), int8.clone()])
            .unwrap();
    let make = |position, procedure, strategy| {
        IndexKeyExclusionSemanticsObservation::new(
            parent_index(),
            position,
            operator.clone(),
            procedure,
            strategy,
        )
    };
    assert_eq!(
        make(0, procedure.clone(), 1),
        Err(ObservationError::InvalidOrdinalPosition)
    );
    assert_eq!(
        make(1, procedure.clone(), 0),
        Err(ObservationError::InvalidObservationField {
            field: "exclusion_strategy"
        })
    );
    for arguments in [vec![], vec![int4.clone(), int8.clone(), int4.clone()]] {
        assert_eq!(
            QualifiedProcedureSignature::new("pg_catalog", "int48eq", arguments),
            Err(ObservationError::InvalidObservationField {
                field: "exclusion_procedure_argument_types",
            })
        );
    }
    for arguments in [
        vec![int4.clone()],
        vec![int8.clone(), int4.clone()],
        vec![int4.clone(), int4.clone()],
    ] {
        let wrong = QualifiedProcedureSignature::new("pg_catalog", "int48eq", arguments).unwrap();
        assert_eq!(
            make(1, wrong, 1),
            Err(ObservationError::InvalidObservationField {
                field: "exclusion_operator_procedure_signature",
            })
        );
    }
    let accepted = make(1, procedure.clone(), 1).unwrap();
    assert_eq!(accepted.operator(), &operator);
    assert_eq!(accepted.procedure(), &procedure);
    assert_eq!(accepted.key_position(), 1);
    assert_eq!(accepted.strategy(), 1);
}

#[test]
fn exclusion_snapshot_rejects_a_predecessor_from_another_capture() {
    let (base, relations, indexes, families) = predecessor(Some(true), Some(true));
    let old_exclusion = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![
            exclusion(parent_index(), "=", "int4eq", 3),
            exclusion(child_index(), "=", "int4eq", 3),
        ],
    )
    .unwrap();
    for (revision, observed_at) in [
        (
            "extractor-index-exclusion-equivalence-v2",
            base.observed_at_utc(),
        ),
        (base.extractor_revision(), "2026-09-15T17:08:00Z"),
    ] {
        let fresh = PostgresSchemaSnapshotV3::new(
            &authorized_source(),
            revision,
            observed_at,
            base.relations().to_vec(),
            vec![],
            vec![],
        )
        .unwrap();
        let fresh_relations =
            RelationPartitionSnapshot::new(&fresh, relations.observations().to_vec()).unwrap();
        assert_eq!(
            IndexPartitionSnapshot::new(&fresh, &relations, indexes.observations().to_vec()),
            Err(ObservationError::InvalidObservationField {
                field: "index_partition_relation_snapshot_binding",
            })
        );
        assert_eq!(
            IndexOperatorFamilySnapshot::new(
                &fresh,
                &fresh_relations,
                &indexes,
                families.observations().to_vec(),
            ),
            Err(ObservationError::InvalidObservationField {
                field: "index_operator_family_predecessor_binding",
            })
        );
        let fresh_indexes =
            IndexPartitionSnapshot::new(&fresh, &fresh_relations, indexes.observations().to_vec())
                .unwrap();
        let facts = vec![
            exclusion(parent_index(), "=", "int4eq", 3),
            exclusion(child_index(), "=", "int4eq", 3),
        ];
        assert_eq!(
            IndexExclusionSemanticsSnapshot::new(
                &fresh,
                &fresh_relations,
                &fresh_indexes,
                &families,
                facts.clone(),
            ),
            Err(ObservationError::InvalidObservationField {
                field: "index_exclusion_semantics_predecessor_binding",
            })
        );
        let fresh_families = IndexOperatorFamilySnapshot::new(
            &fresh,
            &fresh_relations,
            &fresh_indexes,
            families.observations().to_vec(),
        )
        .unwrap();
        let accepted = IndexExclusionSemanticsSnapshot::new(
            &fresh,
            &fresh_relations,
            &fresh_indexes,
            &fresh_families,
            facts,
        )
        .unwrap();
        assert_eq!(
            IndexExpressionSemanticsSnapshot::new(
                &fresh,
                &fresh_relations,
                &fresh_indexes,
                &fresh_families,
                &old_exclusion,
                vec![],
                vec![],
            ),
            Err(ObservationError::InvalidObservationField {
                field: "index_expression_semantics_predecessor_binding",
            })
        );
        let expressions = IndexExpressionSemanticsSnapshot::new(
            &fresh,
            &fresh_relations,
            &fresh_indexes,
            &fresh_families,
            &accepted,
            vec![],
            vec![],
        )
        .unwrap();
        assert_eq!(expressions.extractor_revision(), revision);
        assert_eq!(expressions.observed_at_utc(), observed_at);
        let receipt = accepted.source_receipt(&parent_index(), 1).unwrap();
        assert_eq!(receipt.extractor_revision(), revision);
        assert_eq!(receipt.observed_at_utc(), observed_at);
    }
}

#[test]
fn attached_child_must_preserve_exclusion_presence() {
    let (base, relations, indexes, families) = predecessor(Some(true), Some(false));

    let error =
        IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, vec![])
            .expect_err("PostgreSQL CompareIndexInfo rejects one-sided exclusion semantics");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "index_partition_definition_exclusion_presence",
        }
    );
}

#[test]
fn attached_child_must_preserve_exclusion_operator_procedure_and_strategy() {
    let (base, relations, indexes, families) = predecessor(Some(true), Some(true));

    for (child_operator, child_procedure, child_strategy, expected_field) in [
        (
            "<",
            "int4lt",
            3,
            "index_partition_definition_exclusion_operator",
        ),
        (
            "=",
            "int4lt",
            3,
            "index_partition_definition_exclusion_procedure",
        ),
        (
            "=",
            "int4eq",
            1,
            "index_partition_definition_exclusion_strategy",
        ),
    ] {
        let error = IndexExclusionSemanticsSnapshot::new(
            &base,
            &relations,
            &indexes,
            &families,
            vec![
                exclusion(parent_index(), "=", "int4eq", 3),
                exclusion(
                    child_index(),
                    child_operator,
                    child_procedure,
                    child_strategy,
                ),
            ],
        )
        .expect_err("every exclusion key semantic must match its attached parent");

        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: expected_field,
            }
        );
    }
}

#[test]
fn matching_exclusion_semantics_are_admissible_and_complete() {
    let (base, relations, indexes, families) = predecessor(Some(true), Some(true));

    let snapshot = IndexExclusionSemanticsSnapshot::new(
        &base,
        &relations,
        &indexes,
        &families,
        vec![
            exclusion(parent_index(), "=", "int4eq", 3),
            exclusion(child_index(), "=", "int4eq", 3),
        ],
    )
    .expect("matching PostgreSQL exclusion arrays should remain admissible");
    assert_eq!(snapshot.observations().len(), 2);

    for observation in snapshot.observations() {
        let receipt = snapshot
            .source_receipt(observation.index(), observation.key_position())
            .unwrap();
        assert_eq!(receipt.location(), observation);
        assert_eq!(
            receipt.location().canonical_location(),
            observation.canonical_location()
        );
        assert_eq!(receipt.source_id(), snapshot.source_connection_key());
        assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
        assert_eq!(
            receipt.connection_policy_binding(),
            snapshot.connection_policy_binding()
        );
        assert_eq!(receipt.extractor_revision(), snapshot.extractor_revision());
        assert_eq!(receipt.observed_at_utc(), snapshot.observed_at_utc());
        for key_position in [0, 2] {
            assert_eq!(
                snapshot
                    .source_receipt(observation.index(), key_position)
                    .unwrap_err(),
                ObservationError::UnknownObservationLocation {
                    location: format!(
                        "{}/keys/{key_position}/exclusion",
                        observation.index().canonical_location()
                    ),
                }
            );
        }
        let index = observation.index();
        for (schema, relation, kind, name) in [
            (
                "archive",
                index.relation_name(),
                index.relation_kind(),
                index.index_name(),
            ),
            (
                index.schema_name(),
                "unobserved_relation",
                index.relation_kind(),
                index.index_name(),
            ),
            (
                index.schema_name(),
                index.relation_name(),
                RelationKind::ForeignTable,
                index.index_name(),
            ),
            (
                index.schema_name(),
                index.relation_name(),
                index.relation_kind(),
                "unobserved_index",
            ),
        ] {
            let absent = IndexPartitionCoordinate::new(schema, relation, kind, name).unwrap();
            assert_eq!(
                snapshot.source_receipt(&absent, 1).unwrap_err(),
                ObservationError::UnknownObservationLocation {
                    location: format!("{}/keys/1/exclusion", absent.canonical_location()),
                }
            );
        }
    }

    let complete = snapshot.observations().to_vec();
    for position in 0..complete.len() {
        let mut missing = complete.clone();
        missing.remove(position);
        let mut duplicate = complete.clone();
        duplicate.push(complete[position].clone());
        let mut extra = complete.clone();
        let original = &complete[position];
        extra.push(
            IndexKeyExclusionSemanticsObservation::new(
                original.index().clone(),
                2,
                original.operator().clone(),
                original.procedure().clone(),
                original.strategy(),
            )
            .unwrap(),
        );
        for (observations, field) in [
            (missing, "index_exclusion_semantics_completeness"),
            (duplicate, "index_exclusion_semantics_coordinate"),
            (extra, "index_exclusion_semantics_completeness"),
        ] {
            assert_eq!(
                IndexExclusionSemanticsSnapshot::new(
                    &base,
                    &relations,
                    &indexes,
                    &families,
                    observations,
                )
                .unwrap_err(),
                ObservationError::InvalidObservationField { field }
            );
        }
    }
    let mut reversed = complete;
    reversed.reverse();
    assert_eq!(
        snapshot,
        IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, reversed,)
            .unwrap()
    );
}

#[test]
fn unknown_exclusion_flags_cannot_be_promoted_to_observed_false() {
    for (parent, child) in [(None, Some(false)), (Some(false), None)] {
        let (base, relations, indexes, families) = predecessor(parent, child);
        assert_eq!(
            IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, vec![],)
                .unwrap_err(),
            ObservationError::InvalidObservationField {
                field: "index_exclusion_semantics_catalog_flags",
            }
        );
    }
    let (base, relations, indexes, families) = predecessor(Some(false), Some(false));
    assert!(
        IndexExclusionSemanticsSnapshot::new(&base, &relations, &indexes, &families, vec![],)
            .unwrap()
            .observations()
            .is_empty()
    );
}
