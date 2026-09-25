//! PostgreSQL ordinary exclusion-constraint transform-converter function extension-membership evidence.
//!
//! A converter function can be attached to or detached from a PostgreSQL extension without changing
//! its `pg_proc` identity or the `pg_transform` row that selects it. PostgreSQL records that lifecycle
//! edge in `pg_depend` with `deptype='e'`; an extension member can only be dropped through its owning
//! extension and is handled as extension-owned state by `pg_dump`. This successor preserves exact
//! absence or the resolved `pg_extension.extname` without copying extension-owned metadata.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_EXTENSION_MEMBERSHIP_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.extension_membership.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact converter-function `pg_depend.deptype='e'` membership for one converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation
{
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    extension_name: Option<String>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation {
    /// Records exact absence or the resolved member extension name for one converter function.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
        converter_schema_name: impl Into<String>,
        converter_function_name: impl Into<String>,
        extension_name: Option<String>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let converter_schema_name = converter_schema_name.into();
        let converter_function_name = converter_function_name.into();
        validate_nonblank(
            &converter_schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_function_schema",
        )?;
        validate_nonblank(
            &converter_function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_function_name",
        )?;
        if let Some(extension_name) = extension_name.as_deref() {
            validate_nonblank(
                extension_name,
                "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_extension_name",
            )?;
        }

        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            direction,
            converter_schema_name,
            converter_function_name,
            extension_name,
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

    /// Returns the transform type whose converter function is being described.
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

    /// Returns the exact converter-function schema repeated for predecessor binding.
    #[must_use]
    pub fn converter_schema_name(&self) -> &str {
        &self.converter_schema_name
    }

    /// Returns the exact converter-function name repeated for predecessor binding.
    #[must_use]
    pub fn converter_function_name(&self) -> &str {
        &self.converter_function_name
    }

    /// Returns the resolved member extension name, or `None` when no extension-membership edge exists.
    #[must_use]
    pub fn extension_name(&self) -> Option<&str> {
        self.extension_name.as_deref()
    }

    /// Returns the collision-safe evidence location for this converter-function lifecycle fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_extension_membership_location(
            &self.coordinate,
            self.key_position,
            &self.transform_type,
            self.direction,
        )
    }
}

/// Immutable provenance receipt for one exact converter-function extension-membership observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSourceReceipt
{
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location:
        IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSourceReceipt {
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

    /// Returns the owner-computed extension-membership successor digest.
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

    /// Returns the exact validated converter-function extension-membership observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation
    {
        &self.location
    }
}

/// Complete converter-function extension-membership evidence over one exact transform-selection predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    converter_snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<
        IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation,
    >,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSnapshot {
    /// Creates complete converter-function extension-membership evidence for every converter direction.
    pub fn new(
        transform_types_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesSnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by_key(extension_membership_key);

        let expected = transform_types_snapshot
            .observations()
            .iter()
            .map(|observation| {
                extension_membership_coordinate_key(
                    observation.coordinate(),
                    observation.key_position(),
                    observation.transform_type(),
                    observation.direction(),
                )
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(extension_membership_key)
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = transform_types_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                        && candidate.transform_type() == observation.transform_type()
                        && candidate.direction() == observation.direction()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_completeness",
                    )
                })?;
            if predecessor.converter_schema_name() != observation.converter_schema_name()
                || predecessor.converter_function_name() != observation.converter_function_name()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_extension_membership_binding",
                ));
            }
        }

        let snapshot_digest = compute_transform_converter_extension_membership_digest(
            transform_types_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: transform_types_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: transform_types_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            converter_snapshot_digest: transform_types_snapshot
                .converter_snapshot_digest()
                .to_owned(),
            extractor_revision: transform_types_snapshot.extractor_revision().to_owned(),
            observed_at_utc: transform_types_snapshot.observed_at_utc().to_owned(),
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

    /// Returns the domain-separated converter-function extension-membership successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the immutable raw transform-converter root digest.
    #[must_use]
    pub fn converter_snapshot_digest(&self) -> &str {
        &self.converter_snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor chain.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation time inherited from the predecessor chain.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete observations in deterministic converter-direction order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation]{
        &self.observations
    }

    /// Issues exact provenance for one observed converter-function extension-membership identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSourceReceipt,
        ObservationError,
    > {
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
                location: procedure_transform_converter_extension_membership_location(
                    &coordinate,
                    key_position,
                    &transform_type,
                    direction,
                ),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ConverterExtensionMembershipCoordinateKey {
    schema_name: String,
    relation_name: String,
    relation_kind: String,
    constraint_name: String,
    key_position: u32,
    transform_type_schema_name: String,
    transform_type_name: String,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
}

fn compute_transform_converter_extension_membership_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_EXTENSION_MEMBERSHIP_DIGEST_DOMAIN_V1,
    );
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_type(&mut hasher, observation.transform_type());
        encode_str(&mut hasher, direction_token(observation.direction()));
        encode_str(&mut hasher, observation.converter_schema_name());
        encode_str(&mut hasher, observation.converter_function_name());
        match observation.extension_name() {
            None => hasher.update([0]),
            Some(extension_name) => {
                hasher.update([1]);
                encode_str(&mut hasher, extension_name);
            }
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn extension_membership_key(
    observation: &IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation,
) -> ConverterExtensionMembershipCoordinateKey {
    extension_membership_coordinate_key(
        observation.coordinate(),
        observation.key_position(),
        observation.transform_type(),
        observation.direction(),
    )
}

fn extension_membership_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> ConverterExtensionMembershipCoordinateKey {
    ConverterExtensionMembershipCoordinateKey {
        schema_name: coordinate.schema_name().to_owned(),
        relation_name: coordinate.relation_name().to_owned(),
        relation_kind: coordinate.relation_kind().token().to_owned(),
        constraint_name: coordinate.constraint_name().to_owned(),
        key_position,
        transform_type_schema_name: transform_type.schema_name().to_owned(),
        transform_type_name: transform_type.type_name().to_owned(),
        direction,
    }
}

fn procedure_transform_converter_extension_membership_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> String {
    let transform_schema = encode_location_component(transform_type.schema_name());
    let transform_name = encode_location_component(transform_type.type_name());
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-converters/{transform_schema}.{transform_name}/{}/function-extension-membership",
        coordinate.canonical_location(),
        direction_token(direction),
    )
}

fn encode_location_component(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' => {
                encoded.push(char::from(byte));
            }
            _ => {
                encoded.push('%');
                encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
        }
    }
    encoded
}

const fn direction_token(
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> &'static str {
    match direction {
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql => "from_sql",
        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql => "to_sql",
    }
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
