use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionError,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review,
};

fn item(key: &str, version: u64, item_type: &str, parent_item: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: item_type.into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: parent_item.into(),
            collections: vec![],
            tags: vec![],
        },
    }
}

fn resolution(
    key: &str,
    version: u64,
    item_type: &str,
    parent_item: &str,
    library_version: u64,
) -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: key.into(),
        item_version: version,
        library_version,
        item_type: item_type.into(),
        parent_item_key: parent_item.into(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "The source remains useful independent evidence.".into(),
    }
}

#[test]
fn source_resolution_review_requires_the_exact_pending_snapshot_set() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
            item("PAPER", 40, "journalArticle", ""),
            item("SOURCE", 41, "attachment", ""),
            item("NOTE", 42, "note", "SOURCE"),
        ],
    );

    assert!(matches!(
        prepare_source_resolution_review(&report, vec![]),
        Err(SourceResolutionError::Missing(_))
    ));

    let review = prepare_source_resolution_review(
        &report,
        vec![
            resolution("SOURCE", 41, "attachment", "", 42),
            resolution("NOTE", 42, "note", "SOURCE", 42),
        ],
    )
    .expect("every pending source has one exact-snapshot resolution");

    assert_eq!(review.library_version, 42);
    assert_eq!(review.resolved_sources.len(), 2);
    assert_eq!(
        review
            .resolved_sources
            .iter()
            .map(|resolution| resolution.item_key.as_str())
            .collect::<Vec<_>>(),
        vec!["NOTE", "SOURCE"]
    );
}

#[test]
fn source_resolution_review_round_trips_owned_json() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![item("SOURCE", 41, "attachment", "")],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![resolution("SOURCE", 41, "attachment", "", 42)],
    )
    .expect("the exact pending source is resolvable");

    let serialized = serde_json::to_string(&review).expect("review must serialize");
    let decoded: conceptweave_zotero::SourceResolutionReview =
        serde_json::from_str(&serialized).expect("owned JSON must deserialize");

    assert_eq!(decoded, review);
}

#[test]
fn source_resolution_rejects_duplicate_unknown_stale_and_blank_decisions() {
    let report = classify_snapshot(
        "10.0.1".into(),
        None,
        7,
        vec![item("SOURCE", 3, "attachment", "")],
    );
    let exact = resolution("SOURCE", 3, "attachment", "", 7);

    let mut duplicate = exact.clone();
    duplicate.reason = "second decision".into();
    assert!(matches!(
        prepare_source_resolution_review(&report, vec![exact.clone(), duplicate]),
        Err(SourceResolutionError::Duplicate(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("OTHER", 3, "attachment", "", 7)]
        ),
        Err(SourceResolutionError::Unknown(key)) if key == "OTHER"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 2, "attachment", "", 7)]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "note", "", 7)]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "attachment", "PARENT", 7)]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "attachment", "", 8)]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    let mut blank = exact;
    blank.reason = " \t\n".into();
    assert!(matches!(
        prepare_source_resolution_review(&report, vec![blank]),
        Err(SourceResolutionError::BlankReason(key)) if key == "SOURCE"
    ));
}

#[test]
fn source_resolution_rejects_pending_keys_missing_from_retained_inventory() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        None,
        7,
        vec![item("SOURCE", 3, "attachment", "")],
    );
    report.unclassified_items.clear();

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "attachment", "", 7)]
        ),
        Err(SourceResolutionError::MissingInventory(key)) if key == "SOURCE"
    ));
}
