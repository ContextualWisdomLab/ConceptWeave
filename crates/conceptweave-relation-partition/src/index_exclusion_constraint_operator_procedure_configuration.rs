//! PostgreSQL ordinary exclusion-constraint implementation-function local configuration evidence.
//!
//! `pg_proc.proconfig` stores function-local run-time configuration independently from the
//! routine's stable identity, owner, executable definition, and execution flags. PostgreSQL
//! permits `ALTER FUNCTION ... SET/RESET` without changing the function's input identity; for
//! `SECURITY DEFINER` routines, settings such as `search_path` are security material. This layer
//! therefore binds the exact same-row `proconfig` array as source evidence while reducing its raw
//! entries immediately to a domain-separated digest so arbitrary or custom GUC values are not
//! copied into downstream receipts.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate, IndexExclusionConstraintOperatorProcedureOwnerSnapshot,
    QualifiedOperatorSignature, QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_MATERIAL_DIGEST_DOMAIN_V1:
    &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.configuration.material.v1";
const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.configuration.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Privacy-preserving identity for one exact raw `pg_proc.proconfig` catalog value.
///
/// PostgreSQL represents no function-local configuration as `NULL`; an empty array is distinct
/// catalog state and remains distinct here. Array entry order and bytes are preserved in the
/// digest framing rather than normalized or reconstructed from session configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureConfigurationMaterial {
    configured: bool,
    entry_count: usize,
    digest: String,
}

impl IndexExclusionConstraintOperatorProcedureConfigurationMaterial {
    /// Reduces the exact raw same-row `pg_proc.proconfig` value to a domain-separated digest.
    #[must_use]
    pub fn from_proconfig(proconfig: Option<Vec<String>>) -> Self {
        let configured = proconfig.is_some();
        let entry_count = match proconfig.as_ref() {
            Some(entries) => entries.len(),
            None => 0,
        };
        let mut hasher = Sha256::new();
        hasher.update(
            INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_MATERIAL_DIGEST_DOMAIN_V1,
        );
        match proconfig {
            None => hasher.update([0]),
            Some(entries) => {
                hasher.update([1]);
                encode_len(&mut hasher, entries.len());
                for entry in entries {
                    encode_str(&mut hasher, &entry);
                }
            }
        }
        Self {
            configured,
            entry_count,
            digest: format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize()),
        }
    }

    /// Returns whether the source catalog value was non-null.
    #[must_use]
    pub const fn is_configured(&self) -> bool {
        self.configured
    }

    /// Returns the exact number of raw catalog-array entries consumed into the digest.
    #[must_use]
    pub const fn entry_count(&self) -> usize {
        self.entry_count
    }

    /// Returns the privacy-preserving digest of the exact raw catalog-array value.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// Exact configuration evidence for one governed ordinary-EXCLUDE implementation function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureConfigurationObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    configuration: IndexExclusionConstraintOperatorProcedureConfigurationMaterial,
}

impl IndexExclusionConstraintOperatorProcedureConfigurationObservation {
    /// Records one exact operator/function binding and its same-row `pg_proc.proconfig` identity.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        configuration: IndexExclusionConstraintOperatorProcedureConfigurationMaterial,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            configuration,
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

    /// Returns the exact `pg_operator.oprcode` function repeated for predecessor binding.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns the privacy-preserving identity of the exact raw `pg_proc.proconfig` value.
    #[must_use]
    pub const fn configuration(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureConfigurationMaterial {
        &self.configuration
    }

    /// Returns the collision-safe evidence location for this function-local configuration fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_configuration_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact implementation-function configuration observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureConfigurationSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureConfigurationObservation,
}

impl IndexExclusionConstraintOperatorProcedureConfigurationSourceReceipt {
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

    /// Returns the owner-computed configuration successor digest.
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

    /// Returns the exact validated configuration observation without exposing raw GUC values.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureConfigurationObservation {
        &self.location
    }
}

/// Complete same-row `pg_proc.proconfig` evidence over one exact function-owner predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureConfigurationSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureConfigurationObservation>,
}

impl IndexExclusionConstraintOperatorProcedureConfigurationSnapshot {
    /// Creates complete function-local configuration evidence over one exact owner predecessor.
    pub fn new(
        owner_snapshot: &IndexExclusionConstraintOperatorProcedureOwnerSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureConfigurationObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = owner_snapshot
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
                "index_exclusion_constraint_operator_procedure_configuration_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_configuration_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = owner_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_configuration_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_configuration_binding",
                ));
            }
        }

        let snapshot_digest =
            compute_procedure_configuration_digest(owner_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: owner_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: owner_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: owner_snapshot.extractor_revision().to_owned(),
            observed_at_utc: owner_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated implementation-function configuration successor digest.
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

    /// Returns complete configuration observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureConfigurationObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed implementation-function configuration identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureConfigurationSourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_configuration_location(&coordinate, key_position),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureConfigurationSourceReceipt {
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

fn compute_procedure_configuration_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureConfigurationObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        encode_str(&mut hasher, observation.configuration().digest());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_configuration_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-configuration",
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
