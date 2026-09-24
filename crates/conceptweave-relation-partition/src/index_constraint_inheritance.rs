//! PostgreSQL partition-constraint inheritance state layered over exact key-constraint parentage.
//!
//! `pg_constraint.conparentid`, `conislocal`, and `coninhcount` form one catalog invariant for
//! partition constraints. PostgreSQL's `ConstraintSetParentConstraint()` sets a newly parented
//! constraint to `conislocal = false`, increments `coninhcount` from zero to one, and records the
//! parent OID. Detach reverses those fields. This successor preserves the two inheritance fields
//! without changing the already-issued parentage digest domain.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexConstraintParentageCoordinate, IndexConstraintParentageSnapshot};

const INDEX_CONSTRAINT_INHERITANCE_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.constraint_parentage.inheritance_state.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_constraint.conislocal` and `coninhcount` observation for one key constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintInheritanceObservation {
    coordinate: IndexConstraintParentageCoordinate,
    is_local: bool,
    inheritance_count: i16,
}

impl IndexConstraintInheritanceObservation {
    /// Records exact PostgreSQL key-constraint inheritance state.
    pub fn new(
        coordinate: IndexConstraintParentageCoordinate,
        is_local: bool,
        inheritance_count: i16,
    ) -> Result<Self, ObservationError> {
        if inheritance_count < 0 {
            return Err(invalid("index_constraint_inheritance_count"));
        }
        Ok(Self {
            coordinate,
            is_local,
            inheritance_count,
        })
    }

    /// Returns the exact key-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexConstraintParentageCoordinate {
        &self.coordinate
    }

    /// Returns observed `pg_constraint.conislocal`.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }

    /// Returns observed `pg_constraint.coninhcount`.
    #[must_use]
    pub const fn inheritance_count(&self) -> i16 {
        self.inheritance_count
    }

    /// Returns a collision-safe evidence location distinct from the predecessor parentage path.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        inheritance_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact key-constraint inheritance-state observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintInheritanceSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexConstraintInheritanceObservation,
}

impl IndexConstraintInheritanceSourceReceipt {
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

    /// Returns the owner-computed inheritance-state successor digest.
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

    /// Returns the exact constraint inheritance-state observation.
    #[must_use]
    pub const fn location(&self) -> &IndexConstraintInheritanceObservation {
        &self.location
    }
}

/// Complete key-constraint inheritance state over one exact parentage snapshot.
///
/// A constraint with a resolved partition parent must be non-local with exactly one inheritance
/// parent. A key constraint without `conparentid` must remain local with zero inheritance parents.
/// This matches PostgreSQL's partition-constraint attach/detach transitions and intentionally does
/// not infer generic constraint inheritance outside the bounded PK/UNIQUE parentage family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConstraintInheritanceSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexConstraintInheritanceObservation>,
}

impl IndexConstraintInheritanceSnapshot {
    /// Creates complete inheritance-state evidence over an exact key-constraint parentage snapshot.
    pub fn new(
        parentage_snapshot: &IndexConstraintParentageSnapshot,
        mut observations: Vec<IndexConstraintInheritanceObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

        let expected = parentage_snapshot
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();

        if observed.len() != observations.len() {
            return Err(invalid("index_constraint_inheritance_coordinate"));
        }
        if observed != expected {
            return Err(invalid("index_constraint_inheritance_completeness"));
        }

        for observation in &observations {
            let parentage = parentage_snapshot
                .observations()
                .iter()
                .find(|candidate| candidate.coordinate() == observation.coordinate())
                .ok_or_else(|| invalid("index_constraint_inheritance_coordinate"))?;
            let expected_state = if parentage.parent_constraint().is_some() {
                (false, 1)
            } else {
                (true, 0)
            };
            if (observation.is_local(), observation.inheritance_count()) != expected_state {
                return Err(invalid("index_constraint_inheritance_state"));
            }
        }

        let snapshot_digest =
            compute_inheritance_digest(parentage_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: parentage_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: parentage_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: parentage_snapshot.extractor_revision().to_owned(),
            observed_at_utc: parentage_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the owner-computed domain-separated inheritance-state digest.
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

    /// Returns complete key-constraint inheritance state in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexConstraintInheritanceObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed key-constraint inheritance-state coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexConstraintParentageCoordinate,
    ) -> Result<IndexConstraintInheritanceSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: inheritance_location(&coordinate),
            })?;
        Ok(IndexConstraintInheritanceSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_inheritance_digest(
    predecessor_digest: &str,
    observations: &[IndexConstraintInheritanceObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_CONSTRAINT_INHERITANCE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update([u8::from(observation.is_local())]);
        hasher.update(observation.inheritance_count().to_be_bytes());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn inheritance_location(coordinate: &IndexConstraintParentageCoordinate) -> String {
    format!("{}/inheritance-state", coordinate.canonical_location())
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexConstraintParentageCoordinate) {
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
