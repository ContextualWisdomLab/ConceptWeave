//! PostgreSQL ordinary exclusion-constraint transform-converter owner evidence.
//!
//! The transform-converter predecessor content-binds each nonzero `pg_transform` converter function,
//! but a converter's `pg_proc.proowner` is independently mutable catalog state. PostgreSQL permits
//! `ALTER FUNCTION ... OWNER TO` without changing the function's input identity or implementation
//! body, while ownership carries the authority to alter or drop that function. This observational
//! successor therefore binds every nonzero FROM-SQL and TO-SQL converter to its exact raw owner OID
//! and same-generation resolved role name. It does not infer ownership from schema ownership,
//! transform ownership, creator/session identity, ACLs, or security mode.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.owner.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Direction of one nonzero `pg_transform` converter function.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum IndexExclusionConstraintOperatorProcedureTransformConverterDirection {
    /// `pg_transform.trffromsql`: SQL value to procedural-language representation.
    FromSql,
    /// `pg_transform.trftosql`: procedural-language representation to SQL value.
    ToSql,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterDirection {
    const fn token(self) -> &'static str {
        match self {
            Self::FromSql => "from_sql",
            Self::ToSql => "to_sql",
        }
    }
}

/// Exact `pg_proc.proowner` evidence for one nonzero transform converter function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    owner_oid: u32,
    owner_role_name: String,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation {
    /// Records independently observed converter owner identity for one exact converter direction.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
        converter_schema_name: impl Into<String>,
        converter_function_name: impl Into<String>,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let converter_schema_name = converter_schema_name.into();
        let converter_function_name = converter_function_name.into();
        let owner_role_name = owner_role_name.into();
        validate_nonblank(
            &converter_schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_owner_function_schema",
        )?;
        validate_nonblank(
            &converter_function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_owner_function_name",
        )?;
        if owner_oid == 0 {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_owner_oid",
            ));
        }
        validate_nonblank(
            &owner_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_owner_role_name",
        )?;
        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            direction,
            converter_schema_name,
            converter_function_name,
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

    /// Returns the exact selected transform type whose row owns this converter direction.
    #[must_use]
    pub const fn transform_type(&self) -> &QualifiedTypeName {
        &self.transform_type
    }

    /// Returns whether this is the FROM-SQL or TO-SQL converter.
    #[must_use]
    pub const fn direction(
        &self,
    ) -> IndexExclusionConstraintOperatorProcedureTransformConverterDirection {
        self.direction
    }

    /// Returns the exact schema containing the nonzero converter function.
    #[must_use]
    pub fn converter_schema_name(&self) -> &str {
        &self.converter_schema_name
    }

    /// Returns the exact converter-function name.
    #[must_use]
    pub fn converter_function_name(&self) -> &str {
        &self.converter_function_name
    }

    /// Returns the exact raw nonzero `pg_proc.proowner` OID.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }

    /// Returns the exact same-generation role name resolved for [`Self::owner_oid`].
    #[must_use]
    pub fn owner_role_name(&self) -> &str {
        &self.owner_role_name
    }

    /// Returns the collision-safe evidence location for this converter-owner fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_owner_location(
            &self.coordinate,
            self.key_position,
            &self.transform_type,
            self.direction,
        )
    }
}

/// Immutable provenance receipt for one exact converter-owner observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSourceReceipt {
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

    /// Returns the owner-computed converter-owner successor digest.
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

    /// Returns the exact validated converter-owner observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation {
        &self.location
    }
}

/// Complete converter-function owner evidence over one exact transform-converter predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot {
    /// Creates complete owner evidence for every nonzero converter function in the predecessor.
    pub fn new(
        converter_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot,
        mut observations: Vec<
            IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation,
        >,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| owner_key(left).cmp(&owner_key(right)));

        let expected = converter_snapshot
            .observations()
            .iter()
            .flat_map(|observation| {
                observation.converters().iter().flat_map(move |binding| {
                    let from_sql = binding.from_sql().map(|_| {
                        owner_coordinate_key(
                            observation.coordinate(),
                            observation.key_position(),
                            binding.transform_type(),
                            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                        )
                    });
                    let to_sql = binding.to_sql().map(|_| {
                        owner_coordinate_key(
                            observation.coordinate(),
                            observation.key_position(),
                            binding.transform_type(),
                            IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
                        )
                    });
                    from_sql.into_iter().chain(to_sql)
                })
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| {
                owner_coordinate_key(
                    observation.coordinate(),
                    observation.key_position(),
                    observation.transform_type(),
                    observation.direction(),
                )
            })
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_owner_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_owner_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = converter_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_converter_owner_completeness",
                    )
                })?;
            let binding = predecessor
                .converters()
                .iter()
                .find(|candidate| candidate.transform_type() == observation.transform_type())
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_converter_owner_completeness",
                    )
                })?;
            let converter = match observation.direction() {
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql => {
                    binding.from_sql()
                }
                IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql => {
                    binding.to_sql()
                }
            }
            .ok_or_else(|| {
                invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_owner_completeness",
                )
            })?;
            if converter.schema_name() != observation.converter_schema_name()
                || converter.function_name() != observation.converter_function_name()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_owner_binding",
                ));
            }
        }

        let snapshot_digest = compute_transform_converter_owner_digest(
            converter_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: converter_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: converter_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: converter_snapshot.extractor_revision().to_owned(),
            observed_at_utc: converter_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated converter-owner successor digest.
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

    /// Returns complete owner observations in deterministic converter order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed converter owner.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    ) -> Result<IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSourceReceipt, ObservationError>
    {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
                    && observation.transform_type() == &transform_type
                    && observation.direction() == direction
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_transform_converter_owner_location(
                    &coordinate,
                    key_position,
                    &transform_type,
                    direction,
                ),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_transform_converter_owner_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_OWNER_DIGEST_DOMAIN_V1,
    );
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_type(&mut hasher, observation.transform_type());
        encode_str(&mut hasher, observation.direction().token());
        encode_str(&mut hasher, observation.converter_schema_name());
        encode_str(&mut hasher, observation.converter_function_name());
        hasher.update(observation.owner_oid().to_be_bytes());
        encode_str(&mut hasher, observation.owner_role_name());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn owner_key(
    observation: &IndexExclusionConstraintOperatorProcedureTransformConverterOwnerObservation,
) -> (String, String, String, String, u32, String, String, String) {
    (
        observation.coordinate().schema_name().to_owned(),
        observation.coordinate().relation_name().to_owned(),
        observation.coordinate().relation_kind().token().to_owned(),
        observation.coordinate().constraint_name().to_owned(),
        observation.key_position(),
        observation.transform_type().schema_name().to_owned(),
        observation.transform_type().type_name().to_owned(),
        observation.direction().token().to_owned(),
    )
}

fn owner_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> (String, String, String, String, u32, String, String, String) {
    (
        coordinate.schema_name().to_owned(),
        coordinate.relation_name().to_owned(),
        coordinate.relation_kind().token().to_owned(),
        coordinate.constraint_name().to_owned(),
        key_position,
        transform_type.schema_name().to_owned(),
        transform_type.type_name().to_owned(),
        direction.token().to_owned(),
    )
}

fn procedure_transform_converter_owner_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-converters/{}.{}/{}/owner",
        coordinate.canonical_location(),
        transform_type.schema_name(),
        transform_type.type_name(),
        direction.token(),
    )
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexExclusionConstraintCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
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

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
