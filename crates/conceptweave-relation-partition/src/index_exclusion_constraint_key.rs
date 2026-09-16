//! PostgreSQL ordinary exclusion-constraint `conkey` evidence.
//!
//! PostgreSQL 18 persists the first `ii_NumIndexKeyAttrs` values from
//! `IndexInfo::ii_IndexAttrNumbers` into `pg_constraint.conkey` when creating an index-backed
//! constraint. For an ordinary `EXCLUDE` constraint, simple columns therefore carry exact relation
//! attribute numbers and expression key positions carry zero. This successor preserves that raw
//! independently stored catalog vector and verifies it against the already-governed backing-index
//! key layout without rewriting any predecessor digest domain.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintPeriodSnapshot,
    IndexExclusionConstraintSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_KEY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.key.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact raw `pg_constraint.conkey` state for one ordinary `EXCLUDE` constraint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintKeyObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    attribute_numbers: Vec<i16>,
}

impl IndexExclusionConstraintKeyObservation {
    /// Records the exact ordered `int2[]` stored in `pg_constraint.conkey`.
    ///
    /// Values are retained raw here. The enclosing snapshot validates the complete vector against
    /// the exact backing-index key layout so contradictory catalog evidence fails closed.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        attribute_numbers: Vec<i16>,
    ) -> Result<Self, ObservationError> {
        if attribute_numbers.is_empty() {
            return Err(invalid("index_exclusion_constraint_key_state"));
        }
        Ok(Self {
            coordinate,
            attribute_numbers,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the exact ordered raw `pg_constraint.conkey` vector.
    #[must_use]
    pub fn attribute_numbers(&self) -> &[i16] {
        &self.attribute_numbers
    }

    /// Returns the collision-safe evidence location for this constraint-key observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        key_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact ordinary EXCLUDE `conkey` observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintKeySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintKeyObservation,
}

impl IndexExclusionConstraintKeySourceReceipt {
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

    /// Returns the owner-computed ordinary EXCLUDE constraint-key successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
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

    /// Returns the exact ordinary EXCLUDE constraint-key observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintKeyObservation {
        &self.location
    }
}

/// Complete `pg_constraint.conkey` evidence over one exact ordinary EXCLUDE period successor.
///
/// Every predecessor ordinary EXCLUDE coordinate receives exactly one raw key vector. The supplied
/// period successor is rebound against the exact EXCLUDE identity snapshot before use, preventing a
/// similarly shaped but unrelated predecessor from being substituted. Each raw vector must equal
/// the backing index's key attributes resolved against the relation: simple columns map to their
/// exact observed PostgreSQL attribute number and expressions map to zero; INCLUDE payload columns
/// are intentionally absent because PostgreSQL persists only `ii_NumIndexKeyAttrs` in `conkey`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintKeySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintKeyObservation>,
}

impl IndexExclusionConstraintKeySnapshot {
    /// Creates complete constraint-key evidence over an exact ordinary EXCLUDE predecessor stack.
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        period_snapshot: &IndexExclusionConstraintPeriodSnapshot,
        mut observations: Vec<IndexExclusionConstraintKeyObservation>,
    ) -> Result<Self, ObservationError> {
        let rebound_period = IndexExclusionConstraintPeriodSnapshot::new(
            constraint_snapshot,
            period_snapshot.observations().to_vec(),
        )?;
        if rebound_period.snapshot_digest() != period_snapshot.snapshot_digest() {
            return Err(invalid("index_exclusion_constraint_key_predecessor_binding"));
        }

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
            return Err(invalid("index_exclusion_constraint_key_coordinate"));
        }
        if observed != expected {
            return Err(invalid("index_exclusion_constraint_key_completeness"));
        }

        for observation in &observations {
            let constraint = constraint_snapshot
                .observations()
                .iter()
                .find(|candidate| candidate.coordinate() == observation.coordinate())
                .ok_or_else(|| invalid("index_exclusion_constraint_key_completeness"))?;
            let expected_key = expected_constraint_key(base_snapshot, constraint)?;
            if observation.attribute_numbers() != expected_key.as_slice() {
                return Err(invalid("index_exclusion_constraint_key_state"));
            }
        }

        let snapshot_digest = compute_key_digest(period_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: period_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: period_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: period_snapshot.extractor_revision().to_owned(),
            observed_at_utc: period_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the owner-computed domain-separated ordinary EXCLUDE constraint-key digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete constraint-key observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintKeyObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE `conkey` coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintKeySourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: key_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintKeySourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn expected_constraint_key(
    base_snapshot: &PostgresSchemaSnapshotV3,
    constraint: &super::IndexExclusionConstraintObservation,
) -> Result<Vec<i16>, ObservationError> {
    let coordinate = constraint.coordinate();
    let relation = base_snapshot
        .relations()
        .iter()
        .find(|relation| {
            relation.schema_name() == coordinate.schema_name()
                && relation.relation_name() == coordinate.relation_name()
                && relation.kind() == coordinate.relation_kind()
        })
        .ok_or_else(|| invalid("index_exclusion_constraint_key_relation"))?;
    let index = relation
        .indexes()
        .iter()
        .find(|index| index.index_name() == constraint.backing_index().index_name())
        .ok_or_else(|| invalid("index_exclusion_constraint_key_backing_index"))?;

    index
        .key_attributes()
        .iter()
        .map(|attribute| {
            let Some(attribute_name) = attribute.attribute_name() else {
                return Ok(0);
            };
            let column = relation
                .columns()
                .iter()
                .find(|column| column.column_name() == attribute_name)
                .ok_or_else(|| invalid("index_exclusion_constraint_key_attribute"))?;
            i16::try_from(column.ordinal_position())
                .map_err(|_| invalid("index_exclusion_constraint_key_attribute_number"))
        })
        .collect()
}

fn compute_key_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintKeyObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_KEY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        encode_len(&mut hasher, observation.attribute_numbers().len());
        for attribute_number in observation.attribute_numbers() {
            hasher.update(attribute_number.to_be_bytes());
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn key_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/key-attributes", coordinate.canonical_location())
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
