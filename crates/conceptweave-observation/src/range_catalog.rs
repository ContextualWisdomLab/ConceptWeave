//! Exact PostgreSQL range definition evidence beyond range/multirange type pairing.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, PostgresTypeKind, QualifiedCollationName, QualifiedOperatorClassName,
    QualifiedTypeName, TypeKindObservation, encode_bytes, encode_len, encode_sha256, encode_str,
};

const DIGEST_DOMAIN: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.range_catalog.v1";

/// Exact qualified procedure coordinate. PostgreSQL fixes the argument signature for each range
/// function role: the canonical function takes the range; the difference function takes two subtype
/// values. Catalog OIDs are only join coordinates.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualifiedRangeProcedure {
    schema_name: String,
    procedure_name: String,
}

impl QualifiedRangeProcedure {
    /// Preserves exact PostgreSQL identifier text without `search_path` inference.
    pub fn new(
        schema_name: impl Into<String>,
        procedure_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let procedure_name = procedure_name.into();
        validate_postgresql_identifier(&schema_name, "range_procedure_schema_name")?;
        validate_postgresql_identifier(&procedure_name, "range_procedure_name")?;
        Ok(Self {
            schema_name,
            procedure_name,
        })
    }

    /// Returns the exact procedure namespace.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact procedure name.
    #[must_use]
    pub fn procedure_name(&self) -> &str {
        &self.procedure_name
    }
}

/// Material `pg_range` definition for one observed schema-local range type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeCatalogObservation {
    range_type: QualifiedTypeName,
    subtype: QualifiedTypeName,
    subtype_operator_class: QualifiedOperatorClassName,
    collation: Option<QualifiedCollationName>,
    canonical: Option<QualifiedRangeProcedure>,
    subtype_difference: Option<QualifiedRangeProcedure>,
}

impl RangeCatalogObservation {
    /// Records exact catalog coordinates; the adapter verifies their PostgreSQL 18 joins.
    pub fn new(
        range_type: QualifiedTypeName,
        subtype: QualifiedTypeName,
        subtype_operator_class: QualifiedOperatorClassName,
        collation: Option<QualifiedCollationName>,
        canonical: Option<QualifiedRangeProcedure>,
        subtype_difference: Option<QualifiedRangeProcedure>,
    ) -> Self {
        Self {
            range_type,
            subtype,
            subtype_operator_class,
            collation,
            canonical,
            subtype_difference,
        }
    }

    /// Returns the exact range-type coordinate.
    #[must_use]
    pub const fn range_type(&self) -> &QualifiedTypeName {
        &self.range_type
    }

    /// Returns the exact range subtype.
    #[must_use]
    pub const fn subtype(&self) -> &QualifiedTypeName {
        &self.subtype
    }

    /// Returns the exact B-tree operator class used to order subtype values.
    #[must_use]
    pub const fn subtype_operator_class(&self) -> &QualifiedOperatorClassName {
        &self.subtype_operator_class
    }

    /// Returns the range comparison collation, when present.
    #[must_use]
    pub const fn collation(&self) -> Option<&QualifiedCollationName> {
        self.collation.as_ref()
    }

    /// Returns the optional canonicalization procedure.
    #[must_use]
    pub const fn canonical(&self) -> Option<&QualifiedRangeProcedure> {
        self.canonical.as_ref()
    }

    /// Returns the optional subtype difference procedure.
    #[must_use]
    pub const fn subtype_difference(&self) -> Option<&QualifiedRangeProcedure> {
        self.subtype_difference.as_ref()
    }
}

/// Provenance for one exact observed range definition in a successor source snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeCatalogSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    range_type: QualifiedTypeName,
}

impl RangeCatalogSourceReceipt {
    pub(crate) fn new(
        source_id: String,
        connection_policy_binding: String,
        source_digest: String,
        extractor_revision: String,
        observed_at_utc: String,
        range_type: QualifiedTypeName,
    ) -> Self {
        Self {
            source_id,
            connection_policy_binding,
            source_digest,
            extractor_revision,
            observed_at_utc,
            range_type,
        }
    }

    /// Returns the stable source registry key.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the exact authorized connection-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the immutable range-aware source digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the extractor revision used for this observation.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the canonical UTC observation time.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact qualified range type.
    #[must_use]
    pub const fn range_type(&self) -> &QualifiedTypeName {
        &self.range_type
    }

    /// Returns a collision-safe source coordinate.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        fn escape(value: &str) -> String {
            value.replace('~', "~0").replace('/', "~1")
        }
        format!(
            "/schemas/{}/ranges/{}/catalog",
            escape(self.range_type.schema_name()),
            escape(self.range_type.type_name())
        )
    }
}

pub(crate) fn canonicalize(
    type_kinds: &[TypeKindObservation],
    mut observations: Vec<RangeCatalogObservation>,
) -> Result<Vec<RangeCatalogObservation>, ObservationError> {
    observations.sort_by(|a, b| {
        (a.range_type.schema_name(), a.range_type.type_name())
            .cmp(&(b.range_type.schema_name(), b.range_type.type_name()))
    });
    let expected = type_kinds
        .iter()
        .filter(|kind| kind.kind() == PostgresTypeKind::Range)
        .map(|kind| (kind.type_name().schema_name(), kind.type_name().type_name()))
        .collect::<BTreeSet<_>>();
    let actual = observations
        .iter()
        .map(|item| (item.range_type.schema_name(), item.range_type.type_name()))
        .collect::<BTreeSet<_>>();
    if actual != expected || observations.len() != expected.len() {
        return Err(ObservationError::InvalidObservationField {
            field: "range_catalog_coverage",
        });
    }
    Ok(observations)
}

pub(crate) fn digest(base: &str, observations: &[RangeCatalogObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, DIGEST_DOMAIN);
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, observations.len());
    for item in observations {
        for value in [
            item.range_type.schema_name(),
            item.range_type.type_name(),
            item.subtype.schema_name(),
            item.subtype.type_name(),
            item.subtype_operator_class.schema_name(),
            item.subtype_operator_class.operator_class_name(),
        ] {
            encode_str(&mut hasher, value);
        }
        for optional in [
            item.collation
                .as_ref()
                .map(|value| (value.schema_name(), value.collation_name())),
            item.canonical
                .as_ref()
                .map(|value| (value.schema_name(), value.procedure_name())),
            item.subtype_difference
                .as_ref()
                .map(|value| (value.schema_name(), value.procedure_name())),
        ] {
            match optional {
                Some((schema, name)) => {
                    hasher.update([1]);
                    encode_str(&mut hasher, schema);
                    encode_str(&mut hasher, name);
                }
                None => hasher.update([0]),
            }
        }
    }
    encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_catalog_requires_one_definition_per_observed_range() {
        let range = QualifiedTypeName::new("source", "span").unwrap();
        let multi = QualifiedTypeName::new("source", "span_multirange").unwrap();
        let kinds = [
            TypeKindObservation::range(range.clone(), multi.clone()),
            TypeKindObservation::multirange(multi, range.clone()),
        ];
        let definition = RangeCatalogObservation::new(
            range,
            QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
            None,
            None,
            None,
        );
        assert!(canonicalize(&kinds, vec![]).is_err());
        assert!(canonicalize(&kinds, vec![definition.clone(), definition.clone()]).is_err());
        assert_eq!(
            canonicalize(&kinds, vec![definition.clone()]).unwrap(),
            [definition]
        );
    }

    #[test]
    fn all_range_definition_fields_change_the_successor_digest() {
        let base = RangeCatalogObservation::new(
            QualifiedTypeName::new("source", "span").unwrap(),
            QualifiedTypeName::new("pg_catalog", "int4").unwrap(),
            QualifiedOperatorClassName::new("pg_catalog", "int4_ops").unwrap(),
            None,
            None,
            None,
        );
        let original = digest("sha256:prior", std::slice::from_ref(&base));
        let mut changed = base.clone();
        changed.subtype = QualifiedTypeName::new("pg_catalog", "int8").unwrap();
        assert_ne!(original, digest("sha256:prior", &[changed]));
        let mut changed = base.clone();
        changed.subtype_operator_class =
            QualifiedOperatorClassName::new("source", "custom_ops").unwrap();
        assert_ne!(original, digest("sha256:prior", &[changed]));
        let mut changed = base.clone();
        changed.collation = Some(QualifiedCollationName::new("pg_catalog", "C").unwrap());
        assert_ne!(original, digest("sha256:prior", &[changed]));
        let mut changed = base.clone();
        changed.canonical = Some(QualifiedRangeProcedure::new("source", "canonical").unwrap());
        assert_ne!(original, digest("sha256:prior", &[changed]));
        let mut changed = base;
        changed.subtype_difference =
            Some(QualifiedRangeProcedure::new("source", "difference").unwrap());
        assert_ne!(original, digest("sha256:prior", &[changed]));
    }
}
