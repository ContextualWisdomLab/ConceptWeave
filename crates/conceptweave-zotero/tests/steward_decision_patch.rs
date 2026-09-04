use conceptweave_zotero::{
    Disposition, ItemData, StewardDecisionPatch, StewardDecisionUpdate, WorksheetError, ZoteroItem,
    apply_steward_decision_patch, build_steward_review_worksheet, classify_snapshot,
};

fn item(key: &str, title: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "book".into(),
            title: title.into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

fn report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "ontology learning"), item("B", "unmatched")],
    )
}

fn patch(item_key: &str, item_version: u64, disposition: Disposition) -> StewardDecisionPatch {
    let report = report();
    StewardDecisionPatch {
        library_version: report.library_version,
        rule_revision: report.rule_revision,
        snapshot_digest: report.snapshot_digest,
        decisions: vec![StewardDecisionUpdate {
            item_key: item_key.into(),
            item_version,
            reviewed_disposition: disposition,
        }],
    }
}

#[test]
fn decision_patch_is_snapshot_bound_idempotent_and_non_overwriting() {
    let report = report();
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let update = patch("A", 7, Disposition::AlignmentVersioning);

    let updated = apply_steward_decision_patch(&report, &worksheet, &update).unwrap();
    assert_eq!(
        updated.decisions[0].reviewed_disposition,
        Some(Disposition::AlignmentVersioning)
    );
    assert_eq!(
        apply_steward_decision_patch(&report, &updated, &update).unwrap(),
        updated
    );

    let conflicting = patch("A", 7, Disposition::OutOfScope);
    assert_eq!(
        apply_steward_decision_patch(&report, &updated, &conflicting),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn decision_patch_rejects_invalid_identity_and_truth() {
    let report = report();
    let worksheet = build_steward_review_worksheet(&report).unwrap();

    let mut invalid_report = crate::report();
    invalid_report.rule_revision.clear();
    assert_eq!(
        apply_steward_decision_patch(
            &invalid_report,
            &worksheet,
            &patch("A", 7, Disposition::AlignmentVersioning)
        ),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid_worksheet = worksheet.clone();
    invalid_worksheet.decisions[0].item_version += 1;
    assert_eq!(
        apply_steward_decision_patch(
            &report,
            &invalid_worksheet,
            &patch("A", 7, Disposition::AlignmentVersioning)
        ),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = patch("A", 7, Disposition::AlignmentVersioning);
    invalid.library_version += 1;
    assert_eq!(
        apply_steward_decision_patch(&report, &worksheet, &invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = patch("A", 7, Disposition::AlignmentVersioning);
    invalid.rule_revision.clear();
    assert_eq!(
        apply_steward_decision_patch(&report, &worksheet, &invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut invalid = patch("A", 7, Disposition::AlignmentVersioning);
    invalid.snapshot_digest.clear();
    assert_eq!(
        apply_steward_decision_patch(&report, &worksheet, &invalid),
        Err(WorksheetError::InvalidReport)
    );

    for invalid in [
        patch(" ", 7, Disposition::OutOfScope),
        patch("UNKNOWN", 7, Disposition::OutOfScope),
        patch("A", 8, Disposition::OutOfScope),
        patch("A", 7, Disposition::NeedsStewardReview),
    ] {
        assert_eq!(
            apply_steward_decision_patch(&report, &worksheet, &invalid),
            Err(WorksheetError::InvalidReport)
        );
    }

    let mut duplicate = patch("A", 7, Disposition::AlignmentVersioning);
    duplicate.decisions.push(duplicate.decisions[0].clone());
    assert_eq!(
        apply_steward_decision_patch(&report, &worksheet, &duplicate),
        Err(WorksheetError::InvalidReport)
    );

    let mut empty = patch("A", 7, Disposition::AlignmentVersioning);
    empty.decisions.clear();
    assert_eq!(
        apply_steward_decision_patch(&report, &worksheet, &empty),
        Err(WorksheetError::InvalidReport)
    );
}
