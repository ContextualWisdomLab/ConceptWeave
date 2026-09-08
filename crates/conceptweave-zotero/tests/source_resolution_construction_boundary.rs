const LIB_SOURCE: &str = include_str!("../src/lib.rs");

#[test]
fn source_resolution_review_does_not_expose_mutable_construction_fields() {
    let start = LIB_SOURCE
        .find("pub struct SourceResolutionReview {")
        .expect("SourceResolutionReview must remain part of the public API");
    let rest = &LIB_SOURCE[start..];
    let end = rest
        .find("\n}\n")
        .expect("SourceResolutionReview declaration must terminate");
    let declaration = &rest[..end];

    for field in [
        "zotero_version",
        "server_id",
        "library_version",
        "rule_revision",
        "pending_source_item_keys",
        "expected_source_identities",
        "resolved_sources",
    ] {
        assert!(
            !declaration
                .lines()
                .any(|line| line.trim_start().starts_with(&format!("pub {field}:"))),
            "SourceResolutionReview::{field} must be constructor-bound and read-only outside the crate"
        );
    }
}
