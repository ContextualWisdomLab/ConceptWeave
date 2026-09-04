use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, SemanticReleaseClient,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};

const DETACHED_ARTIFACT_BYTES: &[u8] = b"conceptweave-semantic-model-artifact-v1";
const DETACHED_ARTIFACT_DIGEST: &str =
    "sha256:13b5f6c3d51da7bd481e8d267a135f0c7ef2a7a4e3987ceb6a1b610e215ccefd";

fn release_with_state(digest: &str, publication_state: PublicationState) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(
            "semantic-release-grc-integrity-v1",
            "1.0.0",
            "grc-ontology-2026-09",
        )
        .unwrap(),
        TruthStatus::Authoritative,
        publication_state,
        ReleaseDigest::new(digest).unwrap(),
        vec![
            EvidenceReference::new(
                "snapshot:grc-schema-2026-09-01",
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                "public.control_evidence.control_identifier",
            )
            .unwrap(),
        ],
        vec!["control.evidence".to_string()],
    )
    .unwrap()
}

fn published_release(digest: &str) -> SemanticRelease {
    release_with_state(digest, PublicationState::Published)
}

#[test]
fn detached_artifact_digest_verification_accepts_exact_bytes_offline() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = published_release(DETACHED_ARTIFACT_DIGEST);

    assert_eq!(
        client.verify_detached_artifact(&release, DETACHED_ARTIFACT_BYTES),
        Ok(())
    );
}

#[test]
fn detached_artifact_digest_verification_rejects_changed_bytes() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = published_release(DETACHED_ARTIFACT_DIGEST);

    let result = client.verify_detached_artifact(
        &release,
        b"conceptweave-semantic-model-artifact-v1-tampered",
    );

    assert!(matches!(
        result,
        Err(ReleaseContractError::ArtifactDigestMismatch { .. })
    ));
}

#[test]
fn detached_artifact_digest_verification_rejects_unpublished_release() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release_with_state(DETACHED_ARTIFACT_DIGEST, PublicationState::Proposed);

    assert_eq!(
        client.verify_detached_artifact(&release, DETACHED_ARTIFACT_BYTES),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Proposed,
        })
    );
}
