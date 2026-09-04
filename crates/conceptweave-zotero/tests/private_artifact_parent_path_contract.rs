#[test]
fn private_artifact_open_does_not_reuse_the_untrusted_raw_path_after_parent_validation() {
    let source = include_str!("../src/main.rs");
    let start = source
        .find("fn read_private_json")
        .expect("missing private artifact reader");
    let end = source[start..]
        .find("fn open_with_metadata")
        .map(|offset| start + offset)
        .expect("missing private artifact open helper");
    let reader = &source[start..end];

    assert!(
        !reader.contains("fs::symlink_metadata(&path)"),
        "path metadata must be checked through a path rebuilt from the validated canonical parent"
    );
    assert!(
        !reader.contains("open_with_metadata(&path)"),
        "the opened path must be rebuilt from the validated canonical parent so a replaced parent symlink cannot redirect the read"
    );
}
