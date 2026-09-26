//! PostgreSQL ordinary exclusion-constraint operator implementation-function owner evidence.
//!
//! `pg_proc.proowner` is independent catalog state from a function's stable input identity,
//! execution-definition material, and `prosecdef` mode. PostgreSQL permits `ALTER FUNCTION ...
//! OWNER TO` without changing the function's input identity and explicitly states that a
//! `SECURITY DEFINER` function subsequently executes as the new owner. This successor therefore
//! preserves the raw owner OID together with the exact role name independently resolved through
//! public `pg_roles`. It does not infer owner identity from schema ownership, session identity, or
//! function naming, and it does not impose a SECURITY-DEFINER-only admission rule.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureDefinitionSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.owner.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact raw `pg_proc.proowner` and resolved `pg_roles` identity for one implementation function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureOwnerObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    owner_oid: u32,
    owner_role_name: String,
}

impl IndexExclusionConstraintOperatorProcedureOwnerObservation {
    /// Records one exact governed operator/function binding and its independently observed owner.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        if owner_oid == 0 {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_owner_oid",
            ));
        }
        let owner_role_name = owner_role_name.into();
        if owner_role_name.is_empty() || owner_role_name.contains('\0') {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_owner_role_name",
            ));
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            owner_oid,
            owner_role_name,
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

    /// Returns the exact raw nonzero `pg_proc.proowner` OID observed in this source generation.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }

    /// Returns the exact `pg_roles.rolname` independently resolved for [`Self::owner_oid`].
    #[must_use]
    pub fn owner_role_name(&self) -> &str {
        &self.owner_role_name
    }

    /// Returns the collision-safe evidence location for this function-owner fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_owner_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact implementation-function owner observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureOwnerSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureOwnerObservation,
}

impl IndexExclusionConstraintOperatorProcedureOwnerSourceReceipt {
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

    /// Returns the owner-computed function-owner successor digest.
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

    /// Returns the exact validated function-owner observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorProcedureOwnerObservation {
        &self.location
    }
}

/// Complete `pg_proc.proowner` evidence over one exact implementation-definition predecessor.
///
/// Every governed ordinary-EXCLUDE operator position must receive exactly one owner observation
/// bound to the same stable operator and exact `oprcode` function. Both invoker and definer
/// functions use this observational layer because ownership is independent catalog state; the
/// predecessor's `prosecdef` value determines whether that owner is the execution principal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureOwnerSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureOwnerObservation>,
}

impl IndexExclusionConstraintOperatorProcedureOwnerSnapshot {
    /// Creates complete function-owner evidence over one exact definition predecessor.
    pub fn new(
        definition_snapshot: &IndexExclusionConstraintOperatorProcedureDefinitionSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureOwnerObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = definition_snapshot
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
                "index_exclusion_constraint_operator_procedure_owner_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_owner_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = definition_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid("index_exclusion_constraint_operator_procedure_owner_completeness")
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_owner_binding",
                ));
            }
        }

        let snapshot_digest =
            compute_procedure_owner_digest(definition_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: definition_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: definition_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: definition_snapshot.extractor_revision().to_owned(),
            observed_at_utc: definition_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated implementation-function owner successor digest.
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

    /// Returns complete owner observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorProcedureOwnerObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE implementation-function owner.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureOwnerSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_owner_location(&coordinate, key_position),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureOwnerSourceReceipt {
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

fn compute_procedure_owner_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureOwnerObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        hasher.update(observation.owner_oid().to_be_bytes());
        encode_str(&mut hasher, observation.owner_role_name());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_owner_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-owner",
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
