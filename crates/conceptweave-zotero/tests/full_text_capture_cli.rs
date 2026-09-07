use std::fs::{self, OpenOptions};
use std::io::Write;
use std::process::Command;

#[test]
#[cfg(unix)]
fn full_text_capture_rejects_an_unbound_report_before_creating_output() {
    use std::os::unix::fs::OpenOptionsExt;

    let report = conceptweave_zotero::classify_snapshot("9.0.6".into(), None, 0, vec![]);
    let input_path = std::env::temp_dir().join(format!(
        "conceptweave-full-text-unbound-{}.json",
        std::process::id()
    ));
    let output_path = input_path.with_extension("capture.json");
    let mut input_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&input_path)
        .unwrap();
    input_file
        .write_all(&serde_json::to_vec(&report).unwrap())
        .unwrap();
    drop(input_file);

    let command = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .arg("--capture-full-text")
        .arg(&input_path)
        .arg(&output_path)
        .output()
        .unwrap();
    fs::remove_file(input_path).unwrap();

    assert!(!command.status.success());
    assert!(!output_path.exists());
    assert!(command.stdout.is_empty());
    assert!(
        String::from_utf8(command.stderr)
            .unwrap()
            .contains("full-text capture requires a bound Zotero 10+ report")
    );
}
