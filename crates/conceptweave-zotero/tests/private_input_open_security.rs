#[test]
fn owner_only_input_open_must_not_follow_final_component_symlinks() {
    let source = include_str!("../src/main.rs");
    let open_start = source
        .find("fn open_with_metadata")
        .expect("private artifact open helper must remain present");
    let open_tail = &source[open_start..];
    let open_end = open_tail
        .find("\n#[cfg(unix)]")
        .unwrap_or(open_tail.len());
    let open_body = &open_tail[..open_end];

    assert!(
        !open_body.contains("File::open(path)"),
        "checked-path metadata followed by File::open is vulnerable to a final-component symlink swap"
    );
    assert!(
        source.contains("O_NOFOLLOW")
            || source.contains("no_follow")
            || source.contains("nofollow")
            || source.contains("follow_links(false)"),
        "the opened private-artifact handle must be obtained with an explicit no-follow primitive"
    );
}
