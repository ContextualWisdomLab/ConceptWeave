#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! Private canonical Draft 2020-12 transport-to-domain admission seam.
//!
//! The mapper consumes only the repository-owned procedural draft shape after strict
//! byte/UTF-8/JSON admission. It does not authenticate callers or referenced artifacts,
//! publish a model, or authorize execution merely because deterministic checks succeed.

use crate::EvidenceReference;
use crate::procedural_model_transport::{
    ProceduralTransportError, StrictJsonValue, parse_procedural_json_transport_bytes,
};
use crate::procedural_model_validation::{
    ArtifactReferenceView, LocaleAnnotationsView, ProceduralModelView, ProceduralRelationKind,
    ProceduralScope, ProceduralValidationError, ProcedureKind, ProcedureNodeView,
    ProcedureRelationView, ReachabilityRule, TopologySummary, validate_procedural_model,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

const SCHEMA_VERSION: &str = "0.1.0-draft.1";
type JsonObject = BTreeMap<String, StrictJsonValue>;

/// Fixed ingress failures; diagnostics never echo untrusted draft content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProceduralIngressError {
    /// Raw bytes failed the bounded strict-JSON transport gate.
    Transport(ProceduralTransportError),
    /// JSON was syntactically valid but did not satisfy the canonical draft shape.
    SchemaInvalid,
    /// A schema-valid projection failed deterministic semantic/topology validation.
    Validation(ProceduralValidationError),
}

impl fmt::Display for ProceduralIngressError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transport(error) => write!(formatter, "transport:{error}"),
            Self::SchemaInvalid => formatter.write_str("schema_invalid"),
            Self::Validation(error) => write!(formatter, "validation:{error}"),
        }
    }
}

impl std::error::Error for ProceduralIngressError {}

impl From<ProceduralTransportError> for ProceduralIngressError {
    fn from(error: ProceduralTransportError) -> Self {
        Self::Transport(error)
    }
}

impl From<ProceduralValidationError> for ProceduralIngressError {
    fn from(error: ProceduralValidationError) -> Self {
        Self::Validation(error)
    }
}

#[derive(Clone, Debug)]
struct OwnedEvidenceReference {
    source_id: String,
    source_digest: String,
    location: String,
}

#[derive(Clone, Debug)]
struct OwnedArtifactReference {
    authority_ref: String,
    release_ref: String,
    artifact_digest: String,
    object_ref: String,
}

impl OwnedArtifactReference {
    fn as_view(&self) -> ArtifactReferenceView<'_> {
        ArtifactReferenceView {
            authority_ref: &self.authority_ref,
            release_ref: &self.release_ref,
            artifact_digest: &self.artifact_digest,
            object_ref: &self.object_ref,
        }
    }
}

#[derive(Clone, Debug)]
struct OwnedLocaleAnnotations {
    ko: Option<String>,
    en: Option<String>,
    ja: Option<String>,
    zh: Option<String>,
    vi: Option<String>,
    es: Option<String>,
    de: Option<String>,
    fr: Option<String>,
}

impl OwnedLocaleAnnotations {
    fn as_view(&self) -> LocaleAnnotationsView<'_> {
        LocaleAnnotationsView {
            ko: self.ko.as_deref(),
            en: self.en.as_deref(),
            ja: self.ja.as_deref(),
            zh: self.zh.as_deref(),
            vi: self.vi.as_deref(),
            es: self.es.as_deref(),
            de: self.de.as_deref(),
            fr: self.fr.as_deref(),
        }
    }
}

#[derive(Clone, Debug)]
struct OwnedProcedureNode {
    procedure_id: String,
    procedure_kind: ProcedureKind,
    locale_labels: OwnedLocaleAnnotations,
    semantic_refs: Option<Vec<OwnedArtifactReference>>,
    tool_contract_ref: Option<OwnedArtifactReference>,
    source_evidence: Vec<OwnedEvidenceReference>,
}

#[derive(Clone, Debug)]
struct OwnedProcedureRelation {
    source_procedure_id: String,
    relation_type: ProceduralRelationKind,
    target_procedure_id: String,
    condition: OwnedLocaleAnnotations,
    guidance: OwnedLocaleAnnotations,
    pitfalls: OwnedLocaleAnnotations,
    source_evidence: Vec<OwnedEvidenceReference>,
}

#[derive(Clone, Debug)]
struct OwnedProceduralDraft {
    model_id: String,
    tenant_ref: String,
    task_type: String,
    domain_owner_ref: String,
    entry_procedure_id: String,
    source_evidence: Vec<OwnedEvidenceReference>,
    procedure_nodes: Vec<OwnedProcedureNode>,
    procedure_relations: Vec<OwnedProcedureRelation>,
}

fn schema_invalid<T>() -> Result<T, ProceduralIngressError> {
    Err(ProceduralIngressError::SchemaInvalid)
}

fn into_object(value: StrictJsonValue) -> Result<JsonObject, ProceduralIngressError> {
    match value {
        StrictJsonValue::Object(object) => Ok(object),
        _ => schema_invalid(),
    }
}

fn into_array(value: StrictJsonValue) -> Result<Vec<StrictJsonValue>, ProceduralIngressError> {
    match value {
        StrictJsonValue::Array(values) => Ok(values),
        _ => schema_invalid(),
    }
}

fn into_string(value: StrictJsonValue) -> Result<String, ProceduralIngressError> {
    match value {
        StrictJsonValue::String(value) => Ok(value),
        _ => schema_invalid(),
    }
}

fn require_exact_keys(
    object: &JsonObject,
    required: &[&str],
    optional: &[&str],
) -> Result<(), ProceduralIngressError> {
    if required.iter().any(|key| !object.contains_key(*key)) {
        return schema_invalid();
    }
    if object
        .keys()
        .any(|key| !required.contains(&key.as_str()) && !optional.contains(&key.as_str()))
    {
        return schema_invalid();
    }
    Ok(())
}

fn take_required(
    object: &mut JsonObject,
    key: &str,
) -> Result<StrictJsonValue, ProceduralIngressError> {
    object.remove(key).ok_or(ProceduralIngressError::SchemaInvalid)
}

fn take_required_string(
    object: &mut JsonObject,
    key: &str,
) -> Result<String, ProceduralIngressError> {
    into_string(take_required(object, key)?)
}

fn take_optional_string(
    object: &mut JsonObject,
    key: &str,
) -> Result<Option<String>, ProceduralIngressError> {
    object.remove(key).map(into_string).transpose()
}

fn ensure_unique(values: &[StrictJsonValue]) -> Result<(), ProceduralIngressError> {
    let mut unique = BTreeSet::new();
    for value in values {
        if !unique.insert(value) {
            return schema_invalid();
        }
    }
    Ok(())
}

fn bounded_array(
    value: StrictJsonValue,
    minimum: usize,
    maximum: usize,
) -> Result<Vec<StrictJsonValue>, ProceduralIngressError> {
    let values = into_array(value)?;
    if values.len() < minimum || values.len() > maximum {
        return schema_invalid();
    }
    ensure_unique(&values)?;
    Ok(values)
}

fn valid_identity(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && (bytes[0].is_ascii_alphanumeric() || bytes[0] == b'_')
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:/@+=#-".contains(byte))
}

fn valid_sha256(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn is_ecmascript_whitespace(character: char) -> bool {
    matches!(
        character,
        '\u{0009}'
            | '\u{000a}'
            | '\u{000b}'
            | '\u{000c}'
            | '\u{000d}'
            | '\u{0020}'
            | '\u{00a0}'
            | '\u{1680}'
            | '\u{2000}'
            | '\u{2001}'
            | '\u{2002}'
            | '\u{2003}'
            | '\u{2004}'
            | '\u{2005}'
            | '\u{2006}'
            | '\u{2007}'
            | '\u{2008}'
            | '\u{2009}'
            | '\u{200a}'
            | '\u{2028}'
            | '\u{2029}'
            | '\u{202f}'
            | '\u{205f}'
            | '\u{3000}'
            | '\u{feff}'
    )
}

fn valid_annotation(value: &str) -> bool {
    let length = value.chars().count();
    length > 0
        && length <= 2048
        && !value.contains('\0')
        && value.chars().any(|character| !is_ecmascript_whitespace(character))
}

fn parse_evidence(value: StrictJsonValue) -> Result<OwnedEvidenceReference, ProceduralIngressError> {
    let mut object = into_object(value)?;
    require_exact_keys(&object, &["source_id", "source_digest", "location"], &[])?;
    let source_id = take_required_string(&mut object, "source_id")?;
    let source_digest = take_required_string(&mut object, "source_digest")?;
    let location = take_required_string(&mut object, "location")?;
    if !valid_identity(&source_id) || !valid_sha256(&source_digest) || !valid_annotation(&location) {
        return schema_invalid();
    }
    Ok(OwnedEvidenceReference {
        source_id,
        source_digest,
        location,
    })
}

fn parse_evidence_list(
    value: StrictJsonValue,
) -> Result<Vec<OwnedEvidenceReference>, ProceduralIngressError> {
    bounded_array(value, 1, 64)?
        .into_iter()
        .map(parse_evidence)
        .collect()
}

fn parse_artifact_reference(
    value: StrictJsonValue,
) -> Result<OwnedArtifactReference, ProceduralIngressError> {
    let mut object = into_object(value)?;
    require_exact_keys(
        &object,
        &["authority_ref", "release_ref", "artifact_digest", "object_ref"],
        &[],
    )?;
    let authority_ref = take_required_string(&mut object, "authority_ref")?;
    let release_ref = take_required_string(&mut object, "release_ref")?;
    let artifact_digest = take_required_string(&mut object, "artifact_digest")?;
    let object_ref = take_required_string(&mut object, "object_ref")?;
    if !valid_identity(&authority_ref)
        || !valid_identity(&release_ref)
        || !valid_sha256(&artifact_digest)
        || !valid_identity(&object_ref)
    {
        return schema_invalid();
    }
    Ok(OwnedArtifactReference {
        authority_ref,
        release_ref,
        artifact_digest,
        object_ref,
    })
}

fn parse_artifact_list(
    value: StrictJsonValue,
) -> Result<Vec<OwnedArtifactReference>, ProceduralIngressError> {
    bounded_array(value, 1, 32)?
        .into_iter()
        .map(parse_artifact_reference)
        .collect()
}

fn parse_locale_annotations(
    value: StrictJsonValue,
) -> Result<OwnedLocaleAnnotations, ProceduralIngressError> {
    let mut object = into_object(value)?;
    require_exact_keys(&object, &[], &["ko", "en", "ja", "zh", "vi", "es", "de", "fr"])?;
    if object.is_empty() {
        return schema_invalid();
    }
    let annotations = OwnedLocaleAnnotations {
        ko: take_optional_string(&mut object, "ko")?,
        en: take_optional_string(&mut object, "en")?,
        ja: take_optional_string(&mut object, "ja")?,
        zh: take_optional_string(&mut object, "zh")?,
        vi: take_optional_string(&mut object, "vi")?,
        es: take_optional_string(&mut object, "es")?,
        de: take_optional_string(&mut object, "de")?,
        fr: take_optional_string(&mut object, "fr")?,
    };
    for value in [
        annotations.ko.as_deref(),
        annotations.en.as_deref(),
        annotations.ja.as_deref(),
        annotations.zh.as_deref(),
        annotations.vi.as_deref(),
        annotations.es.as_deref(),
        annotations.de.as_deref(),
        annotations.fr.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        if !valid_annotation(value) {
            return schema_invalid();
        }
    }
    Ok(annotations)
}

fn parse_procedure_kind(value: String) -> Result<ProcedureKind, ProceduralIngressError> {
    match value.as_str() {
        "tool_operation" => Ok(ProcedureKind::ToolOperation),
        "reasoning_step" => Ok(ProcedureKind::ReasoningStep),
        "skill_procedure" => Ok(ProcedureKind::SkillProcedure),
        "task_state" => Ok(ProcedureKind::TaskState),
        _ => schema_invalid(),
    }
}

fn parse_relation_kind(value: String) -> Result<ProceduralRelationKind, ProceduralIngressError> {
    match value.as_str() {
        "leads_to" => Ok(ProceduralRelationKind::LeadsTo),
        "requires" => Ok(ProceduralRelationKind::Requires),
        "enables" => Ok(ProceduralRelationKind::Enables),
        _ => schema_invalid(),
    }
}

fn parse_node(value: StrictJsonValue) -> Result<OwnedProcedureNode, ProceduralIngressError> {
    let mut object = into_object(value)?;
    require_exact_keys(
        &object,
        &["procedure_id", "procedure_kind", "locale_labels", "source_evidence"],
        &["semantic_refs", "tool_contract_ref"],
    )?;
    let procedure_id = take_required_string(&mut object, "procedure_id")?;
    if !valid_identity(&procedure_id) {
        return schema_invalid();
    }
    let procedure_kind = parse_procedure_kind(take_required_string(&mut object, "procedure_kind")?)?;
    let locale_labels = parse_locale_annotations(take_required(&mut object, "locale_labels")?)?;
    let source_evidence = parse_evidence_list(take_required(&mut object, "source_evidence")?)?;
    let semantic_refs = object.remove("semantic_refs").map(parse_artifact_list).transpose()?;
    let tool_contract_ref = object
        .remove("tool_contract_ref")
        .map(parse_artifact_reference)
        .transpose()?;
    if procedure_kind == ProcedureKind::ToolOperation && tool_contract_ref.is_none() {
        return schema_invalid();
    }
    Ok(OwnedProcedureNode {
        procedure_id,
        procedure_kind,
        locale_labels,
        semantic_refs,
        tool_contract_ref,
        source_evidence,
    })
}

fn parse_relation(value: StrictJsonValue) -> Result<OwnedProcedureRelation, ProceduralIngressError> {
    let mut object = into_object(value)?;
    require_exact_keys(
        &object,
        &[
            "source_procedure_id",
            "relation_type",
            "target_procedure_id",
            "condition",
            "guidance",
            "pitfalls",
            "source_evidence",
        ],
        &[],
    )?;
    let source_procedure_id = take_required_string(&mut object, "source_procedure_id")?;
    let target_procedure_id = take_required_string(&mut object, "target_procedure_id")?;
    if !valid_identity(&source_procedure_id) || !valid_identity(&target_procedure_id) {
        return schema_invalid();
    }
    Ok(OwnedProcedureRelation {
        source_procedure_id,
        relation_type: parse_relation_kind(take_required_string(&mut object, "relation_type")?)?,
        target_procedure_id,
        condition: parse_locale_annotations(take_required(&mut object, "condition")?)?,
        guidance: parse_locale_annotations(take_required(&mut object, "guidance")?)?,
        pitfalls: parse_locale_annotations(take_required(&mut object, "pitfalls")?)?,
        source_evidence: parse_evidence_list(take_required(&mut object, "source_evidence")?)?,
    })
}

fn parse_draft(value: StrictJsonValue) -> Result<OwnedProceduralDraft, ProceduralIngressError> {
    let mut object = into_object(value)?;
    require_exact_keys(
        &object,
        &[
            "schema_version",
            "model_id",
            "tenant_ref",
            "task_type",
            "domain_owner_ref",
            "publication_state",
            "truth_status",
            "entry_procedure_id",
            "source_evidence",
            "procedure_nodes",
            "procedure_edges",
        ],
        &[],
    )?;

    if take_required_string(&mut object, "schema_version")? != SCHEMA_VERSION
        || take_required_string(&mut object, "publication_state")? != "draft"
        || take_required_string(&mut object, "truth_status")? != "inferred"
    {
        return schema_invalid();
    }

    let model_id = take_required_string(&mut object, "model_id")?;
    let tenant_ref = take_required_string(&mut object, "tenant_ref")?;
    let task_type = take_required_string(&mut object, "task_type")?;
    let domain_owner_ref = take_required_string(&mut object, "domain_owner_ref")?;
    let entry_procedure_id = take_required_string(&mut object, "entry_procedure_id")?;
    for value in [
        model_id.as_str(),
        tenant_ref.as_str(),
        task_type.as_str(),
        domain_owner_ref.as_str(),
        entry_procedure_id.as_str(),
    ] {
        if !valid_identity(value) {
            return schema_invalid();
        }
    }

    let source_evidence = parse_evidence_list(take_required(&mut object, "source_evidence")?)?;
    let procedure_nodes = bounded_array(take_required(&mut object, "procedure_nodes")?, 1, 256)?
        .into_iter()
        .map(parse_node)
        .collect::<Result<Vec<_>, _>>()?;
    let procedure_relations = bounded_array(take_required(&mut object, "procedure_edges")?, 0, 512)?
        .into_iter()
        .map(parse_relation)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(OwnedProceduralDraft {
        model_id,
        tenant_ref,
        task_type,
        domain_owner_ref,
        entry_procedure_id,
        source_evidence,
        procedure_nodes,
        procedure_relations,
    })
}

fn evidence_domain(
    references: &[OwnedEvidenceReference],
) -> Result<Vec<EvidenceReference>, ProceduralIngressError> {
    references
        .iter()
        .map(|reference| {
            EvidenceReference::new(
                reference.source_id.clone(),
                reference.source_digest.clone(),
                reference.location.clone(),
            )
            .map_err(|_| ProceduralIngressError::SchemaInvalid)
        })
        .collect()
}

impl OwnedProceduralDraft {
    fn validate(
        &self,
        expected_scope: ProceduralScope<'_>,
        reachability: ReachabilityRule,
    ) -> Result<TopologySummary, ProceduralIngressError> {
        let root_evidence = evidence_domain(&self.source_evidence)?;
        let node_evidence = self
            .procedure_nodes
            .iter()
            .map(|node| evidence_domain(&node.source_evidence))
            .collect::<Result<Vec<_>, _>>()?;
        let relation_evidence = self
            .procedure_relations
            .iter()
            .map(|relation| evidence_domain(&relation.source_evidence))
            .collect::<Result<Vec<_>, _>>()?;
        let semantic_reference_views = self
            .procedure_nodes
            .iter()
            .map(|node| {
                node.semantic_refs.as_ref().map(|references| {
                    references
                        .iter()
                        .map(OwnedArtifactReference::as_view)
                        .collect::<Vec<_>>()
                })
            })
            .collect::<Vec<_>>();

        let nodes = self
            .procedure_nodes
            .iter()
            .zip(node_evidence.iter())
            .zip(semantic_reference_views.iter())
            .map(|((node, evidence), semantic_refs)| ProcedureNodeView {
                procedure_id: &node.procedure_id,
                procedure_kind: node.procedure_kind,
                locale_labels: node.locale_labels.as_view(),
                semantic_refs: semantic_refs.as_deref(),
                tool_contract_ref: node
                    .tool_contract_ref
                    .as_ref()
                    .map(OwnedArtifactReference::as_view),
                source_evidence: evidence,
            })
            .collect::<Vec<_>>();

        let relations = self
            .procedure_relations
            .iter()
            .zip(relation_evidence.iter())
            .map(|(relation, evidence)| ProcedureRelationView {
                source_procedure_id: &relation.source_procedure_id,
                relation_type: relation.relation_type,
                target_procedure_id: &relation.target_procedure_id,
                condition: relation.condition.as_view(),
                guidance: relation.guidance.as_view(),
                pitfalls: relation.pitfalls.as_view(),
                source_evidence: evidence,
            })
            .collect::<Vec<_>>();

        let model = ProceduralModelView {
            scope: ProceduralScope {
                model_id: &self.model_id,
                tenant_ref: &self.tenant_ref,
                task_type: &self.task_type,
                domain_owner_ref: &self.domain_owner_ref,
            },
            entry_procedure_id: &self.entry_procedure_id,
            source_evidence: &root_evidence,
            procedure_nodes: &nodes,
            procedure_relations: &relations,
        };
        validate_procedural_model(&model, expected_scope, reachability).map_err(Into::into)
    }
}

/// Admits one canonical procedural-model draft from raw bytes through deterministic
/// transport, Draft 2020-12 shape mapping, and borrowed-domain validation.
///
/// Unknown, missing, wrong-typed and schema-invalid members fail with a fixed diagnostic
/// before a domain view is constructed. Canonical constants are consumed as invariants;
/// optional `semantic_refs` absence remains distinct from a present array, and the
/// conditional `tool_contract_ref` requirement is enforced for tool operations.
/// `expected_scope` is caller-supplied context only: equality does not authenticate it.
/// Released artifact authenticity, ACL, stewardship, publication and runtime authority
/// remain separate gates.
pub fn validate_procedural_model_json_transport(
    input: &[u8],
    expected_scope: ProceduralScope<'_>,
    reachability: ReachabilityRule,
) -> Result<TopologySummary, ProceduralIngressError> {
    let decoded = parse_procedural_json_transport_bytes(input)?;
    let draft = parse_draft(decoded)?;
    draft.validate(expected_scope, reachability)
}
