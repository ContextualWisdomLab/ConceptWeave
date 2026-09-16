//! PostgreSQL ordinary exclusion-constraint catalog-family shape evidence.
//!
//! `pg_constraint` stores several fields that are meaningful only for domain, foreign-key, or CHECK
//! constraints. Ordinary `contype = 'x'` EXCLUDE rows must not silently retain residue from those
//! other families. This successor requires explicit adapter evidence for the family discriminator,
//! table/domain ownership sentinels, foreign-key-only payload, and CHECK expression presence before
//! governed evidence is issued. It does not synthesize those facts from the EXCLUDE coordinate.

use std::collections::BTreeSet;

use conceptweave_observation::ObservationError;
use sha2::{Digest, Sha256};

use super::{IndexExclusionConstraintCoordinate, IndexExclusionConstraintSnapshot};

const INDEX_EXCLUSION_CONSTRAINT_CATALOG_SHAPE_DIGEST_DOMAIN_V1: &[u8] =
    b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.catalog_shape.v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Raw foreign-key action-code sentinels observed on one ordinary EXCLUDE catalog row.
///
/// PostgreSQL stores spaces in these three fixed-width fields for non-foreign-key constraints. The
/// values are retained independently rather than inferred from `contype`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintForeignActionCodes {
    update_action: char,
    delete_action: char,
    match_type: char,
}

impl IndexExclusionConstraintForeignActionCodes {
    /// Records raw `confupdtype`, `confdeltype`, and `confmatchtype` catalog characters.
    #[must_use]
    pub const fn new(update_action: char, delete_action: char, match_type: char) -> Self {
        Self {
            update_action,
            delete_action,
            match_type,
        }
    }

    /// Returns raw `pg_constraint.confupdtype`.
    #[must_use]
    pub const fn update_action(self) -> char {
        self.update_action
    }

    /// Returns raw `pg_constraint.confdeltype`.
    #[must_use]
    pub const fn delete_action(self) -> char {
        self.delete_action
    }

    /// Returns raw `pg_constraint.confmatchtype`.
    #[must_use]
    pub const fn match_type(self) -> char {
        self.match_type
    }

    const fn is_non_foreign_key_sentinel(self) -> bool {
        self.update_action == ' ' && self.delete_action == ' ' && self.match_type == ' '
    }
}

/// Presence state for catalog payload defined only for foreign-key constraints.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintForeignPayloadPresence {
    foreign_relation_present: bool,
    foreign_key_columns_present: bool,
    equality_operator_vectors_present: [bool; 3],
    delete_set_columns_present: bool,
}

impl IndexExclusionConstraintForeignPayloadPresence {
    /// Records whether FK-only fields are physically present on the observed row.
    ///
    /// `equality_operator_vectors_present` is ordered as `conpfeqop`, `conppeqop`, `conffeqop`.
    #[must_use]
    pub const fn new(
        foreign_relation_present: bool,
        foreign_key_columns_present: bool,
        equality_operator_vectors_present: [bool; 3],
        delete_set_columns_present: bool,
    ) -> Self {
        Self {
            foreign_relation_present,
            foreign_key_columns_present,
            equality_operator_vectors_present,
            delete_set_columns_present,
        }
    }

    /// Returns whether `confrelid` was nonzero.
    #[must_use]
    pub const fn foreign_relation_present(self) -> bool {
        self.foreign_relation_present
    }

    /// Returns whether `confkey` was non-NULL.
    #[must_use]
    pub const fn foreign_key_columns_present(self) -> bool {
        self.foreign_key_columns_present
    }

    /// Returns presence of `conpfeqop`, `conppeqop`, and `conffeqop`, in that order.
    #[must_use]
    pub const fn equality_operator_vectors_present(self) -> [bool; 3] {
        self.equality_operator_vectors_present
    }

    /// Returns whether `confdelsetcols` was non-NULL.
    #[must_use]
    pub const fn delete_set_columns_present(self) -> bool {
        self.delete_set_columns_present
    }

    const fn is_empty(self) -> bool {
        !self.foreign_relation_present
            && !self.foreign_key_columns_present
            && !self.equality_operator_vectors_present[0]
            && !self.equality_operator_vectors_present[1]
            && !self.equality_operator_vectors_present[2]
            && !self.delete_set_columns_present
    }
}

/// Exact mutually exclusive `pg_constraint` family shape for one ordinary EXCLUDE row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintCatalogShapeObservation {
    coordinate: IndexExclusionConstraintCoordinate,
    constraint_type_code: char,
    relation_owner_present: bool,
    domain_owner_present: bool,
    foreign_action_codes: IndexExclusionConstraintForeignActionCodes,
    foreign_payload_presence: IndexExclusionConstraintForeignPayloadPresence,
    check_expression_present: bool,
}

impl IndexExclusionConstraintCatalogShapeObservation {
    /// Records independent family-shape evidence for one ordinary EXCLUDE catalog row.
    ///
    /// The accepted PostgreSQL 18 shape is `contype='x'`, nonzero `conrelid`, zero `contypid`, space
    /// foreign-action sentinels, no FK-only variable-length payload, and NULL `conbin`. `conkey` and
    /// `conexclop` are intentionally outside this value object because dedicated successors preserve
    /// their exact ordered values.
    pub fn new(
        coordinate: IndexExclusionConstraintCoordinate,
        constraint_type_code: char,
        relation_owner_present: bool,
        domain_owner_present: bool,
        foreign_action_codes: IndexExclusionConstraintForeignActionCodes,
        foreign_payload_presence: IndexExclusionConstraintForeignPayloadPresence,
        check_expression_present: bool,
    ) -> Result<Self, ObservationError> {
        if constraint_type_code != 'x' {
            return Err(invalid("index_exclusion_constraint_catalog_type"));
        }
        if !relation_owner_present
            || domain_owner_present
            || !foreign_action_codes.is_non_foreign_key_sentinel()
            || !foreign_payload_presence.is_empty()
            || check_expression_present
        {
            return Err(invalid("index_exclusion_constraint_catalog_shape"));
        }
        Ok(Self {
            coordinate,
            constraint_type_code,
            relation_owner_present,
            domain_owner_present,
            foreign_action_codes,
            foreign_payload_presence,
            check_expression_present,
        })
    }

    /// Returns the exact ordinary EXCLUDE coordinate.
    #[must_use]
    pub const fn coordinate(&self) -> &IndexExclusionConstraintCoordinate {
        &self.coordinate
    }

    /// Returns raw `pg_constraint.contype`.
    #[must_use]
    pub const fn constraint_type_code(&self) -> char {
        self.constraint_type_code
    }

    /// Returns whether raw `pg_constraint.conrelid` was nonzero.
    #[must_use]
    pub const fn relation_owner_present(&self) -> bool {
        self.relation_owner_present
    }

    /// Returns whether raw `pg_constraint.contypid` was nonzero.
    #[must_use]
    pub const fn domain_owner_present(&self) -> bool {
        self.domain_owner_present
    }

    /// Returns raw non-FK action-code sentinels.
    #[must_use]
    pub const fn foreign_action_codes(&self) -> IndexExclusionConstraintForeignActionCodes {
        self.foreign_action_codes
    }

    /// Returns presence state for FK-only payload.
    #[must_use]
    pub const fn foreign_payload_presence(&self) -> IndexExclusionConstraintForeignPayloadPresence {
        self.foreign_payload_presence
    }

    /// Returns whether `pg_constraint.conbin` was non-NULL.
    #[must_use]
    pub const fn check_expression_present(&self) -> bool {
        self.check_expression_present
    }

    /// Returns a collision-safe source location for family-shape evidence.
    #[must_use]
    pub fn canonical_location(&self) -> String {
        catalog_shape_location(&self.coordinate)
    }
}

/// Immutable provenance receipt for one exact ordinary EXCLUDE family-shape observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintCatalogShapeSourceReceipt {
    source_id: String,
    connection_policy_binding: String,
    source_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    location: IndexExclusionConstraintCatalogShapeObservation,
}

impl IndexExclusionConstraintCatalogShapeSourceReceipt {
    /// Returns the stable source registry key, never credential material.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the owner-computed family-shape successor digest.
    #[must_use]
    pub fn source_digest(&self) -> &str {
        &self.source_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the exact catalog-family shape observation.
    #[must_use]
    pub const fn location(&self) -> &IndexExclusionConstraintCatalogShapeObservation {
        &self.location
    }
}

/// Complete mutually exclusive catalog-family shape over ordinary EXCLUDE constraints.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExclusionConstraintCatalogShapeSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
    observations: Vec<IndexExclusionConstraintCatalogShapeObservation>,
}

impl IndexExclusionConstraintCatalogShapeSnapshot {
    /// Creates complete family-shape evidence over one exact ordinary EXCLUDE predecessor.
    pub fn new(
        constraint_snapshot: &IndexExclusionConstraintSnapshot,
        mut observations: Vec<IndexExclusionConstraintCatalogShapeObservation>,
    ) -> Result<Self, ObservationError> {
        observations.sort_by(|left, right| left.coordinate().cmp(right.coordinate()));
        let expected = constraint_snapshot
            .observations()
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();
        let observed = observations
            .iter()
            .map(|observation| observation.coordinate().clone())
            .collect::<BTreeSet<_>>();

        if observed.len() != observations.len() {
            return Err(invalid("index_exclusion_constraint_catalog_shape_coordinate"));
        }
        if observed != expected {
            return Err(invalid(
                "index_exclusion_constraint_catalog_shape_completeness",
            ));
        }

        let snapshot_digest = compute_catalog_shape_digest(
            constraint_snapshot.snapshot_digest(),
            &observations,
        );
        Ok(Self {
            source_connection_key: constraint_snapshot.source_connection_key().to_owned(),
            connection_policy_binding: constraint_snapshot.connection_policy_binding().to_owned(),
            snapshot_digest,
            extractor_revision: constraint_snapshot.extractor_revision().to_owned(),
            observed_at_utc: constraint_snapshot.observed_at_utc().to_owned(),
            observations,
        })
    }

    /// Returns the stable source-connection registry key.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable source-policy binding.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the domain-separated family-shape successor digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical UTC observation time inherited from the predecessor.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns complete family-shape observations in deterministic coordinate order.
    #[must_use]
    pub fn observations(&self) -> &[IndexExclusionConstraintCatalogShapeObservation] {
        &self.observations
    }

    /// Issues exact provenance for one observed ordinary EXCLUDE family shape.
    pub fn source_receipt(
        &self,
        coordinate: IndexExclusionConstraintCoordinate,
    ) -> Result<IndexExclusionConstraintCatalogShapeSourceReceipt, ObservationError> {
        let observation = self
            .observations
            .iter()
            .find(|observation| observation.coordinate() == &coordinate)
            .ok_or_else(|| ObservationError::UnknownObservationLocation {
                location: catalog_shape_location(&coordinate),
            })?;
        Ok(IndexExclusionConstraintCatalogShapeSourceReceipt {
            source_id: self.source_connection_key.clone(),
            connection_policy_binding: self.connection_policy_binding.clone(),
            source_digest: self.snapshot_digest.clone(),
            extractor_revision: self.extractor_revision.clone(),
            observed_at_utc: self.observed_at_utc.clone(),
            location: observation.clone(),
        })
    }
}

fn compute_catalog_shape_digest(
    predecessor_digest: &str,
    observations: &[IndexExclusionConstraintCatalogShapeObservation],
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(INDEX_EXCLUSION_CONSTRAINT_CATALOG_SHAPE_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, predecessor_digest);
    encode_len(&mut hasher, observations.len());
    for observation in observations {
        encode_coordinate(&mut hasher, observation.coordinate());
        encode_char(&mut hasher, observation.constraint_type_code());
        encode_bool(&mut hasher, observation.relation_owner_present());
        encode_bool(&mut hasher, observation.domain_owner_present());
        let actions = observation.foreign_action_codes();
        encode_char(&mut hasher, actions.update_action());
        encode_char(&mut hasher, actions.delete_action());
        encode_char(&mut hasher, actions.match_type());
        let payload = observation.foreign_payload_presence();
        encode_bool(&mut hasher, payload.foreign_relation_present());
        encode_bool(&mut hasher, payload.foreign_key_columns_present());
        for present in payload.equality_operator_vectors_present() {
            encode_bool(&mut hasher, present);
        }
        encode_bool(&mut hasher, payload.delete_set_columns_present());
        encode_bool(&mut hasher, observation.check_expression_present());
    }
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn catalog_shape_location(coordinate: &IndexExclusionConstraintCoordinate) -> String {
    format!("{}/catalog-family-shape", coordinate.canonical_location())
}

fn encode_coordinate(hasher: &mut Sha256, coordinate: &IndexExclusionConstraintCoordinate) {
    encode_str(hasher, coordinate.schema_name());
    encode_str(hasher, coordinate.relation_name());
    encode_str(hasher, coordinate.relation_kind().token());
    encode_str(hasher, coordinate.constraint_name());
}

fn encode_len(hasher: &mut Sha256, value: usize) {
    let value = u64::try_from(value).expect("Rust target usize must fit into canonical u64 length");
    hasher.update(value.to_be_bytes());
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    encode_len(hasher, value.len());
    hasher.update(value.as_bytes());
}

fn encode_char(hasher: &mut Sha256, value: char) {
    hasher.update(u32::from(value).to_be_bytes());
}

fn encode_bool(hasher: &mut Sha256, value: bool) {
    hasher.update([u8::from(value)]);
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
