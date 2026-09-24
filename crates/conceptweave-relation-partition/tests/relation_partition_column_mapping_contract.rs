use conceptweave_observation::{
    ColumnObservationV3, ObservationError, PostgresSchemaSnapshotV3, QualifiedTypeName,
    RelationKind, RelationObservation,
};
use conceptweave_relation_partition::{
    PartitionParentRelationCoordinate, RelationPartitionObservation, RelationPartitionSnapshot,
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

fn snapshot(child_columns: Vec<ColumnObservationV3>) -> Result<RelationPartitionSnapshot, ObservationError> {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-partition-column-map-v1",
        "2026-09-14T18:04:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                vec![
                    column("account_id", 1, "integer", "int4"),
                    column("account_email", 2, "text", "text"),
                ],
            ),
            relation("accounts_2026", RelationKind::Table, child_columns),
        ],
        vec![],
        vec![],
    )
    .unwrap();

    RelationPartitionSnapshot::new(
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
}

#[test]
fn partition_child_must_have_the_same_column_set_as_parent() {
    for child_columns in [
        vec![column("account_id", 1, "integer", "int4")],
        vec![
            column("account_id", 1, "integer", "int4"),
            column("account_email", 2, "text", "text"),
            column("tenant_id", 3, "integer", "int4"),
        ],
    ] {
        let error = snapshot(child_columns)
            .expect_err("PostgreSQL declarative partitions cannot omit or add user columns");
        assert_eq!(
            error,
            ObservationError::InvalidObservationField {
                field: "relation_partition_column_mapping",
            }
        );
    }
}

#[test]
fn partition_child_must_preserve_named_column_type_identity() {
    let error = snapshot(vec![
        column("account_id", 1, "bigint", "int8"),
        column("account_email", 2, "text", "text"),
    ])
    .expect_err("PostgreSQL ATTACH PARTITION rejects same-named columns with different types");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_type",
        }
    );
}

#[test]
fn partition_child_must_preserve_type_modifier_rendering_until_typmod_is_structured() {
    let base = PostgresSchemaSnapshotV3::new(
        &authorized_source(),
        "extractor-relation-partition-column-map-v1",
        "2026-09-14T18:04:00Z",
        vec![
            relation(
                "accounts",
                RelationKind::PartitionedTable,
                vec![column("account_code", 1, "character varying(32)", "varchar")],
            ),
            relation(
                "accounts_2026",
                RelationKind::Table,
                vec![column("account_code", 1, "character varying(64)", "varchar")],
            ),
        ],
        vec![],
        vec![],
    )
    .unwrap();
    let error = RelationPartitionSnapshot::new(
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
    .expect_err("PostgreSQL rowtype mapping also requires equal type modifiers");

    assert_eq!(
        error,
        ObservationError::InvalidObservationField {
            field: "relation_partition_column_type_modifier",
        }
    );
}

#[test]
fn different_physical_column_order_remains_admissible() {
    snapshot(vec![
        column("account_email", 1, "text", "text"),
        column("account_id", 2, "integer", "int4"),
    ])
    .expect("PostgreSQL build_attrmap_by_name allows a valid name/type map across different ordinals");
}
