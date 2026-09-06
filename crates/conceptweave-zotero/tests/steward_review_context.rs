use conceptweave_zotero::{
    Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData, ReviewedGoldenSet,
    ZoteroItem, classification_proposal_digest, classify_snapshot, evaluate_reviewed_golden_set,
};

fn item(key: &str, title: &str, abstract_note: &str) -> ZoteroItem {
    ZoteroItem {
        source_record: None,
        key: key.into(),
        version: 7,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: title.into(),
            abstract_note: abstract_note.into(),
            doi: String::new(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

#[test]
fn abstentions_retain_only_the_abstract_needed_for_local_steward_review() {
    let review_abstract = "A domain-specific vocabulary outside the deterministic rules.";
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("REVIEW01", "Unmatched domain study", review_abstract),
            item(
                "DECIDED1",
                "Ontology learning",
                "This abstract must not be copied into review-only context.",
            ),
            item("EMPTY001", "Unmatched title", ""),
            item("SPACE001", "Unmatched title", " \n\t "),
            item("CJK00001", "Unmatched title", "地域固有の語彙を調査する。"),
            item(
                "CONFLICT",
                "Unmatched title",
                "Ontology learning and ontology matching are compared.",
            ),
        ],
    );

    let review = report
        .classified_items
        .iter()
        .find(|item| item.item_key == "REVIEW01")
        .unwrap();
    assert_eq!(review.proposed_disposition, Disposition::NeedsStewardReview);
    assert_eq!(
        review.review_abstract_note.as_deref(),
        Some(review_abstract)
    );

    let decided = report
        .classified_items
        .iter()
        .find(|item| item.item_key == "DECIDED1")
        .unwrap();
    assert_eq!(decided.proposed_disposition, Disposition::Generation);
    assert!(decided.review_abstract_note.is_none());

    let empty = report
        .classified_items
        .iter()
        .find(|item| item.item_key == "EMPTY001")
        .unwrap();
    assert_eq!(empty.proposed_disposition, Disposition::NeedsStewardReview);
    assert!(empty.review_abstract_note.is_none());

    let whitespace = report
        .classified_items
        .iter()
        .find(|item| item.item_key == "SPACE001")
        .unwrap();
    assert!(whitespace.review_abstract_note.is_none());
    assert!(
        serde_json::to_value(whitespace)
            .unwrap()
            .get("review_abstract_note")
            .is_none()
    );
    let non_english = report
        .classified_items
        .iter()
        .find(|item| item.item_key == "CJK00001")
        .unwrap();
    assert_eq!(
        non_english.proposed_disposition,
        Disposition::NeedsStewardReview
    );
    assert_eq!(
        non_english.review_abstract_note.as_deref(),
        Some("地域固有の語彙を調査する。")
    );

    let conflict = report
        .classified_items
        .iter()
        .find(|item| item.item_key == "CONFLICT")
        .unwrap();
    assert_eq!(
        conflict.proposed_disposition,
        Disposition::NeedsStewardReview
    );
    assert!(conflict.evidence.field_values.contains_key("abstract_note"));
    assert!(conflict.review_abstract_note.is_none());
    let serialized = serde_json::to_string(conflict).unwrap();
    assert_eq!(serialized.matches("Ontology learning").count(), 1);
}

#[test]
fn changed_review_context_invalidates_prior_approval_before_verification() {
    for (original, replacement) in [
        (
            "Unmatched original abstract.",
            Some("Changed review context."),
        ),
        ("Unmatched original abstract.", None),
        ("", Some("Added review context.")),
    ] {
        let mut report = classify_snapshot(
            "10.0.1".into(),
            Some("synthetic-server".into()),
            42,
            vec![item("REVIEW01", "Unmatched domain study", original)],
        );
        let approved = ReviewedGoldenSet {
            approval: GoldenSetApproval {
                receipt_id: "synthetic-receipt".into(),
                reviewer_subject: "synthetic-steward".into(),
                library_version: report.library_version,
                rule_revision: report.rule_revision.into(),
                snapshot_digest: report.snapshot_digest.clone(),
                proposal_digest: classification_proposal_digest(&report),
                snapshot_items: report.snapshot_items.clone(),
            },
            labels: vec![GoldenLabel::new("REVIEW01", Disposition::Generation)],
        };
        let result =
            evaluate_reviewed_golden_set(&report, &approved, |set| set == &approved).unwrap();
        let aggregate = serde_json::to_string(&result).unwrap();
        for private_value in ["REVIEW01", "synthetic-steward", "Unmatched domain study"] {
            assert!(!aggregate.contains(private_value));
        }
        if !original.is_empty() {
            assert!(!aggregate.contains(original));
        }
        report.classified_items[0].review_abstract_note = replacement.map(str::to_owned);
        assert_ne!(
            classification_proposal_digest(&report),
            approved.approval.proposal_digest
        );
        assert_eq!(
            evaluate_reviewed_golden_set(&report, &approved, |_| {
                panic!("changed review context must fail before governance")
            }),
            Err(EvaluationError::SnapshotMismatch)
        );
        let mut rewritten = approved.clone();
        rewritten.approval.proposal_digest = classification_proposal_digest(&report);
        assert_eq!(
            evaluate_reviewed_golden_set(&report, &rewritten, |set| set == &approved),
            Err(EvaluationError::UnverifiedApproval)
        );
    }
}
