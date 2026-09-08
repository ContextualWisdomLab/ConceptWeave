use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionError,
    SourceResolutionRestoreError, ZoteroItem, classify_snapshot, prepare_source_resolution_review,
    restore_source_resolution_review,
};

fn restored_is_err(
    report: &conceptweave_zotero::ClassificationReport,
    value: serde_json::Value,
) -> bool {
    let bytes = serde_json::to_vec(&value).expect("stored value must serialize");
    restore_source_resolution_review(report, &bytes).is_err()
}

fn item(key: &str, version: u64, item_type: &str, parent_item: &str) -> ZoteroItem {
    ZoteroItem {
        key: key.into(),
        version,
        data: ItemData {
            item_type: item_type.into(),
            title: String::new(),
            abstract_note: String::new(),
            doi: String::new(),
            parent_item: parent_item.into(),
            collections: vec![],
            tags: vec![],
        },
    }
}

fn resolution(
    key: &str,
    version: u64,
    item_type: &str,
    parent_item: &str,
    library_version: u64,
    server_id: Option<&str>,
) -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: key.into(),
        item_version: version,
        library_version,
        server_id: server_id.map(str::to_owned),
        item_type: item_type.into(),
        parent_item_key: parent_item.into(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: "The source remains useful independent evidence.".into(),
    }
}

#[test]
fn source_resolution_review_requires_the_exact_pending_snapshot_set() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
            item("PAPER", 40, "journalArticle", ""),
            item("SOURCE", 41, "attachment", ""),
            item("NOTE", 42, "note", "SOURCE"),
        ],
    );

    assert!(matches!(
        prepare_source_resolution_review(&report, vec![]),
        Err(SourceResolutionError::Missing(_))
    ));

    let review = prepare_source_resolution_review(
        &report,
        vec![
            resolution("SOURCE", 41, "attachment", "", 42, Some("local-server")),
            resolution("NOTE", 42, "note", "SOURCE", 42, Some("local-server")),
        ],
    )
    .expect("every pending source has one exact-snapshot resolution");

    assert_eq!(review.library_version, 42);
    assert_eq!(review.resolved_sources.len(), 2);
    assert_eq!(
        review
            .resolved_sources
            .iter()
            .map(|resolution| resolution.item_key.as_str())
            .collect::<Vec<_>>(),
        vec!["NOTE", "SOURCE"]
    );
}

#[test]
fn source_resolution_review_round_trips_owned_json() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![item("SOURCE", 41, "attachment", "")],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![resolution(
            "SOURCE",
            41,
            "attachment",
            "",
            42,
            Some("local-server"),
        )],
    )
    .expect("the exact pending source is resolvable");

    let serialized = serde_json::to_string(&review).expect("review must serialize");
    let decoded = restore_source_resolution_review(&report, serialized.as_bytes())
        .expect("report-bound owned JSON must restore");

    assert_eq!(decoded, review);
}

#[test]
fn source_resolution_review_json_rejects_missing_or_blank_server_identity() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![item("SOURCE", 41, "attachment", "")],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![resolution(
            "SOURCE",
            41,
            "attachment",
            "",
            42,
            Some("local-server"),
        )],
    )
    .expect("the exact pending source is resolvable");
    let serialized = serde_json::to_value(review).expect("review must serialize");

    for server_id in [
        serde_json::Value::Null,
        serde_json::Value::String(" \t".into()),
    ] {
        let mut candidate = serialized.clone();
        candidate["server_id"] = server_id;
        assert!(restored_is_err(&report, candidate));
    }
}

#[test]
fn source_resolution_review_json_rejects_nested_server_identity_mismatch() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![item("SOURCE", 41, "attachment", "")],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![resolution(
            "SOURCE",
            41,
            "attachment",
            "",
            42,
            Some("local-server"),
        )],
    )
    .expect("the exact pending source is resolvable");
    let serialized = serde_json::to_value(review).expect("review must serialize");

    for server_id in [
        serde_json::Value::Null,
        serde_json::Value::String(" \t".into()),
        serde_json::Value::String("other-server".into()),
    ] {
        let mut candidate = serialized.clone();
        candidate["resolved_sources"][0]["server_id"] = server_id;
        assert!(restored_is_err(&report, candidate));
    }
}

#[test]
fn source_resolution_review_json_rejects_nested_library_version_mismatch() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![item("SOURCE", 41, "attachment", "")],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![resolution(
            "SOURCE",
            41,
            "attachment",
            "",
            42,
            Some("local-server"),
        )],
    )
    .expect("the exact pending source is resolvable");
    let mut serialized = serde_json::to_value(review).expect("review must serialize");
    serialized["resolved_sources"][0]["library_version"] = serde_json::json!(43);

    assert!(restored_is_err(&report, serialized));
}

#[test]
fn source_resolution_review_json_rejects_duplicate_unsorted_or_blank_decisions() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
            item("SOURCE", 41, "attachment", ""),
            item("NOTE", 42, "note", ""),
        ],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![
            resolution("SOURCE", 41, "attachment", "", 42, Some("local-server")),
            resolution("NOTE", 42, "note", "", 42, Some("local-server")),
        ],
    )
    .expect("both pending sources are resolved");
    let serialized = serde_json::to_value(review).expect("review must serialize");

    let mut duplicate = serialized.clone();
    duplicate["resolved_sources"][1]["item_key"] = serde_json::json!("NOTE");
    assert!(restored_is_err(&report, duplicate));

    let mut unsorted = serialized.clone();
    unsorted["resolved_sources"]
        .as_array_mut()
        .expect("resolved sources are an array")
        .swap(0, 1);
    assert!(restored_is_err(&report, unsorted));

    let mut unsorted_pending = serialized.clone();
    unsorted_pending["pending_source_item_keys"]
        .as_array_mut()
        .expect("pending source keys are an array")
        .swap(0, 1);
    assert!(restored_is_err(&report, unsorted_pending));

    let mut blank_key = serialized.clone();
    blank_key["resolved_sources"][0]["item_key"] = serde_json::json!(" \t\n");
    assert!(restored_is_err(&report, blank_key));

    let mut blank = serialized;
    blank["resolved_sources"][0]["reason"] = serde_json::json!(" \t\n");
    assert!(restored_is_err(&report, blank));
}

#[test]
fn source_resolution_restore_rejects_report_metadata_and_identity_drift() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![item("SOURCE", 41, "attachment", "")],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![resolution(
            "SOURCE",
            41,
            "attachment",
            "",
            42,
            Some("local-server"),
        )],
    )
    .expect("the exact pending source is resolvable");
    let serialized = serde_json::to_value(review).expect("review must serialize");

    for (field, replacement) in [
        ("zotero_version", serde_json::json!("10.0.2")),
        ("library_version", serde_json::json!(43)),
        ("rule_revision", serde_json::json!("other-rules")),
    ] {
        let mut candidate = serialized.clone();
        candidate[field] = replacement;
        if field == "library_version" {
            candidate["resolved_sources"][0][field] = serde_json::json!(43);
        }
        assert!(restored_is_err(&report, candidate));
    }

    let mut server = serialized.clone();
    server["server_id"] = serde_json::json!("other-server");
    server["resolved_sources"][0]["server_id"] = serde_json::json!("other-server");
    assert!(restored_is_err(&report, server));

    let mut pending = serialized.clone();
    pending["pending_source_item_keys"][0] = serde_json::json!("OTHER");
    pending["expected_source_identities"][0]["item_key"] = serde_json::json!("OTHER");
    pending["resolved_sources"][0]["item_key"] = serde_json::json!("OTHER");
    assert!(restored_is_err(&report, pending));

    let mut identity = serialized.clone();
    identity["expected_source_identities"][0]["item_version"] = serde_json::json!(40);
    assert!(restored_is_err(&report, identity));
}

#[test]
fn source_resolution_restore_rejects_unordered_or_misaligned_expected_identities() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
            item("NOTE", 42, "note", ""),
            item("SOURCE", 41, "attachment", ""),
        ],
    );
    let review = prepare_source_resolution_review(
        &report,
        vec![
            resolution("NOTE", 42, "note", "", 42, Some("local-server")),
            resolution("SOURCE", 41, "attachment", "", 42, Some("local-server")),
        ],
    )
    .expect("pending sources are resolvable");
    let serialized = serde_json::to_value(review).expect("review must serialize");

    let mut unordered = serialized.clone();
    unordered["expected_source_identities"]
        .as_array_mut()
        .expect("expected identities are an array")
        .swap(0, 1);
    assert!(restored_is_err(&report, unordered));

    let mut misaligned = serialized;
    misaligned["expected_source_identities"][0]["item_key"] = serde_json::json!("OTHER");
    assert!(restored_is_err(&report, misaligned));
}

#[test]
fn source_resolution_rejects_duplicate_unknown_stale_and_blank_decisions() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![item("SOURCE", 3, "attachment", "")],
    );
    let exact = resolution("SOURCE", 3, "attachment", "", 7, Some("local-server"));

    let mut duplicate = exact.clone();
    duplicate.reason = "second decision".into();
    assert!(matches!(
        prepare_source_resolution_review(&report, vec![exact.clone(), duplicate]),
        Err(SourceResolutionError::Duplicate(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("OTHER", 3, "attachment", "", 7, Some("local-server"))]
        ),
        Err(SourceResolutionError::Unknown(key)) if key == "OTHER"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 2, "attachment", "", 7, Some("local-server"))]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "note", "", 7, Some("local-server"))]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "attachment", "PARENT", 7, Some("local-server"))]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "attachment", "", 8, Some("local-server"))]
        ),
        Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
    ));

    let mut blank = exact;
    blank.reason = " \t\n".into();
    assert!(matches!(
        prepare_source_resolution_review(&report, vec![blank]),
        Err(SourceResolutionError::BlankReason(key)) if key == "SOURCE"
    ));
}

#[test]
fn source_resolution_rejects_pending_keys_missing_from_retained_inventory() {
    let mut report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![item("SOURCE", 3, "attachment", "")],
    );
    report.unclassified_items.clear();

    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("SOURCE", 3, "attachment", "", 7, Some("local-server"))]
        ),
        Err(SourceResolutionError::MissingInventory(key)) if key == "SOURCE"
    ));
}

#[test]
fn source_resolution_errors_explain_each_rejection() {
    let cases = [
        (
            SourceResolutionError::Missing("SOURCE".into()),
            "pending source lacks resolution: SOURCE",
        ),
        (
            SourceResolutionError::Unknown("SOURCE".into()),
            "resolution is not pending in the report: SOURCE",
        ),
        (
            SourceResolutionError::Duplicate("SOURCE".into()),
            "pending source has duplicate resolutions: SOURCE",
        ),
        (
            SourceResolutionError::Stale("SOURCE".into()),
            "resolution does not match report source identity: SOURCE",
        ),
        (
            SourceResolutionError::BlankReason("SOURCE".into()),
            "source resolution reason is blank: SOURCE",
        ),
        (
            SourceResolutionError::MissingInventory("SOURCE".into()),
            "pending source is absent from report inventory: SOURCE",
        ),
        (
            SourceResolutionError::MissingServerIdentity,
            "report lacks a non-blank Zotero server identity",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn source_resolution_restore_errors_explain_each_rejection() {
    let cases = [
        (
            SourceResolutionRestoreError::InvalidStoredArtifact,
            "stored source-resolution artifact is invalid",
        ),
        (
            SourceResolutionRestoreError::UnboundReport,
            "stored source-resolution artifact is not bound to the report",
        ),
        (
            SourceResolutionRestoreError::SourceResolution(SourceResolutionError::Missing(
                "SOURCE".into(),
            )),
            "pending source lacks resolution: SOURCE",
        ),
    ];
    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
