//! PostgreSQL ordinary exclusion-constraint backing-index lifecycle integrity.
//!
//! `pg_constraint.conindid` identifies the index that enforces an ordinary `EXCLUDE` constraint,
//! while `pg_index.indisready`, `indisvalid`, and `indislive` independently describe whether that
//! exact index accepts writes, is valid for query use, and is still live. The generic v3 index
//! observation preserves each bit as optional source evidence. This successor binds those facts to
//! the exact backing index already proven by the namespace generation and fails closed rather than
//! deriving a healthy lifecycle from `contype = 'x'` or from any constraint state.

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintIndexNamespaceSnapshot,
    IndexPartitionCoordinate,
};

const INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.index_lifecycle.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Verified lifecycle state of one exact ordinary-EXCLUDE backing index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexLifecycleObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
    index_ready: bool,
    index_valid: bool,
    index_live: bool,
}

impl IndexExclusionConstraintIndexLifecycleObservation {
    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the exact `conindid` backing-index coordinate proven by the predecessor.
    #[must_use]
    pub const fn backing_index(&self) -> &IndexPartitionCoordinate {
        &self.backing_index
    }

    /// Returns independently observed backing-index `pg_index.indisready`.
    #[must_use]
    pub const fn index_ready(&self) -> bool {
        self.index_ready
    }

    /// Returns independently observed backing-index `pg_index.indisvalid`.
    #[must_use]
    pub const fn index_valid(&self) -> bool {
        self.index_valid
    }

    /// Returns independently observed backing-index `pg_index.indislive`.
    #[must_use]
    pub const fn index_live(&self) -> bool {
        self.index_live
    }

    /// Returns a collision-safe source location for the backing-index lifecycle vector.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        index_lifecycle_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one validated ordinary-EXCLUDE backing-index lifecycle vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexLifecycleSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintIndexLifecycleObservation,
}

impl IndexExclusionConstraintIndexLifecycleSourceReceipt {
    /// Returns the stable source registry key, never credential material.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed backing-index lifecycle digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the source generation.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the source generation.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the validated backing-index lifecycle observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintIndexLifecycleObservation {
        &self.location
    }
}

/// Complete ready/valid/live evidence for every exact ordinary-EXCLUDE backing index.
///
/// PostgreSQL defines `indisready = false` as ignored by `INSERT`/`UPDATE`, `indisvalid = false` as
/// possibly incomplete and unsafe for queries, and `indislive = false` as being dropped and ignored
/// for all purposes. A governed ordinary `EXCLUDE` constraint therefore cannot publish unless its
/// exact backing index independently reports all three states as true. `indcheckxmin` is intentionally
/// outside this invariant because PostgreSQL defines it as a planner visibility horizon rather than
/// index liveness or write-enforcement state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexLifecycleSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintIndexLifecycleObservation>,
}

impl IndexExclusionConstraintIndexLifecycleSnapshot {
    /// Binds the current namespace successor to the exact v3 backing-index lifecycle state.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        namespace_snapshot: &IndexExclusionConstraintIndexNamespaceSnapshot,
    ) -> Result<Self, ObservationError> {
        if base_snapshot.source_connection_key() != namespace_snapshot.source_connection_key()
            || base_snapshot.connection_policy_binding()
                != namespace_snapshot.connection_policy_binding()
            || base_snapshot.snapshot_digest() != namespace_snapshot.source_snapshot_digest()
            || base_snapshot.extractor_revision() != namespace_snapshot.extractor_revision()
            || base_snapshot.observed_at_utc() != namespace_snapshot.observed_at_utc()
        {
            return Err(invalid(
                "index_exclusion_constraint_index_lifecycle_predecessor_binding",
            ));
        }

        let mut observations = Vec::with_capacity(namespace_snapshot.observations().len());
        for namespace in namespace_snapshot.observations() {
            let backing_index = find_backing_index(base_snapshot, namespace.backing_index())?;
            let index_ready = backing_index.ready().ok_or_else(|| {
                invalid("index_exclusion_constraint_index_lifecycle_presence")
            })?;
            let index_valid = backing_index.valid().ok_or_else(|| {
                invalid("index_exclusion_constraint_index_lifecycle_presence")
            })?;
            let index_live = backing_index.live().ok_or_else(|| {
                invalid("index_exclusion_constraint_index_lifecycle_presence")
            })?;

            if !index_ready || !index_valid || !index_live {
                return Err(invalid("index_exclusion_constraint_index_lifecycle_state"));
            }

            observations.push(IndexExclusionConstraintIndexLifecycleObservation {
                coordinate: namespace.coordinate().clone(),
                backing_index: namespace.backing_index().clone(),
                index_ready,
                index_valid,
                index_live,
            });
        }

        let snapshot_digest = compute_index_lifecycle_digest(
            base_snapshot.snapshot_digest(),
            namespace_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: namespace_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: namespace_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: namespace_snapshot.extractor_revision().to_owned(),
            observed_at_utc: namespace_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry reference.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding inherited from the predecessor.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated backing-index lifecycle digest.
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

    /// Returns validated lifecycle observations in deterministic predecessor order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintIndexLifecycleObservation] {
        &self.observations
    }

    /// Issues provenance for one validated ordinary-EXCLUDE backing-index lifecycle vector.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintIndexLifecycleSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: index_lifecycle_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintIndexLifecycleSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn find_backing_index<'a>(
    base_snapshot: &'a PostgresSchemaSnapshotV3,
    coordinate: &IndexPartitionCoordinate,
) -> Result<&'a conceptweave_observation::IndexObservation, ObservationError> {
    base_snapshot
        .relations()
        .iter()
        .find(|relation| {
            relation.schema_name() == coordinate.schema_name()
                && relation.relation_name() == coordinate.relation_name()
                && relation.kind() == coordinate.relation_kind()
        })
        .and_then(|relation| {
            relation
                .indexes()
                .iter()
                .find(|index| index.index_name() == coordinate.index_name())
        })
        .ok_or_else(|| invalid("index_exclusion_constraint_index_lifecycle_backing_index"))
}

fn compute_index_lifecycle_digest(
    base_digest: &str,
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintIndexLifecycleObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, base_digest);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
        hasher.update([u8::from(observation.index_ready())]);
        hasher.update([u8::from(observation.index_valid())]);
        hasher.update([u8::from(observation.index_live())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn index_lifecycle_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/backing-index-lifecycle", coordinate.canonical_location())
}

fn encode_constraint_coordinate(
    hasher: &mut Sha256,
    coordinate: &IndexExclusionConstraintCoordinate,
) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
}

fn encode_index_coordinate(hasher: &mut Sha256, coordinate: &IndexPartitionCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.index_name());
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_len(hasher, value.len());
    hasher.update(value.as_bytes());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
