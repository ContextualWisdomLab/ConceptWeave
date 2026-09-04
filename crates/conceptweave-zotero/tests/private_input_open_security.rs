#[cfg(unix)]
#[test]
fn checked_path_can_be_swapped_to_a_symlink_that_preserves_inode_identity() {
    use std::fs;
    use std::os::unix::fs::{MetadataExt, PermissionsExt, symlink};
    use std::time::{SystemTime, UNIX_EPOCH};

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after the Unix epoch")
        .as_nanos();
    let original = std::env::temp_dir().join(format!(
        "conceptweave-zotero-{}-{nonce}-checked.json",
        std::process::id()
    ));
    let moved = std::env::temp_dir().join(format!(
        "conceptweave-zotero-{}-{nonce}-moved.json",
        std::process::id()
    ));

    let _ = fs::remove_file(&original);
    let _ = fs::remove_file(&moved);
    fs::write(&original, br#"{"review":"private"}"#).unwrap();
    fs::set_permissions(&original, fs::Permissions::from_mode(0o600)).unwrap();

    let checked = fs::symlink_metadata(&original).unwrap();
    assert!(!checked.file_type().is_symlink());
    fs::rename(&original, &moved).unwrap();
    symlink(&moved, &original).unwrap();

    let followed = fs::File::open(&original).unwrap().metadata().unwrap();
    assert_eq!(
        (checked.dev(), checked.ino()),
        (followed.dev(), followed.ino())
    );
    assert_eq!(followed.nlink(), 1);
    assert_eq!(followed.permissions().mode() & 0o777, 0o600);

    fs::remove_file(&original).unwrap();
    fs::remove_file(&moved).unwrap();
}

#[test]
fn owner_only_input_open_must_not_follow_final_component_symlinks() {
    let source = include_str!("../src/main.rs");
    let open_start = source
        .find("fn open_with_metadata")
        .expect("private artifact open helper must remain present");
    let open_tail = &source[open_start..];
    let open_end = open_tail.find("\n#[cfg(unix)]").unwrap_or(open_tail.len());
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
