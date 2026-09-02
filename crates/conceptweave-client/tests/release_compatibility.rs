use conceptweave_client::{
    ContractVersionCompatibility, ReleaseContractError, ReleaseDigest, ReleaseMetadata,
    SemanticRelease, SemanticReleaseClient,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};

fn evidence() -> EvidenceReference {
    EvidenceReference::new(
        "snapshot:grc-schema-2026-09-01",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "public.control_evidence.control_identifier",
    )
    .expect("evidence fixture is valid")
}

fn release(contract_version: &str) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(
            format!("semantic_release_{contract_version}"),
            contract_version,
            "grc_ontology_2026_09",
        )
        .expect("metadata fixture is valid"),
        TruthStatus::Authoritative,
        PublicationState::Published,
        ReleaseDigest::new(
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
        )
        .expect("digest fixture is valid"),
        vec![evidence()],
        vec!["control.evidence".to_owned()],
    )
    .expect("release fixture is valid")
}

#[test]
fn client_explicitly_distinguishes_current_supported_legacy_and_unknown_versions() {
    let client = SemanticReleaseClient::with_supported_legacy_contract_versions(
        "2.0.0",
        vec!["1.1.0".to_owned(), "1.0.0".to_owned()],
    )
    .expect("explicit compatibility policy is valid");

    assert_eq!(
        client.compatibility(&release("2.0.0")),
        ContractVersionCompatibility::Current
    );
    assert_eq!(
        client.compatibility(&release("1.0.0")),
        ContractVersionCompatibility::SupportedLegacy
    );
    assert_eq!(
        client.compatibility(&release("3.0.0")),
        ContractVersionCompatibility::Unsupported
    );
}

#[test]
fn supported_legacy_release_passes_the_same_authoritative_use_gate() {
    let client = SemanticReleaseClient::with_supported_legacy_contract_versions(
        "2.0.0",
        vec!["1.0.0".to_owned()],
    )
    .expect("explicit compatibility policy is valid");

    assert_eq!(
        client.validate_for_authoritative_use(&release("1.0.0")),
        Ok(())
    );
}

#[test]
fn unknown_version_still_fails_closed_when_legacy_support_exists() {
    let client = SemanticReleaseClient::with_supported_legacy_contract_versions(
        "2.0.0",
        vec!["1.0.0".to_owned()],
    )
    .expect("explicit compatibility policy is valid");

    assert_eq!(
        client.validate_for_authoritative_use(&release("0.9.0")),
        Err(ReleaseContractError::UnsupportedContractVersion {
            expected: "2.0.0".to_owned(),
            actual: "0.9.0".to_owned(),
        })
    );
}

#[test]
fn compatibility_policy_rejects_blank_or_current_version_as_legacy() {
    assert_eq!(
        SemanticReleaseClient::with_supported_legacy_contract_versions(
            "2.0.0",
            vec!["  ".to_owned()]
        ),
        Err(ReleaseContractError::EmptyField(
            "supported_legacy_contract_version"
        ))
    );

    assert_eq!(
        SemanticReleaseClient::with_supported_legacy_contract_versions(
            "2.0.0",
            vec!["2.0.0".to_owned()]
        ),
        Err(ReleaseContractError::CurrentContractVersionMarkedLegacy(
            "2.0.0".to_owned()
        ))
    );
}
