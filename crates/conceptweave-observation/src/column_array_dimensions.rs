//! Exact declared PostgreSQL array dimensions for each observed relation column.

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ArrayTypeObservation, ObservationError, RelationKind, RelationObservation, encode_bytes,
    encode_len, encode_sha256, encode_str,
};

const DIGEST_DOMAIN: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.column_array_dimensions.v1";

/// Exact `pg_attribute.attndims` for one bounded relation column.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnArrayDimensionsObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    column_name: String,
    dimensions: u16,
}

impl ColumnArrayDimensionsObservation {
    /// Records the catalog declaration count; zero is explicit for scalar columns.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        dimensions: u16,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let column_name = column_name.into();
        validate_postgresql_identifier(&schema_name, "array_dimensions_schema_name")?;
        validate_postgresql_identifier(&relation_name, "array_dimensions_relation_name")?;
        validate_postgresql_identifier(&column_name, "array_dimensions_column_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            dimensions,
        })
    }

    /// Returns the exact source schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source relation.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the observed relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source column.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns the declared PostgreSQL array-dimension count.
    #[must_use]
    pub const fn dimensions(&self) -> u16 {
        self.dimensions
    }
}

pub(crate) fn canonicalize(
    relations: &[RelationObservation],
    array_types: &[ArrayTypeObservation],
    mut observations: Vec<ColumnArrayDimensionsObservation>,
) -> Result<Vec<ColumnArrayDimensionsObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.column_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.column_name(),
            ))
    });
    let expected = relations
        .iter()
        .flat_map(|relation| {
            relation.columns().iter().map(move |column| {
                (
                    (
                        relation.schema_name(),
                        relation.relation_name(),
                        relation.kind().token(),
                        column.column_name(),
                    ),
                    column.type_binding(),
                )
            })
        })
        .collect::<BTreeMap<_, _>>();
    if observations.len() != expected.len() {
        return Err(ObservationError::InvalidObservationField {
            field: "column_array_dimensions_coverage",
        });
    }
    for pair in observations.windows(2) {
        if (
            pair[0].schema_name(),
            pair[0].relation_name(),
            pair[0].relation_kind(),
            pair[0].column_name(),
        ) == (
            pair[1].schema_name(),
            pair[1].relation_name(),
            pair[1].relation_kind(),
            pair[1].column_name(),
        ) {
            return Err(ObservationError::InvalidObservationField {
                field: "column_array_dimensions_coordinate",
            });
        }
    }
    for item in &observations {
        let coordinate = (
            item.schema_name(),
            item.relation_name(),
            item.relation_kind().token(),
            item.column_name(),
        );
        let Some(type_binding) = expected.get(&coordinate) else {
            return Err(ObservationError::InvalidObservationField {
                field: "column_array_dimensions_coordinate",
            });
        };
        if item.dimensions() > 0
            && !array_types
                .iter()
                .any(|array| array.array_type() == *type_binding)
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_array_dimensions_type",
            });
        }
    }
    Ok(observations)
}

pub(crate) fn digest(base: &str, observations: &[ColumnArrayDimensionsObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, DIGEST_DOMAIN);
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, observations.len());
    for item in observations {
        encode_str(&mut hasher, item.schema_name());
        encode_str(&mut hasher, item.relation_name());
        encode_str(&mut hasher, item.relation_kind().token());
        encode_str(&mut hasher, item.column_name());
        hasher.update(item.dimensions().to_be_bytes());
    }
    encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ColumnObservationV3, QualifiedTypeName};

    #[test]
    fn dimension_evidence_is_complete_and_binds_declaration_count() {
        let array = QualifiedTypeName::new("pg_catalog", "_int4").unwrap();
        let element = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
        let relations = [RelationObservation::new(
            "source",
            "record",
            RelationKind::Table,
            vec![
                ColumnObservationV3::new("payload", 1, "integer[]", array.clone(), true, None)
                    .unwrap(),
            ],
        )
        .unwrap()];
        let arrays = [ArrayTypeObservation::new(array, element).unwrap()];
        let one = ColumnArrayDimensionsObservation::new(
            "source",
            "record",
            RelationKind::Table,
            "payload",
            1,
        )
        .unwrap();
        assert!(canonicalize(&relations, &arrays, vec![]).is_err());
        assert!(canonicalize(&relations, &arrays, vec![one.clone(), one.clone()]).is_err());
        assert!(canonicalize(&relations, &[], vec![one.clone()]).is_err());
        let two = ColumnArrayDimensionsObservation::new(
            "source",
            "record",
            RelationKind::Table,
            "payload",
            2,
        )
        .unwrap();
        assert_ne!(
            digest("sha256:prior", &[one]),
            digest("sha256:prior", &[two])
        );
    }
}
