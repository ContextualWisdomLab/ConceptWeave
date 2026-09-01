use conceptweave_client::{
    ReleaseContractError, ReleaseDigest, ReleaseMetadata, SemanticRelease, SemanticReleaseClient,
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
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
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

    let diff = client.diff(&previous, &current).unwrap();

    assert_eq!(diff.previous_release_id(), "semantic-release-grc-v1");
    assert_eq!(diff.current_release_id(), "semantic-release-grc-v2");
    assert_eq!(diff.added_concept_ids(), ["control.effectiveness"]);
    assert_eq!(diff.removed_concept_ids(), ["control.owner"]);
}

#[test]
fn release_diff_fails_closed_when_either_release_is_not_admissible() {
    let client = SemanticReleaseClient::new("1.0.0").unwrap();
    let previous = release(
        "semantic-release-grc-v1",
        &["control.evidence"],
        PublicationState::Published,
    );
    let reviewed = release(
        "semantic-release-grc-v2",
        &["control.effectiveness"],
        PublicationState::Reviewed,
    );

    assert_eq!(
        client.diff(&previous, &reviewed),
        Err(ReleaseContractError::ReleaseNotPublished {
            actual: PublicationState::Reviewed,
        })
    );
}
