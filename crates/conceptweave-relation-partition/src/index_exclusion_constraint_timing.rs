//! PostgreSQL exclusion-constraint deferrability and initial timing evidence.
//!
//! `pg_constraint.condeferrable` and `condeferred` are independent source facts for `EXCLUDE`
//! constraints. The predecessor exclusion-constraint snapshot preserves catalog identity, backing
//! index, partition parentage, and inheritance state, but intentionally keeps its issued digest
//! stable. This successor adds the two timing bits in a separate digest domain so transactional
//! enforcement semantics cannot collapse into the same governed identity.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot};

const INDEX_EXCLUSION_CONSTRAINT_TIMING_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.timing.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_constraint.condeferrable` and `condeferred` values for one `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintTimingObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    deferrable: bool,
    initially_deferred: bool,
}

impl IndexExclusionConstraintTimingObservation {
    /// Records exact PostgreSQL exclusion-constraint timing state.
    ///
    /// PostgreSQL only gives `condeferred` meaning to a deferrable constraint. A catalog tuple with
    /// `condeferrable = false` and `condeferred = true` is therefore contradictory source evidence
    /// and fails closed rather than being normalized.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        deferrable: bool,
        initially_deferred: bool,
    ) -> Result<Self, ObservationError> {
        if !deferrable && initially_deferred {
            return Err(invalid("index_exclusion_constraint_timing_state"));
        }
        Ok(Self {
            coordinate,
            deferrable,
            initially_deferred,
        })
    }

    /// Returns the exact exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns observed `pg_constraint.condeferrable`.
    #[must_use]
    pub const fn deferrable(&self) -> bool {
        self.deferrable
    }

    /// Returns observed `pg_constraint.condeferred`.
    #[must_use]
    pub const fn initially_deferred(&self) -> bool {
        self.initially_deferred
    }

    /// Returns a collision-safe evidence location distinct from the predecessor constraint path.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        timing_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact exclusion-constraint timing observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintTimingSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintTimingObservation,
}

impl IndexExclusionConstraintTimingSourceReceipt {
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

    /// Returns the owner-computed exclusion-constraint timing successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact exclusion-constraint timing observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintTimingObservation {
        &self.location
    }
}

/// Complete exclusion-constraint timing evidence over one exact exclusion-constraint snapshot.
///
/// Every predecessor `EXCLUDE` constraint receives exactly one explicit timing observation. This
/// preserves the difference between `NOT DEFERRABLE`, `DEFERRABLE INITIALLY IMMEDIATE`, and
/// `DEFERRABLE INITIALLY DEFERRED` without inferring either PostgreSQL default from index flags.
/// Parent/child timing equality is deliberately not invented here; this successor records the exact
/// catalog tuple for each constraint and leaves any future cross-edge invariant to source evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintTimingSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintTimingObservation>,
}

impl IndexExclusionConstraintTimingSnapshot {
    /// Creates complete timing evidence over one exact exclusion-constraint predecessor snapshot.
    pub fn new(
        exclusion_constraint_snapshot: &IndexExclusionConstraintSnapshot,
        mut observations: Vec<IndexExclusionConstraintTimingObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

        let expected = exclusion_constraint_snapshot
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();

        if observed.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_timing_coordinate"));
        }
        if observed != expected {
            return Err(invalid("index_exclusion_constraint_timing_completeness"));
        }

        let snapshot_digest = compute_timing_digest(
            exclusion_constraint_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: exclusion_constraint_snapshot
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: exclusion_constraint_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision: exclusion_constraint_snapshot.extractor_revision().to_owned(),
            observed_at_utc: exclusion_constraint_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated timing successor digest.
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

    /// Returns complete timing observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintTimingObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed exclusion-constraint timing coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintTimingSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: timing_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintTimingSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_timing_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintTimingObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_TIMING_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update([u8::from(observation.deferrable())]);
        hasher.update([u8::from(observation.initially_deferred())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn timing_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/timing", coordinate.canonical_location())
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexExclusionConstraintCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
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
