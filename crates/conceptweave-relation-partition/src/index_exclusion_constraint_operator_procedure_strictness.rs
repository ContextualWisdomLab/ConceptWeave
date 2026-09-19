//! PostgreSQL ordinary exclusion-constraint operator procedure strictness evidence.
//!
//! `pg_proc.proisstrict` is independent from procedure identity, return type, and `proretset`.
//! PostgreSQL's exclusion recheck path explicitly assumes strict operator semantics when it
//! short-circuits NULL index values, while the ordinary EXCLUDE creation path does not make raw
//! `proisstrict` part of the constraint identity. This successor therefore preserves the exact
//! catalog fact without inventing a stronger PostgreSQL admission rule: both strict and non-strict
//! source procedures are representable, but they must never collapse to the same governed digest.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorProcedureScalarSnapshot,
    QualifiedOperatorSignature, QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.strictness.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw `pg_proc.proisstrict` evidence for one exact ordinary-EXCLUDE implementation procedure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureStrictnessObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    is_strict: bool,
}

impl IndexExclusionConstraintOperatorProcedureStrictnessObservation {
    /// Records one exact governed operator/procedure binding and raw `pg_proc.proisstrict` state.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        is_strict: bool,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            is_strict,
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

    /// Returns the exact governed exclusion operator repeated for predecessor binding.
    #[must_use]
    pub const fn operator(&self) -> &QualifiedOperatorSignature {
        &self.operator
    }

    /// Returns the exact `pg_operator.oprcode` procedure repeated for predecessor binding.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns raw `pg_proc.proisstrict` state for the exact implementation procedure.
    #[must_use]
    pub const fn is_strict(&self) -> bool {
        self.is_strict
    }

    /// Returns the collision-safe evidence location for this strictness fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_strictness_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact procedure strictness observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureStrictnessSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureStrictnessObservation,
}

impl IndexExclusionConstraintOperatorProcedureStrictnessSourceReceipt {
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

    /// Returns the owner-computed strictness successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact validated strictness observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorProcedureStrictnessObservation {
        &self.location
    }
}

/// Complete raw `pg_proc.proisstrict` evidence over one exact scalar-procedure predecessor.
///
/// This layer is deliberately observational rather than prescriptive. PostgreSQL exposes
/// `proisstrict` as independent catalog state and its exclusion executor assumes strict semantics,
/// but ordinary EXCLUDE DDL does not establish `proisstrict=true` as a separate creation gate.
/// Consequently both Boolean states are admitted and domain-separated; completeness and exact
/// operator/procedure binding are the invariants enforced here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureStrictnessSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureStrictnessObservation>,
}

impl IndexExclusionConstraintOperatorProcedureStrictnessSnapshot {
    /// Creates complete strictness evidence over one exact scalar-procedure predecessor.
    pub fn new(
        scalar_snapshot: &IndexExclusionConstraintOperatorProcedureScalarSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureStrictnessObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = scalar_snapshot
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
                "index_exclusion_constraint_operator_procedure_strictness_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_strictness_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = scalar_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_strictness_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_strictness_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_strictness_digest(
            scalar_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: scalar_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: scalar_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: scalar_snapshot.extractor_revision().to_owned(),
            observed_at_utc: scalar_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated strictness successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete strictness observations in deterministic coordinate/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorProcedureStrictnessObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE procedure position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureStrictnessSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_strictness_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureStrictnessSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_procedure_strictness_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureStrictnessObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        hasher.update([u8::from(observation.is_strict())]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_strictness_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-strictness",
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

fn encode_procedure(hasher: &mut Sha256, procedure: &QualifiedProcedureSignature) {
    encode_str(hasher, procedure.schema_name());
    encode_str(hasher, procedure.procedure_name());
    encode_len(hasher, procedure.argument_types().len());
    for argument_type in procedure.argument_types() {
        encode_type(hasher, argument_type);
    }
}

fn encode_type(
    hasher: &mut Sha256,
    qualified_type: &conceptweave_observation::QualifiedTypeName,
) {
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
