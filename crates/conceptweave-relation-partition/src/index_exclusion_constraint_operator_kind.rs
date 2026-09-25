//! PostgreSQL ordinary exclusion-constraint raw operator-kind evidence.
//!
//! `pg_operator.oprkind` is independent catalog state. A stable operator signature with two resolved
//! operand types does not prove that the source row itself was observed as a binary operator. This
//! successor binds the raw discriminator to each exact governed ordinary-EXCLUDE operator position
//! and admits only PostgreSQL's binary `b` state without rewriting predecessor digest domains.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorResultSnapshot,
    QualifiedOperatorSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.kind.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw `pg_operator.oprkind` observed for one exact ordinary-EXCLUDE operator position.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorKindObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    operator_kind: char,
}

impl IndexExclusionConstraintOperatorKindObservation {
    /// Records the raw catalog discriminator for one exact governed operator position.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        operator_kind: char,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            operator_kind,
        })
    }

    /// Returns the exact ordinary exclusion-constraint coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns the one-based exclusion-key position.
    #[must_use]
    pub const fn key_position(&self) -> u32 {
        self.key_position
    }

    /// Returns the exact governed operator repeated for predecessor binding.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the raw `pg_operator.oprkind` discriminator.
    #[must_use]
    pub const fn operator_kind(&self) -> char {
        self.operator_kind
    }

    /// Returns the collision-safe evidence location for this raw catalog fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        operator_kind_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact ordinary-EXCLUDE operator-kind observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorKindSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorKindObservation,
}

impl IndexExclusionConstraintOperatorKindSourceReceipt {
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

    /// Returns the owner-computed operator-kind successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the result predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the result predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact validated operator-kind observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorKindObservation {
        &self.location
    }
}

/// Complete raw operator-kind evidence over one exact ordinary-EXCLUDE result-contract snapshot.
///
/// Every governed `conexclop` position receives exactly one independently observed
/// `pg_operator.oprkind`. The repeated operator must equal the exact result predecessor position,
/// and the raw discriminator must be `b`, PostgreSQL's binary/infix state. The normalized left/right
/// operand signature is deliberately not used to infer this fact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorKindSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorKindObservation>,
}

impl IndexExclusionConstraintOperatorKindSnapshot {
    /// Creates complete raw operator-kind evidence over one exact result-contract predecessor.
    pub fn new(
        result_snapshot: &IndexExclusionConstraintOperatorResultSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorKindObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = result_snapshot
            .observations()
            .iter()
            .map(|observation| (observation.coordinate().clone(), observation.key_position()))
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| (observation.coordinate().clone(), observation.key_position()))
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_kind_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_kind_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = result_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| invalid("index_exclusion_constraint_operator_kind_completeness"))?;
            if predecessor.operator() != observation.operator() {
                return Err(invalid("index_exclusion_constraint_operator_kind_binding"));
            }
            if observation.operator_kind() != 'b' {
                return Err(invalid("index_exclusion_constraint_operator_kind_state"));
            }
        }

        let snapshot_digest =
            compute_operator_kind_digest(result_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: result_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: result_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: result_snapshot.extractor_revision().to_owned(),
            observed_at_utc: result_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated operator-kind successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the result predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the result predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete raw operator-kind observations in deterministic coordinate/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorKindObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE operator-kind position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorKindSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: operator_kind_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorKindSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_operator_kind_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorKindObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        hasher.update((observation.operator_kind() as u32).to_be_bytes());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn operator_kind_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/operator-kind",
        coordinate.canonical_location()
    )
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexExclusionConstraintCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
}

fn encode_operator(hasher: &mut Sha256, operator: &QualifiedOperatorSignature) {
    encode_str(hasher, operator.schema_name());
    encode_str(hasher, operator.operator_name());
    encode_type(hasher, operator.left_type());
    encode_type(hasher, operator.right_type());
}

fn encode_type(hasher: &mut Sha256, qualified_type: &conceptweave_observation::QualifiedTypeName) {
    encode_str(hasher, qualified_type.schema_name());
    encode_str(hasher, qualified_type.type_name());
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
