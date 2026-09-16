//! PostgreSQL ordinary exclusion-constraint backing-index role evidence.
//!
//! PostgreSQL transforms an ordinary `EXCLUDE` constraint into a non-unique, non-primary index and
//! marks that index as exclusion-backed in `pg_index`. The exact `conindid` edge and the index role
//! flags are independently observed catalog facts. This successor rebinds the ordinary-EXCLUDE and
//! timing/immediacy predecessors, then rejects contradictory role vectors without synthesizing any
//! flag from the constraint kind or rewriting an issued predecessor digest.

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintImmediacySnapshot,
    IndexExclusionConstraintSnapshot, IndexExclusionConstraintTimingSnapshot,
    IndexPartitionCoordinate, IndexPartitionSnapshot,
};
use crate::RelationPartitionSnapshot;

const INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.index_role.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Verified `pg_index` role state for one ordinary `EXCLUDE` constraint's exact backing index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexRoleObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
    index_unique: bool,
    index_primary: bool,
    index_exclusion: bool,
}

impl IndexExclusionConstraintIndexRoleObservation {
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

    /// Returns independently observed backing-index `pg_index.indisunique`.
    #[must_use]
    pub const fn index_unique(&self) -> bool {
        self.index_unique
    }

    /// Returns independently observed backing-index `pg_index.indisprimary`.
    #[must_use]
    pub const fn index_primary(&self) -> bool {
        self.index_primary
    }

    /// Returns independently observed backing-index `pg_index.indisexclusion`.
    #[must_use]
    pub const fn index_exclusion(&self) -> bool {
        self.index_exclusion
    }

    /// Returns a collision-safe source location for the validated backing-index role vector.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        index_role_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one validated ordinary EXCLUDE backing-index role vector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexRoleSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintIndexRoleObservation,
}

impl IndexExclusionConstraintIndexRoleSourceReceipt {
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

    /// Returns the owner-computed ordinary EXCLUDE backing-index role digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the immediacy predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the immediacy predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the validated backing-index role observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintIndexRoleObservation {
        &self.location
    }
}

/// Validates the exact backing-index role vector for every ordinary `EXCLUDE` constraint.
///
/// PostgreSQL's index-constraint transform sets an ordinary `EXCLUDE` index to non-unique and
/// non-primary while retaining exclusion behavior separately. Temporal PRIMARY KEY/UNIQUE
/// `WITHOUT OVERLAPS` indexes are not ordinary `contype='x'` constraints and remain in the
/// key-constraint owner family. This successor preserves the raw index flags and fails closed unless
/// each exact `conindid` backing index is `indisunique=false`, `indisprimary=false`, and
/// `indisexclusion=true`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexRoleSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintIndexRoleObservation>,
}

impl IndexExclusionConstraintIndexRoleSnapshot {
    /// Rebinds the exact predecessor chain and validates ordinary EXCLUDE backing-index roles.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        timing_snapshot: &IndexExclusionConstraintTimingSnapshot,
        immediacy_snapshot: &IndexExclusionConstraintImmediacySnapshot,
    ) -> Result<Self, ObservationError> {
        let rebound_constraint = IndexExclusionConstraintSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            constraint_snapshot.observations().to_vec(),
        )?;
        if rebound_constraint.snapshot_digest() != constraint_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_index_role_predecessor_binding",
            ));
        }

        let rebound_timing = IndexExclusionConstraintTimingSnapshot::new(
            &rebound_constraint,
            timing_snapshot.observations().to_vec(),
        )?;
        if rebound_timing.snapshot_digest() != timing_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_index_role_predecessor_binding",
            ));
        }

        let rebound_immediacy = IndexExclusionConstraintImmediacySnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            &rebound_constraint,
            &rebound_timing,
        )?;
        if rebound_immediacy.snapshot_digest() != immediacy_snapshot.snapshot_digest() {
            return Err(invalid(
                "index_exclusion_constraint_index_role_predecessor_binding",
            ));
        }

        let mut observations = Vec::with_capacity(rebound_constraint.observations().len());
        for constraint in rebound_constraint.observations() {
            let backing_index = find_backing_index(base_snapshot, constraint.backing_index())?;
            let flags = backing_index.catalog_flags().ok_or_else(|| {
                invalid("index_exclusion_constraint_index_role_catalog_flags")
            })?;
            let index_unique = backing_index.is_unique();
            let index_primary = flags.primary();
            let index_exclusion = flags.exclusion();
            if index_unique || index_primary || !index_exclusion {
                return Err(invalid("index_exclusion_constraint_index_role_state"));
            }

            observations.push(IndexExclusionConstraintIndexRoleObservation {
                coordinate: constraint.coordinate().clone(),
                backing_index: constraint.backing_index().clone(),
                index_unique,
                index_primary,
                index_exclusion,
            });
        }

        let snapshot_digest =
            compute_index_role_digest(rebound_immediacy.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: rebound_immediacy.source_connection_key().to_owned(),
            connection_policy_binding: rebound_immediacy.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: rebound_immediacy.extractor_revision().to_owned(),
            observed_at_utc: rebound_immediacy.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated ordinary EXCLUDE backing-index role digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the immediacy predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the immediacy predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns validated backing-index role observations in deterministic constraint order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintIndexRoleObservation] {
        &self.observations
    }

    /// Issues provenance for one validated ordinary EXCLUDE backing-index role vector.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintIndexRoleSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: index_role_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintIndexRoleSourceReceipt {
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
        .ok_or_else(|| invalid("index_exclusion_constraint_index_role_backing_index"))
}

fn compute_index_role_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintIndexRoleObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_INDEX_ROLE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
        hasher.update([u8::from(observation.index_unique())]);
        hasher.update([u8::from(observation.index_primary())]);
        hasher.update([u8::from(observation.index_exclusion())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn index_role_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/backing-index-role", coordinate.canonical_location())
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
