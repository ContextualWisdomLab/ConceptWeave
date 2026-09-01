use conceptweave_domain::{
    CandidateKind, ContractError, EvidenceReference, PublicationState, SemanticCandidate,
};

fn validated_candidate() -> SemanticCandidate {
    let evidence = EvidenceReference::new("source-1", "sha256:abc", "public.orders").unwrap();
    let mut candidate =
        SemanticCandidate::new("candidate-1", CandidateKind::Concept, vec![evidence]).unwrap();

    candidate.transition(PublicationState::Proposed).unwrap();
    candidate.transition(PublicationState::Validated).unwrap();
    candidate
}

#[test]
fn evidence_remains_read_only_before_governance_review() {
    let candidate = validated_candidate();

    assert_eq!(candidate.publication_state(), PublicationState::Validated);
    assert_eq!(candidate.evidence().len(), 1);
    assert!(!candidate.is_publishable());
}

#[test]
fn external_callers_cannot_enter_reviewed_state_without_governance_authority() {
    let mut candidate = validated_candidate();

    assert_eq!(
        candidate.transition(PublicationState::Reviewed),
        Err(ContractError::GovernanceAuthorizationRequired {
            target: PublicationState::Reviewed,
        })
    );
    assert_eq!(candidate.publication_state(), PublicationState::Validated);
}

#[test]
fn external_callers_cannot_publish_by_skipping_governance() {
    let mut candidate = validated_candidate();

    assert_eq!(
        candidate.transition(PublicationState::Published),
        Err(ContractError::InvalidTransition {
            from: PublicationState::Validated,
            to: PublicationState::Published,
        })
    );
    assert_eq!(candidate.publication_state(), PublicationState::Validated);
}
