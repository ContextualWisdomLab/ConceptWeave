use conceptweave_domain::{CandidateKind, EvidenceReference, SemanticCandidate};

#[test]
fn draft_candidate_is_not_publishable_before_governance_review() {
    let evidence = EvidenceReference::new("source-1", "sha256:abc", "schema.orders.total")
        .expect("valid evidence fixture");
    let candidate = SemanticCandidate::new("candidate-1", CandidateKind::Concept, vec![evidence])
        .expect("valid semantic candidate fixture");

    assert!(!candidate.is_publishable());
}
