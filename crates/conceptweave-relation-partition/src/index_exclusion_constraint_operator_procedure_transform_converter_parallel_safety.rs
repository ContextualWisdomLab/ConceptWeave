//! PostgreSQL ordinary exclusion-constraint transform-converter parallel-safety evidence.
//!
//! Converter definition, owner, object-level `EXECUTE` ACL, function-local configuration,
//! invoker/definer context, leakproof classification, strictness, and volatility do not determine
//! `pg_proc.proparallel`. PostgreSQL records `s`, `r`, and `u` for parallel safe, restricted, and
//! unsafe functions. This successor binds that raw same-row discriminator for every exact converter
//! direction without inventing a parallel-safe-only ordinary-EXCLUDE admission rule.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_PARALLEL_SAFETY_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.parallel_safety.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw same-row `pg_proc.proparallel` evidence for one nonzero transform converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    parallel_safety: char,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation {
    /// Records raw converter `pg_proc.proparallel` while repeating exact predecessor binding identity.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
        converter_schema_name: impl Into<String>,
        converter_function_name: impl Into<String>,
        parallel_safety: char,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let converter_schema_name = converter_schema_name.into();
        let converter_function_name = converter_function_name.into();
        validate_nonblank(
            &converter_schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_function_schema",
        )?;
        validate_nonblank(
            &converter_function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_function_name",
        )?;
        if !matches!(parallel_safety, 's' | 'r' | 'u') {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety",
            ));
        }
        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            direction,
            converter_schema_name,
            converter_function_name,
            parallel_safety,
        })
    }

    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate { &self.coordinate }
    #[must_use]
    pub const fn key_position(&self) -> u32 { self.key_position }
    #[must_use]
    pub const fn transform_type(&self) -> &QualifiedTypeName { &self.transform_type }
    #[must_use]
    pub const fn direction(&self) -> IndexExclusionConstraintOperatorProcedureTransformConverterDirection { self.direction }
    #[must_use]
    pub fn converter_schema_name(&self) -> &str { &self.converter_schema_name }
    #[must_use]
    pub fn converter_function_name(&self) -> &str { &self.converter_function_name }
    #[must_use]
    pub const fn parallel_safety(&self) -> char { self.parallel_safety }
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_parallel_safety_location(&self.coordinate, self.key_position, &self.transform_type, self.direction)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySourceReceipt {
    #[must_use]
    pub fn source_id(&self) -> &str { &self.source_id }
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str { &self.connection_policy_binding }
    #[must_use]
    pub fn source_digest(&self) -> &str { &self.source_digest }
    #[must_use]
    pub fn extractor_revision(&self) -> &str { &self.extractor_revision }
    #[must_use]
    pub fn observed_at_utc(&self) -> &str { &self.observed_at_utc }
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation { &self.location }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    converter_snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot {
    pub fn new(
        volatility_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot,
        mut observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by_key(parallel_safety_key);
        let expected = volatility_snapshot.observations().iter().map(|observation| {
            parallel_safety_coordinate_key(observation.coordinate(), observation.key_position(), observation.transform_type(), observation.direction())
        }).collect::<BTreeSet<_>>();
        let observed = observations.iter().map(parallel_safety_key).collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_coordinate"));
        }
        if observed != expected {
            return Err(invalid("index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_completeness"));
        }
        for observation in &observations {
            let predecessor = volatility_snapshot.observations().iter().find(|candidate| {
                candidate.coordinate() == observation.coordinate()
                    && candidate.key_position() == observation.key_position()
                    && candidate.transform_type() == observation.transform_type()
                    && candidate.direction() == observation.direction()
            }).ok_or_else(|| invalid("index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_completeness"))?;
            if predecessor.converter_schema_name() != observation.converter_schema_name()
                || predecessor.converter_function_name() != observation.converter_function_name()
            {
                return Err(invalid("index_exclusion_constraint_operator_procedure_transform_converter_parallel_safety_binding"));
            }
        }
        let snapshot_digest = compute_transform_converter_parallel_safety_digest(volatility_snapshot.snapshot_digest(), &observations);
        Ok(Self {
            source_connection_key: volatility_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: volatility_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            converter_snapshot_digest: volatility_snapshot.converter_snapshot_digest().to_owned(),
            extractor_revision: volatility_snapshot.extractor_revision().to_owned(),
            observed_at_utc: volatility_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    #[must_use]
    pub fn source_connection_key(&self) -> &str { &self.source_connection_key }
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str { &self.connection_policy_binding }
    #[must_use]
    pub fn snapshot_digest(&self) -> &str { &self.snapshot_digest }
    /// Returns the immutable raw transform-converter root digest.
    #[must_use]
    pub fn converter_snapshot_digest(&self) -> &str { &self.converter_snapshot_digest }
    #[must_use]
    pub fn extractor_revision(&self) -> &str { &self.extractor_revision }
    #[must_use]
    pub fn observed_at_utc(&self) -> &str { &self.observed_at_utc }
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation] { &self.observations }

    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    ) -> Result<IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySourceReceipt, ObservationError> {
        let observation = self.observations.iter().find(|observation| {
            observation.coordinate() == &coordinate
                && observation.key_position() == key_position
                && observation.transform_type() == &transform_type
                && observation.direction() == direction
        }).ok_or_else(|| ObservationError::UnknownObservationLocation {
            location: procedure_transform_converter_parallel_safety_location(&coordinate, key_position, &transform_type, direction),
        })?;
        Ok(IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySourceReceipt {
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
struct ConverterParallelSafetyCoordinateKey {
    schema_name: String,
    relation_name: String,
    relation_kind: String,
    constraint_name: String,
    key_position: u32,
    transform_type_schema_name: String,
    transform_type_name: String,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
}

fn compute_transform_converter_parallel_safety_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_PARALLEL_SAFETY_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_type(&mut hasher, observation.transform_type());
        encode_str(&mut hasher, direction_token(observation.direction()));
        encode_str(&mut hasher, observation.converter_schema_name());
        encode_str(&mut hasher, observation.converter_function_name());
        hasher.update([observation.parallel_safety() as u8]);
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn parallel_safety_key(observation: &IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation) -> ConverterParallelSafetyCoordinateKey {
    parallel_safety_coordinate_key(observation.coordinate(), observation.key_position(), observation.transform_type(), observation.direction())
}

fn parallel_safety_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> ConverterParallelSafetyCoordinateKey {
    ConverterParallelSafetyCoordinateKey {
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

fn procedure_transform_converter_parallel_safety_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> String {
    let transform_schema = encode_location_component(transform_type.schema_name());
    let transform_name = encode_location_component(transform_type.type_name());
    format!("{}/exclusion-operators/{key_position}/procedure-transform-converters/{transform_schema}.{transform_name}/{}/parallel-safety", coordinate.canonical_location(), direction_token(direction))
}

fn encode_location_component(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'_' | b'-' => encoded.push(char::from(byte)),
            _ => {
                encoded.push('%');
                encoded.push(char::from(HEX[usize::from(byte >> 4)]));
                encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
            }
        }
    }
    encoded
}

const fn direction_token(direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection) -> &'static str {
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
    if value.trim().is_empty() { return Err(invalid(field)); }
    Ok(())
}
fn invalid(field: &'static str) -> ObservationError { ObservationError::InvalidObservationField { field } }
