#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Deterministic topology and evidence-membership checks for procedural drafts.
//!
//! This module validates a borrowed projection after strict input-shape checking.
//! It does not parse JSON, authenticate sources, evaluate natural-language edge
//! conditions, judge a task, publish a model or authorize execution. The projection
//! intentionally excludes labels/annotations: their preservation and validation
//! remain prerequisites of a future production adapter, not implied by this result.

use crate::EvidenceReference;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// Exact coordinates supplied separately by the draft and its trusted caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProceduralScope<'a> {
    /// Model identity within the tenant's authoring workspace.
    pub model_id: &'a str,
    /// Tenant reference; equality is not tenant authentication.
    pub tenant_ref: &'a str,
    /// Task family that the procedural representation describes.
    pub task_type: &'a str,
    /// Product/domain owner, not a transfer of its business truth.
    pub domain_owner_ref: &'a str,
}

/// Read-only structural projection of a single procedure definition.
#[derive(Clone, Copy, Debug)]
pub struct ProcedureNodeView<'a> {
    /// Logical identity, independent of its label or position in an array.
    pub procedure_id: &'a str,
    /// Exact references that must also occur in the model evidence inventory.
    pub source_evidence: &'a [EvidenceReference],
}

/// Initial advisory relation vocabulary; none of these variants grants a capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProceduralRelationKind {
    /// A described next-step connection, not an executable transition.
    LeadsTo,
    /// A described prerequisite, not an evaluated policy or OWL restriction.
    Requires,
    /// A described enabling relationship, not an authorization result.
    Enables,
}

/// Read-only directed relation projection, including its supporting evidence.
#[derive(Clone, Copy, Debug)]
pub struct ProcedureRelationView<'a> {
    /// Source procedure; direction is preserved without interpreting prose.
    pub source_procedure_id: &'a str,
    /// Typed advisory relationship.
    pub relation_type: ProceduralRelationKind,
    /// Target procedure that must exist in the same model.
    pub target_procedure_id: &'a str,
    /// Exact evidence coordinates for this relationship.
    pub source_evidence: &'a [EvidenceReference],
}

/// Structural input only; callers must not treat construction as admission.
#[derive(Clone, Copy, Debug)]
pub struct ProceduralModelView<'a> {
    /// Scope claimed by the candidate being examined.
    pub scope: ProceduralScope<'a>,
    /// A procedure identity that must occur in the candidate's node set.
    pub entry_procedure_id: &'a str,
    /// Complete allowed evidence inventory for this projection.
    pub source_evidence: &'a [EvidenceReference],
    /// Procedure records, including distinct records that may share an invalid ID.
    pub procedure_nodes: &'a [ProcedureNodeView<'a>],
    /// Directed advisory relations, not instructions to execute.
    pub procedure_relations: &'a [ProcedureRelationView<'a>],
}

/// Explicit authoring policy; cycles are legal in both modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReachabilityRule {
    /// Permit disconnected incomplete authoring drafts, but never dangling references.
    AllowPartialDraft,
    /// Require every node to be reachable from the entry along directed relations.
    RequireEntryReachability,
}

/// Structural counts only; no field means approved, published or executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TopologySummary {
    /// Number of uniquely identified procedure definitions checked.
    pub procedure_count: usize,
    /// Number of uniquely identified typed relations checked.
    pub relation_count: usize,
    /// Number of nodes reachable under structural, not operational, traversal.
    pub reachable_count: usize,
}

/// Fixed error codes avoid returning untrusted identifiers or evidence text.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProceduralValidationError {
    /// Identifier violates the local authoring grammar.
    InvalidIdentity,
    /// Claimed scope differs from the independently supplied caller scope.
    ScopeMismatch,
    /// A collection is empty where required or exceeds the local bound.
    CollectionLimit,
    /// Two procedure records use the same logical identity.
    DuplicateProcedure,
    /// The declared entry does not exist in the node set.
    MissingEntry,
    /// A relation endpoint does not exist in the node set.
    DanglingRelation,
    /// A source/type/target triple occurs more than once.
    DuplicateRelation,
    /// Evidence syntax is invalid; this does not identify authentic evidence.
    InvalidEvidence,
    /// The same exact evidence coordinate is repeated in one inventory.
    DuplicateEvidence,
    /// One snapshot identity is associated with conflicting digest strings.
    ConflictingSourceRevision,
    /// Node or relation evidence is absent from the model's root inventory.
    UnboundEvidence,
    /// The selected policy requires entry reachability that is not satisfied.
    UnreachableProcedure,
    /// Total examined identity/evidence bytes exceed the projection budget.
    EvidenceBudgetExceeded,
}
impl fmt::Display for ProceduralValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Self::InvalidIdentity => "invalid_identity",
            Self::ScopeMismatch => "scope_mismatch",
            Self::CollectionLimit => "collection_limit",
            Self::DuplicateProcedure => "duplicate_procedure",
            Self::MissingEntry => "missing_entry",
            Self::DanglingRelation => "dangling_relation",
            Self::DuplicateRelation => "duplicate_relation",
            Self::InvalidEvidence => "invalid_evidence",
            Self::DuplicateEvidence => "duplicate_evidence",
            Self::ConflictingSourceRevision => "conflicting_source_revision",
            Self::UnboundEvidence => "unbound_evidence",
            Self::UnreachableProcedure => "unreachable_procedure",
            Self::EvidenceBudgetExceeded => "evidence_budget_exceeded",
        };
        formatter.write_str(code)
    }
}
impl std::error::Error for ProceduralValidationError {}

type EvidenceKey<'a> = (&'a str, &'a str, &'a str);

fn identity(value: &str) -> Result<(), ProceduralValidationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes.len() > 128
        || !(bytes[0].is_ascii_alphanumeric() || bytes[0] == b'_')
        || !bytes.iter().all(|byte| byte.is_ascii_alphanumeric() || b"._:/@+=#-".contains(byte))
    {
        return Err(ProceduralValidationError::InvalidIdentity);
    }
    Ok(())
}
fn charge(bytes: usize, budget: &mut usize) -> Result<(), ProceduralValidationError> {
    *budget = budget.saturating_add(bytes);
    if *budget > 1_048_576 { return Err(ProceduralValidationError::EvidenceBudgetExceeded); }
    Ok(())
}
fn evidence_keys<'a>(references: &'a [EvidenceReference], budget: &mut usize) -> Result<BTreeSet<EvidenceKey<'a>>, ProceduralValidationError> {
    if references.is_empty() || references.len() > 64 { return Err(ProceduralValidationError::CollectionLimit); }
    let mut keys = BTreeSet::new();
    let mut revisions = BTreeMap::new();
    for reference in references {
        let source = reference.source_id();
        let digest = reference.source_digest();
        let location = reference.location();
        identity(source).map_err(|_| ProceduralValidationError::InvalidEvidence)?;
        if digest.len() != 71 || !digest.starts_with("sha256:")
            || !digest.as_bytes()[7..].iter().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
            || location.len() > 8192 || location.chars().count() > 2048
            || location.trim().is_empty() || location.contains('\0')
        { return Err(ProceduralValidationError::InvalidEvidence); }
        charge(source.len() + digest.len() + location.len(), budget)?;
        if let Some(previous) = revisions.insert(source, digest) {
            if previous != digest { return Err(ProceduralValidationError::ConflictingSourceRevision); }
        }
        if !keys.insert((source, digest, location)) { return Err(ProceduralValidationError::DuplicateEvidence); }
    }
    Ok(keys)
}

/// Checks logical identities, endpoint/entry membership, evidence closure and reachability.
///
/// `expected_scope` must come from the calling application's authenticated context,
/// not be copied from the candidate. Equality alone does not authenticate that context.
/// At most 256 nodes, 512 relations and 64 references per evidence set are examined.
/// A 1 MiB cumulative budget covers this projection's identifiers/evidence, not omitted
/// annotations or serialized input. Transport bounds must be enforced before parsing.
///
/// Returns counts only. Success is neither a full semantic-validation receipt nor a
/// publishable aggregate: source signatures, factual correctness, annotation semantics,
/// tool contracts, version lineage, evaluation and stewardship remain separate gates.
pub fn validate_procedural_model(
    model: &ProceduralModelView<'_>,
    expected_scope: ProceduralScope<'_>,
    reachability: ReachabilityRule,
) -> Result<TopologySummary, ProceduralValidationError> {
    for scope in [model.scope, expected_scope] {
        for value in [scope.model_id, scope.tenant_ref, scope.task_type, scope.domain_owner_ref] { identity(value)?; }
    }
    if model.scope != expected_scope { return Err(ProceduralValidationError::ScopeMismatch); }
    if model.procedure_nodes.is_empty() || model.procedure_nodes.len() > 256 || model.procedure_relations.len() > 512 {
        return Err(ProceduralValidationError::CollectionLimit);
    }
    identity(model.entry_procedure_id)?;
    let mut budget = 0;
    for value in [model.scope.model_id, model.scope.tenant_ref, model.scope.task_type, model.scope.domain_owner_ref, model.entry_procedure_id] { charge(value.len(), &mut budget)?; }
    let inventory = evidence_keys(model.source_evidence, &mut budget)?;
    let mut nodes = BTreeSet::new();
    for node in model.procedure_nodes {
        identity(node.procedure_id)?;
        charge(node.procedure_id.len(), &mut budget)?;
        if !nodes.insert(node.procedure_id) { return Err(ProceduralValidationError::DuplicateProcedure); }
        if !evidence_keys(node.source_evidence, &mut budget)?.is_subset(&inventory) { return Err(ProceduralValidationError::UnboundEvidence); }
    }
    if !nodes.contains(model.entry_procedure_id) { return Err(ProceduralValidationError::MissingEntry); }
    let mut triples = BTreeSet::new();
    let mut adjacency: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in model.procedure_relations {
        identity(edge.source_procedure_id)?; identity(edge.target_procedure_id)?;
        charge(edge.source_procedure_id.len() + edge.target_procedure_id.len(), &mut budget)?;
        if !nodes.contains(edge.source_procedure_id) || !nodes.contains(edge.target_procedure_id) { return Err(ProceduralValidationError::DanglingRelation); }
        if !triples.insert((edge.source_procedure_id, edge.relation_type, edge.target_procedure_id)) { return Err(ProceduralValidationError::DuplicateRelation); }
        if !evidence_keys(edge.source_evidence, &mut budget)?.is_subset(&inventory) { return Err(ProceduralValidationError::UnboundEvidence); }
        adjacency.entry(edge.source_procedure_id).or_default().push(edge.target_procedure_id);
    }
    let mut visited = BTreeSet::from([model.entry_procedure_id]);
    let mut pending = vec![model.entry_procedure_id];
    while let Some(node) = pending.pop() {
        if let Some(neighbors) = adjacency.get(node) {
            for &neighbor in neighbors { if visited.insert(neighbor) { pending.push(neighbor); } }
        }
    }
    if reachability == ReachabilityRule::RequireEntryReachability && visited.len() != nodes.len() { return Err(ProceduralValidationError::UnreachableProcedure); }
    Ok(TopologySummary { procedure_count: nodes.len(), relation_count: triples.len(), reachable_count: visited.len() })
}
