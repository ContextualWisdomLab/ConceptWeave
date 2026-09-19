//! PostgreSQL ordinary exclusion-constraint timing/backing-index coherence evidence.
//!
//! PostgreSQL stores constraint deferrability in `pg_constraint.condeferrable` and independently
//! stores backing-index immediacy in `pg_index.indimmediate`. For an index created to enforce an
//! ordinary `EXCLUDE` constraint, PostgreSQL 18 writes `indimmediate = !condeferrable`. Both source
//! facts remain independently observed; this successor only proves that the exact constraint and its
//! resolved `conindid` backing index agree with that catalog invariant.

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot,
    IndexExclusionConstraintTimingSnapshot, IndexPartitionCoordinate, IndexPartitionSnapshot,
};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.immediacy.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Verified timing/immediacy state for one exact ordinary `EXCLUDE` constraint and backing index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintImmediacyObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
    constraint_deferrable: bool,
    index_immediate: bool,
}

impl IndexExclusionConstraintImmediacyObservation {
    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the exact resolved `pg_constraint.conindid` backing-index coordinate.
    #[must_use]
    pub const fn backing_index(&self) -> &IndexPartitionCoordinate {
        &self.backing_index
    }

    /// Returns independently observed `pg_constraint.condeferrable`.
    #[must_use]
    pub const fn constraint_deferrable(&self) -> bool {
        self.constraint_deferrable
    }

    /// Returns independently observed backing-index `pg_index.indimmediate`.
    #[must_use]
    pub const fn index_immediate(&self) -> bool {
        self.index_immediate
    }

    /// Returns a collision-safe location for the validated cross-catalog invariant.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        immediacy_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one validated EXCLUDE timing/index-immediacy edge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintImmediacySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintImmediacyObservation,
}

impl IndexExclusionConstraintImmediacySourceReceipt {
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

    /// Returns the owner-computed timing/index-immediacy coherence digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the timing predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the timing predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the validated cross-catalog observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintImmediacyObservation {
        &self.location
    }
}

/// Validates `condeferrable` against `indimmediate` for every ordinary EXCLUDE constraint.
///
/// The supplied v3, relation-partition, index-partition, ordinary-EXCLUDE, and timing predecessors
/// are rebound before use. PostgreSQL 18 writes `indimmediate = !condeferrable` when creating the
/// supporting index. A mismatch therefore fails closed rather than normalizing either independently
/// observed catalog field. `condeferred` remains owned by the timing predecessor, so both DEFERRABLE
/// initial-timing modes retain distinct identity while sharing `indimmediate = false`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintImmediacySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintImmediacyObservation>,
}

impl IndexExclusionConstraintImmediacySnapshot {
    /// Rebinds the exact predecessor chain and validates every constraint/backing-index timing edge.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        timing_snapshot: &IndexExclusionConstraintTimingSnapshot,
    ) -> Result<Self, ObservationError> {
        let rebound_constraint = IndexExclusionConstraintSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            constraint_snapshot.observations().to_vec(),
        )?;
        if rebound_constraint.snapshot_digest() != constraint_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_immediacy_predecessor_binding",
            ));
        }

        let rebound_timing = IndexExclusionConstraintTimingSnapshot::new(
            &rebound_constraint,
            timing_snapshot.observations().to_vec(),
        )?;
        if rebound_timing.snapshot_digest() != timing_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_immediacy_predecessor_binding",
            ));
        }

        let mut observations = Vec::with_capacity(rebound_timing.observations().len());
        for timing in rebound_timing.observations() {
            let constraint = rebound_constraint
                .observations()
                .iter()
                .find(|candidate| candidate.coordinate() == timing.coordinate())
                .ok_or_else(|| invalid("index_exclusion_constraint_immediacy_completeness"))?;
            let backing_index = find_backing_index(base_snapshot, constraint.backing_index())?;
            let flags = backing_index.catalog_flags().ok_or_else(|| {
                invalid("index_exclusion_constraint_immediacy_catalog_flags")
            })?;
            let index_immediate = flags.immediate();
            if index_immediate == timing.deferrable() {
                return Err(invalid("index_exclusion_constraint_immediacy_state"));
            }

            observations.push(IndexExclusionConstraintImmediacyObservation {
                coordinate: timing.coordinate().clone(),
                backing_index: constraint.backing_index().clone(),
                constraint_deferrable: timing.deferrable(),
                index_immediate,
            });
        }

        let snapshot_digest =
            compute_immediacy_digest(rebound_timing.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: rebound_timing.source_connection_key().to_owned(),
            connection_policy_binding: rebound_timing.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: rebound_timing.extractor_revision().to_owned(),
            observed_at_utc: rebound_timing.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry reference.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy revision.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated timing/index-immediacy coherence digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the timing predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the timing predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns every validated constraint/backing-index timing edge in deterministic order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintImmediacyObservation] {
        &self.observations
    }

    /// Issues provenance for one validated ordinary EXCLUDE timing/index-immediacy edge.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintImmediacySourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: immediacy_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintImmediacySourceReceipt {
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
        .ok_or_else(|| invalid("index_exclusion_constraint_immediacy_backing_index"))
}

fn compute_immediacy_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintImmediacyObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_IMMEDIACY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
        hasher.update([u8::from(observation.constraint_deferrable())]);
        hasher.update([u8::from(observation.index_immediate())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn immediacy_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/timing-index-coherence", coordinate.canonical_location())
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
