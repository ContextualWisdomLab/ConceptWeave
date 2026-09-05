use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, SemanticReleaseClient,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};

fn evidence() -> EvidenceReference {
    EvidenceReference::new(
        "snapshot:grc-schema-2026-09-01",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "public.control_item.control_identifier",
    )
    .unwrap()
}

fn release(publication_state: PublicationState) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(
            "semantic-release-grc-2026-09-01",
            "1.0.0",
            "grc-ontology-2026-09",
        )
        .unwrap(),
        TruthStatus::Authoritative,
        publication_state,
        ReleaseDigest::new(
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .unwrap(),
        vec![evidence()],
        vec![
            "control.internal_control".to_string(),
            "evidence.control_evidence".to_string(),
        ],
    )
    .unwrap()
}

#[test]
fn exact_concept_resolution_is_deterministic_and_does_not_fuzzy_match() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release(PublicationState::Published);

    assert_eq!(
        client.resolve_concept(&release, "control.internal_control"),
        Ok(Some("control.internal_control"))
    );
    assert_eq!(
        client.resolve_concept(&release, "internal control"),
        Ok(None)
    );
    assert_eq!(
        client.resolve_concept(&release, "CONTROL.INTERNAL_CONTROL"),
        Ok(None)
    );
}

#[test]
fn concept_resolution_reuses_authoritative_release_admission() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let reviewed = release(PublicationState::Reviewed);

    assert_eq!(
        client.resolve_concept(&reviewed, "control.internal_control"),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Reviewed,
        })
    );
}

#[test]
fn concept_resolution_rejects_blank_identifiers() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release(PublicationState::Published);

    assert_eq!(
        client.resolve_concept(&release, "  "),
        Err(ReleaseContractError::EmptyField("concept_id"))
    );
}
