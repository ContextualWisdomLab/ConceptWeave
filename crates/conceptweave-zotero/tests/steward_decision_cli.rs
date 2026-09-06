#![cfg(unix)]

use conceptweave_zotero::{
    Disposition, ItemData, StewardDecisionPatch, StewardDecisionUpdate, StewardReviewWorksheet,
    ZoteroItem, build_steward_review_worksheet, classify_snapshot,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[test]
fn decision_patch_cli_writes_one_owner_only_updated_worksheet() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![ZoteroItem {
            source_record: None,
            key: "ITEM".into(),
            version: 7,
            data: ItemData {
                item_type: "book".into(),
                title: "ontology learning".into(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }],
    );
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let patch = StewardDecisionPatch {
        proposal_digest: worksheet.proposal_digest.clone(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.clone(),
        snapshot_digest: report.snapshot_digest.clone(),
        decisions: vec![StewardDecisionUpdate {
            item_key: "ITEM".into(),
            item_version: 7,
            reviewed_disposition: Disposition::AlignmentVersioning,
        }],
    };

    let report_path = private_input("decision-cli-report", &report);
    let worksheet_path = private_input("decision-cli-worksheet", &worksheet);
    let patch_path = private_input("decision-cli-patch", &patch);
    let output_path = temp_path("decision-cli-output");
    let replay_path = temp_path("decision-cli-replay");
    let _ = fs::remove_file(&output_path);
    let _ = fs::remove_file(&replay_path);

    let status = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--apply-decision-patch",
            report_path.to_str().unwrap(),
            worksheet_path.to_str().unwrap(),
            patch_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();

    assert!(status.success());
    let updated: StewardReviewWorksheet =
        serde_json::from_slice(&fs::read(&output_path).unwrap()).unwrap();
    assert_eq!(
        updated.decisions[0].reviewed_disposition,
        Some(Disposition::AlignmentVersioning)
    );
    assert_eq!(
        fs::metadata(&output_path).unwrap().permissions().mode() & 0o777,
        0o600
    );

    let replay_status = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--apply-decision-patch",
            report_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
            patch_path.to_str().unwrap(),
            replay_path.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    assert!(replay_status.success());
    assert_eq!(
        fs::read(&replay_path).unwrap(),
        fs::read(&output_path).unwrap()
    );

    for path in [
        report_path,
        worksheet_path,
        patch_path,
        output_path,
        replay_path,
    ] {
        fs::remove_file(path).unwrap();
    }
}

#[test]
fn decision_patch_cli_never_overwrites_or_emits_invalid_work() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![ZoteroItem {
            source_record: None,
            key: "ITEM".into(),
            version: 7,
            data: ItemData {
                item_type: "book".into(),
                title: "ontology learning".into(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }],
    );
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let invalid_patch = StewardDecisionPatch {
        proposal_digest: worksheet.proposal_digest.clone(),
        library_version: report.library_version + 1,
        rule_revision: report.rule_revision.clone(),
        snapshot_digest: report.snapshot_digest.clone(),
        decisions: vec![StewardDecisionUpdate {
            item_key: "ITEM".into(),
            item_version: 7,
            reviewed_disposition: Disposition::OutOfScope,
        }],
    };
    let report_path = private_input("decision-invalid-report", &report);
    let worksheet_path = private_input("decision-invalid-worksheet", &worksheet);
    let patch_path = private_input("decision-invalid-patch", &invalid_patch);
    let absent_output = temp_path("decision-invalid-output");
    let existing_output = temp_path("decision-existing-output");
    let _ = fs::remove_file(&absent_output);
    let _ = fs::remove_file(&existing_output);

    assert!(!run_patch(&report_path, &worksheet_path, &patch_path, &absent_output).success());
    assert!(!absent_output.exists());

    fs::write(&existing_output, b"preserve me").unwrap();
    assert!(!run_patch(&report_path, &worksheet_path, &patch_path, &existing_output).success());
    assert_eq!(fs::read(&existing_output).unwrap(), b"preserve me");

    for path in [report_path, worksheet_path, patch_path, existing_output] {
        fs::remove_file(path).unwrap();
    }
}

fn run_patch(
    report: &std::path::Path,
    worksheet: &std::path::Path,
    patch: &std::path::Path,
    output: &std::path::Path,
) -> std::process::ExitStatus {
    Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--apply-decision-patch",
            report.to_str().unwrap(),
            worksheet.to_str().unwrap(),
            patch.to_str().unwrap(),
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap()
}

fn private_input(name: &str, value: &impl serde::Serialize) -> std::path::PathBuf {
    let path = temp_path(name);
    let _ = fs::remove_file(&path);
    fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    path
}

fn temp_path(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!(
        "conceptweave-zotero-{}-{name}.json",
        std::process::id()
    ))
}
