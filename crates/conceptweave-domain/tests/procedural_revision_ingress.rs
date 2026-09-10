//! RED/GREEN contract for canonical procedural revision-envelope admission.

use conceptweave_domain::EvidenceReference;
#[path = "../src/procedural_model_transport.rs"]
mod procedural_model_transport;
#[path = "../src/procedural_model_validation.rs"]
mod procedural_model_validation;
#[path = "../src/procedural_model_ingress.rs"]
mod procedural_model_ingress;

use procedural_model_ingress::*;
use procedural_model_validation::{ArtifactReferenceView, ProceduralScope, ReachabilityRule};

const DIGEST_A: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const DIGEST_B: &str = "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

fn expected_scope() -> ProceduralScope<'static> {
    ProceduralScope {
        model_id: "review_procedure_model",
        tenant_ref: "unit_fixture_tenant",
        task_type: "review_repair",
        domain_owner_ref: "ContextualWisdomLab/.github",
    }
}

fn expected_base() -> ArtifactReferenceView<'static> {
    ArtifactReferenceView {
        authority_ref: "ContextualWisdomLab/ConceptWeave",
        release_ref: "unit_fixture_base",
        artifact_digest: DIGEST_B,
        object_ref: "review_procedure_model",
    }
}

fn expected_revision() -> ProceduralRevisionExpectation<'static> {
    ProceduralRevisionExpectation {
        proposal_id: "review_revision_proposal",
        base_model_ref: expected_base(),
        candidate_scope: expected_scope(),
    }
}

fn canonical_revision() -> String {
    format!(
        r#"{{"schema_version":"0.1.0-draft.1","proposal_id":"review_revision_proposal","proposal_origin":"model_assisted","proposal_state":"proposed","decision_authority":"none","base_model_ref":{{"authority_ref":"ContextualWisdomLab/ConceptWeave","release_ref":"unit_fixture_base","artifact_digest":"{digest_b}","object_ref":"review_procedure_model"}},"candidate_model":{{"schema_version":"0.1.0-draft.1","model_id":"review_procedure_model","tenant_ref":"unit_fixture_tenant","task_type":"review_repair","domain_owner_ref":"ContextualWisdomLab/.github","publication_state":"draft","truth_status":"inferred","entry_procedure_id":"read_review","source_evidence":[{{"source_id":"unit_fixture_review_sop","source_digest":"{digest_a}","location":"section_2/step_1"}}],"procedure_nodes":[{{"procedure_id":"read_review","procedure_kind":"reasoning_step","locale_labels":{{"en":"Read review evidence"}},"source_evidence":[{{"source_id":"unit_fixture_review_sop","source_digest":"{digest_a}","location":"section_2/step_1"}}]}}],"procedure_edges":[]}},"evidence_partition":"training","training_evidence":[{{"source_id":"unit_fixture_review_sop","source_digest":"{digest_a}","location":"section_2/step_1"}}],"rejected_edit_refs":[],"change_rationale":"Unit fixture only: bind an exact revision before validation."}}"#,
        digest_a = DIGEST_A,
        digest_b = DIGEST_B,
    )
}

fn validate(input: &str, expected: ProceduralRevisionExpectation<'_>) -> Result<procedural_model_validation::TopologySummary, ProceduralIngressError> {
    validate_procedural_revision_json_transport(
        input.as_bytes(),
        expected,
        ReachabilityRule::RequireEntryReachability,
    )
}

#[test]
fn canonical_revision_maps_and_binds_external_context() {
    let summary = validate(&canonical_revision(), expected_revision())
        .expect("canonical revision envelope should bind before semantic validation");
    assert_eq!(summary.procedure_count, 1);
    assert_eq!(summary.relation_count, 0);
    assert_eq!(summary.reachable_count, 1);
}

#[test]
fn proposal_id_mismatch_fails_closed() {
    let input = canonical_revision().replace(
        "\"proposal_id\":\"review_revision_proposal\"",
        "\"proposal_id\":\"other_revision\"",
    );
    assert_eq!(
        validate(&input, expected_revision()),
        Err(ProceduralIngressError::RevisionContextMismatch)
    );
}

#[test]
fn base_model_ref_mismatch_fails_closed() {
    let input = canonical_revision().replace(
        "\"release_ref\":\"unit_fixture_base\"",
        "\"release_ref\":\"stale_base\"",
    );
    assert_eq!(
        validate(&input, expected_revision()),
        Err(ProceduralIngressError::RevisionContextMismatch)
    );
}

#[test]
fn candidate_scope_mismatch_fails_before_domain_validation() {
    let input = canonical_revision().replace(
        "\"tenant_ref\":\"unit_fixture_tenant\"",
        "\"tenant_ref\":\"other_tenant\"",
    );
    assert_eq!(
        validate(&input, expected_revision()),
        Err(ProceduralIngressError::RevisionContextMismatch)
    );
}

#[test]
fn revision_authority_constants_and_origin_fail_closed() {
    for invalid in [
        canonical_revision().replace("\"decision_authority\":\"none\"", "\"decision_authority\":\"self\""),
        canonical_revision().replace("\"proposal_state\":\"proposed\"", "\"proposal_state\":\"approved\""),
        canonical_revision().replace("\"evidence_partition\":\"training\"", "\"evidence_partition\":\"validation\""),
        canonical_revision().replace("\"proposal_origin\":\"model_assisted\"", "\"proposal_origin\":\"runtime_generated\""),
    ] {
        assert_eq!(
            validate(&invalid, expected_revision()),
            Err(ProceduralIngressError::SchemaInvalid)
        );
    }
}

#[test]
fn revision_shape_bounds_fail_closed() {
    assert_eq!(
        validate(
            &canonical_revision().replace("\"training_evidence\":[{", "\"training_evidence\":[] , \"ignored\":[{") ,
            expected_revision(),
        ),
        Err(ProceduralIngressError::SchemaInvalid)
    );
    assert_eq!(
        validate(
            &canonical_revision().replace("\"change_rationale\":", "\"unexpected_authority\":true,\"change_rationale\":"),
            expected_revision(),
        ),
        Err(ProceduralIngressError::SchemaInvalid)
    );
}
