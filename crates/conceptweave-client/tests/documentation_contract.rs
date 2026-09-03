//! Keep public Client Consumption documentation aligned with the Rust API.

const PRD: &str = include_str!("../../../docs/PRD.md");
const TRD: &str = include_str!("../../../docs/TRD.md");
const UML: &str = include_str!("../../../docs/UML.md");
const ADR: &str = include_str!("../../../docs/adr/0004-semantic-release-client-boundary.md");
const GAP_BASELINE: &str = include_str!("../../../docs/product-technical-gap-baseline.md");
const TEST_STRATEGY: &str = include_str!("../../../TEST_STRATEGY.md");
const SECURITY: &str = include_str!("../../../SECURITY.md");

#[test]
fn retired_serialized_artifact_api_is_absent_from_public_docs() {
    for (name, document) in [
        ("PRD", PRD),
        ("TRD", TRD),
        ("UML", UML),
        ("ADR 0004", ADR),
        ("gap baseline", GAP_BASELINE),
        ("test strategy", TEST_STRATEGY),
        ("security", SECURITY),
    ] {
        assert!(
            !document.contains("verify_serialized_artifact"),
            "{name} still names the retired serialized-artifact API"
        );
    }
}

#[test]
fn detached_artifact_integrity_is_documented_as_current_behavior() {
    for (name, document) in [
        ("PRD", PRD),
        ("TRD", TRD),
        ("UML", UML),
        ("ADR 0004", ADR),
        ("gap baseline", GAP_BASELINE),
        ("test strategy", TEST_STRATEGY),
        ("security", SECURITY),
    ] {
        assert!(
            document.contains("verify_detached_artifact"),
            "{name} does not document the current detached-artifact integrity boundary"
        );
    }
}
