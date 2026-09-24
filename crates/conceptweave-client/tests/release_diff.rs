use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, SemanticReleaseClient,
    TrustedReleaseManifest,
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

fn release(release_id: &str, concept_ids: &[&str], state: PublicationState) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(release_id, "1.0.0", "grc-ontology-2026-09").unwrap(),
        if state == PublicationState::Published {
            TruthStatus::Authoritative
        } else {
            TruthStatus::Inferred
        },
        state,
        ReleaseDigest::new(
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .unwrap(),
        vec![evidence()],
        concept_ids
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    )
    .unwrap()
}

#[test]
fn release_diff_reports_deterministic_added_and_removed_concepts() {
    let previous = release(
        "semantic-release-grc-v1",
        &["control.evidence", "control.owner"],
        PublicationState::Published,
    );
    let current = release(
        "semantic-release-grc-v2",
        &["control.effectiveness", "control.evidence"],
        PublicationState::Published,
    );
    let client = SemanticReleaseClient::with_trusted_release_manifests(
        "1.0.0",
        vec![],
        [&previous, &current]
            .into_iter()
            .map(|release| {
                TrustedReleaseManifest::new(release.release_id(), release.manifest_digest())
                    .unwrap()
            })
            .collect(),
    )
    .unwrap();

    let diff = client.diff(&previous, &current).unwrap();

    assert_eq!(diff.previous_release_id(), "semantic-release-grc-v1");
    assert_eq!(diff.current_release_id(), "semantic-release-grc-v2");
    assert_eq!(diff.added_concept_ids(), ["control.effectiveness"]);
    assert_eq!(diff.removed_concept_ids(), ["control.owner"]);
}

#[test]
fn release_diff_fails_closed_when_either_release_is_not_admissible() {
    let published = release(
        "semantic-release-grc-published",
        &["control.evidence"],
        PublicationState::Published,
    );
    let reviewed_previous = release(
        "semantic-release-grc-reviewed-previous",
        &["control.owner"],
        PublicationState::Reviewed,
    );
    let reviewed_current = release(
        "semantic-release-grc-reviewed-current",
        &["control.effectiveness"],
        PublicationState::Reviewed,
    );
    let client = SemanticReleaseClient::with_trusted_release_manifests(
        "1.0.0",
        vec![],
        vec![
            TrustedReleaseManifest::new(published.release_id(), published.manifest_digest())
                .unwrap(),
        ],
    )
    .unwrap();
    let expected = Err(ReleaseContractError::ReleaseNotPublished {
        actual: PublicationState::Reviewed,
    });

    assert_eq!(client.diff(&reviewed_previous, &published), expected);
    assert_eq!(client.diff(&published, &reviewed_current), expected);
}
