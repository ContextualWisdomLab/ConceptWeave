#[test]
fn golden_set_extends_the_canonical_research_intake_crate_root() {
    let manifest = include_str!("../Cargo.toml");
    let crate_root = include_str!("../src/lib.rs");

    assert!(
        !manifest.contains("path = \"src/merged_lib.rs\""),
        "golden-set reconciliation must not wrap the canonical Research Intake crate root"
    );
    assert!(
        crate_root.contains("mod golden_set;"),
        "the canonical crate root must own the golden-set module declaration"
    );
    assert!(
        crate_root.contains("pub use golden_set::*;"),
        "the canonical crate root must expose the golden-set contract"
    );
}
