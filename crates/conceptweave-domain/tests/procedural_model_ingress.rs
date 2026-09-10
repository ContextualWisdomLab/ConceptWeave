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

fn canonical_draft(extra_root_member: &str) -> String {
    format!(
        r#"{{"schema_version":"0.1.0-draft.1","model_id":"model_one","tenant_ref":"tenant_one","task_type":"semantic_authoring","domain_owner_ref":"ConceptWeave","publication_state":"draft","truth_status":"inferred","entry_procedure_id":"observe_source","source_evidence":[{{"source_id":"source_snapshot","source_digest":"sha256:{digest}","location":"section_2"}}],"procedure_nodes":[{{"procedure_id":"observe_source","procedure_kind":"skill_procedure","locale_labels":{{"en":"Observe source evidence"}},"source_evidence":[{{"source_id":"source_snapshot","source_digest":"sha256:{digest}","location":"section_2"}}]}}],"procedure_edges":[]{extra_root_member}}}"#,
        digest = "a".repeat(64),
    )
}

#[test]
fn canonical_draft_maps_before_domain_validation() {
    let input = canonical_draft("");
    let summary = validate_procedural_model_json_transport(
        input.as_bytes(),
        expected_scope(),
        ReachabilityRule::RequireEntryReachability,
    )
    .expect("canonical Draft 2020-12 document should map losslessly");
    assert_eq!(summary.procedure_count, 1);
    assert_eq!(summary.relation_count, 0);
    assert_eq!(summary.reachable_count, 1);
}

#[test]
fn unknown_root_member_fails_before_domain_construction() {
    let input = canonical_draft(",\"unexpected_authority\":true");
    assert_eq!(
        validate_procedural_model_json_transport(
            input.as_bytes(),
            expected_scope(),
            ReachabilityRule::RequireEntryReachability,
        ),
        Err(ProceduralIngressError::SchemaInvalid)
    );
}
