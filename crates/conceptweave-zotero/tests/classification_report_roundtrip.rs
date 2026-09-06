use conceptweave_zotero::{
    ClassificationReport, Disposition, EvaluationError, GoldenSetApproval, ItemData,
    ReviewedGoldenSet, WorksheetError, ZoteroItem, build_steward_review_worksheet,
    classification_proposal_digest, classify_snapshot, evaluate_complete_reviewed_classification,
    reviewed_golden_set_from_worksheet,
};

#[test]
fn owner_only_report_roundtrip_preserves_the_review_workload() {
    let mut item = ZoteroItem {
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
    item.data.tags.push(conceptweave_zotero::ItemTag {
        tag: "ontology".into(),
        tag_type: Some(1),
    });
    let original = classify_snapshot("9.0.6".into(), None, 42, vec![item]);
    let serialized = serde_json::to_vec(&original).unwrap();

    let restored: ClassificationReport = serde_json::from_slice(&serialized).unwrap();

    assert_eq!(restored.library_version, original.library_version);
    assert_eq!(restored.rule_revision, original.rule_revision);
    assert_eq!(restored.snapshot_digest, original.snapshot_digest);
    assert_eq!(
        build_steward_review_worksheet(&restored).unwrap(),
        build_steward_review_worksheet(&original).unwrap()
    );
    assert_eq!(
        classification_proposal_digest(&restored),
        classification_proposal_digest(&original)
    );

    let mut worksheet = build_steward_review_worksheet(&original).unwrap();
    worksheet.decisions[0].reviewed_disposition = Some(Disposition::AlignmentVersioning);
    let approval = GoldenSetApproval {
        receipt_id: "synthetic-roundtrip-receipt".into(),
        reviewer_subject: "synthetic-steward".into(),
        library_version: original.library_version,
        rule_revision: original.rule_revision.clone(),
        snapshot_digest: original.snapshot_digest.clone(),
        proposal_digest: classification_proposal_digest(&original),
        snapshot_items: original.snapshot_items.clone(),
    };
    let golden = reviewed_golden_set_from_worksheet(&original, &worksheet, approval).unwrap();
    let restored_golden: ReviewedGoldenSet =
        serde_json::from_slice(&serde_json::to_vec(&golden).unwrap()).unwrap();
    assert_eq!(
        evaluate_complete_reviewed_classification(&restored, &restored_golden, |candidate| {
            candidate == &golden
        })
        .unwrap()
        .correct_count,
        1
    );

    for alter_title in [false, true] {
        let mut changed_json = serde_json::to_value(&original).unwrap();
        if alter_title {
            changed_json["classified_items"][0]["title"] = "changed after review".into();
        } else {
            changed_json["classified_items"][0]["evidence"]["field_values"] = serde_json::json!({});
        }
        let changed_report: ClassificationReport = serde_json::from_value(changed_json).unwrap();
        assert_eq!(changed_report.snapshot_digest, original.snapshot_digest);
        assert_ne!(
            build_steward_review_worksheet(&changed_report).unwrap(),
            build_steward_review_worksheet(&original).unwrap()
        );
        assert_eq!(
            reviewed_golden_set_from_worksheet(
                &changed_report,
                &worksheet,
                restored_golden.approval.clone()
            ),
            Err(EvaluationError::SnapshotMismatch)
        );
        let verifier_calls = std::cell::Cell::new(0);
        assert_eq!(
            evaluate_complete_reviewed_classification(
                &changed_report,
                &restored_golden,
                |candidate| {
                    verifier_calls.set(verifier_calls.get() + 1);
                    candidate == &golden
                }
            ),
            Err(EvaluationError::SnapshotMismatch)
        );
        assert_eq!(verifier_calls.get(), 0);
    }
}

#[test]
fn shared_report_validation_binds_every_retained_parent_coordinate() {
    for parent_key in ["A", "missing", "source", ""] {
        let items = [("A", "book", ""), ("source", "attachment", parent_key)]
            .into_iter()
            .map(|(key, item_type, parent)| {
                serde_json::from_value::<ZoteroItem>(serde_json::json!({
                    "key": key, "version": 1,
                    "data": {"itemType": item_type, "parentItem": parent, "title": "synthetic"}
                }))
                .unwrap()
            })
            .collect();
        let mut report = classify_snapshot("9.0.6".into(), None, 42, items);
        assert!(conceptweave_zotero::validate_classification_report(&report).is_ok());
        report
            .snapshot_items
            .iter_mut()
            .find(|item| item.item_key == "source")
            .unwrap()
            .parent_item_key = Some("different-parent".into());
        assert_eq!(
            conceptweave_zotero::validate_classification_report(&report),
            Err(EvaluationError::InvalidReview)
        );
    }
}

#[test]
fn pending_metadata_roundtrip_preserves_review_identity_not_raw_capture() {
    for parent_key in ["missing", "source", ""] {
        let item: ZoteroItem = serde_json::from_value(serde_json::json!({
            "key": "source", "version": 1, "unknown_provider_field": "raw-only-sentinel",
            "data": {"itemType": "attachment", "parentItem": parent_key, "title": "synthetic"}
        }))
        .unwrap();
        let original = classify_snapshot("9.0.6".into(), None, 42, vec![item]);
        let bytes = serde_json::to_vec(&original).unwrap();
        assert!(
            !String::from_utf8(bytes.clone())
                .unwrap()
                .contains("raw-only-sentinel")
        );
        let restored: ClassificationReport = serde_json::from_slice(&bytes).unwrap();
        let restored_again: ClassificationReport =
            serde_json::from_slice(&serde_json::to_vec(&restored).unwrap()).unwrap();
        assert_eq!(serde_json::to_vec(&restored_again).unwrap(), bytes);
        assert_eq!(
            classification_proposal_digest(&original),
            classification_proposal_digest(&restored_again)
        );
        assert_eq!(
            build_steward_review_worksheet(&original).unwrap(),
            build_steward_review_worksheet(&restored_again).unwrap()
        );
    }
}

#[test]
fn restored_report_rejects_child_provenance_outside_the_bound_snapshot() {
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
    let original = classify_snapshot("9.0.6".into(), None, 42, vec![item]);
    let serialized = serde_json::to_vec(&original).unwrap();
    let mut restored: ClassificationReport = serde_json::from_slice(&serialized).unwrap();

    restored.classified_items[0].child_item_keys = vec!["UNKNOWN_CHILD".into()];

    assert_eq!(
        build_steward_review_worksheet(&restored),
        Err(WorksheetError::InvalidReport)
    );

    let mut blank_parent: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    blank_parent.snapshot_items[0].parent_item_key = Some(" ".into());
    assert_eq!(
        build_steward_review_worksheet(&blank_parent),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn restored_report_rejects_classified_or_reused_child_provenance() {
    let bibliographic = |key: &str| ZoteroItem {
        source_record: None,
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: "ontology alignment".into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    };
    let child = ZoteroItem {
        source_record: None,
        key: "CHILD".into(),
        version: 3,
        data: ItemData {
            item_type: "note".into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: "PARENT_A".into(),
            collections: vec![],
            tags: vec![],
        },
    };
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("PARENT_A"), bibliographic("PARENT_B"), child],
    );
    let serialized = serde_json::to_vec(&report).unwrap();

    let mut classified_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    classified_child.classified_items[0].child_item_keys = vec!["PARENT_A".into()];
    assert_eq!(
        build_steward_review_worksheet(&classified_child),
        Err(WorksheetError::InvalidReport)
    );

    let mut reused_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    reused_child.classified_items[1].child_item_keys = vec!["CHILD".into()];
    assert_eq!(
        build_steward_review_worksheet(&reused_child),
        Err(WorksheetError::InvalidReport)
    );

    let mut omitted_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    omitted_child.classified_items[0].child_item_keys.clear();
    assert_eq!(
        build_steward_review_worksheet(&omitted_child),
        Err(WorksheetError::InvalidReport)
    );

    let mut duplicate_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    duplicate_child.classified_items[0]
        .child_item_keys
        .push("CHILD".into());
    assert_eq!(
        build_steward_review_worksheet(&duplicate_child),
        Err(WorksheetError::InvalidReport)
    );

    let mut orphaned_child: ClassificationReport = serde_json::from_slice(&serialized).unwrap();
    orphaned_child.classified_items[0].child_item_keys.clear();
    orphaned_child
        .snapshot_items
        .iter_mut()
        .find(|item| item.item_key == "CHILD")
        .unwrap()
        .parent_item_key = Some("UNCLASSIFIED_PARENT".into());
    assert_eq!(
        build_steward_review_worksheet(&orphaned_child),
        Err(WorksheetError::InvalidReport)
    );
}

#[test]
fn restored_report_accepts_snapshot_bound_nested_child_provenance() {
    let nested_item = |key: &str, item_type: &str, parent_item: &str| ZoteroItem {
        source_record: None,
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: item_type.into(),
            title: if key == "BOOK" {
                "ontology alignment"
            } else {
                ""
            }
            .into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: parent_item.into(),
            collections: vec![],
            tags: vec![],
        },
    };
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            nested_item("BOOK", "book", ""),
            nested_item("ATTACHMENT", "attachment", "BOOK"),
            nested_item("ANNOTATION", "annotation", "ATTACHMENT"),
        ],
    );

    assert!(build_steward_review_worksheet(&report).is_ok());
}
