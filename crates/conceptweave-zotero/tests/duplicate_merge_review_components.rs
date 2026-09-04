use conceptweave_zotero::{
    DuplicateMergeDecision, DuplicateReviewError, ItemData, ReviewedDuplicateMergeSet, ZoteroItem,
    build_duplicate_merge_review_manifest, classify_snapshot,
};

fn item(key: &str, version: u64, title: &str, doi: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: "journalArticle".into(),
            title: title.into(),
            abstract_note: String::new(),
            doi: doi.into(),
            parent_item: String::new(),
            collections: vec![],
            tags: vec![],
        },
    }
}

#[test]
fn transitive_duplicate_component_accepts_one_component_level_canonical_key() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", 1, "Alpha", "10.1000/ab"),
            item("B", 2, "Bridge", "10.1000/ab"),
            item("C", 3, "Bridge", "10.1000/cd"),
            item("D", 4, "Delta", "10.1000/cd"),
        ],
    );
    assert_eq!(report.duplicate_candidates.len(), 3);

    let reviewed = ReviewedDuplicateMergeSet {
        review_id: "review-transitive-component".into(),
        authority_receipt: "authority-transitive-component".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        decisions: report
            .duplicate_candidates
            .iter()
            .map(|candidate| DuplicateMergeDecision {
                identity_kind: candidate.identity_kind.into(),
                normalized_identity: candidate.normalized_identity.clone(),
                retained_item_key: "A".into(),
            })
            .collect(),
    };

    let manifest = build_duplicate_merge_review_manifest(&report, &reviewed, |_| true).expect(
        "one steward-selected canonical key must be valid across a transitive duplicate component",
    );

    assert_eq!(manifest.operations.len(), 3);
    for operation in manifest.operations {
        assert_eq!(operation.retained_item_key, "A");
        assert_eq!(operation.source_items.len(), 4);
        assert!(
            operation
                .source_items
                .iter()
                .any(|item| item.item_key == "A")
        );
        assert!(
            operation
                .after_canonical_keys
                .values()
                .all(|canonical_key| canonical_key == "A")
        );
    }
}

#[test]
fn duplicate_review_rejects_blank_snapshot_item_identity() {
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("", 1, "Blank identity", "10.1000/blank-key"),
            item("B", 2, "Other identity", "10.1000/blank-key"),
        ],
    );
    assert_eq!(report.duplicate_candidates.len(), 1);

    let candidate = &report.duplicate_candidates[0];
    let reviewed = ReviewedDuplicateMergeSet {
        review_id: "review-blank-key".into(),
        authority_receipt: "authority-blank-key".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        decisions: vec![DuplicateMergeDecision {
            identity_kind: candidate.identity_kind.into(),
            normalized_identity: candidate.normalized_identity.clone(),
            retained_item_key: "B".into(),
        }],
    };

    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &reviewed, |_| true),
        Err(DuplicateReviewError::InvalidReview),
        "blank Zotero keys are not stable provenance identities and must fail closed before manifest materialization"
    );

    report.snapshot_items.remove(0);
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &reviewed, |_| true),
        Err(DuplicateReviewError::SnapshotMismatch),
        "every duplicate candidate must remain bound to its reviewed snapshot revision"
    );
}

#[test]
fn duplicate_review_binds_every_item_revision_to_the_reviewed_snapshot() {
    let mut report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", 1, "Same title", "10.1000/same"),
            item("B", 2, "Same title", "10.1000/same"),
        ],
    );
    let reviewed = ReviewedDuplicateMergeSet {
        review_id: "review-revisions".into(),
        authority_receipt: "authority-revisions".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        decisions: report
            .duplicate_candidates
            .iter()
            .map(|candidate| DuplicateMergeDecision {
                identity_kind: candidate.identity_kind.into(),
                normalized_identity: candidate.normalized_identity.clone(),
                retained_item_key: "A".into(),
            })
            .collect(),
    };
    report.snapshot_items[0].item_version += 1;

    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &reviewed, |_| true),
        Err(DuplicateReviewError::SnapshotMismatch)
    );
}
