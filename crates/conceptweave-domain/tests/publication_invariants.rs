use conceptweave_domain::{CandidateKind, EvidenceReference, PublicationState, SemanticCandidate};

#[test]
fn reviewed_candidate_without_evidence_is_not_publishable() {
    let evidence = EvidenceReference::new("source-1", "sha256:abc", "public.orders").unwrap();
    let mut candidate =
        SemanticCandidate::new("candidate-1", CandidateKind::Concept, vec![evidence]).unwrap();

    for state in [
        PublicationState::Proposed,
        PublicationState::Validated,
        PublicationState::Reviewed,
    ] {
        candidate.transition(state).unwrap();
    }

    candidate.evidence.clear();
    assert!(!candidate.is_publishable());
}
