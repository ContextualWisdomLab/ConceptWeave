use conceptweave_zotero::{
    CapturedZoteroItem, Disposition, EvaluationError, GoldenLabel, GoldenSetApproval, ItemData,
    ReviewedGoldenSet, ZoteroItem, classification_proposal_digest, classification_snapshot_digest,
    classify_captured_golden_snapshot, classify_typed_golden_snapshot,
    evaluate_reviewed_golden_set, validate_classification_report,
};
use serde_json::json;

fn bibliographic(key: &str, version: u64, title: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
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

fn child_note(key: &str, version: u64, parent_item: &str, title: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: "note".into(),
            title: title.into(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: parent_item.into(),
            collections: vec![],
            tags: vec![],
        },
    }
}

fn approval(snapshot: &conceptweave_zotero::GoldenSnapshot) -> GoldenSetApproval {
    GoldenSetApproval {
        receipt_id: "approved-review".into(),
        reviewer_subject: "synthetic-steward".into(),
        library_version: snapshot.report().library_version(),
        rule_revision: snapshot.report().rule_revision().into(),
        snapshot_digest: classification_snapshot_digest(snapshot),
        proposal_digest: classification_proposal_digest(snapshot),
        snapshot_items: snapshot.snapshot_items().to_vec(),
    }
}

#[test]
fn empty_or_unresolved_scope_is_valid_evidence_without_reviewed_papers() {
    for items in [
        vec![],
        vec![child_note("S", 0, "", "")],
        vec![child_note("C", 1, "MISSING", "")],
        vec![child_note("C", 1, "D", ""), child_note("D", 1, "C", "")],
    ] {
        let snapshot = classify_typed_golden_snapshot("10.0.1".into(), None, 42, items);
        assert_eq!(validate_classification_report(&snapshot), Ok(()));
        assert!(snapshot.report().classified_items().is_empty());
    }
}

#[test]
fn complete_snapshot_binding_includes_linked_child_revisions() {
    let snapshot = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("PARENT", 7, "ontology learning"),
            child_note("NOTE1", 3, "PARENT", "evidence"),
        ],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(&snapshot),
        labels: vec![GoldenLabel::new("PARENT", Disposition::Generation)],
    };
    assert!(evaluate_reviewed_golden_set(&snapshot, &golden, |_| true).is_ok());
    assert_eq!(snapshot.snapshot_items().len(), 2);
}

#[test]
fn duplicate_zotero_keys_fail_closed() {
    let snapshot = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("A", 1, "ontology learning"),
            bibliographic("A", 2, "ontology evaluation"),
        ],
    );
    assert_eq!(validate_classification_report(&snapshot), Err(EvaluationError::InvalidReview));
}

#[test]
fn independent_approval_must_bind_labels_not_only_receipt_coordinates() {
    let snapshot = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(&snapshot),
        labels: vec![GoldenLabel::new("A", Disposition::AlignmentVersioning)],
    };
    assert_eq!(
        evaluate_reviewed_golden_set(&snapshot, &golden, |candidate| {
            candidate.labels == vec![GoldenLabel::new("A", Disposition::Generation)]
        }),
        Err(EvaluationError::UnverifiedApproval)
    );
}

#[test]
fn proposal_digest_binds_unreviewed_proposals_current_lifecycle_and_retained_sources() {
    let first = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("A", 1, "ontology learning"),
            bibliographic("B", 1, "ontology evaluation"),
            child_note("S", 1, "", "retained source one"),
        ],
    );
    let reordered = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            child_note("S", 1, "", "retained source one"),
            bibliographic("B", 1, "ontology evaluation"),
            bibliographic("A", 1, "ontology learning"),
        ],
    );
    assert_eq!(classification_proposal_digest(&first), classification_proposal_digest(&reordered));
    let changed = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            bibliographic("A", 1, "ontology learning"),
            bibliographic("B", 1, "ontology alignment"),
            child_note("S", 1, "", "retained source two"),
        ],
    );
    assert_ne!(classification_proposal_digest(&first), classification_proposal_digest(&changed));
    let serialized = serde_json::to_value(first.report()).unwrap();
    assert_eq!(serialized["classified_items"][0]["truth_status"], "proposed");
    assert_eq!(serialized["classified_items"][0]["publication_state"], "proposed");
}

#[test]
fn old_or_locally_rewritten_receipts_do_not_authorize_changed_evidence() {
    let approved = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    let golden = ReviewedGoldenSet {
        approval: approval(&approved),
        labels: vec![GoldenLabel::new("A", Disposition::Generation)],
    };
    let changed = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology alignment")],
    );
    assert_eq!(
        evaluate_reviewed_golden_set(&changed, &golden, |_| panic!("stale evidence reached governance")),
        Err(EvaluationError::SnapshotMismatch)
    );
    let mut rewritten = golden.clone();
    rewritten.approval.snapshot_digest = classification_snapshot_digest(&changed);
    rewritten.approval.proposal_digest = classification_proposal_digest(&changed);
    assert_eq!(
        evaluate_reviewed_golden_set(&changed, &rewritten, |candidate| candidate == &golden),
        Err(EvaluationError::UnverifiedApproval)
    );
}

#[test]
fn provider_capture_binds_unknown_metadata_without_expanding_zotero_item_api() {
    let original = CapturedZoteroItem::try_from(json!({
        "key": "SYNTH001", "version": 7, "meta": {"opaque": 1},
        "data": {"itemType": "book", "title": "ontology learning"}
    }))
    .unwrap();
    let changed = CapturedZoteroItem::try_from(json!({
        "key": "SYNTH001", "version": 7, "meta": {"opaque": 2},
        "data": {"itemType": "book", "title": "ontology learning"}
    }))
    .unwrap();
    let original = classify_captured_golden_snapshot("9.0.6".into(), None, 42, vec![original]);
    let changed = classify_captured_golden_snapshot("9.0.6".into(), None, 42, vec![changed]);
    assert_ne!(classification_snapshot_digest(&original), classification_snapshot_digest(&changed));
}

#[test]
fn invalid_labels_fail_before_governance() {
    let snapshot = classify_typed_golden_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![bibliographic("A", 1, "ontology learning")],
    );
    for (labels, expected) in [
        (vec![GoldenLabel::new(" ", Disposition::Generation)], EvaluationError::InvalidReview),
        (vec![GoldenLabel::new("A", Disposition::NeedsStewardReview)], EvaluationError::InvalidExpectedDisposition),
        (vec![GoldenLabel::new("absent", Disposition::Generation)], EvaluationError::UnknownItem),
        (vec![GoldenLabel::new("A", Disposition::Generation); 2], EvaluationError::DuplicateItem),
    ] {
        let golden = ReviewedGoldenSet { approval: approval(&snapshot), labels };
        assert_eq!(
            evaluate_reviewed_golden_set(&snapshot, &golden, |_| panic!("invalid labels reached governance")),
            Err(expected)
        );
    }
}
