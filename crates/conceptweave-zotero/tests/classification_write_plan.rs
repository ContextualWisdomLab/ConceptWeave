use conceptweave_zotero::{
    ClassificationItemState, ClassificationWriteOutcome, Disposition, ItemData, ItemTag,
    ReviewedClassificationChange, ReviewedClassificationWriteSet, WriteMode, WritePlanError,
    ZoteroItem, build_classification_write_plan, classify_snapshot,
    execute_classification_write_plan,
};

fn tag(name: &str, tag_type: Option<u64>) -> ItemTag {
    ItemTag {
        tag: name.into(),
        tag_type,
    }
}

#[test]
fn execution_preflights_every_item_and_returns_reversible_partial_failure() {
    let report = classification_report("10.0.0");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true)
            .unwrap();
    let mut preflighted = Vec::new();
    let mut written = Vec::new();
    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| {
            preflighted.push(item_key.to_owned());
            let operation = plan
                .operations
                .iter()
                .find(|operation| operation.item_key == item_key)
                .unwrap();
            Ok::<_, ()>(ClassificationItemState {
                server_id: "server-1".into(),
                library_version: 42,
                item_key: item_key.into(),
                item_version: operation.item_version,
                collection_keys: operation.before_collection_keys.clone(),
                tags: operation.before_tags.clone(),
            })
        },
        |request| {
            written.push(request.item_key.clone());
            if request.item_key == "B" {
                return Err(());
            }
            Ok(ClassificationItemState {
                server_id: request.server_id.clone(),
                library_version: request.library_version + 1,
                item_key: request.item_key.clone(),
                item_version: request.item_version + 1,
                collection_keys: request.collection_keys.clone(),
                tags: request.tags.clone(),
            })
        },
    );

    assert_eq!(preflighted, ["A", "B"]);
    assert_eq!(written, ["A", "B"]);
    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
    assert_eq!(receipt.failed_item_key.as_deref(), Some("B"));
    assert!(receipt.not_attempted_item_keys.is_empty());
    assert_eq!(receipt.applied_item_keys, ["A"]);
    assert_eq!(receipt.rollback_operations.len(), 1);
    assert_eq!(receipt.rollback_operations[0].item_key, "A");
    assert_eq!(receipt.rollback_operations[0].item_version, 8);
    assert_eq!(
        receipt.rollback_operations[0].collection_keys,
        Vec::<String>::new()
    );
    assert!(receipt.rollback_operations[0].tags.is_empty());
}

#[test]
fn dry_run_execution_never_calls_the_write_boundary() {
    let report = classification_report("10.0.0");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::DryRun, |_| true)
            .unwrap();
    let receipt = execute_classification_write_plan(
        &plan,
        |_| -> Result<ClassificationItemState, ()> { panic!("dry-run must not preflight") },
        |_| -> Result<ClassificationItemState, ()> { panic!("dry-run must not write") },
    );

    assert_eq!(receipt.outcome, ClassificationWriteOutcome::DryRun);
    assert!(receipt.applied_item_keys.is_empty());
    assert!(receipt.rollback_operations.is_empty());
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
        library_version: report.library_version,
        rule_revision: report.rule_revision.into(),
        snapshot_digest: report.snapshot_digest.clone(),
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
        library_version: no_server.library_version,
        rule_revision: no_server.rule_revision.into(),
        snapshot_digest: no_server.snapshot_digest.clone(),
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
