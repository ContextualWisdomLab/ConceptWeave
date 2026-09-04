use conceptweave_client::{
    ReleaseDigest, ReleaseMetadata, ReleaseSupersession, SemanticRelease, SemanticReleaseClient,
    SemanticReleaseReference,
};
use conceptweave_domain::{EvidenceReference, PublicationState, TruthStatus};
use std::{fs, path::PathBuf};

fn digest(hex: char) -> ReleaseDigest {
    ReleaseDigest::new(&format!("sha256:{}", hex.to_string().repeat(64)))
        .expect("digest fixture must be canonical")
}

fn evidence() -> EvidenceReference {
    EvidenceReference::new(
        "snapshot:client-review-regression",
        "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "governance_core.control_evidence.control_identifier",
    )
    .expect("evidence fixture must be valid")
}

fn release(
    release_id: &str,
    digest_hex: char,
    truth_status: TruthStatus,
    publication_state: PublicationState,
    concept_ids: &[&str],
) -> SemanticRelease {
    SemanticRelease::new(
        ReleaseMetadata::new(release_id, "1.0.0", "ontology_client_review")
            .expect("metadata fixture must be valid"),
        truth_status,
        publication_state,
        digest(digest_hex),
        vec![evidence()],
        concept_ids
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
    )
    .expect("release fixture must be structurally valid")
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("client crate must live below the repository root")
        .to_path_buf()
}

#[test]
fn diff_fails_closed_when_one_release_id_names_conflicting_immutable_content() {
    let client = SemanticReleaseClient::new("1.0.0").expect("client policy must be valid");
    let previous = release(
        "semantic_release_same_id",
        'b',
        TruthStatus::Authoritative,
        PublicationState::Published,
        &["control.evidence"],
    );
    let conflicting = release(
        "semantic_release_same_id",
        'c',
        TruthStatus::Authoritative,
        PublicationState::Published,
        &["control.owner"],
    );

    assert!(
        client.diff(&previous, &conflicting).is_err(),
        "one stable release id must not be treated as ordinary evolution when its immutable content conflicts"
    );
}

#[test]
fn supersession_accepts_the_governed_superseded_predecessor_state() {
    let client = SemanticReleaseClient::new("1.0.0").expect("client policy must be valid");
    let previous = release(
        "semantic_release_previous",
        'b',
        TruthStatus::Superseded,
        PublicationState::Superseded,
        &["control.evidence"],
    );
    let successor = release(
        "semantic_release_successor",
        'c',
        TruthStatus::Authoritative,
        PublicationState::Published,
        &["control.evidence", "control.owner"],
    );
    let declaration = ReleaseSupersession::new(
        SemanticReleaseReference::from_release(&previous),
        SemanticReleaseReference::from_release(&successor),
        "steward-approved immutable correction",
    )
    .expect("supersession declaration must be valid");

    assert_eq!(
        client.validate_supersession(&declaration, &previous, &successor),
        Ok(()),
        "supersession validation must accept the predecessor after Governance marks it Superseded"
    );
}

#[test]
fn supersession_rejects_a_predecessor_with_only_one_superseded_state() {
    let client = SemanticReleaseClient::new("1.0.0").expect("client policy must be valid");
    let previous = release(
        "semantic_release_previous",
        'b',
        TruthStatus::Authoritative,
        PublicationState::Superseded,
        &["control.evidence"],
    );
    let successor = release(
        "semantic_release_successor",
        'c',
        TruthStatus::Authoritative,
        PublicationState::Published,
        &["control.evidence"],
    );
    let declaration = ReleaseSupersession::new(
        SemanticReleaseReference::from_release(&previous),
        SemanticReleaseReference::from_release(&successor),
        "steward-approved immutable correction",
    )
    .unwrap();

    assert!(
        client
            .validate_supersession(&declaration, &previous, &successor)
            .is_err()
    );
}

#[test]
fn supersession_rejects_an_incompatible_governed_predecessor() {
    let client = SemanticReleaseClient::new("1.0.0").expect("client policy must be valid");
    let previous = SemanticRelease::new(
        ReleaseMetadata::new(
            "semantic_release_previous",
            "2.0.0",
            "ontology_client_review",
        )
        .unwrap(),
        TruthStatus::Superseded,
        PublicationState::Superseded,
        digest('b'),
        vec![evidence()],
        vec!["control.evidence".to_owned()],
    )
    .unwrap();
    let successor = release(
        "semantic_release_successor",
        'c',
        TruthStatus::Authoritative,
        PublicationState::Published,
        &["control.evidence"],
    );
    let declaration = ReleaseSupersession::new(
        SemanticReleaseReference::from_release(&previous),
        SemanticReleaseReference::from_release(&successor),
        "steward-approved immutable correction",
    )
    .unwrap();

    assert!(
        client
            .validate_supersession(&declaration, &previous, &successor)
            .is_err()
    );
}

#[test]
fn diff_accepts_reusing_the_same_release_object() {
    let client = SemanticReleaseClient::new("1.0.0").expect("client policy must be valid");
    let release = release(
        "semantic_release_same_id",
        'b',
        TruthStatus::Authoritative,
        PublicationState::Published,
        &["control.evidence"],
    );
    assert!(client.diff(&release, &release).is_ok());
}

#[test]
fn public_contract_and_coverage_gates_encode_the_reviewed_fail_closed_rules() {
    let root = repository_root();
    let release_schema = fs::read_to_string(root.join("contracts/semantic-release.schema.json"))
        .expect("semantic-release schema must exist");
    let product_workflow = fs::read_to_string(root.join(".github/workflows/product.yml"))
        .expect("Product workflow must exist");
    let coverage_gate = fs::read_to_string(root.join("scripts/check_coverage.sh"))
        .expect("coverage gate must exist");

    assert!(
        release_schema.contains("\"contract_version\"")
            && release_schema.contains("\"const\": \"1.0.0\""),
        "the versioned 1.0.0 schema must reject unknown contract_version values"
    );
    assert!(
        product_workflow.contains("semantic-release-supersession.invalid-self.json")
            && product_workflow.contains("validate_semantic_release_supersession"),
        "the public contract gate must exercise a language-neutral self-supersession negative fixture through an explicit semantic validator"
    );
    assert!(
        !coverage_gate.contains(".data[0].totals.regions.percent == 100")
            && coverage_gate.contains("select(.name | contains(\"5tests\") | not)")
            && coverage_gate.contains("all(.[]; .count > 0)"),
        "coverage must aggregate owned production source coordinates instead of double-counting test-crate monomorphizations"
    );
}
