//! Immutable PostgreSQL schema-observation contracts for ConceptWeave.
//!
//! The public aggregate derives source-content identity from deterministic observed metadata.
//! Source connection, connection-policy revision, extractor revision, and observation time remain
//! separate provenance coordinates and therefore do not change the source-content digest.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod array_type;
mod collation_definition;
mod column_array_dimensions;
mod column_collation;
mod column_expression;
mod column_generation;
mod column_identity;
mod constraint_period;
mod constraint_timing;
mod foreign_key_catalog;
mod model;
mod not_null_constraint;
mod range_catalog;
mod relation_owner;
mod relation_tablespace;
mod representation_v3;
mod schema_owner;
mod type_kind;

pub use array_type::{ArrayTypeLocation, ArrayTypeObservation, ArrayTypeSourceReceipt};
pub use collation_definition::{
    CollationDefinitionObservation, CollationLocaleFields, CollationProvider,
    DatabaseLocaleDefinition,
};
pub use column_array_dimensions::ColumnArrayDimensionsObservation;
pub use column_collation::ColumnCollationObservation;
pub use column_expression::ColumnExpressionObservation;
pub use column_generation::ColumnGenerationObservation;
pub use column_identity::{ColumnIdentityObservation, IdentitySequenceObservation};
pub use constraint_period::ConstraintPeriodObservation;
pub use constraint_timing::{ConstraintDeferrability, ConstraintTimingObservation};
pub use foreign_key_catalog::{ForeignKeyCatalogObservation, ForeignKeyOperatorObservation};
pub use model::{
    CheckConstraintObservation, ColumnObservation, ForeignKeyAction, ForeignKeyDeferrability,
    ForeignKeyMatchType, ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationError,
    ObservationLocation, ObservationLocationKind, PrimaryKeyObservation,
    TableConstraintObservation, TableObservation, UniqueConstraintObservation,
};
pub use not_null_constraint::{NotNullConstraintObservation, ParentNotNullConstraintCoordinate};
pub use range_catalog::{
    QualifiedRangeProcedure, RangeCatalogObservation, RangeCatalogSourceReceipt,
};
pub use relation_owner::RelationOwnerObservation;
pub use relation_tablespace::RelationTablespaceObservation;
pub use representation_v3::{
    ColumnObservationV3, DomainCheckConstraintObservation, DomainObservation, EnumObservation,
    IndexAttributeKind, IndexAttributeObservation, IndexAttributeSource, IndexCatalogFlags,
    IndexKeySemantics, IndexObservation, IndexStorageOption, IndexTablespace, OperatorClassOption,
    QualifiedCollationName, QualifiedOperatorClassName, QualifiedTypeName, RelationKind,
    RelationObservation, ReplicaIdentityMode, SchemaObjectLocation, SchemaObjectLocationKind,
};
pub use schema_owner::{SchemaOwnerLocation, SchemaOwnerObservation, SchemaOwnerSourceReceipt};
pub use type_kind::TypeOwnerObservation;
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
const SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_PERIODS_V2: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.constraint_periods.v2";
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
/// optional column-collation, column-generation, column-expression, column-identity, and first-class
/// PostgreSQL 18 NOT NULL constraint state, PRIMARY KEY/UNIQUE timing, and explicit temporal-
/// constraint evidence before exposing immutable governed evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSchemaSnapshotV3 {
    inner: representation_v3::PostgresSchemaSnapshotV3,
    snapshot_digest: String,
    authorized_schema_names: Vec<String>,
    relations: Vec<RelationObservation>,
    domains: Vec<DomainObservation>,
    enums: Vec<EnumObservation>,
    array_types: Vec<ArrayTypeObservation>,
    array_types_observed: bool,
    type_kinds: Vec<TypeKindObservation>,
    type_kinds_observed: bool,
    type_owners: Vec<TypeOwnerObservation>,
    type_owners_observed: bool,
    schema_owners: Vec<SchemaOwnerObservation>,
    schema_owners_observed: bool,
    column_array_dimensions: Vec<ColumnArrayDimensionsObservation>,
    column_array_dimensions_observed: bool,
    column_collations: Vec<ColumnCollationObservation>,
    column_collations_observed: bool,
    column_generations: Vec<ColumnGenerationObservation>,
    column_generations_observed: bool,
    column_expressions: Vec<ColumnExpressionObservation>,
    column_expressions_observed: bool,
    column_identities: Vec<ColumnIdentityObservation>,
    column_identities_observed: bool,
    not_null_constraints: Vec<NotNullConstraintObservation>,
    not_null_constraints_observed: bool,
    constraint_timings: Vec<ConstraintTimingObservation>,
    constraint_timings_observed: bool,
    constraint_periods: Vec<ConstraintPeriodObservation>,
    constraint_periods_observed: bool,
    foreign_key_catalog: Vec<ForeignKeyCatalogObservation>,
    foreign_key_catalog_observed: bool,
    range_catalog: Vec<RangeCatalogObservation>,
    range_catalog_observed: bool,
    relation_tablespaces: Vec<RelationTablespaceObservation>,
    relation_tablespaces_observed: bool,
    relation_owners: Vec<RelationOwnerObservation>,
    relation_owners_observed: bool,
    collation_definitions: Vec<CollationDefinitionObservation>,
    collation_definitions_observed: bool,
}

impl PostgresSchemaSnapshotV3 {
    /// Creates the original deterministic v3 snapshot without claiming true-array, type-kind,
    /// column-collation, column-generation, column-expression, column-identity, NOT NULL constraint,
    /// key-constraint timing, or temporal constraint inventory.
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
            authorized_schema_names: authorized_request.request().allowed_schema_names().to_vec(),
            relations,
            domains,
            enums,
            array_types: Vec::new(),
            array_types_observed: false,
            type_kinds: Vec::new(),
            type_kinds_observed: false,
            type_owners: Vec::new(),
            type_owners_observed: false,
            schema_owners: Vec::new(),
            schema_owners_observed: false,
            column_array_dimensions: Vec::new(),
            column_array_dimensions_observed: false,
            column_collations: Vec::new(),
            column_collations_observed: false,
            column_generations: Vec::new(),
            column_generations_observed: false,
            column_expressions: Vec::new(),
            column_expressions_observed: false,
            column_identities: Vec::new(),
            column_identities_observed: false,
            not_null_constraints: Vec::new(),
            not_null_constraints_observed: false,
            constraint_timings: Vec::new(),
            constraint_timings_observed: false,
            constraint_periods: Vec::new(),
            constraint_periods_observed: false,
            foreign_key_catalog: Vec::new(),
            foreign_key_catalog_observed: false,
            range_catalog: Vec::new(),
            range_catalog_observed: false,
            relation_tablespaces: Vec::new(),
            relation_tablespaces_observed: false,
            relation_owners: Vec::new(),
            relation_owners_observed: false,
            collation_definitions: Vec::new(),
            collation_definitions_observed: false,
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
        let type_kinds = canonicalize_type_kind_observations(
            Some(authorized_request.request().allowed_schema_names()),
            &relations,
            &domains,
            &enums,
            type_kinds,
        )?;
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
            authorized_schema_names: authorized_request.request().allowed_schema_names().to_vec(),
            relations,
            domains,
            enums,
            array_types: Vec::new(),
            array_types_observed: false,
            type_kinds,
            type_kinds_observed: true,
            type_owners: Vec::new(),
            type_owners_observed: false,
            schema_owners: Vec::new(),
            schema_owners_observed: false,
            column_array_dimensions: Vec::new(),
            column_array_dimensions_observed: false,
            column_collations: Vec::new(),
            column_collations_observed: false,
            column_generations: Vec::new(),
            column_generations_observed: false,
            column_expressions: Vec::new(),
            column_expressions_observed: false,
            column_identities: Vec::new(),
            column_identities_observed: false,
            not_null_constraints: Vec::new(),
            not_null_constraints_observed: false,
            constraint_timings: Vec::new(),
            constraint_timings_observed: false,
            constraint_periods: Vec::new(),
            constraint_periods_observed: false,
            foreign_key_catalog: Vec::new(),
            foreign_key_catalog_observed: false,
            range_catalog: Vec::new(),
            range_catalog_observed: false,
            relation_tablespaces: Vec::new(),
            relation_tablespaces_observed: false,
            relation_owners: Vec::new(),
            relation_owners_observed: false,
            collation_definitions: Vec::new(),
            collation_definitions_observed: false,
        })
    }

    /// Creates a type-kind-aware v3 snapshot with explicit PRIMARY KEY/UNIQUE timing evidence.
    #[expect(
        clippy::too_many_arguments,
        reason = "preserve public constructor compatibility"
    )]
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

    /// Creates a deterministic v3 snapshot with explicitly observed PostgreSQL column collations.
    ///
    /// The family is complete for the bounded relation-column inventory. Explicit `attcollation = 0`
    /// remains distinct from a family that the adapter did not observe at all, while collatable
    /// columns retain exact qualified `pg_collation` identity and source-authoritative determinism.
    pub fn new_with_column_collations(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        column_collations: Vec<ColumnCollationObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )?
        .with_observed_column_collations(column_collations)
    }

    /// Creates a deterministic v3 snapshot with explicitly observed PostgreSQL generated-column mode.
    ///
    /// The family is complete for the bounded relation-column inventory and distinguishes explicit
    /// ordinary columns from a generation family that the adapter did not observe at all. Generation
    /// expressions and dependencies remain separate evidence and are never inferred from defaults.
    pub fn new_with_column_generations(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        column_generations: Vec<ColumnGenerationObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )?
        .with_observed_column_generations(column_generations)
    }

    /// Creates a deterministic v3 snapshot with source-authoritative column generation and expression evidence.
    ///
    /// `pg_attribute.attgenerated` is attached before the corresponding `pg_attrdef` expression
    /// family so generated/default expression kind can be checked against source-authoritative `attgenerated` state. The family is
    /// attached before identity, NOT NULL constraint, constraint timing, and PERIOD evidence; reverse-
    /// order attachment is rejected so optional-family order cannot become a semantic escape hatch.
    #[expect(
        clippy::too_many_arguments,
        reason = "preserve public constructor compatibility"
    )]
    pub fn new_with_column_expressions(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        column_generations: Vec<ColumnGenerationObservation>,
        column_expressions: Vec<ColumnExpressionObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new_with_column_generations(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
            column_generations,
        )?
        .with_observed_column_expressions(column_expressions)
    }

    /// Creates a deterministic v3 snapshot with explicitly observed PostgreSQL column identity mode.
    ///
    /// The family is complete for the bounded relation-column inventory and distinguishes explicit
    /// non-identity columns from an identity family that was not observed at all. Identity sequence
    /// options remain outside this declaration family and are never inferred from defaults or names.
    pub fn new_with_column_identities(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        column_identities: Vec<ColumnIdentityObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )?
        .with_observed_column_identities(column_identities)
    }

    /// Creates a deterministic v3 snapshot with explicitly observed PostgreSQL 18 `NOT NULL`
    /// constraint rows.
    ///
    /// The family is complete for the bounded non-null column inventory. An empty vector is therefore
    /// observed-empty evidence and remains distinct from a snapshot whose adapter did not observe the
    /// PostgreSQL 18 `pg_constraint.contype = 'n'` family at all.
    pub fn new_with_not_null_constraints(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        not_null_constraints: Vec<NotNullConstraintObservation>,
    ) -> Result<Self, ObservationError> {
        Self::new(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            relations,
            domains,
            enums,
        )?
        .with_observed_not_null_constraints(not_null_constraints)
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
            &[],
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
            authorized_schema_names: authorized_request.request().allowed_schema_names().to_vec(),
            relations,
            domains,
            enums,
            array_types,
            array_types_observed: true,
            type_kinds: Vec::new(),
            type_kinds_observed: false,
            type_owners: Vec::new(),
            type_owners_observed: false,
            schema_owners: Vec::new(),
            schema_owners_observed: false,
            column_array_dimensions: Vec::new(),
            column_array_dimensions_observed: false,
            column_collations: Vec::new(),
            column_collations_observed: false,
            column_generations: Vec::new(),
            column_generations_observed: false,
            column_expressions: Vec::new(),
            column_expressions_observed: false,
            column_identities: Vec::new(),
            column_identities_observed: false,
            not_null_constraints: Vec::new(),
            not_null_constraints_observed: false,
            constraint_timings: Vec::new(),
            constraint_timings_observed: false,
            constraint_periods: Vec::new(),
            constraint_periods_observed: false,
            foreign_key_catalog: Vec::new(),
            foreign_key_catalog_observed: false,
            range_catalog: Vec::new(),
            range_catalog_observed: false,
            relation_tablespaces: Vec::new(),
            relation_tablespaces_observed: false,
            relation_owners: Vec::new(),
            relation_owners_observed: false,
            collation_definitions: Vec::new(),
            collation_definitions_observed: false,
        })
    }

    /// Captures exact true-array pairs alongside all schema-local PostgreSQL type kinds.
    #[expect(
        clippy::too_many_arguments,
        reason = "preserve source evidence constructor"
    )]
    pub fn new_with_array_types_and_type_kinds(
        authorized_request: &AuthorizedObservationRequest,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        relations: Vec<RelationObservation>,
        domains: Vec<DomainObservation>,
        enums: Vec<EnumObservation>,
        array_types: Vec<ArrayTypeObservation>,
        type_kinds: Vec<TypeKindObservation>,
    ) -> Result<Self, ObservationError> {
        validate_schema_relation_invariants(&relations)?;
        let type_kinds = canonicalize_type_kind_observations(
            Some(authorized_request.request().allowed_schema_names()),
            &relations,
            &domains,
            &enums,
            type_kinds,
        )?;
        let array_types = canonicalize_array_type_observations(
            authorized_request,
            &relations,
            &domains,
            &enums,
            &type_kinds,
            array_types,
        )?;
        if array_types.iter().any(|array| {
            array.array_type().schema_name() != POSTGRES_CATALOG_SCHEMA_NAME
                && !type_kinds.iter().any(|kind| {
                    same_type_coordinate(kind.type_name(), array.array_type())
                        && kind.kind() == PostgresTypeKind::Base
                })
        }) {
            return Err(ObservationError::InvalidObservationField {
                field: "array_type_kind",
            });
        }
        validate_type_bindings_with_type_kinds_and_arrays(
            &relations,
            &domains,
            &enums,
            &array_types,
            &type_kinds,
        )?;
        let projected_relations = relations
            .iter()
            .map(|relation| project_relation_array_bindings(relation, &array_types))
            .collect::<Result<Vec<_>, _>>()?;
        let projected_domains = domains
            .iter()
            .map(|domain| project_domain_array_binding(domain, &array_types))
            .collect::<Result<Vec<_>, _>>()?;
        let projected_type_kinds = type_kinds
            .iter()
            .map(|kind| {
                if kind.kind() != PostgresTypeKind::Domain {
                    return Ok(kind.clone());
                }
                let domain = projected_domains
                    .iter()
                    .find(|domain| {
                        domain.schema_name() == kind.type_name().schema_name()
                            && domain.domain_name() == kind.type_name().type_name()
                    })
                    .ok_or(ObservationError::InvalidObservationField {
                        field: "type_kind_coordinate",
                    })?;
                Ok(TypeKindObservation::domain(
                    kind.type_name().clone(),
                    domain.base_type().clone(),
                ))
            })
            .collect::<Result<Vec<_>, ObservationError>>()?;
        let mut snapshot = Self::new_with_type_kinds(
            authorized_request,
            extractor_revision,
            observed_at_utc,
            projected_relations,
            projected_domains,
            enums,
            projected_type_kinds,
        )?;
        snapshot.type_kinds = type_kinds;
        snapshot.relations = relations;
        snapshot.relations.sort_by(|left, right| {
            (left.schema_name(), left.relation_name())
                .cmp(&(right.schema_name(), right.relation_name()))
        });
        snapshot.domains = domains;
        snapshot.domains.sort_by(|left, right| {
            (left.schema_name(), left.domain_name())
                .cmp(&(right.schema_name(), right.domain_name()))
        });
        snapshot.snapshot_digest = compute_type_kind_aware_snapshot_digest(
            &compute_array_aware_snapshot_digest(
                snapshot.inner.snapshot_digest(),
                &snapshot.relations,
                &snapshot.domains,
                &array_types,
            ),
            &snapshot.relations,
            &snapshot.domains,
            &snapshot.type_kinds,
        );
        snapshot.array_types = array_types;
        snapshot.array_types_observed = true;
        Ok(snapshot)
    }

    /// Creates a deterministic v3 snapshot with both true-array identity and key-constraint timing.
    ///
    /// The array-aware digest is computed first; timing evidence then adds its own domain-separated
    /// layer. This keeps each observed catalog family explicit while supporting one immutable source
    /// snapshot containing both families.
    #[expect(
        clippy::too_many_arguments,
        reason = "preserve public constructor compatibility"
    )]
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
    /// The family must be attached before column-collation, column-generation, column-expression,
    /// column-identity, NOT NULL constraint, constraint timing, or period evidence so optional family
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
        if self.column_collations_observed
            || self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_observation_order",
            });
        }
        let type_kinds = canonicalize_type_kind_observations(
            None,
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

    /// Adds one complete explicitly observed `pg_attribute.attcollation` family to this snapshot.
    ///
    /// The family is attached after any type-kind/array identity layer and before column-generation,
    /// column-expression, column-identity, NOT NULL constraint, constraint timing, or PERIOD evidence,
    /// preserving one canonical optional-family order. It validates exact bounded column coordinates,
    /// completeness, repeated-collation determinism, and PostgreSQL's FK collation consistency rule
    /// before extending the source digest.
    pub fn with_observed_column_collations(
        mut self,
        column_collations: Vec<ColumnCollationObservation>,
    ) -> Result<Self, ObservationError> {
        if self.column_collations_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "column_collation_already_observed",
            });
        }
        if self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_collation_observation_order",
            });
        }
        let column_collations =
            column_collation::canonicalize_column_collations(&self.relations, column_collations)?;
        self.snapshot_digest = column_collation::compute_column_collation_digest(
            &self.snapshot_digest,
            &column_collations,
        );
        self.column_collations = column_collations;
        self.column_collations_observed = true;
        Ok(self)
    }

    /// Adds one complete explicitly observed `pg_attribute.attgenerated` family to this snapshot.
    ///
    /// The family is attached after type/array and column-collation evidence and before column-expression,
    /// column-identity, NOT NULL constraint, constraint timing, or PERIOD evidence. It validates exact
    /// bounded column coordinates, completeness, explicit not-generated state, canonical input order,
    /// and extends source identity in a dedicated digest domain.
    pub fn with_observed_column_generations(
        mut self,
        column_generations: Vec<ColumnGenerationObservation>,
    ) -> Result<Self, ObservationError> {
        if self.column_generations_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "column_generation_already_observed",
            });
        }
        if self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_generation_observation_order",
            });
        }
        let column_generations = column_generation::canonicalize_column_generations(
            &self.relations,
            column_generations,
        )?;
        self.snapshot_digest = column_generation::compute_column_generation_digest(
            &self.snapshot_digest,
            &column_generations,
        );
        self.column_generations = column_generations;
        self.column_generations_observed = true;
        Ok(self)
    }

    /// Adds one complete explicitly observed `pg_attrdef` column-expression family to this snapshot.
    ///
    /// Generation declaration evidence must already be observed so default versus generated
    /// expression kind is validated against source-authoritative `attgenerated` state. The family is
    /// attached before identity, NOT NULL constraint, constraint timing, and PERIOD evidence; reverse-
    /// order attachment is rejected so optional-family order cannot become a semantic escape hatch.
    pub fn with_observed_column_expressions(
        mut self,
        column_expressions: Vec<ColumnExpressionObservation>,
    ) -> Result<Self, ObservationError> {
        if self.column_expressions_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_already_observed",
            });
        }
        if !self.column_generations_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_generation_required",
            });
        }
        if self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_expression_observation_order",
            });
        }
        let column_expressions = column_expression::canonicalize_column_expressions(
            &self.relations,
            &self.column_generations,
            column_expressions,
        )?;
        self.snapshot_digest = column_expression::compute_column_expression_digest(
            &self.snapshot_digest,
            &column_expressions,
        );
        self.column_expressions = column_expressions;
        self.column_expressions_observed = true;
        Ok(self)
    }

    /// Adds one complete explicitly observed `pg_attribute.attidentity` family to this snapshot.
    ///
    /// The family is attached after any type/array, column-collation, column-generation, and optional
    /// column-expression evidence and before NOT NULL constraint, constraint timing, or PERIOD evidence.
    /// It validates exact bounded column coordinates and completeness, keeps explicit not-identity
    /// distinct from unobserved evidence, rejects a generated-column/identity contradiction,
    /// canonicalizes input order, and extends source identity in its own digest domain.
    pub fn with_observed_column_identities(
        mut self,
        column_identities: Vec<ColumnIdentityObservation>,
    ) -> Result<Self, ObservationError> {
        if self.column_identities_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "column_identity_already_observed",
            });
        }
        if self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_identity_observation_order",
            });
        }
        let column_identities =
            column_identity::canonicalize_column_identities(&self.relations, column_identities)?;
        if self.column_generations_observed
            && column_generation::generated_identity_conflicts(
                &self.column_generations,
                &column_identities,
            )
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_generation_identity",
            });
        }
        self.snapshot_digest = column_identity::compute_column_identity_digest(
            &self.snapshot_digest,
            &column_identities,
        );
        self.column_identities = column_identities;
        self.column_identities_observed = true;
        Ok(self)
    }

    /// Adds one complete explicitly observed PostgreSQL 18 `pg_constraint.contype = 'n'` family.
    ///
    /// The family is attached after column-identity evidence, when present, and before key-constraint
    /// timing or PERIOD evidence. It binds the exact first-class NOT NULL constraint rows to the
    /// frozen column nullability summary, distinguishes observed-empty from unobserved state, and
    /// extends source identity exactly once in its own digest domain.
    pub fn with_observed_not_null_constraints(
        mut self,
        not_null_constraints: Vec<NotNullConstraintObservation>,
    ) -> Result<Self, ObservationError> {
        if self.not_null_constraints_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_already_observed",
            });
        }
        if self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "not_null_constraint_observation_order",
            });
        }
        let not_null_constraints = not_null_constraint::canonicalize_not_null_constraints(
            &self.relations,
            not_null_constraints,
        )?;
        self.snapshot_digest = not_null_constraint::compute_not_null_constraint_digest(
            &self.snapshot_digest,
            &not_null_constraints,
        );
        self.not_null_constraints = not_null_constraints;
        self.not_null_constraints_observed = true;
        Ok(self)
    }

    /// Adds complete PRIMARY KEY and UNIQUE deferrability evidence after column and NOT NULL
    /// families. The backing index must agree with every observed timing state.
    pub fn with_observed_constraint_timings(
        mut self,
        constraint_timings: Vec<ConstraintTimingObservation>,
    ) -> Result<Self, ObservationError> {
        if self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "constraint_timing_observation_order",
            });
        }
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
    /// `WITHOUT OVERLAPS` PRIMARY KEY/UNIQUE observations also require the exact resolved ordered
    /// `pg_constraint.conexclop` operator signatures and coherent same-name GiST/exclusion backing
    /// evidence; those facts never invent `conperiod`. A PERIOD foreign key targeting a relation
    /// inside the same bounded snapshot must resolve to an explicitly observed `WITHOUT OVERLAPS`,
    /// `NOT DEFERRABLE` key on the referenced columns; referenced-key timing is never inferred from
    /// index shape. This consuming method may be applied only once.
    pub fn with_observed_constraint_periods(
        mut self,
        constraint_periods: Vec<ConstraintPeriodObservation>,
    ) -> Result<Self, ObservationError> {
        if self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
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

    /// Adds a complete foreign-key catalog family after all other observed families.
    /// The referenced relation and selected unique index must be in this bounded snapshot.
    pub fn with_observed_foreign_key_catalog(
        mut self,
        observations: Vec<ForeignKeyCatalogObservation>,
    ) -> Result<Self, ObservationError> {
        if self.foreign_key_catalog_observed || self.collation_definitions_observed {
            return Err(ObservationError::InvalidObservationField {
                field: "foreign_key_catalog_already_observed",
            });
        }
        let observations = foreign_key_catalog::canonicalize(&self.relations, observations)?;
        self.snapshot_digest = foreign_key_catalog::digest(&self.snapshot_digest, &observations);
        self.foreign_key_catalog = observations;
        self.foreign_key_catalog_observed = true;
        Ok(self)
    }

    /// Adds complete resolved storage coordinates for every observed ordinary table.
    /// The predecessor v3 digest remains reproducible; this is an explicit successor family.
    pub fn with_observed_relation_tablespaces(
        mut self,
        observations: Vec<RelationTablespaceObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.type_kinds_observed
            || self.relation_tablespaces_observed
            || self.relation_owners_observed
            || self.range_catalog_observed
            || self.column_collations_observed
            || self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "relation_tablespace_observation_order",
            });
        }
        let observations = relation_tablespace::canonicalize(&self.relations, observations)?;
        self.snapshot_digest = relation_tablespace::digest(&self.snapshot_digest, &observations);
        self.relation_tablespaces = observations;
        self.relation_tablespaces_observed = true;
        Ok(self)
    }

    /// Adds exact same-generation relation owner OIDs and resolved role names.
    /// This successor leaves historical v3 and table-storage digests reproducible.
    pub fn with_observed_relation_owners(
        mut self,
        observations: Vec<RelationOwnerObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.type_kinds_observed
            || (self
                .relations
                .iter()
                .any(|relation| relation.kind() == RelationKind::Table)
                && !self.relation_tablespaces_observed)
            || self.relation_owners_observed
            || self.range_catalog_observed
            || self.column_collations_observed
            || self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "relation_owner_observation_order",
            });
        }
        let observations = relation_owner::canonicalize(&self.relations, observations)?;
        self.snapshot_digest = relation_owner::digest(&self.snapshot_digest, &observations);
        self.relation_owners = observations;
        self.relation_owners_observed = true;
        Ok(self)
    }

    /// Adds a complete same-generation owner role for every schema-local type.
    /// Historical type-kind and relation-owner digests remain reproducible.
    pub fn with_observed_type_owners(
        mut self,
        observations: Vec<TypeOwnerObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.type_kinds_observed
            || (!self.relations.is_empty() && !self.relation_owners_observed)
            || self.type_owners_observed
            || self.range_catalog_observed
            || self.column_collations_observed
            || self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_owner_observation_order",
            });
        }
        let observations = type_kind::canonicalize_owners(&self.type_kinds, observations)?;
        self.snapshot_digest = type_kind::owner_digest(&self.snapshot_digest, &observations);
        self.type_owners = observations;
        self.type_owners_observed = true;
        Ok(self)
    }

    /// Adds exact same-generation owner identities for every observed source schema.
    /// Earlier v3 and type-owner digests remain reproducible.
    pub fn with_observed_schema_owners(
        mut self,
        observations: Vec<SchemaOwnerObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.type_owners_observed
            || self.schema_owners_observed
            || self.range_catalog_observed
            || self.column_collations_observed
            || self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "schema_owner_observation_order",
            });
        }
        let observations = schema_owner::canonicalize(&self.authorized_schema_names, observations)?;
        self.snapshot_digest = schema_owner::digest(&self.snapshot_digest, &observations);
        self.schema_owners = observations;
        self.schema_owners_observed = true;
        Ok(self)
    }

    /// Adds complete `pg_range` subtype, ordering, collation, and function coordinates.
    /// This successor digest leaves historical type-kind identities unchanged.
    pub fn with_observed_range_catalog(
        mut self,
        observations: Vec<RangeCatalogObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.type_kinds_observed
            || self.range_catalog_observed
            || self.column_collations_observed
            || self.column_generations_observed
            || self.column_expressions_observed
            || self.column_identities_observed
            || self.not_null_constraints_observed
            || self.constraint_timings_observed
            || self.constraint_periods_observed
            || self.foreign_key_catalog_observed
            || self.collation_definitions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "range_catalog_observation_order",
            });
        }
        let observations = range_catalog::canonicalize(&self.type_kinds, observations)?;
        for observation in &observations {
            if !type_binding_is_resolvable_with_type_kinds_and_arrays(
                observation.subtype(),
                &self.relations,
                &self.domains,
                &self.enums,
                &self.array_types,
                &self.type_kinds,
            ) {
                return Err(ObservationError::UnknownTypeBinding {
                    schema_name: observation.subtype().schema_name().to_owned(),
                    type_name: observation.subtype().type_name().to_owned(),
                });
            }
        }
        self.snapshot_digest = range_catalog::digest(&self.snapshot_digest, &observations);
        if observations
            .iter()
            .any(|item| item.canonical().is_some() || item.subtype_difference().is_some())
        {
            self.snapshot_digest =
                range_catalog::procedure_definition_digest(&self.snapshot_digest, &observations);
        }
        self.range_catalog = observations;
        self.range_catalog_observed = true;
        Ok(self)
    }

    /// Adds the complete definitions of collations referenced by observed columns, domains, index
    /// keys, and range types. This final successor family binds stored and actual provider versions, and the
    /// effective database locale when PostgreSQL's default collation is referenced.
    pub fn with_observed_collation_definitions(
        mut self,
        definitions: Vec<CollationDefinitionObservation>,
    ) -> Result<Self, ObservationError> {
        if self.collation_definitions_observed
            || (!self.relations.is_empty() && !self.column_collations_observed)
        {
            return Err(ObservationError::InvalidObservationField {
                field: "collation_definition_observation_order",
            });
        }
        let columns = self
            .column_collations_observed
            .then_some(self.column_collations.as_slice());
        let definitions = collation_definition::canonicalize(
            &self.relations,
            &self.domains,
            columns,
            self.range_catalog_observed
                .then_some(self.range_catalog.as_slice()),
            definitions,
        )?;
        self.snapshot_digest = collation_definition::digest(&self.snapshot_digest, &definitions);
        self.collation_definitions = definitions;
        self.collation_definitions_observed = true;
        Ok(self)
    }

    /// Adds exact declared array dimensions for every bounded column after all earlier families.
    /// The historical v3 and prior successor digests remain reproducible.
    pub fn with_observed_column_array_dimensions(
        mut self,
        observations: Vec<ColumnArrayDimensionsObservation>,
    ) -> Result<Self, ObservationError> {
        if !self.collation_definitions_observed
            || !self.array_types_observed
            || self.column_array_dimensions_observed
        {
            return Err(ObservationError::InvalidObservationField {
                field: "column_array_dimensions_observation_order",
            });
        }
        let observations = column_array_dimensions::canonicalize(
            &self.relations,
            &self.array_types,
            observations,
        )?;
        self.snapshot_digest =
            column_array_dimensions::digest(&self.snapshot_digest, &observations);
        self.column_array_dimensions = observations;
        self.column_array_dimensions_observed = true;
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

    /// Returns complete type-owner evidence, or `None` when unobserved.
    #[must_use]
    pub fn type_owners(&self) -> Option<&[TypeOwnerObservation]> {
        self.type_owners_observed
            .then_some(self.type_owners.as_slice())
    }

    /// Returns complete observed schema-owner evidence, or `None` when unobserved.
    #[must_use]
    pub fn schema_owners(&self) -> Option<&[SchemaOwnerObservation]> {
        self.schema_owners_observed
            .then_some(self.schema_owners.as_slice())
    }

    /// Returns complete declared column-array dimensions, or `None` when unobserved.
    #[must_use]
    pub fn column_array_dimensions(&self) -> Option<&[ColumnArrayDimensionsObservation]> {
        self.column_array_dimensions_observed
            .then_some(self.column_array_dimensions.as_slice())
    }

    /// Returns explicitly observed PostgreSQL column-collation evidence, or `None` when that catalog
    /// family was not observed.
    #[must_use]
    pub fn column_collations(&self) -> Option<&[ColumnCollationObservation]> {
        self.column_collations_observed
            .then_some(self.column_collations.as_slice())
    }

    /// Returns explicitly observed PostgreSQL generated-column evidence, or `None` when that catalog
    /// family was not observed.
    #[must_use]
    pub fn column_generations(&self) -> Option<&[ColumnGenerationObservation]> {
        self.column_generations_observed
            .then_some(self.column_generations.as_slice())
    }

    /// Returns explicitly observed PostgreSQL column default/generated-expression evidence, or `None`
    /// when that catalog family was not observed.
    #[must_use]
    pub fn column_expressions(&self) -> Option<&[ColumnExpressionObservation]> {
        self.column_expressions_observed
            .then_some(self.column_expressions.as_slice())
    }

    /// Returns explicitly observed PostgreSQL column-identity evidence, or `None` when that catalog
    /// family was not observed.
    #[must_use]
    pub fn column_identities(&self) -> Option<&[ColumnIdentityObservation]> {
        self.column_identities_observed
            .then_some(self.column_identities.as_slice())
    }

    /// Returns explicitly observed PostgreSQL 18 first-class NOT NULL constraint evidence, or `None`
    /// when the adapter did not observe that catalog family.
    #[must_use]
    pub fn not_null_constraints(&self) -> Option<&[NotNullConstraintObservation]> {
        self.not_null_constraints_observed
            .then_some(self.not_null_constraints.as_slice())
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

    /// Returns the complete observed foreign-key comparison and backing-index catalog family.
    #[must_use]
    pub fn foreign_key_catalog(&self) -> Option<&[ForeignKeyCatalogObservation]> {
        self.foreign_key_catalog_observed
            .then_some(self.foreign_key_catalog.as_slice())
    }

    /// Returns complete range catalog evidence, or `None` when that family was unobserved.
    #[must_use]
    pub fn range_catalog(&self) -> Option<&[RangeCatalogObservation]> {
        self.range_catalog_observed
            .then_some(self.range_catalog.as_slice())
    }

    /// Returns the complete ordinary-table storage family, or `None` when unobserved.
    #[must_use]
    pub fn relation_tablespaces(&self) -> Option<&[RelationTablespaceObservation]> {
        self.relation_tablespaces_observed
            .then_some(self.relation_tablespaces.as_slice())
    }

    /// Returns complete relation-owner evidence, or `None` when unobserved.
    #[must_use]
    pub fn relation_owners(&self) -> Option<&[RelationOwnerObservation]> {
        self.relation_owners_observed
            .then_some(self.relation_owners.as_slice())
    }

    /// Issues provenance only for an exact observed range catalog coordinate.
    pub fn range_catalog_source_receipt(
        &self,
        range_type: QualifiedTypeName,
    ) -> Result<RangeCatalogSourceReceipt, ObservationError> {
        if !self.range_catalog_observed
            || !self
                .range_catalog
                .iter()
                .any(|item| item.range_type() == &range_type)
        {
            return Err(ObservationError::UnknownTypeBinding {
                schema_name: range_type.schema_name().to_owned(),
                type_name: range_type.type_name().to_owned(),
            });
        }
        Ok(RangeCatalogSourceReceipt::new(
            self.source_connection_key().to_owned(),
            self.connection_policy_binding().to_owned(),
            self.snapshot_digest.clone(),
            self.extractor_revision().to_owned(),
            self.observed_at_utc().to_owned(),
            range_type,
        ))
    }

    /// Returns exact definitions for every collation referenced by the observed schema evidence.
    #[must_use]
    pub fn collation_definitions(&self) -> Option<&[CollationDefinitionObservation]> {
        self.collation_definitions_observed
            .then_some(self.collation_definitions.as_slice())
    }

    /// Issues provenance for an exact successor coordinate only when it exists in this snapshot.
    pub fn source_receipt(
        &self,
        location: SchemaObjectLocation,
    ) -> Result<SuccessorSourceReceipt, ObservationError> {
        let observed_not_null = self.not_null_constraints_observed
            && self.not_null_constraints.iter().any(|observation| {
                SchemaObjectLocation::constraint(
                    observation.schema_name(),
                    observation.relation_name(),
                    observation.relation_kind(),
                    observation.constraint_name(),
                )
                .is_ok_and(|observed_location| observed_location == location)
            });
        let observed_collation = self.collation_definitions_observed
            && self.collation_definitions.iter().any(|definition| {
                SchemaObjectLocation::collation(
                    definition.collation().schema_name(),
                    definition.collation().collation_name(),
                )
                .is_ok_and(|observed_location| observed_location == location)
            });
        if observed_not_null || observed_collation {
            return Ok(SuccessorSourceReceipt {
                source_id: self.inner.source_connection_key().to_owned(),
                connection_policy_binding: self.inner.connection_policy_binding().to_owned(),
                source_digest: self.snapshot_digest.clone(),
                extractor_revision: self.inner.extractor_revision().to_owned(),
                observed_at_utc: self.inner.observed_at_utc().to_owned(),
                location,
            });
        }
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

    /// Issues provenance for an exact observed source-schema owner, including an empty schema.
    ///
    /// The dedicated coordinate preserves the existing [`SchemaObjectLocation`] vocabulary.
    pub fn schema_owner_source_receipt(
        &self,
        location: SchemaOwnerLocation,
    ) -> Result<SchemaOwnerSourceReceipt, ObservationError> {
        let exists = self.schema_owners_observed
            && self
                .schema_owners
                .iter()
                .any(|owner| owner.schema_name() == location.schema_name());
        if !exists {
            return Err(ObservationError::UnknownObservationLocation {
                location: location.canonical_location(),
            });
        }
        Ok(SchemaOwnerSourceReceipt::new(
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
        if !observed_names.insert((relation.schema_name(), relation.relation_name())) {
            return Err(ObservationError::DuplicateRelationObservation {
                schema_name: relation.schema_name().to_owned(),
                relation_name: relation.relation_name().to_owned(),
            });
        }
    }
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
                && foreign_key
                    .reference_behavior()
                    .is_some_and(|behavior| behavior.match_type() == ForeignKeyMatchType::Partial)
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

        for primary_index in relation.indexes().iter().filter(|index| {
            index
                .catalog_flags()
                .is_some_and(|catalog_flags| catalog_flags.primary())
        }) {
            let matching_primary_key = relation.constraints().iter().any(|constraint| {
                matches!(constraint, TableConstraintObservation::PrimaryKey(_))
                    && constraint.constraint_name() == primary_index.index_name()
                    && primary_index.predicate().is_none()
                    && primary_index.nulls_not_distinct() != Some(true)
                    && primary_index.key_attributes().len() == constraint.column_names().len()
                    && primary_index
                        .key_attributes()
                        .iter()
                        .zip(constraint.column_names())
                        .all(|(attribute, column_name)| {
                            attribute.attribute_name() == Some(column_name.as_str())
                        })
            });
            if !matching_primary_key {
                return Err(ObservationError::InvalidObservationField {
                    field: "constraint_backing_index",
                });
            }
        }

        let clustered_index_count = relation
            .indexes()
            .iter()
            .filter(|index| {
                index
                    .catalog_flags()
                    .is_some_and(|catalog_flags| catalog_flags.clustered())
            })
            .count();
        if clustered_index_count > 1 {
            return Err(ObservationError::InvalidObservationField {
                field: "index_clustered",
            });
        }
        if relation.indexes().iter().any(|index| {
            index
                .catalog_flags()
                .is_some_and(|catalog_flags| catalog_flags.clustered())
                && index.predicate().is_some()
        }) {
            return Err(ObservationError::InvalidObservationField {
                field: "index_clustered",
            });
        }
        if relation.indexes().iter().any(|index| {
            index
                .catalog_flags()
                .is_some_and(|catalog_flags| catalog_flags.clustered())
                && index.valid() == Some(false)
        }) {
            return Err(ObservationError::InvalidObservationField {
                field: "index_clustered",
            });
        }

        let replica_identity_index_count = relation
            .indexes()
            .iter()
            .filter(|index| {
                index
                    .catalog_flags()
                    .is_some_and(|catalog_flags| catalog_flags.replica_identity())
            })
            .count();
        if replica_identity_index_count > 1 {
            return Err(ObservationError::InvalidObservationField {
                field: "index_replica_identity",
            });
        }
        if relation
            .replica_identity_mode()
            .is_some_and(|mode| mode != ReplicaIdentityMode::Index)
            && replica_identity_index_count != 0
        {
            return Err(ObservationError::InvalidObservationField {
                field: "index_replica_identity",
            });
        }

        for (replica_identity_index, catalog_flags) in
            relation.indexes().iter().filter_map(|index| {
                let catalog_flags = index.catalog_flags()?;
                catalog_flags
                    .replica_identity()
                    .then_some((index, catalog_flags))
            })
        {
            let relation_kind_is_eligible = matches!(
                relation.kind(),
                RelationKind::Table | RelationKind::PartitionedTable
            );
            let key_columns_are_not_null = !replica_identity_index.key_attributes().is_empty()
                && replica_identity_index
                    .key_attributes()
                    .iter()
                    .all(|attribute| {
                        attribute.attribute_name().is_some_and(|attribute_name| {
                            relation.columns().iter().any(|column| {
                                column.column_name() == attribute_name && !column.nullable()
                            })
                        })
                    });
            if !relation_kind_is_eligible
                || !replica_identity_index.is_unique()
                || !catalog_flags.immediate()
                || replica_identity_index.predicate().is_some()
                || !key_columns_are_not_null
            {
                return Err(ObservationError::InvalidObservationField {
                    field: "index_replica_identity",
                });
            }
        }

        for index in relation.indexes() {
            if !observed_names.insert((relation.schema_name(), index.index_name())) {
                return Err(ObservationError::InvalidObservationField {
                    field: "schema_relation_namespace",
                });
            }
        }
    }
    Ok(())
}

fn canonicalize_type_kind_observations(
    allowed_schema_names: Option<&[String]>,
    relations: &[RelationObservation],
    domains: &[DomainObservation],
    enums: &[EnumObservation],
    mut type_kinds: Vec<TypeKindObservation>,
) -> Result<Vec<TypeKindObservation>, ObservationError> {
    type_kinds.sort_by(|left, right| {
        (left.type_name().schema_name(), left.type_name().type_name()).cmp(&(
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
        .chain(
            enums
                .iter()
                .map(|observed_enum| observed_enum.schema_name()),
        )
        .collect::<BTreeSet<_>>();

    for type_kind in &type_kinds {
        let coordinate = type_kind.type_name();
        let explicitly_authorized = allowed_schema_names.is_some_and(|schema_names| {
            schema_names
                .iter()
                .any(|schema_name| schema_name == coordinate.schema_name())
        });
        if coordinate.schema_name() != POSTGRES_CATALOG_SCHEMA_NAME
            && !explicitly_authorized
            && !observed_schemas.contains(coordinate.schema_name())
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_schema",
            });
        }

        if let Some(domain) = domains.iter().find(|domain| {
            domain.schema_name() == coordinate.schema_name()
                && domain.domain_name() == coordinate.type_name()
        }) && (type_kind.kind() != PostgresTypeKind::Domain
            || !type_kind
                .domain_base_type()
                .is_some_and(|base| same_type_coordinate(base, domain.base_type())))
        {
            return Err(ObservationError::InvalidObservationField {
                field: "type_kind_domain_base",
            });
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
                PostgresTypeKind::Base | PostgresTypeKind::Range | PostgresTypeKind::Multirange
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
            binding, relations, domains, enums, type_kinds,
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
                    PostgresTypeKind::Base | PostgresTypeKind::Range | PostgresTypeKind::Multirange
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
    if let Some(replica_identity_mode) = relation.replica_identity_mode() {
        projected = projected.with_replica_identity_mode(replica_identity_mode);
    }
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

fn key_constraint_backing_index_static_shape_matches(
    constraint: &TableConstraintObservation,
    backing_index: &IndexObservation,
    catalog_flags: &IndexCatalogFlags,
) -> bool {
    let (constraint_columns, expected_nulls_not_distinct) = match constraint {
        TableConstraintObservation::PrimaryKey(primary_key) => (primary_key.column_names(), None),
        TableConstraintObservation::Unique(unique) => {
            (unique.column_names(), unique.nulls_not_distinct())
        }
        TableConstraintObservation::ForeignKey(_) | TableConstraintObservation::Check(_) => {
            return false;
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

    backing_index.is_unique()
        && key_columns_match
        && backing_index.predicate().is_none()
        && null_treatment_matches
        && catalog_flags.primary() == expected_primary
        && backing_index.ready() == Some(true)
        && backing_index.valid() == Some(true)
        && backing_index.live() == Some(true)
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
            relation
                .constraints()
                .iter()
                .filter(|constraint| {
                    matches!(
                        constraint,
                        TableConstraintObservation::PrimaryKey(_)
                            | TableConstraintObservation::Unique(_)
                    )
                })
                .map(move |constraint| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        constraint.constraint_name().to_owned(),
                    )
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
        let expected_immediate = matches!(
            timing.deferrability(),
            ConstraintDeferrability::NotDeferrable
        );
        let backing_access_method_matches = if catalog_flags.exclusion() {
            backing_index.access_method() == Some("gist")
        } else {
            backing_index.access_method() == Some("btree")
        };
        if !key_constraint_backing_index_static_shape_matches(
            constraint,
            backing_index,
            catalog_flags,
        ) || !backing_access_method_matches
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
            relation
                .constraints()
                .iter()
                .filter(|constraint| {
                    matches!(
                        constraint,
                        TableConstraintObservation::PrimaryKey(_)
                            | TableConstraintObservation::Unique(_)
                            | TableConstraintObservation::ForeignKey(_)
                    )
                })
                .map(move |constraint| {
                    (
                        relation.schema_name().to_owned(),
                        relation.relation_name().to_owned(),
                        relation.kind().token().to_owned(),
                        constraint.constraint_name().to_owned(),
                    )
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
                let backing_index = relation
                    .indexes()
                    .iter()
                    .find(|index| index.index_name() == period.constraint_name());
                if period.has_period_semantics() {
                    validate_constraint_period_column_type(relation, constraint, type_kinds)?;
                    let Some(exclusion_operators) = period.exclusion_operators() else {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_exclusion_operators",
                        });
                    };
                    if exclusion_operators.len() != constraint.column_names().len() {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_exclusion_operators",
                        });
                    }
                    let Some(backing_index) = backing_index else {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_backing_index",
                        });
                    };
                    let Some(catalog_flags) = backing_index.catalog_flags() else {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_backing_index",
                        });
                    };
                    if !key_constraint_backing_index_static_shape_matches(
                        constraint,
                        backing_index,
                        catalog_flags,
                    ) || !catalog_flags.exclusion()
                        || backing_index.access_method() != Some("gist")
                    {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_backing_index",
                        });
                    }
                } else {
                    if period.exclusion_operators().is_some() {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_exclusion_operators",
                        });
                    }
                    if let Some(backing_index) = backing_index
                        && let Some(catalog_flags) = backing_index.catalog_flags()
                        && catalog_flags.exclusion()
                    {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_backing_index",
                        });
                    }
                }
            }
            TableConstraintObservation::ForeignKey(foreign_key) => {
                if period.exclusion_operators().is_some() {
                    return Err(ObservationError::InvalidObservationField {
                        field: "constraint_period_exclusion_operators",
                    });
                }
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
                    let Some(referenced_relation) = relations.iter().find(|candidate| {
                        candidate.schema_name() == foreign_key.referenced_schema_name()
                            && candidate.relation_name() == foreign_key.referenced_table_name()
                    }) else {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_reference",
                        });
                    };
                    let referenced_temporal_key =
                        referenced_relation
                            .constraints()
                            .iter()
                            .find(|candidate_constraint| {
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
                            });
                    let Some(referenced_temporal_key) = referenced_temporal_key else {
                        return Err(ObservationError::InvalidObservationField {
                            field: "constraint_period_reference",
                        });
                    };
                    let referenced_key_is_nondeferrable = constraint_timings
                        .and_then(|timings| {
                            timings.iter().find(|timing| {
                                timing.schema_name() == referenced_relation.schema_name()
                                    && timing.relation_name() == referenced_relation.relation_name()
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
    type_kinds: &[TypeKindObservation],
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

        if array_type.array_type().schema_name() != POSTGRES_CATALOG_SCHEMA_NAME
            && !authorized_request
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
            && !type_kinds.iter().any(|kind| {
                same_type_coordinate(kind.type_name(), array_type.element_type())
                    && !array_type_names.contains(&element_coordinate)
            })
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
    if let Some(replica_identity_mode) = relation.replica_identity_mode() {
        projected = projected.with_replica_identity_mode(replica_identity_mode);
    }
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
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_TIMINGS_V1);
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
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V3_CONSTRAINT_PERIODS_V2);
    encode_str(&mut hasher, base_snapshot_digest);
    encode_len(&mut hasher, constraint_periods.len());
    for period in constraint_periods {
        encode_str(&mut hasher, period.schema_name());
        encode_str(&mut hasher, period.relation_name());
        encode_str(&mut hasher, period.relation_kind().token());
        encode_str(&mut hasher, period.constraint_name());
        encode_bool(&mut hasher, period.has_period_semantics());
        match period.exclusion_operators() {
            None => hasher.update([0]),
            Some(operators) => {
                hasher.update([1]);
                encode_len(&mut hasher, operators.len());
                for operator in operators {
                    hasher.update(operator.position().to_be_bytes());
                    encode_str(&mut hasher, operator.operator_schema_name());
                    encode_str(&mut hasher, operator.operator_name());
                    encode_str(&mut hasher, operator.left_type().schema_name());
                    encode_str(&mut hasher, operator.left_type().type_name());
                    encode_str(&mut hasher, operator.right_type().schema_name());
                    encode_str(&mut hasher, operator.right_type().type_name());
                }
            }
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
