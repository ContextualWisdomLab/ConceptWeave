use conceptweave_zotero::{
    ClassificationItemState, ClassificationRollbackOutcome, ClassificationWriteRequest,
    Disposition, ItemData, ItemTag, ReviewedClassificationChange, ReviewedClassificationWriteSet,
    WriteMode, ZoteroItem, build_classification_write_plan, classify_snapshot,
    execute_classification_rollback, execute_classification_write_plan,
};

fn tag(name: &str) -> ItemTag {
    ItemTag {
        tag: name.into(),
        tag_type: None,
    }
}

fn plan() -> conceptweave_zotero::ClassificationWritePlan {
    let report = classify_snapshot(
        "10.0.0".into(),
        Some("server-1".into()),
        42,
        vec![
            ZoteroItem {
                key: "A".into(),
                version: 7,
                data: ItemData {
                    item_type: "book".into(),
                    title: "ontology learning".into(),
                    abstract_note: String::new(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec![],
                    tags: vec![],
                },
            },
            ZoteroItem {
                key: "B".into(),
                version: 9,
                data: ItemData {
                    item_type: "book".into(),
                    title: "ontology evaluation".into(),
                    abstract_note: String::new(),
                    doi: String::new(),
                    parent_item: String::new(),
                    collections: vec!["source".into()],
                    tags: vec![tag("Imported")],
                },
            },
        ],
    );
    let reviewed = ReviewedClassificationWriteSet {
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
                item_key: "A".into(),
                item_version: 7,
                reviewed_disposition: Disposition::Generation,
                before_collection_keys: vec![],
                after_collection_keys: vec!["generation".into()],
                before_tags: vec![],
                after_tags: vec![tag("Generation")],
            },
            ReviewedClassificationChange {
                item_key: "B".into(),
                item_version: 9,
                reviewed_disposition: Disposition::EvaluationGovernance,
                before_collection_keys: vec!["source".into()],
                after_collection_keys: vec!["evaluation".into()],
                before_tags: vec![tag("Imported")],
                after_tags: vec![tag("Evaluation")],
            },
        ],
    };
    build_classification_write_plan(&report, &reviewed, WriteMode::Execute, |_| true).unwrap()
}

fn applied_receipt() -> conceptweave_zotero::ClassificationWriteReceipt {
    let plan = plan();
    execute_classification_write_plan(
        &plan,
        |key| {
            let operation = plan
                .operations()
                .iter()
                .find(|item| item.item_key == key)
                .unwrap();
            Ok::<_, ()>(ClassificationItemState {
                server_id: "server-1".into(),
                library_version: 42,
                item_key: key.into(),
                item_version: operation.item_version,
                collection_keys: operation.before_collection_keys.clone(),
                tags: operation.before_tags.clone(),
            })
        },
        |request| Ok::<_, ()>(written(request)),
    )
}

fn written(request: &ClassificationWriteRequest) -> ClassificationItemState {
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
fn rollback_preflights_every_item_then_restores_in_receipt_order() {
    let receipt = applied_receipt();
    assert_eq!(receipt.rollback_operations[0].item_key, "B");
    assert_eq!(receipt.rollback_operations[0].server_id, "server-1");
    assert_eq!(
        receipt.rollback_operations[0].expected_collection_keys,
        ["evaluation"]
    );
    assert_eq!(
        receipt.rollback_operations[0].expected_tags,
        [tag("Evaluation")]
    );

    let mut reads = Vec::new();
    let mut writes = Vec::new();
    let result = execute_classification_rollback(
        &receipt.rollback_operations,
        |key| {
            reads.push(key.to_owned());
            let operation = receipt
                .rollback_operations
                .iter()
                .find(|item| item.item_key == key)
                .unwrap();
            Ok::<_, ()>(ClassificationItemState {
                server_id: operation.server_id.clone(),
                library_version: 44,
                item_key: key.into(),
                item_version: operation.item_version,
                collection_keys: operation.expected_collection_keys.clone(),
                tags: operation.expected_tags.clone(),
            })
        },
        |request| {
            writes.push((request.item_key.clone(), request.library_version));
            Ok::<_, ()>(written(request))
        },
    );

    assert_eq!(reads, ["B", "A"]);
    assert_eq!(writes, [("B".into(), 44), ("A".into(), 45)]);
    assert_eq!(result.outcome, ClassificationRollbackOutcome::Restored);
    assert_eq!(result.restored_item_keys, ["B", "A"]);
    assert!(result.failed_item_key.is_none());
    assert!(result.indeterminate_item_key.is_none());
    assert!(result.not_attempted_item_keys.is_empty());
    assert!(result.remaining_operations.is_empty());
}

#[test]
fn rollback_is_one_shot_and_second_execution_fails_before_write() {
    use std::cell::Cell;

    let receipt = applied_receipt();
    let restored = Cell::new(false);
    let writes = Cell::new(0);
    let run = || {
        execute_classification_rollback(
            &receipt.rollback_operations,
            |key| {
                let operation = receipt
                    .rollback_operations
                    .iter()
                    .find(|item| item.item_key == key)
                    .unwrap();
                Ok::<_, ()>(ClassificationItemState {
                    server_id: operation.server_id.clone(),
                    library_version: if restored.get() { 46 } else { 44 },
                    item_key: key.into(),
                    item_version: operation.item_version + u64::from(restored.get()),
                    collection_keys: if restored.get() {
                        operation.collection_keys.clone()
                    } else {
                        operation.expected_collection_keys.clone()
                    },
                    tags: if restored.get() {
                        operation.tags.clone()
                    } else {
                        operation.expected_tags.clone()
                    },
                })
            },
            |request| {
                writes.set(writes.get() + 1);
                Ok::<_, ()>(written(request))
            },
        )
    };
    assert_eq!(run().outcome, ClassificationRollbackOutcome::Restored);
    restored.set(true);
    assert_eq!(
        run().outcome,
        ClassificationRollbackOutcome::PreflightFailure
    );
    assert_eq!(writes.get(), 2);
}

#[test]
fn rollback_reconciles_failed_write_without_guessing() {
    for (current, expected_outcome) in [
        ("restored", (vec!["B"], None)),
        ("unchanged", (vec![], None)),
        ("indeterminate", (vec![], Some("B"))),
    ] {
        let receipt = applied_receipt();
        let mut reads = 0;
        let result = execute_classification_rollback(
            &receipt.rollback_operations,
            |key| {
                reads += 1;
                let operation = receipt
                    .rollback_operations
                    .iter()
                    .find(|item| item.item_key == key)
                    .unwrap();
                let reconciliation = reads > receipt.rollback_operations.len();
                Ok::<_, ()>(ClassificationItemState {
                    server_id: operation.server_id.clone(),
                    library_version: if reconciliation && current != "unchanged" {
                        45
                    } else {
                        44
                    },
                    item_key: key.into(),
                    item_version: operation.item_version
                        + u64::from(reconciliation && current != "unchanged"),
                    collection_keys: if reconciliation && current == "restored" {
                        operation.collection_keys.clone()
                    } else if reconciliation && current == "indeterminate" {
                        vec!["other".into()]
                    } else {
                        operation.expected_collection_keys.clone()
                    },
                    tags: if reconciliation && current == "restored" {
                        operation.tags.clone()
                    } else {
                        operation.expected_tags.clone()
                    },
                })
            },
            |_| Err::<ClassificationItemState, _>(()),
        );
        assert_eq!(
            result.outcome,
            ClassificationRollbackOutcome::PartialFailure
        );
        assert_eq!(result.restored_item_keys, expected_outcome.0);
        assert_eq!(result.failed_item_key.as_deref(), Some("B"));
        assert_eq!(result.indeterminate_item_key.as_deref(), expected_outcome.1);
        assert_eq!(result.not_attempted_item_keys, ["A"]);
        assert_eq!(
            result.remaining_operations.len(),
            if current == "restored" { 1 } else { 2 }
        );
    }
}

#[test]
fn rollback_receipts_are_secret_free_serializable_evidence() {
    let receipt = applied_receipt();
    let json = serde_json::to_value(&receipt.rollback_operations).unwrap();
    let text = json.to_string();
    assert!(text.contains("server-1"));
    assert!(text.contains("expected_collection_keys"));
    assert!(!text.to_ascii_lowercase().contains("api_key"));
}

#[test]
fn rollback_preflight_rejects_unreadable_mixed_or_mismatched_state() {
    let receipt = applied_receipt();
    let failed = execute_classification_rollback(
        &receipt.rollback_operations,
        |_| Err::<ClassificationItemState, _>(()),
        |_| -> Result<ClassificationItemState, ()> { panic!("preflight must prevent writes") },
    );
    assert_eq!(
        failed.outcome,
        ClassificationRollbackOutcome::PreflightFailure
    );

    let mismatches: [fn(&mut ClassificationItemState); 6] = [
        |state| state.server_id = "other-server".into(),
        |state| state.item_key = "other-item".into(),
        |state| state.item_version += 1,
        |state| state.collection_keys = vec!["other".into()],
        |state| state.tags = vec![tag("Other")],
        |state| state.collection_keys.push(" ".into()),
    ];
    for mismatch in mismatches {
        let failed = execute_classification_rollback(
            &receipt.rollback_operations,
            |key| {
                let operation = receipt
                    .rollback_operations
                    .iter()
                    .find(|item| item.item_key == key)
                    .unwrap();
                let mut state = ClassificationItemState {
                    server_id: operation.server_id.clone(),
                    library_version: 44,
                    item_key: key.into(),
                    item_version: operation.item_version,
                    collection_keys: operation.expected_collection_keys.clone(),
                    tags: operation.expected_tags.clone(),
                };
                if key == "B" {
                    mismatch(&mut state);
                }
                Ok::<_, ()>(state)
            },
            |_| -> Result<ClassificationItemState, ()> { panic!("preflight must prevent writes") },
        );
        assert_eq!(
            failed.outcome,
            ClassificationRollbackOutcome::PreflightFailure
        );
    }

    let mixed = execute_classification_rollback(
        &receipt.rollback_operations,
        |key| {
            let operation = receipt
                .rollback_operations
                .iter()
                .find(|item| item.item_key == key)
                .unwrap();
            Ok::<_, ()>(ClassificationItemState {
                server_id: operation.server_id.clone(),
                library_version: if key == "B" { 44 } else { 45 },
                item_key: key.into(),
                item_version: operation.item_version,
                collection_keys: operation.expected_collection_keys.clone(),
                tags: operation.expected_tags.clone(),
            })
        },
        |_| -> Result<ClassificationItemState, ()> { panic!("preflight must prevent writes") },
    );
    assert_eq!(
        mixed.outcome,
        ClassificationRollbackOutcome::PreflightFailure
    );
}

#[test]
fn rollback_rejects_each_unverified_restoration_response() {
    let receipt = applied_receipt();
    let mismatches: [fn(&mut ClassificationItemState); 6] = [
        |state| state.server_id = "other-server".into(),
        |state| state.library_version = 44,
        |state| state.item_key = "other-item".into(),
        |state| state.item_version -= 1,
        |state| state.collection_keys = vec!["other".into()],
        |state| state.tags = vec![tag("Other")],
    ];
    for mismatch in mismatches {
        let failed = execute_classification_rollback(
            &receipt.rollback_operations,
            |key| {
                let operation = receipt
                    .rollback_operations
                    .iter()
                    .find(|item| item.item_key == key)
                    .unwrap();
                Ok::<_, ()>(ClassificationItemState {
                    server_id: operation.server_id.clone(),
                    library_version: 44,
                    item_key: key.into(),
                    item_version: operation.item_version,
                    collection_keys: operation.expected_collection_keys.clone(),
                    tags: operation.expected_tags.clone(),
                })
            },
            |request| {
                let mut state = written(request);
                mismatch(&mut state);
                Ok::<_, ()>(state)
            },
        );
        assert_eq!(
            failed.outcome,
            ClassificationRollbackOutcome::PartialFailure
        );
        assert_eq!(failed.indeterminate_item_key, None);
    }
}
