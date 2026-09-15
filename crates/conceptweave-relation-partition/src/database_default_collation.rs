//! Effective PostgreSQL 18 database-default collation evidence.
//!
//! A `pg_collation` row with `collprovider = 'd'` delegates its behavior to the current database's
//! default locale definition. That definition is not contained in the collation row: PostgreSQL 18
//! stores the database locale provider and locale/version fields in `pg_database` and exposes the
//! current provider version through `pg_database_collation_actual_version(oid)`. This successor binds
//! that database-level state only when a bounded used collation actually delegates to provider `d`.

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexCollationDatabaseEncodingSnapshot, IndexCollationDefinitionSnapshot,
    IndexExpressionCollationIdentitySnapshot, IndexPartitionCollationIdentitySnapshot,
    PostgresCollationProvider,
};

const DATABASE_DEFAULT_COLLATION_ITEM_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.database_default_collation_definition.v1";
const EFFECTIVE_COLLATION_DEFINITION_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.expression_collation_catalog_identity.database_encoding.definition.effective_database_default.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// PostgreSQL 18 `pg_database.datlocprovider` values.
///
/// Unlike `pg_collation.collprovider`, the database catalog does not use `d` recursively: the
/// database default itself resolves to one concrete built-in, libc, or ICU provider.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PostgresDatabaseLocaleProvider {
    /// PostgreSQL built-in locale provider (`b`).
    Builtin,
    /// Operating-system libc locale provider (`c`).
    Libc,
    /// ICU locale provider (`i`).
    Icu,
}

impl PostgresDatabaseLocaleProvider {
    /// Returns the exact PostgreSQL catalog token.
    #[must_use]
    pub const fn token(self) -> u8 {
        match self {
            Self::Builtin => b'b',
            Self::Libc => b'c',
            Self::Icu => b'i',
        }
    }
}

impl TryFrom<char> for PostgresDatabaseLocaleProvider {
    type Error = ObservationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'b' => Ok(Self::Builtin),
            'c' => Ok(Self::Libc),
            'i' => Ok(Self::Icu),
            _ => Err(invalid("database_default_collation_provider")),
        }
    }
}

/// Exact PostgreSQL 18 database-default collation definition for one bounded source observation.
///
/// PostgreSQL 18 requires `datcollate` and `datctype` to be non-NULL for every database. `datlocale`
/// is NULL for libc and present for built-in/ICU; built-in databases use only PostgreSQL 18's
/// `C`, `C.UTF-8`, or `PG_UNICODE_FAST` locale identifiers; `daticurules` is ICU-only. The
/// option-bearing fields retain the raw catalog representation at the API boundary, but the
/// constructor rejects combinations that cannot represent a valid PostgreSQL 18 database locale
/// definition. `recorded_version` comes from `pg_database.datcollversion`; `actual_version` comes
/// from `pg_database_collation_actual_version(database_oid)` in the same bounded capture. Keeping
/// both values makes a provider upgrade visible before `ALTER DATABASE ... REFRESH COLLATION
/// VERSION`.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DatabaseDefaultCollationDefinitionObservation {
    provider: PostgresDatabaseLocaleProvider,
    lc_collate: Option<String>,
    lc_ctype: Option<String>,
    locale: Option<String>,
    icu_rules: Option<String>,
    recorded_version: Option<String>,
    actual_version: Option<String>,
}

impl DatabaseDefaultCollationDefinitionObservation {
    /// Creates exact database-level locale/provider evidence without normalizing provider fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        provider: PostgresDatabaseLocaleProvider,
        lc_collate: Option<String>,
        lc_ctype: Option<String>,
        locale: Option<String>,
        icu_rules: Option<String>,
        recorded_version: Option<String>,
        actual_version: Option<String>,
    ) -> Result<Self, ObservationError> {
        for value in [
            lc_collate.as_deref(),
            lc_ctype.as_deref(),
            locale.as_deref(),
            icu_rules.as_deref(),
            recorded_version.as_deref(),
            actual_version.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if value.contains('\0') {
                return Err(invalid("database_default_collation_text"));
            }
        }
        validate_database_provider_shape(
            provider,
            lc_collate.as_deref(),
            lc_ctype.as_deref(),
            locale.as_deref(),
            icu_rules.as_deref(),
        )?;
        Ok(Self {
            provider,
            lc_collate,
            lc_ctype,
            locale,
            icu_rules,
            recorded_version,
            actual_version,
        })
    }

    /// Returns raw `pg_database.datlocprovider` as a validated PostgreSQL 18 provider.
    #[must_use]
    pub const fn provider(&self) -> PostgresDatabaseLocaleProvider {
        self.provider
    }

    /// Returns raw `pg_database.datcollate`.
    #[must_use]
    pub fn lc_collate(&self) -> Option<&str> {
        self.lc_collate.as_deref()
    }

    /// Returns raw `pg_database.datctype`.
    #[must_use]
    pub fn lc_ctype(&self) -> Option<&str> {
        self.lc_ctype.as_deref()
    }

    /// Returns raw `pg_database.datlocale`.
    #[must_use]
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    /// Returns raw `pg_database.daticurules`.
    #[must_use]
    pub fn icu_rules(&self) -> Option<&str> {
        self.icu_rules.as_deref()
    }

    /// Returns recorded `pg_database.datcollversion`.
    #[must_use]
    pub fn recorded_version(&self) -> Option<&str> {
        self.recorded_version.as_deref()
    }

    /// Returns capture-time `pg_database_collation_actual_version(database_oid)`.
    #[must_use]
    pub fn actual_version(&self) -> Option<&str> {
        self.actual_version.as_deref()
    }

    /// Reports an explicit recorded-versus-current provider-version mismatch when both exist.
    #[must_use]
    pub fn has_version_mismatch(&self) -> bool {
        matches!(
            (self.recorded_version(), self.actual_version()),
            (Some(recorded), Some(actual)) if recorded != actual
        )
    }

    /// Returns a domain-separated digest of this exact database-default definition.
    #[must_use]
    pub fn canonical_digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(DATABASE_DEFAULT_COLLATION_ITEM_DIGEST_DOMAIN_V1);
        encode_database_default(&mut hasher, self);
        format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
    }
}

/// Immutable receipt for the effective database-default collation definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabaseDefaultCollationDefinitionSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
}

impl DatabaseDefaultCollationDefinitionSourceReceipt {
    /// Returns the stable source registry key, never credentials.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the effective-definition successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
}

/// Effective collation-definition proof including database-default delegation when required.
///
/// The predecessor already proves complete material `pg_collation` rows for every distinct used
/// coordinate. This successor rebound-validates that predecessor and then inspects whether any used
/// definition delegates through `collprovider = 'd'`. Exactly one database-default definition is
/// required in that case; supplying one when no used collation delegates to the database default is
/// rejected as unrelated evidence. No predecessor digest is reinterpreted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexEffectiveCollationDefinitionSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    database_default_definition: Option<DatabaseDefaultCollationDefinitionObservation>,
}

impl IndexEffectiveCollationDefinitionSnapshot {
    /// Creates an effective collation proof over exact material-definition predecessors.
    pub fn new(
        material_definition_predecessor: &IndexCollationDefinitionSnapshot,
        database_encoding_predecessor: &IndexCollationDatabaseEncodingSnapshot,
        key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
        expression_collation_predecessor: &IndexExpressionCollationIdentitySnapshot,
        database_default_definition: Option<DatabaseDefaultCollationDefinitionObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound = IndexCollationDefinitionSnapshot::new(
            database_encoding_predecessor,
            key_collation_predecessor,
            expression_collation_predecessor,
            material_definition_predecessor.definitions().to_vec(),
        )?;
        if rebound.snapshot_digest() != material_definition_predecessor.snapshot_digest() {
            return Err(invalid("effective_collation_definition_predecessor"));
        }
        if material_definition_predecessor.source_connection_key()
            != database_encoding_predecessor.source_connection_key()
            || material_definition_predecessor.connection_policy_binding()
                != database_encoding_predecessor.connection_policy_binding()
            || material_definition_predecessor.extractor_revision()
                != database_encoding_predecessor.extractor_revision()
            || material_definition_predecessor.observed_at_utc()
                != database_encoding_predecessor.observed_at_utc()
        {
            return Err(invalid("effective_collation_definition_provenance"));
        }

        let requires_database_default = material_definition_predecessor
            .definitions()
            .iter()
            .any(|definition| definition.provider() == PostgresCollationProvider::DatabaseDefault);
        if requires_database_default != database_default_definition.is_some() {
            return Err(invalid("database_default_collation_definition_presence"));
        }

        let predecessor_digest = material_definition_predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_snapshot_digest(
            &predecessor_digest,
            database_default_definition.as_ref(),
        );
        Ok(Self {
            source_connection_key: material_definition_predecessor
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: material_definition_predecessor
                .connection_policy_binding()
                .to_owned(),
            predecessor_digest,
            snapshot_digest,
            extractor_revision: material_definition_predecessor.extractor_revision().to_owned(),
            observed_at_utc: material_definition_predecessor.observed_at_utc().to_owned(),
            database_default_definition,
        })
    }

    /// Returns the stable source registry key inherited from the predecessor stack.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding inherited from the predecessor stack.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact material-definition predecessor digest.
    #[must_use]
    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    /// Returns the domain-separated effective-definition successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns database-default definition evidence when a used collation delegates to provider `d`.
    #[must_use]
    pub const fn database_default_definition(
        &self,
    ) -> Option<&DatabaseDefaultCollationDefinitionObservation> {
        self.database_default_definition.as_ref()
    }

    /// Reports any stored-versus-current provider-version mismatch in effective collation evidence.
    #[must_use]
    pub fn has_database_default_version_mismatch(&self) -> bool {
        self.database_default_definition
            .as_ref()
            .is_some_and(DatabaseDefaultCollationDefinitionObservation::has_version_mismatch)
    }

    /// Issues provenance for database-default evidence when it is part of this successor.
    pub fn database_default_source_receipt(
        &self,
    ) -> Result<DatabaseDefaultCollationDefinitionSourceReceipt, ObservationError> {
        if self.database_default_definition.is_none() {
            return Err(ObservationError::UnknownObservationLocation {
                location: "/database/default-collation-definition".to_owned(),
            });
        }
        Ok(DatabaseDefaultCollationDefinitionSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
        })
    }
}

fn validate_database_provider_shape(
    provider: PostgresDatabaseLocaleProvider,
    lc_collate: Option<&str>,
    lc_ctype: Option<&str>,
    locale: Option<&str>,
    icu_rules: Option<&str>,
) -> Result<(), ObservationError> {
    let common_fields_present = lc_collate.is_some() && lc_ctype.is_some();
    let provider_fields_valid = match provider {
        PostgresDatabaseLocaleProvider::Libc => locale.is_none() && icu_rules.is_none(),
        PostgresDatabaseLocaleProvider::Builtin => {
            icu_rules.is_none()
                && matches!(locale, Some("C" | "C.UTF-8" | "PG_UNICODE_FAST"))
        }
        PostgresDatabaseLocaleProvider::Icu => locale.is_some(),
    };

    if common_fields_present && provider_fields_valid {
        Ok(())
    } else {
        Err(invalid("database_default_collation_provider_shape"))
    }
}

fn compute_snapshot_digest(
    predecessor_digest: &str,
    database_default_definition: Option<&DatabaseDefaultCollationDefinitionObservation>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(EFFECTIVE_COLLATION_DEFINITION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    match database_default_definition {
        Some(definition) => {
            hasher.update([1]);
            encode_database_default(&mut hasher, definition);
        }
        None => hasher.update([0]),
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_database_default(
    hasher: &mut Sha256,
    definition: &DatabaseDefaultCollationDefinitionObservation,
) {
    hasher.update([definition.provider().token()]);
    encode_optional_str(hasher, definition.lc_collate());
    encode_optional_str(hasher, definition.lc_ctype());
    encode_optional_str(hasher, definition.locale());
    encode_optional_str(hasher, definition.icu_rules());
    encode_optional_str(hasher, definition.recorded_version());
    encode_optional_str(hasher, definition.actual_version());
}

fn encode_optional_str(hasher: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            hasher.update([1]);
            encode_str(hasher, value);
        }
        None => hasher.update([0]),
    }
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let len = u64::try_from(value.len()).expect("semantic string length fits in u64");
    hasher.update(len.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
