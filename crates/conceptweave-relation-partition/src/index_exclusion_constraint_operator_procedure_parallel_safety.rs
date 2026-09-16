//! PostgreSQL ordinary exclusion-constraint operator procedure parallel-safety evidence.
//!
//! `pg_proc.proparallel` is independent from procedure identity, return type, set-returning state,
//! strictness, and volatility. PostgreSQL records `s`, `r`, and `u` for parallel safe, restricted,
//! and unsafe routines. This successor preserves that raw catalog discriminator for the exact
//! `pg_operator.oprcode` implementation procedure without inventing an EXCLUDE-specific parallel
//! admission rule.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureVolatilitySnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.parallel_safety.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw `pg_proc.proparallel` evidence for one exact ordinary-EXCLUDE implementation procedure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureParallelSafetyObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    parallel_safety: char,
}

impl IndexExclusionConstraintOperatorProcedureParallelSafetyObservation {
    /// Records one exact governed operator/procedure binding and raw PostgreSQL parallel-safety state.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        parallel_safety: char,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if !matches!(parallel_safety, 's' | 'r' | 'u') {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_parallel_safety",
            ));
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            parallel_safety,
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

    /// Returns raw `pg_proc.proparallel`: `s` safe, `r` restricted, or `u` unsafe.
    #[must_use]
    pub const fn parallel_safety(&self) -> char {
        self.parallel_safety
    }

    /// Returns the collision-safe evidence location for this parallel-safety fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_parallel_safety_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact procedure parallel-safety observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureParallelSafetySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureParallelSafetyObservation,
}

impl IndexExclusionConstraintOperatorProcedureParallelSafetySourceReceipt {
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

    /// Returns the owner-computed procedure parallel-safety successor digest.
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

    /// Returns the exact validated procedure parallel-safety observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureParallelSafetyObservation {
        &self.location
    }
}

/// Complete raw `pg_proc.proparallel` evidence over one exact volatility predecessor.
///
/// This layer is observational. It preserves all three PostgreSQL catalog states because parallel
/// safety changes planner and execution behavior, but it does not create a local rule requiring an
/// ordinary exclusion operator's implementation procedure to be parallel safe.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureParallelSafetyObservation>,
}

impl IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot {
    /// Creates complete procedure parallel-safety evidence over one exact volatility predecessor.
    pub fn new(
        volatility_snapshot: &IndexExclusionConstraintOperatorProcedureVolatilitySnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureParallelSafetyObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = volatility_snapshot
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
                "index_exclusion_constraint_operator_procedure_parallel_safety_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_parallel_safety_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = volatility_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_parallel_safety_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_parallel_safety_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_parallel_safety_digest(
            volatility_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: volatility_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: volatility_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: volatility_snapshot.extractor_revision().to_owned(),
            observed_at_utc: volatility_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated procedure parallel-safety successor digest.
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

    /// Returns complete parallel-safety observations in deterministic coordinate/key order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureParallelSafetyObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE procedure position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureParallelSafetySourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_parallel_safety_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureParallelSafetySourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_procedure_parallel_safety_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureParallelSafetyObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        hasher.update([observation.parallel_safety() as u8]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_parallel_safety_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-parallel-safety",
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

#[cfg(test)]
mod tests {
    use super::*;
    use conceptweave_observation::{QualifiedTypeName, RelationKind};

    fn int4() -> QualifiedTypeName {
        QualifiedTypeName::new("pg_catalog", "int4").unwrap()
    }

    fn coordinate() -> IndexExclusionConstraintCoordinate {
        IndexExclusionConstraintCoordinate::new(
            "public",
            "bookings",
            RelationKind::Table,
            "bookings_no_overlap",
        )
        .unwrap()
    }

    fn operator() -> QualifiedOperatorSignature {
        QualifiedOperatorSignature::new("pg_catalog", "=", int4(), int4()).unwrap()
    }

    fn procedure() -> QualifiedProcedureSignature {
        QualifiedProcedureSignature::new("pg_catalog", "int4eq", vec![int4(), int4()]).unwrap()
    }

    fn observation(state: char) -> IndexExclusionConstraintOperatorProcedureParallelSafetyObservation {
        IndexExclusionConstraintOperatorProcedureParallelSafetyObservation::new(
            coordinate(),
            1,
            operator(),
            procedure(),
            state,
        )
        .unwrap()
    }

    #[test]
    fn parallel_safety_digest_domain_separates_all_postgres_states() {
        let safe = compute_procedure_parallel_safety_digest("sha256:predecessor", &[observation('s')]);
        let restricted =
            compute_procedure_parallel_safety_digest("sha256:predecessor", &[observation('r')]);
        let unsafe_state =
            compute_procedure_parallel_safety_digest("sha256:predecessor", &[observation('u')]);
        assert_ne!(safe, restricted);
        assert_ne!(safe, unsafe_state);
        assert_ne!(restricted, unsafe_state);
    }
}
