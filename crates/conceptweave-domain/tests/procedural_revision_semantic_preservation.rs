//! RED/GREEN contract for lossless canonical revision-envelope semantics.

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
const DIGEST_C: &str = "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

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

fn canonical_revision_with_distinct_semantics() -> String {
    format!(
        r#"{{"schema_version":"0.1.0-draft.1","proposal_id":"review_revision_proposal","proposal_origin":"steward_authored","proposal_state":"proposed","decision_authority":"none","base_model_ref":{{"authority_ref":"ContextualWisdomLab/ConceptWeave","release_ref":"unit_fixture_base","artifact_digest":"{digest_b}","object_ref":"review_procedure_model"}},"candidate_model":{{"schema_version":"0.1.0-draft.1","model_id":"review_procedure_model","tenant_ref":"unit_fixture_tenant","task_type":"review_repair","domain_owner_ref":"ContextualWisdomLab/.github","publication_state":"draft","truth_status":"inferred","entry_procedure_id":"read_review","source_evidence":[{{"source_id":"unit_fixture_review_sop","source_digest":"{digest_a}","location":"section_2/step_1"}}],"procedure_nodes":[{{"procedure_id":"read_review","procedure_kind":"reasoning_step","locale_labels":{{"en":"Read review evidence"}},"source_evidence":[{{"source_id":"unit_fixture_review_sop","source_digest":"{digest_a}","location":"section_2/step_1"}}]}}],"procedure_edges":[]}},"evidence_partition":"training","training_evidence":[{{"source_id":"unit_fixture_training_trace","source_digest":"{digest_a}","location":"training/fold_7"}}],"rejected_edit_refs":[{{"authority_ref":"ContextualWisdomLab/ConceptWeave","release_ref":"unit_fixture_rejection_log","artifact_digest":"{digest_c}","object_ref":"rejected_edit_17"}}],"change_rationale":"Preserve this exact rationale for later independent evaluation and steward review."}}"#,
        digest_a = DIGEST_A,
        digest_b = DIGEST_B,
        digest_c = DIGEST_C,
    )
}

#[test]
fn canonical_revision_semantics_survive_one_shot_admission() {
    let admission = admit_procedural_revision_json_transport(
        canonical_revision_with_distinct_semantics().as_bytes(),
        expected_revision(),
        ReachabilityRule::RequireEntryReachability,
    )
    .expect("canonical revision should retain proposal semantics after validation");

    assert_eq!(admission.proposal_id(), "review_revision_proposal");
    assert_eq!(admission.proposal_origin(), ProceduralProposalOrigin::StewardAuthored);
    assert_eq!(admission.base_model_ref(), expected_base());
    assert_eq!(admission.candidate_scope(), expected_scope());
    assert_eq!(
        admission.training_evidence().collect::<Vec<_>>(),
        vec![ProceduralEvidenceView {
            source_id: "unit_fixture_training_trace",
            source_digest: DIGEST_A,
            location: "training/fold_7",
        }]
    );
    assert_eq!(
        admission.rejected_edit_refs().collect::<Vec<_>>(),
        vec![ArtifactReferenceView {
            authority_ref: "ContextualWisdomLab/ConceptWeave",
            release_ref: "unit_fixture_rejection_log",
            artifact_digest: DIGEST_C,
            object_ref: "rejected_edit_17",
        }]
    );
    assert_eq!(
        admission.change_rationale(),
        "Preserve this exact rationale for later independent evaluation and steward review."
    );
    assert_eq!(admission.topology_summary().procedure_count, 1);
}

#[test]
fn retained_semantics_do_not_weaken_external_context_binding() {
    let expected = ProceduralRevisionExpectation {
        proposal_id: "other_revision",
        ..expected_revision()
    };
    let error = admit_procedural_revision_json_transport(
        canonical_revision_with_distinct_semantics().as_bytes(),
        expected,
        ReachabilityRule::RequireEntryReachability,
    )
    .expect_err("mismatched external revision context must fail closed");
    assert_eq!(error, ProceduralIngressError::RevisionContextMismatch);
}
