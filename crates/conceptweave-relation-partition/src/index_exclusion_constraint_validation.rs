//! PostgreSQL exclusion-constraint validation evidence.
//!
//! PostgreSQL 18 records `pg_constraint.convalidated` independently for every constraint row.
//! `index_constraint_create()` creates index-backed constraints, including ordinary `EXCLUDE`, with
//! validation state set to true. This successor retains that raw catalog bit over the exact
//! exclusion-constraint enforcement predecessor without rewriting any issued digest.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexExclusionConstraintCoordinate, IndexExclusionConstraintEnforcementSnapshot};

const INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.validation.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_constraint.convalidated` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintValidationObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    validated: bool,
}

impl IndexExclusionConstraintValidationObservation {
    /// Records exact PostgreSQL exclusion-constraint validation state.
    ///
    /// PostgreSQL 18 `index_constraint_create()` supplies `isValidated = true` when it creates an
    /// index-backed constraint. An ordinary `contype = 'x'` observation with `convalidated = false`
    /// is therefore contradictory source evidence and fails closed instead of being normalized.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        validated: bool,
    ) -> Result<Self, ObservationError> {
        if !validated {
            return Err(invalid("index_exclusion_constraint_validation_state"));
        }
        Ok(Self {
            coordinate,
            validated,
        })
    }

    /// Returns the exact exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns observed `pg_constraint.convalidated`.
    #[must_use]
    pub const fn validated(&self) -> bool {
        self.validated
    }

    /// Returns the collision-safe evidence location for this validation observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        validation_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact exclusion-constraint validation observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintValidationSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintValidationObservation,
}

impl IndexExclusionConstraintValidationSourceReceipt {
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

    /// Returns the owner-computed validation successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the enforcement predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the enforcement predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact exclusion-constraint validation observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintValidationObservation {
        &self.location
    }
}

/// Complete `pg_constraint.convalidated` evidence over one exact EXCLUDE enforcement snapshot.
///
/// Every predecessor EXCLUDE constraint receives exactly one explicit validation observation. The raw
/// bit is retained in this domain-separated digest even though PostgreSQL 18 creates index-backed
/// EXCLUDE constraints validated. This makes extraction completeness auditable and rejects catalog
/// states that the supported creation path cannot produce.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintValidationSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintValidationObservation>,
}

impl IndexExclusionConstraintValidationSnapshot {
    /// Creates complete validation evidence over one exact enforcement predecessor snapshot.
    pub fn new(
        enforcement_snapshot: &IndexExclusionConstraintEnforcementSnapshot,
        mut observations: Vec<IndexExclusionConstraintValidationObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

        let expected = enforcement_snapshot
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();

        if observed.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_validation_coordinate"));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_validation_completeness",
            ));
        }

        let snapshot_digest =
            compute_validation_digest(enforcement_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: enforcement_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: enforcement_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: enforcement_snapshot.extractor_revision().to_owned(),
            observed_at_utc: enforcement_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the owner-computed domain-separated validation successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the enforcement predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the enforcement predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete validation observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintValidationObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed exclusion-constraint validation coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintValidationSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: validation_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintValidationSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_validation_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintValidationObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_VALIDATION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update([u8::from(observation.validated())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn validation_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/validation", coordinate.canonical_location())
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
