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
            "release digest must use sha256:<64 hex>".to_string(),
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
            ReleaseContractError::UnsupportedContractVersion {
                expected: "1.0.0".to_string(),
                actual: "2.0.0".to_string(),
            },
            "semantic release contract version `2.0.0` is unsupported; expected `1.0.0`"
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
