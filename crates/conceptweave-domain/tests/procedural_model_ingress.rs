//! RED/GREEN contract for canonical Draft 2020-12 transport-to-domain admission.

use conceptweave_domain::EvidenceReference;
#[path = "../src/procedural_model_transport.rs"]
mod procedural_model_transport;
#[path = "../src/procedural_model_validation.rs"]
mod procedural_model_validation;
#[path = "../src/procedural_model_ingress.rs"]
mod procedural_model_ingress;

use procedural_model_ingress::*;
use procedural_model_validation::{ProceduralScope, ReachabilityRule};

fn expected_scope() -> ProceduralScope<'static> {
    ProceduralScope {
        model_id: "model_one",
        tenant_ref: "tenant_one",
        task_type: "semantic_authoring",
        domain_owner_ref: "ConceptWeave",
    }
}

fn canonical_draft(node_extra: &str, root_extra: &str) -> String {
    format!(
        r#"{{"schema_version":"0.1.0-draft.1","model_id":"model_one","tenant_ref":"tenant_one","task_type":"semantic_authoring","domain_owner_ref":"ConceptWeave","publication_state":"draft","truth_status":"inferred","entry_procedure_id":"observe_source","source_evidence":[{{"source_id":"source_snapshot","source_digest":"sha256:{digest}","location":"section_2"}}],"procedure_nodes":[{{"procedure_id":"observe_source","procedure_kind":"skill_procedure","locale_labels":{{"en":"Observe source evidence"}}{node_extra},"source_evidence":[{{"source_id":"source_snapshot","source_digest":"sha256:{digest}","location":"section_2"}}]}}],"procedure_edges":[]{root_extra}}}"#,
        digest = "a".repeat(64),
    )
}

fn draft_with_edge(edge_extra: &str) -> String {
    let edge = format!(
        r#"{{"source_procedure_id":"observe_source","relation_type":"leads_to","target_procedure_id":"observe_source","condition":{{"en":"Evidence is available"}},"guidance":{{"en":"Preserve evidence"}},"pitfalls":{{"en":"Do not grant authority"}},"source_evidence":[{{"source_id":"source_snapshot","source_digest":"sha256:{digest}","location":"section_2"}}]{edge_extra}}}"#,
        digest = "a".repeat(64),
    );
    canonical_draft("", "").replace("\"procedure_edges\":[]", &format!("\"procedure_edges\":[{edge}]"))
}

fn assert_schema_invalid(input: &str) {
    assert_eq!(
        validate_procedural_model_json_transport(
            input.as_bytes(),
            expected_scope(),
            ReachabilityRule::RequireEntryReachability,
        ),
        Err(ProceduralIngressError::SchemaInvalid)
    );
}

fn assert_canonical_ok(input: &str) {
    let summary = validate_procedural_model_json_transport(
        input.as_bytes(),
        expected_scope(),
        ReachabilityRule::RequireEntryReachability,
    )
    .expect("canonical Draft 2020-12 document should map losslessly");
    assert_eq!(summary.procedure_count, 1);
    assert_eq!(summary.reachable_count, 1);
}

#[test]
fn canonical_draft_maps_before_domain_validation() {
    let input = canonical_draft("", "");
    assert_canonical_ok(&input);
}

#[test]
fn unknown_root_node_and_edge_members_fail_before_domain_construction() {
    assert_schema_invalid(&canonical_draft("", ",\"unexpected_authority\":true"));
    assert_schema_invalid(&canonical_draft(",\"unexpected_authority\":true", ""));
    assert_schema_invalid(&draft_with_edge(",\"unexpected_authority\":true"));
}

#[test]
fn root_constants_and_types_fail_closed() {
    for invalid in [
        canonical_draft("", "").replace("0.1.0-draft.1", "0.1.0-draft.2"),
        canonical_draft("", "").replace("\"publication_state\":\"draft\"", "\"publication_state\":\"published\""),
        canonical_draft("", "").replace("\"truth_status\":\"inferred\"", "\"truth_status\":\"authoritative\""),
        canonical_draft("", "").replace("\"task_type\":\"semantic_authoring\"", "\"task_type\":false"),
    ] {
        assert_schema_invalid(&invalid);
    }
}

#[test]
fn node_enum_and_locale_constraints_fail_closed() {
    for invalid in [
        canonical_draft("", "").replace("\"skill_procedure\"", "\"executable_tool\""),
        canonical_draft("", "").replace("{\"en\":\"Observe source evidence\"}", "{}"),
        canonical_draft("", "").replace("Observe source evidence", "   "),
    ] {
        assert_schema_invalid(&invalid);
    }
}

#[test]
fn semantic_refs_presence_and_tool_contract_condition_are_preserved() {
    assert_schema_invalid(&canonical_draft(",\"semantic_refs\":[]", ""));
    assert_schema_invalid(&canonical_draft("", "").replace("\"skill_procedure\"", "\"tool_operation\""));

    let artifact = format!(
        r#"{{"authority_ref":"ContextualWisdomLab/context-graph-contracts","release_ref":"v1.0.0","artifact_digest":"sha256:{}","object_ref":"tool_contract/example"}}"#,
        "b".repeat(64)
    );
    assert_canonical_ok(&canonical_draft(&format!(",\"semantic_refs\":[{artifact}]"), ""));
    let tool_with_contract = canonical_draft(&format!(",\"tool_contract_ref\":{artifact}"), "")
        .replace("\"skill_procedure\"", "\"tool_operation\"");
    assert_canonical_ok(&tool_with_contract);
}

#[test]
fn canonical_edge_maps_and_unknown_relation_fails() {
    let valid = draft_with_edge("");
    let summary = validate_procedural_model_json_transport(
        valid.as_bytes(),
        expected_scope(),
        ReachabilityRule::RequireEntryReachability,
    )
    .expect("canonical edge should map");
    assert_eq!(summary.relation_count, 1);

    assert_schema_invalid(&valid.replace("\"leads_to\"", "\"executes\""));
}

#[test]
fn schema_valid_scope_reaches_caller_context_comparison_only_after_mapping() {
    let input = canonical_draft("", "");
    let mismatched_scope = ProceduralScope {
        model_id: "model_one",
        tenant_ref: "another_tenant",
        task_type: "semantic_authoring",
        domain_owner_ref: "ConceptWeave",
    };
    assert_eq!(
        validate_procedural_model_json_transport(
            input.as_bytes(),
            mismatched_scope,
            ReachabilityRule::RequireEntryReachability,
        ),
        Err(ProceduralIngressError::Validation(
            procedural_model_validation::ProceduralValidationError::ScopeMismatch
        ))
    );
}
