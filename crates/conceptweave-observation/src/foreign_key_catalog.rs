use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, QualifiedTypeName, RelationKind, RelationObservation,
    TableConstraintObservation,
};

/// Stable identity of one PostgreSQL binary equality operator used by a foreign key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForeignKeyOperatorObservation {
    schema_name: String,
    operator_name: String,
    left_type: QualifiedTypeName,
    right_type: QualifiedTypeName,
}

impl ForeignKeyOperatorObservation {
    /// Creates an exact operator signature without retaining a database-local OID.
    pub fn new(
        schema_name: impl Into<String>,
        operator_name: impl Into<String>,
        left_type: QualifiedTypeName,
        right_type: QualifiedTypeName,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let operator_name = operator_name.into();
        validate_postgresql_identifier(&schema_name, "foreign_key_operator_schema_name")?;
        validate_postgresql_identifier(&operator_name, "foreign_key_operator_name")?;
        Ok(Self {
            schema_name,
            operator_name,
            left_type,
            right_type,
        })
    }

    /// Returns the exact operator namespace.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact operator name.
    #[must_use]
    pub fn operator_name(&self) -> &str {
        &self.operator_name
    }

    /// Returns the exact left operand type.
    #[must_use]
    pub const fn left_type(&self) -> &QualifiedTypeName {
        &self.left_type
    }

    /// Returns the exact right operand type.
    #[must_use]
    pub const fn right_type(&self) -> &QualifiedTypeName {
        &self.right_type
    }
}

/// Catalog evidence that cannot be reconstructed from a foreign-key definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForeignKeyCatalogObservation {
    schema_name: String,
    relation_name: String,
    constraint_name: String,
    referenced_index_name: String,
    primary_foreign_operators: Vec<ForeignKeyOperatorObservation>,
    primary_primary_operators: Vec<ForeignKeyOperatorObservation>,
    foreign_foreign_operators: Vec<ForeignKeyOperatorObservation>,
}

impl ForeignKeyCatalogObservation {
    /// Creates the three ordered `pg_constraint` operator arrays and referenced backing index.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        constraint_name: impl Into<String>,
        referenced_index_name: impl Into<String>,
        primary_foreign_operators: Vec<ForeignKeyOperatorObservation>,
        primary_primary_operators: Vec<ForeignKeyOperatorObservation>,
        foreign_foreign_operators: Vec<ForeignKeyOperatorObservation>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        let referenced_index_name = referenced_index_name.into();
        for (value, field) in [
            (&schema_name, "foreign_key_schema_name"),
            (&relation_name, "foreign_key_relation_name"),
            (&constraint_name, "foreign_key_constraint_name"),
            (&referenced_index_name, "foreign_key_referenced_index_name"),
        ] {
            validate_postgresql_identifier(value, field)?;
        }
        let arity = primary_foreign_operators.len();
        if arity == 0
            || primary_primary_operators.len() != arity
            || foreign_foreign_operators.len() != arity
        {
            return Err(ObservationError::InvalidObservationField {
                field: "foreign_key_operator_arity",
            });
        }
        Ok(Self {
            schema_name,
            relation_name,
            constraint_name,
            referenced_index_name,
            primary_foreign_operators,
            primary_primary_operators,
            foreign_foreign_operators,
        })
    }

    /// Returns the local schema name.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }
    /// Returns the local relation name.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }
    /// Returns the constraint name.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }
    /// Returns the exact index name selected by PostgreSQL for the referenced key.
    #[must_use]
    pub fn referenced_index_name(&self) -> &str {
        &self.referenced_index_name
    }
    /// Returns ordered primary-key/foreign-key comparison operators.
    #[must_use]
    pub fn primary_foreign_operators(&self) -> &[ForeignKeyOperatorObservation] {
        &self.primary_foreign_operators
    }
    /// Returns ordered primary-key/primary-key comparison operators.
    #[must_use]
    pub fn primary_primary_operators(&self) -> &[ForeignKeyOperatorObservation] {
        &self.primary_primary_operators
    }
    /// Returns ordered foreign-key/foreign-key comparison operators.
    #[must_use]
    pub fn foreign_foreign_operators(&self) -> &[ForeignKeyOperatorObservation] {
        &self.foreign_foreign_operators
    }
}

pub(crate) fn canonicalize(
    relations: &[RelationObservation],
    mut observations: Vec<ForeignKeyCatalogObservation>,
) -> Result<Vec<ForeignKeyCatalogObservation>, ObservationError> {
    observations.sort_by(|a, b| {
        (&a.schema_name, &a.relation_name, &a.constraint_name).cmp(&(
            &b.schema_name,
            &b.relation_name,
            &b.constraint_name,
        ))
    });
    let expected = relations.iter().flat_map(|relation| {
        relation
            .constraints()
            .iter()
            .filter_map(move |constraint| match constraint {
                TableConstraintObservation::ForeignKey(foreign_key) => {
                    Some((relation, foreign_key))
                }
                _ => None,
            })
    });
    let expected_count = expected.clone().count();
    if observations.len() != expected_count {
        return Err(ObservationError::InvalidObservationField {
            field: "foreign_key_catalog_coverage",
        });
    }
    for (relation, foreign_key) in expected {
        let matches = observations
            .iter()
            .filter(|observation| {
                observation.schema_name == relation.schema_name()
                    && observation.relation_name == relation.relation_name()
                    && observation.constraint_name == foreign_key.constraint_name()
            })
            .collect::<Vec<_>>();
        let [observation] = matches.as_slice() else {
            return Err(ObservationError::InvalidObservationField {
                field: "foreign_key_catalog_coverage",
            });
        };
        if observation.primary_foreign_operators.len() != foreign_key.column_names().len() {
            return Err(ObservationError::InvalidObservationField {
                field: "foreign_key_operator_arity",
            });
        }
        let referenced = relations
            .iter()
            .find(|candidate| {
                candidate.schema_name() == foreign_key.referenced_schema_name()
                    && candidate.relation_name() == foreign_key.referenced_table_name()
                    && candidate.kind() == RelationKind::Table
            })
            .ok_or(ObservationError::InvalidObservationField {
                field: "foreign_key_reference_scope",
            })?;
        let index = referenced
            .indexes()
            .iter()
            .find(|index| index.index_name() == observation.referenced_index_name)
            .ok_or(ObservationError::InvalidObservationField {
                field: "foreign_key_referenced_index",
            })?;
        let indexed_columns = index
            .key_attributes()
            .iter()
            .map(|attribute| attribute.attribute_name())
            .collect::<Option<BTreeSet<_>>>();
        let referenced_columns = foreign_key
            .referenced_column_names()
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if !index.is_unique()
            || index.key_attributes().len() != foreign_key.referenced_column_names().len()
            || indexed_columns != Some(referenced_columns)
            || index.predicate().is_some()
            || index.catalog_flags().is_none_or(|flags| !flags.immediate())
            || index.ready() != Some(true)
            || index.valid() != Some(true)
            || index.live() != Some(true)
        {
            return Err(ObservationError::InvalidObservationField {
                field: "foreign_key_referenced_index",
            });
        }
    }
    Ok(observations)
}

pub(crate) fn digest(base: &str, observations: &[ForeignKeyCatalogObservation]) -> String {
    let mut hasher = Sha256::new();
    super::encode_bytes(
        &mut hasher,
        b"conceptweave.postgres_schema_snapshot.v3.foreign_key_catalog.v1",
    );
    super::encode_str(&mut hasher, base);
    super::encode_len(&mut hasher, observations.len());
    for observation in observations {
        for value in [
            &observation.schema_name,
            &observation.relation_name,
            &observation.constraint_name,
            &observation.referenced_index_name,
        ] {
            super::encode_str(&mut hasher, value);
        }
        for operators in [
            &observation.primary_foreign_operators,
            &observation.primary_primary_operators,
            &observation.foreign_foreign_operators,
        ] {
            super::encode_len(&mut hasher, operators.len());
            for operator in operators {
                for value in [&operator.schema_name, &operator.operator_name] {
                    super::encode_str(&mut hasher, value);
                }
                for operand in [&operator.left_type, &operator.right_type] {
                    super::encode_str(&mut hasher, operand.schema_name());
                    super::encode_str(&mut hasher, operand.type_name());
                }
            }
        }
    }
    super::encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operator(name: &str) -> ForeignKeyOperatorObservation {
        let int4 = QualifiedTypeName::new("pg_catalog", "int4").unwrap();
        ForeignKeyOperatorObservation::new("pg_catalog", name, int4.clone(), int4).unwrap()
    }

    fn catalog(name: &str) -> ForeignKeyCatalogObservation {
        ForeignKeyCatalogObservation::new(
            "public",
            "child",
            "parent_fk",
            "parent_pkey",
            vec![operator(name)],
            vec![operator("=")],
            vec![operator("=")],
        )
        .unwrap()
    }

    #[test]
    fn operator_signature_and_backing_index_change_source_identity() {
        let original = catalog("=");
        let changed_operator = catalog("custom_eq");
        assert_ne!(
            digest("base", std::slice::from_ref(&original)),
            digest("base", &[changed_operator])
        );
        let mut changed_index = original.clone();
        changed_index.referenced_index_name = "parent_alt_key".to_owned();
        assert_ne!(
            digest("base", &[original]),
            digest("base", &[changed_index])
        );
    }

    #[test]
    fn incomplete_operator_arrays_are_rejected() {
        assert!(
            ForeignKeyCatalogObservation::new(
                "public",
                "child",
                "parent_fk",
                "parent_pkey",
                vec![operator("=")],
                Vec::new(),
                vec![operator("=")],
            )
            .is_err()
        );
    }
}
