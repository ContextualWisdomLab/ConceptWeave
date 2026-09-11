//! Exact PostgreSQL true-array type evidence for successor schema observations.

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
