//! PostgreSQL ordinary exclusion-constraint backing-index namespace evidence.
//!
//! PostgreSQL stores an index relation's namespace in `pg_class.relnamespace`. An index is created
//! in the same schema as its parent table, but that catalog column remains an independently observed
//! source fact. The existing index coordinate carries the normalized owning-relation schema, so this
//! successor retains a separately resolved `relnamespace` name and fails closed on cross-catalog
//! drift instead of deriving index namespace from relation metadata.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintIndexNameSnapshot,
    IndexPartitionCoordinate,
};

const INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.index_namespace.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Independently resolved `pg_class.relnamespace` for one exact ordinary-EXCLUDE backing index.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexNamespaceObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    backing_index: IndexPartitionCoordinate,
    index_schema_name: String,
}

impl IndexExclusionConstraintIndexNamespaceObservation {
    /// Records the independently resolved backing-index namespace without deriving it from the table.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        backing_index: IndexPartitionCoordinate,
        index_schema_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let index_schema_name = index_schema_name.into();
        validate_nonblank(
            &index_schema_name,
            "index_exclusion_constraint_index_namespace_name",
        )?;
        Ok(Self {
            coordinate,
            backing_index,
            index_schema_name,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the exact `conindid` backing-index coordinate proven by the predecessor.
    #[must_use]
    pub const fn backing_index(&self) -> &IndexPartitionCoordinate {
        &self.backing_index
    }

    /// Returns the namespace name independently resolved from `pg_class.relnamespace`.
    #[must_use]
    pub fn index_schema_name(&self) -> &str {
        &self.index_schema_name
    }

    /// Returns a collision-safe location for the backing-index namespace observation.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        index_namespace_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one ordinary-EXCLUDE backing-index namespace observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexNamespaceSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintIndexNamespaceObservation,
}

impl IndexExclusionConstraintIndexNamespaceSourceReceipt {
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

    /// Returns the owner-computed backing-index namespace digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the index-name predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the validated backing-index namespace observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintIndexNamespaceObservation {
        &self.location
    }
}

/// Complete backing-index `pg_class.relnamespace` evidence over the exact index-name predecessor.
///
/// Every predecessor ordinary EXCLUDE constraint receives exactly one explicit namespace
/// observation bound to the exact backing-index coordinate already proven by `conindid`. The raw
/// resolved namespace enters this successor digest and provenance. PostgreSQL requires the index to
/// live in the parent table's schema, so any independently observed namespace drift fails closed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintIndexNamespaceSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    source_snapshot_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintIndexNamespaceObservation>,
}

impl IndexExclusionConstraintIndexNamespaceSnapshot {
    /// Validates complete independently resolved backing-index namespace evidence.
    pub fn new(
        index_name_snapshot: &IndexExclusionConstraintIndexNameSnapshot,
        mut observations: Vec<IndexExclusionConstraintIndexNamespaceObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));

        let expected = index_name_snapshot
            .observations()
            .iter()
            .map(|observation| binding_key(observation.coordinate(), observation.backing_index()))
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| binding_key(observation.coordinate(), observation.backing_index()))
            .collect::<BTreeSet<_>>();
        let observed_coordinates = observations
            .iter()
            .map(|observation| observation.coordinate().canonical_location())
            .collect::<BTreeSet<_>>();

        if observed_coordinates.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_index_namespace_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_index_namespace_completeness",
            ));
        }

        for observation in &observations {
            if observation.index_schema_name() != observation.backing_index().schema_name()
                || observation.index_schema_name() != observation.coordinate().schema_name()
            {
                return Err(invalid("index_exclusion_constraint_index_namespace_state"));
            }
        }

        let snapshot_digest =
            compute_index_namespace_digest(index_name_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: index_name_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: index_name_snapshot.connection_policy_binding().to_owned(),
            source_snapshot_digest: index_name_snapshot.source_snapshot_digest().to_owned(),
            snapshot_digest,
            extractor_revision: index_name_snapshot.extractor_revision().to_owned(),
            observed_at_utc: index_name_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry reference.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding inherited from the predecessor.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact v3 source-content digest inherited from the index-name predecessor.
    #[must_use]
    pub fn source_snapshot_digest(&self) -> &str {
        &self.source_snapshot_digest
    }

    /// Returns the domain-separated backing-index namespace digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete namespace observations in deterministic constraint order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintIndexNamespaceObservation] {
        &self.observations
    }

    /// Issues provenance for one validated ordinary-EXCLUDE backing-index namespace observation.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintIndexNamespaceSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: index_namespace_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintIndexNamespaceSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn binding_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    backing_index: &IndexPartitionCoordinate,
) -> String {
    format!(
        "{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}\u{1f}{}",
        coordinate.schema_name(),
        coordinate.relation_name(),
        coordinate.relation_kind().token(),
        coordinate.constraint_name(),
        backing_index.schema_name(),
        backing_index.relation_name(),
        backing_index.relation_kind().token(),
        backing_index.index_name(),
    )
}

fn compute_index_namespace_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintIndexNamespaceObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_constraint_coordinate(&mut hasher, observation.coordinate());
        encode_index_coordinate(&mut hasher, observation.backing_index());
        encode_str(&mut hasher, observation.index_schema_name());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn index_namespace_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!(
        "{}/backing-index-namespace",
        coordinate.canonical_location()
    )
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

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
