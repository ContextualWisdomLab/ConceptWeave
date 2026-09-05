//! Immutable PostgreSQL schema-observation contracts for ConceptWeave.
//!
//! The public aggregate derives source-content identity from deterministic observed metadata.
//! Source connection, extractor revision, and observation time remain separate provenance
//! coordinates and therefore do not change the source-content digest.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

mod model;

pub use model::{
    CheckConstraintObservation, ColumnObservation, ForeignKeyAction, ForeignKeyDeferrability,
    ForeignKeyMatchType, ForeignKeyObservation, ForeignKeyReferenceBehavior, ObservationError,
    ObservationLocation, ObservationLocationKind, PrimaryKeyObservation, SourceObservationReceipt,
    TableConstraintObservation, TableObservation, UniqueConstraintObservation,
};

use conceptweave_source_port::ResolvedSourceConnection;
use sha2::{Digest, Sha256};

const SNAPSHOT_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v1";

/// Immutable evidence that one bounded PostgreSQL schema snapshot was observed.
///
/// The snapshot digest is computed by ConceptWeave from a versioned, domain-separated,
/// deterministic framing of the exact observed table, column, and constraint metadata. Source
/// registry identity, extractor revision, and observation time remain separate provenance
/// coordinates and do not participate in source-content identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostgresSchemaSnapshot(model::PostgresSchemaSnapshot);

impl PostgresSchemaSnapshot {
    /// Creates a deterministic snapshot contract from already-bounded source metadata.
    ///
    /// Collection order is canonicalized by exact qualified table identifier before the digest is
    /// computed. Exact UTF-8 source text is preserved without Unicode, case, or quoting
    /// normalization. The source connection reference must be a registry-resolved capability
    /// issued by the Source Observation port. The observation time remains explicit provenance and
    /// must use the canonical UTC form enforced by the underlying observation contract.
    pub fn new(
        source_connection: &ResolvedSourceConnection,
        extractor_revision: impl Into<String>,
        observed_at_utc: impl Into<String>,
        mut tables: Vec<TableObservation>,
    ) -> Result<Self, ObservationError> {
        tables.sort_by(|left, right| {
            (left.schema_name(), left.table_name()).cmp(&(right.schema_name(), right.table_name()))
        });
        let snapshot_digest = compute_snapshot_digest(&tables);
        model::PostgresSchemaSnapshot::new(
            source_connection,
            snapshot_digest,
            extractor_revision,
            observed_at_utc,
            tables,
        )
        .map(Self)
    }

    /// Returns the stable source-connection registry reference, never a credential.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        self.0.source_connection_key()
    }

    /// Returns the owner-computed canonical SHA-256 source-content digest.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        self.0.snapshot_digest()
    }

    /// Returns the exact extractor implementation/configuration revision.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        self.0.extractor_revision()
    }

    /// Returns the exact UTC observation-time evidence supplied by the adapter.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        self.0.observed_at_utc()
    }

    /// Returns qualified tables in deterministic exact-identifier order.
    #[must_use]
    pub fn tables(&self) -> &[TableObservation] {
        self.0.tables()
    }

    /// Issues provenance for an exact coordinate only when that coordinate exists in this snapshot.
    pub fn source_receipt(
        &self,
        location: ObservationLocation,
    ) -> Result<SourceObservationReceipt, ObservationError> {
        self.0.source_receipt(location)
    }
}

fn compute_snapshot_digest(tables: &[TableObservation]) -> String {
    let mut hasher = Sha256::new();
    encode_bytes(&mut hasher, SNAPSHOT_DIGEST_DOMAIN_V1);
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

fn encode_reference_behavior(
    hasher: &mut Sha256,
    behavior: Option<&ForeignKeyReferenceBehavior>,
) {
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

fn encode_foreign_key_deferrability(
    hasher: &mut Sha256,
    deferrability: ForeignKeyDeferrability,
) {
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
        let error = model::PostgresSchemaSnapshot::new(
            &resolved_source(),
            "not-a-digest",
            "postgres_introspector_v1",
            "2026-09-05T03:30:00Z",
            Vec::new(),
        )
        .expect_err("the private storage model must still fail closed on malformed digest input");

        assert_eq!(
            error,
            model::ObservationError::InvalidObservationField {
                field: "snapshot_digest"
            }
        );
    }
}
