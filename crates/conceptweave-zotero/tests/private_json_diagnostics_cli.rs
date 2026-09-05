#![cfg(unix)]

use conceptweave_zotero::{
    Disposition, GoldenSetApproval, ItemData, ReviewedGoldenSet, ZoteroItem,
    build_steward_review_worksheet, classification_proposal_digest, classify_snapshot,
};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::PathBuf;
use std::process::{Command, Output};

/// Owns only synthetic, uniquely named direct-temp-child artifacts for one CLI case.
struct FinalizationFiles {
    paths: [PathBuf; 4],
    inputs: [Vec<u8>; 3],
}

impl FinalizationFiles {
    fn new(case_name: &str, mutate: impl FnOnce(&mut [serde_json::Value; 3])) -> Self {
        let report = classify_snapshot(
            "9.0.6".into(),
            None,
            42,
            vec![ZoteroItem {
                source_record: None,
                key: "SYNTHETIC".into(),
                version: 7,
                data: ItemData {
                    item_type: "journalArticle".into(),
                    title: "synthetic ontology alignment".into(),
                    abstract_note: String::new(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec![],
                    tags: vec![],
                },
            }],
        );
        let mut worksheet = build_steward_review_worksheet(&report).unwrap();
        worksheet.decisions[0].reviewed_disposition = Some(Disposition::AlignmentVersioning);
        let approval = GoldenSetApproval {
            receipt_id: "synthetic-receipt".into(),
            reviewer_subject: "synthetic-steward".into(),
            library_version: worksheet.library_version,
            rule_revision: worksheet.rule_revision.clone(),
            snapshot_digest: worksheet.snapshot_digest.clone(),
            proposal_digest: classification_proposal_digest(&report),
            snapshot_items: worksheet.snapshot_items.clone(),
        };
        let mut values = [
            serde_json::to_value(report).unwrap(),
            serde_json::to_value(worksheet).unwrap(),
            serde_json::to_value(approval).unwrap(),
        ];
        mutate(&mut values);
        let inputs = values.map(|value| serde_json::to_vec(&value).unwrap());
        let paths = ["report", "worksheet", "approval", "golden"].map(|role| {
            std::env::temp_dir().join(format!(
                "conceptweave-pr29-diagnostics-{}-{case_name}-{role}.json",
                std::process::id()
            ))
        });
        assert!(!paths[3].exists());
        for (path, bytes) in paths.iter().zip(&inputs) {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(0o600)
                .open(path)
                .unwrap();
            file.set_permissions(fs::Permissions::from_mode(0o600))
                .unwrap();
            file.write_all(bytes).unwrap();
        }
        Self { paths, inputs }
    }

    fn run(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_conceptweave-zotero"))
            .arg("--finalize")
            .args(&self.paths)
            .output()
            .unwrap()
    }

    fn assert_private_rejection(&self, role: &str) {
        let output = self.run();
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
        assert!(!self.paths[3].exists());
        for (path, original) in self.paths.iter().zip(&self.inputs) {
            assert_eq!(&fs::read(path).unwrap(), original);
        }
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(
            stderr.contains(&format!("{role}: review input is invalid")),
            "{stderr}"
        );
        assert!(!stderr.contains("synthetic-private"), "{stderr}");
    }
}

impl Drop for FinalizationFiles {
    fn drop(&mut self) {
        for path in &self.paths {
            let _ = fs::remove_file(path);
        }
    }
}

#[test]
fn finalization_keeps_valid_synthetic_inputs_and_private_output_compatible() {
    let files = FinalizationFiles::new("valid", |_| {});
    let output = files.run();
    assert!(output.status.success(), "{:?}", output.stderr);
    let golden: ReviewedGoldenSet =
        serde_json::from_slice(&fs::read(&files.paths[3]).unwrap()).unwrap();
    let approval: GoldenSetApproval = serde_json::from_slice(&files.inputs[2]).unwrap();
    assert_eq!(golden.approval, approval);
    assert_eq!(
        golden.labels[0].expected_disposition,
        Disposition::AlignmentVersioning
    );
    assert_eq!(
        fs::metadata(&files.paths[3]).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

#[test]
fn finalization_diagnostic_hides_private_worksheet_enum_value() {
    let files = FinalizationFiles::new("enum", |values| {
        values[1]["decisions"][0]["reviewed_disposition"] =
            serde_json::json!("synthetic-private-enum-sentinel");
    });
    files.assert_private_rejection("worksheet");
}

#[test]
fn finalization_diagnostic_hides_private_approval_scalar_value() {
    let files = FinalizationFiles::new("scalar", |values| {
        values[2]["library_version"] = serde_json::json!("synthetic-private-scalar-sentinel");
    });
    files.assert_private_rejection("approval");
}
