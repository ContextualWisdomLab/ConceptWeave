//! Source-authoritative PostgreSQL generated-column declaration evidence.
//!
//! `pg_attribute.attgenerated` directly declares whether a bounded column is ordinary, stored
//! generated, or virtual generated. This family remains separate from the frozen v3 column
//! representation so an unobserved family preserves the existing digest while an observed family
//! is complete for every bounded relation column.

use std::collections::BTreeSet;

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{ObservationError, RelationKind, RelationObservation};

const SNAPSHOT_DIGEST_DOMAIN_V3_COLUMN_GENERATION_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.column_generation.v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ColumnGenerationMode {
    NotGenerated,
    Stored,
    Virtual,
}

impl ColumnGenerationMode {
    const fn tag(self) -> u8 {
        match self {
            Self::NotGenerated => 0,
            Self::Stored => 1,
            Self::Virtual => 2,
        }
    }
}

/// One exact observed `pg_attribute.attgenerated` state for a bounded relation column.
///
/// The PostgreSQL adapter maps only the documented empty, `s`, and `v` catalog values to this value
/// object. Generation expressions, dependencies, and default expressions are separate source facts
/// and are deliberately not inferred from this declaration mode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnGenerationObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    column_name: String,
    mode: ColumnGenerationMode,
}

impl ColumnGenerationObservation {
    /// Records an explicitly observed ordinary column (`attgenerated = ''`).
    pub fn not_generated(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnGenerationMode::NotGenerated,
        )
    }

    /// Records a stored generated column (`attgenerated = 's'`).
    pub fn stored(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnGenerationMode::Stored,
        )
    }

    /// Records a virtual generated column (`attgenerated = 'v'`).
    pub fn virtual_generated(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnGenerationMode::Virtual,
        )
    }

    fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        mode: ColumnGenerationMode,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let column_name = column_name.into();
        validate_postgresql_identifier(&schema_name, "schema_name")?;
        validate_postgresql_identifier(&relation_name, "relation_name")?;
        validate_postgresql_identifier(&column_name, "column_name")?;
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            mode,
        })
    }

    /// Returns the exact source schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact source relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact observed relation kind for the owning column.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns whether this observation explicitly reports an ordinary non-generated column.
    #[must_use]
    pub const fn is_not_generated(&self) -> bool {
        matches!(self.mode, ColumnGenerationMode::NotGenerated)
    }

    /// Returns whether this observation reports a stored generated column.
    #[must_use]
    pub const fn is_stored(&self) -> bool {
        matches!(self.mode, ColumnGenerationMode::Stored)
    }

    /// Returns whether this observation reports a virtual generated column.
    #[must_use]
    pub const fn is_virtual_generated(&self) -> bool {
        matches!(self.mode, ColumnGenerationMode::Virtual)
    }
}

pub(crate) fn canonicalize_column_generations(
    relations: &[RelationObservation],
    mut column_generations: Vec<ColumnGenerationObservation>,
) -> Result<Vec<ColumnGenerationObservation>, ObservationError> {
    column_generations.sort_by(|left, right| {
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

    for pair in column_generations.windows(2) {
        if same_column_coordinate(&pair[0], &pair[1]) {
            return Err(ObservationError::InvalidObservationField {
                field: "column_generation_coordinate",
            });
        }
    }

    let expected_coordinates = relations
        .iter()
        .flat_map(|relation| {
            relation.columns().iter().map(move |column| {
                (
                    relation.schema_name().to_owned(),
                    relation.relation_name().to_owned(),
                    relation.kind().token().to_owned(),
                    column.column_name().to_owned(),
                )
            })
        })
        .collect::<BTreeSet<_>>();
    let mut observed_coordinates = BTreeSet::new();

    for observation in &column_generations {
        let coordinate_exists = relations.iter().any(|relation| {
            relation.schema_name() == observation.schema_name()
                && relation.relation_name() == observation.relation_name()
                && relation.kind() == observation.relation_kind()
                && relation
                    .columns()
                    .iter()
                    .any(|column| column.column_name() == observation.column_name())
        });
        if !coordinate_exists {
            return Err(ObservationError::InvalidObservationField {
                field: "column_generation_coordinate",
            });
        }
        if !observation.is_not_generated()
            && !relation_kind_supports_generation(observation.relation_kind())
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_generation_relation_kind",
            });
        }

        observed_coordinates.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.column_name().to_owned(),
        ));
    }

    if observed_coordinates != expected_coordinates {
        return Err(ObservationError::InvalidObservationField {
            field: "column_generation_completeness",
        });
    }

    Ok(column_generations)
}

const fn relation_kind_supports_generation(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Table | RelationKind::PartitionedTable | RelationKind::ForeignTable
    )
}

pub(crate) fn generated_identity_conflicts(
    column_generations: &[ColumnGenerationObservation],
    column_identities: &[crate::ColumnIdentityObservation],
) -> bool {
    column_generations.iter().any(|generation| {
        !generation.is_not_generated()
            && column_identities.iter().any(|identity| {
                identity.schema_name() == generation.schema_name()
                    && identity.relation_name() == generation.relation_name()
                    && identity.relation_kind() == generation.relation_kind()
                    && identity.column_name() == generation.column_name()
                    && !identity.is_not_identity()
            })
    })
}

fn same_column_coordinate(
    left: &ColumnGenerationObservation,
    right: &ColumnGenerationObservation,
) -> bool {
    left.schema_name() == right.schema_name()
        && left.relation_name() == right.relation_name()
        && left.relation_kind() == right.relation_kind()
        && left.column_name() == right.column_name()
}

pub(crate) fn compute_column_generation_digest(
    base_snapshot_digest: &str,
    column_generations: &[ColumnGenerationObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V3_COLUMN_GENERATION_V1);
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, column_generations.len());
    for observation in column_generations {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_str(&mut hasher, observation.column_name());
        hasher.update([observation.mode.tag()]);
    }
    encode_sha256(hasher)
}

fn encode_sha256(hasher: Sha256) -> String {
    let digest = hasher.finalize();
    let mut encoded = String::with_capacity("sha256:".len() + digest.len() * 2);
    encoded.push_str("sha256:");
    const HEX: &[u8; 16] = b"0123456789abcdef";
    for byte in digest {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_bytes(hasher, value.as_bytes());
}

fn encode_bytes(hasher: &mut Sha256, value: &[u8]) {
    encode_len(hasher, value.len());
    hasher.update(value);
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}
