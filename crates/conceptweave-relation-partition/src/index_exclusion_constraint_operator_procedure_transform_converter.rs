//! PostgreSQL ordinary exclusion-constraint implementation-function transform-converter evidence.
//!
//! `pg_proc.protrftypes` proves which SQL types one target function elects to transform, but it does
//! not identify the mutable converter row. PostgreSQL stores that independently in `pg_transform`,
//! keyed by `(trftype, trflang)`, with optional `trffromsql` and `trftosql` function OIDs. A
//! `CREATE OR REPLACE TRANSFORM` can therefore change conversion code without changing the target
//! function signature or its selected `protrftypes` set.
//!
//! This observational successor binds every selected transform type to the exact same-generation
//! target language and `pg_transform` converter directions. Every nonzero converter OID is resolved
//! to its stable function coordinate, required `internal` call boundary, return type, implementation
//! language, and a domain-separated digest of exact `prosrc`/`probin`/`prosqlbody` material. Plaintext
//! converter bodies and binary paths are discarded after construction. Auxiliary converter `pg_proc`
//! properties remain a separately reviewable surface; this contract does not invent transform policy.

use std::collections::BTreeSet;

use conceptweave_observation::{ObservationError, QualifiedTypeName};
use sha2::{Digest, Sha256};

use super::{
    IndexExclusionConstraintCoordinate,
    IndexExclusionConstraintOperatorProcedureDefinitionSnapshot,
    IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot, QualifiedOperatorSignature,
    QualifiedProcedureSignature,
};

const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.v1";
const INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_FUNCTION_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.operator.procedure.transform_converter.function.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Privacy-preserving implementation material for one exact transform converter function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition {
    implementation_language_name: String,
    definition_digest: String,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition {
    /// Reduces exact converter implementation material to a domain-separated digest.
    pub fn new(
        implementation_language_name: impl Into<String>,
        prosrc: impl Into<String>,
        probin: Option<String>,
        prosqlbody: Option<String>,
    ) -> Result<Self, ObservationError> {
        let implementation_language_name = implementation_language_name.into();
        validate_nonblank(
            &implementation_language_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_function_language",
        )?;
        let prosrc = prosrc.into();
        let definition_digest = compute_converter_function_definition_digest(
            &implementation_language_name,
            &prosrc,
            probin.as_deref(),
            prosqlbody.as_deref(),
        );
        Ok(Self {
            implementation_language_name,
            definition_digest,
        })
    }

    /// Returns the independently resolved implementation language of the converter function itself.
    #[must_use]
    pub fn implementation_language_name(&self) -> &str {
        &self.implementation_language_name
    }

    /// Returns the content digest of exact converter implementation material.
    #[must_use]
    pub fn definition_digest(&self) -> &str {
        &self.definition_digest
    }
}

/// Content-bound identity for one nonzero `pg_transform` converter-function OID.
///
/// PostgreSQL requires transform conversion functions to accept exactly one `internal` argument.
/// Direction-specific return-type requirements are validated by
/// [`IndexExclusionConstraintOperatorProcedureTransformConverterBinding`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterFunction {
    schema_name: String,
    function_name: String,
    argument_type: QualifiedTypeName,
    return_type: QualifiedTypeName,
    definition: IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterFunction {
    /// Resolves one converter function without retaining plaintext implementation material.
    pub fn new(
        schema_name: impl Into<String>,
        function_name: impl Into<String>,
        argument_type: QualifiedTypeName,
        return_type: QualifiedTypeName,
        definition: IndexExclusionConstraintOperatorProcedureTransformConverterFunctionDefinition,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let function_name = function_name.into();
        validate_nonblank(
            &schema_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_function_schema",
        )?;
        validate_nonblank(
            &function_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_function_name",
        )?;
        if !is_pg_catalog_internal(&argument_type) {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_argument_type",
            ));
        }
        Ok(Self {
            schema_name,
            function_name,
            argument_type,
            return_type,
            definition,
        })
    }

    /// Returns the exact schema containing the converter function.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact converter-function name.
    #[must_use]
    pub fn function_name(&self) -> &str {
        &self.function_name
    }

    /// Returns the required single `pg_catalog.internal` argument type.
    #[must_use]
    pub const fn argument_type(&self) -> &QualifiedTypeName {
        &self.argument_type
    }

    /// Returns the exact independently resolved converter return type.
    #[must_use]
    pub const fn return_type(&self) -> &QualifiedTypeName {
        &self.return_type
    }

    /// Returns the independently resolved implementation language of the converter function itself.
    #[must_use]
    pub fn implementation_language_name(&self) -> &str {
        self.definition.implementation_language_name()
    }

    /// Returns the content digest of exact converter implementation material.
    #[must_use]
    pub fn definition_digest(&self) -> &str {
        self.definition.definition_digest()
    }
}

/// One exact same-generation `pg_transform` row resolved for a selected transform type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterBinding {
    transform_type: QualifiedTypeName,
    from_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
    to_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterBinding {
    /// Records the optional converter directions from one exact `(trftype, trflang)` row.
    pub fn new(
        transform_type: QualifiedTypeName,
        from_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
        to_sql: Option<IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
    ) -> Result<Self, ObservationError> {
        if from_sql.is_none() && to_sql.is_none() {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_direction",
            ));
        }
        if from_sql
            .as_ref()
            .is_some_and(|function| !is_pg_catalog_internal(function.return_type()))
        {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_from_sql_return_type",
            ));
        }
        if to_sql
            .as_ref()
            .is_some_and(|function| function.return_type() != &transform_type)
        {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_to_sql_return_type",
            ));
        }
        Ok(Self {
            transform_type,
            from_sql,
            to_sql,
        })
    }

    /// Returns the same-generation resolved `pg_transform.trftype` identity.
    #[must_use]
    pub const fn transform_type(&self) -> &QualifiedTypeName {
        &self.transform_type
    }

    /// Returns the exact nonzero `trffromsql` converter when PostgreSQL stores one.
    #[must_use]
    pub const fn from_sql(
        &self,
    ) -> Option<&IndexExclusionConstraintOperatorProcedureTransformConverterFunction> {
        self.from_sql.as_ref()
    }

    /// Returns the exact nonzero `trftosql` converter when PostgreSQL stores one.
    #[must_use]
    pub const fn to_sql(
        &self,
    ) -> Option<&IndexExclusionConstraintOperatorProcedureTransformConverterFunction> {
        self.to_sql.as_ref()
    }
}

/// Exact transform-converter evidence for one governed ordinary-EXCLUDE implementation function.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    key_position: u32,
    operator: QualifiedOperatorSignature,
    procedure: QualifiedProcedureSignature,
    target_language_name: String,
    converters: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterBinding>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterObservation {
    /// Records exact same-generation `pg_transform` rows for one target function's transform set.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
        operator: QualifiedOperatorSignature,
        procedure: QualifiedProcedureSignature,
        target_language_name: impl Into<String>,
        mut converters: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterBinding>,
    ) -> Result<Self, ObservationError> {
        if key_position == 0 {
            return Err(ObservationError::InvalidOrdinalPosition);
        }
        let target_language_name = target_language_name.into();
        validate_nonblank(
            &target_language_name,
            "index_exclusion_constraint_operator_procedure_transform_converter_target_language",
        )?;
        converters.sort_by(|left, right| {
            (
                left.transform_type().schema_name(),
                left.transform_type().type_name(),
            )
                .cmp(&(
                    right.transform_type().schema_name(),
                    right.transform_type().type_name(),
                ))
        });
        if converters
            .windows(2)
            .any(|pair| pair[0].transform_type() == pair[1].transform_type())
        {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_duplicate",
            ));
        }
        Ok(Self {
            coordinate,
            key_position,
            operator,
            procedure,
            target_language_name,
            converters,
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

    /// Returns the exact target `pg_operator.oprcode` function repeated for predecessor binding.
    #[must_use]
    pub const fn procedure(&self) -> &QualifiedProcedureSignature {
        &self.procedure
    }

    /// Returns the target function language resolved from the exact `pg_proc.prolang` row.
    #[must_use]
    pub fn target_language_name(&self) -> &str {
        &self.target_language_name
    }

    /// Returns exact selected transform rows in canonical type order.
    #[must_use]
    pub fn converters(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterBinding] {
        &self.converters
    }

    /// Returns the collision-safe evidence location for this transform-converter fact.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        procedure_transform_converter_location(&self.coordinate, self.key_position)
    }
}

/// Immutable provenance receipt for one exact transform-converter observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintOperatorProcedureTransformConverterObservation,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterSourceReceipt {
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

    /// Returns the owner-computed transform-converter successor digest.
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

    /// Returns the exact validated transform-converter observation.
    #[must_use]
    pub const fn location(
        &self,
    ) -> &IndexExclusionConstraintOperatorProcedureTransformConverterObservation {
        &self.location
    }
}

/// Complete exact `pg_transform` converter evidence over one transform-type predecessor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintOperatorProcedureTransformConverterObservation>,
}

impl IndexExclusionConstraintOperatorProcedureTransformConverterSnapshot {
    /// Creates complete converter evidence and cross-checks target language against definition evidence.
    pub fn new(
        transform_types_snapshot: &IndexExclusionConstraintOperatorProcedureTransformTypesSnapshot,
        definition_snapshot: &IndexExclusionConstraintOperatorProcedureDefinitionSnapshot,
        mut observations: Vec<
            IndexExclusionConstraintOperatorProcedureTransformConverterObservation,
        >,
    ) -> Result<Self, ObservationError> {
        if transform_types_snapshot.source_connection_key()
            != definition_snapshot.source_connection_key()
            || transform_types_snapshot.connection_policy_binding()
                != definition_snapshot.connection_policy_binding()
            || transform_types_snapshot.extractor_revision()
                != definition_snapshot.extractor_revision()
            || transform_types_snapshot.observed_at_utc() != definition_snapshot.observed_at_utc()
        {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_generation",
            ));
        }

        observations.sort_by(|left, right| {
            left.coordinate()
                .cmp(right.coordinate())
                .then_with(|| left.key_position().cmp(&right.key_position()))
        });
        let expected = transform_types_snapshot
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
                "index_exclusion_constraint_operator_procedure_transform_converter_coordinate",
            ));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_operator_procedure_transform_converter_completeness",
            ));
        }

        for observation in &observations {
            let predecessor = transform_types_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_converter_completeness",
                    )
                })?;
            if predecessor.operator() != observation.operator()
                || predecessor.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_binding",
                ));
            }

            let definition = definition_snapshot
                .observations()
                .iter()
                .find(|candidate| {
                    candidate.coordinate() == observation.coordinate()
                        && candidate.key_position() == observation.key_position()
                })
                .ok_or_else(|| {
                    invalid(
                        "index_exclusion_constraint_operator_procedure_transform_converter_generation",
                    )
                })?;
            if definition.operator() != observation.operator()
                || definition.procedure() != observation.procedure()
            {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_generation",
                ));
            }
            if definition.language_name() != observation.target_language_name() {
                return Err(invalid(
                    "index_exclusion_constraint_operator_procedure_transform_converter_language_binding",
                ));
            }

            match predecessor.transform_types() {
                None => {
                    if !observation.converters().is_empty() {
                        return Err(invalid(
                            "index_exclusion_constraint_operator_procedure_transform_converter_completeness",
                        ));
                    }
                }
                Some(expected_types) => {
                    if observation.converters().len() != expected_types.len()
                        || observation
                            .converters()
                            .iter()
                            .zip(expected_types.iter())
                            .any(|(binding, expected_type)| {
                                binding.transform_type() != expected_type
                            })
                    {
                        return Err(invalid(
                            "index_exclusion_constraint_operator_procedure_transform_converter_completeness",
                        ));
                    }
                }
            }
        }

        let snapshot_digest = compute_transform_converter_digest(
            transform_types_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: transform_types_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: transform_types_snapshot
                .connection_policy_binding()
                .to_owned(),
            snapshot_digest,
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

    /// Returns the domain-separated transform-converter successor digest.
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

    /// Returns complete transform-converter observations in deterministic constraint/key order.
    #[must_use]
    pub fn observations(
        &self,
    ) -> &[IndexExclusionConstraintOperatorProcedureTransformConverterObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed implementation-function transform-converter identity.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
        key_position: u32,
    ) -> Result<
        IndexExclusionConstraintOperatorProcedureTransformConverterSourceReceipt,
        ObservationError,
    > {
        let observation = self
            .observations
            .iter()
            .find(|observation| {
                observation.coordinate() == &coordinate
                    && observation.key_position() == key_position
            })
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: procedure_transform_converter_location(&coordinate, key_position),
            })?;
        Ok(
            IndexExclusionConstraintOperatorProcedureTransformConverterSourceReceipt {
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

fn compute_converter_function_definition_digest(
    implementation_language_name: &str,
    prosrc: &str,
    probin: Option<&str>,
    prosqlbody: Option<&str>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_FUNCTION_DIGEST_DOMAIN_V1,
    );
    encode_str(&mut hasher, implementation_language_name);
    encode_str(&mut hasher, prosrc);
    encode_optional_str(&mut hasher, probin);
    encode_optional_str(&mut hasher, prosqlbody);
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn compute_transform_converter_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintOperatorProcedureTransformConverterObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher
        .update(INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_TRANSFORM_CONVERTER_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        hasher.update(observation.key_position().to_be_bytes());
        encode_operator(&mut hasher, observation.operator());
        encode_procedure(&mut hasher, observation.procedure());
        encode_str(&mut hasher, observation.target_language_name());
        encode_len(&mut hasher, observation.converters().len());
        for binding in observation.converters() {
            encode_type(&mut hasher, binding.transform_type());
            encode_optional_converter_function(&mut hasher, binding.from_sql());
            encode_optional_converter_function(&mut hasher, binding.to_sql());
        }
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_optional_converter_function(
    hasher: &mut Sha256,
    function: Option<&IndexExclusionConstraintOperatorProcedureTransformConverterFunction>,
) {
    match function {
        Some(function) => {
            hasher.update([1_u8]);
            encode_str(hasher, function.schema_name());
            encode_str(hasher, function.function_name());
            encode_type(hasher, function.argument_type());
            encode_type(hasher, function.return_type());
            encode_str(hasher, function.implementation_language_name());
            encode_str(hasher, function.definition_digest());
        }
        None => hasher.update([0_u8]),
    }
}

fn procedure_transform_converter_location(
    coordinate: &IndexExclusionConstraintCoordinate,
    key_position: u32,
) -> String {
    format!(
        "{}/exclusion-operators/{key_position}/procedure-transform-converters",
        coordinate.canonical_location()
    )
}

fn is_pg_catalog_internal(qualified_type: &QualifiedTypeName) -> bool {
    qualified_type.schema_name() == "pg_catalog" && qualified_type.type_name() == "internal"
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

fn encode_type(hasher: &mut Sha256, qualified_type: &QualifiedTypeName) {
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

fn validate_nonblank(value: &str, field: &'static str) -> Result<(), ObservationError> {
    if value.trim().is_empty() {
        return Err(invalid(field));
    }
    Ok(())
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
