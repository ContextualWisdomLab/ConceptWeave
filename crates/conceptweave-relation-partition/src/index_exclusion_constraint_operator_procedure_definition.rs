//! PostgreSQL ordinary exclusion-constraint operator implementation-definition evidence.
//!
//! Exact `pg_operator.oprcode -> pg_proc` identity is not sufficient to bind executable behavior:
//! PostgreSQL permits `CREATE OR REPLACE FUNCTION` to replace a function definition while preserving
//! the function's name, input argument types, return type, and dependent-object identity. The source
//! representation is language-specific: `pg_proc.prosrc` carries source text or a link symbol,
//! `probin` carries additional invocation information for relevant languages, and `prosqlbody`
//! carries the pre-parsed body for SQL-standard function notation. `prolang` determines how those
//! fields are interpreted.
//!
//! This successor consumes those same-row facts, derives a domain-separated definition digest, and
//! discards the plaintext definition material after construction. The receipt therefore proves a
//! content-bound implementation identity without copying function bodies or binary paths into
//! downstream provenance. Raw `pg_node_tree` text is source-generation evidence only; this contract
//! does not claim cross-major semantic equivalence for PostgreSQL's internal representation.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureLeakproofSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.definition.v1";
const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_MATERIAL_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.definition.material.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Privacy-preserving content identity derived from one exact `pg_proc` implementation definition.
///
/// The constructor consumes the resolved `pg_language.lanname` plus exact `prosrc`, optional `probin`,
/// and optional `prosqlbody`. Plaintext implementation material is reduced immediately to a
/// domain-separated digest and is not retained by this value object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureDefinitionMaterial {
    language_name: String,
    definition_digest: String,
}

impl IndexExclusionConstraintOperatorProcedureDefinitionMaterial {
    /// Derives implementation identity from one exact `pg_proc` row and its resolved language.
    pub fn new(
        language_name: impl Into<String>,
        prosrc: impl Into<String>,
        probin: Option<String>,
        prosqlbody: Option<String>,
    ) -> Result<Self, ObservationError> {
        let language_name = language_name.into();
        if language_name.trim().is_empty() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_definition_language",
            ));
        }
        let prosrc = prosrc.into();
        let definition_digest = compute_definition_material_digest(
            &language_name,
            &prosrc,
            probin.as_deref(),
            prosqlbody.as_deref(),
        );
        Ok(Self {
            language_name,
            definition_digest,
        })
    }

    /// Returns the independently resolved implementation language name.
    #[must_use]
    pub fn language_name(&self) -> &str {
        &self.language_name
    }

    /// Returns the digest of language plus exact `prosrc`/`probin`/`prosqlbody` material.
    #[must_use]
    pub fn definition_digest(&self) -> &str {
        &self.definition_digest
    }
}

/// Content-bound implementation-definition evidence for one exact ordinary-EXCLUDE function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureDefinitionObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    material: IndexExclusionConstraintOperatorProcedureDefinitionMaterial,
}

impl IndexExclusionConstraintOperatorProcedureDefinitionObservation {
    /// Records one exact function definition without retaining its plaintext source material.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        material: IndexExclusionConstraintOperatorProcedureDefinitionMaterial,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            material,
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

    /// Returns the independently resolved implementation language name.
    #[must_use]
    pub fn language_name(&self) -> &str {
        self.material.language_name()
    }

    /// Returns the domain-separated implementation-definition digest.
    #[must_use]
    pub fn definition_digest(&self) -> &str {
        self.material.definition_digest()
    }

    /// Returns the collision-safe evidence location for this implementation definition.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_definition_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact implementation-definition observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureDefinitionSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureDefinitionObservation,
}

impl IndexExclusionConstraintOperatorProcedureDefinitionSourceReceipt {
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

    /// Returns the owner-computed implementation-definition successor digest.
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

    /// Returns the exact validated implementation-definition observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorProcedureDefinitionObservation {
        &self.location
    }
}

/// Complete content-bound function-definition evidence over one exact leakproofness predecessor.
///
/// Every governed ordinary-EXCLUDE operator position must receive exactly one definition observation
/// bound to the same stable operator and exact `oprcode` function. Plaintext source/link/body values
/// are not retained after definition material derives its digest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureDefinitionSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureDefinitionObservation>,
}

impl IndexExclusionConstraintOperatorProcedureDefinitionSnapshot {
    /// Creates complete implementation-definition evidence over one exact leakproofness predecessor.
    pub fn new(
        leakproof_snapshot: &IndexExclusionConstraintOperatorProcedureLeakproofSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureDefinitionObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });

        let expected = leakproof_snapshot
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
                "index_exclusion_constraint_operator_procedure_definition_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_definition_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = leakproof_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_definition_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_definition_binding",
                ));
            }
        }

        let snapshot_digest = compute_procedure_definition_digest(
            leakproof_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: leakproof_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: leakproof_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: leakproof_snapshot.extractor_revision().to_owned(),
            observed_at_utc: leakproof_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated implementation-definition successor digest.
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

    /// Returns complete definition observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorProcedureDefinitionObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary-EXCLUDE implementation position.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<IndexExclusionConstraintOperatorProcedureDefinitionSourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_definition_location(&coordinate, key_position),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureDefinitionSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_definition_material_digest(
    language_name: &str,
    prosrc: &str,
    probin: Option<&str>,
    prosqlbody: Option<&str>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_MATERIAL_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, language_name);
    encode_str(&mut hasher, prosrc);
    encode_optional_str(&mut hasher, probin);
    encode_optional_str(&mut hasher, prosqlbody);
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn compute_procedure_definition_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureDefinitionObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        encode_str(&mut hasher, observation.language_name());
        encode_str(&mut hasher, observation.definition_digest());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn procedure_definition_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-definition",
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

fn encode_optional_str(hasher: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            hasher.update([1_u8]);
            encode_str(hasher, value);
        }
        None => hasher.update([0_u8]),
    }
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
