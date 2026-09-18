//! PostgreSQL ordinary exclusion-constraint transform-converter access-control evidence.
//!
//! The converter-owner predecessor binds each nonzero `pg_transform` converter to its exact
//! definition and owner, but `pg_proc.proacl` remains independently mutable. This successor binds
//! every such converter direction to the exact raw ACL null-state plus a canonical object-level
//! `EXECUTE` grant set resolved in the same source generation. Role-membership closure and product
//! authorization policy remain outside this source-identity boundary.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_MATERIAL_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.access_control.material.v1";
const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.access_control.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum TransformConverterExecuteGrantee {
    Public,
    Role(String),
}

/// One object-level PostgreSQL converter-function `EXECUTE` grant resolved in the same generation.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant {
    grantee: TransformConverterExecuteGrantee,
    grantor_role_name: String,
    grant_option: bool,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant {
    /// Records an `EXECUTE` grant to `PUBLIC` with its exact resolved grantor role.
    pub fn public(
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_access_control_grantor_role_name",
        )?;
        Ok(Self {
            grantee: TransformConverterExecuteGrantee::Public,
            grantor_role_name,
            grant_option,
        })
    }

    /// Records an `EXECUTE` grant to one resolved role with its exact resolved grantor role.
    pub fn role(
        grantee_role_name: impl Into<String>,
        grantor_role_name: impl Into<String>,
        grant_option: bool,
    ) -> Result<Self, ObservationError> {
        let grantee_role_name = grantee_role_name.into();
        let grantor_role_name = grantor_role_name.into();
        validate_nonblank(
            &grantee_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_access_control_grantee_role_name",
        )?;
        validate_nonblank(
            &grantor_role_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_access_control_grantor_role_name",
        )?;
        Ok(Self {
            grantee: TransformConverterExecuteGrantee::Role(grantee_role_name),
            grantor_role_name,
            grant_option,
        })
    }
}

/// Privacy-preserving identity for one converter `pg_proc.proacl` state and EXECUTE grant set.
///
/// `proacl IS NULL` remains distinct from an explicit ACL even when both resolve to the same
/// object-level grants. The adapter must apply PostgreSQL function ACL semantics before this
/// boundary and resolve all non-PUBLIC grantor/grantee OIDs in the same source generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial {
    proacl_was_null: bool,
    grant_count: usize,
    digest: String,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial {
    /// Reduces raw ACL null-state and canonical object-level EXECUTE grants to a stable digest.
    pub fn new(
        proacl_was_null: bool,
        mut grants: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterExecuteGrant>,
    ) -> Result<Self, ObservationError> {
        grants.sort();
        if grants.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_access_control_grant",
            ));
        }

        let mut hasher = Sha256::new();
        hasher.update(
            INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_MATERIAL_DIGEST_DOMAIN_V1,
        );
        hasher.update([u8::from(proacl_was_null)]);
        encode_len(&mut hasher, grants.len());
        for grant in &grants {
            match &grant.grantee {
                TransformConverterExecuteGrantee::Public => hasher.update([0]),
                TransformConverterExecuteGrantee::Role(role_name) => {
                    hasher.update([1]);
                    encode_str(&mut hasher, role_name);
                }
            }
            encode_str(&mut hasher, &grant.grantor_role_name);
            hasher.update([u8::from(grant.grant_option)]);
        }

        Ok(Self {
            proacl_was_null,
            grant_count: grants.len(),
            digest: format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize()),
        })
    }

    /// Returns whether the exact converter `pg_proc.proacl` value was null.
    #[must_use]
    pub const fn proacl_was_null(&self) -> bool {
        self.proacl_was_null
    }

    /// Returns the number of canonical object-level EXECUTE grants consumed into the digest.
    #[must_use]
    pub const fn grant_count(&self) -> usize {
        self.grant_count
    }

    /// Returns the privacy-preserving digest for this exact converter ACL state.
    #[must_use]
    pub fn digest(&self) -> &str {
        &self.digest
    }
}

/// Exact `pg_proc.proacl` evidence for one nonzero transform converter direction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    converter_schema_name: String,
    converter_function_name: String,
    access_control: IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation {
    /// Records independently observed ACL identity for one exact converter direction.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
        converter_schema_name: impl Into<String>,
        converter_function_name: impl Into<String>,
        access_control: IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let converter_schema_name = converter_schema_name.into();
        let converter_function_name = converter_function_name.into();
        validate_nonblank(
            &converter_schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_access_control_function_schema",
        )?;
        validate_nonblank(
            &converter_function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_access_control_function_name",
        )?;
        Ok(Self {
            coordinate,
            key_position,
            transform_type,
            direction,
            converter_schema_name,
            converter_function_name,
            access_control,
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

    /// Returns the exact converter-function schema.
    #[must_use]
    pub fn converter_schema_name(&self) -> &str {
        &self.converter_schema_name
    }

    /// Returns the exact converter-function name.
    #[must_use]
    pub fn converter_function_name(&self) -> &str {
        &self.converter_function_name
    }

    /// Returns the privacy-preserving converter access-control material.
    #[must_use]
    pub const fn access_control(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlMaterial {
        &self.access_control
    }

    /// Returns the collision-safe evidence location for this converter ACL fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_access_control_location(
            &self.coordinate,
            self.key_position,
            &self.transform_type,
            self.direction,
        )
    }
}

/// Immutable provenance receipt for one exact converter access-control observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSourceReceipt {
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

    /// Returns the owner-computed converter access-control successor digest.
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

    /// Returns the exact validated converter access-control observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation {
        &self.location
    }
}

/// Complete converter-function access-control evidence over one exact converter-owner predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation,
    >,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSnapshot {
    /// Creates complete ACL evidence for every converter direction present in the owner predecessor.
    pub fn new(
        owner_snapshot: &IndexExclusionConstraintOperatorProcedureTransformConverterOwnerSnapshot,
        mut observations: Vec<
            IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation,
        >,
    ) -> Result<Self, ObservationError> {
        observations.sort_by_key(access_control_key);

        let expected = owner_snapshot
            .observations()
            .iter()
            .map(|observation| {
                access_control_coordinate_key(
                    observation.coordinate(),
                    observation.key_position(),
                    observation.transform_type(),
                    observation.direction(),
                )
            })
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(access_control_key)
            .collect::<BTreeSet<_>>();
        if observed.len() != observations.len() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_access_control_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_access_control_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = owner_snapshot
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
                        "index_exclusion_constraint_operator_procedure_transform_converter_access_control_completeness",
                    )
                })?;
            if predecessor.converter_schema_name() != observation.converter_schema_name()
                || predecessor.converter_function_name() != observation.converter_function_name()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_access_control_binding",
                ));
            }
        }

        let snapshot_digest = compute_transform_converter_access_control_digest(
            owner_snapshot.snapshot_digest(),
            &observations,
        );
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

    /// Returns the domain-separated converter access-control successor digest.
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

    /// Returns complete access-control observations in deterministic converter order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed converter ACL identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        transform_type: QualifiedTypeName,
        direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSourceReceipt,
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
                location: procedure_transform_converter_access_control_location(
                    &coordinate,
                    key_position,
                    &transform_type,
                    direction,
                ),
            })?;
        Ok(IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlSourceReceipt {
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
struct ConverterAccessControlCoordinateKey {
    schema_name: String,
    relation_name: String,
    relation_kind: String,
    constraint_name: String,
    key_position: u32,
    transform_type_schema_name: String,
    transform_type_name: String,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
}

fn compute_transform_converter_access_control_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_ACCESS_CONTROL_DIGEST_DOMAIN_V1,
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
        encode_str(&mut hasher, observation.access_control().digest());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn access_control_key(
    observation: &IndexExclusionConstraintOperatorProcedureTransformConverterAccessControlObservation,
) -> ConverterAccessControlCoordinateKey {
    access_control_coordinate_key(
        observation.coordinate(),
        observation.key_position(),
        observation.transform_type(),
        observation.direction(),
    )
}

fn access_control_coordinate_key(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> ConverterAccessControlCoordinateKey {
    ConverterAccessControlCoordinateKey {
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

fn procedure_transform_converter_access_control_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
    transform_type: &QualifiedTypeName,
    direction: IndexExclusionConstraintOperatorProcedureTransformConverterDirection,
) -> String {
    let transform_schema = encode_location_component(transform_type.schema_name());
    let transform_name = encode_location_component(transform_type.type_name());
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-converters/{transform_schema}.{transform_name}/{}/access-control",
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
