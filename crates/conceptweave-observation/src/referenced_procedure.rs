//! Qualified definitions and owner identities of procedures referenced by source expressions.

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, QualifiedTypeName, encode_bytes, encode_len, encode_sha256, encode_str,
};
use sha2::{Digest, Sha256};
use std::fmt;

/// Exact overloaded procedure coordinate; catalog procedure OIDs are not portable identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencedProcedureLocation {
    schema_name: String,
    procedure_name: String,
    input_types: Vec<QualifiedTypeName>,
}

impl ReferencedProcedureLocation {
    /// Creates a qualified signature with input argument types in their source order.
    pub fn new(
        schema_name: impl Into<String>,
        procedure_name: impl Into<String>,
        input_types: Vec<QualifiedTypeName>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let procedure_name = procedure_name.into();
        validate_postgresql_identifier(&schema_name, "referenced_procedure_schema")?;
        validate_postgresql_identifier(&procedure_name, "referenced_procedure_name")?;
        Ok(Self {
            schema_name,
            procedure_name,
            input_types,
        })
    }
    /// Returns the exact procedure schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }
    /// Returns the exact procedure name.
    #[must_use]
    pub fn procedure_name(&self) -> &str {
        &self.procedure_name
    }
    /// Returns the ordered input signature, including an empty list for no arguments.
    #[must_use]
    pub fn input_types(&self) -> &[QualifiedTypeName] {
        &self.input_types
    }
    /// Returns an RFC 6901 escaped coordinate with explicit arity and argument positions.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        let escape = |value: &str| value.replace('~', "~0").replace('/', "~1");
        let mut path = format!(
            "/referenced-procedures/{}/{}/input-types/{}",
            escape(&self.schema_name),
            escape(&self.procedure_name),
            self.input_types.len()
        );
        for (position, input) in self.input_types.iter().enumerate() {
            path.push_str(&format!(
                "/{position}/{}/{}",
                escape(input.schema_name()),
                escape(input.type_name())
            ));
        }
        path
    }
}

/// Same-generation reconstructed definition and exact owner of a referenced procedure.
///
/// Definition text is source evidence. It is never executed or promoted to business truth.
/// ACLs, security labels, extension lifecycle and initial privileges are separate material.
#[derive(Clone, Eq, PartialEq)]
pub struct ReferencedProcedureDefinitionObservation {
    location: ReferencedProcedureLocation,
    return_type: QualifiedTypeName,
    owner_oid: u32,
    owner_role_name: String,
    definition: String,
}

impl fmt::Debug for ReferencedProcedureDefinitionObservation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ReferencedProcedureDefinitionObservation")
            .field("location", &self.location)
            .field("return_type", &self.return_type)
            .field("owner_oid", &self.owner_oid)
            .field("owner_role_name", &self.owner_role_name)
            .field("definition", &"<redacted>")
            .finish()
    }
}

impl ReferencedProcedureDefinitionObservation {
    /// Records resolved types, role OID/name and unmodified reconstructed definition text.
    pub fn new(
        location: ReferencedProcedureLocation,
        return_type: QualifiedTypeName,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
        definition: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let owner_role_name = owner_role_name.into();
        let definition = definition.into();
        validate_postgresql_identifier(&owner_role_name, "referenced_procedure_owner_role")?;
        if owner_oid == 0 || definition.is_empty() || definition.contains('\0') {
            return Err(invalid("referenced_procedure_definition"));
        }
        Ok(Self {
            location,
            return_type,
            owner_oid,
            owner_role_name,
            definition,
        })
    }
    /// Returns the qualified overloaded procedure coordinate.
    #[must_use]
    pub const fn location(&self) -> &ReferencedProcedureLocation {
        &self.location
    }
    /// Returns the exact resolved result type.
    #[must_use]
    pub const fn return_type(&self) -> &QualifiedTypeName {
        &self.return_type
    }
    /// Returns the source-local role OID, retained separately from its readable name.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }
    /// Returns the exact resolved owner name.
    #[must_use]
    pub fn owner_role_name(&self) -> &str {
        &self.owner_role_name
    }
    /// Returns sensitive reconstructed source definition evidence; callers must not log it.
    #[must_use]
    pub fn definition(&self) -> &str {
        &self.definition
    }
}

/// Provenance for one exact captured procedure, bound to all observed families in the final digest.
///
/// The earlier definition-specific receipt name remains a compatible alias.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReferencedProcedureSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: ReferencedProcedureLocation,
}

impl ReferencedProcedureSourceReceipt {
    pub(crate) fn new(
        source_id: String,
        connection_policy_binding: String,
        source_digest: String,
        extractor_revision: String,
        observed_at_utc: String,
        location: ReferencedProcedureLocation,
    ) -> Self {
        Self {
            source_id,
            connection_policy_binding,
            source_digest,
            extractor_revision,
            observed_at_utc,
            location,
        }
    }
    /// Returns the authorized source registry reference.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }
    /// Returns the immutable authorized policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }
    /// Returns the procedure-aware final source digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }
    /// Returns the extractor revision supplied for this observation.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }
    /// Returns the source observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
    /// Returns the verified overloaded procedure coordinate.
    #[must_use]
    pub const fn location(&self) -> &ReferencedProcedureLocation {
        &self.location
    }
}

pub(crate) fn canonicalize(
    authorized_schemas: &[String],
    mut observations: Vec<ReferencedProcedureDefinitionObservation>,
) -> Result<Vec<ReferencedProcedureDefinitionObservation>, ObservationError> {
    observations.sort_by_cached_key(|item| item.location.canonical_location());
    if observations
        .windows(2)
        .any(|pair| pair[0].location == pair[1].location)
        || observations.iter().any(|item| {
            item.location.schema_name != "pg_catalog"
                && !authorized_schemas.contains(&item.location.schema_name)
        })
    {
        return Err(invalid("referenced_procedure_definition_coordinates"));
    }
    Ok(observations)
}

pub(crate) fn digest(
    base: &str,
    observations: &[ReferencedProcedureDefinitionObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        b"conceptweave.postgres_schema_snapshot.v3.referenced_procedure_definitions.v1",
    );
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, observations.len());
    for item in observations {
        encode_str(&mut hasher, item.location.schema_name());
        encode_str(&mut hasher, item.location.procedure_name());
        encode_len(&mut hasher, item.location.input_types.len());
        for input in &item.location.input_types {
            encode_type(&mut hasher, input);
        }
        encode_type(&mut hasher, &item.return_type);
        hasher.update(item.owner_oid.to_be_bytes());
        encode_str(&mut hasher, &item.owner_role_name);
        encode_str(&mut hasher, &item.definition);
    }
    encode_sha256(hasher)
}

fn encode_type(hasher: &mut Sha256, value: &QualifiedTypeName) {
    encode_str(hasher, value.schema_name());
    encode_str(hasher, value.type_name());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}

/// Compatible name for existing procedure-definition provenance consumers.
pub type ReferencedProcedureDefinitionSourceReceipt = ReferencedProcedureSourceReceipt;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signatures_preserve_exact_identifiers_and_definition_logs_are_redacted() {
        let input = QualifiedTypeName::new("type/schema", "kind~name").unwrap();
        let location = ReferencedProcedureLocation::new(" ", "function/~", vec![input]).unwrap();
        assert_eq!(
            location.canonical_location(),
            "/referenced-procedures/ /function~1~0/input-types/1/0/type~1schema/kind~0name"
        );
        let no_arguments = ReferencedProcedureLocation::new(" ", "function/~", vec![]).unwrap();
        assert_ne!(location, no_arguments);
        assert!(
            no_arguments
                .canonical_location()
                .ends_with("/input-types/0")
        );
        for (schema, name) in [("", "f"), ("s", ""), ("s\0", "f"), ("s", "f\0")] {
            assert!(ReferencedProcedureLocation::new(schema, name, vec![]).is_err());
        }
        let result = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
        let observation = ReferencedProcedureDefinitionObservation::new(
            location.clone(),
            result.clone(),
            42,
            " ",
            "sensitive source body",
        )
        .unwrap();
        assert!(!format!("{observation:?}").contains("sensitive source body"));
        assert_eq!(observation.definition(), "sensitive source body");
        for (oid, owner, definition) in [
            (0, "owner", "body"),
            (42, "", "body"),
            (42, "owner\0", "body"),
            (42, "owner", ""),
            (42, "owner", "body\0"),
        ] {
            assert!(
                ReferencedProcedureDefinitionObservation::new(
                    location.clone(),
                    result.clone(),
                    oid,
                    owner,
                    definition
                )
                .is_err()
            );
        }
        assert!(canonicalize(&[], vec![observation.clone()]).is_err());
        assert!(
            canonicalize(
                &[" ".to_owned()],
                vec![observation.clone(), observation.clone()]
            )
            .is_err()
        );
        assert_eq!(
            canonicalize(&[" ".to_owned()], vec![observation.clone()]).unwrap(),
            vec![observation]
        );
        assert!(canonicalize(&[], vec![]).unwrap().is_empty());
    }
}
