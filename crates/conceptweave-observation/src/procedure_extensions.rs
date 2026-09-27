//! Installed extension evidence is separate from semantic authority and runtime package proof.

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, ReferencedProcedureDefinitionObservation, ReferencedProcedureLocation,
    RelationKind, SchemaObjectLocation, encode_bytes, encode_len, encode_sha256, encode_str,
};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fmt};

/// One original procedure dependency on an installed extension, retaining its native dependency code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcedureExtensionDependency {
    dependency_type: char,
    extension_name: String,
}
impl ProcedureExtensionDependency {
    /// Records a PostgreSQL 18 dependency code and an unqualified database-wide extension name.
    pub fn new(
        dependency_type: char,
        extension_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let extension_name = extension_name.into();
        validate_postgresql_identifier(&extension_name, "procedure_extension_name")?;
        if !matches!(dependency_type, 'n' | 'a' | 'i' | 'P' | 'S' | 'e' | 'x') {
            return Err(invalid());
        }
        Ok(Self {
            dependency_type,
            extension_name,
        })
    }
    /// Returns the native dependency code; e means membership and x means automatic extension dependency.
    #[must_use]
    pub const fn dependency_type(&self) -> char {
        self.dependency_type
    }
    /// Returns the database-wide extension name, which is never schema-qualified.
    #[must_use]
    pub fn extension_name(&self) -> &str {
        &self.extension_name
    }
}

/// Complete extension dependencies for one overloaded procedure; an empty list means observed absence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcedureExtensionDependenciesObservation {
    location: ReferencedProcedureLocation,
    dependencies: Vec<ProcedureExtensionDependency>,
}
impl ProcedureExtensionDependenciesObservation {
    /// Records original dependency multiplicity in canonical order and rejects multiple membership edges.
    pub fn new(
        location: ReferencedProcedureLocation,
        mut dependencies: Vec<ProcedureExtensionDependency>,
    ) -> Result<Self, ObservationError> {
        dependencies.sort_by(|a, b| {
            (&a.extension_name, a.dependency_type).cmp(&(&b.extension_name, b.dependency_type))
        });
        if dependencies
            .iter()
            .filter(|item| item.dependency_type == 'e')
            .count()
            > 1
        {
            return Err(invalid());
        }
        Ok(Self {
            location,
            dependencies,
        })
    }
    /// Returns the exact overloaded procedure coordinate.
    #[must_use]
    pub const fn location(&self) -> &ReferencedProcedureLocation {
        &self.location
    }
    /// Returns all dependency edges without removing duplicates.
    #[must_use]
    pub fn dependencies(&self) -> &[ProcedureExtensionDependency] {
        &self.dependencies
    }
}

/// One extension configuration table coordinate and unchanged dump filter condition.
#[derive(Clone, Eq, PartialEq)]
pub struct ExtensionConfigurationTable {
    location: SchemaObjectLocation,
    condition: String,
}
impl fmt::Debug for ExtensionConfigurationTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtensionConfigurationTable")
            .field("location", &self.location)
            .field("condition", &"<redacted>")
            .finish()
    }
}
impl ExtensionConfigurationTable {
    /// Records a resolved ordinary or partitioned table and opaque filter text, without executing it.
    pub fn new(
        schema: impl Into<String>,
        table: impl Into<String>,
        kind: RelationKind,
        condition: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let condition = condition.into();
        if !matches!(kind, RelationKind::Table | RelationKind::PartitionedTable)
            || condition.contains('\0')
        {
            return Err(invalid());
        }
        let location = SchemaObjectLocation::relation(schema, table, kind)?;
        Ok(Self {
            location,
            condition,
        })
    }
    /// Returns the exact resolved configuration table coordinate; no table rows were observed.
    #[must_use]
    pub const fn location(&self) -> &SchemaObjectLocation {
        &self.location
    }
    /// Returns the original filter condition, whose SQL is never run by the observation adapter.
    #[must_use]
    pub fn condition(&self) -> &str {
        &self.condition
    }
}

/// Complete installed-extension catalog definition with resolved configuration table identities.
#[derive(Clone, Eq, PartialEq)]
pub struct SourceExtensionDefinition {
    name: String,
    owner_oid: u32,
    owner_name: Option<String>,
    object_schema: String,
    relocatable: bool,
    version: String,
    configuration: Option<Vec<ExtensionConfigurationTable>>,
}
impl fmt::Debug for SourceExtensionDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceExtensionDefinition")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}
impl SourceExtensionDefinition {
    /// Records full native metadata. The exported-object schema does not qualify the extension name.
    pub fn new(
        name: impl Into<String>,
        owner_oid: u32,
        owner_name: Option<String>,
        object_schema: impl Into<String>,
        relocatable: bool,
        version: impl Into<String>,
        configuration: Option<Vec<ExtensionConfigurationTable>>,
    ) -> Result<Self, ObservationError> {
        let name = name.into();
        let object_schema = object_schema.into();
        let version = version.into();
        validate_postgresql_identifier(&name, "source_extension_name")?;
        validate_postgresql_identifier(&object_schema, "source_extension_object_schema")?;
        if let Some(owner) = &owner_name {
            validate_postgresql_identifier(owner, "source_extension_owner_name")?;
        }
        if owner_oid == 0 || version.contains('\0') {
            return Err(invalid());
        }
        Ok(Self {
            name,
            owner_oid,
            owner_name,
            object_schema,
            relocatable,
            version,
            configuration,
        })
    }
    /// Returns the database-wide extension name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the exact source owner role OID.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }
    /// Returns optional name enrichment for the source owner role identity.
    #[must_use]
    pub fn owner_name(&self) -> Option<&str> {
        self.owner_name.as_deref()
    }
    /// Returns the schema containing exported objects, not an extension namespace qualifier.
    #[must_use]
    pub fn object_schema(&self) -> &str {
        &self.object_schema
    }
    /// Returns the source relocation flag without inferring additional permissions.
    #[must_use]
    pub const fn relocatable(&self) -> bool {
        self.relocatable
    }
    /// Returns the exact installed version text, not runtime binary verification.
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }
    /// Returns configuration entries in original order; None preserves catalog NULL, distinct from empty.
    #[must_use]
    pub fn configuration(&self) -> Option<&[ExtensionConfigurationTable]> {
        self.configuration.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProcedureExtensionEvidence {
    pub dependencies: Vec<ProcedureExtensionDependenciesObservation>,
    pub extensions: Vec<SourceExtensionDefinition>,
}
pub(crate) fn canonicalize(
    definitions: &[ReferencedProcedureDefinitionObservation],
    mut dependencies: Vec<ProcedureExtensionDependenciesObservation>,
    mut extensions: Vec<SourceExtensionDefinition>,
) -> Result<ProcedureExtensionEvidence, ObservationError> {
    dependencies.sort_by_cached_key(|item| item.location.canonical_location());
    if dependencies.len() != definitions.len()
        || dependencies
            .iter()
            .zip(definitions)
            .any(|(item, definition)| item.location() != definition.location())
    {
        return Err(invalid());
    }
    extensions.sort_by(|a, b| a.name.cmp(&b.name));
    let referenced: BTreeSet<&str> = dependencies
        .iter()
        .flat_map(|item| {
            item.dependencies
                .iter()
                .map(|edge| edge.extension_name.as_str())
        })
        .collect();
    if referenced.len() != extensions.len()
        || referenced
            .into_iter()
            .zip(&extensions)
            .any(|(name, definition)| name != definition.name)
    {
        return Err(invalid());
    }
    Ok(ProcedureExtensionEvidence {
        dependencies,
        extensions,
    })
}
pub(crate) fn digest(base: &str, evidence: &ProcedureExtensionEvidence) -> String {
    let mut hash = Sha256::new();
    encode_bytes(
        &mut hash,
        b"conceptweave.postgres_schema_snapshot.v3.procedure_extensions.v1",
    );
    encode_str(&mut hash, base);
    encode_len(&mut hash, evidence.dependencies.len());
    for item in &evidence.dependencies {
        encode_str(&mut hash, &item.location.canonical_location());
        encode_len(&mut hash, item.dependencies.len());
        for edge in &item.dependencies {
            hash.update([edge.dependency_type as u8]);
            encode_str(&mut hash, &edge.extension_name);
        }
    }
    encode_len(&mut hash, evidence.extensions.len());
    for extension in &evidence.extensions {
        encode_str(&mut hash, &extension.name);
        hash.update(extension.owner_oid.to_be_bytes());
        hash.update([u8::from(extension.owner_name.is_some())]);
        if let Some(owner) = &extension.owner_name {
            encode_str(&mut hash, owner);
        }
        encode_str(&mut hash, &extension.object_schema);
        hash.update([u8::from(extension.relocatable)]);
        encode_str(&mut hash, &extension.version);
        hash.update([u8::from(extension.configuration.is_some())]);
        if let Some(config) = &extension.configuration {
            encode_len(&mut hash, config.len());
            for table in config {
                encode_str(&mut hash, &table.location.canonical_location());
                encode_str(&mut hash, &table.condition);
            }
        }
    }
    encode_sha256(hash)
}
fn invalid() -> ObservationError {
    ObservationError::InvalidObservationField {
        field: "procedure_extensions",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extension_evidence_keeps_configuration_and_requires_exact_closure() {
        let location = ReferencedProcedureLocation::new("pg_catalog", "f", vec![]).unwrap();
        for kind in ['n', 'a', 'i', 'P', 'S', 'e', 'x'] {
            let edge = ProcedureExtensionDependency::new(kind, " ").unwrap();
            assert_eq!(edge.dependency_type(), kind);
            assert_eq!(edge.extension_name(), " ");
        }
        assert!(ProcedureExtensionDependency::new('z', "extension").is_err());
        assert!(ProcedureExtensionDependency::new('e', "").is_err());
        let member = ProcedureExtensionDependency::new('e', "extension").unwrap();
        let automatic = ProcedureExtensionDependency::new('x', "extension").unwrap();
        assert!(
            ProcedureExtensionDependenciesObservation::new(
                location.clone(),
                vec![member.clone(), member.clone()]
            )
            .is_err()
        );
        let dependencies = ProcedureExtensionDependenciesObservation::new(
            location.clone(),
            vec![automatic.clone(), member, automatic],
        )
        .unwrap();
        assert_eq!(dependencies.location(), &location);
        assert_eq!(dependencies.dependencies().len(), 3);
        let config = ExtensionConfigurationTable::new(
            " ",
            "table/~",
            RelationKind::Table,
            " private condition ",
        )
        .unwrap();
        assert_eq!(config.condition(), " private condition ");
        assert_eq!(
            config.location(),
            &SchemaObjectLocation::relation(" ", "table/~", RelationKind::Table).unwrap()
        );
        assert!(!format!("{config:?}").contains("private condition"));
        assert!(ExtensionConfigurationTable::new("s", "t", RelationKind::View, "").is_err());
        assert!(ExtensionConfigurationTable::new("s", "t", RelationKind::Table, "bad\0").is_err());
        let extension = SourceExtensionDefinition::new(
            "extension",
            42,
            None,
            " ",
            false,
            "",
            Some(vec![config.clone(), config]),
        )
        .unwrap();
        assert_eq!(extension.name(), "extension");
        assert_eq!(extension.owner_oid(), 42);
        assert_eq!(extension.owner_name(), None);
        assert_eq!(extension.object_schema(), " ");
        assert!(!extension.relocatable());
        assert_eq!(extension.version(), "");
        assert_eq!(extension.configuration().unwrap().len(), 2);
        assert!(!format!("{extension:?}").contains("private condition"));
        assert!(
            SourceExtensionDefinition::new("extension", 0, None, "s", false, "v", None).is_err()
        );
        assert!(
            SourceExtensionDefinition::new(
                "extension",
                42,
                Some("bad\0".into()),
                "s",
                false,
                "v",
                None
            )
            .is_err()
        );
        assert!(
            SourceExtensionDefinition::new("extension", 42, None, "s", false, "v\0", None).is_err()
        );
        let definition = ReferencedProcedureDefinitionObservation::new(
            location,
            crate::QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            42,
            "owner",
            "definition",
        )
        .unwrap();
        let definitions = [definition];
        assert!(canonicalize(&definitions, vec![], vec![]).is_err());
        assert!(
            canonicalize(
                &definitions,
                vec![dependencies.clone(), dependencies.clone()],
                vec![extension.clone()]
            )
            .is_err()
        );
        let wrong = ProcedureExtensionDependenciesObservation::new(
            ReferencedProcedureLocation::new("pg_catalog", "g", vec![]).unwrap(),
            vec![],
        )
        .unwrap();
        assert!(canonicalize(&definitions, vec![wrong], vec![]).is_err());
        assert!(canonicalize(&definitions, vec![dependencies.clone()], vec![]).is_err());
        assert!(
            canonicalize(
                &definitions,
                vec![dependencies.clone()],
                vec![extension.clone(), extension.clone()]
            )
            .is_err()
        );
        let empty = ProcedureExtensionDependenciesObservation::new(
            definitions[0].location().clone(),
            vec![],
        )
        .unwrap();
        assert!(canonicalize(&definitions, vec![empty], vec![extension.clone()]).is_err());
        let original = canonicalize(
            &definitions,
            vec![dependencies.clone()],
            vec![extension.clone()],
        )
        .unwrap();
        let mut no_config = extension.clone();
        no_config.configuration = None;
        let absent = canonicalize(
            &definitions,
            vec![dependencies.clone()],
            vec![no_config.clone()],
        )
        .unwrap();
        no_config.configuration = Some(vec![]);
        let explicit_empty =
            canonicalize(&definitions, vec![dependencies.clone()], vec![no_config]).unwrap();
        assert_ne!(digest("base", &original), digest("base", &absent));
        assert_ne!(digest("base", &absent), digest("base", &explicit_empty));
        for changed in [
            SourceExtensionDefinition::new(
                "extension",
                43,
                None,
                " ",
                false,
                "",
                extension.configuration.clone(),
            )
            .unwrap(),
            SourceExtensionDefinition::new(
                "extension",
                42,
                Some("owner".into()),
                " ",
                false,
                "",
                extension.configuration.clone(),
            )
            .unwrap(),
            SourceExtensionDefinition::new(
                "extension",
                42,
                None,
                "schema",
                false,
                "",
                extension.configuration.clone(),
            )
            .unwrap(),
            SourceExtensionDefinition::new(
                "extension",
                42,
                None,
                " ",
                true,
                "",
                extension.configuration.clone(),
            )
            .unwrap(),
            SourceExtensionDefinition::new(
                "extension",
                42,
                None,
                " ",
                false,
                "version",
                extension.configuration.clone(),
            )
            .unwrap(),
        ] {
            assert_ne!(
                digest("base", &original),
                digest(
                    "base",
                    &canonicalize(&definitions, vec![dependencies.clone()], vec![changed]).unwrap()
                )
            );
        }
        assert!(
            canonicalize(&[], vec![], vec![])
                .unwrap()
                .extensions
                .is_empty()
        );
    }
}
