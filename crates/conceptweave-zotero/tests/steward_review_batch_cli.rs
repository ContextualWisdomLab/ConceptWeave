#![cfg(unix)]

use conceptweave_zotero::{
    Disposition, ItemData, StewardReviewBatch, StewardReviewWorksheet, ZoteroItem,
    build_steward_review_batch, build_steward_review_worksheet, classify_snapshot,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

#[test]
fn review_batch_cli_writes_sensitive_context_owner_only() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("B", "unmatched B"), item("A", "unmatched A")],
    );
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let report_path = private_input("batch-cli-report", &report);
    let worksheet_path = private_input("batch-cli-worksheet", &worksheet);
    let output_path = temp_path("batch-cli-output");
    let _ = fs::remove_file(&output_path);

    assert!(run_batch(&report_path, &worksheet_path, "1", &output_path).success());
    let output: serde_json::Value =
        serde_json::from_slice(&fs::read(&output_path).unwrap()).unwrap();
    assert_eq!(output["remaining_count"], 2);
    assert_eq!(output["decisions"].as_array().unwrap().len(), 1);
    assert_eq!(output["decisions"][0]["item_key"], "A");
    assert_eq!(
        output["decisions"][0]["reviewed_disposition"],
        serde_json::Value::Null
    );
    assert!(output.get("snapshot_items").is_none());
    assert!(output.get("duplicate_candidates").is_none());
    assert_eq!(
        fs::metadata(&output_path).unwrap().permissions().mode() & 0o777,
        0o600
    );

    for path in [report_path, worksheet_path, output_path] {
        fs::remove_file(path).unwrap();
    }
}

#[test]
fn review_batch_cli_emits_nothing_for_complete_or_existing_output() {
    let report = classify_snapshot("9.0.6".into(), None, 42, vec![item("A", "unmatched")]);
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    worksheet.decisions[0].reviewed_disposition = Some(Disposition::OutOfScope);
    let report_path = private_input("batch-complete-report", &report);
    let worksheet_path = private_input("batch-complete-worksheet", &worksheet);
    let absent_output = temp_path("batch-complete-output");
    let existing_output = temp_path("batch-existing-output");
    let _ = fs::remove_file(&absent_output);
    let _ = fs::remove_file(&existing_output);

    assert!(!run_batch(&report_path, &worksheet_path, "1", &absent_output).success());
    assert!(!absent_output.exists());
    fs::write(&existing_output, b"preserve me").unwrap();
    assert!(!run_batch(&report_path, &worksheet_path, "1", &existing_output).success());
    assert_eq!(fs::read(&existing_output).unwrap(), b"preserve me");

    for path in [report_path, worksheet_path, existing_output] {
        fs::remove_file(path).unwrap();
    }
}

#[test]
fn completed_review_batch_cli_validates_context_before_updating_worksheet() {
    let report = classify_snapshot("9.0.6".into(), None, 42, vec![item("A", "unmatched")]);
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let mut batch = build_steward_review_batch(&report, &worksheet, 1).unwrap();
    batch.decisions[0].reviewed_disposition = Some(Disposition::OutOfScope);
    let report_path = private_input("apply-batch-report", &report);
    let worksheet_path = private_input("apply-batch-worksheet", &worksheet);
    let batch_path = private_input("apply-batch-input", &batch);
    let output_path = temp_path("apply-batch-output");
    let _ = fs::remove_file(&output_path);

    assert!(run_apply_batch(&report_path, &worksheet_path, &batch_path, &output_path).success());
    let updated: StewardReviewWorksheet =
        serde_json::from_slice(&fs::read(&output_path).unwrap()).unwrap();
    assert_eq!(
        updated.decisions[0].reviewed_disposition,
        Some(Disposition::OutOfScope)
    );

    let mut tampered: StewardReviewBatch = batch;
    tampered.decisions[0].title = "different context".into();
    let tampered_path = private_input("apply-batch-tampered", &tampered);
    let rejected_path = temp_path("apply-batch-rejected");
    let _ = fs::remove_file(&rejected_path);
    assert!(!run_apply_batch(&report_path, &worksheet_path, &tampered_path, &rejected_path).success());
    assert!(!rejected_path.exists());

    for path in [
        report_path,
        worksheet_path,
        batch_path,
        output_path,
        tampered_path,
    ] {
        fs::remove_file(path).unwrap();
    }
}

fn item(key: &str, title: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: title.into(),
            abstract_note: "review context".into(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

fn private_input(name: &str, value: &impl serde::Serialize) -> std::path::PathBuf {
    let path = temp_path(name);
    let _ = fs::remove_file(&path);
    fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    path
}

fn run_batch(
    report: &Path,
    worksheet: &Path,
    limit: &str,
    output: &Path,
) -> std::process::ExitStatus {
    Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--review-batch",
            report.to_str().unwrap(),
            worksheet.to_str().unwrap(),
            limit,
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap()
}

fn run_apply_batch(
    report: &Path,
    worksheet: &Path,
    batch: &Path,
    output: &Path,
) -> std::process::ExitStatus {
    Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--apply-review-batch",
            report.to_str().unwrap(),
            worksheet.to_str().unwrap(),
            batch.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap()
}

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "conceptweave-zotero-{}-{name}.json",
        std::process::id()
    ))
}
