//! Exact storage coordinate for each observed ordinary table.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    IndexTablespace, ObservationError, RelationKind, RelationObservation, encode_bytes, encode_len,
    encode_sha256, encode_str,
};

const DIGEST_DOMAIN: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_tablespace.v1";

/// The exact resolved tablespace and catalog-default marker for one ordinary table.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RelationTablespaceObservation {
    schema_name: String,
    relation_name: String,
    tablespace: IndexTablespace,
}

impl RelationTablespaceObservation {
    /// Records a schema-local ordinary-table storage coordinate.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        tablespace: IndexTablespace,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        validate_postgresql_identifier(&schema_name, "relation_tablespace_schema_name")?;
        validate_postgresql_identifier(&relation_name, "relation_tablespace_relation_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            tablespace,
        })
    }

    /// Returns the exact source schema.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact ordinary-table name.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the resolved storage name and whether PostgreSQL used its database default marker.
    #[must_use]
    pub const fn tablespace(&self) -> &IndexTablespace {
        &self.tablespace
    }
}

pub(crate) fn canonicalize(
    relations: &[RelationObservation],
    mut observations: Vec<RelationTablespaceObservation>,
) -> Result<Vec<RelationTablespaceObservation>, ObservationError> {
    observations.sort_by(|left, right| {
        (left.schema_name(), left.relation_name())
            .cmp(&(right.schema_name(), right.relation_name()))
    });
    let expected = relations
        .iter()
        .filter(|relation| relation.kind() == RelationKind::Table)
        .map(|relation| (relation.schema_name(), relation.relation_name()))
        .collect::<BTreeSet<_>>();
    let actual = observations
        .iter()
        .map(|item| (item.schema_name(), item.relation_name()))
        .collect::<BTreeSet<_>>();
    if actual != expected || observations.len() != expected.len() {
        return Err(ObservationError::InvalidObservationField {
            field: "relation_tablespace_coverage",
        });
    }
    Ok(observations)
}

pub(crate) fn digest(base: &str, observations: &[RelationTablespaceObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, DIGEST_DOMAIN);
    encode_str(&mut hasher, base);
    encode_len(&mut hasher, observations.len());
    for item in observations {
        encode_str(&mut hasher, item.schema_name());
        encode_str(&mut hasher, item.relation_name());
        encode_str(&mut hasher, item.tablespace().name());
        hasher.update([u8::from(item.tablespace().is_database_default())]);
    }
    encode_sha256(hasher)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_table_storage_and_default_marker_are_material() {
        let relations =
            [RelationObservation::new("source", "record", RelationKind::Table, vec![]).unwrap()];
        let default = RelationTablespaceObservation::new(
            "source",
            "record",
            IndexTablespace::database_default("pg_default").unwrap(),
        )
        .unwrap();
        assert!(canonicalize(&relations, vec![]).is_err());
        assert!(canonicalize(&relations, vec![default.clone(), default.clone()]).is_err());
        let explicit = RelationTablespaceObservation::new(
            "source",
            "record",
            IndexTablespace::named("pg_default").unwrap(),
        )
        .unwrap();
        assert_ne!(
            digest("sha256:prior", &[default]),
            digest("sha256:prior", &[explicit])
        );
    }
}
