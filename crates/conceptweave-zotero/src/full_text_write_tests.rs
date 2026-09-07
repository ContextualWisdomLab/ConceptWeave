use super::*;
use crate::{
    FullTextWriteScope, WriteMode, build_full_text_write_plan, execute_full_text_write_plan,
};
use std::cell::Cell;
use std::cell::RefCell;

#[test]
fn full_text_write_observation_preserves_original_attempt_without_granting_recovery() {
    for failed_index in 0..2 {
        for observation in 0..5 {
            let report = report_fixture();
            let capture = capture_with(&report, 4096, &mut |request_path, _| {
                Ok(response_fixture(request_path))
            })
            .unwrap();
            let mut scope = write_scope_fixture(&report, &capture);
            scope.mode = WriteMode::Execute;
            let plan =
                build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
            let store = WriteStore::from_report(&report);
            let attempted_request = RefCell::new(None);
            let receipt = execute_full_text_write_plan(
                &plan,
                |item_key| {
                    if store.read_count.get() >= 2 {
                        return Err(());
                    }
                    store.read_item(item_key)
                },
                |request| {
                    if store.write_count.get() == failed_index {
                        *attempted_request.borrow_mut() = Some(request.clone());
                        if observation == 1 {
                            store.write_item(request).unwrap();
                        }
                        return Err(());
                    }
                    store.write_item(request)
                },
            );
            let original = serde_json::to_value(&receipt).unwrap();
            let request = attempted_request.borrow().clone().unwrap();
            assert_eq!(
                request.library_version,
                report.library_version + failed_index as u64
            );
            assert_eq!(
                original["indeterminate_request"],
                serde_json::to_value(&request).unwrap()
            );
            assert_eq!(
                original["write_result"]["indeterminate_item_key"],
                request.item_key
            );
            assert_eq!(
                original["write_result"]["rollback_operations"]
                    .as_array()
                    .unwrap()
                    .len(),
                failed_index
            );
            assert_eq!(
                original["write_result"]["not_attempted_item_keys"]
                    .as_array()
                    .unwrap()
                    .len(),
                1 - failed_index
            );
            let reads_before = store.read_count.get();
            let writes_before = store.write_count.get();
            let observed = crate::observe_full_text_write(&receipt, |item_key| {
                assert_eq!(item_key, request.item_key);
                let mut state = store.read_item(item_key)?;
                match observation {
                    2 => {
                        state.server_id = "OTHER_SERVER".into();
                        state.item_key = "OTHER_ITEM".into();
                    }
                    3 => state.collection_keys = vec![" ".into()],
                    4 => return Err(()),
                    _ => {}
                }
                Ok(state)
            })
            .unwrap();
            let result = serde_json::to_value(observed).unwrap();
            assert_eq!(result["write_receipt"], original);
            assert_eq!(store.read_count.get(), reads_before + 1);
            assert_eq!(store.write_count.get(), writes_before);
            if observation == 4 {
                assert!(result["observed_state"].is_null());
            } else {
                let state = &result["observed_state"];
                match observation {
                    0 => assert_eq!(state["item_version"], request.item_version),
                    1 => assert_eq!(
                        state["collection_keys"],
                        serde_json::json!(request.collection_keys)
                    ),
                    2 => assert_eq!(state["server_id"], "OTHER_SERVER"),
                    _ => assert_eq!(state["collection_keys"], serde_json::json!([" "])),
                }
            }
            assert_eq!(serde_json::to_value(&receipt).unwrap(), original);
            assert!(!result.to_string().contains("synthetic-write-authority"));
            assert!(!result.to_string().contains("synthetic-write-review"));
            assert!(
                crate::execute_full_text_rollback(
                    &receipt,
                    |_| -> Result<crate::ClassificationItemState, ()> {
                        panic!("unknown write read")
                    },
                    |_| -> Result<crate::ClassificationItemState, ()> {
                        panic!("unknown write mutation")
                    },
                )
                .is_err()
            );
        }
    }
}

#[test]
fn full_text_write_observation_refuses_non_unknown_receipts_without_reading() {
    for scenario in 0..4 {
        let report = report_fixture();
        let capture = capture_with(&report, 4096, &mut |request_path, _| {
            Ok(response_fixture(request_path))
        })
        .unwrap();
        let mut scope = write_scope_fixture(&report, &capture);
        if scenario != 0 {
            scope.mode = WriteMode::Execute;
        }
        let plan =
            build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
        let store = WriteStore::from_report(&report);
        if scenario == 1 {
            store.library_version.set(99);
        }
        let receipt = execute_full_text_write_plan(
            &plan,
            |item_key| store.read_item(item_key),
            |request| {
                if scenario == 3 {
                    Err(())
                } else {
                    store.write_item(request)
                }
            },
        );
        if scenario == 3 {
            let observed =
                crate::observe_full_text_write(&receipt, |item_key| store.read_item(item_key))
                    .unwrap();
            let result = serde_json::to_value(observed).unwrap();
            assert_eq!(
                result["write_receipt"],
                serde_json::to_value(&receipt).unwrap()
            );
            assert_eq!(store.write_count.get(), 0);
            continue;
        }
        assert!(
            serde_json::to_value(&receipt)
                .unwrap()
                .get("indeterminate_request")
                .is_none()
        );
        assert!(
            crate::observe_full_text_write(
                &receipt,
                |_| -> Result<crate::ClassificationItemState, ()> { panic!("no unknown request") },
            )
            .is_err()
        );
    }
}

#[test]
fn full_text_write_reconciliation_retains_matching_changed_and_failed_observations() {
    for observation in 0..3 {
        let report = report_fixture();
        let capture = capture_with(&report, 4096, &mut |request_path, _| {
            Ok(response_fixture(request_path))
        })
        .unwrap();
        let mut scope = write_scope_fixture(&report, &capture);
        scope.mode = WriteMode::Execute;
        let plan =
            build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
        let store = WriteStore::from_report(&report);
        let written = execute_full_text_write_plan(
            &plan,
            |item_key| store.read_item(item_key),
            |request| store.write_item(request),
        );
        store.read_count.set(0);
        let partial = crate::execute_full_text_rollback(
            &written,
            |item_key| {
                if store.read_count.get() >= 2 {
                    return Err(());
                }
                store.read_item(item_key)
            },
            |request| -> Result<crate::ClassificationItemState, ()> {
                if observation == 0 {
                    store.write_item(request).unwrap();
                }
                Err(())
            },
        )
        .unwrap();
        let reconciled = crate::reconcile_full_text_rollback(&partial, |item_key| {
            if observation == 2 {
                return Err(());
            }
            let mut state = store.read_item(item_key)?;
            if observation == 1 {
                state.collection_keys = vec!["UNEXPECTED_CHANGE".into()];
            }
            Ok(state)
        })
        .unwrap();
        let result = serde_json::to_value(&reconciled).unwrap();
        assert_eq!(
            result["full_text_write_v1"],
            serde_json::to_value(&plan).unwrap()["full_text_write_v1"]
        );
        assert_eq!(result["remaining_operations"].as_array().unwrap().len(), 1);
        assert_eq!(
            result["rollback_receipt"],
            serde_json::to_value(&partial).unwrap()
        );
        assert_eq!(result["reconciliation_result"]["state"], "indeterminate");
        assert!(
            crate::retry_full_text_reconciled_rollback(
                &reconciled,
                |_| -> Result<crate::ClassificationItemState, ()> {
                    panic!("unresolved retry read")
                },
                |_| -> Result<crate::ClassificationItemState, ()> {
                    panic!("unresolved retry write")
                }
            )
            .is_err()
        );
        assert_eq!(
            store.write_count.get(),
            if observation == 0 { 3 } else { 2 }
        );
    }
}

#[test]
fn full_text_write_unknown_partial_failure_retains_verified_work_without_recovery_authority() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let mut scope = write_scope_fixture(&report, &capture);
    scope.mode = WriteMode::Execute;
    let plan = build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
    let store = WriteStore::from_report(&report);
    let receipt = execute_full_text_write_plan(
        &plan,
        |item_key| store.read_item(item_key),
        |request| {
            if store.write_count.get() == 1 {
                return Err(());
            }
            store.write_item(request)
        },
    );
    let result = serde_json::to_value(&receipt).unwrap();
    assert_eq!(result["write_result"]["outcome"], "partial_failure");
    assert_eq!(result["write_result"]["indeterminate_item_key"], "DEFG5678");
    assert_eq!(
        result["indeterminate_request"],
        result["write_result"]["indeterminate_request"]
    );
    assert_eq!(
        result["write_result"]["rollback_operations"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(
        crate::execute_full_text_rollback(
            &receipt,
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("unknown original read") },
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("unknown original write") },
        )
        .is_err()
    );
    assert_eq!(store.write_count.get(), 1);
}

struct WriteStore {
    items: RefCell<BTreeMap<String, crate::ClassificationItemState>>,
    library_version: Cell<u64>,
    read_count: Cell<usize>,
    write_count: Cell<usize>,
}

impl WriteStore {
    fn from_report(report: &ClassificationReport) -> Self {
        Self {
            items: RefCell::new(
                report
                    .classified_items
                    .iter()
                    .map(|item| {
                        (
                            item.item_key.clone(),
                            crate::ClassificationItemState {
                                server_id: report.server_id.clone().unwrap(),
                                library_version: report.library_version,
                                item_key: item.item_key.clone(),
                                item_version: item.item_version,
                                collection_keys: item.collection_keys.clone(),
                                tags: item.tags.clone(),
                            },
                        )
                    })
                    .collect(),
            ),
            library_version: Cell::new(report.library_version),
            read_count: Cell::new(0),
            write_count: Cell::new(0),
        }
    }

    fn read_item(&self, item_key: &str) -> Result<crate::ClassificationItemState, ()> {
        self.read_count.set(self.read_count.get() + 1);
        let mut item = self.items.borrow().get(item_key).ok_or(())?.clone();
        item.library_version = self.library_version.get();
        Ok(item)
    }

    fn write_item(
        &self,
        request: &crate::ClassificationWriteRequest,
    ) -> Result<crate::ClassificationItemState, ()> {
        assert_eq!(request.library_version, self.library_version.get());
        assert_eq!(
            request.item_version,
            self.items.borrow()[&request.item_key].item_version
        );
        self.write_count.set(self.write_count.get() + 1);
        self.library_version.set(self.library_version.get() + 1);
        let state = crate::ClassificationItemState {
            server_id: request.server_id.clone(),
            library_version: self.library_version.get(),
            item_key: request.item_key.clone(),
            item_version: self.library_version.get(),
            collection_keys: request.collection_keys.clone(),
            tags: request.tags.clone(),
        };
        self.items
            .borrow_mut()
            .insert(request.item_key.clone(), state.clone());
        Ok(state)
    }
}

#[test]
fn full_text_write_rejects_substituted_mode_destinations_and_labels_under_old_authority() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let original = serde_json::to_value(write_scope_fixture(&report, &capture)).unwrap();
    for scenario in 0..3 {
        let mut scope = write_scope_fixture(&report, &capture);
        match scenario {
            0 => scope.mode = WriteMode::Execute,
            1 => {
                scope.reviewed_writes.changes[0].after_collection_keys =
                    vec!["OTHER_COLLECTION".into()]
            }
            _ => {
                let mut reviewed = serde_json::to_value(&scope.full_text_review).unwrap();
                reviewed["full_text_golden_set_v1"]["labels"][0]["expected_disposition"] =
                    serde_json::json!(crate::Disposition::SemanticConsumptionBridge);
                scope.full_text_review = serde_json::from_value(reviewed).unwrap();
                scope.reviewed_writes.changes[0].reviewed_disposition =
                    crate::Disposition::SemanticConsumptionBridge;
            }
        }
        assert!(
            build_full_text_write_plan(
                &report,
                &capture,
                scope,
                |reviewed| serde_json::to_value(reviewed).unwrap() == original["full_text_review"],
                |received| serde_json::to_value(received).unwrap() == original
            )
            .is_err()
        );
    }
}

#[test]
fn full_text_write_and_rollback_preserve_binding_and_conditional_state() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let mut scope = write_scope_fixture(&report, &capture);
    scope.mode = WriteMode::Execute;
    let plan = build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
    let expected_binding = serde_json::to_value(&plan).unwrap()["full_text_write_v1"].clone();
    let store = WriteStore::from_report(&report);
    let receipt = execute_full_text_write_plan(
        &plan,
        |item_key| store.read_item(item_key),
        |request| store.write_item(request),
    );
    let result = serde_json::to_value(&receipt).unwrap();
    assert_eq!(result["full_text_write_v1"], expected_binding);
    assert_eq!(result["write_result"]["outcome"], "applied");
    assert_eq!(store.read_count.get(), 2);
    assert_eq!(store.write_count.get(), 2);
    let rollback = crate::execute_full_text_rollback(
        &receipt,
        |item_key| store.read_item(item_key),
        |request| store.write_item(request),
    )
    .unwrap();
    let result = serde_json::to_value(&rollback).unwrap();
    assert_eq!(result["full_text_write_v1"], expected_binding);
    assert_eq!(result["rollback_result"]["outcome"], "restored");
    assert!(
        store
            .items
            .borrow()
            .values()
            .all(|item| item.collection_keys.is_empty())
    );
    assert_eq!(store.write_count.get(), 4);
    assert!(
        crate::retry_full_text_rollback(
            &rollback,
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("nothing to retry") },
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("nothing to retry") }
        )
        .is_err()
    );
    assert!(
        crate::reconcile_full_text_rollback(
            &rollback,
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("nothing to reconcile") }
        )
        .is_err()
    );
}

#[test]
fn full_text_write_retries_only_unattempted_rollback_after_failed_preflight() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let mut scope = write_scope_fixture(&report, &capture);
    scope.mode = WriteMode::Execute;
    let plan = build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
    let store = WriteStore::from_report(&report);
    let written = execute_full_text_write_plan(
        &plan,
        |item_key| store.read_item(item_key),
        |request| store.write_item(request),
    );
    let failed = crate::execute_full_text_rollback(
        &written,
        |_| -> Result<crate::ClassificationItemState, ()> { Err(()) },
        |_| -> Result<crate::ClassificationItemState, ()> {
            panic!("failed preflight cannot write")
        },
    )
    .unwrap();
    let prior = serde_json::to_value(&failed).unwrap();
    assert_eq!(prior["rollback_result"]["outcome"], "preflight_failure");
    assert!(prior["rollback_result"]["indeterminate_request"].is_null());
    assert_eq!(
        prior["rollback_result"]["not_attempted_item_keys"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(store.write_count.get(), 2);
    store.read_count.set(0);
    let restored = crate::retry_full_text_rollback(
        &failed,
        |item_key| store.read_item(item_key),
        |request| store.write_item(request),
    )
    .unwrap();
    let result = serde_json::to_value(&restored).unwrap();
    assert_eq!(result["full_text_write_v1"], prior["full_text_write_v1"]);
    assert_eq!(result["rollback_result"]["outcome"], "restored");
    assert_eq!(
        result["rollback_result"]["restored_item_keys"]
            .as_array()
            .unwrap()
            .len(),
        2
    );
    assert_eq!(store.read_count.get(), 2);
    assert_eq!(store.write_count.get(), 4);
    assert_eq!(serde_json::to_value(&failed).unwrap(), prior);
}

#[test]
fn full_text_write_stale_preflight_and_unknown_writes_never_grant_empty_restoration() {
    for unknown_write in [false, true] {
        let report = report_fixture();
        let capture = capture_with(&report, 4096, &mut |request_path, _| {
            Ok(response_fixture(request_path))
        })
        .unwrap();
        let mut scope = write_scope_fixture(&report, &capture);
        scope.mode = WriteMode::Execute;
        let plan =
            build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
        let store = WriteStore::from_report(&report);
        if !unknown_write {
            store.library_version.set(99);
        }
        let receipt = execute_full_text_write_plan(
            &plan,
            |item_key| {
                if store.read_count.get() >= 2 {
                    return Err(());
                }
                store.read_item(item_key)
            },
            |_| -> Result<crate::ClassificationItemState, ()> {
                assert!(unknown_write);
                Err(())
            },
        );
        let result = serde_json::to_value(&receipt).unwrap();
        assert_eq!(
            result["full_text_write_v1"],
            serde_json::to_value(&plan).unwrap()["full_text_write_v1"]
        );
        assert_eq!(
            result["write_result"]["outcome"],
            if unknown_write {
                "partial_failure"
            } else {
                "preflight_failure"
            }
        );
        assert!(
            crate::execute_full_text_rollback(
                &receipt,
                |_| -> Result<crate::ClassificationItemState, ()> {
                    panic!("no known restoration")
                },
                |_| -> Result<crate::ClassificationItemState, ()> {
                    panic!("no known restoration")
                }
            )
            .is_err()
        );
    }
}

#[test]
fn full_text_write_bound_rollback_retry_and_delayed_reconciliation() {
    for delayed_read in [false, true] {
        for failed_index in 0..2 {
            let report = report_fixture();
            let capture = capture_with(&report, 4096, &mut |request_path, _| {
                Ok(response_fixture(request_path))
            })
            .unwrap();
            let mut scope = write_scope_fixture(&report, &capture);
            scope.mode = WriteMode::Execute;
            let plan =
                build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
            let store = WriteStore::from_report(&report);
            let written = execute_full_text_write_plan(
                &plan,
                |item_key| store.read_item(item_key),
                |request| store.write_item(request),
            );
            store.read_count.set(0);
            let partial = crate::execute_full_text_rollback(
                &written,
                |item_key| {
                    if delayed_read && store.read_count.get() >= 2 {
                        return Err(());
                    }
                    store.read_item(item_key)
                },
                |request| {
                    if store.write_count.get() == 2 + failed_index {
                        Err(())
                    } else {
                        store.write_item(request)
                    }
                },
            )
            .unwrap();
            assert!(
                crate::retry_full_text_rollback(
                    &partial,
                    |_| -> Result<crate::ClassificationItemState, ()> { panic!("unknown inverse") },
                    |_| -> Result<crate::ClassificationItemState, ()> { panic!("unknown inverse") },
                )
                .is_err()
            );
            let reconciled =
                crate::reconcile_full_text_rollback(&partial, |item_key| store.read_item(item_key))
                    .unwrap();
            let result = serde_json::to_value(&reconciled).unwrap();
            assert_eq!(
                result["rollback_receipt"],
                serde_json::to_value(&partial).unwrap()
            );
            assert_eq!(
                result["full_text_write_v1"],
                serde_json::to_value(&plan).unwrap()["full_text_write_v1"]
            );
            assert_eq!(result["reconciliation_result"]["state"], "indeterminate");
            assert!(
                crate::retry_full_text_reconciled_rollback(
                    &reconciled,
                    |_| -> Result<crate::ClassificationItemState, ()> {
                        panic!("observation is not authority")
                    },
                    |_| -> Result<crate::ClassificationItemState, ()> {
                        panic!("observation is not authority")
                    },
                )
                .is_err()
            );
            assert_eq!(
                result["rollback_receipt"]["rollback_result"]["restored_item_keys"]
                    .as_array()
                    .unwrap()
                    .len(),
                failed_index
            );
            assert_eq!(store.write_count.get(), 2 + failed_index);
        }
    }
}

pub(super) fn write_scope_fixture(
    report: &ClassificationReport,
    capture: &FullTextCapture,
) -> FullTextWriteScope {
    let worksheet = build_full_text_review_worksheet(report, capture).unwrap();
    let completed = completed_full_text_view(report, &worksheet, capture, 2);
    let decided = apply_full_text_review_view(report, &worksheet, capture, &completed).unwrap();
    let full_text_review = finalize_full_text_review(
        report,
        &decided,
        capture,
        full_text_approval_fixture(report, capture),
    )
    .unwrap();
    let reviewed_writes = crate::ReviewedClassificationWriteSet {
        review_id: "synthetic-write-review".into(),
        authority_receipt: "synthetic-write-authority".into(),
        server_id: report.server_id.clone(),
        zotero_version: report.zotero_version.clone(),
        library_version: report.library_version,
        rule_revision: report.rule_revision.clone(),
        snapshot_digest: report.snapshot_digest.clone(),
        proposal_digest: crate::classification_proposal_digest(report),
        snapshot_items: report.snapshot_items.clone(),
        changes: report
            .classified_items
            .iter()
            .map(|item| crate::ReviewedClassificationChange {
                item_key: item.item_key.clone(),
                item_version: item.item_version,
                reviewed_disposition: crate::Disposition::OutOfScope,
                before_collection_keys: item.collection_keys.clone(),
                after_collection_keys: vec!["EXPLICIT_COLLECTION".into()],
                before_tags: item.tags.clone(),
                after_tags: item.tags.clone(),
            })
            .collect(),
    };
    FullTextWriteScope {
        full_text_review,
        reviewed_writes,
        mode: WriteMode::DryRun,
    }
}

#[test]
fn full_text_write_validates_both_inputs_before_either_authority() {
    for scenario in 0..7 {
        let report = report_fixture();
        let capture = capture_with(&report, 4096, &mut |request_path, _| {
            Ok(response_fixture(request_path))
        })
        .unwrap();
        let mut scope = write_scope_fixture(&report, &capture);
        match scenario {
            0 => scope.reviewed_writes.changes[1].after_collection_keys = vec![" ".into()],
            1 => {
                scope.reviewed_writes.changes[1].reviewed_disposition =
                    crate::Disposition::SemanticConsumptionBridge
            }
            2 => scope.reviewed_writes.snapshot_digest = "changed".into(),
            3 => scope.reviewed_writes.changes[1].item_version += 1,
            _ => {
                let mut value = serde_json::to_value(&scope.full_text_review).unwrap();
                match scenario {
                    4 => {
                        value["full_text_golden_set_v1"]["labels"]
                            .as_array_mut()
                            .unwrap()
                            .pop();
                    }
                    5 => value["capture_digest"] = "changed".into(),
                    _ => {
                        value["full_text_golden_set_v1"]["approval"]["proposal_digest"] =
                            "changed".into()
                    }
                }
                scope.full_text_review = serde_json::from_value(value).unwrap();
            }
        }
        let calls = Cell::new(0);
        assert!(
            build_full_text_write_plan(
                &report,
                &capture,
                scope,
                |_| {
                    calls.set(calls.get() + 1);
                    true
                },
                |_| {
                    calls.set(calls.get() + 1);
                    true
                }
            )
            .is_err(),
            "scenario {scenario}"
        );
        assert_eq!(calls.get(), 0, "scenario {scenario}");
    }
}

#[test]
fn full_text_write_verifiers_receive_exact_scope_and_mode() {
    for mode in [WriteMode::DryRun, WriteMode::Execute] {
        for semantic_allowed in [false, true] {
            for write_allowed in [false, true] {
                let report = report_fixture();
                let capture = capture_with(&report, 4096, &mut |request_path, _| {
                    Ok(response_fixture(request_path))
                })
                .unwrap();
                let mut scope = write_scope_fixture(&report, &capture);
                scope.mode = mode;
                let expected = serde_json::to_value(&scope).unwrap();
                let semantic_calls = Cell::new(0);
                let write_calls = Cell::new(0);
                let result = build_full_text_write_plan(
                    &report,
                    &capture,
                    scope,
                    |actual| {
                        semantic_calls.set(semantic_calls.get() + 1);
                        assert_eq!(
                            serde_json::to_value(actual).unwrap(),
                            expected["full_text_review"]
                        );
                        semantic_allowed
                    },
                    |actual| {
                        write_calls.set(write_calls.get() + 1);
                        assert_eq!(serde_json::to_value(actual).unwrap(), expected);
                        write_allowed
                    },
                );
                assert_eq!(result.is_ok(), semantic_allowed && write_allowed);
                assert_eq!(semantic_calls.get(), 1);
                assert_eq!(write_calls.get(), usize::from(semantic_allowed));
            }
        }
    }
}

#[test]
fn full_text_write_dry_run_preserves_binding_without_reads_or_writes() {
    let report = report_fixture();
    let capture = capture_with(&report, 4096, &mut |request_path, _| {
        Ok(response_fixture(request_path))
    })
    .unwrap();
    let scope = write_scope_fixture(&report, &capture);
    let plan = build_full_text_write_plan(&report, &capture, scope, |_| true, |_| true).unwrap();
    let receipt = execute_full_text_write_plan(
        &plan,
        |_| -> Result<crate::ClassificationItemState, ()> { panic!("dry-run read") },
        |_| -> Result<crate::ClassificationItemState, ()> { panic!("dry-run write") },
    );
    let plan_json = serde_json::to_value(&plan).unwrap();
    let receipt_json = serde_json::to_value(&receipt).unwrap();
    assert!(
        crate::execute_full_text_rollback(
            &receipt,
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("dry-run recovery read") },
            |_| -> Result<crate::ClassificationItemState, ()> { panic!("dry-run recovery write") }
        )
        .is_err()
    );
    assert_eq!(
        receipt_json["full_text_write_v1"],
        plan_json["full_text_write_v1"]
    );
    assert_eq!(receipt_json["write_result"]["outcome"], "dry_run");
    assert!(!receipt_json.to_string().contains("fixture text"));
    assert!(
        !receipt_json
            .to_string()
            .contains("synthetic-write-authority")
    );
}
