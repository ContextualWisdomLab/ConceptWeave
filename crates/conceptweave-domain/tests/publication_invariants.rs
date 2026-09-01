use conceptweave_domain::{CandidateKind, EvidenceReference, PublicationState, SemanticCandidate};

fn reviewed_candidate() -> SemanticCandidate {
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
    candidate
}

#[test]
fn reviewed_candidate_exposes_evidence_read_only() {
    let candidate = reviewed_candidate();

    assert_eq!(candidate.publication_state(), PublicationState::Reviewed);
    assert_eq!(candidate.evidence().len(), 1);
    assert!(candidate.is_publishable());
}

#[test]
fn reviewed_candidate_can_publish_only_through_validated_transition() {
    let mut candidate = reviewed_candidate();

    candidate.transition(PublicationState::Published).unwrap();

    assert_eq!(candidate.publication_state(), PublicationState::Published);
    assert!(!candidate.is_publishable());
}
