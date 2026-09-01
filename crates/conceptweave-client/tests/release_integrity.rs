use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, SemanticReleaseClient,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};

const ARTIFACT_BYTES: &[u8] = b"conceptweave-semantic-release-v1";
const ARTIFACT_DIGEST: &str =
    "sha256:a141df3d94076487b7063ccb10d62a723f922b4440fa145fa16fd661d7259d1d";

fn release_with_digest(digest: &str) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(
            "semantic-release-grc-integrity-v1",
            "1.0.0",
            "grc-ontology-2026-09",
        )
        .unwrap(),
        TruthStatus::Authoritative,
        PublicationState::Published,
        ReleaseDigest::new(digest).unwrap(),
        vec![EvidenceReference::new(
            "snapshot:grc-schema-2026-09-01",
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "public.control_evidence.control_identifier",
        )
        .unwrap()],
        vec!["control.evidence".to_string()],
    )
    .unwrap()
}

#[test]
fn serialized_artifact_digest_verification_accepts_exact_bytes_offline() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release_with_digest(ARTIFACT_DIGEST);

    assert_eq!(client.verify_serialized_artifact(&release, ARTIFACT_BYTES), Ok(()));
}

#[test]
fn serialized_artifact_digest_verification_rejects_changed_bytes() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let release = release_with_digest(ARTIFACT_DIGEST);

    let result = client.verify_serialized_artifact(
        &release,
        b"conceptweave-semantic-release-v1-tampered",
    );

    assert!(matches!(
        result,
        Err(ReleaseContractError::ArtifactDigestMismatch { .. })
    ));
}
