#![cfg(unix)]

use conceptweave_zotero::{
    ClassificationReport, FullTextCapture, ZoteroItem, build_full_text_review_worksheet,
    classify_snapshot,
};
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
fn full_text_worksheet_cli_creates_blank_private_capture_bound_rows() {
    let files = FixtureFiles::new("blank");
    let original_report = fs::read(&files.report).unwrap();
    let original_capture = fs::read(&files.capture).unwrap();
    let command = run(&files.report, &files.capture, &files.output);
    assert!(command.status.success(), "{:?}", command.stderr);
    assert!(command.stdout.is_empty());
    let bytes = fs::read(&files.output).unwrap();
    let worksheet: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let capture: serde_json::Value = serde_json::from_slice(&original_capture).unwrap();
    assert_eq!(worksheet["capture_digest"], capture["capture_digest"]);
    let rows = worksheet["full_text_worksheet_v1"]["decisions"]
        .as_array()
        .unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row["reviewed_disposition"].is_null()));
    assert_eq!(
        fs::metadata(&files.output).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert!(
        !run(&files.report, &files.capture, &files.output)
            .status
            .success()
    );
    assert_eq!(fs::read(&files.output).unwrap(), bytes);
    assert_eq!(fs::read(&files.report).unwrap(), original_report);
    assert_eq!(fs::read(&files.capture).unwrap(), original_capture);
}

#[test]
fn full_text_worksheet_cli_rejects_wrong_capture_or_report_without_output() {
    let files = FixtureFiles::new("mismatch");
    let original_report = fs::read(&files.report).unwrap();
    let mut report: ClassificationReport = serde_json::from_slice(&original_report).unwrap();
    report.zotero_version = "10.0.2".into();
    fs::write(&files.report, serde_json::to_vec(&report).unwrap()).unwrap();
    let command = run(&files.report, &files.capture, &files.output);
    assert!(!command.status.success());
    assert!(
        String::from_utf8(command.stderr)
            .unwrap()
            .contains("full-text capture evidence is invalid")
    );
    assert!(!files.output.exists());
    fs::write(&files.report, original_report).unwrap();
    let mut capture: serde_json::Value =
        serde_json::from_slice(&fs::read(&files.capture).unwrap()).unwrap();
    capture["capture_digest"] = "synthetic-wrong-digest".into();
    fs::write(&files.capture, serde_json::to_vec(&capture).unwrap()).unwrap();
    let command = run(&files.report, &files.capture, &files.output);
    assert!(!command.status.success());
    assert!(
        String::from_utf8(command.stderr)
            .unwrap()
            .contains("full-text capture evidence is invalid")
    );
    assert!(command.stdout.is_empty());
    assert!(!files.output.exists());
}

#[test]
fn full_text_worksheet_cli_rejects_aliases_and_unsafe_inputs() {
    let files = FixtureFiles::new("alias");
    for (report, capture, output) in [
        (&files.report, &files.report, &files.output),
        (&files.report, &files.capture, &files.report),
        (&files.report, &files.capture, &files.capture),
    ] {
        assert!(!run(report, capture, output).status.success());
    }
    let alias = files
        .report
        .parent()
        .unwrap()
        .join(".")
        .join(files.report.file_name().unwrap());
    assert!(!run(&files.report, &alias, &files.output).status.success());
    let hard_link = files.output.with_extension("alias.json");
    fs::hard_link(&files.capture, &hard_link).unwrap();
    assert!(
        !run(&files.report, &hard_link, &files.output)
            .status
            .success()
    );
    fs::remove_file(hard_link).unwrap();
    fs::set_permissions(&files.capture, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(
        !run(&files.report, &files.capture, &files.output)
            .status
            .success()
    );
    assert!(!files.output.exists());
}

struct FixtureFiles {
    report: PathBuf,
    capture: PathBuf,
    output: PathBuf,
}

impl FixtureFiles {
    fn new(case: &str) -> Self {
        let prefix = format!(
            "conceptweave-fulltext-worksheet-{}-{case}",
            std::process::id()
        );
        let files = Self {
            report: std::env::temp_dir().join(format!("{prefix}-report.json")),
            capture: std::env::temp_dir().join(format!("{prefix}-capture.json")),
            output: std::env::temp_dir().join(format!("{prefix}-worksheet.json")),
        };
        let (report, capture) = synthetic_fixture();
        for (path, bytes) in [
            (&files.report, serde_json::to_vec(&report).unwrap()),
            (&files.capture, serde_json::to_vec(&capture).unwrap()),
        ] {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
                .unwrap()
                .write_all(&bytes)
                .unwrap();
        }
        files
    }
}

impl Drop for FixtureFiles {
    fn drop(&mut self) {
        for path in [&self.report, &self.capture, &self.output] {
            let _ = fs::remove_file(path);
        }
    }
}

fn run(report: &Path, capture: &Path, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .arg("--full-text-worksheet")
        .args([report, capture, output])
        .output()
        .unwrap()
}

fn synthetic_fixture() -> (ClassificationReport, FullTextCapture) {
    let items: Vec<ZoteroItem> = serde_json::from_value(serde_json::json!([
        {"key":"ABCD2345","version":2,"data":{"itemType":"journalArticle","title":"synthetic ontology engineering"}},
        {"key":"BCDE3456","version":2,"data":{"itemType":"book","title":"synthetic unmatched paper"}}
    ])).unwrap();
    let mut report = classify_snapshot("10.0.1".into(), Some("synthetic-server".into()), 2, items);
    report.api_version = Some(3);
    report.schema_version = Some(44);
    let library = r#"{"status":200,"version":2,"body":"[]"}"#;
    let manifest = r#"{"status":200,"version":null,"body":"{}"}"#;
    let evidence = format!(
        concat!(
            "{{\"capture_kind\":\"non_atomic_fulltext_sweep_v1\",",
            "\"metadata_report_digest\":\"sha256:{:x}\",\"metadata_snapshot_digest\":\"{}\",",
            "\"bibliographic_item_count\":2,\"started_unix_ms\":0,\"finished_unix_ms\":1,",
            "\"library_before\":{},\"manifest_before\":{},\"records\":[],",
            "\"manifest_after\":{},\"library_after\":{}}}"
        ),
        Sha256::digest(serde_json::to_vec(&report).unwrap()),
        report.snapshot_digest,
        library,
        manifest,
        manifest,
        library,
    );
    let capture: FullTextCapture = serde_json::from_str(&format!(
        "{{\"capture_digest\":\"sha256:{:x}\",\"capture_evidence\":{evidence}}}",
        Sha256::digest(evidence.as_bytes())
    ))
    .unwrap();
    build_full_text_review_worksheet(&report, &capture).unwrap();
    (report, capture)
}
