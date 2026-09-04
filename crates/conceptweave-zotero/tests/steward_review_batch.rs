use conceptweave_zotero::{
    Disposition, ItemData, StewardDecisionPatch, WorksheetError, ZoteroItem,
    apply_steward_decision_patch, build_steward_review_batch, build_steward_review_worksheet,
    classify_snapshot,
};

fn item(key: &str, title: &str, abstract_note: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: title.into(),
            abstract_note: abstract_note.into(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec!["COLLECTION".into()],
            tags: vec![],
        },
    }
}

#[test]
fn review_batch_is_deterministic_bounded_and_patch_compatible() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("C", "unmatched C", "context C"),
            item("A", "ontology alignment", ""),
            item("B", "unmatched B", "context B"),
        ],
    );
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    worksheet.decisions[0].reviewed_disposition = Some(Disposition::AlignmentVersioning);

    let batch = build_steward_review_batch(&report, &worksheet, 1).unwrap();
    assert_eq!(batch.remaining_count, 2);
    assert_eq!(batch.decisions.len(), 1);
    assert_eq!(batch.decisions[0].item_key, "B");
    assert_eq!(batch.decisions[0].title, "unmatched B");
    assert_eq!(
        batch.decisions[0].review_abstract_note.as_deref(),
        Some("context B")
    );
    assert_eq!(batch.decisions[0].reviewed_disposition, None);
    assert_eq!(
        build_steward_review_batch(&report, &worksheet, 1).unwrap(),
        batch
    );
    let serialized = serde_json::to_string(&batch).unwrap();
    assert_eq!(serialized.matches("context B").count(), 1);
    for omitted in [
        "snapshot_items",
        "child_item_keys",
        "model_receipt",
        "audit_summary",
    ] {
        assert!(!serialized.contains(omitted));
    }

    let mut completed_batch = batch.clone();
    completed_batch.decisions[0].reviewed_disposition = Some(Disposition::OutOfScope);
    let patch: StewardDecisionPatch =
        serde_json::from_value(serde_json::to_value(&completed_batch).unwrap()).unwrap();
    let updated = apply_steward_decision_patch(&report, &worksheet, &patch).unwrap();
    assert_eq!(
        updated.decisions[1].reviewed_disposition,
        Some(Disposition::OutOfScope)
    );

    assert_eq!(
        build_steward_review_batch(&report, &worksheet, 0),
        Err(WorksheetError::InvalidBatchLimit)
    );
    assert_eq!(
        build_steward_review_batch(&report, &worksheet, 101),
        Err(WorksheetError::InvalidBatchLimit)
    );
}

#[test]
fn review_batch_rejects_invalid_or_complete_workloads() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![item("A", "unmatched", "review context")],
    );
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();

    let mut invalid = worksheet.clone();
    invalid.decisions[0].item_version += 1;
    assert_eq!(
        build_steward_review_batch(&report, &invalid, 1),
        Err(WorksheetError::InvalidReport)
    );

    worksheet.decisions[0].reviewed_disposition = Some(Disposition::OutOfScope);
    assert_eq!(
        build_steward_review_batch(&report, &worksheet, 1),
        Err(WorksheetError::NoPendingDecisions)
    );
    assert_eq!(
        WorksheetError::InvalidBatchLimit.to_string(),
        "review batch limit must be between 1 and 100"
    );
    assert_eq!(
        WorksheetError::NoPendingDecisions.to_string(),
        "steward worksheet has no pending decisions"
    );
}
