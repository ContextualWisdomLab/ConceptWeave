use conceptweave_zotero::{
    Disposition, ItemData, WorksheetError, ZoteroItem, assess_steward_review_progress,
    build_steward_review_worksheet, classify_snapshot,
};

fn report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            ZoteroItem {
                key: "B".into(),
                version: 8,
                data: ItemData {
                    item_type: "book".into(),
                    title: "unknown vocabulary".into(),
                    abstract_note: "private review context".into(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec!["PRIVATE_COLLECTION".into()],
                    tags: vec![],
                },
            },
            ZoteroItem {
                key: "A".into(),
                version: 7,
                data: ItemData {
                    item_type: "journalArticle".into(),
                    title: "ontology learning".into(),
                    abstract_note: String::new(),
                    doi: "10.1000/private".into(),
                    parent_item: String::new(),
                    collections: vec![],
                    tags: vec![],
                },
            },
        ],
    )
}

#[test]
fn progress_is_exact_aggregate_only_and_fail_closed() {
    let report = report();
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();

    let blank = assess_steward_review_progress(&report, &worksheet).unwrap();
    assert_eq!(blank.total_count, 2);
    assert_eq!(blank.decided_count, 0);
    assert_eq!(blank.remaining_count, 2);
    assert!(!blank.complete);

    worksheet.decisions[0].reviewed_disposition = Some(Disposition::Generation);
    let partial = assess_steward_review_progress(&report, &worksheet).unwrap();
    assert_eq!(partial.decided_count, 1);
    assert_eq!(partial.remaining_count, 1);
    assert!(!partial.complete);
    let serialized = serde_json::to_string(&partial).unwrap();
    assert!(!serialized.contains("PRIVATE"));
    assert!(!serialized.contains("10.1000"));
    assert!(!serialized.contains("item_key"));
    assert!(!serialized.contains("reviewer"));
    assert!(!serialized.contains("receipt"));

    worksheet.decisions[1].reviewed_disposition = Some(Disposition::OutOfScope);
    let complete = assess_steward_review_progress(&report, &worksheet).unwrap();
    assert_eq!(complete.decided_count, 2);
    assert_eq!(complete.remaining_count, 0);
    assert!(complete.complete);

    let mut invalid = worksheet.clone();
    invalid.decisions[0].reviewed_disposition = Some(Disposition::NeedsStewardReview);
    assert_eq!(
        assess_steward_review_progress(&report, &invalid),
        Err(WorksheetError::InvalidReport)
    );

    let mut tampered = worksheet.clone();
    tampered.decisions.swap(0, 1);
    assert_eq!(
        assess_steward_review_progress(&report, &tampered),
        Err(WorksheetError::InvalidReport)
    );

    let mut missing = worksheet;
    missing.decisions.pop();
    assert_eq!(
        assess_steward_review_progress(&report, &missing),
        Err(WorksheetError::InvalidReport)
    );
}
