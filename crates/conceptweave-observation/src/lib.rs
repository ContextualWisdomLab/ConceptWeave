//! Immutable PostgreSQL schema-observation contracts for ConceptWeave.
//!
//! The public aggregate derives source-content identity from deterministic observed metadata.
//! Source connection, connection-policy revision, extractor revision, and observation time remain
//! separate provenance coordinates and therefore do not change the source-content digest.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod array_type;
mod constraint_period;
mod constraint_timing;
mod model;
mod representation_v3;
mod type_kind;

pub use array_type::{ArrayTypeLocation, ArrayTypeObservation, ArrayTypeSourceReceipt};
pub use constraint_period::ConstraintPeriodObservation;
pub use constraint_timing::{ConstraintDeferrability, ConstraintTimingObservation};
pub use model::{
    CheckConstraintObservation, ColumnObservation, ForeignKeyAction, ForeignKeyDeferrability,
    ForeignKeyMatchType, ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationError,
    ObservationLocation, ObservationLocationKind, PrimaryKeyObservation,
    TableConstraintObservation, TableObservation, UniqueConstraintObservation,
};
pub use representation_v3::{
    ColumnObservationV3, DomainCheckConstraintObservation, DomainObservation, EnumObservation,
    IndexAttributeKind, IndexAttributeObservation, IndexAttributeSource, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, IndexStorageOption, IndexTablespace, OperatorClassOption,
    QualifiedCollationName, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation, SchemaObjectLocation, SchemaObjectLocationKind,
};
pub use type_kind::{PostgresTypeKind, TypeKindObservation};

use std::collections::BTreeSet;

use conceptweave_source_port::AuthorizedObservationRequest;
use sha2::{Digest, Sha256};

const SNAPSHOT_DIGEST_DOMAIN_V2: &[u8] = b"conceptweave.postgres_schema_snapshot.v2";
const SNAPSHOT_DIGEST_DOMAIN_V3_ARRAY_TYPES_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.array_types.v1";
const SNAPSHOT_DIGEST_DOMAIN_V3_TYPE_KINDS_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.type_kinds.v1";
const SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_TIMINGS_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.constraint_timings.v1";
const SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_PERIODS_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.constraint_periods.v1";
const POSTGRES_CATALOG_SCHEMA_NAME: &str = "pg_catalog";

/// Immutable receipt binding one exact successor source coordinate to snapshot provenance.
///
/// The receipt always carries the public aggregate's governed source digest. This matters for
/// array-aware v3 snapshots because the private representation remains the compatibility validator
/// for the original v3 constructor while the public owner adds domain-separated identity extensions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SuccessorSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: SchemaObjectLocation,
}

impl SuccessorSourceReceipt {
    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the opaque immutable connection-policy revision used for this observation.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the immutable canonical successor snapshot digest.
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

    /// Returns the verified exact successor source coordinate inside the snapshot.
    #[must_use]
    pub const fn location(&self) -> &SchemaObjectLocation {
        &self.location
    }
}

/// Public v3 aggregate enforcing PostgreSQL schema-local relation and type invariants.
///
/// The representation module remains an implementation detail. This owner-level aggregate validates
/// PostgreSQL's unique `(relname, relnamespace)` catalog namespace, relation-kind ownership rules,
/// exact schema-local `pg_type` identity, optional observed true-array and type-kind relationships,
/// optional PRIMARY KEY/UNIQUE timing evidence, and optional explicit temporal-constraint state
/// before exposing immutable governed evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSchemaSnapshotV3 {
    inner: representation_v3::PostgresSchemaSnapshotV3,
    snapshot_digest: String,
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
    array_types: Vec<ArrayTypeObservation>,
    array_types_observed: bool,
    type_kinds: Vec<TypeKindObservation>,
    type_kinds_observed: bool,
    constraint_timings: Vec<ConstraintTimingObservation>,
    constraint_timings_observed: bool,
    constraint_periods: Vec<ConstraintPeriodObservation>,
    constraint_periods_observed: bool,
}

impl PostgresSchemaSnapshotV3 {
    /// Creates the original deterministic v3 snapshot without claiming true-array, type-kind,
    /// key-constraint timing, or temporal-constraint inventory.
    ///
    /// This constructor deliberately preserves its existing digest contract. Use the explicit
    /// observed-family constructors or consuming family methods only when the adapter captured those
    /// catalog families.
    pub fn new(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
    ) -> Result<Self, ObservationError> {
        validate_schema_relation_invariants(&relations)?;
        let inner = representation_v3::PostgresSchemaSnapshotV3::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )?;
        let snapshot_digest = inner.snapshot_digest().to_owned();
        let relations = inner.relations().to_vec();
        let domains = inner.domains().to_vec();
        let enums = inner.enums().to_vec();
        Ok(Self {
            inner,
            snapshot_digest,
            relations,
            domains,
            enums,
            array_types: Vec::new(),
            array_types_observed: false,
            type_kinds: Vec::new(),
            type_kinds_observed: false,
            constraint_timings: Vec::new(),
            constraint_timings_observed: false,
            constraint_periods: Vec::new(),
            constraint_periods_observed: false,
        })
    }

    /// Creates a deterministic v3 snapshot with explicitly observed PostgreSQL type-kind evidence.
    ///
    /// This constructor is the compatibility seam for direct user-defined range, multirange, and
    /// base-type coordinates. Public evidence retains the exact source binding while the private
    /// legacy v3 validator receives a bounded projection for type kinds it did not originally model.
    /// The original binding, direct `pg_type.typtype`, domain base, and `pg_range` reciprocity are all
    /// bound into a separate successor digest domain before any temporal semantics can be admitted.
    pub fn new_with_type_kinds(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        mut relations: Vec<RelationObservation>,
        mut domains: Vec<DomainObservation>,
        mut enums: Vec<EnumObservation>,
        type_kinds: Vec<TypeKindObservation>,
    ) -> Result<Self, ObservationError> {
        validate_schema_relation_invariants(&relations)?;
        let type_kinds =
            canonicalize_type_kind_observations(&relations, &domains, &enums, type_kinds)?;
        validate_type_bindings_with_type_kinds(&relations, &domains, &enums, &type_kinds)?;

        let projected_relations = relations
            .iter()
            .map(|relation| project_relation_type_kind_bindings(relation, &type_kinds))
            .collect::<Result<Vec<_>, _>>()?;
        let projected_domains = domains
            .iter()
            .map(|domain| project_domain_type_kind_binding(domain, &type_kinds))
            .collect::<Result<Vec<_>, _>>()?;

        let extractor_revision = extractor_revision.into();
        let observed_at_utc = observed_at_utc.into();
        let inner = representation_v3::PostgresSchemaSnapshotV3::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            projected_relations,
            projected_domains,
            enums.clone(),
        )?;

        relations.sort_by(|left, right| {
            (left.schema_name(), left.relation_name())
                .cmp(&(right.schema_name(), right.relation_name()))
        });
        domains.sort_by(|left, right| {
            (left.schema_name(), left.domain_name())
                .cmp(&(right.schema_name(), right.domain_name()))
        });
        enums.sort_by(|left, right| {
            (left.schema_name(), left.enum_name()).cmp(&(right.schema_name(), right.enum_name()))
        });
        let snapshot_digest = compute_type_kind_aware_snapshot_digest(
            inner.snapshot_digest(),
            &relations,
            &domains,
            &type_kinds,
        );
        Ok(Self {
            inner,
            snapshot_digest,
            relations,
            domains,
            enums,
            array_types: Vec::new(),
            array_types_observed: false,
            type_kinds,
            type_kinds_observed: true,
            constraint_timings: Vec::new(),
            constraint_timings_observed: false,
            constraint_periods: Vec::new(),
            constraint_periods_observed: false,
        })
    }

    /// Creates a type-kind-aware v3 snapshot with explicit PRIMARY KEY/UNIQUE timing evidence.
    pub fn new_with_type_kinds_and_constraint_timings(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        type_kinds: Vec<TypeKindObservation>,
        constraint_timings: Vec<ConstraintTimingObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new_with_type_kinds(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
            type_kinds,
        )?
        .with_observed_constraint_timings(constraint_timings)
    }

    /// Creates a deterministic v3 snapshot with explicitly observed PRIMARY KEY/UNIQUE timing.
    ///
    /// The observed timing family is validated against exact relation and constraint coordinates and
    /// is framed behind its own digest domain. An empty timing vector therefore means observed-empty,
    /// not unobserved, and is valid only when the snapshot contains no PRIMARY KEY or UNIQUE
    /// constraints. The legacy [`Self::new`] digest remains unchanged.
    pub fn new_with_constraint_timings(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        constraint_timings: Vec<ConstraintTimingObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )?
        .with_observed_constraint_timings(constraint_timings)
    }

    /// Creates a deterministic v3 snapshot with explicitly observed PostgreSQL true-array identity.
    ///
    /// Array names are accepted only as exact catalog coordinates; no underscore convention,
    /// `search_path`, OID, or display text is used as identity. The constructor validates the shared
    /// schema-local `pg_type` namespace, exact element resolution, one-element-to-one-true-array
    /// reciprocity, and PostgreSQL's no-array-of-array type-system invariant. Existing v3 source
    /// identity remains untouched: this constructor adds a separate domain-separated digest framing
    /// that also binds the original qualified type references whose private validation projection
    /// resolves through their array element coordinates.
    pub fn new_with_array_types(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        mut relations: Vec<RelationObservation>,
        mut domains: Vec<DomainObservation>,
        mut enums: Vec<EnumObservation>,
        array_types: Vec<ArrayTypeObservation>,
    ) -> Result<Self, ObservationError> {
        validate_schema_relation_invariants(&relations)?;
        let array_types = canonicalize_array_type_observations(
            authorized_request,
            &relations,
            &domains,
            &enums,
            array_types,
        )?;
        validate_type_bindings_with_arrays(&relations, &domains, &enums, &array_types)?;

        let projected_relations = relations
            .iter()
            .map(|relation| project_relation_array_bindings(relation, &array_types))
            .collect::<Result<Vec<_>, _>>()?;
        let projected_domains = domains
            .iter()
            .map(|domain| project_domain_array_binding(domain, &array_types))
            .collect::<Result<Vec<_>, _>>()?;

        let extractor_revision = extractor_revision.into();
        let observed_at_utc = observed_at_utc.into();
        let inner = representation_v3::PostgresSchemaSnapshotV3::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            projected_relations,
            projected_domains,
            enums.clone(),
        )?;

        relations.sort_by(|left, right| {
            (left.schema_name(), left.relation_name())
                .cmp(&(right.schema_name(), right.relation_name()))
        });
        domains.sort_by(|left, right| {
            (left.schema_name(), left.domain_name())
                .cmp(&(right.schema_name(), right.domain_name()))
        });
        enums.sort_by(|left, right| {
            (left.schema_name(), left.enum_name()).cmp(&(right.schema_name(), right.enum_name()))
        });

        let snapshot_digest = compute_array_aware_snapshot_digest(
            inner.snapshot_digest(),
            &relations,
            &domains,
            &array_types,
        );
        Ok(Self {
            inner,
            snapshot_digest,
            relations,
            domains,
            enums,
            array_types,
            array_types_observed: true,
            type_kinds: Vec::new(),
            type_kinds_observed: false,
            constraint_timings: Vec::new(),
            constraint_timings_observed: false,
            constraint_periods: Vec::new(),
            constraint_periods_observed: false,
        })
    }

    /// Creates a deterministic v3 snapshot with both true-array identity and key-constraint timing.
    ///
    /// The array-aware digest is computed first; timing evidence then adds its own domain-separated
    /// layer. This keeps each observed catalog family explicit while supporting one immutable source
    /// snapshot containing both families.
    pub fn new_with_array_types_and_constraint_timings(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        array_types: Vec<ArrayTypeObservation>,
        constraint_timings: Vec<ConstraintTimingObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new_with_array_types(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
            array_types,
        )?
        .with_observed_constraint_timings(constraint_timings)
    }

    /// Adds source-authoritative PostgreSQL type-kind/domain-base/range-pair evidence.
    ///
    /// The family must be attached before constraint timing or period evidence so optional family
    /// order cannot create a second identity for the same source facts. Direct user-defined base,
    /// range, or multirange column bindings that the private compatibility validator cannot represent
    /// must use [`Self::new_with_type_kinds`] instead.
    pub fn with_observed_type_kinds(
        mut self,
        type_kinds: Vec<TypeKindObservation>,
    ) -> Result<Self, ObservationError> {
        if self.type_kinds_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_already_observed",
            });
        }
        if self.constraint_timings_observed || self.constraint_periods_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_observation_order",
            });
        }
        let type_kinds = canonicalize_type_kind_observations(
            &self.relations,
            &self.domains,
            &self.enums,
            type_kinds,
        )?;
        validate_type_bindings_with_type_kinds_and_arrays(
            &self.relations,
            &self.domains,
            &self.enums,
            &self.array_types,
            &type_kinds,
        )?;
        self.snapshot_digest = compute_type_kind_aware_snapshot_digest(
            &self.snapshot_digest,
            &self.relations,
            &self.domains,
            &type_kinds,
        );
        self.type_kinds = type_kinds;
        self.type_kinds_observed = true;
        Ok(self)
    }

    fn with_observed_constraint_timings(
        mut self,
        constraint_timings: Vec<ConstraintTimingObservation>,
    ) -> Result<Self, ObservationError> {
        let constraint_timings =
            canonicalize_constraint_timings(&self.relations, constraint_timings)?;
        self.snapshot_digest =
            compute_constraint_timing_digest(&self.snapshot_digest, &constraint_timings);
        self.constraint_timings = constraint_timings;
        self.constraint_timings_observed = true;
        Ok(self)
    }

    /// Adds one complete explicitly observed `pg_constraint.conperiod` family to this snapshot.
    ///
    /// The family is domain-separated from prior v3 identity and must cover every PRIMARY KEY,
    /// UNIQUE, and FOREIGN KEY constraint in the bounded snapshot. Explicit `false` therefore remains
    /// distinct from an unobserved family. Every `conperiod=true` local final column must resolve,
    /// through source-authoritative observed type-kind/domain-base evidence, to a range or multirange.
    /// PRIMARY KEY/UNIQUE values are cross-checked against any already-observed same-name backing-
    /// index exclusion flags, while those index facts are never used to invent `conperiod`. A PERIOD
    /// foreign key targeting a relation inside the same bounded snapshot must resolve to an explicitly
    /// observed `WITHOUT OVERLAPS`, `NOT DEFERRABLE` key on the referenced columns; referenced-key
    /// timing is never inferred from index shape. This consuming method may be applied only once.
    pub fn with_observed_constraint_periods(
        mut self,
        constraint_periods: Vec<ConstraintPeriodObservation>,
    ) -> Result<Self, ObservationError> {
        if self.constraint_periods_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_period_already_observed",
            });
        }
        let constraint_timings = self
            .constraint_timings_observed
            .then_some(self.constraint_timings.as_slice());
        let type_kinds = self
            .type_kinds_observed
            .then_some(self.type_kinds.as_slice());
        let constraint_periods = canonicalize_constraint_periods(
            &self.relations,
            constraint_timings,
            type_kinds,
            constraint_periods,
        )?;
        self.snapshot_digest =
            compute_constraint_period_digest(&self.snapshot_digest, &constraint_periods);
        self.constraint_periods = constraint_periods;
        self.constraint_periods_observed = true;
        Ok(self)
    }

    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        self.inner.source_connection_key()
    }

    /// Returns the opaque immutable connection-policy revision authorized for this snapshot.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        self.inner.connection_policy_binding()
    }

    /// Returns the owner-computed canonical SHA-256 successor source-content digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        self.inner.extractor_revision()
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        self.inner.observed_at_utc()
    }

    /// Returns qualified relations in deterministic exact-identifier order.
    #[must_use]
    pub fn relations(&self) -> &[RelationObservation] {
        &self.relations
    }

    /// Returns qualified domains in deterministic exact-identifier order.
    #[must_use]
    pub fn domains(&self) -> &[DomainObservation] {
        &self.domains
    }

    /// Returns qualified enums in deterministic exact-identifier order.
    #[must_use]
    pub fn enums(&self) -> &[EnumObservation] {
        &self.enums
    }

    /// Returns explicitly observed true-array relationships, or `None` when that catalog family was
    /// not observed by the constructor.
    #[must_use]
    pub fn array_types(&self) -> Option<&[ArrayTypeObservation]> {
        self.array_types_observed
            .then_some(self.array_types.as_slice())
    }

    /// Returns explicitly observed PostgreSQL `pg_type`/`pg_range` evidence, or `None` when that
    /// family was not observed.
    #[must_use]
    pub fn type_kinds(&self) -> Option<&[TypeKindObservation]> {
        self.type_kinds_observed
            .then_some(self.type_kinds.as_slice())
    }

    /// Returns explicitly observed PRIMARY KEY/UNIQUE timing, or `None` when that catalog family was
    /// not observed by the constructor.
    #[must_use]
    pub fn constraint_timings(&self) -> Option<&[ConstraintTimingObservation]> {
        self.constraint_timings_observed
            .then_some(self.constraint_timings.as_slice())
    }

    /// Returns explicitly observed PostgreSQL temporal-constraint state, or `None` when the
    /// `pg_constraint.conperiod` family was not observed.
    #[must_use]
    pub fn constraint_periods(&self) -> Option<&[ConstraintPeriodObservation]> {
        self.constraint_periods_observed
            .then_some(self.constraint_periods.as_slice())
    }

    /// Issues provenance for an exact successor coordinate only when it exists in this snapshot.
    pub fn source_receipt(
        &self,
        location: SchemaObjectLocation,
    ) -> Result<SuccessorSourceReceipt, ObservationError> {
        let verified = self.inner.source_receipt(location.clone())?;
        Ok(SuccessorSourceReceipt {
            source_id: verified.source_id().to_owned(),
            connection_policy_binding: verified.connection_policy_binding().to_owned(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: verified.extractor_revision().to_owned(),
            observed_at_utc: verified.observed_at_utc().to_owned(),
            location,
        })
    }

    /// Issues provenance for one exact observed true-array coordinate.
    ///
    /// This separate successor seam keeps all pre-array [`SchemaObjectLocation`] meanings frozen.
    /// The receipt is available only when this snapshot explicitly observed the array family and the
    /// requested exact array coordinate exists in that immutable inventory.
    pub fn array_type_source_receipt(
        &self,
        location: ArrayTypeLocation,
    ) -> Result<ArrayTypeSourceReceipt, ObservationError> {
        let exists = self.array_types_observed
            && self.array_types.iter().any(|array_type| {
                array_type.array_type().schema_name() == location.schema_name()
                    && array_type.array_type().type_name() == location.array_type_name()
            });
        if !exists {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(ArrayTypeSourceReceipt::new(
            self.source_connection_key().to_owned(),
            self.connection_policy_binding().to_owned(),
            self.snapshot_digest.clone(),
            self.extractor_revision().to_owned(),
            self.observed_at_utc().to_owned(),
            location,
        ))
    }
}

fn validate_schema_relation_invariants(
    relations: &[RelationObservation],
) -> Result<(), ObservationError> {
    let mut observed_names = BTreeSet::new();
    for relation in relations {
        if !relation.indexes().is_empty()
            && !matches!(
                relation.kind(),
                RelationKind::Table
                    | RelationKind::PartitionedTable
                    | RelationKind::MaterializedView
            )
        {
            return Err(ObservationError::InvalidObservationField {
                field: "index_relation_kind",
            });
        }

        if !relation.constraints().is_empty() {
            let constraints_supported = match relation.kind() {
                RelationKind::Table | RelationKind::PartitionedTable => true,
                RelationKind::ForeignTable => relation
                    .constraints()
                    .iter()
                    .all(|constraint| matches!(constraint, TableConstraintObservation::Check(_))),
                RelationKind::View
                | RelationKind::MaterializedView
                | RelationKind::Sequence
                | RelationKind::CompositeType => false,
            };
            if !constraints_supported {
                return Err(ObservationError::InvalidObservationField {
                    field: "relation_constraint_kind",
                });
            }
        }

        for constraint in relation.constraints() {
            if let TableConstraintObservation::ForeignKey(foreign_key) = constraint
                && foreign_key.reference_behavior().is_some_and(|behavior| {
                    behavior.match_type() == ForeignKeyMatchType::Partial
                })
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "foreign_key_match_type",
                });
            }
        }

        let mut primary_keys = relation.constraints().iter().filter_map(|constraint| {
            if let TableConstraintObservation::PrimaryKey(primary_key) = constraint {
                Some(primary_key)
            } else {
                None
            }
        });
        if let Some(primary_key) = primary_keys.next() {
            if primary_keys.next().is_some() {
                return Err(ObservationError::InvalidObservationField {
                    field: "primary_key_cardinality",
                });
            }
            if primary_key.column_names().iter().any(|column_name| {
                relation
                    .columns()
                    .iter()
                    .any(|column| column.column_name() == column_name && column.nullable())
            }) {
                return Err(ObservationError::InvalidObservationField {
                    field: "primary_key_nullable_column",
                });
            }
        }

        let schema_name = relation.schema_name().to_owned();
        if !observed_names.insert((schema_name.clone(), relation.relation_name().to_owned())) {
            return Err(ObservationError::InvalidObservationField {
                field: "schema_relation_namespace",
            });
        }
        for index in relation.indexes() {
            if !observed_names.insert((schema_name.clone(), index.index_name().to_owned())) {
                return Err(ObservationError::InvalidObservationField {
                    field: "schema_relation_namespace",
                });
            }
        }
    }
    Ok(())
}

fn canonicalize_type_kind_observations(
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    mut type_kinds: Vec<TypeKindObservation>,
) -> Result<Vec<TypeKindObservation>, ObservationError> {
    type_kinds.sort_by(|left, right| {
        (
            left.type_name().schema_name(),
            left.type_name().type_name(),
        )
            .cmp(&(
                right.type_name().schema_name(),
                right.type_name().type_name(),
            ))
    });
    for pair in type_kinds.windows(2) {
        if same_type_coordinate(pair[0].type_name(), pair[1].type_name()) {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_coordinate",
            });
        }
    }

    let observed_schemas = relations
        .iter()
        .map(|relation| relation.schema_name())
        .chain(domains.iter().map(|domain| domain.schema_name()))
        .chain(enums.iter().map(|observed_enum| observed_enum.schema_name()))
        .collect::<BTreeSet<_>>();

    for type_kind in &type_kinds {
        let coordinate = type_kind.type_name();
        if coordinate.schema_name() != POSTGRES_CATALOG_SCHEMA_NAME
            && !observed_schemas.contains(coordinate.schema_name())
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_schema",
            });
        }

        if let Some(domain) = domains.iter().find(|domain| {
            domain.schema_name() == coordinate.schema_name()
                && domain.domain_name() == coordinate.type_name()
        }) {
            if type_kind.kind() != PostgresTypeKind::Domain
                || !type_kind
                    .domain_base_type()
                    .is_some_and(|base| same_type_coordinate(base, domain.base_type()))
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "type_kind_domain_base",
                });
            }
        }
        if enums.iter().any(|observed_enum| {
            observed_enum.schema_name() == coordinate.schema_name()
                && observed_enum.enum_name() == coordinate.type_name()
        }) && type_kind.kind() != PostgresTypeKind::Enum
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_coordinate",
            });
        }
        if relations.iter().any(|relation| {
            relation_has_row_type(relation.kind())
                && relation.schema_name() == coordinate.schema_name()
                && relation.relation_name() == coordinate.type_name()
        }) && type_kind.kind() != PostgresTypeKind::Composite
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_coordinate",
            });
        }

        if matches!(
            type_kind.kind(),
            PostgresTypeKind::Range | PostgresTypeKind::Multirange
        ) {
            let Some(counterpart) = type_kind.range_counterpart() else {
                return Err(ObservationError::InvalidObservationField {
                    field: "type_kind_range_reciprocity",
                });
            };
            let Some(counterpart_observation) = type_kinds
                .iter()
                .find(|candidate| same_type_coordinate(candidate.type_name(), counterpart))
            else {
                return Err(ObservationError::InvalidObservationField {
                    field: "type_kind_range_reciprocity",
                });
            };
            let expected_counterpart_kind = match type_kind.kind() {
                PostgresTypeKind::Range => PostgresTypeKind::Multirange,
                PostgresTypeKind::Multirange => PostgresTypeKind::Range,
                _ => unreachable!("range-kind branch was validated"),
            };
            if counterpart_observation.kind() != expected_counterpart_kind
                || !counterpart_observation
                    .range_counterpart()
                    .is_some_and(|back| same_type_coordinate(back, coordinate))
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "type_kind_range_reciprocity",
                });
            }
        }
    }

    for type_kind in &type_kinds {
        if type_kind.kind() != PostgresTypeKind::Domain {
            continue;
        }
        let mut current = Some(type_kind.type_name());
        let mut visited = BTreeSet::new();
        while let Some(coordinate) = current {
            let key = (
                coordinate.schema_name().to_owned(),
                coordinate.type_name().to_owned(),
            );
            if !visited.insert(key) {
                return Err(ObservationError::InvalidObservationField {
                    field: "type_kind_domain_cycle",
                });
            }
            current = type_kinds
                .iter()
                .find(|candidate| same_type_coordinate(candidate.type_name(), coordinate))
                .and_then(|candidate| {
                    (candidate.kind() == PostgresTypeKind::Domain)
                        .then(|| candidate.domain_base_type())
                        .flatten()
                });
        }
    }

    Ok(type_kinds)
}

fn type_binding_is_resolvable_with_type_kinds(
    binding: &QualifiedTypeName,
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    type_kinds: &[TypeKindObservation],
) -> bool {
    if binding.schema_name() == POSTGRES_CATALOG_SCHEMA_NAME {
        return true;
    }
    domains.iter().any(|domain| {
        domain.schema_name() == binding.schema_name() && domain.domain_name() == binding.type_name()
    }) || enums.iter().any(|observed_enum| {
        observed_enum.schema_name() == binding.schema_name()
            && observed_enum.enum_name() == binding.type_name()
    }) || relations.iter().any(|relation| {
        relation_has_row_type(relation.kind())
            && relation.schema_name() == binding.schema_name()
            && relation.relation_name() == binding.type_name()
    }) || type_kinds.iter().any(|type_kind| {
        same_type_coordinate(type_kind.type_name(), binding)
            && matches!(
                type_kind.kind(),
                PostgresTypeKind::Base
                    | PostgresTypeKind::Range
                    | PostgresTypeKind::Multirange
            )
    })
}

fn validate_type_bindings_with_type_kinds(
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    type_kinds: &[TypeKindObservation],
) -> Result<(), ObservationError> {
    for domain in domains {
        if !type_binding_is_resolvable_with_type_kinds(
            domain.base_type(),
            relations,
            domains,
            enums,
            type_kinds,
        ) {
            return Err(ObservationError::UnknownTypeBinding {
                schema_name: domain.base_type().schema_name().to_owned(),
                type_name: domain.base_type().type_name().to_owned(),
            });
        }
    }
    for relation in relations {
        for column in relation.columns() {
            if !type_binding_is_resolvable_with_type_kinds(
                column.type_binding(),
                relations,
                domains,
                enums,
                type_kinds,
            ) {
                return Err(ObservationError::UnknownTypeBinding {
                    schema_name: column.type_binding().schema_name().to_owned(),
                    type_name: column.type_binding().type_name().to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn type_binding_is_resolvable_with_type_kinds_and_arrays(
    binding: &QualifiedTypeName,
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    array_types: &[ArrayTypeObservation],
    type_kinds: &[TypeKindObservation],
) -> bool {
    array_types
        .iter()
        .any(|array_type| same_type_coordinate(array_type.array_type(), binding))
        || type_binding_is_resolvable_with_type_kinds(
            binding,
            relations,
            domains,
            enums,
            type_kinds,
        )
}

fn validate_type_bindings_with_type_kinds_and_arrays(
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    array_types: &[ArrayTypeObservation],
    type_kinds: &[TypeKindObservation],
) -> Result<(), ObservationError> {
    for domain in domains {
        if !type_binding_is_resolvable_with_type_kinds_and_arrays(
            domain.base_type(),
            relations,
            domains,
            enums,
            array_types,
            type_kinds,
        ) {
            return Err(ObservationError::UnknownTypeBinding {
                schema_name: domain.base_type().schema_name().to_owned(),
                type_name: domain.base_type().type_name().to_owned(),
            });
        }
    }
    for relation in relations {
        for column in relation.columns() {
            if !type_binding_is_resolvable_with_type_kinds_and_arrays(
                column.type_binding(),
                relations,
                domains,
                enums,
                array_types,
                type_kinds,
            ) {
                return Err(ObservationError::UnknownTypeBinding {
                    schema_name: column.type_binding().schema_name().to_owned(),
                    type_name: column.type_binding().type_name().to_owned(),
                });
            }
        }
    }
    Ok(())
}

fn projected_type_kind_binding(
    binding: &QualifiedTypeName,
    type_kinds: &[TypeKindObservation],
) -> QualifiedTypeName {
    let needs_projection = binding.schema_name() != POSTGRES_CATALOG_SCHEMA_NAME
        && type_kinds.iter().any(|type_kind| {
            same_type_coordinate(type_kind.type_name(), binding)
                && matches!(
                    type_kind.kind(),
                    PostgresTypeKind::Base
                        | PostgresTypeKind::Range
                        | PostgresTypeKind::Multirange
                )
        });
    if needs_projection {
        QualifiedTypeName::new(POSTGRES_CATALOG_SCHEMA_NAME, "text")
            .expect("fixed PostgreSQL compatibility projection is valid")
    } else {
        binding.clone()
    }
}

fn project_relation_type_kind_bindings(
    relation: &RelationObservation,
    type_kinds: &[TypeKindObservation],
) -> Result<RelationObservation, ObservationError> {
    let columns = relation
        .columns()
        .iter()
        .map(|column| {
            ColumnObservationV3::new(
                column.column_name(),
                column.ordinal_position(),
                column.data_type(),
                projected_type_kind_binding(column.type_binding(), type_kinds),
                column.nullable(),
                column.source_comment().map(str::to_owned),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let mut projected = RelationObservation::new(
        relation.schema_name(),
        relation.relation_name(),
        relation.kind(),
        columns,
    )?;
    if !relation.constraints().is_empty() {
        projected = projected.with_constraints(relation.constraints().to_vec())?;
    }
    if !relation.indexes().is_empty() {
        projected = projected.with_indexes(relation.indexes().to_vec())?;
    }
    if let Some(source_comment) = relation.source_comment() {
        projected = projected.with_source_comment(source_comment.to_owned());
    }
    Ok(projected)
}

fn project_domain_type_kind_binding(
    domain: &DomainObservation,
    type_kinds: &[TypeKindObservation],
) -> Result<DomainObservation, ObservationError> {
    let mut projected = DomainObservation::new(
        domain.schema_name(),
        domain.domain_name(),
        projected_type_kind_binding(domain.base_type(), type_kinds),
    )?;
    if let Some(type_modifier) = domain.type_modifier() {
        projected = projected.with_type_modifier(type_modifier);
    }
    if let Some(array_dimensions) = domain.array_dimensions() {
        projected = projected.with_array_dimensions(array_dimensions);
    }
    if let Some(collation) = domain.collation() {
        projected = projected.with_collation(collation.clone());
    }
    if let Some(not_null) = domain.not_null() {
        projected = projected.with_not_null(not_null);
    }
    if let Some(default_expression) = domain.default_expression() {
        projected = projected.with_default_expression(default_expression.to_owned());
    }
    if !domain.check_constraints().is_empty() {
        projected = projected.with_check_constraints(domain.check_constraints().to_vec())?;
    }
    if let Some(source_comment) = domain.source_comment() {
        projected = projected.with_source_comment(source_comment.to_owned());
    }
    Ok(projected)
}

fn resolves_to_range_or_multirange(
    binding: &QualifiedTypeName,
    type_kinds: &[TypeKindObservation],
) -> bool {
    let mut current = (
        binding.schema_name().to_owned(),
        binding.type_name().to_owned(),
    );
    let mut visited = BTreeSet::new();
    for _ in 0..=type_kinds.len() {
        if !visited.insert(current.clone()) {
            return false;
        }
        let Some(type_kind) = type_kinds.iter().find(|candidate| {
            candidate.type_name().schema_name() == current.0
                && candidate.type_name().type_name() == current.1
        }) else {
            return false;
        };
        match type_kind.kind() {
            PostgresTypeKind::Range | PostgresTypeKind::Multirange => return true,
            PostgresTypeKind::Domain => {
                let Some(base_type) = type_kind.domain_base_type() else {
                    return false;
                };
                current = (
                    base_type.schema_name().to_owned(),
                    base_type.type_name().to_owned(),
                );
            }
            PostgresTypeKind::Base
            | PostgresTypeKind::Composite
            | PostgresTypeKind::Enum
            | PostgresTypeKind::Pseudo => return false,
        }
    }
    false
}

fn validate_constraint_period_column_type(
    relation: &RelationObservation,
    constraint: &TableConstraintObservation,
    type_kinds: Option<&[TypeKindObservation]>,
) -> Result<(), ObservationError> {
    let Some(final_column_name) = constraint.column_names().last() else {
        return Err(ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        });
    };
    let Some(final_column) = relation
        .columns()
        .iter()
        .find(|column| column.column_name() == final_column_name)
    else {
        return Err(ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        });
    };
    if !type_kinds.is_some_and(|observations| {
        resolves_to_range_or_multirange(final_column.type_binding(), observations)
    }) {
        return Err(ObservationError::InvalidObservationField {
            field: "constraint_period_column_type",
        });
    }
    Ok(())
}

fn canonicalize_constraint_timings(
    relations: &[RelationObservation],
    mut constraint_timings: Vec<ConstraintTimingObservation>,
) -> Result<Vec<ConstraintTimingObservation>, ObservationError> {
    constraint_timings.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.constraint_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.constraint_name(),
            ))
    });

    for pair in constraint_timings.windows(2) {
        if pair[0].schema_name() == pair[1].schema_name()
            && pair[0].relation_name() == pair[1].relation_name()
            && pair[0].relation_kind() == pair[1].relation_kind()
            && pair[0].constraint_name() == pair[1].constraint_name()
        {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_timing_coordinate",
            });
        }
    }

    let expected_key_coordinates = relations
        .iter()
        .flat_map(|relation| {
            relation.constraints().iter().filter_map(move |constraint| {
                matches!(
                    constraint,
                    TableConstraintObservation::PrimaryKey(_)
                        | TableConstraintObservation::Unique(_)
                )
                .then(|| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        constraint.constraint_name().to_owned(),
                    )
                })
            })
        })
        .collect::<BTreeSet<_>>();
    let mut observed_key_coordinates = BTreeSet::new();

    for timing in &constraint_timings {
        let Some(relation) = relations.iter().find(|relation| {
            relation.schema_name() == timing.schema_name()
                && relation.relation_name() == timing.relation_name()
                && relation.kind() == timing.relation_kind()
        }) else {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_timing_coordinate",
            });
        };
        let Some(constraint) = relation
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == timing.constraint_name())
        else {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_timing_coordinate",
            });
        };
        if !matches!(
            constraint,
            TableConstraintObservation::PrimaryKey(_) | TableConstraintObservation::Unique(_)
        ) {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_timing_kind",
            });
        }

        let Some(backing_index) = relation
            .indexes()
            .iter()
            .find(|index| index.index_name() == timing.constraint_name())
        else {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_backing_index",
            });
        };
        let Some(catalog_flags) = backing_index.catalog_flags() else {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_backing_index",
            });
        };
        let (constraint_columns, expected_nulls_not_distinct) = match constraint {
            TableConstraintObservation::PrimaryKey(primary_key) => {
                (primary_key.column_names(), None)
            }
            TableConstraintObservation::Unique(unique) => {
                (unique.column_names(), unique.nulls_not_distinct())
            }
            TableConstraintObservation::ForeignKey(_) | TableConstraintObservation::Check(_) => {
                unreachable!("key-constraint kind was validated above")
            }
        };
        let key_columns_match = backing_index.key_attributes().len() == constraint_columns.len()
            && backing_index
                .key_attributes()
                .iter()
                .zip(constraint_columns)
                .all(|(attribute, column_name)| {
                    attribute.attribute_name() == Some(column_name.as_str())
                });
        let null_treatment_matches = expected_nulls_not_distinct
            .is_none_or(|expected| backing_index.nulls_not_distinct() == Some(expected));
        let expected_primary = matches!(constraint, TableConstraintObservation::PrimaryKey(_));
        let expected_immediate = matches!(
            timing.deferrability(),
            ConstraintDeferrability::NotDeferrable
        );
        let exclusion_access_method_matches =
            !catalog_flags.exclusion() || backing_index.access_method() == Some("gist");
        if !backing_index.is_unique()
            || !key_columns_match
            || backing_index.predicate().is_some()
            || !null_treatment_matches
            || !exclusion_access_method_matches
            || catalog_flags.primary() != expected_primary
            || catalog_flags.immediate() != expected_immediate
        {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_backing_index",
            });
        }

        observed_key_coordinates.insert((
            timing.schema_name().to_owned(),
            timing.relation_name().to_owned(),
            timing.relation_kind().token().to_owned(),
            timing.constraint_name().to_owned(),
        ));
    }

    if observed_key_coordinates != expected_key_coordinates {
        return Err(ObservationError::InvalidObservationField {
            field: "constraint_timing_completeness",
        });
    }

    Ok(constraint_timings)
}

fn canonicalize_constraint_periods(
    relations: &[RelationObservation],
    constraint_timings: Option<&[ConstraintTimingObservation]>,
    type_kinds: Option<&[TypeKindObservation]>,
    mut constraint_periods: Vec<ConstraintPeriodObservation>,
) -> Result<Vec<ConstraintPeriodObservation>, ObservationError> {
    constraint_periods.sort_by(|left, right| {
        (
            left.schema_name(),
            left.relation_name(),
            left.relation_kind().token(),
            left.constraint_name(),
        )
            .cmp(&(
                right.schema_name(),
                right.relation_name(),
                right.relation_kind().token(),
                right.constraint_name(),
            ))
    });

    for pair in constraint_periods.windows(2) {
        if pair[0].schema_name() == pair[1].schema_name()
            && pair[0].relation_name() == pair[1].relation_name()
            && pair[0].relation_kind() == pair[1].relation_kind()
            && pair[0].constraint_name() == pair[1].constraint_name()
        {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_period_coordinate",
            });
        }
    }

    let expected_period_coordinates = relations
        .iter()
        .flat_map(|relation| {
            relation.constraints().iter().filter_map(move |constraint| {
                matches!(
                    constraint,
                    TableConstraintObservation::PrimaryKey(_)
                        | TableConstraintObservation::Unique(_)
                        | TableConstraintObservation::ForeignKey(_)
                )
                .then(|| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        constraint.constraint_name().to_owned(),
                    )
                })
            })
        })
        .collect::<BTreeSet<_>>();
    let mut observed_period_coordinates = BTreeSet::new();

    for period in &constraint_periods {
        let Some(relation) = relations.iter().find(|relation| {
            relation.schema_name() == period.schema_name()
                && relation.relation_name() == period.relation_name()
                && relation.kind() == period.relation_kind()
        }) else {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_period_coordinate",
            });
        };
        let Some(constraint) = relation
            .constraints()
            .iter()
            .find(|constraint| constraint.constraint_name() == period.constraint_name())
        else {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_period_coordinate",
            });
        };

        match constraint {
            TableConstraintObservation::PrimaryKey(_) | TableConstraintObservation::Unique(_) => {
                if period.has_period_semantics() {
                    validate_constraint_period_column_type(relation, constraint, type_kinds)?;
                }
                if let Some(backing_index) = relation
                    .indexes()
                    .iter()
                    .find(|index| index.index_name() == period.constraint_name())
                    && let Some(catalog_flags) = backing_index.catalog_flags()
                {
                    let period_matches_exclusion =
                        catalog_flags.exclusion() == period.has_period_semantics();
                    let temporal_access_method_matches = !period.has_period_semantics()
                        || backing_index.access_method() == Some("gist");
                    if !period_matches_exclusion || !temporal_access_method_matches {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_backing_index",
                        });
                    }
                }
            }
            TableConstraintObservation::ForeignKey(foreign_key) => {
                if period.has_period_semantics() {
                    if foreign_key.column_names().len() < 2 {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_shape",
                        });
                    }
                    validate_constraint_period_column_type(relation, constraint, type_kinds)?;
                    let Some(reference_behavior) = foreign_key.reference_behavior() else {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_action",
                        });
                    };
                    if reference_behavior.update_action() != ForeignKeyAction::NoAction
                        || reference_behavior.delete_action() != ForeignKeyAction::NoAction
                    {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_action",
                        });
                    }
                    if let Some(referenced_relation) = relations.iter().find(|candidate| {
                        candidate.schema_name() == foreign_key.referenced_schema_name()
                            && candidate.relation_name() == foreign_key.referenced_table_name()
                    }) {
                        let referenced_temporal_key = referenced_relation.constraints().iter().find(
                            |candidate_constraint| {
                                matches!(
                                    candidate_constraint,
                                    TableConstraintObservation::PrimaryKey(_)
                                        | TableConstraintObservation::Unique(_)
                                ) && candidate_constraint.column_names()
                                    == foreign_key.referenced_column_names()
                                    && constraint_periods.iter().any(|candidate_period| {
                                        candidate_period.schema_name()
                                            == referenced_relation.schema_name()
                                            && candidate_period.relation_name()
                                                == referenced_relation.relation_name()
                                            && candidate_period.relation_kind()
                                                == referenced_relation.kind()
                                            && candidate_period.constraint_name()
                                                == candidate_constraint.constraint_name()
                                            && candidate_period.has_period_semantics()
                                    })
                            },
                        );
                        let Some(referenced_temporal_key) = referenced_temporal_key else {
                            return Err(ObservationError::InvalidObservationField {
                                field: "constraint_period_reference",
                            });
                        };
                        let referenced_key_is_nondeferrable = constraint_timings
                            .and_then(|timings| {
                                timings.iter().find(|timing| {
                                    timing.schema_name() == referenced_relation.schema_name()
                                        && timing.relation_name()
                                            == referenced_relation.relation_name()
                                        && timing.relation_kind() == referenced_relation.kind()
                                        && timing.constraint_name()
                                            == referenced_temporal_key.constraint_name()
                                })
                            })
                            .is_some_and(|timing| {
                                timing.deferrability() == ConstraintDeferrability::NotDeferrable
                            });
                        if !referenced_key_is_nondeferrable {
                            return Err(ObservationError::InvalidObservationField {
                                field: "constraint_period_reference_timing",
                            });
                        }
                    }
                }
            }
            TableConstraintObservation::Check(_) => {
                return Err(ObservationError::InvalidObservationField {
                    field: "constraint_period_kind",
                });
            }
        }

        observed_period_coordinates.insert((
            period.schema_name().to_owned(),
            period.relation_name().to_owned(),
            period.relation_kind().token().to_owned(),
            period.constraint_name().to_owned(),
        ));
    }

    if observed_period_coordinates != expected_period_coordinates {
        return Err(ObservationError::InvalidObservationField {
            field: "constraint_period_completeness",
        });
    }

    Ok(constraint_periods)
}

fn relation_has_row_type(kind: RelationKind) -> bool {
    !matches!(kind, RelationKind::Sequence)
}

fn canonicalize_array_type_observations(
    authorized_request: &AuthorizedObservationRequest,
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    mut array_types: Vec<ArrayTypeObservation>,
) -> Result<Vec<ArrayTypeObservation>, ObservationError> {
    let mut scalar_type_names = BTreeSet::new();
    for domain in domains {
        scalar_type_names.insert((
            domain.schema_name().to_owned(),
            domain.domain_name().to_owned(),
        ));
    }
    for observed_enum in enums {
        scalar_type_names.insert((
            observed_enum.schema_name().to_owned(),
            observed_enum.enum_name().to_owned(),
        ));
    }
    for relation in relations {
        if relation_has_row_type(relation.kind()) {
            scalar_type_names.insert((
                relation.schema_name().to_owned(),
                relation.relation_name().to_owned(),
            ));
        }
    }

    array_types.sort_by(|left, right| {
        (
            left.array_type().schema_name(),
            left.array_type().type_name(),
            left.element_type().schema_name(),
            left.element_type().type_name(),
        )
            .cmp(&(
                right.array_type().schema_name(),
                right.array_type().type_name(),
                right.element_type().schema_name(),
                right.element_type().type_name(),
            ))
    });

    for pair in array_types.windows(2) {
        if same_type_coordinate(pair[0].array_type(), pair[1].array_type()) {
            return Err(ObservationError::InvalidObservationField {
                field: "array_type_coordinate",
            });
        }
    }

    let array_type_names = array_types
        .iter()
        .map(|array_type| {
            (
                array_type.array_type().schema_name().to_owned(),
                array_type.array_type().type_name().to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let mut observed_elements = BTreeSet::new();

    for array_type in &array_types {
        let array_coordinate = (
            array_type.array_type().schema_name().to_owned(),
            array_type.array_type().type_name().to_owned(),
        );
        let element_coordinate = (
            array_type.element_type().schema_name().to_owned(),
            array_type.element_type().type_name().to_owned(),
        );

        if !authorized_request
            .request()
            .allowed_schema_names()
            .iter()
            .any(|schema_name| schema_name == array_type.array_type().schema_name())
        {
            return Err(ObservationError::InvalidObservationField {
                field: "unauthorized_schema_name",
            });
        }
        if scalar_type_names.contains(&array_coordinate) {
            return Err(ObservationError::InvalidObservationField {
                field: "array_type_namespace",
            });
        }
        if !observed_elements.insert(element_coordinate.clone()) {
            return Err(ObservationError::InvalidObservationField {
                field: "array_type_reciprocity",
            });
        }
        if array_type_names.contains(&element_coordinate) {
            return Err(ObservationError::InvalidObservationField {
                field: "array_type_element",
            });
        }
        if array_type.element_type().schema_name() != POSTGRES_CATALOG_SCHEMA_NAME
            && !scalar_type_names.contains(&element_coordinate)
        {
            return Err(ObservationError::UnknownTypeBinding {
                schema_name: element_coordinate.0,
                type_name: element_coordinate.1,
            });
        }
    }

    Ok(array_types)
}

fn validate_type_bindings_with_arrays(
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    array_types: &[ArrayTypeObservation],
) -> Result<(), ObservationError> {
    let mut resolvable = BTreeSet::new();
    for domain in domains {
        resolvable.insert((
            domain.schema_name().to_owned(),
            domain.domain_name().to_owned(),
        ));
    }
    for observed_enum in enums {
        resolvable.insert((
            observed_enum.schema_name().to_owned(),
            observed_enum.enum_name().to_owned(),
        ));
    }
    for relation in relations {
        if relation_has_row_type(relation.kind()) {
            resolvable.insert((
                relation.schema_name().to_owned(),
                relation.relation_name().to_owned(),
            ));
        }
    }
    for array_type in array_types {
        resolvable.insert((
            array_type.array_type().schema_name().to_owned(),
            array_type.array_type().type_name().to_owned(),
        ));
    }

    let validate_binding = |binding: &QualifiedTypeName| -> Result<(), ObservationError> {
        if binding.schema_name() == POSTGRES_CATALOG_SCHEMA_NAME
            || resolvable.contains(&(
                binding.schema_name().to_owned(),
                binding.type_name().to_owned(),
            ))
        {
            Ok(())
        } else {
            Err(ObservationError::UnknownTypeBinding {
                schema_name: binding.schema_name().to_owned(),
                type_name: binding.type_name().to_owned(),
            })
        }
    };

    for domain in domains {
        validate_binding(domain.base_type())?;
    }
    for relation in relations {
        for column in relation.columns() {
            validate_binding(column.type_binding())?;
        }
    }
    Ok(())
}

fn same_type_coordinate(left: &QualifiedTypeName, right: &QualifiedTypeName) -> bool {
    left.schema_name() == right.schema_name() && left.type_name() == right.type_name()
}

fn projected_type_binding(
    binding: &QualifiedTypeName,
    array_types: &[ArrayTypeObservation],
) -> QualifiedTypeName {
    array_types
        .iter()
        .find(|array_type| same_type_coordinate(array_type.array_type(), binding))
        .map_or_else(
            || binding.clone(),
            |array_type| array_type.element_type().clone(),
        )
}

fn project_relation_array_bindings(
    relation: &RelationObservation,
    array_types: &[ArrayTypeObservation],
) -> Result<RelationObservation, ObservationError> {
    let columns = relation
        .columns()
        .iter()
        .map(|column| {
            ColumnObservationV3::new(
                column.column_name(),
                column.ordinal_position(),
                column.data_type(),
                projected_type_binding(column.type_binding(), array_types),
                column.nullable(),
                column.source_comment().map(str::to_owned),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut projected = RelationObservation::new(
        relation.schema_name(),
        relation.relation_name(),
        relation.kind(),
        columns,
    )?;
    if !relation.constraints().is_empty() {
        projected = projected.with_constraints(relation.constraints().to_vec())?;
    }
    if !relation.indexes().is_empty() {
        projected = projected.with_indexes(relation.indexes().to_vec())?;
    }
    if let Some(source_comment) = relation.source_comment() {
        projected = projected.with_source_comment(source_comment.to_owned());
    }
    Ok(projected)
}

fn project_domain_array_binding(
    domain: &DomainObservation,
    array_types: &[ArrayTypeObservation],
) -> Result<DomainObservation, ObservationError> {
    let mut projected = DomainObservation::new(
        domain.schema_name(),
        domain.domain_name(),
        projected_type_binding(domain.base_type(), array_types),
    )?;
    if let Some(type_modifier) = domain.type_modifier() {
        projected = projected.with_type_modifier(type_modifier);
    }
    if let Some(array_dimensions) = domain.array_dimensions() {
        projected = projected.with_array_dimensions(array_dimensions);
    }
    if let Some(collation) = domain.collation() {
        projected = projected.with_collation(collation.clone());
    }
    if let Some(not_null) = domain.not_null() {
        projected = projected.with_not_null(not_null);
    }
    if let Some(default_expression) = domain.default_expression() {
        projected = projected.with_default_expression(default_expression.to_owned());
    }
    if !domain.check_constraints().is_empty() {
        projected = projected.with_check_constraints(domain.check_constraints().to_vec())?;
    }
    if let Some(source_comment) = domain.source_comment() {
        projected = projected.with_source_comment(source_comment.to_owned());
    }
    Ok(projected)
}

fn compute_array_aware_snapshot_digest(
    base_snapshot_digest: &str,
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    array_types: &[ArrayTypeObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V3_ARRAY_TYPES_V1);
    encode_str(&mut hasher, base_snapshot_digest);

    encode_len(&mut hasher, array_types.len());
    for array_type in array_types {
        encode_str(&mut hasher, array_type.array_type().schema_name());
        encode_str(&mut hasher, array_type.array_type().type_name());
        encode_str(&mut hasher, array_type.element_type().schema_name());
        encode_str(&mut hasher, array_type.element_type().type_name());
    }

    encode_len(&mut hasher, domains.len());
    for domain in domains {
        encode_str(&mut hasher, domain.schema_name());
        encode_str(&mut hasher, domain.domain_name());
        encode_str(&mut hasher, domain.base_type().schema_name());
        encode_str(&mut hasher, domain.base_type().type_name());
    }

    encode_len(&mut hasher, relations.len());
    for relation in relations {
        encode_str(&mut hasher, relation.schema_name());
        encode_str(&mut hasher, relation.relation_name());
        encode_str(&mut hasher, relation.kind().token());
        encode_len(&mut hasher, relation.columns().len());
        for column in relation.columns() {
            encode_str(&mut hasher, column.column_name());
            hasher.update(column.ordinal_position().to_be_bytes());
            encode_str(&mut hasher, column.type_binding().schema_name());
            encode_str(&mut hasher, column.type_binding().type_name());
        }
    }

    encode_sha256(hasher)
}

fn compute_type_kind_aware_snapshot_digest(
    base_snapshot_digest: &str,
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    type_kinds: &[TypeKindObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V3_TYPE_KINDS_V1);
    encode_str(&mut hasher, base_snapshot_digest);

    encode_len(&mut hasher, type_kinds.len());
    for type_kind in type_kinds {
        encode_str(&mut hasher, type_kind.type_name().schema_name());
        encode_str(&mut hasher, type_kind.type_name().type_name());
        hasher.update([type_kind.kind().tag()]);
        match type_kind.domain_base_type() {
            None => hasher.update([0]),
            Some(base_type) => {
                hasher.update([1]);
                encode_str(&mut hasher, base_type.schema_name());
                encode_str(&mut hasher, base_type.type_name());
            }
        }
        match type_kind.range_counterpart() {
            None => hasher.update([0]),
            Some(counterpart) => {
                hasher.update([1]);
                encode_str(&mut hasher, counterpart.schema_name());
                encode_str(&mut hasher, counterpart.type_name());
            }
        }
    }

    encode_len(&mut hasher, domains.len());
    for domain in domains {
        encode_str(&mut hasher, domain.schema_name());
        encode_str(&mut hasher, domain.domain_name());
        encode_str(&mut hasher, domain.base_type().schema_name());
        encode_str(&mut hasher, domain.base_type().type_name());
    }

    encode_len(&mut hasher, relations.len());
    for relation in relations {
        encode_str(&mut hasher, relation.schema_name());
        encode_str(&mut hasher, relation.relation_name());
        encode_str(&mut hasher, relation.kind().token());
        encode_len(&mut hasher, relation.columns().len());
        for column in relation.columns() {
            encode_str(&mut hasher, column.column_name());
            hasher.update(column.ordinal_position().to_be_bytes());
            encode_str(&mut hasher, column.type_binding().schema_name());
            encode_str(&mut hasher, column.type_binding().type_name());
        }
    }

    encode_sha256(hasher)
}

fn compute_constraint_timing_digest(
    base_snapshot_digest: &str,
    constraint_timings: &[ConstraintTimingObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_TIMINGS_V1,
    );
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, constraint_timings.len());
    for timing in constraint_timings {
        encode_str(&mut hasher, timing.schema_name());
        encode_str(&mut hasher, timing.relation_name());
        encode_str(&mut hasher, timing.relation_kind().token());
        encode_str(&mut hasher, timing.constraint_name());
        hasher.update([timing.deferrability().tag()]);
    }
    encode_sha256(hasher)
}

fn compute_constraint_period_digest(
    base_snapshot_digest: &str,
    constraint_periods: &[ConstraintPeriodObservation],
) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(
        &mut hasher,
        SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_PERIODS_V1,
    );
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, constraint_periods.len());
    for period in constraint_periods {
        encode_str(&mut hasher, period.schema_name());
        encode_str(&mut hasher, period.relation_name());
        encode_str(&mut hasher, period.relation_kind().token());
        encode_str(&mut hasher, period.constraint_name());
        encode_bool(&mut hasher, period.has_period_semantics());
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

/// Immutable receipt binding one exact observed source coordinate to snapshot provenance.
///
/// The receipt preserves the stable source key and the opaque immutable connection-policy binding
/// that was authorized before source access. The binding is provider-independent provenance, never
/// a credential or connection string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceObservationReceipt {
    inner: model::SourceObservationReceipt,
    connection_policy_binding: String,
}

impl SourceObservationReceipt {
    /// Returns the stable source reference used by candidate evidence binding.
    #[must_use]
    pub fn source_id(&self) -> &str {
        self.inner.source_id()
    }

    /// Returns the opaque immutable connection-policy revision used for this observation.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the immutable canonical snapshot digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        self.inner.source_digest()
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        self.inner.extractor_revision()
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        self.inner.observed_at_utc()
    }

    /// Returns the verified exact source coordinate inside the snapshot.
    #[must_use]
    pub const fn location(&self) -> &ObservationLocation {
        self.inner.location()
    }
}

/// Immutable evidence that one bounded PostgreSQL schema snapshot was observed.
///
/// The snapshot digest is computed by ConceptWeave from a versioned, domain-separated,
/// deterministic framing of the exact observed table, column, and constraint metadata. Source
/// registry identity, connection-policy binding, extractor revision, and observation time remain
/// separate provenance coordinates and do not participate in source-content identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSchemaSnapshot {
    inner: model::PostgresSchemaSnapshot,
    connection_policy_binding: String,
}

impl PostgresSchemaSnapshot {
    /// Creates a deterministic snapshot contract from already-bounded, authorized source metadata.
    ///
    /// Collection order is canonicalized by exact qualified table identifier before the digest is
    /// computed. Exact UTF-8 source text is preserved without Unicode, case, or quoting
    /// normalization. The complete registry-authorized request is required so every observed local
    /// table schema can be checked against the exact request allowlist before immutable evidence or
    /// receipts are created and so the authorized immutable connection-policy binding is retained as
    /// provenance. Referenced foreign-key schemas are relationship evidence and are not treated as
    /// locally observed table schemas. The observation time remains explicit provenance and must use
    /// the canonical UTC form enforced by the underlying observation contract.
    pub fn new(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        mut tables: Vec<TableObservation>,
    ) -> Result<Self, ObservationError> {
        for table in &tables {
            if !authorized_request
                .request()
                .allowed_schema_names()
                .iter()
                .any(|schema_name| schema_name == table.schema_name())
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "unauthorized_schema_name",
                });
            }
        }

        tables.sort_by(|left, right| {
            (left.schema_name(), left.table_name()).cmp(&(right.schema_name(), right.table_name()))
        });
        let snapshot_digest = compute_snapshot_digest(&tables);
        let connection_policy_binding = authorized_request
            .source_connection()
            .connection_policy_binding()
            .to_owned();
        let inner = model::PostgresSchemaSnapshot::new(
            authorized_request.source_connection(),
            snapshot_digest,
            extractor_revision,
            observed_at_utc,
            tables,
        )?;
        Ok(Self {
            inner,
            connection_policy_binding,
        })
    }

    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        self.inner.source_connection_key()
    }

    /// Returns the opaque immutable connection-policy revision authorized for this snapshot.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed canonical SHA-256 source-content digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        self.inner.snapshot_digest()
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        self.inner.extractor_revision()
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        self.inner.observed_at_utc()
    }

    /// Returns qualified tables in deterministic exact-identifier order.
    #[must_use]
    pub fn tables(&self) -> &[TableObservation] {
        self.inner.tables()
    }

    /// Issues provenance for an exact coordinate only when that coordinate exists in this snapshot.
    pub fn source_receipt(
        &self,
        location: ObservationLocation,
    ) -> Result<SourceObservationReceipt, ObservationError> {
        let inner = self.inner.source_receipt(location)?;
        Ok(SourceObservationReceipt {
            inner,
            connection_policy_binding: self.connection_policy_binding.clone(),
        })
    }
}

fn compute_snapshot_digest(tables: &[TableObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V2);
    encode_len(&mut hasher, tables.len());

    for table in tables {
        encode_str(&mut hasher, table.schema_name());
        encode_str(&mut hasher, table.table_name());

        encode_len(&mut hasher, table.columns().len());
        for column in table.columns() {
            encode_str(&mut hasher, column.column_name());
            hasher.update(column.ordinal_position().to_be_bytes());
            encode_str(&mut hasher, column.data_type());
            encode_bool(&mut hasher, column.nullable());
            encode_optional_str(&mut hasher, column.source_comment());
        }

        encode_len(&mut hasher, table.constraints().len());
        for constraint in table.constraints() {
            match constraint {
                TableConstraintObservation::PrimaryKey(observation) => {
                    hasher.update([0]);
                    encode_str(&mut hasher, observation.constraint_name());
                    encode_str_slice(&mut hasher, observation.column_names());
                }
                TableConstraintObservation::Unique(observation) => {
                    hasher.update([1]);
                    encode_str(&mut hasher, observation.constraint_name());
                    encode_str_slice(&mut hasher, observation.column_names());
                    encode_optional_bool(&mut hasher, observation.nulls_not_distinct());
                }
                TableConstraintObservation::ForeignKey(observation) => {
                    hasher.update([2]);
                    encode_str(&mut hasher, observation.constraint_name());
                    encode_str_slice(&mut hasher, observation.column_names());
                    encode_str(&mut hasher, observation.referenced_schema_name());
                    encode_str(&mut hasher, observation.referenced_table_name());
                    encode_str_slice(&mut hasher, observation.referenced_column_names());
                    encode_reference_behavior(&mut hasher, observation.reference_behavior());
                    encode_optional_bool(&mut hasher, observation.validated());
                    encode_optional_bool(&mut hasher, observation.enforced());
                }
                TableConstraintObservation::Check(observation) => {
                    hasher.update([3]);
                    encode_str(&mut hasher, observation.constraint_name());
                    encode_str(&mut hasher, observation.definition());
                    encode_bool(&mut hasher, observation.validated());
                    encode_bool(&mut hasher, observation.enforced());
                    encode_bool(&mut hasher, observation.no_inherit());
                }
            }
        }
    }

    encode_sha256(hasher)
}

fn encode_reference_behavior(hasher: &mut Sha256, behavior: Option<&ForeignKeyReferenceBehavior>) {
    match behavior {
        None => hasher.update([0]),
        Some(behavior) => {
            hasher.update([1]);
            encode_foreign_key_action(hasher, behavior.update_action());
            encode_foreign_key_action(hasher, behavior.delete_action());
            match behavior.delete_target_columns() {
                None => hasher.update([0]),
                Some(columns) => {
                    hasher.update([1]);
                    encode_str_slice(hasher, columns);
                }
            }
            encode_foreign_key_match_type(hasher, behavior.match_type());
            encode_foreign_key_deferrability(hasher, behavior.deferrability());
        }
    }
}

fn encode_foreign_key_action(hasher: &mut Sha256, action: ForeignKeyAction) {
    let tag = match action {
        ForeignKeyAction::NoAction => 0,
        ForeignKeyAction::Restrict => 1,
        ForeignKeyAction::Cascade => 2,
        ForeignKeyAction::SetNull => 3,
        ForeignKeyAction::SetDefault => 4,
    };
    hasher.update([tag]);
}

fn encode_foreign_key_match_type(hasher: &mut Sha256, match_type: ForeignKeyMatchType) {
    let tag = match match_type {
        ForeignKeyMatchType::Simple => 0,
        ForeignKeyMatchType::Full => 1,
        ForeignKeyMatchType::Partial => 2,
    };
    hasher.update([tag]);
}

fn encode_foreign_key_deferrability(hasher: &mut Sha256, deferrability: ForeignKeyDeferrability) {
    let tag = match deferrability {
        ForeignKeyDeferrability::NotDeferrable => 0,
        ForeignKeyDeferrability::InitiallyImmediate => 1,
        ForeignKeyDeferrability::InitiallyDeferred => 2,
    };
    hasher.update([tag]);
}

fn encode_optional_bool(hasher: &mut Sha256, value: Option<bool>) {
    match value {
        None => hasher.update([0]),
        Some(value) => {
            hasher.update([1]);
            encode_bool(hasher, value);
        }
    }
}

fn encode_optional_str(hasher: &mut Sha256, value: Option<&str>) {
    match value {
        None => hasher.update([0]),
        Some(value) => {
            hasher.update([1]);
            encode_str(hasher, value);
        }
    }
}

fn encode_str_slice(hasher: &mut Sha256, values: &[String]) {
    encode_len(hasher, values.len());
    for value in values {
        encode_str(hasher, value);
    }
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

fn encode_bool(hasher: &mut Sha256, value: bool) {
    hasher.update([u8::from(value)]);
}

#[cfg(test)]
mod internal_model_tests {
    use super::model;
    use conceptweave_source_port::{
        ObservationLimits, ObservationRequest, ObservationRequestBudget, ResolvedSourceConnection,
        SourceConnectionRegistry,
    };

    struct ExactRegistry;

    impl SourceConnectionRegistry for ExactRegistry {
        fn contains_source_connection(&self, source_connection_key: &str) -> bool {
            source_connection_key == "warehouse_primary"
        }

        fn connection_policy_binding(&self, source_connection_key: &str) -> Option<String> {
            (source_connection_key == "warehouse_primary")
                .then(|| "fixture_policy_revision_a".to_owned())
        }
    }

    fn resolved_source() -> ResolvedSourceConnection {
        ObservationRequest::new(
            "warehouse_primary",
            vec!["public".to_owned()],
            ObservationRequestBudget::new(4, 256).unwrap(),
            ObservationLimits::new(1_000, 10, 1_024, 1).unwrap(),
        )
        .unwrap()
        .resolve_source_connection(&ExactRegistry)
        .unwrap()
    }

    #[test]
    fn internal_snapshot_model_rejects_noncanonical_digest_input() {
        for digest_input in [
            "not-a-digest".to_owned(),
            format!("SHA256:{}", "a".repeat(64)),
            format!("sha256:{}", "A".repeat(64)),
            format!("sha256:{}", "g".repeat(64)),
        ] {
            let error = model::PostgresSchemaSnapshot::new(
                &resolved_source(),
                digest_input,
                "postgres_introspector_v1",
                "2026-09-05T03:30:00Z",
                Vec::new(),
            )
            .expect_err(
                "the private storage model must still fail closed on malformed digest input",
            );
            assert_eq!(
                error,
                model::ObservationError::InvalidObservationField {
                    field: "snapshot_digest"
                }
            );
        }
    }
}
