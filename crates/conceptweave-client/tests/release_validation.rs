use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, SemanticRelease, SemanticReleaseClient,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};

fn evidence() -> EvidenceReference {
    EvidenceReference::new(
        "snapshot:grc-schema-2026-09-01",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "public.control_evidence.control_identifier",
    )
    .unwrap()
}

fn digest() -> ReleaseDigest {
    ReleaseDigest::new("sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb")
        .unwrap()
}

fn release(
    contract_version: &str,
    truth_status: TruthStatus,
    publication_state: PublicationState,
) -> SemanticRelease {
    SemanticRelease::new(
        "semantic-release-grc-2026-09-01",
        contract_version,
        "grc-ontology-2026-09",
        truth_status,
        publication_state,
        digest(),
        vec![evidence()],
        vec!["control.evidence".to_string(), "control.owner".to_string()],
    )
    .unwrap()
}

#[test]
fn authoritative_published_release_is_admitted_offline() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release(
        "1.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );

    assert_eq!(client.validate_for_authoritative_use(&release), Ok(()));
}

#[test]
fn client_fails_closed_on_unpublished_or_non_authoritative_release() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();

    let reviewed = release(
        "1.0.0",
        TruthStatus::Inferred,
        PublicationState::Reviewed,
    );
    assert_eq!(
        client.validate_for_authoritative_use(&reviewed),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Reviewed,
        })
    );

    let wrong_truth = release(
        "1.0.0",
        TruthStatus::Proposed,
        PublicationState::Published,
    );
    assert_eq!(
        client.validate_for_authoritative_use(&wrong_truth),
        Err(ReleaseContractError::ReleaseNotAuthoritative {
            actual: TruthStatus::Proposed,
        })
    );
}

#[test]
fn client_rejects_unsupported_contract_version_before_use() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release(
        "2.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );

    assert_eq!(
        client.validate_for_authoritative_use(&release),
        Err(ReleaseContractError::UnsupportedContractVersion {
            expected: "1.0.0".to_string(),
            actual: "2.0.0".to_string(),
        })
    );
}

#[test]
fn release_requires_identity_provenance_and_unique_non_blank_concepts() {
    assert_eq!(
        SemanticRelease::new(
            " ",
            "1.0.0",
            "ontology-1",
            TruthStatus::Authoritative,
            PublicationState::Published,
            digest(),
            vec![evidence()],
            vec!["concept.one".to_string()],
        ),
        Err(ReleaseContractError::EmptyField("release_id"))
    );

    assert_eq!(
        SemanticRelease::new(
            "release-1",
            "1.0.0",
            "ontology-1",
            TruthStatus::Authoritative,
            PublicationState::Published,
            digest(),
            vec![],
            vec!["concept.one".to_string()],
        ),
        Err(ReleaseContractError::MissingProvenance)
    );

    assert_eq!(
        SemanticRelease::new(
            "release-1",
            "1.0.0",
            "ontology-1",
            TruthStatus::Authoritative,
            PublicationState::Published,
            digest(),
            vec![evidence()],
            vec![" ".to_string()],
        ),
        Err(ReleaseContractError::EmptyField("concept_id"))
    );

    assert_eq!(
        SemanticRelease::new(
            "release-1",
            "1.0.0",
            "ontology-1",
            TruthStatus::Authoritative,
            PublicationState::Published,
            digest(),
            vec![evidence()],
            vec!["concept.one".to_string(), "concept.one".to_string()],
        ),
        Err(ReleaseContractError::DuplicateConceptId(
            "concept.one".to_string()
        ))
    );
}

#[test]
fn digest_contract_rejects_non_sha256_and_malformed_hex() {
    assert_eq!(
        ReleaseDigest::new("md5:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        Err(ReleaseContractError::InvalidDigest)
    );
    assert_eq!(
        ReleaseDigest::new("sha256:abc"),
        Err(ReleaseContractError::InvalidDigest)
    );
    assert_eq!(
        ReleaseDigest::new("sha256:gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"),
        Err(ReleaseContractError::InvalidDigest)
    );
}

#[test]
fn client_requires_non_blank_supported_contract_version() {
    assert_eq!(
        SemanticReleaseClient::new(" "),
        Err(ReleaseContractError::EmptyField("supported_contract_version"))
    );
}
