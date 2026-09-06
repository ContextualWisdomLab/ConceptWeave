use conceptweave_zotero::{
    Disposition, ItemData, ItemTag, ReviewedClassificationChange, ReviewedClassificationWriteSet,
    WriteMode, WritePlanError, ZoteroItem, build_classification_write_plan, classify_snapshot,
};

#[test]
fn write_scope_rejects_changed_evidence_before_authority() {
    let mut report = classification_report("10.0.1");
    let review = reviewed(&report);
    report.classified_items[0].title.push_str(" changed evidence");
    let called = std::cell::Cell::new(false);
    let result = build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| {
        called.set(true);
        true
    });
    assert_eq!(result, Err(WritePlanError::SnapshotMismatch));
    assert!(!called.get());
}

#[test]
fn write_scope_rejects_inconsistent_inventory_before_authority() {
    for mutation in ["count", "pending", "audit"] {
        let mut report = classification_report("10.0.1");
        match mutation {
            "count" => report.observed_item_count += 1,
            "pending" => report.pending_source_item_keys.push("absent".into()),
            "audit" => report.audit_summary.failure_count += 1,
            _ => unreachable!(),
        }
        let review = reviewed(&report);
        let called = std::cell::Cell::new(false);
        let result = build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| {
            called.set(true);
            true
        });
        assert_eq!(result, Err(WritePlanError::InvalidReview), "{mutation}");
        assert!(!called.get(), "{mutation}");
    }
}

fn tag(name: &str, tag_type: Option<u64>) -> ItemTag {
    ItemTag {
        tag: name.into(),
        tag_type,
    }
}

fn classification_report(version: &str) -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        version.into(),
        Some("server-1".into()),
        42,
        vec![
            ZoteroItem {
                source_record: None,
                key: "B".into(),
                version: 9,
                data: ItemData {
                    item_type: "book".into(),
                    title: "ontology evaluation".into(),
                    abstract_note: String::new(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec!["source_collection".into()],
                    tags: vec![tag("Imported", Some(1))],
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
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec![],
                    tags: vec![],
                },
            },
        ],
    )
}

fn reviewed(report: &conceptweave_zotero::ClassificationReport) -> ReviewedClassificationWriteSet {
    ReviewedClassificationWriteSet {
        review_id: "review-1".into(),
        authority_receipt: "authority-1".into(),
        server_id: report.server_id.clone(),
        zotero_version: report.zotero_version.clone(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
        snapshot_items: report.snapshot_items.clone(),
        changes: vec![
            ReviewedClassificationChange {
                item_key: "B".into(),
                item_version: 9,
                reviewed_disposition: Disposition::EvaluationGovernance,
                before_collection_keys: vec!["source_collection".into()],
                after_collection_keys: vec!["evaluation_collection".into()],
                before_tags: vec![tag("Imported", Some(1))],
                after_tags: vec![tag("Evaluation", None), tag("Imported", Some(1))],
            },
            ReviewedClassificationChange {
                item_key: "A".into(),
                item_version: 7,
                reviewed_disposition: Disposition::Generation,
                before_collection_keys: vec![],
                after_collection_keys: vec!["generation_collection".into()],
                before_tags: vec![],
                after_tags: vec![tag("Generation", None)],
            },
        ],
    }
}

#[test]
fn dry_run_is_default_and_preserves_exact_rollback_state() {
    assert_eq!(WriteMode::default(), WriteMode::DryRun);
    let report = classification_report("9.0.6");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::default(), |_| {
            true
        })
        .expect("reviewed dry-run changes must produce a plan");

    assert_eq!(plan.mode, WriteMode::DryRun);
    assert_eq!(plan.operations[0].item_key, "A");
    assert_eq!(plan.operations[1].item_key, "B");
    assert_eq!(
        plan.operations[1].rollback_collection_keys,
        plan.operations[1].before_collection_keys
    );
    assert_eq!(
        plan.operations[1].rollback_tags,
        vec![tag("Imported", Some(1))]
    );
    assert!(plan.source_records_preserved);
}

#[test]
fn write_plan_fails_closed_for_untrusted_stale_or_unsafe_changes() {
    let report = classification_report("9.0.6");
    assert_eq!(
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true),
        Err(WritePlanError::UnsupportedExecute)
    );
    assert_eq!(
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::DryRun, |_| false),
        Err(WritePlanError::UnverifiedApproval)
    );

    let mut review = reviewed(&report);
    review.snapshot_digest = "sha256:stale".into();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::SnapshotMismatch)
    );
    review = reviewed(&report);
    review.changes[0].item_version += 1;
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::StaleItem)
    );
    review = reviewed(&report);
    review.changes[0].before_tags.clear();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::StaleItem)
    );
    review = reviewed(&report);
    review.changes[0].after_tags = review.changes[0].before_tags.clone();
    review.changes[0].after_collection_keys = review.changes[0].before_collection_keys.clone();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::NoChange)
    );
    review = reviewed(&report);
    review.changes[0].reviewed_disposition = Disposition::NeedsStewardReview;
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::UnreviewedDisposition)
    );
    review = reviewed(&report);
    review.changes[1] = review.changes[0].clone();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::DuplicateItem)
    );
    review = reviewed(&report);
    review.changes[0].item_key = "missing".into();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::UnknownItem)
    );
    review = reviewed(&report);
    review.changes[0].after_tags.push(tag(" ", None));
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::InvalidMetadata)
    );

    for invalidate in [
        |review: &mut ReviewedClassificationWriteSet| review.review_id.clear(),
        |review: &mut ReviewedClassificationWriteSet| review.authority_receipt.clear(),
        |review: &mut ReviewedClassificationWriteSet| review.rule_revision.clear(),
        |review: &mut ReviewedClassificationWriteSet| review.snapshot_digest.clear(),
        |review: &mut ReviewedClassificationWriteSet| review.changes.clear(),
    ] {
        review = reviewed(&report);
        invalidate(&mut review);
        assert_eq!(
            build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
            Err(WritePlanError::InvalidReview)
        );
    }
    for make_stale in [
        |review: &mut ReviewedClassificationWriteSet| review.server_id = Some("other".into()),
        |review: &mut ReviewedClassificationWriteSet| review.library_version += 1,
        |review: &mut ReviewedClassificationWriteSet| review.rule_revision = "other".into(),
        |review: &mut ReviewedClassificationWriteSet| {
            review.snapshot_digest = "sha256:other".into()
        },
    ] {
        review = reviewed(&report);
        make_stale(&mut review);
        assert_eq!(
            build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
            Err(WritePlanError::SnapshotMismatch)
        );
    }
    review = reviewed(&report);
    review.changes[0].item_key.clear();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::InvalidReview)
    );
    review = reviewed(&report);
    review.changes[0].before_collection_keys.clear();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::StaleItem)
    );
    for invalidate_metadata in [
        |change: &mut ReviewedClassificationChange| change.after_collection_keys.push(" ".into()),
        |change: &mut ReviewedClassificationChange| {
            change
                .after_collection_keys
                .push(change.after_collection_keys[0].clone())
        },
        |change: &mut ReviewedClassificationChange| {
            change.after_tags.push(change.after_tags[0].clone())
        },
    ] {
        review = reviewed(&report);
        invalidate_metadata(&mut review.changes[0]);
        assert_eq!(
            build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
            Err(WritePlanError::InvalidMetadata)
        );
    }

    review = reviewed(&report);
    review.changes[0].before_tags.push(tag("Imported", None));
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true),
        Err(WritePlanError::InvalidMetadata)
    );
    let mut invalid_report = classification_report("9.0.6");
    invalid_report.classified_items[0].tags.push(tag(" ", None));
    assert_eq!(
        build_classification_write_plan(
            &invalid_report,
            &reviewed(&invalid_report),
            WriteMode::DryRun,
            |_| true
        ),
        Err(WritePlanError::InvalidMetadata)
    );

    review = reviewed(&report);
    review.changes[0].after_collection_keys = review.changes[0].before_collection_keys.clone();
    assert!(build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| true).is_ok());

    let version_ten = classification_report("10.0.0");
    assert!(
        build_classification_write_plan(
            &version_ten,
            &reviewed(&version_ten),
            WriteMode::Execute,
            |_| true
        )
        .is_ok()
    );
    let no_server = classify_snapshot(
        "10.0.0".into(),
        None,
        42,
        vec![ZoteroItem {
            source_record: None,
            key: "A".into(),
            version: 7,
            data: ItemData {
                item_type: "journalArticle".into(),
                title: "ontology learning".into(),
                abstract_note: String::new(),
                doi: String::new(),
                parent_item: String::new(),
                collections: vec![],
                tags: vec![],
            },
        }],
    );
    let mut no_server_review = ReviewedClassificationWriteSet {
        review_id: "review".into(),
        authority_receipt: "authority".into(),
        server_id: None,
        zotero_version: no_server.zotero_version.clone(),
        library_version: no_server.library_version,
        rule_revision: no_server.rule_revision.into(),
        snapshot_digest: no_server.snapshot_digest.clone(),
        snapshot_items: no_server.snapshot_items.clone(),
        changes: vec![ReviewedClassificationChange {
            item_key: "A".into(),
            item_version: 7,
            reviewed_disposition: Disposition::Generation,
            before_collection_keys: vec![],
            after_collection_keys: vec!["generation_collection".into()],
            before_tags: vec![],
            after_tags: vec![],
        }],
    };
    assert_eq!(
        build_classification_write_plan(&no_server, &no_server_review, WriteMode::Execute, |_| {
            true
        }),
        Err(WritePlanError::MissingServerIdentity)
    );
    no_server_review.server_id = Some(" ".into());
    let mut blank_server_report = no_server;
    blank_server_report.server_id = Some(" ".into());
    assert_eq!(
        build_classification_write_plan(
            &blank_server_report,
            &no_server_review,
            WriteMode::Execute,
            |_| true
        ),
        Err(WritePlanError::MissingServerIdentity)
    );

    let mut duplicate_report = classification_report("10.0.0");
    duplicate_report.classified_items[1].item_key = "A".into();
    assert_eq!(
        build_classification_write_plan(
            &duplicate_report,
            &reviewed(&duplicate_report),
            WriteMode::DryRun,
            |_| true
        ),
        Err(WritePlanError::InvalidReview)
    );
    let malformed_version = classification_report("unknown");
    assert_eq!(
        build_classification_write_plan(
            &malformed_version,
            &reviewed(&malformed_version),
            WriteMode::Execute,
            |_| true
        ),
        Err(WritePlanError::UnsupportedExecute)
    );

    let mut changed_version = classification_report("9.0.6");
    let changed_version_review = reviewed(&changed_version);
    changed_version.zotero_version = "10.0.0".into();
    assert_eq!(
        build_classification_write_plan(
            &changed_version,
            &changed_version_review,
            WriteMode::Execute,
            |_| true
        ),
        Err(WritePlanError::SnapshotMismatch)
    );

    let exact_report = classification_report("10.0.0");
    let mut changed_snapshot_review = reviewed(&exact_report);
    changed_snapshot_review.snapshot_items[0].item_version += 1;
    assert_eq!(
        build_classification_write_plan(
            &exact_report,
            &changed_snapshot_review,
            WriteMode::DryRun,
            |_| true
        ),
        Err(WritePlanError::SnapshotMismatch)
    );

    let mut blank_snapshot_key = classification_report("10.0.0");
    blank_snapshot_key.snapshot_items[0].item_key = " ".into();
    assert_eq!(
        build_classification_write_plan(
            &blank_snapshot_key,
            &reviewed(&blank_snapshot_key),
            WriteMode::DryRun,
            |_| true
        ),
        Err(WritePlanError::InvalidReview)
    );

    let mut duplicate_snapshot = classification_report("10.0.0");
    duplicate_snapshot
        .snapshot_items
        .push(duplicate_snapshot.snapshot_items[0].clone());
    assert_eq!(
        build_classification_write_plan(
            &duplicate_snapshot,
            &reviewed(&duplicate_snapshot),
            WriteMode::DryRun,
            |_| true
        ),
        Err(WritePlanError::InvalidReview)
    );

    let mut detached_item = classification_report("10.0.0");
    detached_item.classified_items[0].item_version += 1;
    assert_eq!(
        build_classification_write_plan(
            &detached_item,
            &reviewed(&detached_item),
            WriteMode::DryRun,
            |_| true
        ),
        Err(WritePlanError::StaleItem)
    );

    let mut manual_marker = reviewed(&version_ten);
    manual_marker.changes[0].after_tags[0].tag_type = Some(0);
    let manual_plan =
        build_classification_write_plan(&version_ten, &manual_marker, WriteMode::DryRun, |_| true)
            .unwrap();
    assert_eq!(manual_plan.operations[1].after_tags[0].tag_type, None);

    manual_marker.changes[0].after_tags[0].tag_type = Some(2);
    assert_eq!(
        build_classification_write_plan(&version_ten, &manual_marker, WriteMode::DryRun, |_| true),
        Err(WritePlanError::InvalidMetadata)
    );

    for (error, fragment) in [
        (WritePlanError::InvalidReview, "invalid"),
        (WritePlanError::SnapshotMismatch, "snapshot"),
        (WritePlanError::UnverifiedApproval, "unverified"),
        (WritePlanError::UnknownItem, "unknown"),
        (WritePlanError::DuplicateItem, "repeats"),
        (WritePlanError::StaleItem, "stale"),
        (WritePlanError::InvalidMetadata, "duplicated"),
        (WritePlanError::NoChange, "no metadata"),
        (WritePlanError::UnreviewedDisposition, "cannot be written"),
        (WritePlanError::UnsupportedExecute, "does not support"),
        (WritePlanError::MissingServerIdentity, "server identity"),
    ] {
        assert!(error.to_string().contains(fragment));
    }
}

/// A governance verifier may consume a one-use receipt; local rejection must not call it.
fn assert_rejected_before_verification(
    report: &conceptweave_zotero::ClassificationReport,
    review: &ReviewedClassificationWriteSet,
    mode: WriteMode,
    expected: WritePlanError,
) {
    for verifier_result in [true, false] {
        let calls = std::cell::Cell::new(0);
        let result = build_classification_write_plan(report, review, mode, |_| {
            calls.set(calls.get() + 1);
            verifier_result
        });
        assert_eq!(calls.get(), 0, "local {expected:?} must preserve approval");
        assert_eq!(result, Err(expected));
    }
}

#[test]
fn write_verifier_waits_for_every_item_change_to_be_valid() {
    let report = classification_report("10.0.0");
    for expected in [
        WritePlanError::UnknownItem,
        WritePlanError::StaleItem,
        WritePlanError::DuplicateItem,
        WritePlanError::UnreviewedDisposition,
        WritePlanError::NoChange,
        WritePlanError::InvalidReview,
    ] {
        let mut review = reviewed(&report);
        // Reject the second operation after the first has passed local validation.
        match expected {
            WritePlanError::UnknownItem => review.changes[1].item_key = "missing".into(),
            WritePlanError::StaleItem => review.changes[1].item_version += 1,
            WritePlanError::DuplicateItem => review.changes[1] = review.changes[0].clone(),
            WritePlanError::UnreviewedDisposition => {
                review.changes[1].reviewed_disposition = Disposition::NeedsStewardReview;
            }
            WritePlanError::NoChange => {
                review.changes[1].after_collection_keys =
                    review.changes[1].before_collection_keys.clone();
                review.changes[1].after_tags = review.changes[1].before_tags.clone();
            }
            WritePlanError::InvalidReview => review.changes[1].item_key.clear(),
            _ => unreachable!("only enumerated local item failures are tested"),
        }
        assert_rejected_before_verification(&report, &review, WriteMode::DryRun, expected);
    }
}

#[test]
fn write_verifier_waits_for_metadata_normalization_and_before_state_checks() {
    for location in [
        "actual",
        "before",
        "after",
        "tag_type",
        "duplicate",
        "stale",
    ] {
        let mut report = classification_report("10.0.0");
        let mut review = reviewed(&report);
        let mut expected = WritePlanError::InvalidMetadata;
        match location {
            "actual" => report.classified_items[0].tags.push(tag(" ", None)),
            "before" => review.changes[1].before_collection_keys.push(" ".into()),
            "after" => review.changes[1].after_tags.push(tag(" ", None)),
            "tag_type" => review.changes[1].after_tags[0].tag_type = Some(2),
            "duplicate" => {
                let collection = review.changes[1].after_collection_keys[0].clone();
                review.changes[1].after_collection_keys.push(collection);
            }
            "stale" => {
                review.changes[1].before_tags.push(tag("unobserved", None));
                expected = WritePlanError::StaleItem;
            }
            _ => unreachable!("only enumerated metadata locations are tested"),
        }
        assert_rejected_before_verification(&report, &review, WriteMode::DryRun, expected);
    }
}

#[test]
fn write_verifier_waits_for_report_membership_and_review_identity_checks() {
    for location in ["blank", "duplicate", "classified", "detached"] {
        let mut report = classification_report("10.0.0");
        let mut expected = WritePlanError::InvalidReview;
        match location {
            "blank" => report.snapshot_items[0].item_key.clear(),
            "duplicate" => report.snapshot_items.push(report.snapshot_items[0].clone()),
            "classified" => report.classified_items[1].item_key = "A".into(),
            "detached" => {
                report.classified_items[0].item_version += 1;
                expected = WritePlanError::StaleItem;
            }
            _ => unreachable!("only enumerated report failures are tested"),
        }
        let review = reviewed(&report);
        assert_rejected_before_verification(&report, &review, WriteMode::DryRun, expected);
    }
    let report = classification_report("10.0.0");
    let mut review = reviewed(&report);
    review.review_id.clear();
    assert_rejected_before_verification(
        &report,
        &review,
        WriteMode::DryRun,
        WritePlanError::InvalidReview,
    );
    let mut review = reviewed(&report);
    review.library_version += 1;
    assert_rejected_before_verification(
        &report,
        &review,
        WriteMode::DryRun,
        WritePlanError::SnapshotMismatch,
    );
}

#[test]
fn write_verifier_waits_for_supported_execute_controls() {
    for version in ["9.0.6", "unknown"] {
        let report = classification_report(version);
        assert_rejected_before_verification(
            &report,
            &reviewed(&report),
            WriteMode::Execute,
            WritePlanError::UnsupportedExecute,
        );
    }
    for server_id in [None, Some(" ".to_owned())] {
        let mut report = classification_report("10.0.0");
        report.server_id = server_id;
        assert_rejected_before_verification(
            &report,
            &reviewed(&report),
            WriteMode::Execute,
            WritePlanError::MissingServerIdentity,
        );
    }
}

#[test]
fn write_verifier_receives_valid_complete_review_once_and_preserves_plan_behavior() {
    let report = classification_report("10.0.0");
    let review = reviewed(&report);
    for mode in [WriteMode::DryRun, WriteMode::Execute] {
        for verified in [false, true] {
            let calls = std::cell::Cell::new(0);
            let result = build_classification_write_plan(&report, &review, mode, |received| {
                calls.set(calls.get() + 1);
                assert_eq!(received, &review);
                verified
            });
            assert_eq!(calls.get(), 1);
            if !verified {
                assert_eq!(result, Err(WritePlanError::UnverifiedApproval));
                continue;
            }
            let plan = serde_json::to_value(result.unwrap()).unwrap();
            assert_eq!(plan["mode"], serde_json::to_value(mode).unwrap());
            assert_eq!(plan["review_id"], review.review_id);
            assert_eq!(plan["authority_receipt"], review.authority_receipt);
            assert_eq!(plan["source_records_preserved"], true);
            assert_eq!(plan["operations"][0]["item_key"], "A");
            assert_eq!(plan["operations"][1]["item_key"], "B");
            for operation in plan["operations"].as_array().unwrap() {
                assert_eq!(
                    operation["rollback_collection_keys"],
                    operation["before_collection_keys"]
                );
                assert_eq!(operation["rollback_tags"], operation["before_tags"]);
            }
        }
    }
}
