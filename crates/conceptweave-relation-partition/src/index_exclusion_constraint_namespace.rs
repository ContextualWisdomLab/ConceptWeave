//! PostgreSQL ordinary exclusion-constraint namespace evidence.
//!
//! PostgreSQL stores `pg_constraint.connamespace` independently from `conrelid`. For an index-backed
//! ordinary `EXCLUDE` constraint, PostgreSQL 18 creates the constraint in the owning relation's
//! namespace, but an extractor must still observe and resolve `connamespace` rather than deriving it
//! from the relation coordinate. This successor retains that resolved catalog fact and rejects a
//! cross-catalog mismatch without rewriting any issued predecessor digest.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot};

const INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.namespace.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact resolved `pg_constraint.connamespace` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintNamespaceObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    constraint_schema_name: String,
}

impl IndexExclusionConstraintNamespaceObservation {
    /// Records the independently resolved constraint namespace for one ordinary EXCLUDE constraint.
    ///
    /// The adapter must resolve `pg_constraint.connamespace` through `pg_namespace`; callers must not
    /// substitute the owning relation schema merely because normal PostgreSQL DDL keeps them equal.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        constraint_schema_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let constraint_schema_name = constraint_schema_name.into();
        validate_nonblank(
            &constraint_schema_name,
            "index_exclusion_constraint_namespace_name",
        )?;
        if constraint_schema_name != coordinate.schema_name() {
            return Err(invalid("index_exclusion_constraint_namespace_state"));
        }
        Ok(Self {
            coordinate,
            constraint_schema_name,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the independently resolved `pg_constraint.connamespace` schema name.
    #[must_use]
    pub fn constraint_schema_name(&self) -> &str {
        &self.constraint_schema_name
    }

    /// Returns a collision-safe source location for the constraint namespace observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        namespace_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact ordinary EXCLUDE namespace observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintNamespaceSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintNamespaceObservation,
}

impl IndexExclusionConstraintNamespaceSourceReceipt {
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

    /// Returns the owner-computed ordinary EXCLUDE namespace successor digest.
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

    /// Returns the exact resolved constraint namespace observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintNamespaceObservation {
        &self.location
    }
}

/// Complete resolved `pg_constraint.connamespace` evidence for ordinary EXCLUDE constraints.
///
/// Every predecessor constraint receives exactly one explicit namespace observation. PostgreSQL 18
/// creates an index-backed constraint using `RelationGetNamespace(heapRelation)` as the constraint
/// namespace; this contract therefore rejects a resolved namespace that differs from the owning
/// relation schema while preserving the raw independently observed name in digest and provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintNamespaceSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintNamespaceObservation>,
}

impl IndexExclusionConstraintNamespaceSnapshot {
    /// Creates complete namespace evidence over one exact ordinary EXCLUDE identity predecessor.
    pub fn new(
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        mut observations: Vec<IndexExclusionConstraintNamespaceObservation>,
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
            return Err(invalid("index_exclusion_constraint_namespace_coordinate"));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_namespace_completeness",
            ));
        }

        let snapshot_digest =
            compute_namespace_digest(constraint_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: constraint_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: constraint_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: constraint_snapshot.extractor_revision().to_owned(),
            observed_at_utc: constraint_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry reference.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy revision inherited from the predecessor snapshot.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed domain-separated ordinary EXCLUDE namespace digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor snapshot.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor snapshot.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete namespace observations in deterministic constraint-coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintNamespaceObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE constraint namespace.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintNamespaceSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: namespace_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintNamespaceSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_namespace_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintNamespaceObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_NAMESPACE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        encode_str(&mut hasher, observation.constraint_schema_name());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn namespace_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/constraint-namespace", coordinate.canonical_location())
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

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
