use conceptweave_zotero::{
    ClassificationItemState, ClassificationWriteOutcome, Disposition, ItemData, ItemTag,
    ReviewedClassificationChange, ReviewedClassificationWriteSet, WriteMode, WritePlanError,
    ZoteroItem, build_classification_write_plan, classification_proposal_digest, classify_snapshot,
    execute_classification_write_plan,
};

#[test]
fn write_scope_rejects_changed_evidence_before_authority() {
    let mut report = classification_report("10.0.1");
    let review = reviewed(&report);
    report.classified_items[0]
        .title
        .push_str(" changed evidence");
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

#[test]
fn write_scope_requires_binding_and_independent_approval() {
    let mut report = classification_report("10.0.1");
    let approved = reviewed(&report);
    let mut serialized = serde_json::to_value(&approved).unwrap();
    serialized
        .as_object_mut()
        .unwrap()
        .remove("proposal_digest");
    assert!(serde_json::from_value::<ReviewedClassificationWriteSet>(serialized).is_err());

    let mut review = approved.clone();
    review.proposal_digest = " ".into();
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |_| {
            panic!("blank binding must not reach authority")
        }),
        Err(WritePlanError::InvalidReview)
    );
    let plan = build_classification_write_plan(&report, &approved, WriteMode::DryRun, |set| {
        set == &approved
    })
    .unwrap();
    assert_eq!(
        serde_json::to_value(&plan).unwrap()["proposal_digest"],
        approved.proposal_digest
    );

    report.classified_items[0]
        .title
        .push_str(" changed evidence");
    review = approved.clone();
    review.proposal_digest = classification_proposal_digest(&report);
    let calls = std::cell::Cell::new(0);
    assert_eq!(
        build_classification_write_plan(&report, &review, WriteMode::DryRun, |set| {
            calls.set(calls.get() + 1);
            set == &approved
        }),
        Err(WritePlanError::UnverifiedApproval)
    );
    assert_eq!(calls.get(), 1);
}

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
                .operations()
                .iter()
                .find(|operation| operation.item_key == item_key)
                .unwrap();
            Ok::<_, ()>(ClassificationItemState {
                server_id: "server-1".into(),
                library_version: if preflighted.len() > plan.operations().len() {
                    43
                } else {
                    42
                },
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

    assert_eq!(preflighted, ["A", "B", "B"]);
    assert_eq!(written, ["A", "B"]);
    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
    assert_eq!(receipt.failed_item_key.as_deref(), Some("B"));
    assert_eq!(receipt.indeterminate_item_key.as_deref(), Some("B"));
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
    assert_eq!(receipt.indeterminate_item_key, None);
    assert!(receipt.rollback_operations.is_empty());
}

#[test]
fn matching_observation_does_not_prove_a_lost_write_completed() {
    let report = classification_report("10.0.0");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true)
            .unwrap();
    let mut b_reads = 0;
    let mut submitted_requests = Vec::new();
    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| {
            if item_key == "B" {
                b_reads += 1;
                if b_reads == 2 {
                    let operation = &plan.operations()[1];
                    return Ok::<_, ()>(ClassificationItemState {
                        server_id: "server-1".into(),
                        library_version: 44,
                        item_key: "B".into(),
                        item_version: 10,
                        collection_keys: operation.after_collection_keys.clone(),
                        tags: operation.after_tags.clone(),
                    });
                }
            }
            Ok::<_, ()>(preflight_state(&plan, item_key))
        },
        |request| {
            submitted_requests.push(request.clone());
            if request.item_key == "B" {
                Err(())
            } else {
                Ok(applied_state(request))
            }
        },
    );

    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
    assert_eq!(receipt.failed_item_key.as_deref(), Some("B"));
    assert_eq!(receipt.indeterminate_item_key.as_deref(), Some("B"));
    assert_eq!(receipt.applied_item_keys, ["A"]);
    assert_eq!(receipt.rollback_operations.len(), 1);
    assert_eq!(receipt.rollback_operations[0].item_key, "A");
    assert_eq!(receipt.rollback_operations[0].item_version, 8);
    let audit = serde_json::to_value(&receipt).unwrap();
    assert_eq!(
        receipt.indeterminate_request.as_ref(),
        submitted_requests.last()
    );
    assert_eq!(audit["indeterminate_request"]["item_key"], "B");
    assert_eq!(audit["indeterminate_request"]["library_version"], 43);
    assert_eq!(audit["indeterminate_request"]["item_version"], 9);
    assert_eq!(audit["reconciliation_observation"]["item_version"], 10);
}

#[test]
fn execution_names_an_item_when_failed_write_reconciliation_is_unavailable() {
    let report = classification_report("10.0.0");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true)
            .unwrap();
    let mut reads = 0;
    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| {
            reads += 1;
            if reads > plan.operations().len() {
                Err(())
            } else {
                Ok(preflight_state(&plan, item_key))
            }
        },
        |_| Err::<ClassificationItemState, _>(()),
    );

    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
    assert_eq!(receipt.failed_item_key.as_deref(), Some("A"));
    assert_eq!(receipt.indeterminate_item_key.as_deref(), Some("A"));
    assert_eq!(
        receipt.indeterminate_request.as_ref().unwrap().item_key,
        "A"
    );
    assert!(receipt.reconciliation_observation.is_none());
    assert!(receipt.rollback_operations.is_empty());
    assert_eq!(receipt.not_attempted_item_keys, ["B"]);
}

fn preflight_state(
    plan: &conceptweave_zotero::ClassificationWritePlan,
    item_key: &str,
) -> ClassificationItemState {
    let operation = plan
        .operations()
        .iter()
        .find(|operation| operation.item_key == item_key)
        .unwrap();
    ClassificationItemState {
        server_id: "server-1".into(),
        library_version: plan.library_version(),
        item_key: item_key.into(),
        item_version: operation.item_version,
        collection_keys: operation.before_collection_keys.clone(),
        tags: operation.before_tags.clone(),
    }
}

#[test]
fn execution_fails_closed_for_each_preflight_mismatch() {
    let report = classification_report("10.0.0");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true)
            .unwrap();
    let mismatches: [fn(&mut ClassificationItemState); 6] = [
        |state| state.server_id = "other-server".into(),
        |state| state.library_version += 1,
        |state| state.item_key = "other-item".into(),
        |state| state.item_version += 1,
        |state| state.collection_keys = vec!["other-collection".into()],
        |state| state.tags = vec![tag("Other", None)],
    ];
    for mismatch in mismatches {
        let receipt = execute_classification_write_plan(
            &plan,
            |item_key| {
                let mut state = preflight_state(&plan, item_key);
                if item_key == "A" {
                    mismatch(&mut state);
                }
                Ok::<_, ()>(state)
            },
            |_| -> Result<ClassificationItemState, ()> { panic!("preflight must finish first") },
        );
        assert_eq!(
            receipt.outcome,
            ClassificationWriteOutcome::PreflightFailure
        );
        assert_eq!(receipt.failed_item_key.as_deref(), Some("A"));
        assert_eq!(receipt.not_attempted_item_keys, ["A", "B"]);
    }

    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| {
            let mut state = preflight_state(&plan, item_key);
            state.collection_keys.push(" ".into());
            Ok::<_, ()>(state)
        },
        |_| -> Result<ClassificationItemState, ()> { panic!("invalid metadata must fail first") },
    );
    assert_eq!(
        receipt.outcome,
        ClassificationWriteOutcome::PreflightFailure
    );

    let receipt = execute_classification_write_plan(
        &plan,
        |_| Err::<ClassificationItemState, _>(()),
        |_| -> Result<ClassificationItemState, ()> {
            panic!("failed preflight must prevent writes")
        },
    );
    assert_eq!(
        receipt.outcome,
        ClassificationWriteOutcome::PreflightFailure
    );
}

fn applied_state(
    request: &conceptweave_zotero::ClassificationWriteRequest,
) -> ClassificationItemState {
    ClassificationItemState {
        server_id: request.server_id.clone(),
        library_version: request.library_version + 1,
        item_key: request.item_key.clone(),
        item_version: request.item_version + 1,
        collection_keys: request.collection_keys.clone(),
        tags: request.tags.clone(),
    }
}

#[test]
fn execution_verifies_each_write_response_and_success_receipt() {
    let report = classification_report("10.0.0");
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true)
            .unwrap();
    let mismatches: [fn(&mut ClassificationItemState); 6] = [
        |state| state.server_id = "other-server".into(),
        |state| state.library_version -= 1,
        |state| state.item_key = "other-item".into(),
        |state| state.item_version -= 1,
        |state| state.collection_keys = vec!["other-collection".into()],
        |state| state.tags = vec![tag("Other", None)],
    ];
    for mismatch in mismatches {
        let receipt = execute_classification_write_plan(
            &plan,
            |item_key| Ok::<_, ()>(preflight_state(&plan, item_key)),
            |request| {
                let mut state = applied_state(request);
                mismatch(&mut state);
                Ok::<_, ()>(state)
            },
        );
        assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
        assert_eq!(receipt.failed_item_key.as_deref(), Some("A"));
        assert_eq!(receipt.not_attempted_item_keys, ["B"]);
    }

    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| Ok::<_, ()>(preflight_state(&plan, item_key)),
        |request| {
            let mut state = applied_state(request);
            state.tags.push(tag(" ", None));
            Ok::<_, ()>(state)
        },
    );
    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);

    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| Ok::<_, ()>(preflight_state(&plan, item_key)),
        |request| Ok::<_, ()>(applied_state(request)),
    );
    assert_eq!(receipt.outcome, ClassificationWriteOutcome::Applied);
    assert_eq!(receipt.applied_item_keys, ["A", "B"]);
    assert_eq!(receipt.rollback_operations[0].item_key, "B");
    assert_eq!(receipt.rollback_operations[1].item_key, "A");
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
        rule_revision: report.rule_revision.clone(),
        snapshot_digest: report.snapshot_digest.clone(),
        proposal_digest: classification_proposal_digest(report),
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

    assert_eq!(plan.mode(), WriteMode::DryRun);
    assert_eq!(plan.operations()[0].item_key, "A");
    assert_eq!(plan.operations()[1].item_key, "B");
    assert_eq!(
        plan.operations()[1].rollback_collection_keys,
        plan.operations()[1].before_collection_keys
    );
    assert_eq!(
        plan.operations()[1].rollback_tags,
        vec![tag("Imported", Some(1))]
    );
    assert!(plan.source_records_preserved());
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
        rule_revision: no_server.rule_revision.clone(),
        snapshot_digest: no_server.snapshot_digest.clone(),
        proposal_digest: classification_proposal_digest(&no_server),
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
    assert_eq!(manual_plan.operations()[1].after_tags[0].tag_type, None);

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
