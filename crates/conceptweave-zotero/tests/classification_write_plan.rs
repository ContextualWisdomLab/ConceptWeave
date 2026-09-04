use conceptweave_zotero::{
    Disposition, ItemData, ItemTag, ReviewedClassificationChange, ReviewedClassificationWriteSet,
    WriteMode, WritePlanError, ZoteroItem, build_classification_write_plan, classify_snapshot,
};

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
    let manual_plan = build_classification_write_plan(
        &version_ten,
        &manual_marker,
        WriteMode::DryRun,
        |_| true,
    )
    .unwrap();
    assert_eq!(manual_plan.operations[1].after_tags[0].tag_type, None);

    manual_marker.changes[0].after_tags[0].tag_type = Some(2);
    assert_eq!(
        build_classification_write_plan(
            &version_ten,
            &manual_marker,
            WriteMode::DryRun,
            |_| true
        ),
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
