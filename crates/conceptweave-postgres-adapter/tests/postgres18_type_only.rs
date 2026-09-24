use std::{collections::BTreeMap, str::FromStr};

use conceptweave_observation::SchemaObjectLocation;
use conceptweave_postgres_adapter::PostgresUnixAdapter;
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

fn adapter(config: Config) -> PostgresUnixAdapter {
    PostgresUnixAdapter::new(BTreeMap::from([(
        "fixture_source".to_owned(),
        ("fixture_policy".to_owned(), config),
    )]))
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
async fn postgres18_type_only_catalog_is_observed_in_one_read_only_transaction() {
    let Ok(dsn) = std::env::var("CONCEPTWEAVE_PG18_TEST_DSN") else {
        return;
    };
    let config = Config::from_str(&dsn).unwrap();
    let (client, connection) = config.connect(NoTls).await.unwrap();
    let connection_task = tokio::spawn(connection);
    let schema = format!("cw_type_fixture_{}", std::process::id());
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
            .batch_execute(&format!("CREATE TABLE \"{schema}\".item (id integer)"))
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
        .batch_execute(&format!("DROP SCHEMA \"{schema}\" CASCADE"))
        .await
        .unwrap();
    connection_task.abort();
    result.unwrap();
}
