//! Database-encoding binding for PostgreSQL 18 catalog-exact index collation evidence.
//!
//! The preceding catalog-identity successors preserve raw `pg_collation.collencoding`, but a raw
//! value is not yet proof that the referenced collation can exist as a usable collation in the
//! observed database. PostgreSQL 18 resolves database objects against backend encodings only, and
//! usable collation rows have `collencoding = -1` (encoding-independent) or the exact
//! `pg_database.encoding` of the source database. This successor binds that source invariant over
//! every modeled key, expression/predicate, and relation-`Var` collation identity without changing
//! any issued predecessor digest.

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    CollationCatalogIdentity, IndexExpressionCollationIdentitySnapshot,
    IndexPartitionCollationIdentitySnapshot,
};

const COLLATION_DATABASE_ENCODING_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.expression_collation_catalog_identity.database_encoding.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

// PostgreSQL 18 `PG_ENCODING_BE_LAST == PG_KOI8U == 34`. IDs above this point are client-only or
// the enum sentinel and cannot be `pg_database.encoding` values.
const POSTGRES18_BACKEND_ENCODING_MAX: i32 = 34;

/// Exact PostgreSQL 18 source-database encoding observed from `pg_database.encoding`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PostgresDatabaseEncodingObservation {
    encoding: i32,
}

impl PostgresDatabaseEncodingObservation {
    /// Creates one PostgreSQL 18 backend/database encoding observation.
    pub fn new(encoding: i32) -> Result<Self, ObservationError> {
        if !(0..=POSTGRES18_BACKEND_ENCODING_MAX).contains(&encoding) {
            return Err(invalid("postgres_database_encoding"));
        }
        Ok(Self { encoding })
    }

    /// Returns the raw PostgreSQL backend encoding ID.
    #[must_use]
    pub const fn encoding(self) -> i32 {
        self.encoding
    }

    /// Validates that a resolved catalog collation is usable in this source database.
    pub fn validate_collation_identity(
        self,
        collation: &CollationCatalogIdentity,
    ) -> Result<(), ObservationError> {
        if collation.encoding() != -1 && collation.encoding() != self.encoding {
            return Err(invalid("index_collation_database_encoding_binding"));
        }
        Ok(())
    }
}

/// Immutable source receipt for the database-encoding-bound collation successor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexCollationDatabaseEncodingSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    database_encoding: PostgresDatabaseEncodingObservation,
}

impl IndexCollationDatabaseEncodingSourceReceipt {
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

    /// Returns the domain-separated successor digest.
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

    /// Returns the exact source-database encoding observation.
    #[must_use]
    pub const fn database_encoding(&self) -> PostgresDatabaseEncodingObservation {
        self.database_encoding
    }
}

/// Complete PostgreSQL 18 database-encoding binding over all modeled index collation identities.
///
/// This successor consumes the exact per-key catalog-identity snapshot and the exact expression /
/// relation-`Var` catalog-identity composition. It proves that each non-null collation identity is
/// either encoding-independent (`collencoding = -1`) or belongs to the exact source database
/// encoding. The predecessor identities and their digests remain immutable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexCollationDatabaseEncodingSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    key_collation_predecessor_digest: String,
    expression_collation_predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    database_encoding: PostgresDatabaseEncodingObservation,
}

impl IndexCollationDatabaseEncodingSnapshot {
    /// Creates complete source-database encoding binding over exact collation predecessors.
    pub fn new(
        database_encoding: PostgresDatabaseEncodingObservation,
        key_collation_predecessor: &IndexPartitionCollationIdentitySnapshot,
        expression_collation_predecessor: &IndexExpressionCollationIdentitySnapshot,
    ) -> Result<Self, ObservationError> {
        if expression_collation_predecessor.key_collation_predecessor_digest()
            != key_collation_predecessor.snapshot_digest()
        {
            return Err(invalid("index_collation_database_encoding_predecessor"));
        }
        if key_collation_predecessor.source_connection_key()
            != expression_collation_predecessor.source_connection_key()
            || key_collation_predecessor.connection_policy_binding()
                != expression_collation_predecessor.connection_policy_binding()
            || key_collation_predecessor.extractor_revision()
                != expression_collation_predecessor.extractor_revision()
            || key_collation_predecessor.observed_at_utc()
                != expression_collation_predecessor.observed_at_utc()
        {
            return Err(invalid("index_collation_database_encoding_provenance"));
        }

        for observation in key_collation_predecessor.observations() {
            if let Some(collation) = observation.collation() {
                database_encoding.validate_collation_identity(collation)?;
            }
        }
        for observation in expression_collation_predecessor.expression_observations() {
            database_encoding.validate_collation_identity(observation.collation())?;
        }
        for observation in expression_collation_predecessor.relation_var_observations() {
            if let Some(collation) = observation.collation() {
                database_encoding.validate_collation_identity(collation)?;
            }
        }

        let key_collation_predecessor_digest = key_collation_predecessor.snapshot_digest().to_owned();
        let expression_collation_predecessor_digest =
            expression_collation_predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest(
            &key_collation_predecessor_digest,
            &expression_collation_predecessor_digest,
            database_encoding,
        );

        Ok(Self {
            source_connection_key: expression_collation_predecessor
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: expression_collation_predecessor
                .connection_policy_binding()
                .to_owned(),
            key_collation_predecessor_digest,
            expression_collation_predecessor_digest,
            snapshot_digest,
            extractor_revision: expression_collation_predecessor.extractor_revision().to_owned(),
            observed_at_utc: expression_collation_predecessor.observed_at_utc().to_owned(),
            database_encoding,
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

    /// Returns the exact per-key collation predecessor digest.
    #[must_use]
    pub fn key_collation_predecessor_digest(&self) -> &str {
        &self.key_collation_predecessor_digest
    }

    /// Returns the exact expression/relation-`Var` collation predecessor digest.
    #[must_use]
    pub fn expression_collation_predecessor_digest(&self) -> &str {
        &self.expression_collation_predecessor_digest
    }

    /// Returns the domain-separated database-encoding successor digest.
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

    /// Returns the exact source-database encoding observation.
    #[must_use]
    pub const fn database_encoding(&self) -> PostgresDatabaseEncodingObservation {
        self.database_encoding
    }

    /// Issues immutable provenance for the database-encoding binding.
    #[must_use]
    pub fn source_receipt(&self) -> IndexCollationDatabaseEncodingSourceReceipt {
        IndexCollationDatabaseEncodingSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            database_encoding: self.database_encoding,
        }
    }
}

fn compute_digest(
    key_predecessor_digest: &str,
    expression_predecessor_digest: &str,
    database_encoding: PostgresDatabaseEncodingObservation,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(COLLATION_DATABASE_ENCODING_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, key_predecessor_digest);
    encode_str(&mut hasher, expression_predecessor_digest);
    hasher.update(database_encoding.encoding().to_be_bytes());
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let length = u64::try_from(value.len()).expect("semantic string length fits in u64");
    hasher.update(length.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
