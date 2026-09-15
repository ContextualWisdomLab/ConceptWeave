//! PostgreSQL 18 collation-definition evidence over catalog-coordinate predecessors.
//!
//! The preceding collation successors deliberately preserve the catalog coordinate required to
//! distinguish resolved `pg_collation` rows inside one captured catalog: namespace, name, and raw
//! `collencoding`. That coordinate is not the complete content of a collation definition. PostgreSQL
//! 18 also records provider, deterministic comparison mode, provider locale fields, ICU rules, and a
//! provider-specific version. The effective provider version can change before the stored catalog
//! version is refreshed. This domain-separated successor preserves both states without rewriting any
//! issued coordinate or database-encoding digest.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    CollationCatalogIdentity, IndexCollationDatabaseEncodingSnapshot,
    IndexExpressionCollationIdentitySnapshot, IndexPartitionCollationIdentitySnapshot,
};

const COLLATION_DEFINITION_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.expression_collation_catalog_identity.database_encoding.definition.v1";
const COLLATION_DEFINITION_ITEM_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.collation_definition.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// PostgreSQL 18 `pg_collation.collprovider` values.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PostgresCollationProvider {
    /// Database-default collation provider (`d`).
    DatabaseDefault,
    /// PostgreSQL built-in collation provider (`b`).
    Builtin,
    /// Operating-system libc provider (`c`).
    Libc,
    /// ICU provider (`i`).
    Icu,
}

impl PostgresCollationProvider {
    /// Returns the exact PostgreSQL catalog token.
    #[must_use]
    pub const fn token(self) -> u8 {
        match self {
            Self::DatabaseDefault => b'd',
            Self::Builtin => b'b',
            Self::Libc => b'c',
            Self::Icu => b'i',
        }
    }
}

impl TryFrom<char> for PostgresCollationProvider {
    type Error = ObservationError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'd' => Ok(Self::DatabaseDefault),
            'b' => Ok(Self::Builtin),
            'c' => Ok(Self::Libc),
            'i' => Ok(Self::Icu),
            _ => Err(invalid("index_collation_definition_provider")),
        }
    }
}

/// Exact material PostgreSQL 18 definition of one resolved collation catalog coordinate.
///
/// Optional strings preserve the catalog distinction between SQL `NULL` and a present string. The
/// recorded `collversion` and capture-time `pg_collation_actual_version(oid)` are separate because a
/// provider upgrade can change effective ordering before the catalog version is refreshed. `oid`
/// remains a capture-time join coordinate and `collowner` remains authorization metadata rather than
/// collation comparison behavior, so neither is part of this semantic-definition successor.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CollationDefinitionObservation {
    identity: CollationCatalogIdentity,
    provider: PostgresCollationProvider,
    deterministic: bool,
    lc_collate: Option<String>,
    lc_ctype: Option<String>,
    locale: Option<String>,
    icu_rules: Option<String>,
    version: Option<String>,
    actual_version: Option<String>,
}

impl CollationDefinitionObservation {
    /// Creates one lossless modeled `pg_collation` definition plus actual provider-version evidence.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        identity: CollationCatalogIdentity,
        provider: PostgresCollationProvider,
        deterministic: bool,
        lc_collate: Option<String>,
        lc_ctype: Option<String>,
        locale: Option<String>,
        icu_rules: Option<String>,
        version: Option<String>,
        actual_version: Option<String>,
    ) -> Result<Self, ObservationError> {
        for value in [
            lc_collate.as_deref(),
            lc_ctype.as_deref(),
            locale.as_deref(),
            icu_rules.as_deref(),
            version.as_deref(),
            actual_version.as_deref(),
        ]
        .into_iter()
        .flatten()
        {
            if value.contains('\0') {
                return Err(invalid("index_collation_definition_text"));
            }
        }
        Ok(Self {
            identity,
            provider,
            deterministic,
            lc_collate,
            lc_ctype,
            locale,
            icu_rules,
            version,
            actual_version,
        })
    }

    /// Returns the exact namespace/name/encoding catalog coordinate.
    #[must_use]
    pub const fn identity(&self) -> &CollationCatalogIdentity {
        &self.identity
    }

    /// Returns raw `pg_collation.collprovider` as a validated PostgreSQL 18 provider.
    #[must_use]
    pub const fn provider(&self) -> PostgresCollationProvider {
        self.provider
    }

    /// Returns raw `pg_collation.collisdeterministic`.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }

    /// Returns raw `pg_collation.collcollate`.
    #[must_use]
    pub fn lc_collate(&self) -> Option<&str> {
        self.lc_collate.as_deref()
    }

    /// Returns raw `pg_collation.collctype`.
    #[must_use]
    pub fn lc_ctype(&self) -> Option<&str> {
        self.lc_ctype.as_deref()
    }

    /// Returns raw `pg_collation.colllocale`.
    #[must_use]
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }

    /// Returns raw `pg_collation.collicurules`.
    #[must_use]
    pub fn icu_rules(&self) -> Option<&str> {
        self.icu_rules.as_deref()
    }

    /// Returns recorded `pg_collation.collversion`.
    #[must_use]
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// Returns capture-time `pg_collation_actual_version(oid)`.
    #[must_use]
    pub fn actual_version(&self) -> Option<&str> {
        self.actual_version.as_deref()
    }

    /// Reports an explicit stored-versus-actual provider-version mismatch when both are available.
    #[must_use]
    pub fn has_version_mismatch(&self) -> bool {
        matches!(
            (self.version(), self.actual_version()),
            (Some(recorded), Some(actual)) if recorded != actual
        )
    }

    /// Returns a domain-separated digest of this exact material definition.
    ///
    /// This item digest is useful for local comparison and evidence attribution. Governed snapshot
    /// identity additionally binds the exact database-encoding predecessor and complete required set.
    #[must_use]
    pub fn canonical_digest(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(COLLATION_DEFINITION_ITEM_DIGEST_DOMAIN_V1);
        encode_definition(&mut hasher, self);
        format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
    }
}

/// Immutable receipt for one exact collation definition in the bounded source observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollationDefinitionSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    identity: CollationCatalogIdentity,
}

impl CollationDefinitionSourceReceipt {
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

    /// Returns the complete definition-successor snapshot digest.
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

    /// Returns the exact collation catalog coordinate covered by this receipt.
    #[must_use]
    pub const fn identity(&self) -> &CollationCatalogIdentity {
        &self.identity
    }
}

/// Complete material collation-definition evidence for all collations used by modeled indexes.
///
/// The constructor rebinds the database-encoding successor to its exact key and expression
/// predecessors, derives the distinct non-null catalog identities actually used by those snapshots,
/// and requires exactly one complete definition observation for every such identity. Missing,
/// duplicate, or unrelated definitions fail closed. The resulting digest changes when material
/// provider definition, stored version, or actual provider version changes while the coordinate does
/// not. A version mismatch is preserved as evidence rather than normalized away.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexCollationDefinitionSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    definitions: Vec<CollationDefinitionObservation>,
}

impl IndexCollationDefinitionSnapshot {
    /// Creates complete material collation-definition evidence over exact predecessor snapshots.
    pub fn new(
        database_encoding_predecessor: &IndexCollationDatabaseEncodingSnapshot,
        key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
        expression_collation_predecessor: &IndexExpressionCollationIdentitySnapshot,
        mut definitions: Vec<CollationDefinitionObservation>,
    ) -> Result<Self, ObservationError> {
        validate_predecessors(
            database_encoding_predecessor,
            key_collation_predecessor,
            expression_collation_predecessor,
        )?;

        definitions.sort_by(|left, right| left.identity().cmp(right.identity()));
        validate_definition_completeness(
            key_collation_predecessor,
            expression_collation_predecessor,
            &definitions,
        )?;

        let predecessor_digest = database_encoding_predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_snapshot_digest(&predecessor_digest, &definitions);
        Ok(Self {
            source_connection_key: database_encoding_predecessor
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: database_encoding_predecessor
                .connection_policy_binding()
                .to_owned(),
            predecessor_digest,
            snapshot_digest,
            extractor_revision: database_encoding_predecessor.extractor_revision().to_owned(),
            observed_at_utc: database_encoding_predecessor.observed_at_utc().to_owned(),
            definitions,
        })
    }

    /// Returns the stable source registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact database-encoding predecessor digest.
    #[must_use]
    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    /// Returns the domain-separated material collation-definition snapshot digest.
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

    /// Returns complete definitions in deterministic catalog-coordinate order.
    #[must_use]
    pub fn definitions(&self) -> &[CollationDefinitionObservation] {
        &self.definitions
    }

    /// Reports whether any bounded collation has a recorded-versus-actual provider-version mismatch.
    #[must_use]
    pub fn has_version_mismatch(&self) -> bool {
        self.definitions
            .iter()
            .any(CollationDefinitionObservation::has_version_mismatch)
    }

    /// Issues provenance for one exact collation definition.
    pub fn source_receipt(
        &self,
        identity: &CollationCatalogIdentity,
    ) -> Result<CollationDefinitionSourceReceipt, ObservationError> {
        if !self
            .definitions
            .iter()
            .any(|definition| definition.identity() == identity)
        {
            return Err(ObservationError::UnknownObservationLocation {
                location: format!(
                    "/collations/{}/{}/encoding/{}/definition",
                    identity.schema_name(),
                    identity.collation_name(),
                    identity.encoding()
                ),
            });
        }
        Ok(CollationDefinitionSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            identity: identity.clone(),
        })
    }
}

fn validate_predecessors(
    database_encoding_predecessor: &IndexCollationDatabaseEncodingSnapshot,
    key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
    expression_collation_predecessor: &IndexExpressionCollationIdentitySnapshot,
) -> Result<(), ObservationError> {
    if database_encoding_predecessor.key_collation_predecessor_digest()
        != key_collation_predecessor.snapshot_digest()
        || database_encoding_predecessor.expression_collation_predecessor_digest()
            != expression_collation_predecessor.snapshot_digest()
    {
        return Err(invalid("index_collation_definition_predecessor"));
    }

    let rebound = IndexCollationDatabaseEncodingSnapshot::new(
        database_encoding_predecessor.database_encoding(),
        key_collation_predecessor,
        expression_collation_predecessor,
    )?;
    if rebound.snapshot_digest() != database_encoding_predecessor.snapshot_digest() {
        return Err(invalid("index_collation_definition_predecessor"));
    }

    if database_encoding_predecessor.source_connection_key()
        != key_collation_predecessor.source_connection_key()
        || database_encoding_predecessor.source_connection_key()
            != expression_collation_predecessor.source_connection_key()
        || database_encoding_predecessor.connection_policy_binding()
            != key_collation_predecessor.connection_policy_binding()
        || database_encoding_predecessor.connection_policy_binding()
            != expression_collation_predecessor.connection_policy_binding()
        || database_encoding_predecessor.extractor_revision()
            != key_collation_predecessor.extractor_revision()
        || database_encoding_predecessor.extractor_revision()
            != expression_collation_predecessor.extractor_revision()
        || database_encoding_predecessor.observed_at_utc()
            != key_collation_predecessor.observed_at_utc()
        || database_encoding_predecessor.observed_at_utc()
            != expression_collation_predecessor.observed_at_utc()
    {
        return Err(invalid("index_collation_definition_provenance"));
    }
    Ok(())
}

fn validate_definition_completeness(
    key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
    expression_collation_predecessor: &IndexExpressionCollationIdentitySnapshot,
    definitions: &[CollationDefinitionObservation],
) -> Result<(), ObservationError> {
    let required = required_identities(key_collation_predecessor, expression_collation_predecessor);
    let observed = definitions
        .iter()
        .map(|definition| definition.identity().clone())
        .collect::<BTreeSet<_>>();

    if observed.len() != definitions.len() || observed != required {
        return Err(invalid("index_collation_definition_completeness"));
    }
    Ok(())
}

fn required_identities(
    key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
    expression_collation_predecessor: &IndexExpressionCollationIdentitySnapshot,
) -> BTreeSet<CollationCatalogIdentity> {
    let mut identities = BTreeSet::new();
    for observation in key_collation_predecessor.observations() {
        if let Some(identity) = observation.collation() {
            identities.insert(identity.clone());
        }
    }
    for observation in expression_collation_predecessor.expression_observations() {
        identities.insert(observation.collation().clone());
    }
    for observation in expression_collation_predecessor.relation_var_observations() {
        if let Some(identity) = observation.collation() {
            identities.insert(identity.clone());
        }
    }
    identities
}

fn compute_snapshot_digest(
    predecessor_digest: &str,
    definitions: &[CollationDefinitionObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(COLLATION_DEFINITION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, definitions.len());
    for definition in definitions {
        encode_definition(&mut hasher, definition);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_definition(hasher: &mut Sha256, definition: &CollationDefinitionObservation) {
    encode_str(hasher, definition.identity().schema_name());
    encode_str(hasher, definition.identity().collation_name());
    hasher.update(definition.identity().encoding().to_be_bytes());
    hasher.update([definition.provider().token()]);
    hasher.update([u8::from(definition.deterministic())]);
    encode_optional_str(hasher, definition.lc_collate());
    encode_optional_str(hasher, definition.lc_ctype());
    encode_optional_str(hasher, definition.locale());
    encode_optional_str(hasher, definition.icu_rules());
    encode_optional_str(hasher, definition.version());
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

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize fits in u64");
    hasher.update(value.to_be_bytes());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
