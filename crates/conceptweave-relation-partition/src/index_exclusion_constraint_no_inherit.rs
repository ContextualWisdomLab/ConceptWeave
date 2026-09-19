//! PostgreSQL exclusion-constraint `connoinherit` evidence.
//!
//! PostgreSQL 18 records `pg_constraint.connoinherit` independently. Creation with an explicit
//! parent constraint and creation as a standalone constraint initialize the bit differently, while
//! `ConstraintSetParentConstraint()` later changes parentage/locality without rewriting it. This
//! successor therefore retains the raw bit as governed evidence rather than deriving it from current
//! parentage.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot};

const INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.no_inherit.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_constraint.connoinherit` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintNoInheritObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    no_inherit: bool,
}

impl IndexExclusionConstraintNoInheritObservation {
    /// Records the raw PostgreSQL `connoinherit` bit for one exact EXCLUDE constraint.
    #[must_use]
    pub const fn new(coordinate: IndexExclusionConstraintCoordinate, no_inherit: bool) -> Self {
        Self {
            coordinate,
            no_inherit,
        }
    }

    /// Returns the exact exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns observed `pg_constraint.connoinherit`.
    #[must_use]
    pub const fn no_inherit(&self) -> bool {
        self.no_inherit
    }

    /// Returns the collision-safe evidence location for this no-inherit observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        no_inherit_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact EXCLUDE `connoinherit` observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintNoInheritSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintNoInheritObservation,
}

impl IndexExclusionConstraintNoInheritSourceReceipt {
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

    /// Returns the owner-computed no-inherit successor digest.
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

    /// Returns the exact no-inherit observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintNoInheritObservation {
        &self.location
    }
}

/// Complete `pg_constraint.connoinherit` evidence over one exact EXCLUDE identity snapshot.
///
/// Every predecessor EXCLUDE constraint receives exactly one explicit raw observation. PostgreSQL 18
/// can retain different `connoinherit` values for the same current parentage depending on whether a
/// child constraint was cloned with its parent or attached later as a preexisting standalone
/// constraint. The bit is therefore retained in its own digest domain without parentage-derived
/// normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintNoInheritSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintNoInheritObservation>,
}

impl IndexExclusionConstraintNoInheritSnapshot {
    /// Creates complete raw no-inherit evidence over one exact EXCLUDE identity predecessor.
    pub fn new(
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        mut observations: Vec<IndexExclusionConstraintNoInheritObservation>,
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
            return Err(invalid("index_exclusion_constraint_no_inherit_coordinate"));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_no_inherit_completeness",
            ));
        }

        let snapshot_digest =
            compute_no_inherit_digest(constraint_snapshot.snapshot_digest(), &observations);
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

    /// Returns the owner-computed domain-separated no-inherit successor digest.
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

    /// Returns complete no-inherit observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintNoInheritObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed EXCLUDE no-inherit coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintNoInheritSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: no_inherit_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintNoInheritSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_no_inherit_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintNoInheritObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_NO_INHERIT_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update([u8::from(observation.no_inherit())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn no_inherit_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/no-inherit", coordinate.canonical_location())
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
