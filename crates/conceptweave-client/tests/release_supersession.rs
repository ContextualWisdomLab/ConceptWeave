use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, ReleaseSupersession, SemanticRelease,
    SemanticReleaseClient, SemanticReleaseReference,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};

fn evidence() -> EvidenceReference {
    EvidenceReference::new(
        "snapshot:grc-schema-2026-09-01",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "governance_core.control_evidence.control_identifier",
    )
    .expect("evidence fixture is valid")
}

fn digest(hex: char) -> ReleaseDigest {
    ReleaseDigest::new(format!("sha256:{}", hex.to_string().repeat(64)))
        .expect("digest fixture is valid")
}

fn release(
    release_id: &str,
    digest_hex: char,
    publication_state: PublicationState,
) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(release_id, "2.0.0", format!("ontology_{release_id}"))
            .expect("metadata fixture is valid"),
        TruthStatus::Authoritative,
        publication_state,
        digest(digest_hex),
        vec![evidence()],
        vec!["control.evidence".to_owned()],
    )
    .expect("release fixture is valid")
}

#[test]
fn supersession_preserves_exact_immutable_release_references_and_rationale() {
    let previous = release("semantic_release_2026_09", 'b', PublicationState::Published);
    let successor = release("semantic_release_2026_10", 'c', PublicationState::Published);
    let declaration = ReleaseSupersession::new(
        SemanticReleaseReference::from_release(&previous),
        SemanticReleaseReference::from_release(&successor),
        "Correct the governed control taxonomy while preserving the prior release.",
    )
    .expect("supersession declaration is valid");

    assert_eq!(
        declaration.superseded().release_id(),
        "semantic_release_2026_09"
    );
    assert_eq!(
        declaration.superseded().artifact_digest(),
        previous.artifact_digest()
    );
    assert_eq!(
        declaration.successor().release_id(),
        "semantic_release_2026_10"
    );
    assert_eq!(
        declaration.successor().artifact_digest(),
        successor.artifact_digest()
    );
    assert_eq!(
        declaration.rationale(),
        "Correct the governed control taxonomy while preserving the prior release."
    );
}

#[test]
fn supersession_rejects_blank_reference_fields_blank_rationale_and_self_supersession() {
    assert_eq!(
        SemanticReleaseReference::new("  ", digest('b')),
        Err(ReleaseContractError::EmptyField("release_reference_id"))
    );
    assert_eq!(
        ReleaseSupersession::new(
            SemanticReleaseReference::new("semantic_release_2026_09", digest('b'))
                .expect("reference is valid"),
            SemanticReleaseReference::new("semantic_release_2026_10", digest('c'))
                .expect("reference is valid"),
            "\t",
        ),
        Err(ReleaseContractError::EmptyField("supersession_rationale"))
    );
    assert_eq!(
        ReleaseSupersession::new(
            SemanticReleaseReference::new("semantic_release_2026_09", digest('b'))
                .expect("reference is valid"),
            SemanticReleaseReference::new("semantic_release_2026_09", digest('c'))
                .expect("reference is valid"),
            "replacement",
        ),
        Err(ReleaseContractError::SelfSupersession(
            "semantic_release_2026_09".to_owned()
        ))
    );
}

#[test]
fn client_accepts_only_an_explicit_supersession_bound_to_both_exact_release_identities() {
    let client = SemanticReleaseClient::new("2.0.0").expect("client policy is valid");
    let previous = release("semantic_release_2026_09", 'b', PublicationState::Published);
    let successor = release("semantic_release_2026_10", 'c', PublicationState::Published);
    let declaration = ReleaseSupersession::new(
        SemanticReleaseReference::from_release(&previous),
        SemanticReleaseReference::from_release(&successor),
        "superseded by steward-approved correction",
    )
    .expect("supersession declaration is valid");

    assert_eq!(
        client.validate_supersession(&declaration, &previous, &successor),
        Ok(())
    );

    let wrong_previous_digest =
        release("semantic_release_2026_09", 'd', PublicationState::Published);
    assert_eq!(
        client.validate_supersession(&declaration, &wrong_previous_digest, &successor),
        Err(ReleaseContractError::SupersededReleaseReferenceMismatch)
    );

    let wrong_successor_digest =
        release("semantic_release_2026_10", 'e', PublicationState::Published);
    assert_eq!(
        client.validate_supersession(&declaration, &previous, &wrong_successor_digest),
        Err(ReleaseContractError::SuccessorReleaseReferenceMismatch)
    );
}

#[test]
fn supersession_never_bypasses_either_authoritative_release_admission_gate() {
    let client = SemanticReleaseClient::new("2.0.0").expect("client policy is valid");
    let reviewed_previous = release("semantic_release_2026_09", 'b', PublicationState::Reviewed);
    let published_previous = release("semantic_release_2026_09", 'b', PublicationState::Published);
    let published_successor = release("semantic_release_2026_10", 'c', PublicationState::Published);
    let reviewed_successor = release("semantic_release_2026_10", 'c', PublicationState::Reviewed);
    let declaration = ReleaseSupersession::new(
        SemanticReleaseReference::from_release(&published_previous),
        SemanticReleaseReference::from_release(&published_successor),
        "attempted supersession",
    )
    .expect("supersession declaration is structurally valid");

    assert_eq!(
        client.validate_supersession(&declaration, &reviewed_previous, &published_successor),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Reviewed,
        })
    );
    assert_eq!(
        client.validate_supersession(&declaration, &published_previous, &reviewed_successor),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Reviewed,
        })
    );
}
