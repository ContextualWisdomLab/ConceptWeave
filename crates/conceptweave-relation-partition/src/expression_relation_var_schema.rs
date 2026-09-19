//! Composed PostgreSQL 18 expression equality proof for relation-`Var` trees.
//!
//! Historical node-schema v1 proves complete modeled non-`Var` node fields but admits incomplete
//! relation-`Var` leaves. Relation-Var v1 proves those leaves completely, but intentionally branches
//! from raw expression semantics and therefore does not itself prove the enclosing node schemas.
//! This successor binds both exact proofs without redefining either issued predecessor.

use conceptweave_observation::{ObservationError, PostgresSchemaSnapshotV3};
use sha2::{Digest, Sha256};

use crate::RelationPartitionSnapshot;

use super::{
    IndexExclusionSemanticsSnapshot, IndexExpressionNodeSchemaSnapshot,
    IndexExpressionRelationVarSnapshot, IndexExpressionSemanticsSnapshot,
    IndexOperatorFamilySnapshot, IndexPartitionSnapshot, RelationPartitionTypeModifierSnapshot,
};

const COMPOSED_SCHEMA_DIGEST_DOMAIN_V1: &[u8] = b"conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.relation_var.node_schema.v1";
const COMPOSED_SCHEMA_REVISION_V1: &str =
    "postgresql-18-equal-node-schema-plus-relation-var-v1";
const SHA256_DIGEST_PREFIX: &str = "sha256:";

/// Immutable proof that one exact Var-bearing expression predecessor satisfies both historical
/// non-`Var` node-field completeness and complete relation-`Var` equality semantics.
///
/// The constructor rebound-validates node-schema v1 from the exact expression predecessor and
/// relation-Var v1 from the entire bounded predecessor stack. A Var-bearing tree can therefore use
/// v1's complete `FuncExpr`/`OpExpr` field schemas without pretending that node-schema v2 — which
/// deliberately rejects every relation `Var` — could be its predecessor. Unsupported node kinds and
/// `Const` remain rejected by node-schema v1 until their complete PostgreSQL equality schemas exist.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexExpressionRelationVarNodeSchemaSnapshot {
    source_connection_key: String,
    connection_policy_binding: String,
    node_schema_predecessor_digest: String,
    relation_var_predecessor_digest: String,
    snapshot_digest: String,
    extractor_revision: String,
    observed_at_utc: String,
}

impl IndexExpressionRelationVarNodeSchemaSnapshot {
    /// Rebound-validates both exact predecessor proofs and frames their conjunction under a new
    /// domain-separated digest.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        base_snapshot: &PostgresSchemaSnapshotV3,
        relation_partition_snapshot: &RelationPartitionSnapshot,
        index_partition_snapshot: &IndexPartitionSnapshot,
        operator_family_snapshot: &IndexOperatorFamilySnapshot,
        exclusion_snapshot: &IndexExclusionSemanticsSnapshot,
        expression_snapshot: &IndexExpressionSemanticsSnapshot,
        type_modifier_snapshot: &RelationPartitionTypeModifierSnapshot,
        node_schema_predecessor: &IndexExpressionNodeSchemaSnapshot,
        relation_var_predecessor: &IndexExpressionRelationVarSnapshot,
    ) -> Result<Self, ObservationError> {
        let rebound_node_schema = IndexExpressionNodeSchemaSnapshot::new(expression_snapshot)?;
        if rebound_node_schema.snapshot_digest() != node_schema_predecessor.snapshot_digest()
            || rebound_node_schema.predecessor_digest()
                != node_schema_predecessor.predecessor_digest()
        {
            return Err(invalid("index_expression_relation_var_node_schema_predecessor"));
        }

        let rebound_relation_var = IndexExpressionRelationVarSnapshot::new(
            base_snapshot,
            relation_partition_snapshot,
            index_partition_snapshot,
            operator_family_snapshot,
            exclusion_snapshot,
            expression_snapshot,
            type_modifier_snapshot,
            relation_var_predecessor.observations().to_vec(),
        )?;
        if rebound_relation_var.snapshot_digest() != relation_var_predecessor.snapshot_digest() {
            return Err(invalid("index_expression_relation_var_predecessor"));
        }

        if node_schema_predecessor.source_connection_key()
            != relation_var_predecessor.source_connection_key()
            || node_schema_predecessor.connection_policy_binding()
                != relation_var_predecessor.connection_policy_binding()
            || node_schema_predecessor.extractor_revision()
                != relation_var_predecessor.extractor_revision()
            || node_schema_predecessor.observed_at_utc()
                != relation_var_predecessor.observed_at_utc()
        {
            return Err(invalid("index_expression_relation_var_node_schema_provenance"));
        }

        let node_schema_predecessor_digest = node_schema_predecessor.snapshot_digest().to_owned();
        let relation_var_predecessor_digest = relation_var_predecessor.snapshot_digest().to_owned();
        let snapshot_digest = compute_digest(
            &node_schema_predecessor_digest,
            &relation_var_predecessor_digest,
        );

        Ok(Self {
            source_connection_key: relation_var_predecessor.source_connection_key().to_owned(),
            connection_policy_binding: relation_var_predecessor
                .connection_policy_binding()
                .to_owned(),
            node_schema_predecessor_digest,
            relation_var_predecessor_digest,
            snapshot_digest,
            extractor_revision: relation_var_predecessor.extractor_revision().to_owned(),
            observed_at_utc: relation_var_predecessor.observed_at_utc().to_owned(),
        })
    }

    /// Returns the stable source registry key inherited from the bounded predecessor stack.
    #[must_use]
    pub fn source_connection_key(&self) -> &str {
        &self.source_connection_key
    }

    /// Returns the immutable connection-policy binding inherited from the predecessor stack.
    #[must_use]
    pub fn connection_policy_binding(&self) -> &str {
        &self.connection_policy_binding
    }

    /// Returns the exact historical node-schema-v1 digest used by this composed proof.
    #[must_use]
    pub fn node_schema_predecessor_digest(&self) -> &str {
        &self.node_schema_predecessor_digest
    }

    /// Returns the exact relation-Var-v1 digest used by this composed proof.
    #[must_use]
    pub fn relation_var_predecessor_digest(&self) -> &str {
        &self.relation_var_predecessor_digest
    }

    /// Returns the domain-separated digest proving the conjunction of both exact predecessors.
    #[must_use]
    pub fn snapshot_digest(&self) -> &str {
        &self.snapshot_digest
    }

    /// Returns the exact extractor revision inherited from the predecessor stack.
    #[must_use]
    pub fn extractor_revision(&self) -> &str {
        &self.extractor_revision
    }

    /// Returns the exact canonical observation time inherited from the predecessor stack.
    #[must_use]
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }

    /// Returns the semantic revision of this composed proof family.
    #[must_use]
    pub const fn schema_revision() -> &'static str {
        COMPOSED_SCHEMA_REVISION_V1
    }
}

fn compute_digest(node_schema_digest: &str, relation_var_digest: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(COMPOSED_SCHEMA_DIGEST_DOMAIN_V1);
    encode_str(&mut hasher, COMPOSED_SCHEMA_REVISION_V1);
    encode_str(&mut hasher, node_schema_digest);
    encode_str(&mut hasher, relation_var_digest);
    format!("{SHA256_DIGEST_PREFIX}{:x}", hasher.finalize())
}

fn encode_str(hasher: &mut Sha256, value: &str) {
    let len = u64::try_from(value.len()).expect("semantic string length fits in u64");
    hasher.update(len.to_be_bytes());
    hasher.update(value.as_bytes());
}

fn invalid(field: &'static str) -> ObservationError {
    ObservationError::InvalidObservationField { field }
}
