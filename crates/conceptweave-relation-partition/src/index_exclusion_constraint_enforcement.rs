//! PostgreSQL exclusion-constraint enforcement evidence.
//!
//! PostgreSQL 18 exposes `pg_constraint.conenforced` for every constraint row, but `NOT ENFORCED`
//! is accepted only for `CHECK` and foreign-key constraints. An ordinary `EXCLUDE` constraint is
//! therefore source-valid only when `conenforced = true`. This successor binds that raw catalog bit
//! over the exact exclusion-constraint timing snapshot without rewriting any predecessor digest.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintTimingSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.enforcement.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_constraint.conenforced` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintEnforcementObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    enforced: bool,
}

impl IndexExclusionConstraintEnforcementObservation {
    /// Records exact PostgreSQL exclusion-constraint enforcement state.
    ///
    /// PostgreSQL 18 accepts `NOT ENFORCED` only for `CHECK` and foreign-key constraints. A
    /// `contype = 'x'` row with `conenforced = false` is therefore contradictory source evidence
    /// and fails closed instead of being normalized to the PostgreSQL default.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        enforced: bool,
    ) -> Result<Self, ObservationError> {
        if !enforced {
            return Err(invalid("index_exclusion_constraint_enforcement_state"));
        }
        Ok(Self {
            coordinate,
            enforced,
        })
    }

    /// Returns the exact exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns observed `pg_constraint.conenforced`.
    #[must_use]
    pub const fn enforced(&self) -> bool {
        self.enforced
    }

    /// Returns the collision-safe evidence location for this enforcement observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        enforcement_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact exclusion-constraint enforcement observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintEnforcementSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintEnforcementObservation,
}

impl IndexExclusionConstraintEnforcementSourceReceipt {
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

    /// Returns the owner-computed enforcement successor digest.
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

    /// Returns the exact exclusion-constraint enforcement observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintEnforcementObservation {
        &self.location
    }
}

/// Complete `pg_constraint.conenforced` evidence over one exact EXCLUDE timing snapshot.
///
/// Every predecessor EXCLUDE constraint receives exactly one explicit enforcement observation. The
/// raw bit is retained in this domain-separated digest even though PostgreSQL 18 permits only the
/// enforced state for EXCLUDE constraints. This makes extraction completeness auditable and rejects
/// catalog states that PostgreSQL's supported DDL cannot create.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintEnforcementSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintEnforcementObservation>,
}

impl IndexExclusionConstraintEnforcementSnapshot {
    /// Creates complete enforcement evidence over one exact timing predecessor snapshot.
    pub fn new(
        timing_snapshot: &IndexExclusionConstraintTimingSnapshot,
        mut observations: Vec<IndexExclusionConstraintEnforcementObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

        let expected = timing_snapshot
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();

        if observed.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_enforcement_coordinate"));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_enforcement_completeness",
            ));
        }

        let snapshot_digest = compute_enforcement_digest(timing_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: timing_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: timing_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: timing_snapshot.extractor_revision().to_owned(),
            observed_at_utc: timing_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the owner-computed domain-separated enforcement successor digest.
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

    /// Returns complete enforcement observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintEnforcementObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed exclusion-constraint enforcement coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintEnforcementSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: enforcement_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintEnforcementSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_enforcement_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintEnforcementObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_ENFORCEMENT_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update([u8::from(observation.enforced())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn enforcement_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/enforcement", coordinate.canonical_location())
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
