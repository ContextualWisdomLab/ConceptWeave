#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Deterministic semantic/topology admission for borrowed procedural drafts.
//!
//! This module validates a lossless schema-significant projection after strict
//! transport parsing. It does not parse JSON, authenticate sources or referenced
//! artifacts, evaluate natural-language edge conditions, judge a task, publish a
//! model or authorize execution. Artifact references remain coordinates only.

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

/// Procedure category from the authoring contract; no variant grants authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProcedureKind {
    /// A proposed tool operation that still requires independent capability admission.
    ToolOperation,
    /// A reasoning step represented as procedural guidance.
    ReasoningStep,
    /// A reusable skill-level procedure.
    SkillProcedure,
    /// A task-state procedure in the authored graph.
    TaskState,
}

/// Bounded labels/advice for the eight locales supported by the draft contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocaleAnnotationsView<'a> {
    /// Korean annotation.
    pub ko: Option<&'a str>,
    /// English annotation.
    pub en: Option<&'a str>,
    /// Japanese annotation.
    pub ja: Option<&'a str>,
    /// Chinese annotation.
    pub zh: Option<&'a str>,
    /// Vietnamese annotation.
    pub vi: Option<&'a str>,
    /// Spanish annotation.
    pub es: Option<&'a str>,
    /// German annotation.
    pub de: Option<&'a str>,
    /// French annotation.
    pub fr: Option<&'a str>,
}
impl<'a> LocaleAnnotationsView<'a> {
    fn values(self) -> [Option<&'a str>; 8] {
        [self.ko, self.en, self.ja, self.zh, self.vi, self.es, self.de, self.fr]
    }
}

/// Immutable coordinates for a released-owner artifact reference.
///
/// Structural validity does not establish that the authority, release or object
/// exists, is authentic, is authorized for the caller, or grants a capability.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ArtifactReferenceView<'a> {
    /// Canonical owner of the referenced artifact.
    pub authority_ref: &'a str,
    /// Versioned or immutable release coordinate asserted by the draft.
    pub release_ref: &'a str,
    /// Content digest asserted by the draft.
    pub artifact_digest: &'a str,
    /// Object coordinate within the asserted release.
    pub object_ref: &'a str,
}

/// Read-only semantic projection of a single procedure definition.
#[derive(Clone, Copy, Debug)]
pub struct ProcedureNodeView<'a> {
    /// Logical identity, independent of its label or position in an array.
    pub procedure_id: &'a str,
    /// Procedure category retained from the authoring contract.
    pub procedure_kind: ProcedureKind,
    /// Locale-specific labels retained without interpretation.
    pub locale_labels: LocaleAnnotationsView<'a>,
    /// Optional semantic artifact references; empty means the optional field was absent.
    pub semantic_refs: &'a [ArtifactReferenceView<'a>],
    /// Tool contract coordinate required for tool operations.
    pub tool_contract_ref: Option<ArtifactReferenceView<'a>>,
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

/// Read-only directed relation projection, including guidance and evidence.
#[derive(Clone, Copy, Debug)]
pub struct ProcedureRelationView<'a> {
    /// Source procedure; direction is preserved without interpreting prose.
    pub source_procedure_id: &'a str,
    /// Typed advisory relationship.
    pub relation_type: ProceduralRelationKind,
    /// Target procedure that must exist in the same model.
    pub target_procedure_id: &'a str,
    /// Conditions under which the relation is described as relevant.
    pub condition: LocaleAnnotationsView<'a>,
    /// Human/model guidance retained as authored evidence-bound text.
    pub guidance: LocaleAnnotationsView<'a>,
    /// Known pitfalls retained as authored evidence-bound text.
    pub pitfalls: LocaleAnnotationsView<'a>,
    /// Exact evidence coordinates for this relationship.
    pub source_evidence: &'a [EvidenceReference],
}

/// Structural and semantic input only; callers must not treat construction as admission.
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

/// Fixed error codes avoid returning untrusted identifiers, annotations or evidence text.
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
    /// Locale annotation is absent, blank, too long or contains NUL.
    InvalidAnnotation,
    /// Artifact-reference syntax is invalid; authenticity is not implied.
    InvalidArtifactReference,
    /// The same semantic artifact reference is repeated in one node.
    DuplicateArtifactReference,
    /// A tool operation omits the required tool-contract coordinate.
    MissingToolContract,
    /// The selected policy requires entry reachability that is not satisfied.
    UnreachableProcedure,
    /// Total examined projection bytes exceed the deterministic admission budget.
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
            Self::InvalidAnnotation => "invalid_annotation",
            Self::InvalidArtifactReference => "invalid_artifact_reference",
            Self::DuplicateArtifactReference => "duplicate_artifact_reference",
            Self::MissingToolContract => "missing_tool_contract",
            Self::UnreachableProcedure => "unreachable_procedure",
            Self::EvidenceBudgetExceeded => "evidence_budget_exceeded",
        };
        formatter.write_str(code)
    }
}
impl std::error::Error for ProceduralValidationError {}

type EvidenceKey<'a> = (&'a str, &'a str, &'a str);
type ArtifactKey<'a> = (&'a str, &'a str, &'a str, &'a str);

fn identity(value: &str) -> Result<(), ProceduralValidationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 128
        || !(bytes[0].is_ascii_alphanumeric() || bytes[0] == b'_')
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:/@+=#-".contains(byte))
    {
        return Err(ProceduralValidationError::InvalidIdentity);
    }
    Ok(())
}

fn sha256_digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn charge(bytes: usize, budget: &mut usize) -> Result<(), ProceduralValidationError> {
    *budget = budget.saturating_add(bytes);
    if *budget > 1_048_576 {
        return Err(ProceduralValidationError::EvidenceBudgetExceeded);
    }
    Ok(())
}

fn annotations(
    annotations: LocaleAnnotationsView<'_>,
    budget: &mut usize,
) -> Result<(), ProceduralValidationError> {
    let mut count = 0;
    for value in annotations.values().into_iter().flatten() {
        count += 1;
        if value.trim().is_empty() || value.contains('\0') || value.chars().count() > 2048 {
            return Err(ProceduralValidationError::InvalidAnnotation);
        }
        charge(value.len(), budget)?;
    }
    if count == 0 {
        return Err(ProceduralValidationError::InvalidAnnotation);
    }
    Ok(())
}

fn artifact_reference<'a>(
    reference: ArtifactReferenceView<'a>,
    budget: &mut usize,
) -> Result<ArtifactKey<'a>, ProceduralValidationError> {
    for value in [
        reference.authority_ref,
        reference.release_ref,
        reference.object_ref,
    ] {
        identity(value).map_err(|_| ProceduralValidationError::InvalidArtifactReference)?;
    }
    if !sha256_digest(reference.artifact_digest) {
        return Err(ProceduralValidationError::InvalidArtifactReference);
    }
    charge(
        reference.authority_ref.len()
            + reference.release_ref.len()
            + reference.artifact_digest.len()
            + reference.object_ref.len(),
        budget,
    )?;
    Ok((
        reference.authority_ref,
        reference.release_ref,
        reference.artifact_digest,
        reference.object_ref,
    ))
}

fn semantic_artifacts(
    references: &[ArtifactReferenceView<'_>],
    budget: &mut usize,
) -> Result<(), ProceduralValidationError> {
    if references.len() > 32 {
        return Err(ProceduralValidationError::CollectionLimit);
    }
    let mut unique = BTreeSet::new();
    for reference in references {
        let key = artifact_reference(*reference, budget)?;
        if !unique.insert(key) {
            return Err(ProceduralValidationError::DuplicateArtifactReference);
        }
    }
    Ok(())
}

fn evidence_keys<'a>(
    references: &'a [EvidenceReference],
    budget: &mut usize,
) -> Result<BTreeSet<EvidenceKey<'a>>, ProceduralValidationError> {
    if references.is_empty() || references.len() > 64 {
        return Err(ProceduralValidationError::CollectionLimit);
    }
    let mut keys = BTreeSet::new();
    let mut revisions = BTreeMap::new();
    for reference in references {
        let source = reference.source_id();
        let digest = reference.source_digest();
        let location = reference.location();
        identity(source).map_err(|_| ProceduralValidationError::InvalidEvidence)?;
        if !sha256_digest(digest)
            || location.len() > 8192
            || location.chars().count() > 2048
            || location.trim().is_empty()
            || location.contains('\0')
        {
            return Err(ProceduralValidationError::InvalidEvidence);
        }
        charge(source.len() + digest.len() + location.len(), budget)?;
        if let Some(previous) = revisions.insert(source, digest) {
            if previous != digest {
                return Err(ProceduralValidationError::ConflictingSourceRevision);
            }
        }
        if !keys.insert((source, digest, location)) {
            return Err(ProceduralValidationError::DuplicateEvidence);
        }
    }
    Ok(keys)
}

/// Checks scope, semantic-field invariants, topology, evidence closure and reachability.
///
/// `expected_scope` must come from the calling application's authenticated context,
/// not be copied from the candidate. Equality alone does not authenticate that context.
/// At most 256 nodes, 512 relations, 64 references per evidence set and 32 semantic
/// references per node are examined. Locale fields are limited to the eight contract
/// locales and 2,048 Unicode scalar values per annotation. A 1 MiB cumulative budget
/// covers identifiers, annotations, artifact coordinates and evidence in this borrowed
/// projection; serialized transport must apply its own byte/depth/duplicate-key bounds.
///
/// Returns counts only. Success is neither a semantic-release receipt nor a publishable
/// aggregate: source signatures, artifact authenticity/ACL, factual correctness,
/// natural-language interpretation, evaluation and stewardship remain separate gates.
pub fn validate_procedural_model(
    model: &ProceduralModelView<'_>,
    expected_scope: ProceduralScope<'_>,
    reachability: ReachabilityRule,
) -> Result<TopologySummary, ProceduralValidationError> {
    for scope in [model.scope, expected_scope] {
        for value in [
            scope.model_id,
            scope.tenant_ref,
            scope.task_type,
            scope.domain_owner_ref,
        ] {
            identity(value)?;
        }
    }
    if model.scope != expected_scope {
        return Err(ProceduralValidationError::ScopeMismatch);
    }
    if model.procedure_nodes.is_empty()
        || model.procedure_nodes.len() > 256
        || model.procedure_relations.len() > 512
    {
        return Err(ProceduralValidationError::CollectionLimit);
    }
    identity(model.entry_procedure_id)?;
    let mut budget = 0;
    for value in [
        model.scope.model_id,
        model.scope.tenant_ref,
        model.scope.task_type,
        model.scope.domain_owner_ref,
        model.entry_procedure_id,
    ] {
        charge(value.len(), &mut budget)?;
    }
    let inventory = evidence_keys(model.source_evidence, &mut budget)?;
    let mut nodes = BTreeSet::new();
    for node in model.procedure_nodes {
        identity(node.procedure_id)?;
        charge(node.procedure_id.len(), &mut budget)?;
        if !nodes.insert(node.procedure_id) {
            return Err(ProceduralValidationError::DuplicateProcedure);
        }
        annotations(node.locale_labels, &mut budget)?;
        semantic_artifacts(node.semantic_refs, &mut budget)?;
        if node.procedure_kind == ProcedureKind::ToolOperation && node.tool_contract_ref.is_none() {
            return Err(ProceduralValidationError::MissingToolContract);
        }
        if let Some(tool_contract) = node.tool_contract_ref {
            artifact_reference(tool_contract, &mut budget)?;
        }
        if !evidence_keys(node.source_evidence, &mut budget)?.is_subset(&inventory) {
            return Err(ProceduralValidationError::UnboundEvidence);
        }
    }
    if !nodes.contains(model.entry_procedure_id) {
        return Err(ProceduralValidationError::MissingEntry);
    }
    let mut triples = BTreeSet::new();
    let mut adjacency: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for edge in model.procedure_relations {
        identity(edge.source_procedure_id)?;
        identity(edge.target_procedure_id)?;
        charge(
            edge.source_procedure_id.len() + edge.target_procedure_id.len(),
            &mut budget,
        )?;
        if !nodes.contains(edge.source_procedure_id) || !nodes.contains(edge.target_procedure_id) {
            return Err(ProceduralValidationError::DanglingRelation);
        }
        if !triples.insert((
            edge.source_procedure_id,
            edge.relation_type,
            edge.target_procedure_id,
        )) {
            return Err(ProceduralValidationError::DuplicateRelation);
        }
        annotations(edge.condition, &mut budget)?;
        annotations(edge.guidance, &mut budget)?;
        annotations(edge.pitfalls, &mut budget)?;
        if !evidence_keys(edge.source_evidence, &mut budget)?.is_subset(&inventory) {
            return Err(ProceduralValidationError::UnboundEvidence);
        }
        adjacency
            .entry(edge.source_procedure_id)
            .or_default()
            .push(edge.target_procedure_id);
    }
    let mut visited = BTreeSet::from([model.entry_procedure_id]);
    let mut pending = vec![model.entry_procedure_id];
    while let Some(node) = pending.pop() {
        if let Some(neighbors) = adjacency.get(node) {
            for &neighbor in neighbors {
                if visited.insert(neighbor) {
                    pending.push(neighbor);
                }
            }
        }
    }
    if reachability == ReachabilityRule::RequireEntryReachability && visited.len() != nodes.len() {
        return Err(ProceduralValidationError::UnreachableProcedure);
    }
    Ok(TopologySummary {
        procedure_count: nodes.len(),
        relation_count: triples.len(),
        reachable_count: visited.len(),
    })
}
