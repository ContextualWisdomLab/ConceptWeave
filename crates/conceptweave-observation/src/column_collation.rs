//! Source-authoritative PostgreSQL column-collation evidence.
//!
//! `pg_attribute.attcollation` is column state, not index or rendered type state. This module keeps
//! that catalog family separate from the frozen v3 representation so an unobserved family preserves
//! the legacy digest while an observed family is complete for every bounded relation column.

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::column_identity::validate_postgresql_identifier;
use crate::{
    ObservationError, QualifiedCollationName, RelationKind, RelationObservation,
    TableConstraintObservation,
};

const SNAPSHOT_DIGEST_DOMAIN_V3_COLUMN_COLLATIONS_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.column_collations.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
enum ColumnCollationState {
    Uncollatable,
    Collatable {
        collation: QualifiedCollationName,
        deterministic: bool,
    },
}

/// One exact observed `pg_attribute.attcollation` state for a bounded relation column.
///
/// PostgreSQL reports `attcollation = 0` for an uncollatable column. A nonzero catalog OID is a
/// capture-time join coordinate only: before this value crosses the Source Observation ACL the
/// adapter resolves it to the exact qualified `pg_collation` coordinate and the matching
/// `collisdeterministic` value from the same catalog snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColumnCollationObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    column_name: String,
    state: ColumnCollationState,
}

impl ColumnCollationObservation {
    /// Records an explicitly observed `attcollation = 0` column.
    pub fn uncollatable(
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
            ColumnCollationState::Uncollatable,
        )
    }

    /// Records a collatable column with exact qualified collation identity and determinism.
    pub fn collatable(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        collation: QualifiedCollationName,
        deterministic: bool,
    ) -> Result<Self, ObservationError> {
        Self::new(
            schema_name,
            relation_name,
            relation_kind,
            column_name,
            ColumnCollationState::Collatable {
                collation,
                deterministic,
            },
        )
    }

    fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        column_name: impl Into<String>,
        state: ColumnCollationState,
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
            state,
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

    /// Returns the resolved exact qualified collation, or `None` for observed `attcollation = 0`.
    #[must_use]
    pub fn collation(&self) -> Option<&QualifiedCollationName> {
        match &self.state {
            ColumnCollationState::Uncollatable => None,
            ColumnCollationState::Collatable { collation, .. } => Some(collation),
        }
    }

    /// Returns `collisdeterministic` for a collatable column, or `None` when it is uncollatable.
    #[must_use]
    pub fn deterministic(&self) -> Option<bool> {
        match &self.state {
            ColumnCollationState::Uncollatable => None,
            ColumnCollationState::Collatable { deterministic, .. } => Some(*deterministic),
        }
    }
}

pub(crate) fn canonicalize_column_collations(
    relations: &[RelationObservation],
    mut column_collations: Vec<ColumnCollationObservation>,
) -> Result<Vec<ColumnCollationObservation>, ObservationError> {
    column_collations.sort_by(|left, right| {
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

    for pair in column_collations.windows(2) {
        if same_column_coordinate(&pair[0], &pair[1]) {
            return Err(ObservationError::InvalidObservationField {
                field: "column_collation_coordinate",
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
    let mut determinism_by_collation = BTreeMap::<(String, String), bool>::new();

    for observation in &column_collations {
        let Some(relation) = relations.iter().find(|relation| {
            relation.schema_name() == observation.schema_name()
                && relation.relation_name() == observation.relation_name()
                && relation.kind() == observation.relation_kind()
        }) else {
            return Err(ObservationError::InvalidObservationField {
                field: "column_collation_coordinate",
            });
        };
        if !relation
            .columns()
            .iter()
            .any(|column| column.column_name() == observation.column_name())
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_collation_coordinate",
            });
        }

        observed_coordinates.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.column_name().to_owned(),
        ));

        if let (Some(collation), Some(deterministic)) =
            (observation.collation(), observation.deterministic())
        {
            let coordinate = (
                collation.schema_name().to_owned(),
                collation.collation_name().to_owned(),
            );
            if determinism_by_collation
                .insert(coordinate, deterministic)
                .is_some_and(|prior| prior != deterministic)
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "column_collation_determinism",
                });
            }
        }
    }

    if observed_coordinates != expected_coordinates {
        return Err(ObservationError::InvalidObservationField {
            field: "column_collation_completeness",
        });
    }

    validate_foreign_key_collations(relations, &column_collations)?;
    Ok(column_collations)
}

fn validate_foreign_key_collations(
    relations: &[RelationObservation],
    column_collations: &[ColumnCollationObservation],
) -> Result<(), ObservationError> {
    for relation in relations {
        for constraint in relation.constraints() {
            let TableConstraintObservation::ForeignKey(foreign_key) = constraint else {
                continue;
            };
            let Some(referenced_relation) = relations.iter().find(|candidate| {
                candidate.schema_name() == foreign_key.referenced_schema_name()
                    && candidate.relation_name() == foreign_key.referenced_table_name()
            }) else {
                // The observed family is intentionally bounded to local relation columns. An
                // external referenced relation needs its own immutable evidence before ConceptWeave
                // can adjudicate its column collation; absence here must not invent that truth.
                continue;
            };

            for (local_column_name, referenced_column_name) in foreign_key
                .column_names()
                .iter()
                .zip(foreign_key.referenced_column_names())
            {
                let local = find_column_collation(
                    column_collations,
                    relation,
                    local_column_name,
                )
                .expect("complete bounded column-collation family contains local FK column");
                let referenced = find_column_collation(
                    column_collations,
                    referenced_relation,
                    referenced_column_name,
                )
                .expect("complete bounded column-collation family contains referenced FK column");

                if !foreign_key_collation_pair_is_valid(local, referenced) {
                    return Err(ObservationError::InvalidObservationField {
                        field: "foreign_key_collation",
                    });
                }
            }
        }
    }
    Ok(())
}

fn find_column_collation<'a>(
    column_collations: &'a [ColumnCollationObservation],
    relation: &RelationObservation,
    column_name: &str,
) -> Option<&'a ColumnCollationObservation> {
    column_collations.iter().find(|observation| {
        observation.schema_name() == relation.schema_name()
            && observation.relation_name() == relation.relation_name()
            && observation.relation_kind() == relation.kind()
            && observation.column_name() == column_name
    })
}

fn foreign_key_collation_pair_is_valid(
    local: &ColumnCollationObservation,
    referenced: &ColumnCollationObservation,
) -> bool {
    let (Some(local_collation), Some(referenced_collation)) =
        (local.collation(), referenced.collation())
    else {
        // PostgreSQL's additional consistency rule applies to a collatable pair. Collation evidence
        // does not invent type compatibility for a pair where either side is explicitly uncollatable.
        return true;
    };

    let both_deterministic = local.deterministic() == Some(true)
        && referenced.deterministic() == Some(true);
    let exact_same_collation = local_collation.schema_name() == referenced_collation.schema_name()
        && local_collation.collation_name() == referenced_collation.collation_name();
    both_deterministic || exact_same_collation
}

fn same_column_coordinate(
    left: &ColumnCollationObservation,
    right: &ColumnCollationObservation,
) -> bool {
    left.schema_name() == right.schema_name()
        && left.relation_name() == right.relation_name()
        && left.relation_kind() == right.relation_kind()
        && left.column_name() == right.column_name()
}

pub(crate) fn compute_column_collation_digest(
    base_snapshot_digest: &str,
    column_collations: &[ColumnCollationObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        SNAPSHOT_DIGEST_DOMAIN_V3_COLUMN_COLLATIONS_V1,
    );
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, column_collations.len());
    for observation in column_collations {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_str(&mut hasher, observation.column_name());
        match (&observation.state, observation.deterministic()) {
            (ColumnCollationState::Uncollatable, None) => hasher.update([0]),
            (
                ColumnCollationState::Collatable { collation, .. },
                Some(deterministic),
            ) => {
                hasher.update([1]);
                encode_str(&mut hasher, collation.schema_name());
                encode_str(&mut hasher, collation.collation_name());
                hasher.update([u8::from(deterministic)]);
            }
            _ => unreachable!("column collation state keeps determinism structurally coherent"),
        }
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
