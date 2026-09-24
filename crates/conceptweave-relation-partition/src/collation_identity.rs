//! Catalog-exact PostgreSQL collation identity for attached-index key equivalence.
//!
//! Historical v3/index-partition evidence resolves `pg_index.indcollation` to schema/name only.
//! PostgreSQL 18 can contain distinct `pg_collation` rows with the same schema/name when
//! `collencoding` differs, while `CompareIndexInfo()` compares the actual collation OIDs. This
//! successor preserves the catalog row's unique `(namespace, name, encoding)` coordinate without
//! changing any issued predecessor digest.

use std::collections::{BTreeMap, BTreeSet};

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use crate::{IndexPartitionCoordinate, IndexPartitionSnapshot, RelationPartitionSnapshot};

const COLLATION_IDENTITY_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.collation_catalog_identity.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Stable identity of one resolved PostgreSQL `pg_collation` catalog row.
///
/// PostgreSQL 18 uniquely identifies a collation row by `collname`, `collencoding`, and
/// `collnamespace`. The database-local OID is therefore used only as a capture-time join coordinate;
/// governed identity keeps exact schema/name plus the raw signed `collencoding` value (`-1` means
/// all encodings).
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CollationCatalogIdentity {
    schema_name: String,
    collation_name: String,
    encoding: i32,
}

impl CollationCatalogIdentity {
    /// Creates one exact resolved `pg_collation` catalog identity.
    pub fn new(
        schema_name: impl Into<String>,
        collation_name: impl Into<String>,
        encoding: i32,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let collation_name = collation_name.into();
        validate_nonblank(&schema_name, "index_collation_catalog_schema_name")?;
        validate_nonblank(&collation_name, "index_collation_catalog_name")?;
        Ok(Self {
            schema_name,
            collation_name,
            encoding,
        })
    }

    /// Returns the exact collation namespace name.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact collation name.
    #[must_use]
    pub fn collation_name(&self) -> &str {
        &self.collation_name
    }

    /// Returns raw signed PostgreSQL `pg_collation.collencoding`.
    #[must_use]
    pub const fn encoding(&self) -> i32 {
        self.encoding
    }
}

/// Catalog-exact collation identity for one index key position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexKeyCollationIdentityObservation {
    index: IndexPartitionCoordinate,
    key_position: u32,
    collation: Option<CollationCatalogIdentity>,
}

impl IndexKeyCollationIdentityObservation {
    /// Creates one key-position identity observation; `None` means PostgreSQL reported no collation.
    pub fn new(
        index: IndexPartitionCoordinate,
        key_position: u32,
        collation: Option<CollationCatalogIdentity>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            index,
            key_position,
            collation,
        })
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        &self.index
    }

    /// Returns the one-based index key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }

    /// Returns the resolved catalog row identity, or `None` for an uncollated key.
    #[must_use]
    pub const fn collation(&self) -> Option<&CollationCatalogIdentity> {
        self.collation.as_ref()
    }

    /// Returns the collision-safe governed evidence coordinate.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "{}/keys/{}/collation-catalog-identity",
            self.index.canonical_location(),
            self.key_position
        )
    }
}

/// Immutable receipt for one exact index-key collation catalog identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexKeyCollationIdentitySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    index: IndexPartitionCoordinate,
    key_position: u32,
}

impl IndexKeyCollationIdentitySourceReceipt {
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

    /// Returns the owner-computed successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact relation-scoped index coordinate.
    #[must_use]
    pub const fn index(&self) -> &IndexPartitionCoordinate {
        &self.index
    }

    /// Returns the one-based key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }
}

/// Complete catalog-exact key-collation evidence over one exact index-partition predecessor.
///
/// Every key-semantic position in the bounded v3 snapshot receives one observation. The successor
/// rebound-validates the supplied index-partition predecessor, requires each observation's
/// schema/name to agree with the issued v3 key collation, and compares full resolved catalog identity
/// across every direct attached parent/child index key. This closes schema/name aliasing without
/// redefining historical v3 or index-partition digests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexPartitionCollationIdentitySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexKeyCollationIdentityObservation>,
}

impl IndexPartitionCollationIdentitySnapshot {
    /// Creates complete catalog-exact collation evidence over an exact predecessor stack.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        observations: Vec<IndexKeyCollationIdentityObservation>,
    ) -> Result<Self, ObservationError> {
        validate_predecessor(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
        )?;
        let observations = canonicalize_and_validate(base_snapshot, observations)?;
        validate_attached_collation_identity(index_partition_snapshot, &observations)?;

        let predecessor_digest = index_partition_snapshot.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest(&predecessor_digest, &observations);
        Ok(Self {
            source_connection_key: index_partition_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: index_partition_snapshot
                .connection_policy_binding()
                .to_owned(),
            predecessor_digest,
            snapshot_digest,
            extractor_revision: index_partition_snapshot.extractor_revision().to_owned(),
            observed_at_utc: index_partition_snapshot.observed_at_utc().to_owned(),
            observations,
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

    /// Returns the exact index-partition predecessor digest.
    #[must_use]
    pub fn predecessor_digest(&self) -> &str {
        &self.predecessor_digest
    }

    /// Returns the domain-separated catalog-identity successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete observations in deterministic index/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexKeyCollationIdentityObservation] {
        &self.observations
    }

    /// Issues provenance for one exact index/key collation identity when present.
    pub fn source_receipt(
        &self,
        index: &IndexPartitionCoordinate,
        key_position: u32,
    ) -> Result<IndexKeyCollationIdentitySourceReceipt, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if !self.observations.iter().any(|observation| {
            observation.index() == index && observation.key_position() == key_position
        }) {
            return Err(ObservationError::UnknownObservationLocation {
                location: format!(
                    "{}/keys/{key_position}/collation-catalog-identity",
                    index.canonical_location()
                ),
            });
        }
        Ok(IndexKeyCollationIdentitySourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            index: index.clone(),
            key_position,
        })
    }
}

fn validate_predecessor(
    base_snapshot: &PostgresSchemaSnapshotV3,
    relation_partition_snapshot: &RelationPartitionSnapshot,
    index_partition_snapshot: &IndexPartitionSnapshot,
) -> Result<(), ObservationError> {
    if base_snapshot.source_connection_key() != index_partition_snapshot.source_connection_key()
        || base_snapshot.connection_policy_binding()
            != index_partition_snapshot.connection_policy_binding()
        || base_snapshot.extractor_revision() != index_partition_snapshot.extractor_revision()
        || base_snapshot.observed_at_utc() != index_partition_snapshot.observed_at_utc()
    {
        return Err(invalid("index_partition_collation_catalog_predecessor"));
    }

    let rebound = IndexPartitionSnapshot::new(
        base_snapshot,
        relation_partition_snapshot,
        index_partition_snapshot.observations().to_vec(),
    )?;
    if rebound.snapshot_digest() != index_partition_snapshot.snapshot_digest() {
        return Err(invalid("index_partition_collation_catalog_predecessor"));
    }
    Ok(())
}

type ObservationKey = (IndexPartitionCoordinate, u32);
type QualifiedName = Option<(String, String)>;

fn canonicalize_and_validate(
    base_snapshot: &PostgresSchemaSnapshotV3,
    mut observations: Vec<IndexKeyCollationIdentityObservation>,
) -> Result<Vec<IndexKeyCollationIdentityObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        (left.index(), left.key_position()).cmp(&(right.index(), right.key_position()))
    });

    let mut expected = BTreeMap::<ObservationKey, QualifiedName>::new();
    for relation in base_snapshot.relations() {
        for index in relation.indexes() {
            let Some(key_semantics) = index.key_semantics() else {
                continue;
            };
            let coordinate = IndexPartitionCoordinate::new(
                relation.schema_name(),
                relation.relation_name(),
                relation.kind(),
                index.index_name(),
            )?;
            for key in key_semantics {
                let qualified_name = key.collation().map(|collation| {
                    (
                        collation.schema_name().to_owned(),
                        collation.collation_name().to_owned(),
                    )
                });
                expected.insert((coordinate.clone(), key.position()), qualified_name);
            }
        }
    }

    let observed_keys = observations
        .iter()
        .map(|observation| (observation.index().clone(), observation.key_position()))
        .collect::<BTreeSet<_>>();
    if observed_keys.len() != observations.len()
        || observed_keys != expected.keys().cloned().collect()
    {
        return Err(invalid("index_partition_collation_catalog_completeness"));
    }

    for observation in &observations {
        let key = (observation.index().clone(), observation.key_position());
        let expected_name = expected
            .get(&key)
            .ok_or_else(|| invalid("index_partition_collation_catalog_coordinate"))?;
        let actual_name = observation.collation().map(|collation| {
            (
                collation.schema_name().to_owned(),
                collation.collation_name().to_owned(),
            )
        });
        if &actual_name != expected_name {
            return Err(invalid("index_partition_collation_catalog_binding"));
        }
    }

    Ok(observations)
}

fn validate_attached_collation_identity(
    index_partition_snapshot: &IndexPartitionSnapshot,
    observations: &[IndexKeyCollationIdentityObservation],
) -> Result<(), ObservationError> {
    let by_key = observations
        .iter()
        .map(|observation| {
            (
                (observation.index().clone(), observation.key_position()),
                observation.collation(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    for membership in index_partition_snapshot
        .observations()
        .iter()
        .filter(|observation| observation.is_partition())
    {
        let Some(parent_index) = membership.parent_index() else {
            continue;
        };
        for child in observations
            .iter()
            .filter(|observation| observation.index() == membership.coordinate())
        {
            let parent = by_key
                .get(&(parent_index.clone(), child.key_position()))
                .ok_or_else(|| invalid("index_partition_collation_catalog_completeness"))?;
            if *parent != child.collation() {
                return Err(invalid("index_partition_collation_catalog_identity"));
            }
        }
    }
    Ok(())
}

fn compute_digest(
    predecessor_digest: &str,
    observations: &[IndexKeyCollationIdentityObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(COLLATION_IDENTITY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_str(&mut hasher, observation.index().schema_name());
        encode_str(&mut hasher, observation.index().relation_name());
        encode_str(&mut hasher, observation.index().relation_kind().token());
        encode_str(&mut hasher, observation.index().index_name());
        hasher.update(observation.key_position().to_be_bytes());
        match observation.collation() {
            Some(collation) => {
                hasher.update([1]);
                encode_str(&mut hasher, collation.schema_name());
                encode_str(&mut hasher, collation.collation_name());
                hasher.update(collation.encoding().to_be_bytes());
            }
            None => hasher.update([0]),
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let length = u64::try_from(value.len()).expect("semantic string length fits in u64");
    hasher.update(length.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize fits in u64");
    hasher.update(value.to_be_bytes());
}
