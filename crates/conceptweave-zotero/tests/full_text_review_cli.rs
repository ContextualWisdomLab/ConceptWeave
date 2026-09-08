#![cfg(unix)]

use conceptweave_zotero::{
    ClassificationReport, FullTextCapture, StewardReviewWorksheet, ZoteroItem,
    build_steward_review_worksheet, classify_snapshot, verify_full_text_capture,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
fn full_text_review_cli_preserves_private_evidence_without_becoming_a_decision() {
    let inputs = FixtureFiles::new("positive");
    let output_path = inputs.path("view");
    let command = inputs.run("2", &output_path);
    assert!(command.status.success(), "{:?}", command.stderr);
    assert!(command.stdout.is_empty());
    let output: serde_json::Value =
        serde_json::from_slice(&fs::read(&output_path).unwrap()).unwrap();
    assert_eq!(output["view_kind"], "full_text_review_view_v1");
    assert_eq!(output["bibliographic_item_count"], 2);
    assert_eq!(
        output["review_batch"]["decisions"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(
        output["attachment_evidence"]["CDEF4567"],
        serde_json::json!([])
    );
    assert_eq!(
        output["attachment_evidence"]["ABCD2345"][0]["content_response"]["body"],
        r#"{"content":"synthetic private text","providerExtra":true}"#
    );
    assert!(output.get("decisions").is_none());
    assert_eq!(
        fs::metadata(&output_path).unwrap().permissions().mode() & 0o777,
        0o600
    );

    let original = fs::read(&inputs.worksheet_path).unwrap();
    for apply_mode in ["--apply-review-batch", "--apply-decision-patch"] {
        let rejected_path = inputs.path(apply_mode);
        let command = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
            .arg(apply_mode)
            .args([
                &inputs.report_path,
                &inputs.worksheet_path,
                &output_path,
                &rejected_path,
            ])
            .output()
            .unwrap();
        assert!(!command.status.success());
        assert!(!rejected_path.exists());
        assert_eq!(fs::read(&inputs.worksheet_path).unwrap(), original);
    }
    let again = inputs.run("2", &output_path);
    assert!(!again.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&fs::read(&output_path).unwrap()).unwrap(),
        output
    );
    fs::remove_file(output_path).unwrap();
}

#[test]
fn full_text_review_cli_rejects_invalid_or_oversized_capture_without_echoing_content() {
    let inputs = FixtureFiles::new("invalid");
    let output_path = inputs.path("view");
    let original = fs::read(&inputs.worksheet_path).unwrap();
    for content in [
        b"{\"synthetic-private-sentinel\":true}".as_slice(),
        b"{\"capture_digest\":\"synthetic-private-sentinel\",",
        b"\xff",
    ] {
        fs::write(&inputs.capture_path, content).unwrap();
        let command = inputs.run("1", &output_path);
        assert!(!command.status.success());
        assert!(!output_path.exists());
        assert!(command.stdout.is_empty());
        let stderr = String::from_utf8(command.stderr).unwrap();
        assert!(
            stderr.contains("full-text capture input is invalid"),
            "{stderr}"
        );
        assert!(!stderr.contains("synthetic-private-sentinel"));
    }
    let file = OpenOptions::new()
        .write(true)
        .open(&inputs.capture_path)
        .unwrap();
    file.set_len(512 * 1024 * 1024 + 1).unwrap();
    drop(file);
    let command = inputs.run("1", &output_path);
    assert!(!command.status.success());
    assert!(!output_path.exists());
    assert!(
        String::from_utf8(command.stderr)
            .unwrap()
            .contains("full-text capture input exceeds the file size limit")
    );
    assert_eq!(fs::read(&inputs.worksheet_path).unwrap(), original);
}

#[test]
fn full_text_review_cli_rejects_missing_alias_and_unsafe_files() {
    let inputs = FixtureFiles::new("unsafe");
    let output_path = inputs.path("view");
    let saved = inputs.path("saved");
    fs::rename(&inputs.capture_path, &saved).unwrap();
    assert!(!inputs.run("1", &output_path).status.success());
    symlink(&saved, &inputs.capture_path).unwrap();
    assert!(!inputs.run("1", &output_path).status.success());
    fs::remove_file(&inputs.capture_path).unwrap();
    fs::hard_link(&saved, &inputs.capture_path).unwrap();
    assert!(!inputs.run("1", &output_path).status.success());
    fs::remove_file(&inputs.capture_path).unwrap();
    fs::rename(&saved, &inputs.capture_path).unwrap();
    fs::set_permissions(&inputs.capture_path, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(!inputs.run("1", &output_path).status.success());
    fs::set_permissions(&inputs.capture_path, fs::Permissions::from_mode(0o600)).unwrap();
    for capture_path in [&inputs.report_path, &inputs.worksheet_path, &output_path] {
        let command = run(
            &inputs.report_path,
            &inputs.worksheet_path,
            capture_path,
            "1",
            &output_path,
        );
        assert!(!command.status.success());
    }
    // Different path spellings that resolve to one inode must still fail closed.
    let alias = inputs
        .report_path
        .parent()
        .unwrap()
        .join(".")
        .join(inputs.report_path.file_name().unwrap());
    assert!(
        !run(
            &inputs.report_path,
            &inputs.worksheet_path,
            &alias,
            "1",
            &output_path
        )
        .status
        .success()
    );
    assert!(
        !run(
            &inputs.report_path,
            &inputs.worksheet_path,
            Path::new("relative.json"),
            "1",
            &output_path
        )
        .status
        .success()
    );
    assert!(!output_path.exists());
}

#[test]
fn full_text_review_cli_capture_file_budget_is_separate_from_metadata_budget() {
    let inputs = FixtureFiles::new("budgets");
    let output_path = inputs.path("view");
    // Legal JSON whitespace makes the file exceed 16 MiB without a large text value.
    let mut file = OpenOptions::new()
        .append(true)
        .open(&inputs.capture_path)
        .unwrap();
    let padding = [b' '; 8192];
    for _ in 0..2048 {
        file.write_all(&padding).unwrap();
    }
    drop(file);
    assert!(inputs.run("1", &output_path).status.success());
    fs::remove_file(&output_path).unwrap();
    for metadata_path in [&inputs.report_path, &inputs.worksheet_path] {
        let original = fs::read(metadata_path).unwrap();
        let file = OpenOptions::new().write(true).open(metadata_path).unwrap();
        file.set_len(16 * 1024 * 1024 + 1).unwrap();
        drop(file);
        assert!(!inputs.run("1", &output_path).status.success());
        assert!(!output_path.exists());
        fs::write(metadata_path, original).unwrap();
    }
}

struct FixtureFiles {
    name: &'static str,
    report_path: PathBuf,
    worksheet_path: PathBuf,
    capture_path: PathBuf,
}

impl FixtureFiles {
    fn new(name: &'static str) -> Self {
        let (report, worksheet, capture) = synthetic_fixture();
        let files = Self {
            name,
            report_path: temp_path(name, "report"),
            worksheet_path: temp_path(name, "worksheet"),
            capture_path: temp_path(name, "capture"),
        };
        for (path, bytes) in [
            (&files.report_path, serde_json::to_vec(&report).unwrap()),
            (
                &files.worksheet_path,
                serde_json::to_vec(&worksheet).unwrap(),
            ),
            (&files.capture_path, serde_json::to_vec(&capture).unwrap()),
        ] {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
                .unwrap();
            file.write_all(&bytes).unwrap();
        }
        files
    }
    fn path(&self, suffix: &str) -> PathBuf {
        temp_path(self.name, suffix)
    }
    fn run(&self, limit: &str, output: &Path) -> Output {
        run(
            &self.report_path,
            &self.worksheet_path,
            &self.capture_path,
            limit,
            output,
        )
    }
}

impl Drop for FixtureFiles {
    fn drop(&mut self) {
        for path in [&self.report_path, &self.worksheet_path, &self.capture_path] {
            let _ = fs::remove_file(path);
        }
    }
}

fn run(report: &Path, worksheet: &Path, capture: &Path, limit: &str, output: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .arg("--full-text-review")
        .args([report, worksheet, capture])
        .arg(limit)
        .arg(output)
        .output()
        .unwrap()
}

fn temp_path(name: &str, suffix: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "conceptweave-fulltext-review-{}-{name}-{suffix}.json",
        std::process::id()
    ))
}

fn synthetic_fixture() -> (
    ClassificationReport,
    StewardReviewWorksheet,
    FullTextCapture,
) {
    let items: Vec<ZoteroItem> = serde_json::from_value(serde_json::json!([
        {"key":"ABCD2345","version":2,"data":{"itemType":"journalArticle","title":"synthetic paper"}},
        {"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}},
        {"key":"CDEF4567","version":2,"data":{"itemType":"book","title":"synthetic unattached paper"}}
    ])).unwrap();
    let mut report = classify_snapshot("10.0.1".into(), Some("synthetic-server".into()), 2, items);
    report.api_version = Some(3);
    report.schema_version = Some(44);
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    // Field order is the capture wire format, not a real provider observation.
    let evidence = format!(
        concat!(
            "{{\"capture_kind\":\"non_atomic_fulltext_sweep_v1\",",
            "\"metadata_report_digest\":\"{}\",\"metadata_snapshot_digest\":\"{}\",",
            "\"bibliographic_item_count\":2,\"started_unix_ms\":0,\"finished_unix_ms\":1,",
            "\"library_before\":{},\"manifest_before\":{},\"records\":[{{",
            "\"item_key\":\"BCDE3456\",\"metadata_response\":{},\"content_response\":{}}}],",
            "\"manifest_after\":{},\"library_after\":{}}}"
        ),
        json_digest(&report),
        report.snapshot_digest,
        response(200, Some(2), "[]"),
        response(200, None, r#"{"BCDE3456":3}"#),
        response(
            200,
            Some(1),
            r#"{"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}}"#
        ),
        response(
            200,
            Some(3),
            r#"{"content":"synthetic private text","providerExtra":true}"#
        ),
        response(200, None, r#"{"BCDE3456":3}"#),
        response(200, Some(2), "[]"),
    );
    let capture: FullTextCapture = serde_json::from_str(&format!(
        "{{\"capture_digest\":\"sha256:{:x}\",\"capture_evidence\":{evidence}}}",
        Sha256::digest(evidence.as_bytes())
    ))
    .unwrap();
    verify_full_text_capture(&capture, &report).unwrap();
    (report, worksheet, capture)
}

fn response(status: u16, version: Option<u64>, body: &str) -> String {
    #[derive(Serialize)]
    struct Response<'a> {
        status: u16,
        version: Option<u64>,
        body: &'a str,
    }
    serde_json::to_string(&Response {
        status,
        version,
        body,
    })
    .unwrap()
}

fn json_digest(value: &impl Serialize) -> String {
    format!(
        "sha256:{:x}",
        Sha256::digest(serde_json::to_vec(value).unwrap())
    )
}
