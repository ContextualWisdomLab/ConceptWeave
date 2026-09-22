use conceptweave_client::ReleaseContractError;
use conceptweave_domain::{PublicationState, TruthStatus};

#[test]
fn contract_errors_explain_the_failed_admission_invariant() {
    let cases = [
        (
            ReleaseContractError::EmptyField("release_id"),
            "required field `release_id` is blank".to_string(),
        ),
        (
            ReleaseContractError::InvalidDigest,
            "release digest must use sha256:<64 lowercase hex>".to_string(),
        ),
        (
            ReleaseContractError::ArtifactDigestMismatch {
                declared: "sha256:declared".to_string(),
                computed: "sha256:computed".to_string(),
            },
            "semantic release artifact digest mismatch: declared `sha256:declared`, computed `sha256:computed`"
                .to_string(),
        ),
        (
            ReleaseContractError::MissingProvenance,
            "semantic releases require provenance evidence".to_string(),
        ),
        (
            ReleaseContractError::DuplicateConceptId("concept.one".to_string()),
            "semantic release contains duplicate concept id `concept.one`".to_string(),
        ),
        (
            ReleaseContractError::CurrentContractVersionMarkedLegacy("2.0.0".to_string()),
            "current semantic release contract version `2.0.0` cannot also be marked legacy"
                .to_string(),
        ),
        (
            ReleaseContractError::SelfSupersession("semantic_release_2026_09".to_string()),
            "semantic release `semantic_release_2026_09` cannot supersede itself".to_string(),
        ),
        (
            ReleaseContractError::ConflictingReleaseIdentity(
                "semantic_release_2026_09".to_string(),
            ),
            "semantic release `semantic_release_2026_09` identifies conflicting immutable content"
                .to_string(),
        ),
        (
            ReleaseContractError::SupersededReleaseReferenceMismatch,
            "supersession predecessor reference does not match the exact supplied release"
                .to_string(),
        ),
        (
            ReleaseContractError::SuccessorReleaseReferenceMismatch,
            "supersession successor reference does not match the exact supplied release"
                .to_string(),
        ),
        (
            ReleaseContractError::UnsupportedContractVersion {
                expected: "1.0.0".to_string(),
                actual: "2.0.0".to_string(),
            },
            "semantic release contract version `2.0.0` is unsupported; current version is `1.0.0`"
                .to_string(),
        ),
        (
            ReleaseContractError::ReleaseNotPublished {
                actual: PublicationState::Reviewed,
            },
            "semantic release is Reviewed, not Published".to_string(),
        ),
        (
            ReleaseContractError::ReleaseNotAuthoritative {
                actual: TruthStatus::Proposed,
            },
            "semantic release truth status is Proposed, not Authoritative".to_string(),
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
