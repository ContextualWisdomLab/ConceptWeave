use conceptweave_zotero::{
    ItemData, PendingSourceResolution, SourceResolutionDisposition, SourceResolutionError,
    ZoteroItem, classify_snapshot, prepare_source_resolution_review,
};

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
    reason: &str,
) -> PendingSourceResolution {
    PendingSourceResolution {
        item_key: key.into(),
        item_version: version,
        library_version,
        item_type: item_type.into(),
        parent_item_key: parent_item.into(),
        disposition: SourceResolutionDisposition::RetainStandaloneEvidence,
        reason: reason.into(),
    }
}

#[test]
fn source_resolution_requires_a_complete_exact_pending_set() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        42,
        vec![
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
            resolution("NOTE", 42, "note", "SOURCE", 42, "keep"),
            resolution("SOURCE", 41, "attachment", "", 42, "keep"),
        ],
    )
    .expect("exact pending sources resolve");
    assert_eq!(review.resolved_sources.len(), 2);
    assert_eq!(review.resolved_sources[0].item_key, "NOTE");
}

#[test]
fn source_resolution_rejects_duplicate_unknown_stale_and_blank_decisions() {
    let report = classify_snapshot(
        "10.0.1".into(),
        Some("local-server".into()),
        7,
        vec![item("SOURCE", 3, "attachment", "")],
    );
    let exact = resolution("SOURCE", 3, "attachment", "", 7, "keep");
    assert!(matches!(
        prepare_source_resolution_review(&report, vec![exact.clone(), exact.clone()]),
        Err(SourceResolutionError::Duplicate(key)) if key == "SOURCE"
    ));
    assert!(matches!(
        prepare_source_resolution_review(
            &report,
            vec![resolution("OTHER", 3, "attachment", "", 7, "keep")]
        ),
        Err(SourceResolutionError::Unknown(key)) if key == "OTHER"
    ));
    for stale in [
        resolution("SOURCE", 2, "attachment", "", 7, "keep"),
        resolution("SOURCE", 3, "note", "", 7, "keep"),
        resolution("SOURCE", 3, "attachment", "PARENT", 7, "keep"),
        resolution("SOURCE", 3, "attachment", "", 8, "keep"),
    ] {
        assert!(matches!(
            prepare_source_resolution_review(&report, vec![stale]),
            Err(SourceResolutionError::Stale(key)) if key == "SOURCE"
        ));
    }
    assert!(matches!(
        prepare_source_resolution_review(&report, vec![resolution("SOURCE", 3, "attachment", "", 7, " ")]),
        Err(SourceResolutionError::BlankReason(key)) if key == "SOURCE"
    ));
}

#[test]
fn source_resolution_errors_remain_explainable() {
    let messages = [
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
    ];
    for (error, expected) in messages {
        assert_eq!(error.to_string(), expected);
    }
}
