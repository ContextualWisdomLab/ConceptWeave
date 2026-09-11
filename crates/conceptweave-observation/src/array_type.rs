//! Exact PostgreSQL true-array type evidence and successor receipt coordinates.

use crate::model::ObservationError;
use crate::representation_v3::QualifiedTypeName;

/// Exact `pg_type` relationship between one true array type and its element type.
///
/// The observation preserves the two schema-qualified catalog coordinates exposed by the element
/// row's `typarray` and the array row's `typelem`. PostgreSQL keeps an associated true array in the
/// same schema as its element type, including when that type is moved. Catalog OIDs, `search_path`,
/// display text, and PostgreSQL's conventional underscore naming are deliberately excluded from
/// governed identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayTypeObservation {
    array_type: QualifiedTypeName,
    element_type: QualifiedTypeName,
}

impl ArrayTypeObservation {
    /// Creates one exact true-array/element pair from already-resolved catalog coordinates.
    pub fn new(
        array_type: QualifiedTypeName,
        element_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        if array_type.schema_name() != element_type.schema_name()
            || array_type.type_name() == element_type.type_name()
        {
            return Err(ObservationError::InvalidObservationField {
                field: "array_type_element",
            });
        }
        Ok(Self {
            array_type,
            element_type,
        })
    }

    /// Returns the exact schema-qualified true-array type coordinate.
    #[must_use]
    pub const fn array_type(&self) -> &QualifiedTypeName {
        &self.array_type
    }

    /// Returns the exact schema-qualified element type coordinate.
    #[must_use]
    pub const fn element_type(&self) -> &QualifiedTypeName {
        &self.element_type
    }
}

/// Exact successor evidence coordinate for one observed PostgreSQL true-array type.
///
/// This is deliberately separate from the pre-array [`crate::SchemaObjectLocation`] vocabulary so
/// adding true-array evidence does not reinterpret existing relation/domain/enum receipt paths.
/// Identifier tokens retain exact source text and use RFC 6901 escaping in the canonical string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayTypeLocation {
    schema_name: String,
    array_type_name: String,
}

impl ArrayTypeLocation {
    /// Creates an exact schema-qualified true-array receipt coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        array_type_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let array_type_name = array_type_name.into();
        QualifiedTypeName::new(schema_name.clone(), array_type_name.clone())?;
        Ok(Self {
            schema_name,
            array_type_name,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact observed true-array type identifier.
    #[must_use]
    pub fn array_type_name(&self) -> &str {
        &self.array_type_name
    }

    /// Returns the deterministic collision-safe successor receipt path.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        format!(
            "/schemas/{}/array-types/{}",
            escape_coordinate_token(&self.schema_name),
            escape_coordinate_token(&self.array_type_name)
        )
    }
}

/// Immutable provenance receipt for one exact observed PostgreSQL true-array coordinate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayTypeSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: ArrayTypeLocation,
}

impl ArrayTypeSourceReceipt {
    pub(crate) fn new(
        source_id: String,
        connection_policy_binding: String,
        source_digest: String,
        extractor_revision: String,
        observed_at_utc: String,
        location: ArrayTypeLocation,
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

    /// Returns the stable source reference used by candidate evidence binding.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable connection-policy revision used for this observation.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the array-aware governed source digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the verified exact true-array coordinate inside the snapshot.
    #[must_use]
    pub const fn location(&self) -> &ArrayTypeLocation {
        &self.location
    }
}

fn escape_coordinate_token(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}
