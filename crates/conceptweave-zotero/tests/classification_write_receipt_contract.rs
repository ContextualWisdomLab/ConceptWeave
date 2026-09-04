use conceptweave_zotero::{
    ClassificationItemState, ClassificationWriteOutcome, Disposition, ItemData, ItemTag,
    ReviewedClassificationChange, ReviewedClassificationWriteSet, WriteMode, ZoteroItem,
    build_classification_write_plan, classify_snapshot, execute_classification_write_plan,
};

fn tag(name: &str, tag_type: Option<u64>) -> ItemTag {
    ItemTag {
        tag: name.into(),
        tag_type,
    }
}

fn classification_report() -> conceptweave_zotero::ClassificationReport {
    classify_snapshot(
        "10.0.0".into(),
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
        rule_revision: report.rule_revision.clone(),
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
fn every_receipt_binds_to_the_reviewed_plan_coordinates() {
    let report = classification_report();
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::DryRun, |_| true)
            .unwrap();
    let receipt = execute_classification_write_plan(
        &plan,
        |_| -> Result<ClassificationItemState, ()> { panic!("dry-run must not preflight") },
        |_| -> Result<ClassificationItemState, ()> { panic!("dry-run must not write") },
    );

    assert_eq!(receipt.review_id, "review-1");
    assert_eq!(receipt.authority_receipt, "authority-1");
    assert_eq!(receipt.server_id.as_deref(), Some("server-1"));
    assert_eq!(receipt.library_version, 42);
    assert_eq!(receipt.rule_revision, report.rule_revision);
    assert_eq!(receipt.snapshot_digest, report.snapshot_digest);
}

#[test]
fn dry_run_receipt_enumerates_every_operation_as_not_attempted() {
    let report = classification_report();
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::DryRun, |_| true)
            .unwrap();
    let receipt = execute_classification_write_plan(
        &plan,
        |_| -> Result<ClassificationItemState, ()> { panic!("dry-run must not preflight") },
        |_| -> Result<ClassificationItemState, ()> { panic!("dry-run must not write") },
    );

    assert_eq!(receipt.outcome, ClassificationWriteOutcome::DryRun);
    assert_eq!(receipt.not_attempted_item_keys, ["A", "B"]);
}

#[test]
fn confirmed_unexpected_mutation_retains_known_inverse_rollback() {
    let report = classification_report();
    let plan =
        build_classification_write_plan(&report, &reviewed(&report), WriteMode::Execute, |_| true)
            .unwrap();
    let initial_preflight_count = plan.operations().len();
    let mut reads = 0usize;

    let receipt = execute_classification_write_plan(
        &plan,
        |item_key| {
            reads += 1;
            if reads <= initial_preflight_count {
                return Ok::<_, ()>(preflight_state(&plan, item_key));
            }

            let operation = &plan.operations()[0];
            Ok::<_, ()>(ClassificationItemState {
                server_id: "server-1".into(),
                library_version: 43,
                item_key: operation.item_key.clone(),
                item_version: operation.item_version + 1,
                collection_keys: vec!["unexpected_collection".into()],
                tags: operation.before_tags.clone(),
            })
        },
        |_| Err::<ClassificationItemState, _>(()),
    );

    assert_eq!(receipt.outcome, ClassificationWriteOutcome::PartialFailure);
    assert_eq!(receipt.failed_item_key.as_deref(), Some("A"));
    assert_eq!(receipt.indeterminate_item_key.as_deref(), Some("A"));
    assert_eq!(receipt.rollback_operations.len(), 1);
    assert_eq!(receipt.rollback_operations[0].item_key, "A");
    assert_eq!(receipt.rollback_operations[0].item_version, 8);
    assert!(receipt.rollback_operations[0].collection_keys.is_empty());
    assert!(receipt.rollback_operations[0].tags.is_empty());
}

#[test]
fn unexpected_mutation_requires_the_planned_server_and_item_identity() {
    for (server_id, item_key) in [("other-server", "A"), ("server-1", "other-item")] {
        let report = classification_report();
        let plan = build_classification_write_plan(
            &report,
            &reviewed(&report),
            WriteMode::Execute,
            |_| true,
        )
        .unwrap();
        let initial_preflight_count = plan.operations().len();
        let mut reads = 0usize;

        let receipt = execute_classification_write_plan(
            &plan,
            |requested_item_key| {
                reads += 1;
                if reads <= initial_preflight_count {
                    return Ok::<_, ()>(preflight_state(&plan, requested_item_key));
                }

                Ok::<_, ()>(ClassificationItemState {
                    server_id: server_id.into(),
                    library_version: 43,
                    item_key: item_key.into(),
                    item_version: 8,
                    collection_keys: vec!["unexpected_collection".into()],
                    tags: vec![],
                })
            },
            |_| Err::<ClassificationItemState, _>(()),
        );

        assert_eq!(receipt.indeterminate_item_key.as_deref(), Some("A"));
        assert!(receipt.rollback_operations.is_empty());
    }
}
