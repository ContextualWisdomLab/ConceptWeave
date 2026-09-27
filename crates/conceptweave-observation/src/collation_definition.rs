use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
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

/// Exact catalog owner of one referenced PostgreSQL collation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollationOwnerObservation {
    collation: QualifiedCollationName,
    owner_oid: u32,
    owner_role_name: String,
}

impl CollationOwnerObservation {
    /// Records the owner OID and its resolved role name for one collation.
    pub fn new(
        collation: QualifiedCollationName,
        owner_oid: u32,
        owner_role_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let owner_role_name = owner_role_name.into();
        validate_postgresql_identifier(&owner_role_name, "collation_owner_role_name")?;
        if owner_oid == 0 {
            return Err(ObservationError::InvalidObservationField {
                field: "collation_owner_oid",
            });
        }
        Ok(Self {
            collation,
            owner_oid,
            owner_role_name,
        })
    }

    /// Returns the exact qualified collation coordinate.
    #[must_use]
    pub const fn collation(&self) -> &QualifiedCollationName {
        &self.collation
    }

    /// Returns the catalog role OID.
    #[must_use]
    pub const fn owner_oid(&self) -> u32 {
        self.owner_oid
    }

    /// Returns the resolved role name.
    #[must_use]
    pub fn owner_role_name(&self) -> &str {
        &self.owner_role_name
    }
}

pub(crate) fn canonicalize_owners(
    definitions: &[CollationDefinitionObservation],
    mut owners: Vec<CollationOwnerObservation>,
) -> Result<Vec<CollationOwnerObservation>, ObservationError> {
    owners.sort_by(|left, right| {
        (
            left.collation.schema_name(),
            left.collation.collation_name(),
        )
            .cmp(&(
                right.collation.schema_name(),
                right.collation.collation_name(),
            ))
    });
    let expected = definitions
        .iter()
        .map(|item| {
            (
                item.collation().schema_name(),
                item.collation().collation_name(),
            )
        })
        .collect::<BTreeSet<_>>();
    let actual = owners
        .iter()
        .map(|item| {
            (
                item.collation.schema_name(),
                item.collation.collation_name(),
            )
        })
        .collect::<BTreeSet<_>>();
    if expected != actual || owners.len() != expected.len() {
        return Err(ObservationError::InvalidObservationField {
            field: "collation_owner_coverage",
        });
    }
    Ok(owners)
}

pub(crate) fn owner_digest(base: &str, owners: &[CollationOwnerObservation]) -> String {
    let mut hasher = Sha256::new();
    super::encode_bytes(
        &mut hasher,
        b"conceptweave.postgres_schema_snapshot.v3.collation_owner.v1",
    );
    super::encode_str(&mut hasher, base);
    super::encode_len(&mut hasher, owners.len());
    for owner in owners {
        super::encode_str(&mut hasher, owner.collation.schema_name());
        super::encode_str(&mut hasher, owner.collation.collation_name());
        hasher.update(owner.owner_oid.to_be_bytes());
        super::encode_str(&mut hasher, &owner.owner_role_name);
    }
    super::encode_sha256(hasher)
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
    fn provider_shapes_preserve_supported_null_fields_and_reject_mixed_shapes() {
        let database = DatabaseLocaleDefinition::new(
            CollationProvider::Libc,
            CollationLocaleFields::new(Some("C".into()), Some("C".into()), None, None, None, None)
                .unwrap(),
        )
        .unwrap();
        let mut observed_digests = BTreeSet::new();
        // Bits describe LC_COLLATE, LC_CTYPE, locale, rules, recorded and actual version.
        for (provider, admitted_masks) in [
            (CollationProvider::DatabaseDefault, &[0_u8, 32][..]),
            (CollationProvider::Builtin, &[20, 52][..]),
            (CollationProvider::Libc, &[3, 19, 35, 51][..]),
            (CollationProvider::Icu, &[52, 60][..]),
        ] {
            for mask in 0_u8..64 {
                let [collate, ctype, locale, rules, recorded, actual] =
                    std::array::from_fn(|slot| {
                        (mask & (1 << slot) != 0)
                            .then(|| ["C", "C", "C", "", "1", "1"][slot].into())
                    });
                let fields =
                    CollationLocaleFields::new(collate, ctype, locale, rules, recorded, actual)
                        .unwrap();
                for deterministic in [false, true] {
                    let is_default = provider == CollationProvider::DatabaseDefault;
                    let observed = CollationDefinitionObservation::new(
                        QualifiedCollationName::new(
                            if is_default { "pg_catalog" } else { "public" },
                            if is_default { "default" } else { "comparison" },
                        )
                        .unwrap(),
                        -1,
                        6,
                        provider,
                        deterministic,
                        fields.clone(),
                        is_default.then(|| database.clone()),
                        None,
                    );
                    let admitted = admitted_masks.contains(&mask)
                        && (deterministic || provider == CollationProvider::Icu);
                    assert_eq!(
                        observed.is_ok(),
                        admitted,
                        "{provider:?}/{mask}/{deterministic}"
                    );
                    match observed {
                        Ok(observed) => {
                            assert_eq!(observed.fields(), &fields);
                            assert_eq!(observed.deterministic(), deterministic);
                            assert!(
                                observed_digests
                                    .insert(digest("base", std::slice::from_ref(&observed))),
                                "distinct admitted locale evidence must have distinct content identity"
                            );
                        }
                        Err(error) => assert_eq!(
                            error,
                            ObservationError::InvalidObservationField {
                                field: "collation_definition_shape",
                            }
                        ),
                    }
                }
            }
        }
        for slot in 0..6 {
            let mut raw = [None, None, None, None, None, None];
            raw[slot] = Some("bad\0text".into());
            let [collate, ctype, locale, rules, recorded, actual] = raw;
            assert_eq!(
                CollationLocaleFields::new(collate, ctype, locale, rules, recorded, actual),
                Err(ObservationError::InvalidObservationField {
                    field: "collation_locale_text"
                })
            );
        }
    }

    #[test]
    fn definition_encoding_and_default_coordinate_boundaries_fail_closed() {
        let baseline = definition("1", "1");
        let mut cases = vec![baseline.clone(); 7];
        cases[0].encoding = -2;
        cases[1].database_encoding = -1;
        cases[2].encoding = 0;
        cases[3].collation = QualifiedCollationName::new("pg_catalog", "default").unwrap();
        cases[4].comment = Some("bad\0comment".into());
        cases[5].database_default = Some(
            DatabaseLocaleDefinition::new(CollationProvider::Icu, baseline.fields.clone()).unwrap(),
        );
        cases[6].collation = QualifiedCollationName::new("pg_catalog", "default").unwrap();
        cases[6].provider = CollationProvider::DatabaseDefault;
        cases[6].encoding = 0;
        cases[6].deterministic = true;
        cases[6].fields = CollationLocaleFields::new(None, None, None, None, None, None).unwrap();
        cases[6].database_default = cases[5].database_default.clone();
        for invalid in cases {
            assert_eq!(
                CollationDefinitionObservation::new(
                    invalid.collation,
                    invalid.encoding,
                    invalid.database_encoding,
                    invalid.provider,
                    invalid.deterministic,
                    invalid.fields,
                    invalid.database_default,
                    invalid.comment
                ),
                Err(ObservationError::InvalidObservationField {
                    field: "collation_definition_shape"
                })
            );
        }
        for locale in ["C", "C.UTF-8", "PG_UNICODE_FAST", "unsupported"] {
            let builtin_fields = CollationLocaleFields::new(
                None,
                None,
                Some(locale.into()),
                None,
                Some("1".into()),
                Some("1".into()),
            )
            .unwrap();
            assert_eq!(
                CollationDefinitionObservation::new(
                    baseline.collation.clone(),
                    6,
                    6,
                    CollationProvider::Builtin,
                    true,
                    builtin_fields,
                    None,
                    Some(String::new())
                )
                .is_ok(),
                locale != "unsupported"
            );
        }
    }

    #[test]
    fn referenced_definitions_require_exact_complete_coherent_coverage() {
        let first = definition("1", "1");
        let mut second = first.clone();
        second.collation = QualifiedCollationName::new("public", "other").unwrap();
        let column = |definition: &CollationDefinitionObservation, deterministic| {
            ColumnCollationObservation::collatable(
                "public",
                "records",
                crate::RelationKind::Table,
                definition.collation().collation_name(),
                definition.collation().clone(),
                deterministic,
            )
            .unwrap()
        };
        let columns = [column(&first, false), column(&second, false)];
        let canonical = |definitions| canonicalize(&[], &[], Some(&columns), None, definitions);
        let forward = canonical(vec![first.clone(), second.clone()]).unwrap();
        let reverse = canonical(vec![second.clone(), first.clone()]).unwrap();
        assert_eq!(forward, reverse);
        assert_eq!(digest("base", &forward), digest("base", &reverse));
        let mut foreign = second.clone();
        foreign.collation = QualifiedCollationName::new("archive", "other").unwrap();
        let mut mismatched_encoding = second.clone();
        mismatched_encoding.database_encoding = 0;
        for incomplete in [
            vec![],
            vec![first.clone()],
            vec![first.clone(), first.clone(), second.clone()],
            vec![first.clone(), foreign],
            vec![first.clone(), mismatched_encoding],
        ] {
            assert_eq!(
                canonical(incomplete),
                Err(ObservationError::InvalidObservationField {
                    field: "collation_definition_coverage",
                })
            );
        }
        let columns = [column(&first, true), column(&second, false)];
        assert_eq!(
            canonicalize(&[], &[], Some(&columns), None, vec![first, second]),
            Err(ObservationError::InvalidObservationField {
                field: "collation_definition_determinism"
            })
        );
    }

    #[test]
    fn recorded_and_effective_versions_have_distinct_source_identity() {
        let baseline = digest("base", &[definition("153.120", "153.120")]);
        assert_ne!(baseline, digest("base", &[definition("0", "153.120")]));
        assert_ne!(baseline, digest("base", &[definition("153.120", "154.1")]));
    }

    #[test]
    fn collation_owner_digest_requires_complete_exact_coverage() {
        let definitions = vec![definition("153.120", "153.120")];
        let coordinate = definitions[0].collation().clone();
        assert_eq!(
            CollationOwnerObservation::new(coordinate.clone(), 0, "postgres"),
            Err(ObservationError::InvalidObservationField {
                field: "collation_owner_oid"
            })
        );
        let original = CollationOwnerObservation::new(coordinate.clone(), 10, "postgres").unwrap();
        let renamed = CollationOwnerObservation::new(coordinate.clone(), 10, "steward").unwrap();
        let changed = CollationOwnerObservation::new(coordinate, 11, "steward").unwrap();
        assert_ne!(
            owner_digest("base", std::slice::from_ref(&original)),
            owner_digest("base", std::slice::from_ref(&changed))
        );
        assert_ne!(
            owner_digest("base", std::slice::from_ref(&original)),
            owner_digest("base", std::slice::from_ref(&renamed))
        );
        assert!(canonicalize_owners(&definitions, vec![]).is_err());
        assert!(canonicalize_owners(&definitions, vec![original.clone(), original]).is_err());
        assert!(canonicalize_owners(&definitions, vec![changed]).is_ok());

        let mut second = definitions[0].clone();
        second.collation = QualifiedCollationName::new("public", "other").unwrap();
        let first_owner =
            CollationOwnerObservation::new(definitions[0].collation().clone(), 10, "postgres")
                .unwrap();
        let second_owner =
            CollationOwnerObservation::new(second.collation().clone(), 11, "steward").unwrap();
        let complete = [definitions[0].clone(), second];
        let forward =
            canonicalize_owners(&complete, vec![first_owner.clone(), second_owner.clone()])
                .unwrap();
        let reverse = canonicalize_owners(&complete, vec![second_owner, first_owner]).unwrap();
        assert_eq!(
            owner_digest("base", &forward),
            owner_digest("base", &reverse)
        );
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
        assert!(DatabaseLocaleDefinition::new(CollationProvider::Libc, libc.clone()).is_ok());
        let mut incomplete = [libc.clone(), libc.clone(), libc];
        incomplete[0].lc_collate = None;
        incomplete[1].lc_ctype = None;
        incomplete[2].icu_rules = Some(String::new());
        for fields in incomplete {
            assert_eq!(
                DatabaseLocaleDefinition::new(CollationProvider::Libc, fields),
                Err(ObservationError::InvalidObservationField {
                    field: "database_locale_provider"
                })
            );
        }
        let builtin_with_rules = CollationLocaleFields {
            locale: Some("C".into()),
            icu_rules: Some(String::new()),
            ..fields("1", "1")
        };
        assert_eq!(
            DatabaseLocaleDefinition::new(CollationProvider::Builtin, builtin_with_rules),
            Err(ObservationError::InvalidObservationField {
                field: "database_locale_provider"
            })
        );
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
