#![cfg(unix)]

use conceptweave_zotero::{
    Disposition, GoldenSetApproval, ItemData, ZoteroItem, build_steward_review_worksheet,
    classify_snapshot,
};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[test]
fn artifact_commands_reject_distinct_path_spellings_for_one_input_file() {
    let item = ZoteroItem {
        source_record: None,
        key: "ITEM".into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: "ontology alignment".into(),
            abstract_note: "review context".into(),
            doi: "10.1000/example".into(),
            parent_item: String::new(),
            collections: vec!["COLLECTION".into()],
            tags: vec![],
        },
    };
    let report = classify_snapshot("9.0.6".into(), None, 42, vec![item]);
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    for decision in &mut worksheet.decisions {
        decision.reviewed_disposition = Some(Disposition::OutOfScope);
    }
    let approval = GoldenSetApproval {
        receipt_id: "receipt_1".into(),
        reviewer_subject: "steward_1".into(),
        library_version: worksheet.library_version,
        rule_revision: worksheet.rule_revision.clone(),
        snapshot_digest: worksheet.snapshot_digest.clone(),
        proposal_digest: conceptweave_zotero::classification_proposal_digest(&report),
        snapshot_items: worksheet.snapshot_items.clone(),
    };

    // These structs intentionally accept additional owner-only fields. Without checking file
    // identity, one JSON object can therefore be accepted as all three finalization inputs.
    let mut combined = serde_json::to_value(&report).unwrap();
    let object = combined.as_object_mut().unwrap();
    object.insert(
        "decisions".into(),
        serde_json::to_value(&worksheet.decisions).unwrap(),
    );
    object.insert(
        "receipt_id".into(),
        serde_json::to_value(&approval.receipt_id).unwrap(),
    );
    object.insert(
        "reviewer_subject".into(),
        serde_json::to_value(&approval.reviewer_subject).unwrap(),
    );
    object.insert(
        "proposal_digest".into(),
        serde_json::to_value(&approval.proposal_digest).unwrap(),
    );

    let temp = std::env::temp_dir().canonicalize().unwrap();
    let filename = format!(
        "conceptweave-zotero-finalize-alias-{}-input.json",
        std::process::id()
    );
    let input = temp.join(&filename);
    let output = temp.join(format!(
        "conceptweave-zotero-finalize-alias-{}-output.json",
        std::process::id()
    ));
    let _ = fs::remove_file(&input);
    let _ = fs::remove_file(&output);
    fs::write(&input, serde_json::to_vec(&combined).unwrap()).unwrap();
    fs::set_permissions(&input, fs::Permissions::from_mode(0o600)).unwrap();

    let report_path = input.to_str().unwrap().to_owned();
    let worksheet_path = format!("{}/./{}", temp.display(), filename);
    let approval_path = format!("{}//{}", temp.display(), filename);
    assert_ne!(report_path, worksheet_path);
    assert_ne!(worksheet_path, approval_path);

    let status = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--finalize",
            &report_path,
            &worksheet_path,
            &approval_path,
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    let progress_status = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--review-progress",
            &report_path,
            &worksheet_path,
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap();
    let patch_status = Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
        .args([
            "--apply-decision-patch",
            &report_path,
            &worksheet_path,
            &approval_path,
            output.to_str().unwrap(),
        ])
        .status()
        .unwrap();

    let _ = fs::remove_file(&input);
    let _ = fs::remove_file(&output);
    assert!(
        !status.success(),
        "finalization must reject three path spellings that resolve to one input artifact"
    );
    assert!(
        !progress_status.success(),
        "progress must reject two path spellings that resolve to one input artifact"
    );
    assert!(
        !patch_status.success(),
        "decision patching must reject path spellings that resolve to one input artifact"
    );
}
