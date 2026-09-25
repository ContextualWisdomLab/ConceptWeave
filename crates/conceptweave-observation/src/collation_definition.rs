use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::{
    ColumnCollationObservation, DomainObservation, ObservationError, QualifiedCollationName,
    RelationObservation,
};

/// PostgreSQL 18 collation provider recorded by `pg_collation` or `pg_database`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CollationProvider {
    /// Database default collation object.
    DatabaseDefault,
    /// PostgreSQL built-in collation.
    Builtin,
    /// Operating-system libc collation.
    Libc,
    /// ICU collation.
    Icu,
}

impl CollationProvider {
    /// Returns the exact PostgreSQL catalog token.
    #[must_use]
    pub const fn token(self) -> u8 {
        match self {
            Self::DatabaseDefault => b'd',
            Self::Builtin => b'b',
            Self::Libc => b'c',
            Self::Icu => b'i',
        }
    }
}

impl TryFrom<&str> for CollationProvider {
    type Error = ObservationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "d" => Ok(Self::DatabaseDefault),
            "b" => Ok(Self::Builtin),
            "c" => Ok(Self::Libc),
            "i" => Ok(Self::Icu),
            _ => Err(ObservationError::InvalidObservationField {
                field: "collation_provider",
            }),
        }
    }
}

/// Exact nullable locale and version fields recorded by PostgreSQL 18.
/// Recorded and actual versions remain separate because the provider may change before refresh.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollationLocaleFields {
    lc_collate: Option<String>,
    lc_ctype: Option<String>,
    locale: Option<String>,
    icu_rules: Option<String>,
    recorded_version: Option<String>,
    actual_version: Option<String>,
}

impl CollationLocaleFields {
    /// Preserves catalog NULL separately from present text for every locale field.
    pub fn new(
        lc_collate: Option<String>,
        lc_ctype: Option<String>,
        locale: Option<String>,
        icu_rules: Option<String>,
        recorded_version: Option<String>,
        actual_version: Option<String>,
    ) -> Result<Self, ObservationError> {
        for value in [
            &lc_collate,
            &lc_ctype,
            &locale,
            &icu_rules,
            &recorded_version,
            &actual_version,
        ]
        .into_iter()
        .flatten()
        {
            if value.contains('\0') {
                return Err(ObservationError::InvalidObservationField {
                    field: "collation_locale_text",
                });
            }
        }
        Ok(Self {
            lc_collate,
            lc_ctype,
            locale,
            icu_rules,
            recorded_version,
            actual_version,
        })
    }

    /// Returns `LC_COLLATE` when present.
    #[must_use]
    pub fn lc_collate(&self) -> Option<&str> {
        self.lc_collate.as_deref()
    }
    /// Returns `LC_CTYPE` when present.
    #[must_use]
    pub fn lc_ctype(&self) -> Option<&str> {
        self.lc_ctype.as_deref()
    }
    /// Returns the provider locale when present.
    #[must_use]
    pub fn locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }
    /// Returns ICU tailoring rules when present.
    #[must_use]
    pub fn icu_rules(&self) -> Option<&str> {
        self.icu_rules.as_deref()
    }
    /// Returns the version recorded in the catalog when present.
    #[must_use]
    pub fn recorded_version(&self) -> Option<&str> {
        self.recorded_version.as_deref()
    }
    /// Returns the capture-time provider version when present.
    #[must_use]
    pub fn actual_version(&self) -> Option<&str> {
        self.actual_version.as_deref()
    }
}

/// Effective database locale behind PostgreSQL's `pg_catalog.default` collation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DatabaseLocaleDefinition {
    provider: CollationProvider,
    fields: CollationLocaleFields,
}

impl DatabaseLocaleDefinition {
    /// Creates exact default-database locale evidence.
    pub fn new(
        provider: CollationProvider,
        fields: CollationLocaleFields,
    ) -> Result<Self, ObservationError> {
        let provider_shape = match provider {
            CollationProvider::DatabaseDefault => false,
            CollationProvider::Libc => {
                fields.lc_collate().is_some()
                    && fields.lc_ctype().is_some()
                    && fields.locale().is_none()
                    && fields.icu_rules().is_none()
            }
            CollationProvider::Builtin => fields.locale().is_some() && fields.icu_rules().is_none(),
            CollationProvider::Icu => fields.locale().is_some(),
        };
        if !provider_shape {
            return Err(ObservationError::InvalidObservationField {
                field: "database_locale_provider",
            });
        }
        Ok(Self { provider, fields })
    }

    /// Returns the database's effective locale provider.
    #[must_use]
    pub const fn provider(&self) -> CollationProvider {
        self.provider
    }
    /// Returns the database's exact locale and version fields.
    #[must_use]
    pub const fn fields(&self) -> &CollationLocaleFields {
        &self.fields
    }
}

/// Complete comparison definition of one collation referenced by observed schema evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollationDefinitionObservation {
    collation: QualifiedCollationName,
    encoding: i32,
    database_encoding: i32,
    provider: CollationProvider,
    deterministic: bool,
    fields: CollationLocaleFields,
    database_default: Option<DatabaseLocaleDefinition>,
    comment: Option<String>,
}

impl CollationDefinitionObservation {
    /// Creates one catalog definition, including the database locale for `pg_catalog.default`.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        collation: QualifiedCollationName,
        encoding: i32,
        database_encoding: i32,
        provider: CollationProvider,
        deterministic: bool,
        fields: CollationLocaleFields,
        database_default: Option<DatabaseLocaleDefinition>,
        comment: Option<String>,
    ) -> Result<Self, ObservationError> {
        let is_default =
            collation.schema_name() == "pg_catalog" && collation.collation_name() == "default";
        let provider_shape = match provider {
            CollationProvider::DatabaseDefault => {
                encoding == -1
                    && deterministic
                    && fields.lc_collate().is_none()
                    && fields.lc_ctype().is_none()
                    && fields.locale().is_none()
                    && fields.icu_rules().is_none()
                    && fields.recorded_version().is_none()
            }
            CollationProvider::Builtin => {
                deterministic
                    && fields.lc_collate().is_none()
                    && fields.lc_ctype().is_none()
                    && fields.icu_rules().is_none()
                    && matches!(fields.locale(), Some("C" | "C.UTF-8" | "PG_UNICODE_FAST"))
                    && fields.recorded_version().is_some()
            }
            CollationProvider::Libc => {
                deterministic
                    && fields.lc_collate().is_some()
                    && fields.lc_ctype().is_some()
                    && fields.locale().is_none()
                    && fields.icu_rules().is_none()
            }
            CollationProvider::Icu => {
                encoding == -1
                    && fields.lc_collate().is_none()
                    && fields.lc_ctype().is_none()
                    && fields.locale().is_some()
                    && fields.recorded_version().is_some()
                    && fields.actual_version().is_some()
            }
        };
        if encoding < -1
            || database_encoding < 0
            || (encoding != -1 && encoding != database_encoding)
            || (provider == CollationProvider::DatabaseDefault) != is_default
            || database_default.is_some() != is_default
            || comment.as_ref().is_some_and(|value| value.contains('\0'))
            || !provider_shape
        {
            return Err(ObservationError::InvalidObservationField {
                field: "collation_definition_shape",
            });
        }
        Ok(Self {
            collation,
            encoding,
            database_encoding,
            provider,
            deterministic,
            fields,
            database_default,
            comment,
        })
    }

    /// Returns the exact qualified collation coordinate.
    #[must_use]
    pub const fn collation(&self) -> &QualifiedCollationName {
        &self.collation
    }
    /// Returns raw `pg_collation.collencoding`.
    #[must_use]
    pub const fn encoding(&self) -> i32 {
        self.encoding
    }
    /// Returns the current database encoding that selected this catalog row.
    #[must_use]
    pub const fn database_encoding(&self) -> i32 {
        self.database_encoding
    }
    /// Returns the collation catalog provider.
    #[must_use]
    pub const fn provider(&self) -> CollationProvider {
        self.provider
    }
    /// Returns `pg_collation.collisdeterministic`.
    #[must_use]
    pub const fn deterministic(&self) -> bool {
        self.deterministic
    }
    /// Returns the exact catalog locale and version fields.
    #[must_use]
    pub const fn fields(&self) -> &CollationLocaleFields {
        &self.fields
    }
    /// Returns effective database locale evidence for `pg_catalog.default` only.
    #[must_use]
    pub const fn database_default(&self) -> Option<&DatabaseLocaleDefinition> {
        self.database_default.as_ref()
    }
    /// Returns the exact catalog comment when present.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }
}

pub(crate) fn canonicalize(
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    column_collations: Option<&[ColumnCollationObservation]>,
    range_catalog: Option<&[crate::RangeCatalogObservation]>,
    mut definitions: Vec<CollationDefinitionObservation>,
) -> Result<Vec<CollationDefinitionObservation>, ObservationError> {
    let mut expected = BTreeSet::new();
    for domain in domains {
        if let Some(collation) = domain.collation() {
            expected.insert((
                collation.schema_name().to_owned(),
                collation.collation_name().to_owned(),
            ));
        }
    }
    for relation in relations {
        for index in relation.indexes() {
            if let Some(keys) = index.key_semantics() {
                for key in keys {
                    if let Some(collation) = key.collation() {
                        expected.insert((
                            collation.schema_name().to_owned(),
                            collation.collation_name().to_owned(),
                        ));
                    }
                }
            }
        }
    }
    if let Some(columns) = column_collations {
        for column in columns {
            if let Some(collation) = column.collation() {
                expected.insert((
                    collation.schema_name().to_owned(),
                    collation.collation_name().to_owned(),
                ));
            }
        }
    }
    if let Some(ranges) = range_catalog {
        for range in ranges {
            if let Some(collation) = range.collation() {
                expected.insert((
                    collation.schema_name().to_owned(),
                    collation.collation_name().to_owned(),
                ));
            }
        }
    }
    definitions.sort_by(|a, b| {
        (a.collation.schema_name(), a.collation.collation_name())
            .cmp(&(b.collation.schema_name(), b.collation.collation_name()))
    });
    let actual = definitions
        .iter()
        .map(|item| {
            (
                item.collation.schema_name().to_owned(),
                item.collation.collation_name().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    if actual != expected
        || definitions.len() != expected.len()
        || definitions
            .iter()
            .any(|definition| definitions[0].database_encoding != definition.database_encoding)
    {
        return Err(ObservationError::InvalidObservationField {
            field: "collation_definition_coverage",
        });
    }
    if let Some(columns) = column_collations {
        for column in columns {
            if let Some(collation) = column.collation() {
                let definition = definitions
                    .iter()
                    .find(|definition| definition.collation() == collation)
                    .ok_or(ObservationError::InvalidObservationField {
                        field: "collation_definition_coverage",
                    })?;
                if column.deterministic() != Some(definition.deterministic()) {
                    return Err(ObservationError::InvalidObservationField {
                        field: "collation_definition_determinism",
                    });
                }
            }
        }
    }
    Ok(definitions)
}

fn encode_fields(hasher: &mut Sha256, fields: &CollationLocaleFields) {
    for value in [
        &fields.lc_collate,
        &fields.lc_ctype,
        &fields.locale,
        &fields.icu_rules,
        &fields.recorded_version,
        &fields.actual_version,
    ] {
        super::encode_optional_str(hasher, value.as_deref());
    }
}

pub(crate) fn digest(base: &str, definitions: &[CollationDefinitionObservation]) -> String {
    let mut hasher = Sha256::new();
    super::encode_bytes(
        &mut hasher,
        b"conceptweave.postgres_schema_snapshot.v3.collation_definitions.v1",
    );
    super::encode_str(&mut hasher, base);
    super::encode_len(&mut hasher, definitions.len());
    for definition in definitions {
        super::encode_str(&mut hasher, definition.collation.schema_name());
        super::encode_str(&mut hasher, definition.collation.collation_name());
        hasher.update(definition.encoding.to_be_bytes());
        hasher.update(definition.database_encoding.to_be_bytes());
        hasher.update([
            definition.provider.token(),
            u8::from(definition.deterministic),
        ]);
        encode_fields(&mut hasher, &definition.fields);
        match &definition.database_default {
            None => hasher.update([0]),
            Some(database) => {
                hasher.update([1, database.provider.token()]);
                encode_fields(&mut hasher, &database.fields);
            }
        }
        super::encode_optional_str(&mut hasher, definition.comment.as_deref());
    }
    super::encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields(recorded: &str, actual: &str) -> CollationLocaleFields {
        CollationLocaleFields::new(
            None,
            None,
            Some("und-u-ks-level1".to_owned()),
            None,
            Some(recorded.to_owned()),
            Some(actual.to_owned()),
        )
        .unwrap()
    }

    fn definition(recorded: &str, actual: &str) -> CollationDefinitionObservation {
        CollationDefinitionObservation::new(
            QualifiedCollationName::new("public", "casefold").unwrap(),
            -1,
            6,
            CollationProvider::Icu,
            false,
            fields(recorded, actual),
            None,
            None,
        )
        .unwrap()
    }

    #[test]
    fn recorded_and_effective_versions_have_distinct_source_identity() {
        let baseline = digest("base", &[definition("153.120", "153.120")]);
        assert_ne!(baseline, digest("base", &[definition("0", "153.120")]));
        assert_ne!(baseline, digest("base", &[definition("153.120", "154.1")]));
    }

    #[test]
    fn default_database_locale_must_be_explicit() {
        assert!(
            CollationDefinitionObservation::new(
                QualifiedCollationName::new("pg_catalog", "default").unwrap(),
                -1,
                6,
                CollationProvider::DatabaseDefault,
                true,
                CollationLocaleFields::new(None, None, None, None, None, None).unwrap(),
                None,
                None,
            )
            .is_err()
        );
    }

    #[test]
    fn database_locale_provider_requires_its_catalog_fields() {
        let empty = CollationLocaleFields::new(None, None, None, None, None, None).unwrap();
        assert!(DatabaseLocaleDefinition::new(CollationProvider::Icu, empty.clone()).is_err());
        assert!(DatabaseLocaleDefinition::new(CollationProvider::Builtin, empty).is_err());

        let libc_with_locale = CollationLocaleFields::new(
            Some("en_US.UTF-8".to_owned()),
            Some("en_US.UTF-8".to_owned()),
            Some("und".to_owned()),
            None,
            None,
            None,
        )
        .unwrap();
        assert!(DatabaseLocaleDefinition::new(CollationProvider::Libc, libc_with_locale).is_err());

        let libc = CollationLocaleFields::new(
            Some("en_US.UTF-8".to_owned()),
            Some("en_US.UTF-8".to_owned()),
            None,
            None,
            None,
            None,
        )
        .unwrap();
        assert!(DatabaseLocaleDefinition::new(CollationProvider::Libc, libc).is_ok());
    }

    #[test]
    fn effective_default_database_locale_changes_source_identity() {
        let definition = |collate: &str| {
            CollationDefinitionObservation::new(
                QualifiedCollationName::new("pg_catalog", "default").unwrap(),
                -1,
                6,
                CollationProvider::DatabaseDefault,
                true,
                CollationLocaleFields::new(None, None, None, None, None, None).unwrap(),
                Some(
                    DatabaseLocaleDefinition::new(
                        CollationProvider::Libc,
                        CollationLocaleFields::new(
                            Some(collate.to_owned()),
                            Some("en_US.UTF-8".to_owned()),
                            None,
                            None,
                            None,
                            None,
                        )
                        .unwrap(),
                    )
                    .unwrap(),
                ),
                None,
            )
            .unwrap()
        };
        assert_ne!(
            digest("base", &[definition("en_US.UTF-8")]),
            digest("base", &[definition("C")])
        );
    }
}
