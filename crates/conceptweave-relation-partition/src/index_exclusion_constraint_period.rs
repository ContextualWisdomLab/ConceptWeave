//! PostgreSQL ordinary exclusion-constraint `conperiod` evidence.
//!
//! PostgreSQL 18 reserves `pg_constraint.conperiod=true` for `WITHOUT OVERLAPS` PRIMARY KEY/UNIQUE
//! constraints and `PERIOD` FOREIGN KEY constraints. Ordinary `contype='x'` EXCLUDE constraints are
//! a separate catalog family, so this successor requires their explicitly observed `conperiod=false`
//! state without duplicating the temporal key/foreign-key owner contract.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot};

const INDEX_EXCLUSION_CONSTRAINT_PERIOD_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.period.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_constraint.conperiod` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintPeriodObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    has_period_semantics: bool,
}

impl IndexExclusionConstraintPeriodObservation {
    /// Records the PostgreSQL `conperiod` bit for one exact ordinary EXCLUDE constraint.
    ///
    /// PostgreSQL 18 defines `conperiod=true` only for `WITHOUT OVERLAPS` primary/unique constraints
    /// and `PERIOD` foreign keys. The ordinary EXCLUDE family therefore accepts only explicit false;
    /// temporal p/u/f evidence remains owned by `conceptweave-observation`.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        has_period_semantics: bool,
    ) -> Result<Self, ObservationError> {
        if has_period_semantics {
            return Err(invalid("index_exclusion_constraint_period_state"));
        }
        Ok(Self {
            coordinate,
            has_period_semantics,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns observed `pg_constraint.conperiod`.
    #[must_use]
    pub const fn has_period_semantics(&self) -> bool {
        self.has_period_semantics
    }

    /// Returns the collision-safe evidence location for this period-state observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        period_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact ordinary EXCLUDE `conperiod` observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintPeriodSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintPeriodObservation,
}

impl IndexExclusionConstraintPeriodSourceReceipt {
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

    /// Returns the owner-computed ordinary EXCLUDE period-state successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the EXCLUDE identity predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact ordinary EXCLUDE period-state observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintPeriodObservation {
        &self.location
    }
}

/// Complete `pg_constraint.conperiod` evidence over one exact ordinary EXCLUDE identity snapshot.
///
/// Every predecessor ordinary EXCLUDE constraint receives exactly one explicit false observation.
/// This keeps extraction completeness auditable and fails closed on a catalog tuple that would
/// conflate temporal p/u/f semantics with `contype='x'` without changing any predecessor digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintPeriodSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintPeriodObservation>,
}

impl IndexExclusionConstraintPeriodSnapshot {
    /// Creates complete period-state evidence over one exact ordinary EXCLUDE identity predecessor.
    pub fn new(
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        mut observations: Vec<IndexExclusionConstraintPeriodObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

        let expected = constraint_snapshot
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();

        if observed.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_period_coordinate"));
        }
        if observed != expected {
            return Err(invalid("index_exclusion_constraint_period_completeness"));
        }

        let snapshot_digest =
            compute_period_digest(constraint_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: constraint_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: constraint_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: constraint_snapshot.extractor_revision().to_owned(),
            observed_at_utc: constraint_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the owner-computed domain-separated ordinary EXCLUDE period-state digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the EXCLUDE identity predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete period-state observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintPeriodObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE period-state coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintPeriodSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: period_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintPeriodSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_period_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintPeriodObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_PERIOD_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update([u8::from(observation.has_period_semantics())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn period_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/period", coordinate.canonical_location())
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
