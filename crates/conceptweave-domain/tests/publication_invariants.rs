use conceptweave_domain::{
    CandidateKind, ContractError, EvidenceReference, PublicationState, SemanticCandidate,
};

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
fn reviewed_candidate_without_evidence_is_not_publishable() {
    let mut candidate = reviewed_candidate();
    candidate.evidence.clear();

    assert!(!candidate.is_publishable());
}

#[test]
fn reviewed_candidate_without_evidence_cannot_transition_to_published() {
    let mut candidate = reviewed_candidate();
    candidate.evidence.clear();

    assert_eq!(
        candidate.transition(PublicationState::Published),
        Err(ContractError::MissingEvidence)
    );
    assert_eq!(candidate.publication_state, PublicationState::Reviewed);
}
