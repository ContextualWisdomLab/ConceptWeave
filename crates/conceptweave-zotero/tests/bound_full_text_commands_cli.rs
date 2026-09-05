#![cfg(unix)]

use conceptweave_zotero::{
    ClassificationReport, Disposition, FullTextCapture, FullTextReviewApproval,
    FullTextReviewWorksheet, ZoteroItem, apply_full_text_review_view,
    build_bound_full_text_review_json, build_full_text_review_worksheet,
    classification_proposal_digest, classify_snapshot, finalize_full_text_review,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::cell::RefCell;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

#[test]
fn library_two_batch_flow_keeps_the_full_review_and_approval_envelopes() {
    let case = Case::new("library");
    let original = build_full_text_review_worksheet(&case.report, &case.capture).unwrap();
    let mut worksheet = original;
    for _ in 0..2 {
        let view =
            build_bound_full_text_review_json(&case.report, &worksheet, &case.capture, 1).unwrap();
        worksheet =
            apply_full_text_review_view(&case.report, &worksheet, &case.capture, &complete(&view))
                .unwrap();
    }
    assert!(build_bound_full_text_review_json(&case.report, &worksheet, &case.capture, 1).is_err());
    let approval = case.approval();
    let golden =
        finalize_full_text_review(&case.report, &worksheet, &case.capture, approval).unwrap();
    assert_full_golden(&serde_json::to_value(golden).unwrap());
}

#[test]
fn cli_two_batches_finalize_without_issuing_or_rewriting_approval() {
    let case = Case::new("flow");
    let [report, mut worksheet, capture] = case.inputs();
    let first_worksheet = worksheet.clone();
    let original = fs::read(&worksheet).unwrap();
    let approval = case.json("approval", &case.approval());
    let original_approval = fs::read(&approval).unwrap();
    for index in 0..2 {
        let view = case.path(&format!("view-{index}"));
        assert_success(run(
            "--bound-full-text-review",
            &[&report, &worksheet, &capture, Path::new("1"), &view],
        ));
        let displayed: Value = serde_json::from_slice(&fs::read(&view).unwrap()).unwrap();
        assert_eq!(displayed["review_batch"]["remaining_count"], 2 - index);
        let completed = case.bytes(
            &format!("completed-{index}"),
            &complete(&fs::read(&view).unwrap()),
        );
        let next = case.path(&format!("next-{index}"));
        assert_success(run(
            "--apply-full-text-review",
            &[&report, &worksheet, &capture, &completed, &next],
        ));
        let restored: FullTextReviewWorksheet =
            serde_json::from_slice(&fs::read(&next).unwrap()).unwrap();
        let restored = serde_json::to_value(restored).unwrap();
        assert!(restored.get("full_text_worksheet_v1").is_some());
        assert_private(&view);
        assert_private(&next);
        assert!(
            !run(
                "--apply-full-text-review",
                &[&report, &worksheet, &capture, &completed, &next]
            )
            .status
            .success()
        );
        worksheet = next;
    }
    let golden = case.path("golden");
    assert_success(run(
        "--finalize-full-text-review",
        &[&report, &worksheet, &capture, &approval, &golden],
    ));
    assert_full_golden(&serde_json::from_slice(&fs::read(&golden).unwrap()).unwrap());
    assert_private(&golden);
    assert_eq!(fs::read(&approval).unwrap(), original_approval);
    assert_eq!(fs::read(&first_worksheet).unwrap(), original);
    let missing_approval = case.path("missing-approval");
    let rejected = case.path("missing-approval-output");
    assert!(
        !run(
            "--finalize-full-text-review",
            &[&report, &worksheet, &capture, &missing_approval, &rejected]
        )
        .status
        .success()
    );
    assert!(!missing_approval.exists());
    assert!(!rejected.exists());
}

#[test]
fn cli_rejects_replay_stale_tampered_duplicate_and_legacy_review_inputs() {
    let case = Case::new("reject");
    let [report, worksheet, capture] = case.inputs();
    let blank: FullTextReviewWorksheet =
        serde_json::from_slice(&fs::read(&worksheet).unwrap()).unwrap();
    let view = build_bound_full_text_review_json(&case.report, &blank, &case.capture, 1).unwrap();
    let completed = complete(&view);
    let completed_path = case.bytes("completed", &completed);
    let next = case.path("next");
    assert_success(run(
        "--apply-full-text-review",
        &[&report, &worksheet, &capture, &completed_path, &next],
    ));
    let saved_next = fs::read(&next).unwrap();
    let replay = case.path("replay");
    assert!(
        !run(
            "--apply-full-text-review",
            &[&report, &next, &capture, &completed_path, &replay]
        )
        .status
        .success()
    );
    assert!(!replay.exists());
    assert_eq!(fs::read(&next).unwrap(), saved_next);

    let mut tampered: Value = serde_json::from_slice(&completed).unwrap();
    tampered["attachment_evidence"]["ABCD2345"][0]["content_response"]["body"] =
        "synthetic tampering".into();
    let original = String::from_utf8(completed.clone()).unwrap();
    let root_duplicate = format!("{{\"view_kind\":\"wrong\",{}", &original[1..]);
    let nested_duplicate = original.replacen(
        "\"reviewed_disposition\":",
        "\"reviewed_disposition\":null,\"reviewed_disposition\":",
        1,
    );
    let legacy: Value = serde_json::from_slice(&completed).unwrap();
    for (index, bytes) in [
        view,
        serde_json::to_vec(&tampered).unwrap(),
        root_duplicate.into_bytes(),
        nested_duplicate.into_bytes(),
        serde_json::to_vec(&legacy["review_batch"]).unwrap(),
    ]
    .into_iter()
    .enumerate()
    {
        let invalid = case.bytes(&format!("invalid-{index}"), &bytes);
        let rejected = case.path(&format!("rejected-{index}"));
        assert!(
            !run(
                "--apply-full-text-review",
                &[&report, &worksheet, &capture, &invalid, &rejected]
            )
            .status
            .success()
        );
        assert!(!rejected.exists());
    }
    let legacy_worksheet = case.json(
        "legacy-worksheet",
        &serde_json::to_value(blank).unwrap()["full_text_worksheet_v1"],
    );
    let rejected = case.path("legacy-output");
    assert!(
        !run(
            "--bound-full-text-review",
            &[
                &report,
                &legacy_worksheet,
                &capture,
                Path::new("1"),
                &rejected
            ]
        )
        .status
        .success()
    );
    assert!(!rejected.exists());
}

#[test]
fn cli_finalization_rejects_incomplete_wrong_capture_and_legacy_approval() {
    let case = Case::new("finalize");
    let [report, worksheet, capture] = case.inputs();
    let approval = case.json("approval", &case.approval());
    let output = case.path("incomplete");
    assert!(
        !run(
            "--finalize-full-text-review",
            &[&report, &worksheet, &capture, &approval, &output]
        )
        .status
        .success()
    );
    assert!(!output.exists());
    let blank: FullTextReviewWorksheet =
        serde_json::from_slice(&fs::read(&worksheet).unwrap()).unwrap();
    let view = build_bound_full_text_review_json(&case.report, &blank, &case.capture, 2).unwrap();
    let decided =
        apply_full_text_review_view(&case.report, &blank, &case.capture, &complete(&view)).unwrap();
    let decided = case.json("decided", &decided);
    let mut wrong_capture = serde_json::to_value(&case.capture).unwrap();
    wrong_capture["capture_digest"] = "wrong-capture".into();
    let wrong_capture = case.json("wrong-capture", &wrong_capture);
    let mut wrong_approval = serde_json::to_value(case.approval()).unwrap();
    wrong_approval["capture_digest"] = "wrong-capture".into();
    let wrong_approval = case.json("wrong-approval", &wrong_approval);
    let legacy_approval = case.json(
        "legacy-approval",
        &serde_json::to_value(case.approval()).unwrap()["full_text_approval_v1"],
    );
    for (index, (capture_path, approval_path)) in [
        (&wrong_capture, &approval),
        (&capture, &wrong_approval),
        (&capture, &legacy_approval),
    ]
    .into_iter()
    .enumerate()
    {
        let output = case.path(&format!("wrong-{index}"));
        assert!(
            !run(
                "--finalize-full-text-review",
                &[&report, &decided, capture_path, approval_path, &output]
            )
            .status
            .success()
        );
        assert!(!output.exists());
    }
}

#[test]
fn cli_commands_reuse_private_permissions_alias_and_size_boundaries() {
    let case = Case::new("privacy");
    let [report, worksheet, capture] = case.inputs();
    let blank = build_full_text_review_worksheet(&case.report, &case.capture).unwrap();
    let completed = case.bytes(
        "completed",
        &complete(
            &build_bound_full_text_review_json(&case.report, &blank, &case.capture, 1).unwrap(),
        ),
    );
    let output = case.path("output");
    fs::set_permissions(&completed, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(
        !run(
            "--apply-full-text-review",
            &[&report, &worksheet, &capture, &completed, &output]
        )
        .status
        .success()
    );
    fs::set_permissions(&completed, fs::Permissions::from_mode(0o600)).unwrap();
    let symlink_path = case.path("symlink");
    symlink(&completed, &symlink_path).unwrap();
    assert!(
        !run(
            "--apply-full-text-review",
            &[&report, &worksheet, &capture, &symlink_path, &output]
        )
        .status
        .success()
    );
    let hard_link = case.path("hard-link");
    fs::hard_link(&completed, &hard_link).unwrap();
    assert!(
        !run(
            "--apply-full-text-review",
            &[&report, &worksheet, &capture, &hard_link, &output]
        )
        .status
        .success()
    );
    fs::remove_file(hard_link).unwrap();
    let alias = worksheet
        .parent()
        .unwrap()
        .join(".")
        .join(worksheet.file_name().unwrap());
    assert!(
        !run(
            "--apply-full-text-review",
            &[&report, &worksheet, &capture, &alias, &output]
        )
        .status
        .success()
    );
    assert!(
        !run(
            "--apply-full-text-review",
            &[&report, &worksheet, &capture, &completed, &worksheet]
        )
        .status
        .success()
    );
    OpenOptions::new()
        .write(true)
        .open(&completed)
        .unwrap()
        .set_len(16 * 1024 * 1024 + 1)
        .unwrap();
    let oversized = run(
        "--apply-full-text-review",
        &[&report, &worksheet, &capture, &completed, &output],
    );
    assert!(!oversized.status.success());
    assert!(
        String::from_utf8(oversized.stderr)
            .unwrap()
            .contains("size limit")
    );
    assert!(!output.exists());
}

fn assert_success(output: Output) {
    assert!(output.status.success(), "{:?}", output.stderr);
    assert!(output.stdout.is_empty());
}

fn assert_private(path: &Path) {
    assert_eq!(
        fs::metadata(path).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

fn assert_full_golden(golden: &Value) {
    assert!(golden.get("capture_digest").is_some());
    assert!(golden.get("labels").is_none());
    assert!(golden.get("evaluation").is_none());
    let reviewed = &golden["full_text_golden_set_v1"];
    assert_eq!(reviewed["labels"].as_array().unwrap().len(), 2);
    assert!(
        reviewed["labels"]
            .as_array()
            .unwrap()
            .iter()
            .all(|label| label["expected_disposition"] == json!(Disposition::OutOfScope))
    );
    assert_eq!(
        reviewed["approval"]["receipt_id"],
        "synthetic-existing-receipt"
    );
}

fn complete(view: &[u8]) -> Vec<u8> {
    let mut value: Value = serde_json::from_slice(view).unwrap();
    for row in value["review_batch"]["decisions"].as_array_mut().unwrap() {
        row["reviewed_disposition"] = json!(Disposition::OutOfScope);
    }
    serde_json::to_vec(&value).unwrap()
}

fn run(mode: &str, arguments: &[&Path]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .arg(mode)
        .args(arguments)
        .output()
        .unwrap()
}

struct Case {
    name: &'static str,
    report: ClassificationReport,
    capture: FullTextCapture,
    paths: RefCell<Vec<PathBuf>>,
}

impl Case {
    fn new(name: &'static str) -> Self {
        let (report, capture) = synthetic_fixture();
        Self {
            name,
            report,
            capture,
            paths: RefCell::new(Vec::new()),
        }
    }
    fn path(&self, suffix: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!(
            "conceptweave-bound-review-{}-{}-{suffix}.json",
            std::process::id(),
            self.name
        ));
        self.paths.borrow_mut().push(path.clone());
        path
    }
    fn bytes(&self, suffix: &str, bytes: &[u8]) -> PathBuf {
        let path = self.path(suffix);
        OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
            .unwrap()
            .write_all(bytes)
            .unwrap();
        path
    }
    fn json(&self, suffix: &str, value: &impl serde::Serialize) -> PathBuf {
        self.bytes(suffix, &serde_json::to_vec(value).unwrap())
    }
    fn inputs(&self) -> [PathBuf; 3] {
        [
            self.json("report", &self.report),
            self.json(
                "worksheet",
                &build_full_text_review_worksheet(&self.report, &self.capture).unwrap(),
            ),
            self.json("capture", &self.capture),
        ]
    }
    fn approval(&self) -> FullTextReviewApproval {
        serde_json::from_value(json!({
            "capture_digest":serde_json::to_value(&self.capture).unwrap()["capture_digest"],
            "full_text_approval_v1":{
                "receipt_id":"synthetic-existing-receipt", "reviewer_subject":"synthetic-steward",
                "library_version":self.report.library_version, "rule_revision":self.report.rule_revision,
                "snapshot_digest":self.report.snapshot_digest, "proposal_digest":classification_proposal_digest(&self.report),
                "snapshot_items":self.report.snapshot_items,
            }
        })).unwrap()
    }
}

impl Drop for Case {
    fn drop(&mut self) {
        for path in self.paths.get_mut() {
            let _ = fs::remove_file(path);
        }
    }
}

fn synthetic_fixture() -> (ClassificationReport, FullTextCapture) {
    let items: Vec<ZoteroItem> = serde_json::from_value(json!([
        {"key":"ABCD2345","version":2,"data":{"itemType":"journalArticle","title":"synthetic paper"}},
        {"key":"BCDE3456","version":1,"data":{"itemType":"attachment","parentItem":"ABCD2345"}},
        {"key":"CDEF4567","version":2,"data":{"itemType":"book","title":"synthetic unattached paper"}}
    ])).unwrap();
    let mut report = classify_snapshot("10.0.1".into(), Some("synthetic-server".into()), 2, items);
    report.api_version = Some(3);
    report.schema_version = Some(44);
    let evidence = format!(
        concat!(
            "{{\"capture_kind\":\"non_atomic_fulltext_sweep_v1\",",
            "\"metadata_report_digest\":\"sha256:{:x}\",\"metadata_snapshot_digest\":\"{}\",",
            "\"bibliographic_item_count\":2,\"started_unix_ms\":0,\"finished_unix_ms\":1,",
            "\"library_before\":{},\"manifest_before\":{},\"records\":[{{",
            "\"item_key\":\"BCDE3456\",\"metadata_response\":{},\"content_response\":{}}}],",
            "\"manifest_after\":{},\"library_after\":{}}}"
        ),
        Sha256::digest(serde_json::to_vec(&report).unwrap()),
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
    build_full_text_review_worksheet(&report, &capture).unwrap();
    (report, capture)
}

fn response(status: u16, version: Option<u64>, body: &str) -> String {
    #[derive(serde::Serialize)]
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
