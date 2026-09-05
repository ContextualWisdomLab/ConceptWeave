use conceptweave_zotero::{
    Disposition, DuplicateMergeDecision, DuplicateReviewError, ItemData, ReviewedDuplicateMergeSet,
    ZoteroItem, build_duplicate_merge_review_manifest, classify_snapshot,
};

fn item(key: &str, version: u64, title: &str, doi: &str) -> ZoteroItem {
    ZoteroItem {
        source_record: None,
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

fn report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", 7, "Ontology Learning", "10.1000/example"),
            item(
                "B",
                9,
                "Ontology Learning Copy",
                "https://doi.org/10.1000/example",
            ),
            item("C", 3, "Shared Ontology Title", ""),
            item("D", 4, "Shared Ontology Title", ""),
        ],
    )
}

fn reviewed(report: &conceptweave_zotero::ClassificationReport) -> ReviewedDuplicateMergeSet {
    ReviewedDuplicateMergeSet {
        review_id: "review-duplicate-1".into(),
        authority_receipt: "authority-receipt-1".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        duplicate_candidates: report.duplicate_candidates.clone(),
        decisions: vec![
            DuplicateMergeDecision {
                identity_kind: "title".into(),
                normalized_identity: "shared ontology title".into(),
                retained_item_key: "C".into(),
            },
            DuplicateMergeDecision {
                identity_kind: "doi".into(),
                normalized_identity: "10.1000/example".into(),
                retained_item_key: "A".into(),
            },
        ],
    }
}

#[test]
fn reviewed_duplicate_decision_has_exact_before_after_and_rollback_mappings() {
    let report = report();
    let manifest = build_duplicate_merge_review_manifest(&report, &reviewed(&report), |_| true)
        .expect("synthetic reviewed duplicate must produce a manifest");

    assert!(manifest.source_records_preserved);
    assert_eq!(manifest.authority_receipt, "authority-receipt-1");
    assert_eq!(manifest.library_version, report.library_version);
    assert_eq!(manifest.rule_revision, report.rule_revision);
    assert_eq!(manifest.operations.len(), 2);
    assert_eq!(manifest.operations[0].identity_kind, "doi");
    assert_eq!(manifest.operations[1].identity_kind, "title");
    let operation = manifest
        .operations
        .iter()
        .find(|operation| operation.identity_kind == "doi")
        .expect("DOI duplicate operation must exist");
    assert_eq!(operation.retained_item_key, "A");
    assert_eq!(operation.source_items[0].item_version, 7);
    assert_eq!(operation.source_items[1].item_version, 9);
    assert_eq!(operation.before_canonical_keys["B"], "B");
    assert_eq!(operation.after_canonical_keys["B"], "A");
    assert_eq!(
        operation.rollback_canonical_keys,
        operation.before_canonical_keys
    );
    assert_eq!(
        report.classified_items[0].proposed_disposition,
        Disposition::Generation
    );
    let serialized = serde_json::to_value(&manifest).expect("manifest must serialize");
    assert_eq!(
        serialized["operations"][0]["rollback_canonical_keys"],
        serialized["operations"][0]["before_canonical_keys"]
    );
}

#[test]
fn duplicate_review_contract_fails_closed() {
    let report = report();
    let mut review = reviewed(&report);

    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| false),
        Err(DuplicateReviewError::UnverifiedApproval)
    );
    review.snapshot_digest = "sha256:stale".into();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::SnapshotMismatch)
    );
    review = reviewed(&report);
    review.decisions[1].retained_item_key = "missing".into();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::InvalidRetainedItem)
    );
    review = reviewed(&report);
    review.decisions[1].normalized_identity = "missing".into();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::UnknownCandidate)
    );
    review = reviewed(&report);
    review.decisions[1] = review.decisions[0].clone();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::DuplicateDecision)
    );
    review.decisions.clear();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::InvalidReview)
    );
    review = reviewed(&report);
    review.authority_receipt.clear();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::InvalidReview)
    );
    for clear_field in [
        |review: &mut ReviewedDuplicateMergeSet| review.review_id.clear(),
        |review: &mut ReviewedDuplicateMergeSet| review.snapshot_digest.clear(),
        |review: &mut ReviewedDuplicateMergeSet| review.rule_revision.clear(),
    ] {
        review = reviewed(&report);
        clear_field(&mut review);
        assert_eq!(
            build_duplicate_merge_review_manifest(&report, &review, |_| true),
            Err(DuplicateReviewError::InvalidReview)
        );
    }
    review = reviewed(&report);
    review.library_version += 1;
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::SnapshotMismatch)
    );
    review = reviewed(&report);
    review.rule_revision = "stale-rule".into();
    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &review, |_| true),
        Err(DuplicateReviewError::SnapshotMismatch)
    );

    for (error, fragment) in [
        (DuplicateReviewError::InvalidReview, "invalid"),
        (DuplicateReviewError::SnapshotMismatch, "snapshot"),
        (DuplicateReviewError::UnknownCandidate, "unknown"),
        (DuplicateReviewError::DuplicateDecision, "repeats"),
        (DuplicateReviewError::InvalidRetainedItem, "absent"),
        (DuplicateReviewError::UnverifiedApproval, "unverified"),
    ] {
        assert!(error.to_string().contains(fragment));
    }
}

#[test]
fn overlapping_duplicate_groups_require_one_consistent_canonical_choice() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("A", 7, "Same Identity", "10.1000/overlap"),
            item("B", 9, "Same Identity", "10.1000/overlap"),
        ],
    );
    assert_eq!(report.duplicate_candidates.len(), 2);

    let reviewed = ReviewedDuplicateMergeSet {
        review_id: "review-overlap".into(),
        authority_receipt: "authority-overlap".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        duplicate_candidates: report.duplicate_candidates.clone(),
        decisions: vec![
            DuplicateMergeDecision {
                identity_kind: "doi".into(),
                normalized_identity: "10.1000/overlap".into(),
                retained_item_key: "A".into(),
            },
            DuplicateMergeDecision {
                identity_kind: "title".into(),
                normalized_identity: "same identity".into(),
                retained_item_key: "B".into(),
            },
        ],
    };

    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &reviewed, |_| true),
        Err(DuplicateReviewError::InvalidReview),
        "overlapping duplicate groups cannot emit conflicting A->B and B->A canonical mappings"
    );
}

#[test]
fn duplicate_review_rejects_ambiguous_snapshot_key_revisions() {
    let report = classify_snapshot(
        "9.0.6".into(),
        None,
        42,
        vec![
            item("DUPKEY", 7, "Same Identity", "10.1000/duplicate-key"),
            item("DUPKEY", 9, "Same Identity", "10.1000/duplicate-key"),
        ],
    );
    assert_eq!(report.snapshot_items.len(), 2);
    assert!(!report.duplicate_candidates.is_empty());

    let reviewed = ReviewedDuplicateMergeSet {
        review_id: "review-duplicate-key".into(),
        authority_receipt: "authority-duplicate-key".into(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        duplicate_candidates: report.duplicate_candidates.clone(),
        decisions: report
            .duplicate_candidates
            .iter()
            .map(|candidate| DuplicateMergeDecision {
                identity_kind: candidate.identity_kind.clone(),
                normalized_identity: candidate.normalized_identity.clone(),
                retained_item_key: "DUPKEY".into(),
            })
            .collect(),
    };

    assert_eq!(
        build_duplicate_merge_review_manifest(&report, &reviewed, |_| true),
        Err(DuplicateReviewError::InvalidReview),
        "duplicate Zotero keys must fail before a BTreeMap can collapse distinct observed revisions"
    );
}

#[test]
fn duplicate_review_domain_language_and_governance_handoff_are_documented() {
    let ubiquitous_language = include_str!("../../../docs/UBIQUITOUS_LANGUAGE.md");
    for term in [
        "Reviewed Duplicate Merge Set",
        "Authority Receipt",
        "Canonical-Key Operation",
    ] {
        assert!(
            ubiquitous_language.contains(term),
            "UBIQUITOUS_LANGUAGE.md must define `{term}`"
        );
    }

    let context_map = include_str!("../../../docs/CONTEXT_MAP.md");
    assert!(
        context_map.contains("Research Intake -> Governance & Publication"),
        "CONTEXT_MAP.md must name the Research Intake to Governance & Publication verification handoff"
    );
    assert!(
        context_map.contains("Anti-Corruption Layer"),
        "the duplicate-review governance handoff must preserve an explicit ACL boundary"
    );
}
