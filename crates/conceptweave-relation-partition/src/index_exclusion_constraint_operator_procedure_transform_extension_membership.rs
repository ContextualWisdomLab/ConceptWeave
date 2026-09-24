//! PostgreSQL ordinary exclusion-constraint transform-object extension-membership evidence.
//!
//! A `pg_transform` row is an extension-member-capable object independently of the converter
//! functions referenced by `trffromsql` and `trftosql`. PostgreSQL 18 permits
//! `ALTER EXTENSION ... ADD|DROP TRANSFORM FOR type LANGUAGE language`, so the transform object's
//! lifecycle can change without changing its `(trftype, trflang)` identity or either converter
//! function. This successor preserves exact absence or the resolved `pg_extension.extname` for the
//! transform object itself. Converter-function membership, auto-extension dependencies, security
//! labels, initial privileges, and extension-owned metadata stay in their existing owners while the
//! latest converter initial-privilege successor remains the digest predecessor.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot,
    IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_EXTENSION_MEMBERSHIP_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform.extension_membership.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Exact `pg_transform` object extension membership for one transform type/language row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    target_language_name: String,
    extension_name: Option<String>,
}

impl IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation {
    /// Records exact absence or one resolved member extension for a `pg_transform` object.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        target_language_name: impl Into<String>,
        extension_name: Option<String>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let target_language_name = target_language_name.into();
        validate_postgresql_identifier(
            &target_language_name,
            "index_exclusion_constraint_operator_procedure_transform_extension_membership_language",
        )?;
        if let Some(extension_name) = extension_name.as_deref() {
            validate_postgresql_identifier(
                extension_name,
                "index_exclusion_constraint_operator_procedure_transform_extension_membership_extension_name",
            )?;
        }
        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            target_language_name,
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

    /// Returns the exact `pg_transform.trftype` identity.
    #[must_use]
    pub const fn transform_type(&self) -> &QualifiedTypeName {
        &self.transform_type
    }

    /// Returns the exact target language resolved from `pg_transform.trflang`.
    #[must_use]
    pub fn target_language_name(&self) -> &str {
        &self.target_language_name
    }

    /// Returns the resolved member extension name, or `None` when no membership edge exists.
    #[must_use]
    pub fn extension_name(&self) -> Option<&str> {
        self.extension_name.as_deref()
    }

    /// Returns the collision-safe evidence location for this transform-object lifecycle fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_extension_membership_location(
            &self.coordinate,
            self.key_position,
            &self.transform_type,
            &self.target_language_name,
        )
    }
}

/// Immutable provenance receipt for one transform-object extension-membership observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSourceReceipt {
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

    /// Returns the owner-computed transform-object extension-membership successor digest.
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

    /// Returns the exact validated transform-object extension-membership observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation {
        &self.location
    }
}

/// Complete transform-object extension-membership evidence over the latest converter-function chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations:
        Vec<IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot {
    /// Creates one extension-membership fact for each exact same-generation `pg_transform` row.
    ///
    /// `converter_initial_privilege_snapshot` is the digest predecessor. The original
    /// transform-converter snapshot supplies the transform-row identity `(trftype, trflang)` that is
    /// not a per-direction converter-function fact. Both inputs must describe the same source
    /// generation, descend from the exact same immutable raw transform-converter root, and retain
    /// the same converter direction/function set.
    pub fn new(
        converter_initial_privilege_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterInitialPrivilegeSnapshot,
        transform_converter_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot,
        mut observations: Vec<
            IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation,
        >,
    ) -> Result<Self, ObservationError> {
        if converter_initial_privilege_snapshot.source_connection_key()
            != transform_converter_snapshot.source_connection_key()
            || converter_initial_privilege_snapshot.connection_policy_binding()
                != transform_converter_snapshot.connection_policy_binding()
            || converter_initial_privilege_snapshot.extractor_revision()
                != transform_converter_snapshot.extractor_revision()
            || converter_initial_privilege_snapshot.observed_at_utc()
                != transform_converter_snapshot.observed_at_utc()
        {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_extension_membership_generation",
            ));
        }

        if converter_initial_privilege_snapshot.converter_snapshot_digest()
            != transform_converter_snapshot.snapshot_digest()
        {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_extension_membership_lineage",
            ));
        }

        let converter_lifecycle_observations = converter_initial_privilege_snapshot.observations();
        let raw_converter_direction_count = transform_converter_snapshot
            .observations()
            .iter()
            .flat_map(|observation| observation.converters())
            .map(|binding| {
                usize::from(binding.from_sql().is_some()) + usize::from(binding.to_sql().is_some())
            })
            .sum::<usize>();
        if raw_converter_direction_count != converter_lifecycle_observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_extension_membership_binding",
            ));
        }
        for transform_observation in transform_converter_snapshot.observations() {
            for binding in transform_observation.converters() {
                for (direction, converter) in [
                    (
                        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::FromSql,
                        binding.from_sql(),
                    ),
                    (
                        IndexExclusionConstraintOperatorProcedureTransformConverterDirection::ToSql,
                        binding.to_sql(),
                    ),
                ] {
                    let Some(converter) = converter else {
                        continue;
                    };
                    let has_exact_converter_lifecycle = converter_lifecycle_observations
                        .iter()
                        .any(|candidate| {
                            candidate.coordinate() == transform_observation.coordinate()
                                && candidate.key_position() == transform_observation.key_position()
                                && candidate.transform_type() == binding.transform_type()
                                && candidate.direction() == direction
                                && candidate.converter_schema_name() == converter.schema_name()
                                && candidate.converter_function_name() == converter.function_name()
                        });
                    if !has_exact_converter_lifecycle {
                        return Err(invalid(
                            "index_exclusion_constraint_operator_procedure_transform_extension_membership_binding",
                        ));
                    }
                }
            }
        }

        observations.sort_by_key(transform_extension_membership_key);
        let expected = transform_converter_snapshot
            .observations()
            .iter()
            .flat_map(|observation| {
                observation.converters().iter().map(move |binding| {
                    transform_extension_membership_coordinate_key(
                        observation.coordinate(),
                        observation.key_position(),
                        binding.transform_type(),
                        observation.target_language_name(),
                    )
                })
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(transform_extension_membership_key)
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_extension_membership_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_extension_membership_completeness",
            ));
        }

        for observation in &observations {
            let has_converter_direction = converter_initial_privilege_snapshot
                .observations()
                .iter()
                .any(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                        && candidate.transform_type() == observation.transform_type()
                });
            if !has_converter_direction {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_extension_membership_binding",
                ));
            }
        }

        let snapshot_digest = compute_transform_extension_membership_digest(
            converter_initial_privilege_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: converter_initial_privilege_snapshot
                .source_connection_key()
                .to_owned(),
            connection_policy_binding: converter_initial_privilege_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
            extractor_revision: converter_initial_privilege_snapshot
                .extractor_revision()
                .to_owned(),
            observed_at_utc: converter_initial_privilege_snapshot
                .observed_at_utc()
                .to_owned(),
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

    /// Returns the domain-separated transform-object extension-membership successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
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

    /// Returns complete transform-object membership observations in deterministic row order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation] {
        &self.observations
    }

    /// Issues provenance for one exact `(constraint, key, type, language)` transform coordinate.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        target_language_name: &str,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSourceReceipt,
        ObservationError,
    > {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
                    && observation.transform_type() == &transform_type
                    && observation.target_language_name() == target_language_name
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_transform_extension_membership_location(
                    &coordinate,
                    key_position,
                    &transform_type,
                    target_language_name,
                ),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSourceReceipt {
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

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct TransformExtensionMembershipCoordinateKey {
    schema_name: String,
    relation_name: String,
    relation_kind: String,
    constraint_name: String,
    key_position: u32,
    transform_type_schema_name: String,
    transform_type_name: String,
    target_language_name: String,
}

fn transform_extension_membership_key(
    observation: &IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation,
) -> TransformExtensionMembershipCoordinateKey {
    transform_extension_membership_coordinate_key(
        observation.coordinate(),
        observation.key_position(),
        observation.transform_type(),
        observation.target_language_name(),
    )
}

fn transform_extension_membership_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    target_language_name: &str,
) -> TransformExtensionMembershipCoordinateKey {
    TransformExtensionMembershipCoordinateKey {
        schema_name: coordinate.schema_name().to_owned(),
        relation_name: coordinate.relation_name().to_owned(),
        relation_kind: coordinate.relation_kind().token().to_owned(),
        constraint_name: coordinate.constraint_name().to_owned(),
        key_position,
        transform_type_schema_name: transform_type.schema_name().to_owned(),
        transform_type_name: transform_type.type_name().to_owned(),
        target_language_name: target_language_name.to_owned(),
    }
}

fn compute_transform_extension_membership_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_EXTENSION_MEMBERSHIP_DIGEST_DOMAIN_V1,
    );
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_type(&mut hasher, observation.transform_type());
        encode_str(&mut hasher, observation.target_language_name());
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

fn procedure_transform_extension_membership_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    target_language_name: &str,
) -> String {
    let transform_schema = encode_location_component(transform_type.schema_name());
    let transform_name = encode_location_component(transform_type.type_name());
    let language_name = encode_location_component(target_language_name);
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transforms/{transform_schema}.{transform_name}/{language_name}/extension-membership",
        coordinate.canonical_location(),
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

fn encode_len(hasher: &mut Sha256, len: usize) {
    hasher.update((len as u64).to_be_bytes());
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_len(hasher, value.len());
    hasher.update(value.as_bytes());
}

fn validate_postgresql_identifier(
    value: &str,
    field: &'static str,
) -> Result<(), ObservationError> {
    if value.is_empty() || value.contains('\0') {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
