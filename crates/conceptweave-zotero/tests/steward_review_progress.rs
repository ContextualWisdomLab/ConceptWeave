use conceptweave_zotero::{
    Disposition, ItemData, WorksheetError, ZoteroItem, assess_steward_review_progress,
    build_steward_review_worksheet, classify_snapshot,
};

fn classification_report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            ZoteroItem {
                source_record: None,
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
                source_record: None,
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
    let report = classification_report();
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

    let canonical = build_steward_review_worksheet(&report).unwrap();
    let mut invalid_report = classification_report();
    invalid_report.rule_revision.clear();
    assert_eq!(
        assess_steward_review_progress(&invalid_report, &canonical),
        Err(WorksheetError::InvalidReport)
    );
    let mut shifted = canonical.clone();
    shifted.library_version += 1;
    assert_eq!(
        assess_steward_review_progress(&report, &shifted),
        Err(WorksheetError::InvalidReport)
    );
    let mut shifted = canonical.clone();
    shifted.rule_revision.push_str("-changed");
    assert_eq!(
        assess_steward_review_progress(&report, &shifted),
        Err(WorksheetError::InvalidReport)
    );
    let mut shifted = canonical.clone();
    shifted.snapshot_digest.push_str("-changed");
    assert_eq!(
        assess_steward_review_progress(&report, &shifted),
        Err(WorksheetError::InvalidReport)
    );
    let mut shifted = canonical;
    shifted.snapshot_items.pop();
    assert_eq!(
        assess_steward_review_progress(&report, &shifted),
        Err(WorksheetError::InvalidReport)
    );

    let empty_report = classify_snapshot("9.0.6".into(), None, 42, vec![]);
    let empty_worksheet = build_steward_review_worksheet(&empty_report).unwrap();
    let empty = assess_steward_review_progress(&empty_report, &empty_worksheet).unwrap();
    assert_eq!(empty.total_count, 0);
    assert!(!empty.complete);
}

#[test]
fn progress_rejects_stale_or_blank_content_binding() {
    let mut report = classification_report();
    let worksheet = build_steward_review_worksheet(&report).unwrap();
    let mut blank = worksheet.clone();
    blank.proposal_digest.clear();
    assert_eq!(
        assess_steward_review_progress(&report, &blank),
        Err(WorksheetError::InvalidReport)
    );
    report.classified_items[0]
        .title
        .push_str(" changed context");
    assert_eq!(
        assess_steward_review_progress(&report, &worksheet),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn progress_preserves_pending_source_scope_and_opaque_identity() {
    let source = ZoteroItem {
        source_record: None,
        key: "PRIVATE_SOURCE".into(),
        version: 1,
        data: ItemData {
            item_type: "attachment".into(),
            title: "private source".into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    };
    let paper: ZoteroItem = serde_json::from_value(serde_json::json!({
        "key": "PAPER", "version": 1,
        "data": {"itemType": "book", "title": "ontology learning"}
    }))
    .unwrap();
    let report = classify_snapshot("9.0.6".into(), None, 42, vec![source, paper]);
    let mut worksheet = build_steward_review_worksheet(&report).unwrap();
    worksheet.decisions[0].reviewed_disposition = Some(Disposition::Generation);
    let progress = assess_steward_review_progress(&report, &worksheet).unwrap();
    let json = serde_json::to_value(&progress).unwrap();
    assert_eq!(json["pending_source_count"], 1);
    assert_eq!(json["proposal_digest"], worksheet.proposal_digest);
    assert!(!progress.complete);
    assert_eq!(progress.total_count, 1);
    assert_eq!(progress.decided_count, 1);
    assert_eq!(progress.remaining_count, 0);
    assert!(!json.to_string().contains("PRIVATE_SOURCE"));
}
