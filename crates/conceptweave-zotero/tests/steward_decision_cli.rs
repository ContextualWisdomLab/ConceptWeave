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
    let _ = fs::remove_file(&output_path);

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

    for path in [report_path, worksheet_path, patch_path, output_path] {
        fs::remove_file(path).unwrap();
    }
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
