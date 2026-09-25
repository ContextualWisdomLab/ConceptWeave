//! PostgreSQL ordinary exclusion-constraint operator procedure volatility evidence.
//!
//! `pg_proc.provolatile` is independent from procedure identity, return type, set-returning state,
//! and strictness. PostgreSQL assigns `i`, `s`, and `v` distinct semantics for immutable, stable,
//! and volatile routines. This successor preserves that raw catalog discriminator for the exact
//! `pg_operator.oprcode` implementation procedure without inventing a stronger EXCLUDE admission
//! rule than PostgreSQL itself exposes.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureStrictnessSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.volatility.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw `pg_proc.provolatile` evidence for one exact ordinary-EXCLUDE implementation procedure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureVolatilityObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    volatility: char,
}

impl IndexExclusionConstraintOperatorProcedureVolatilityObservation {
    /// Records one exact governed operator/procedure binding and raw PostgreSQL volatility state.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        volatility: char,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if !matches!(volatility, 'i' | 's' | 'v') {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_volatility",
            ));
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            volatility,
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

    /// Returns raw `pg_proc.provolatile`: `i` immutable, `s` stable, or `v` volatile.
    #[must_use]
    pub const fn volatility(&self) -> char {
        self.volatility
    }

    /// Returns the collision-safe evidence location for this volatility fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_volatility_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact procedure-volatility observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureVolatilitySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureVolatilityObservation,
}

impl IndexExclusionConstraintOperatorProcedureVolatilitySourceReceipt {
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

    /// Returns the owner-computed volatility successor digest.
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

    /// Returns the exact validated procedure-volatility observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureVolatilityObservation {
        &self.location
    }
}

/// Complete raw `pg_proc.provolatile` evidence over one exact strictness predecessor.
///
/// This layer is observational. It preserves all three PostgreSQL catalog states because volatility
/// changes routine semantics and optimizer/MVCC behavior, but it does not create a new local rule
/// requiring an ordinary exclusion operator's implementation procedure to be immutable.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureVolatilitySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureVolatilityObservation>,
}

impl IndexExclusionConstraintOperatorProcedureVolatilitySnapshot {
    /// Creates complete procedure-volatility evidence over one exact strictness predecessor.
    pub fn new(
        strictness_snapshot: &IndexExclusionConstraintOperatorProcedureStrictnessSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureVolatilityObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = strictness_snapshot
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
                "index_exclusion_constraint_operator_procedure_volatility_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_volatility_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = strictness_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid("index_exclusion_constraint_operator_procedure_volatility_completeness")
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_volatility_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_volatility_digest(
            strictness_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: strictness_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: strictness_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: strictness_snapshot.extractor_revision().to_owned(),
            observed_at_utc: strictness_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated procedure-volatility successor digest.
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

    /// Returns complete volatility observations in deterministic coordinate/key order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureVolatilityObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE procedure position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureVolatilitySourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_volatility_location(&coordinate, key_position),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureVolatilitySourceReceipt {
                source_id: self.source_connection_key.clone(),
                connection_policy_binding: self.connection_policy_binding.clone(),
                source_digest: self.snapshot_digest.clone(),
                extractor_revision: self.extractor_revision.clone(),
                observed_at_utc: self.observed_at_utc.clone(),
                location: observation.clone(),
            },
        )
    }
}

fn compute_procedure_volatility_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureVolatilityObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        hasher.update([observation.volatility() as u8]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_volatility_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-volatility",
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
