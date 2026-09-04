#![cfg(unix)]

#[test]
fn sensitive_output_path_is_rebuilt_from_the_validated_canonical_parent() {
    let source = include_str!("../src/main.rs");
    let start = source
        .find("fn validate_output_path(raw: &str) -> io::Result<PathBuf> {")
        .expect("validate_output_path must remain owned by the Zotero CLI boundary");
    let tail = &source[start..];
    let end = tail
        .find("\n}\n\n/// Creates a new sensitive report file")
        .expect("validate_output_path function boundary must remain inspectable");
    let function = &tail[..end];

    assert!(
        function.contains("let file_name = path.file_name()"),
        "validated output must retain only the final filename after canonicalizing its parent"
    );
    assert!(
        function.contains("let validated_path = resolved_parent.join(file_name);"),
        "output writes must use a path rebuilt from the validated canonical parent"
    );
    assert!(
        function.contains("fs::symlink_metadata(&validated_path)"),
        "existence/symlink checks must inspect the same canonical path later opened for creation"
    );
    assert!(
        function.contains("Ok(validated_path)"),
        "callers must receive the canonical rebuilt output path rather than the raw symlink-bearing path"
    );
    assert!(
        !function.contains("Ok(path)"),
        "returning the raw path reintroduces an upper-parent symlink TOCTOU between validation and create_new"
    );
}
