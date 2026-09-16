//! PostgreSQL ordinary exclusion-constraint operator-commutator evidence.
//!
//! PostgreSQL 18 requires every operator selected by an `EXCLUDE` constraint to be commutative and
//! checks that requirement by requiring `get_commutator(opid) == opid`. The existing ordinary
//! exclusion-operator successor preserves the resolved `pg_constraint.conexclop` vector, but that
//! operator identity alone does not preserve the independently stored `pg_operator.oprcom` edge.
//! This successor records the resolved commutator signature for every exact constraint/key position
//! and rejects a non-self commutator instead of inferring commutativity from an operator name,
//! operator family, strategy number, or underlying procedure.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorSnapshot,
    QualifiedOperatorSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.commutator.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact independently resolved `pg_operator.oprcom` evidence for one ordinary EXCLUDE key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorCommutatorObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    commutator: QualifiedOperatorSignature,
}

impl IndexExclusionConstraintOperatorCommutatorObservation {
    /// Records one exact exclusion operator and the operator resolved from its `oprcom` OID.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        commutator: QualifiedOperatorSignature,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            commutator,
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

    /// Returns the exact stable operator signature repeated from the governed `conexclop` position.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the exact stable operator signature independently resolved from `pg_operator.oprcom`.
    #[must_use]
    pub const fn commutator(&self) -> &QualifiedOperatorSignature {
        &self.commutator
    }

    /// Returns the collision-safe evidence location for this operator-commutator edge.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        commutator_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact exclusion operator-commutator observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorCommutatorSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorCommutatorObservation,
}

impl IndexExclusionConstraintOperatorCommutatorSourceReceipt {
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

    /// Returns the owner-computed commutator successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the operator predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the operator predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact validated operator-commutator observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorCommutatorObservation {
        &self.location
    }
}

/// Complete self-commutator evidence over an exact ordinary EXCLUDE operator predecessor.
///
/// Every position in every governed `pg_constraint.conexclop` vector must receive exactly one
/// independent commutator observation. The repeated operator signature must equal that predecessor
/// position and the resolved `oprcom` signature must equal the operator itself, matching PostgreSQL
/// 18's `get_commutator(opid) == opid` admission rule. OIDs remain adapter-local join coordinates;
/// only stable resolved signatures enter this immutable digest and provenance family.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorCommutatorSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorCommutatorObservation>,
}

impl IndexExclusionConstraintOperatorCommutatorSnapshot {
    /// Creates complete operator-commutator evidence over one exact ordinary EXCLUDE operator snapshot.
    pub fn new(
        operator_snapshot: &IndexExclusionConstraintOperatorSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorCommutatorObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = operator_snapshot
            .observations()
            .iter()
            .flat_map(|observation| {
                observation
                    .operators()
                    .iter()
                    .enumerate()
                    .map(move |(position, _)| {
                        (
                            observation.coordinate().clone(),
                            u32::try_from(position + 1)
                                .expect("PostgreSQL index key count must fit into canonical u32"),
                        )
                    })
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| (observation.coordinate().clone(), observation.key_position()))
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_commutator_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_commutator_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = operator_snapshot
                .observations()
                .iter()
                .find(|candidate| candidate.coordinate() == observation.coordinate())
                .ok_or_else(|| {
                    invalid("index_exclusion_constraint_operator_commutator_completeness")
                })?;
            let zero_based = observation
                .key_position()
                .checked_sub(1)
                .and_then(|position| usize::try_from(position).ok())
                .ok_or_else(|| invalid("index_exclusion_constraint_operator_commutator_binding"))?;
            let expected_operator = predecessor
                .operators()
                .get(zero_based)
                .ok_or_else(|| invalid("index_exclusion_constraint_operator_commutator_binding"))?;
            if observation.operator() != expected_operator {
                return Err(invalid(
                    "index_exclusion_constraint_operator_commutator_binding",
                ));
            }
            if observation.commutator() != observation.operator() {
                return Err(invalid(
                    "index_exclusion_constraint_operator_commutator_state",
                ));
            }
        }

        let snapshot_digest = compute_commutator_digest(
            operator_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: operator_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: operator_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: operator_snapshot.extractor_revision().to_owned(),
            observed_at_utc: operator_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated operator-commutator successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the operator predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the operator predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete commutator observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorCommutatorObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE operator position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorCommutatorSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: commutator_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorCommutatorSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_commutator_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorCommutatorObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_operator(&mut hasher, observation.commutator());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn commutator_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/commutator",
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

fn encode_type(hasher: &mut Sha256, qualified_type: &QualifiedTypeName) {
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
