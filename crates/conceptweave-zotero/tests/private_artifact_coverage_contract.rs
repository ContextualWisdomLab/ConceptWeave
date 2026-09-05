#[test]
fn private_artifact_helpers_are_not_excluded_from_owned_coverage() {
    let source = include_str!("../src/main.rs");
    for helper in ["fn write_all_and_flush", "fn allowed_output_parents"] {
        let position = source
            .find(helper)
            .unwrap_or_else(|| panic!("missing production helper: {helper}"));
        let prefix = &source[..position];
        let window_start = prefix.len().saturating_sub(240);
        let declaration_context = &prefix[window_start..];
        assert!(
            !declaration_context.contains("#[cfg_attr(coverage_nightly, coverage(off))]"),
            "{helper} must remain inside owned coverage rather than bypass the 100% production gate"
        );
    }
}
