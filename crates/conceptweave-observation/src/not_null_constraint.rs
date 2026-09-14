//! Source-authoritative PostgreSQL 18 `NOT NULL` constraint evidence.
//!
//! PostgreSQL 18 stores explicit column `NOT NULL` specifications in `pg_constraint`. The column
//! `pg_attribute.attnotnull` flag is only a summary and can describe a not-yet-validated constraint,
//! so this family retains the first-class constraint row separately from the frozen v3 column
//! representation. Catalog OIDs remain capture-time join coordinates and never enter governed
//! identity.

use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};

use crate::{ObservationError, RelationKind, RelationObservation};

const SNAPSHOT_DIGEST_DOMAIN_V3_NOT_NULL_CONSTRAINT_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.not_null_constraint.v1";

/// Stable source coordinate for the parent partition constraint represented by `conparentid`.
///
/// PostgreSQL exposes the relationship as a catalog OID. The adapter resolves that OID inside the
/// same catalog snapshot and supplies this source coordinate instead; the OID itself is deliberately
/// excluded from governed identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParentNotNullConstraintCoordinate {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
}

impl ParentNotNullConstraintCoordinate {
    /// Creates one resolved parent-constraint coordinate from exact source identifiers.
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        crate::model::validate_nonblank(&schema_name, "not_null_parent_schema_name")?;
        crate::model::validate_nonblank(&relation_name, "not_null_parent_relation_name")?;
        crate::model::validate_nonblank(&constraint_name, "not_null_parent_constraint_name")?;
        if relation_kind != RelationKind::PartitionedTable {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_parent_relation_kind",
            });
        }
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
        })
    }

    /// Returns the exact parent schema identifier.
    #[must_use]
    pub fn schema_name(&self) -> &str {
        &self.schema_name
    }

    /// Returns the exact parent relation identifier.
    #[must_use]
    pub fn relation_name(&self) -> &str {
        &self.relation_name
    }

    /// Returns the exact observed parent relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact parent constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }
}

/// One exact PostgreSQL 18 relation-scoped `NOT NULL` constraint observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NotNullConstraintObservation {
    schema_name: String,
    relation_name: String,
    relation_kind: RelationKind,
    constraint_name: String,
    column_name: String,
    validated: bool,
    enforced: bool,
    is_local: bool,
    inheritance_ancestor_count: u16,
    no_inherit: bool,
    parent_constraint: Option<ParentNotNullConstraintCoordinate>,
    partition_parent_relation: Option<(String, String)>,
}

impl NotNullConstraintObservation {
    /// Creates one source-authoritative `pg_constraint.contype = 'n'` observation.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        relation_kind: RelationKind,
        constraint_name: impl Into<String>,
        column_name: impl Into<String>,
        validated: bool,
        enforced: bool,
        is_local: bool,
        inheritance_ancestor_count: u16,
        no_inherit: bool,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        let constraint_name = constraint_name.into();
        let column_name = column_name.into();
        crate::model::validate_nonblank(&schema_name, "schema_name")?;
        crate::model::validate_nonblank(&relation_name, "relation_name")?;
        crate::model::validate_nonblank(&constraint_name, "not_null_constraint_name")?;
        crate::model::validate_nonblank(&column_name, "not_null_constraint_column_name")?;
        // PostgreSQL 18 supports NOT ENFORCED only for CHECK and foreign-key constraints.
        // A first-class NOT NULL row with conenforced=false is therefore not source-representable
        // evidence and must fail closed before it can acquire governed identity.
        if !enforced {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_enforcement",
            });
        }
        // PostgreSQL 18 stores pg_constraint.coninhcount as signed int2. Preserve the nonnegative
        // count in the public model, but reject values the source catalog cannot represent.
        if inheritance_ancestor_count > 32_767 {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_inheritance_ancestor_count",
            });
        }
        // PostgreSQL creates an inherited-only NOT NULL with at least one inheritance ancestor.
        // `conislocal=false` with `coninhcount=0` has neither a local origin nor an inherited one,
        // so accepting it would grant governed identity to caller-fabricated catalog evidence.
        if !is_local && inheritance_ancestor_count == 0 {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_inheritance_origin",
            });
        }
        // PostgreSQL 18 requires NOT NULL constraints declared on a partitioned table to be
        // inherited by every partition. A partitioned-table `NO INHERIT` row is therefore not
        // source-representable evidence and must not acquire a governed semantic identity.
        if relation_kind == RelationKind::PartitionedTable && no_inherit {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_no_inherit",
            });
        }
        Ok(Self {
            schema_name,
            relation_name,
            relation_kind,
            constraint_name,
            column_name,
            validated,
            enforced,
            is_local,
            inheritance_ancestor_count,
            no_inherit,
            parent_constraint: None,
            partition_parent_relation: None,
        })
    }

    /// Attaches the resolved source coordinate represented by a nonzero `conparentid`.
    pub fn with_parent_constraint(
        mut self,
        parent_constraint: ParentNotNullConstraintCoordinate,
    ) -> Result<Self, ObservationError> {
        // REL_18_STABLE ConstraintSetParentConstraint() turns the partition-child row into inherited
        // state before storing conparentid: conislocal is false and coninhcount advances from 0 to 1.
        // Regular table inheritance can legitimately have conparentid=0 with other coninhcount values,
        // so these invariants belong specifically at the partition-parent linkage boundary.
        if self.is_local {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_parent_locality",
            });
        }
        if self.inheritance_ancestor_count != 1 {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_parent_inheritance_ancestor_count",
            });
        }
        // A nonzero conparentid represents the inherited copy of a partitioned-table constraint.
        // PostgreSQL requires parent NOT NULL constraints to remain inheritable across partitions;
        // accepting NO INHERIT here would turn an impossible partition tuple into governed identity.
        if self.no_inherit {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_parent_no_inherit",
            });
        }
        if self.schema_name == parent_constraint.schema_name
            && self.relation_name == parent_constraint.relation_name
            && self.relation_kind == parent_constraint.relation_kind
            && self.constraint_name == parent_constraint.constraint_name
        {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_parent",
            });
        }
        self.parent_constraint = Some(parent_constraint);
        Ok(self)
    }

    /// Attaches the direct declarative-partition parent resolved from `pg_inherits`.
    ///
    /// This is validation evidence for the relation edge behind `conparentid`. The adapter must pass
    /// the observed `inhdetachpending` bit rather than defaulting it. A detach-pending edge is a
    /// transitional catalog state and fails closed instead of collapsing into a stable partition
    /// membership assertion. The accepted parent coordinate is already carried by the parent-
    /// constraint coordinate and therefore is not hashed twice.
    pub fn with_partition_parent_relation(
        mut self,
        schema_name: impl Into<String>,
        relation_name: impl Into<String>,
        detach_pending: bool,
    ) -> Result<Self, ObservationError> {
        let schema_name = schema_name.into();
        let relation_name = relation_name.into();
        crate::model::validate_nonblank(
            &schema_name,
            "not_null_constraint_partition_parent_schema_name",
        )?;
        crate::model::validate_nonblank(
            &relation_name,
            "not_null_constraint_partition_parent_relation_name",
        )?;
        if detach_pending {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_partition_parent_detach_pending",
            });
        }
        self.partition_parent_relation = Some((schema_name, relation_name));
        Ok(self)
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

    /// Returns the exact observed relation kind.
    #[must_use]
    pub const fn relation_kind(&self) -> RelationKind {
        self.relation_kind
    }

    /// Returns the exact source constraint identifier.
    #[must_use]
    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    /// Returns the exact constrained source column identifier.
    #[must_use]
    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    /// Returns PostgreSQL `convalidated` state.
    #[must_use]
    pub const fn validated(&self) -> bool {
        self.validated
    }

    /// Returns PostgreSQL `conenforced` state.
    #[must_use]
    pub const fn enforced(&self) -> bool {
        self.enforced
    }

    /// Returns PostgreSQL `conislocal` state.
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }

    /// Returns PostgreSQL `coninhcount` as a nonnegative ancestor count.
    #[must_use]
    pub const fn inheritance_ancestor_count(&self) -> u16 {
        self.inheritance_ancestor_count
    }

    /// Returns PostgreSQL `connoinherit` state.
    #[must_use]
    pub const fn no_inherit(&self) -> bool {
        self.no_inherit
    }

    /// Returns the resolved partition-parent constraint coordinate, when PostgreSQL reports one.
    #[must_use]
    pub const fn parent_constraint(&self) -> Option<&ParentNotNullConstraintCoordinate> {
        self.parent_constraint.as_ref()
    }

    fn partition_parent_relation(&self) -> Option<(&str, &str)> {
        self.partition_parent_relation
            .as_ref()
            .map(|(schema_name, relation_name)| (schema_name.as_str(), relation_name.as_str()))
    }
}

pub(crate) fn canonicalize_not_null_constraints(
    relations: &[RelationObservation],
    mut constraints: Vec<NotNullConstraintObservation>,
) -> Result<Vec<NotNullConstraintObservation>, ObservationError> {
    constraints.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.column_name(),
            left.constraint_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.column_name(),
                right.constraint_name(),
            ))
    });

    for pair in constraints.windows(2) {
        if same_column_coordinate(&pair[0], &pair[1]) {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_column",
            });
        }
    }

    let mut relation_constraint_names = BTreeSet::new();
    for observation in &constraints {
        if !relation_constraint_names.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.constraint_name().to_owned(),
        )) {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_name",
            });
        }
    }

    let expected_columns = relations
        .iter()
        .flat_map(|relation| {
            relation
                .columns()
                .iter()
                .filter(|column| !column.nullable())
                .map(move |column| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        column.column_name().to_owned(),
                    )
                })
        })
        .collect::<BTreeSet<_>>();
    let mut observed_columns = BTreeSet::new();

    for observation in &constraints {
        let relation = relations.iter().find(|relation| {
            relation.schema_name() == observation.schema_name()
                && relation.relation_name() == observation.relation_name()
                && relation.kind() == observation.relation_kind()
        });
        let Some(relation) = relation else {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_coordinate",
            });
        };
        let column = relation
            .columns()
            .iter()
            .find(|column| column.column_name() == observation.column_name());
        let Some(column) = column else {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_coordinate",
            });
        };
        if column.nullable() {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_nullability",
            });
        }

        if let Some(parent) = observation.parent_constraint() {
            let Some((partition_parent_schema_name, partition_parent_relation_name)) =
                observation.partition_parent_relation()
            else {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_partition_parent_relation",
                });
            };
            if partition_parent_schema_name != parent.schema_name()
                || partition_parent_relation_name != parent.relation_name()
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_partition_parent_relation",
                });
            }

            let parent_observation = constraints.iter().find(|candidate| {
                candidate.schema_name() == parent.schema_name()
                    && candidate.relation_name() == parent.relation_name()
                    && candidate.relation_kind() == parent.relation_kind()
                    && candidate.constraint_name() == parent.constraint_name()
            });
            let Some(parent_observation) = parent_observation else {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_parent_coordinate",
                });
            };
            if parent_observation.column_name() != observation.column_name() {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_parent_column",
                });
            }
            // PostgreSQL 18 AdjustNotNullInheritance() refuses to attach a valid inherited parent
            // to an existing NOT VALID child constraint. The inverse is allowed: a child that is
            // already valid may satisfy an inherited parent that remains NOT VALID.
            if parent_observation.validated() && !observation.validated() {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_parent_validation",
                });
            }
        } else if observation.partition_parent_relation().is_some() {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_partition_parent_relation",
            });
        }

        observed_columns.insert((
            observation.schema_name().to_owned(),
            observation.relation_name().to_owned(),
            observation.relation_kind().token().to_owned(),
            observation.column_name().to_owned(),
        ));
    }

    validate_parent_constraint_acyclicity(&constraints)?;

    if observed_columns != expected_columns {
        return Err(ObservationError::InvalidObservationField {
            field: "not_null_constraint_completeness",
        });
    }

    Ok(constraints)
}

fn same_column_coordinate(
    left: &NotNullConstraintObservation,
    right: &NotNullConstraintObservation,
) -> bool {
    left.schema_name() == right.schema_name()
        && left.relation_name() == right.relation_name()
        && left.relation_kind() == right.relation_kind()
        && left.column_name() == right.column_name()
}

fn validate_parent_constraint_acyclicity(
    constraints: &[NotNullConstraintObservation],
) -> Result<(), ObservationError> {
    let constraint_index = constraints
        .iter()
        .enumerate()
        .map(|(index, constraint)| {
            (
                (
                    constraint.schema_name(),
                    constraint.relation_name(),
                    constraint.relation_kind().token(),
                    constraint.constraint_name(),
                ),
                index,
            )
        })
        .collect::<BTreeMap<_, _>>();

    for start_index in 0..constraints.len() {
        let mut visited = BTreeSet::new();
        let mut current_index = start_index;
        loop {
            if !visited.insert(current_index) {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_parent_cycle",
                });
            }

            let current = &constraints[current_index];
            let Some(parent) = current.parent_constraint() else {
                break;
            };
            let parent_key = (
                parent.schema_name(),
                parent.relation_name(),
                parent.relation_kind().token(),
                parent.constraint_name(),
            );
            let Some(parent_index) = constraint_index.get(&parent_key) else {
                return Err(ObservationError::InvalidObservationField {
                    field: "not_null_constraint_parent_coordinate",
                });
            };
            current_index = *parent_index;
        }
    }

    Ok(())
}

pub(crate) fn compute_not_null_constraint_digest(
    base_snapshot_digest: &str,
    constraints: &[NotNullConstraintObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        SNAPSHOT_DIGEST_DOMAIN_V3_NOT_NULL_CONSTRAINT_V1,
    );
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, constraints.len());
    for observation in constraints {
        encode_str(&mut hasher, observation.schema_name());
        encode_str(&mut hasher, observation.relation_name());
        encode_str(&mut hasher, observation.relation_kind().token());
        encode_str(&mut hasher, observation.constraint_name());
        encode_str(&mut hasher, observation.column_name());
        hasher.update([
            u8::from(observation.validated()),
            u8::from(observation.enforced()),
            u8::from(observation.is_local()),
        ]);
        hasher.update(observation.inheritance_ancestor_count().to_be_bytes());
        hasher.update([u8::from(observation.no_inherit())]);
        match observation.parent_constraint() {
            Some(parent) => {
                hasher.update([1]);
                encode_str(&mut hasher, parent.schema_name());
                encode_str(&mut hasher, parent.relation_name());
                encode_str(&mut hasher, parent.relation_kind().token());
                encode_str(&mut hasher, parent.constraint_name());
            }
            None => hasher.update([0]),
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
