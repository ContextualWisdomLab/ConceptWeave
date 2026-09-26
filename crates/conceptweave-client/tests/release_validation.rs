use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, SemanticReleaseClient,
    SignedReleaseManifest, TrustedPublisherKey, TrustedReleaseManifest,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};
use ring::{
    rand::SystemRandom,
    signature::{Ed25519KeyPair, KeyPair},
};

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

fn metadata(contract_version: &str) -> ReleaseMetadata {
    ReleaseMetadata::new(
        "semantic-release-grc-2026-09-01",
        contract_version,
        "grc-ontology-2026-09",
    )
    .unwrap()
}

fn release(
    contract_version: &str,
    truth_status: TruthStatus,
    publication_state: PublicationState,
) -> SemanticRelease {
    SemanticRelease::new(
        metadata(contract_version),
        truth_status,
        publication_state,
        digest(),
        vec![evidence()],
        vec!["control.evidence".to_string(), "control.owner".to_string()],
    )
    .unwrap()
}

#[test]
fn signed_manifest_requires_an_independent_exact_publisher_key() {
    let release = release(
        "1.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );
    let rng = SystemRandom::new();
    let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
    let key = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
    let trusted = TrustedPublisherKey::new("publisher-1", key.public_key().as_ref()).unwrap();
    let digest = release.manifest_digest();
    let message =
        SignedReleaseManifest::signing_message("publisher-1", release.release_id(), &digest)
            .unwrap();
    let signature = key.sign(&message);
    let signed = SignedReleaseManifest::new(
        "publisher-1",
        release.release_id(),
        digest.clone(),
        signature.as_ref(),
    )
    .unwrap();
    let client = SemanticReleaseClient::with_signed_release_manifests(
        "1.0.0",
        vec![],
        std::slice::from_ref(&trusted),
        std::slice::from_ref(&signed),
    )
    .unwrap();
    assert_eq!(client.validate_for_authoritative_use(&release), Ok(()));
    assert_eq!(
        SemanticReleaseClient::new("1.0.0")
            .unwrap()
            .validate_for_authoritative_use(&release),
        Err(ReleaseContractError::UntrustedRelease)
    );
    assert_eq!(
        SemanticReleaseClient::with_signed_release_manifests(
            "1.0.0",
            vec![],
            &[],
            std::slice::from_ref(&signed)
        )
        .unwrap_err(),
        ReleaseContractError::UnknownPublisherKey
    );
    assert_eq!(
        SemanticReleaseClient::with_signed_release_manifests(
            "1.0.0",
            vec![],
            &[trusted.clone(), trusted.clone()],
            std::slice::from_ref(&signed)
        )
        .unwrap_err(),
        ReleaseContractError::DuplicatePublisherKeyId("publisher-1".into())
    );
    let tampered =
        SignedReleaseManifest::new("publisher-1", "other-release", digest, signature.as_ref())
            .unwrap();
    assert_eq!(
        SemanticReleaseClient::with_signed_release_manifests(
            "1.0.0",
            vec![],
            &[trusted],
            &[tampered]
        )
        .unwrap_err(),
        ReleaseContractError::InvalidPublisherSignature
    );
}

#[test]
fn signed_manifest_rejects_invalid_external_identifiers_and_key_material() {
    let key = [1_u8; 32];
    let signature = [0_u8; 64];
    let digest = digest();
    for invalid in [" ", "publisher\0forged"] {
        assert_eq!(
            TrustedPublisherKey::new(invalid, &key),
            Err(ReleaseContractError::InvalidSignedManifest)
        );
        assert_eq!(
            SignedReleaseManifest::signing_message(invalid, "release", &digest),
            Err(ReleaseContractError::InvalidSignedManifest)
        );
        assert_eq!(
            SignedReleaseManifest::new(invalid, "release", digest.clone(), &signature),
            Err(ReleaseContractError::InvalidSignedManifest)
        );
        assert_eq!(
            SignedReleaseManifest::new("publisher", invalid, digest.clone(), &signature),
            Err(ReleaseContractError::InvalidSignedManifest)
        );
    }
    let oversized = "x".repeat(4_097);
    assert_eq!(
        TrustedPublisherKey::new(&oversized, &key),
        Err(ReleaseContractError::InvalidSignedManifest)
    );
    assert_eq!(
        SignedReleaseManifest::new("publisher", &oversized, digest.clone(), &signature),
        Err(ReleaseContractError::InvalidSignedManifest)
    );
    assert_eq!(
        TrustedPublisherKey::new("publisher", &key[..31]),
        Err(ReleaseContractError::InvalidSignedManifest)
    );
    assert_eq!(
        SignedReleaseManifest::new("publisher", "release", digest, &signature[..63]),
        Err(ReleaseContractError::InvalidSignedManifest)
    );
}

#[test]
fn authoritative_published_release_is_admitted_offline() {
    let release = release(
        "1.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );
    let client = SemanticReleaseClient::with_trusted_release_manifests(
        "1.0.0",
        vec![],
        vec![TrustedReleaseManifest::new(release.release_id(), release.manifest_digest()).unwrap()],
    )
    .unwrap();

    assert_eq!(client.supported_contract_version(), "1.0.0");
    assert_eq!(release.release_id(), "semantic-release-grc-2026-09-01");
    assert_eq!(release.contract_version(), "1.0.0");
    assert_eq!(release.ontology_version(), "grc-ontology-2026-09");
    assert_eq!(release.truth_status(), TruthStatus::Authoritative);
    assert_eq!(release.publication_state(), PublicationState::Published);
    assert_eq!(
        release.artifact_digest().as_str(),
        "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
    );
    assert_eq!(release.provenance().len(), 1);
    assert_eq!(release.concept_ids(), ["control.evidence", "control.owner"]);
    assert_eq!(
        release.manifest_digest().as_str(),
        "sha256:4abb03b6f9cf0f4d0d70b5deaf84f141cce4cba71c7178e577e1f883c4f6b974"
    );
    assert_eq!(client.validate_for_authoritative_use(&release), Ok(()));
}

#[test]
fn published_flags_without_a_trusted_manifest_are_not_authority() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release(
        "1.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );

    assert_eq!(
        client.validate_for_authoritative_use(&release),
        Err(ReleaseContractError::UntrustedRelease)
    );
}

#[test]
fn trusted_manifest_binds_provenance_and_concepts_independently_of_input_order() {
    let original = release(
        "1.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );
    let other_evidence = EvidenceReference::new(
        "snapshot:grc-schema-2026-09-01",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "public.control_evidence.owner_identifier",
    )
    .unwrap();
    let with_two_sources = SemanticRelease::new(
        metadata("1.0.0"),
        TruthStatus::Authoritative,
        PublicationState::Published,
        digest(),
        vec![evidence(), other_evidence.clone()],
        original.concept_ids().to_vec(),
    )
    .unwrap();
    let reordered = SemanticRelease::new(
        metadata("1.0.0"),
        TruthStatus::Authoritative,
        PublicationState::Published,
        digest(),
        vec![other_evidence, evidence()],
        original.concept_ids().iter().rev().cloned().collect(),
    )
    .unwrap();
    assert_eq!(
        with_two_sources.manifest_digest(),
        reordered.manifest_digest()
    );
    assert_ne!(
        original.manifest_digest(),
        with_two_sources.manifest_digest()
    );

    let client = SemanticReleaseClient::with_trusted_release_manifests(
        "1.0.0",
        vec![],
        vec![
            TrustedReleaseManifest::new(
                with_two_sources.release_id(),
                with_two_sources.manifest_digest(),
            )
            .unwrap(),
        ],
    )
    .unwrap();
    assert_eq!(client.validate_for_authoritative_use(&reordered), Ok(()));
    assert_eq!(
        client.validate_for_authoritative_use(&original),
        Err(ReleaseContractError::UntrustedRelease)
    );

    let changed_concepts = SemanticRelease::new(
        metadata("1.0.0"),
        TruthStatus::Authoritative,
        PublicationState::Published,
        digest(),
        with_two_sources.provenance().to_vec(),
        vec!["control.evidence".to_owned(), "control.changed".to_owned()],
    )
    .unwrap();
    assert_eq!(
        client.validate_for_authoritative_use(&changed_concepts),
        Err(ReleaseContractError::UntrustedRelease)
    );
}

#[test]
fn duplicate_trusted_release_identity_is_rejected() {
    let release = release(
        "1.0.0",
        TruthStatus::Authoritative,
        PublicationState::Published,
    );
    let pin = TrustedReleaseManifest::new(release.release_id(), release.manifest_digest()).unwrap();
    assert_eq!(
        SemanticReleaseClient::with_trusted_release_manifests(
            "1.0.0",
            vec![],
            vec![pin.clone(), pin]
        ),
        Err(ReleaseContractError::DuplicateTrustedReleaseId(
            release.release_id().to_owned()
        ))
    );
}

#[test]
fn client_fails_closed_on_unpublished_or_non_authoritative_release() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();

    let reviewed = release("1.0.0", TruthStatus::Inferred, PublicationState::Reviewed);
    assert_eq!(
        client.validate_for_authoritative_use(&reviewed),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Reviewed,
        })
    );

    let wrong_truth = release("1.0.0", TruthStatus::Proposed, PublicationState::Published);
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
fn metadata_requires_non_blank_release_contract_and_ontology_versions() {
    for (release_id, contract_version, ontology_version, expected_field) in [
        (" ", "1.0.0", "ontology-1", "release_id"),
        ("release-1", " ", "ontology-1", "contract_version"),
        ("release-1", "1.0.0", " ", "ontology_version"),
    ] {
        assert_eq!(
            ReleaseMetadata::new(release_id, contract_version, ontology_version),
            Err(ReleaseContractError::EmptyField(expected_field))
        );
    }

    let metadata = ReleaseMetadata::new("release-1", "1.0.0", "ontology-1").unwrap();
    assert_eq!(metadata.release_id(), "release-1");
    assert_eq!(metadata.contract_version(), "1.0.0");
    assert_eq!(metadata.ontology_version(), "ontology-1");
}

#[test]
fn release_requires_provenance_and_unique_non_blank_concepts() {
    assert_eq!(
        SemanticRelease::new(
            metadata("1.0.0"),
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
            metadata("1.0.0"),
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
            metadata("1.0.0"),
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
        ReleaseDigest::new(
            "sha256:gggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggggg"
        ),
        Err(ReleaseContractError::InvalidDigest)
    );
}

#[test]
fn client_requires_non_blank_supported_contract_version() {
    assert_eq!(
        SemanticReleaseClient::new(" "),
        Err(ReleaseContractError::EmptyField(
            "supported_contract_version"
        ))
    );
}
