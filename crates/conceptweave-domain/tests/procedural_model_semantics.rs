//! Contract tests for schema-significant procedural semantics in the unpublished Rust projection.
//!
//! The predecessor topology-only projection could not satisfy these cases. The current
//! candidate retains the draft contract's semantic fields without treating artifact
//! references, validation success, or structural reachability as authority.
use conceptweave_domain::EvidenceReference;
#[path = "../src/procedural_model_validation.rs"]
mod procedural_model_validation;
use procedural_model_validation::*;

fn evidence() -> Vec<EvidenceReference> {
    vec![EvidenceReference::new(
        "source_snapshot",
        format!("sha256:{}", "a".repeat(64)),
        "section_2",
    )
    .unwrap()]
}

fn scope() -> ProceduralScope<'static> {
    ProceduralScope {
        model_id: "model_one",
        tenant_ref: "tenant_one",
        task_type: "semantic_authoring",
        domain_owner_ref: "ConceptWeave",
    }
}

fn labels<'a>(ko: Option<&'a str>, en: Option<&'a str>) -> LocaleAnnotationsView<'a> {
    LocaleAnnotationsView {
        ko,
        en,
        ja: None,
        zh: None,
        vi: None,
        es: None,
        de: None,
        fr: None,
    }
}

fn artifact<'a>(object_ref: &'a str, digest: &'a str) -> ArtifactReferenceView<'a> {
    ArtifactReferenceView {
        authority_ref: "context-graph-contracts",
        release_ref: "procedural-contract-v1",
        artifact_digest: digest,
        object_ref,
    }
}

#[test]
fn schema_significant_semantics_are_present_in_the_rust_projection() {
    let evidence = evidence();
    let digest = format!("sha256:{}", "b".repeat(64));
    let semantic_refs = [artifact("semantic:customer", &digest)];
    let tool_contract = artifact("tool:catalog.read", &digest);
    let nodes = [
        ProcedureNodeView {
            procedure_id: "observe_source",
            procedure_kind: ProcedureKind::ToolOperation,
            locale_labels: labels(Some("근거 관찰"), Some("Observe evidence")),
            semantic_refs: Some(&semantic_refs),
            tool_contract_ref: Some(tool_contract),
            source_evidence: &evidence,
        },
        ProcedureNodeView {
            procedure_id: "review_model",
            procedure_kind: ProcedureKind::ReasoningStep,
            locale_labels: labels(Some("모델 검토"), Some("Review model")),
            semantic_refs: None,
            tool_contract_ref: None,
            source_evidence: &evidence,
        },
    ];
    let edges = [ProcedureRelationView {
        source_procedure_id: "observe_source",
        relation_type: ProceduralRelationKind::LeadsTo,
        target_procedure_id: "review_model",
        condition: labels(Some("근거가 수집됨"), Some("Evidence is captured")),
        guidance: labels(Some("출처를 유지"), Some("Preserve provenance")),
        pitfalls: labels(Some("권한으로 승격 금지"), Some("Do not infer authority")),
        source_evidence: &evidence,
    }];
    let model = ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "observe_source",
        source_evidence: &evidence,
        procedure_nodes: &nodes,
        procedure_relations: &edges,
    };

    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::RequireEntryReachability)
            .unwrap(),
        TopologySummary {
            procedure_count: 2,
            relation_count: 1,
            reachable_count: 2,
        }
    );
}

#[test]
fn tool_operation_requires_a_tool_contract_reference() {
    let evidence = evidence();
    let nodes = [ProcedureNodeView {
        procedure_id: "observe_source",
        procedure_kind: ProcedureKind::ToolOperation,
        locale_labels: labels(None, Some("Observe evidence")),
        semantic_refs: None,
        tool_contract_ref: None,
        source_evidence: &evidence,
    }];
    let model = ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "observe_source",
        source_evidence: &evidence,
        procedure_nodes: &nodes,
        procedure_relations: &[],
    };

    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
        Err(ProceduralValidationError::MissingToolContract)
    );
}

#[test]
fn locale_annotations_match_canonical_cross_runtime_nonblank_semantics() {
    for bad in ["", "   ", "\u{feff}", "\u{0085}", "embedded\0nul"] {
        let evidence = evidence();
        let nodes = [ProcedureNodeView {
            procedure_id: "review_model",
            procedure_kind: ProcedureKind::ReasoningStep,
            locale_labels: labels(None, Some(bad)),
            semantic_refs: None,
            tool_contract_ref: None,
            source_evidence: &evidence,
        }];
        let model = ProceduralModelView {
            scope: scope(),
            entry_procedure_id: "review_model",
            source_evidence: &evidence,
            procedure_nodes: &nodes,
            procedure_relations: &[],
        };
        assert_eq!(
            validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
            Err(ProceduralValidationError::InvalidAnnotation)
        );
    }
}

#[test]
fn evidence_locations_match_canonical_ecmascript_non_whitespace_semantics() {
    let evidence = vec![EvidenceReference::new(
        "source_snapshot",
        format!("sha256:{}", "a".repeat(64)),
        "\u{feff}",
    )
    .expect("base EvidenceReference currently permits U+FEFF; procedural validation must enforce its schema")];
    let nodes = [ProcedureNodeView {
        procedure_id: "review_model",
        procedure_kind: ProcedureKind::ReasoningStep,
        locale_labels: labels(None, Some("Review model")),
        semantic_refs: None,
        tool_contract_ref: None,
        source_evidence: &evidence,
    }];
    let model = ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "review_model",
        source_evidence: &evidence,
        procedure_nodes: &nodes,
        procedure_relations: &[],
    };

    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
        Err(ProceduralValidationError::InvalidEvidence)
    );
}

#[test]
fn present_empty_semantic_refs_is_rejected_instead_of_collapsing_to_absence() {
    let evidence = evidence();
    let nodes = [ProcedureNodeView {
        procedure_id: "review_model",
        procedure_kind: ProcedureKind::ReasoningStep,
        locale_labels: labels(None, Some("Review model")),
        semantic_refs: Some(&[]),
        tool_contract_ref: None,
        source_evidence: &evidence,
    }];
    let model = ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "review_model",
        source_evidence: &evidence,
        procedure_nodes: &nodes,
        procedure_relations: &[],
    };

    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
        Err(ProceduralValidationError::CollectionLimit)
    );
}

#[test]
fn semantic_artifact_references_are_bounded_well_formed_and_unique() {
    let evidence = evidence();
    let digest = format!("sha256:{}", "c".repeat(64));
    let semantic = artifact("semantic:customer", &digest);
    let duplicates = [semantic, semantic];
    let nodes = [ProcedureNodeView {
        procedure_id: "review_model",
        procedure_kind: ProcedureKind::ReasoningStep,
        locale_labels: labels(None, Some("Review model")),
        semantic_refs: Some(&duplicates),
        tool_contract_ref: None,
        source_evidence: &evidence,
    }];
    let model = ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "review_model",
        source_evidence: &evidence,
        procedure_nodes: &nodes,
        procedure_relations: &[],
    };
    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
        Err(ProceduralValidationError::DuplicateArtifactReference)
    );

    let invalid = [artifact("semantic:customer", "sha256:abc")];
    let nodes = [ProcedureNodeView {
        procedure_id: "review_model",
        procedure_kind: ProcedureKind::ReasoningStep,
        locale_labels: labels(None, Some("Review model")),
        semantic_refs: Some(&invalid),
        tool_contract_ref: None,
        source_evidence: &evidence,
    }];
    let model = ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "review_model",
        source_evidence: &evidence,
        procedure_nodes: &nodes,
        procedure_relations: &[],
    };
    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
        Err(ProceduralValidationError::InvalidArtifactReference)
    );
}
