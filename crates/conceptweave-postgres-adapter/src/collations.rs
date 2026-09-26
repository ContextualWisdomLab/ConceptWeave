use std::collections::BTreeSet;

use conceptweave_observation::{
    CollationDefinitionObservation, CollationLocaleFields, CollationProvider,
    ColumnCollationObservation, DatabaseLocaleDefinition, DomainObservation,
    QualifiedCollationName, RangeCatalogObservation, RelationObservation,
};
use conceptweave_source_port::{
    AuthorizedObservationRequest, ObservationCancellation, SourceObservationFailure,
};
use tokio_postgres::Transaction;

use super::{CaptureMeter, bounded, field};

const DATABASE_LOCALE_QUERY: &str = "WITH raw AS MATERIALIZED ( \
    SELECT encoding, datlocprovider::text AS provider, datcollate, datctype, datlocale, \
    daticurules, datcollversion, pg_catalog.pg_database_collation_actual_version(oid) AS actual \
    FROM pg_catalog.pg_database WHERE datname = current_database() \
), sized AS MATERIALIZED ( \
    SELECT raw.*, COALESCE(octet_length(datcollate)::bigint, 0) \
      + COALESCE(octet_length(datctype)::bigint, 0) \
      + COALESCE(octet_length(datlocale)::bigint, 0) \
      + COALESCE(octet_length(daticurules)::bigint, 0) \
      + COALESCE(octet_length(datcollversion)::bigint, 0) \
      + COALESCE(octet_length(actual)::bigint, 0) AS bytes FROM raw \
) SELECT encoding, provider, \
    CASE WHEN bytes <= $1 THEN datcollate END, \
    CASE WHEN bytes <= $1 THEN datctype END, \
    CASE WHEN bytes <= $1 THEN datlocale END, \
    CASE WHEN bytes <= $1 THEN daticurules END, \
    CASE WHEN bytes <= $1 THEN datcollversion END, \
    CASE WHEN bytes <= $1 THEN actual END, bytes > $1 FROM sized";

const COLLATION_QUERY: &str = "WITH raw AS MATERIALIZED ( \
    SELECT c.collencoding, c.collprovider::text AS provider, c.collisdeterministic, \
    c.collcollate, c.collctype, c.colllocale, c.collicurules, c.collversion, \
    pg_catalog.pg_collation_actual_version(c.oid) AS actual, \
    pg_catalog.obj_description(c.oid, 'pg_collation') AS comment, \
    EXISTS(SELECT 1 FROM pg_catalog.pg_seclabel \
      WHERE classoid = 'pg_collation'::regclass AND objoid = c.oid) AS security_label, \
    EXISTS(SELECT 1 FROM pg_catalog.pg_depend \
      WHERE classid = 'pg_collation'::regclass AND objid = c.oid AND deptype = 'e') AS extension_owned \
    FROM pg_catalog.pg_collation c \
    JOIN pg_catalog.pg_namespace n ON n.oid = c.collnamespace \
    WHERE n.nspname = $1 AND c.collname = $2 AND c.collencoding IN (-1, $3) \
      AND (n.nspname <> 'pg_catalog' OR c.oid < 16384::oid) \
), sized AS MATERIALIZED ( \
    SELECT raw.*, COALESCE(octet_length(collcollate)::bigint, 0) \
      + COALESCE(octet_length(collctype)::bigint, 0) \
      + COALESCE(octet_length(colllocale)::bigint, 0) \
      + COALESCE(octet_length(collicurules)::bigint, 0) \
      + COALESCE(octet_length(collversion)::bigint, 0) \
      + COALESCE(octet_length(actual)::bigint, 0) \
      + COALESCE(octet_length(comment)::bigint, 0) AS bytes FROM raw \
) SELECT collencoding, provider, collisdeterministic, \
    CASE WHEN bytes <= $4 THEN collcollate END, \
    CASE WHEN bytes <= $4 THEN collctype END, \
    CASE WHEN bytes <= $4 THEN colllocale END, \
    CASE WHEN bytes <= $4 THEN collicurules END, \
    CASE WHEN bytes <= $4 THEN collversion END, \
    CASE WHEN bytes <= $4 THEN actual END, \
    CASE WHEN bytes <= $4 THEN comment END, bytes > $4, security_label, extension_owned \
    FROM sized";

fn locale_fields(
    row: &tokio_postgres::Row,
    start: usize,
) -> Result<CollationLocaleFields, SourceObservationFailure> {
    CollationLocaleFields::new(
        field(row, start)?,
        field(row, start + 1)?,
        field(row, start + 2)?,
        field(row, start + 3)?,
        field(row, start + 4)?,
        field(row, start + 5)?,
    )
    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)
}

fn locale_bytes(fields: &CollationLocaleFields) -> usize {
    [
        fields.lc_collate(),
        fields.lc_ctype(),
        fields.locale(),
        fields.icu_rules(),
        fields.recorded_version(),
        fields.actual_version(),
    ]
    .into_iter()
    .flatten()
    .map(str::len)
    .sum()
}

pub(super) fn references(
    domains: &[DomainObservation],
    relations: &[RelationObservation],
    column_collations: &[ColumnCollationObservation],
    range_catalog: &[RangeCatalogObservation],
) -> BTreeSet<(String, String)> {
    let mut references = BTreeSet::new();
    let mut add = |collation: &QualifiedCollationName| {
        references.insert((
            collation.schema_name().to_owned(),
            collation.collation_name().to_owned(),
        ));
    };
    for domain in domains {
        if let Some(collation) = domain.collation() {
            add(collation);
        }
    }
    for relation in relations {
        for index in relation.indexes() {
            if let Some(keys) = index.key_semantics() {
                for key in keys {
                    if let Some(collation) = key.collation() {
                        add(collation);
                    }
                }
            }
        }
    }
    for column in column_collations {
        if let Some(collation) = column.collation() {
            add(collation);
        }
    }
    for range in range_catalog {
        if let Some(collation) = range.collation() {
            add(collation);
        }
    }
    references
}

pub(super) async fn capture(
    transaction: &Transaction<'_>,
    request: &AuthorizedObservationRequest,
    cancellation: &dyn ObservationCancellation,
    meter: &mut CaptureMeter,
    references: BTreeSet<(String, String)>,
) -> Result<Vec<CollationDefinitionObservation>, SourceObservationFailure> {
    if references.is_empty() {
        return Ok(Vec::new());
    }
    if references.iter().any(|(schema, _)| {
        schema != "pg_catalog" && !request.request().allowed_schema_names().contains(schema)
    }) {
        return Err(SourceObservationFailure::InvalidCapturedMetadata);
    }
    let max_bytes = request.request().limits().max_bytes().min(i64::MAX as u64) as i64;
    let default_referenced = references.contains(&("pg_catalog".to_owned(), "default".to_owned()));
    let database_row = if default_referenced {
        bounded(
            request,
            cancellation,
            transaction.query_one(DATABASE_LOCALE_QUERY, &[&max_bytes]),
        )
        .await?
    } else {
        bounded(
            request,
            cancellation,
            transaction.query_one(
                "SELECT encoding FROM pg_catalog.pg_database WHERE datname = current_database()",
                &[],
            ),
        )
        .await?
    };
    let database_encoding: i32 = field(&database_row, 0)?;
    let database_default = if default_referenced {
        if field::<bool>(&database_row, 8)? {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        let provider: String = field(&database_row, 1)?;
        let provider = CollationProvider::try_from(provider.as_str())
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let fields = locale_fields(&database_row, 2)?;
        meter.add(request, 8 + locale_bytes(&fields))?;
        Some(
            DatabaseLocaleDefinition::new(provider, fields)
                .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        )
    } else {
        meter.add(request, 4)?;
        None
    };
    let mut definitions = Vec::with_capacity(references.len());
    for (schema, name) in references {
        let rows = bounded(
            request,
            cancellation,
            transaction.query(
                COLLATION_QUERY,
                &[&schema, &name, &database_encoding, &max_bytes],
            ),
        )
        .await?;
        let [row] = rows.as_slice() else {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        };
        if field::<bool>(row, 10)? {
            return Err(SourceObservationFailure::ByteLimitExceeded {
                max_bytes: request.request().limits().max_bytes(),
            });
        }
        if field::<bool>(row, 11)? || field::<bool>(row, 12)? {
            return Err(SourceObservationFailure::InvalidCapturedMetadata);
        }
        let provider: String = field(row, 1)?;
        let provider = CollationProvider::try_from(provider.as_str())
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?;
        let fields = locale_fields(row, 3)?;
        let comment: Option<String> = field(row, 9)?;
        meter.add(
            request,
            16 + schema.len()
                + name.len()
                + locale_bytes(&fields)
                + comment.as_ref().map_or(0, String::len),
        )?;
        definitions.push(
            CollationDefinitionObservation::new(
                QualifiedCollationName::new(schema, name)
                    .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
                field(row, 0)?,
                database_encoding,
                provider,
                field(row, 2)?,
                fields,
                database_default
                    .clone()
                    .filter(|_| provider == CollationProvider::DatabaseDefault),
                comment,
            )
            .map_err(|_| SourceObservationFailure::InvalidCapturedMetadata)?,
        );
    }
    Ok(definitions)
}
