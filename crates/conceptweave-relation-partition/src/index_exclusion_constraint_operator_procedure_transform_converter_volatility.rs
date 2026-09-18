//! PostgreSQL ordinary exclusion-constraint transform-converter volatility evidence.
//!
//! Converter definition, owner, object-level `EXECUTE` ACL, function-local configuration,
//! invoker/definer context, leakproof classification, and strictness do not determine
//! `pg_proc.provolatile`. PostgreSQL exposes `i`, `s`, and `v` as independent function state.
//! `check_transform_function()` rejects `v` when a transform is created, but PostgreSQL also allows
//! `ALTER FUNCTION ... VOLATILE`; Source Observation must therefore preserve a live volatile row as
//! post-creation drift instead of making that catalog state unobservable. Admission policy belongs
//! to validation, while this successor binds the raw same-row discriminator for every direction.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_VOLATILITY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.volatility.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw same-row `pg_proc.provolatile` evidence for one nonzero transform converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    volatility: char,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation {
    /// Records raw converter `pg_proc.provolatile` while repeating exact predecessor binding identity.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
        converter_schema_name: impl Into<String>,
        converter_function_name: impl Into<String>,
        volatility: char,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let converter_schema_name = converter_schema_name.into();
        let converter_function_name = converter_function_name.into();
        validate_nonblank(
            &converter_schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_volatility_function_schema",
        )?;
        validate_nonblank(
            &converter_function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_volatility_function_name",
        )?;
        if !matches!(volatility, 'i' | 's' | 'v') {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_volatility",
            ));
        }
        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            direction,
            converter_schema_name,
            converter_function_name,
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

    /// Returns the selected transform type for this converter direction.
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

    /// Returns raw `pg_proc.provolatile`: `i` immutable, `s` stable, or `v` volatile.
    #[must_use]
    pub const fn volatility(&self) -> char {
        self.volatility
    }

    /// Returns the collision-safe evidence location for this converter volatility fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_volatility_location(
            &self.coordinate,
            self.key_position,
            &self.transform_type,
            self.direction,
        )
    }
}

/// Immutable provenance receipt for one exact converter-volatility observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySourceReceipt {
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

    /// Returns the owner-computed converter volatility successor digest.
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

    /// Returns the exact validated converter-volatility observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation {
        &self.location
    }
}

/// Complete raw converter `pg_proc.provolatile` evidence over one exact strictness predecessor.
///
/// This layer is observational. Immutable, stable, and volatile live catalog states are all
/// representable and produce distinct successor digests. A volatile row is evidence of a state
/// that would fail fresh `CREATE TRANSFORM` admission, not a reason to discard the observation.
/// Observations must cover the predecessor converter-direction inventory exactly.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot {
    /// Creates complete volatility evidence for every converter direction in the predecessor.
    pub fn new(
        strictness_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot,
        mut observations: Vec<
            IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation,
        >,
    ) -> Result<Self, ObservationError> {
        observations.sort_by_key(volatility_key);

        let expected = strictness_snapshot
            .observations()
            .iter()
            .map(|observation| {
                volatility_coordinate_key(
                    observation.coordinate(),
                    observation.key_position(),
                    observation.transform_type(),
                    observation.direction(),
                )
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(volatility_key)
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_volatility_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_volatility_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = strictness_snapshot
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
                        "index_exclusion_constraint_operator_procedure_transform_converter_volatility_completeness",
                    )
                })?;
            if predecessor.converter_schema_name() != observation.converter_schema_name()
                || predecessor.converter_function_name() != observation.converter_function_name()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_volatility_binding",
                ));
            }
        }

        let snapshot_digest = compute_transform_converter_volatility_digest(
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

    /// Returns the domain-separated converter volatility successor digest.
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

    /// Returns complete observations in deterministic converter-direction order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed converter-volatility identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySourceReceipt,
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
                location: procedure_transform_converter_volatility_location(
                    &coordinate,
                    key_position,
                    &transform_type,
                    direction,
                ),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySourceReceipt {
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
struct ConverterVolatilityCoordinateKey {
    schema_name: String,
    relation_name: String,
    relation_kind: String,
    constraint_name: String,
    key_position: u32,
    transform_type_schema_name: String,
    transform_type_name: String,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
}

fn compute_transform_converter_volatility_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_VOLATILITY_DIGEST_DOMAIN_V1,
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
        hasher.update([observation.volatility() as u8]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn volatility_key(
    observation: &IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation,
) -> ConverterVolatilityCoordinateKey {
    volatility_coordinate_key(
        observation.coordinate(),
        observation.key_position(),
        observation.transform_type(),
        observation.direction(),
    )
}

fn volatility_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> ConverterVolatilityCoordinateKey {
    ConverterVolatilityCoordinateKey {
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

fn procedure_transform_converter_volatility_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> String {
    let transform_schema = encode_location_component(transform_type.schema_name());
    let transform_name = encode_location_component(transform_type.type_name());
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-converters/{transform_schema}.{transform_name}/{}/volatility",
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
