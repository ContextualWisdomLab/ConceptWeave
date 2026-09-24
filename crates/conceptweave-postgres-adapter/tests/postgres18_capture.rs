use std::{collections::BTreeMap, str::FromStr};

use conceptweave_observation::{
    ConstraintDeferrability, ForeignKeyAction, ForeignKeyDeferrability, ForeignKeyMatchType,
    RelationKind, SchemaObjectLocation, TableConstraintObservation,
};
use conceptweave_postgres_adapter::{PostgresTlsAdapter, PostgresUnixAdapter};
use conceptweave_source_port::{
    ObservationCancellation, ObservationLimits, ObservationRequest, ObservationRequestBudget,
    ObservationResourceEnvelope, ResolvedSourceConnection, SourceConnectionRegistry,
    SourceObservationFailure, SourceObservationPort,
};
use tokio_postgres::{Config, NoTls};

struct Registry;

impl SourceConnectionRegistry for Registry {
    fn contains_source_connection(&self, key: &str) -> bool {
        key == "fixture_source"
    }

    fn connection_policy_binding(&self, key: &str) -> Option<String> {
        (key == "fixture_source").then(|| "fixture_policy".to_owned())
    }

    fn authorizes_schema_scope(&self, connection: &ResolvedSourceConnection, _: &[String]) -> bool {
        connection.connection_policy_binding() == "fixture_policy"
    }

    fn authorizes_resource_envelope(
        &self,
        _: &ResolvedSourceConnection,
        envelope: ObservationResourceEnvelope,
    ) -> bool {
        envelope.limits().max_concurrent_queries() == 1
    }
}

struct NotCancelled;

impl ObservationCancellation for NotCancelled {
    fn is_cancelled(&self) -> bool {
        false
    }
}

struct Cancelled;

impl ObservationCancellation for Cancelled {
    fn is_cancelled(&self) -> bool {
        true
    }
}

fn authorized_with_limits(
    schema: &str,
    max_rows: u64,
    max_bytes: u64,
) -> conceptweave_source_port::AuthorizedObservationRequest {
    ObservationRequest::new(
        "fixture_source",
        vec![schema.to_owned()],
        ObservationRequestBudget::new(1, 128).unwrap(),
        ObservationLimits::with_timeouts(10_000, 3_000, max_rows, max_bytes, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn authorized(schema: &str) -> conceptweave_source_port::AuthorizedObservationRequest {
    authorized_with_limits(schema, 64, 8_192)
}

fn authorized_two(schemas: [&str; 2]) -> conceptweave_source_port::AuthorizedObservationRequest {
    ObservationRequest::new(
        "fixture_source",
        schemas.map(str::to_owned).to_vec(),
        ObservationRequestBudget::new(2, 256).unwrap(),
        ObservationLimits::with_timeouts(10_000, 3_000, 64, 8_192, 1).unwrap(),
    )
    .unwrap()
    .authorize(&Registry)
    .unwrap()
}

fn adapter(config: Config) -> PostgresUnixAdapter {
    PostgresUnixAdapter::new(BTreeMap::from([(
        "fixture_source".to_owned(),
        ("fixture_policy".to_owned(), config),
    )]))
}

#[tokio::test]
async fn postgres18_foreign_key_preserves_comparison_and_referential_evidence() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_fk_fixture_{}", std::process::id());
    let external_schema = format!("cw_fk_external_fixture_{}", std::process::id());
    client.batch_execute(&format!(
        "CREATE SCHEMA \"{schema}\"; \
         CREATE TABLE \"{schema}\".parent (a integer, b integer, CONSTRAINT parent_key UNIQUE (a, b)); \
         CREATE TABLE \"{schema}\".child (x integer, y integer, \
           CONSTRAINT child_parent_fk FOREIGN KEY (x, y) REFERENCES \"{schema}\".parent (b, a) \
           ON UPDATE CASCADE ON DELETE SET NULL (x) DEFERRABLE INITIALLY DEFERRED)"
    )).await.unwrap();

    let result = async {
        let snapshot = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let child = snapshot
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let TableConstraintObservation::ForeignKey(foreign_key) = child
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == "child_parent_fk")
            .unwrap()
        else {
            panic!("captured constraint must be a foreign key");
        };
        assert_eq!(foreign_key.column_names(), &["x", "y"]);
        assert_eq!(foreign_key.referenced_column_names(), &["b", "a"]);
        let behavior = foreign_key.reference_behavior().unwrap();
        assert_eq!(behavior.update_action(), ForeignKeyAction::Cascade);
        assert_eq!(behavior.delete_action(), ForeignKeyAction::SetNull);
        assert_eq!(
            behavior.delete_target_columns(),
            Some(["x".to_owned()].as_slice())
        );
        assert_eq!(
            behavior.deferrability(),
            ForeignKeyDeferrability::InitiallyDeferred
        );
        let catalog = snapshot.foreign_key_catalog().unwrap();
        assert_eq!(catalog.len(), 1);
        assert_eq!(catalog[0].referenced_index_name(), "parent_key");
        assert_eq!(catalog[0].primary_foreign_operators().len(), 2);
        let receipt = snapshot
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "child",
                    RelationKind::Table,
                    "child_parent_fk",
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
        let replay = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_eq!(snapshot.snapshot_digest(), replay.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child DROP CONSTRAINT child_parent_fk; \
             ALTER TABLE \"{schema}\".child ADD CONSTRAINT child_parent_fk \
             FOREIGN KEY (x, y) REFERENCES \"{schema}\".parent (b, a) \
             DEFERRABLE INITIALLY IMMEDIATE"
            ))
            .await
            .unwrap();
        let changed = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(snapshot.snapshot_digest(), changed.snapshot_digest());
        let changed_child = changed
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let TableConstraintObservation::ForeignKey(changed_key) = changed_child
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == "child_parent_fk")
            .unwrap()
        else {
            panic!("captured constraint must be a foreign key");
        };
        assert_eq!(
            changed_key.reference_behavior().unwrap().update_action(),
            ForeignKeyAction::NoAction
        );
        assert_eq!(
            changed_key.reference_behavior().unwrap().deferrability(),
            ForeignKeyDeferrability::InitiallyImmediate
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child DROP CONSTRAINT child_parent_fk; \
                 ALTER TABLE \"{schema}\".child ADD CONSTRAINT child_parent_fk \
                 FOREIGN KEY (x, y) REFERENCES \"{schema}\".parent (b, a) MATCH FULL NOT VALID"
            ))
            .await
            .unwrap();
        let not_valid = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        let not_valid_child = not_valid
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "child")
            .unwrap();
        let TableConstraintObservation::ForeignKey(not_valid_key) = not_valid_child
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == "child_parent_fk")
            .unwrap()
        else {
            panic!("captured constraint must be a foreign key");
        };
        assert_eq!(not_valid_key.validated(), Some(false));
        assert_eq!(
            not_valid_key.reference_behavior().unwrap().match_type(),
            ForeignKeyMatchType::Full
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".child VALIDATE CONSTRAINT child_parent_fk"
            ))
            .await
            .unwrap();
        let valid = adapter(config.clone())
            .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
            .await?;
        assert_ne!(not_valid.snapshot_digest(), valid.snapshot_digest());
        client
            .batch_execute(&format!(
                "CREATE SCHEMA \"{external_schema}\"; \
             CREATE TABLE \"{external_schema}\".other_parent (id integer PRIMARY KEY); \
             ALTER TABLE \"{schema}\".child ADD COLUMN outside_id integer; \
             ALTER TABLE \"{schema}\".child ADD CONSTRAINT outside_fk \
             FOREIGN KEY (outside_id) REFERENCES \"{external_schema}\".other_parent (id)"
            ))
            .await
            .unwrap();
        assert_eq!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 256, 65_536), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::InvalidCapturedMetadata)
        );
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!(
            "DROP SCHEMA IF EXISTS \"{external_schema}\" CASCADE; DROP SCHEMA \"{schema}\" CASCADE"
        ))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn missing_binding_and_tcp_transport_fail_before_source_io() {
    let unbound_adapter = PostgresUnixAdapter::new(BTreeMap::new());
    assert_eq!(
        unbound_adapter
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let tcp = Config::from_str("host=127.0.0.1 dbname=postgres").unwrap();
    assert_eq!(
        adapter(tcp)
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let stale = PostgresUnixAdapter::new(BTreeMap::from([(
        "fixture_source".to_owned(),
        (
            "new_policy".to_owned(),
            Config::from_str("host=/tmp dbname=postgres").unwrap(),
        ),
    )]));
    assert_eq!(
        stale
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let unix = Config::from_str("host=/tmp dbname=postgres").unwrap();
    assert_eq!(
        adapter(unix)
            .observe(authorized("public"), &Cancelled)
            .await
            .err(),
        Some(SourceObservationFailure::Cancelled)
    );
}

#[tokio::test]
async fn postgres18_tcp_requires_valid_ca_and_host_name() {
    let (Ok(dsn), Ok(ca_path), Ok(wrong_ca_path), Ok(plaintext_dsn)) = (
        std::env::var("CONCEPTWEAVE_PG18_TLS_TEST_DSN"),
        std::env::var("CONCEPTWEAVE_PG18_TLS_CA_DER"),
        std::env::var("CONCEPTWEAVE_PG18_TLS_WRONG_CA_DER"),
        std::env::var("CONCEPTWEAVE_PG18_PLAINTEXT_TEST_DSN"),
    ) else {
        return;
    };
    assert!(dsn.contains("host=localhost"));
    let config = Config::from_str(&dsn).unwrap();
    let ca = std::fs::read(ca_path).unwrap();
    let wrong_ca = std::fs::read(wrong_ca_path).unwrap();
    let connection = |config: Config, ca: Vec<u8>| {
        PostgresTlsAdapter::new(BTreeMap::from([(
            "fixture_source".to_owned(),
            ("fixture_policy".to_owned(), config, ca),
        )]))
    };
    assert!(connection(config.clone(), b"invalid certificate".to_vec()).is_err());
    assert!(
        connection(
            Config::from_str("host=/tmp dbname=postgres").unwrap(),
            ca.clone()
        )
        .is_err()
    );

    assert_eq!(
        connection(config.clone(), wrong_ca)
            .unwrap()
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );
    assert_eq!(
        connection(Config::from_str(&plaintext_dsn).unwrap(), ca.clone())
            .unwrap()
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );

    let adapter = connection(config.clone(), ca.clone()).unwrap();
    let snapshot = adapter
        .observe(authorized("public"), &NotCancelled)
        .await
        .unwrap();
    assert!(snapshot.relations().is_empty());

    let wrong_host = Config::from_str(&dsn.replace("host=localhost", "host=127.0.0.1")).unwrap();
    assert_eq!(
        connection(wrong_host, ca)
            .unwrap()
            .observe(authorized("public"), &NotCancelled)
            .await
            .err(),
        Some(SourceObservationFailure::SourceUnavailable)
    );
}

#[tokio::test]
async fn postgres18_column_collation_is_exact_source_evidence() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_collation_fixture_{}", std::process::id());
    client
        .batch_execute(&format!(
            "CREATE SCHEMA \"{schema}\"; \
             CREATE COLLATION \"{schema}\".casefold (provider = icu, locale = 'und-u-ks-level1', deterministic = false); \
             CREATE TABLE \"{schema}\".records (id integer, title text, alias text COLLATE \"{schema}\".casefold)"
        ))
        .await
        .unwrap();

    let result = async {
        let before = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let title = before
            .column_collations()
            .unwrap()
            .iter()
            .find(|column| column.column_name() == "title")
            .unwrap();
        assert_eq!(title.collation().unwrap().schema_name(), "pg_catalog");
        assert_eq!(title.collation().unwrap().collation_name(), "default");
        assert_eq!(title.deterministic(), Some(true));
        let alias = before
            .column_collations()
            .unwrap()
            .iter()
            .find(|column| column.column_name() == "alias")
            .unwrap();
        assert_eq!(alias.collation().unwrap().schema_name(), schema);
        assert_eq!(alias.collation().unwrap().collation_name(), "casefold");
        assert_eq!(alias.deterministic(), Some(false));
        assert!(
            before
                .column_collations()
                .unwrap()
                .iter()
                .find(|column| column.column_name() == "id")
                .unwrap()
                .collation()
                .is_none()
        );

        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".records ALTER COLUMN title TYPE text COLLATE \"C\""
            ))
            .await
            .unwrap();
        let after = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(before.snapshot_digest(), after.snapshot_digest());
        let title = after
            .column_collations()
            .unwrap()
            .iter()
            .find(|column| column.column_name() == "title")
            .unwrap();
        assert_eq!(title.collation().unwrap().collation_name(), "C");
        let receipt = after
            .source_receipt(
                SchemaObjectLocation::column(&schema, "records", RelationKind::Table, "title")
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(receipt.source_digest(), after.snapshot_digest());
        Ok::<_, SourceObservationFailure>(())
    }
    .await;
    client
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}

#[tokio::test]
async fn postgres18_catalog_is_observed_in_one_read_only_transaction() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_type_fixture_{}", std::process::id());
    let second_schema = format!("cw_type_fixture_{}_b", std::process::id());
    let fixture = format!(
        "CREATE SCHEMA \"{schema}\"; \
         CREATE TYPE \"{schema}\".stage AS ENUM ('new', 'done'); \
         CREATE DOMAIN \"{schema}\".risk_score AS integer DEFAULT 5 CHECK (VALUE >= 0); \
         COMMENT ON DOMAIN \"{schema}\".risk_score IS 'risk score'"
    );
    client.batch_execute(&fixture).await.unwrap();

    let result = async {
        let snapshot = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(snapshot.relations().is_empty());
        assert_eq!(snapshot.domains().len(), 1);
        assert_eq!(snapshot.enums().len(), 1);
        assert_eq!(snapshot.domains()[0].source_comment(), Some("risk score"));
        assert_eq!(snapshot.domains()[0].check_constraints().len(), 1);
        assert_eq!(snapshot.enums()[0].labels(), &["new", "done"]);
        let receipt = snapshot
            .source_receipt(SchemaObjectLocation::domain(&schema, "risk_score").unwrap())
            .unwrap();
        assert_eq!(receipt.source_digest(), snapshot.snapshot_digest());
        snapshot
            .source_receipt(SchemaObjectLocation::enum_(&schema, "stage").unwrap())
            .unwrap();

        client
            .batch_execute(&format!(
                "ALTER TYPE \"{schema}\".stage ADD VALUE 'review'; \
                 ALTER DOMAIN \"{schema}\".risk_score SET DEFAULT 7"
            ))
            .await
            .unwrap();
        let changed = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(changed.snapshot_digest(), snapshot.snapshot_digest());
        assert_eq!(changed.enums()[0].labels(), &["new", "done", "review"]);
        assert_ne!(
            changed.domains()[0].default_expression(),
            snapshot.domains()[0].default_expression()
        );

        assert_eq!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 1, 8_192), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::RowLimitExceeded { max_rows: 1 })
        );
        assert_eq!(
            adapter(config.clone())
                .observe(authorized_with_limits(&schema, 64, 40), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::ByteLimitExceeded { max_bytes: 40 })
        );

        client
            .batch_execute(&format!(
                "CREATE TABLE \"{schema}\".item (id integer, other integer); \
                 COMMENT ON TABLE \"{schema}\".item IS 'items'"
            ))
            .await
            .unwrap();
        let with_table = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_eq!(with_table.relations().len(), 1);
        assert_eq!(with_table.relations()[0].source_comment(), Some("items"));
        assert_eq!(with_table.relations()[0].columns().len(), 2);
        assert_eq!(
            with_table.relations()[0].columns()[0]
                .type_binding()
                .schema_name(),
            "pg_catalog"
        );
        with_table
            .source_receipt(
                SchemaObjectLocation::relation(&schema, "item", RelationKind::Table).unwrap(),
            )
            .unwrap();
        with_table
            .source_receipt(
                SchemaObjectLocation::column(&schema, "item", RelationKind::Table, "id").unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "COMMENT ON COLUMN \"{schema}\".item.id IS 'identifier'"
            ))
            .await
            .unwrap();
        let with_column_comment = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(
            with_column_comment.snapshot_digest(),
            with_table.snapshot_digest()
        );

        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_nonnegative CHECK (id >= 0) NOT VALID"
            ))
            .await
            .unwrap();
        let unvalidated = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = unvalidated
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert!(matches!(
            &item.constraints()[0],
            TableConstraintObservation::Check(check)
                if check.constraint_name() == "item_nonnegative"
                    && check.definition().contains("id >= 0")
                    && !check.validated()
                    && check.enforced()
                    && !check.no_inherit()
        ));
        unvalidated
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_nonnegative",
                )
                .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item VALIDATE CONSTRAINT item_nonnegative"
            ))
            .await
            .unwrap();
        let validated = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(unvalidated.snapshot_digest(), validated.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_unique UNIQUE (id)"
            ))
            .await
            .unwrap();
        let with_unique_constraint = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await
            .expect("ordinary UNIQUE constraint capture");
        let item = with_unique_constraint
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert!(item.constraints().iter().any(|constraint| matches!(
            constraint,
            TableConstraintObservation::Unique(unique)
                if unique.constraint_name() == "item_unique"
                    && unique.column_names() == ["id"]
                    && unique.nulls_not_distinct() == Some(false)
        )));
        with_unique_constraint
            .source_receipt(
                SchemaObjectLocation::constraint(&schema, "item", RelationKind::Table, "item_unique")
                    .unwrap(),
            )
            .unwrap();
        assert_ne!(validated.snapshot_digest(), with_unique_constraint.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_deferred UNIQUE (other) DEFERRABLE"
            ))
            .await
            .unwrap();
        let initially_immediate = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(initially_immediate.constraint_timings().unwrap().iter().any(|timing| {
            timing.constraint_name() == "item_deferred"
                && timing.deferrability() == ConstraintDeferrability::InitiallyImmediate
        }));
        initially_immediate
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_deferred",
                )
                .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_deferred; \
                 ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_deferred UNIQUE (other) DEFERRABLE INITIALLY DEFERRED"
            ))
            .await
            .unwrap();
        let initially_deferred = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(initially_deferred.constraint_timings().unwrap().iter().any(|timing| {
            timing.constraint_name() == "item_deferred"
                && timing.deferrability() == ConstraintDeferrability::InitiallyDeferred
        }));
        assert_ne!(
            initially_immediate.snapshot_digest(),
            initially_deferred.snapshot_digest()
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_deferred; \
                 ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_unique"
            ))
            .await
            .unwrap();
        let after_drop = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_eq!(
            after_drop.relations()[0].constraints().len(),
            1,
            "dropped internal triggers must not block the remaining CHECK"
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item ADD CONSTRAINT item_other_not_null \
                 NOT NULL other NOT VALID"
            ))
            .await
            .unwrap();
        let with_not_null = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(with_not_null
            .not_null_constraints()
            .unwrap()
            .iter()
            .any(|constraint| {
                constraint.relation_name() == "item"
                    && constraint.constraint_name() == "item_other_not_null"
                    && constraint.column_name() == "other"
                    && !constraint.validated()
            }));
        with_not_null
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_other_not_null",
                )
                .unwrap(),
            )
            .unwrap();
        assert_ne!(after_drop.snapshot_digest(), with_not_null.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item VALIDATE CONSTRAINT item_other_not_null"
            ))
            .await
            .unwrap();
        let validated_not_null = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(validated_not_null.not_null_constraints().unwrap()[0].validated());
        assert_ne!(with_not_null.snapshot_digest(), validated_not_null.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".item DROP CONSTRAINT item_other_not_null"
            ))
            .await
            .unwrap();
        let restored = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_eq!(after_drop.snapshot_digest(), restored.snapshot_digest());
        assert!(restored
            .source_receipt(
                SchemaObjectLocation::constraint(
                    &schema,
                    "item",
                    RelationKind::Table,
                    "item_other_not_null",
                )
                .unwrap(),
            )
            .is_err());
        client
            .batch_execute(&format!(
                "CREATE TABLE \"{schema}\".keyed (id integer PRIMARY KEY DEFERRABLE INITIALLY DEFERRED)"
            ))
            .await
            .unwrap();
        let with_primary_key = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let keyed = with_primary_key
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "keyed")
            .unwrap();
        assert!(keyed.constraints().iter().any(|constraint| matches!(
            constraint,
            TableConstraintObservation::PrimaryKey(primary)
                if primary.constraint_name() == "keyed_pkey"
                    && primary.column_names() == ["id"]
        )));
        assert!(with_primary_key.constraint_timings().unwrap().iter().any(|timing| {
            timing.constraint_name() == "keyed_pkey"
                && timing.deferrability() == ConstraintDeferrability::InitiallyDeferred
        }));
        assert!(with_primary_key
            .not_null_constraints()
            .unwrap()
            .iter()
            .any(|constraint| {
                constraint.relation_name() == "keyed"
                    && constraint.constraint_name() == "keyed_id_not_null"
                    && constraint.column_name() == "id"
                    && constraint.validated()
            }));
        for name in ["keyed_pkey", "keyed_id_not_null"] {
            let receipt = with_primary_key
                .source_receipt(
                    SchemaObjectLocation::constraint(&schema, "keyed", RelationKind::Table, name)
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(receipt.source_digest(), with_primary_key.snapshot_digest());
        }
        assert_ne!(after_drop.snapshot_digest(), with_primary_key.snapshot_digest());
        client
            .batch_execute(&format!("DROP TABLE \"{schema}\".keyed"))
            .await
            .unwrap();
        client
            .batch_execute(&format!(
                "CREATE FUNCTION \"{schema}\".keep_item() RETURNS trigger LANGUAGE plpgsql \
                 AS $$ BEGIN RETURN NEW; END $$; \
                 CREATE TRIGGER item_live_trigger BEFORE INSERT ON \"{schema}\".item \
                 FOR EACH ROW EXECUTE FUNCTION \"{schema}\".keep_item()"
            ))
            .await
            .unwrap();
        assert_eq!(
            adapter(config.clone())
                .observe(authorized(&schema), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::InvalidCapturedMetadata)
        );
        client
            .batch_execute(&format!(
                "DROP TRIGGER item_live_trigger ON \"{schema}\".item"
            ))
            .await
            .unwrap();
        adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;

        client
            .batch_execute(&format!("CREATE TABLE \"{schema}\".\" \" (\" \" integer)"))
            .await
            .unwrap();
        let quoted = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert!(quoted.relations().iter().any(|relation| {
            relation.relation_name() == " " && relation.columns()[0].column_name() == " "
        }));
        quoted
            .source_receipt(
                SchemaObjectLocation::column(&schema, " ", RelationKind::Table, " ").unwrap(),
            )
            .unwrap();

        client
            .batch_execute(&format!(
                "CREATE SCHEMA \"{second_schema}\"; \
                 CREATE TYPE \"{second_schema}\".stage AS ENUM ('other')"
            ))
            .await
            .unwrap();
        let forward = adapter(config.clone())
            .observe(authorized_two([&schema, &second_schema]), &NotCancelled)
            .await?;
        let reverse = adapter(config.clone())
            .observe(authorized_two([&second_schema, &schema]), &NotCancelled)
            .await?;
        assert_eq!(forward.snapshot_digest(), reverse.snapshot_digest());
        assert_eq!(forward.enums().len(), 2);
        assert_ne!(forward.enums()[0].labels(), forward.enums()[1].labels());
        forward
            .source_receipt(SchemaObjectLocation::enum_(&second_schema, "stage").unwrap())
            .unwrap();

        client
            .batch_execute(&format!(
                "CREATE TABLE \"{schema}\".defaulted (id integer DEFAULT 1, \
                 doubled integer GENERATED ALWAYS AS (id * 2) STORED, \
                 tripled integer GENERATED ALWAYS AS (id * 3) VIRTUAL)"
            ))
            .await
            .unwrap();
        let with_expressions = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let expressions = with_expressions.column_expressions().unwrap();
        let generations = with_expressions.column_generations().unwrap();
        assert!(
            expressions
                .iter()
                .any(|value| { value.column_name() == "id" && value.is_default_expression() })
        );
        assert!(
            expressions.iter().any(|value| {
                value.column_name() == "doubled" && value.is_generation_expression()
            })
        );
        assert!(
            generations
                .iter()
                .any(|value| { value.column_name() == "tripled" && value.is_virtual_generated() })
        );
        client
            .batch_execute(&format!(
                "ALTER TABLE \"{schema}\".defaulted ALTER COLUMN id SET DEFAULT 2"
            ))
            .await
            .unwrap();
        let changed_default = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(
            with_expressions.snapshot_digest(),
            changed_default.snapshot_digest()
        );
        client
            .batch_execute(&format!("DROP TABLE \"{schema}\".defaulted"))
            .await
            .unwrap();

        client
            .batch_execute(&format!(
                "CREATE INDEX item_id_idx ON \"{schema}\".item (id)"
            ))
            .await
            .unwrap();
        let with_index = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_index
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert_eq!(item.indexes().len(), 1);
        assert_eq!(
            item.indexes()[0].key_attributes()[0].attribute_name(),
            Some("id")
        );
        assert_eq!(item.indexes()[0].key_semantics().unwrap().len(), 1);
        with_index
            .source_receipt(
                SchemaObjectLocation::index(&schema, "item", RelationKind::Table, "item_id_idx")
                    .unwrap(),
            )
            .unwrap();
        client
            .batch_execute(&format!(
                "COMMENT ON INDEX \"{schema}\".item_id_idx IS 'lookup'"
            ))
            .await
            .unwrap();
        let changed_index = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        assert_ne!(
            with_index.snapshot_digest(),
            changed_index.snapshot_digest()
        );
        client
            .batch_execute(&format!(
                "CREATE UNIQUE INDEX item_id_unique_idx ON \"{schema}\".item (id) NULLS NOT DISTINCT"
            ))
            .await
            .unwrap();
        let with_unique = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_unique
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert_eq!(item.indexes().len(), 2);
        assert!(item.indexes().iter().any(|index| {
            index.index_name() == "item_id_unique_idx"
                && index.is_unique()
                && index.nulls_not_distinct() == Some(true)
        }));
        assert_ne!(changed_index.snapshot_digest(), with_unique.snapshot_digest());
        client
            .batch_execute(&format!(
                "CREATE INDEX item_expr_idx ON \"{schema}\".item (id, (other + 1)) INCLUDE (other) WHERE id > 0"
            ))
            .await
            .unwrap();
        let with_expression_index = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_expression_index
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        let expression_index = item
            .indexes()
            .iter()
            .find(|index| index.index_name() == "item_expr_idx")
            .unwrap();
        assert_eq!(expression_index.key_attributes()[0].attribute_name(), Some("id"));
        assert!(expression_index.key_attributes()[1]
            .expression_text()
            .unwrap()
            .contains("other"));
        assert_eq!(
            expression_index.include_attributes()[0].attribute_name(),
            Some("other")
        );
        assert!(expression_index.predicate().unwrap().contains("id"));
        with_expression_index
            .source_receipt(
                SchemaObjectLocation::index(&schema, "item", RelationKind::Table, "item_expr_idx")
                    .unwrap(),
            )
            .unwrap();
        assert_ne!(with_unique.snapshot_digest(), with_expression_index.snapshot_digest());
        client
            .batch_execute(&format!(
                "CREATE INDEX item_hash_idx ON \"{schema}\".item USING hash (id)"
            ))
            .await
            .unwrap();
        let with_hash = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_hash
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        assert!(item.indexes().iter().any(|index| {
            index.index_name() == "item_hash_idx" && index.access_method() == Some("hash")
        }));
        client
            .batch_execute(&format!(
                "ALTER INDEX \"{schema}\".item_id_idx SET (fillfactor = 80)"
            ))
            .await
            .unwrap();
        let with_storage_option = adapter(config.clone())
            .observe(authorized(&schema), &NotCancelled)
            .await?;
        let item = with_storage_option
            .relations()
            .iter()
            .find(|relation| relation.relation_name() == "item")
            .unwrap();
        let index = item
            .indexes()
            .iter()
            .find(|index| index.index_name() == "item_id_idx")
            .unwrap();
        assert_eq!(index.storage_options().unwrap()[0].name(), "fillfactor");
        assert_eq!(index.storage_options().unwrap()[0].value(), "80");
        assert_ne!(with_hash.snapshot_digest(), with_storage_option.snapshot_digest());
        client
            .batch_execute(&format!(
                "ALTER INDEX \"{schema}\".item_expr_idx ALTER COLUMN 2 SET STATISTICS 1000"
            ))
            .await
            .unwrap();
        assert_eq!(
            adapter(config.clone())
                .observe(authorized(&schema), &NotCancelled)
                .await
                .err(),
            Some(SourceObservationFailure::InvalidCapturedMetadata)
        );
        Ok::<_, SourceObservationFailure>(())
    }
    .await;

    client
        .batch_execute(&format!(
            "DROP SCHEMA IF EXISTS \"{second_schema}\" CASCADE; \
             DROP SCHEMA \"{schema}\" CASCADE"
        ))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}
