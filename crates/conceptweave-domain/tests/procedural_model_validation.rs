//! Contract tests for the unpublished procedural semantic/topology validator.
//!
//! The explicit path compiles the real candidate module without exporting an
//! unverified package API. Replace it with the public export only after native
//! tests, documentation, coverage and the transport projection are accepted.
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
fn annotations() -> LocaleAnnotationsView<'static> {
    LocaleAnnotationsView {
        ko: None,
        en: Some("validated authoring text"),
        ja: None,
        zh: None,
        vi: None,
        es: None,
        de: None,
        fr: None,
    }
}
fn node<'a>(procedure_id: &'a str, evidence: &'a [EvidenceReference]) -> ProcedureNodeView<'a> {
    ProcedureNodeView {
        procedure_id,
        procedure_kind: ProcedureKind::ReasoningStep,
        locale_labels: annotations(),
        semantic_refs: None,
        tool_contract_ref: None,
        source_evidence: evidence,
    }
}
fn relation<'a>(
    source_procedure_id: &'a str,
    relation_type: ProceduralRelationKind,
    target_procedure_id: &'a str,
    evidence: &'a [EvidenceReference],
) -> ProcedureRelationView<'a> {
    ProcedureRelationView {
        source_procedure_id,
        relation_type,
        target_procedure_id,
        condition: annotations(),
        guidance: annotations(),
        pitfalls: annotations(),
        source_evidence: evidence,
    }
}
fn nodes(evidence: &[EvidenceReference]) -> [ProcedureNodeView<'_>; 3] {
    ["observe_source", "discover_model", "review_model"].map(|procedure_id| node(procedure_id, evidence))
}
fn edges(evidence: &[EvidenceReference]) -> [ProcedureRelationView<'_>; 2] {
    [
        relation(
            "observe_source",
            ProceduralRelationKind::LeadsTo,
            "discover_model",
            evidence,
        ),
        relation(
            "discover_model",
            ProceduralRelationKind::Enables,
            "review_model",
            evidence,
        ),
    ]
}
fn model<'a>(
    evidence: &'a [EvidenceReference],
    nodes: &'a [ProcedureNodeView<'a>],
    edges: &'a [ProcedureRelationView<'a>],
) -> ProceduralModelView<'a> {
    ProceduralModelView {
        scope: scope(),
        entry_procedure_id: "observe_source",
        source_evidence: evidence,
        procedure_nodes: nodes,
        procedure_relations: edges,
    }
}
fn check(model: &ProceduralModelView<'_>) -> Result<TopologySummary, ProceduralValidationError> {
    validate_procedural_model(model, scope(), ReachabilityRule::RequireEntryReachability)
}

#[test]
fn connected_model_has_count_evidence_not_publication_authority() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    let edges = edges(&evidence);
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)).unwrap(),
        TopologySummary {
            procedure_count: 3,
            relation_count: 2,
            reachable_count: 3,
        }
    );
}
#[test]
fn missing_entry_is_not_admitted() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    let edges = edges(&evidence);
    let mut model = model(&evidence, &nodes, &edges);
    model.entry_procedure_id = "missing_entry";
    assert_eq!(check(&model), Err(ProceduralValidationError::MissingEntry));
}
#[test]
fn both_missing_endpoint_directions_are_rejected() {
    for missing_source in [true, false] {
        let evidence = evidence();
        let nodes = nodes(&evidence);
        let mut edges = edges(&evidence);
        if missing_source {
            edges[0].source_procedure_id = "absent_node";
        } else {
            edges[0].target_procedure_id = "absent_node";
        }
        assert_eq!(
            check(&model(&evidence, &nodes, &edges)),
            Err(ProceduralValidationError::DanglingRelation)
        );
    }
}
#[test]
fn equal_node_identity_is_rejected_even_with_different_evidence() {
    let mut evidence = evidence();
    evidence.push(
        EvidenceReference::new(
            "source_snapshot",
            format!("sha256:{}", "a".repeat(64)),
            "section_3",
        )
        .unwrap(),
    );
    let mut nodes = nodes(&evidence);
    let edges = edges(&evidence);
    nodes[1].source_evidence = &evidence[1..];
    nodes[1].procedure_id = nodes[0].procedure_id;
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::DuplicateProcedure)
    );
}
#[test]
fn duplicate_relation_identity_is_rejected() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    let mut edges = edges(&evidence);
    edges[1] = edges[0];
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::DuplicateRelation)
    );
}
#[test]
fn cycles_and_parallel_distinct_relations_are_valid_advisory_structure() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    let mut edges = edges(&evidence).to_vec();
    edges.push(relation(
        "review_model",
        ProceduralRelationKind::Requires,
        "observe_source",
        &evidence,
    ));
    let mut parallel = edges[0];
    parallel.relation_type = ProceduralRelationKind::Requires;
    edges.push(parallel);
    assert_eq!(
        check(&model(&evidence, &nodes, &edges))
            .unwrap()
            .relation_count,
        4
    );
}
#[test]
fn reachability_policy_does_not_turn_empty_skeleton_into_a_global_dag_rule() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    let model = model(&evidence, &nodes, &[]);
    assert_eq!(
        check(&model),
        Err(ProceduralValidationError::UnreachableProcedure)
    );
    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft)
            .unwrap()
            .reachable_count,
        1
    );
}
#[test]
fn scope_mismatch_rejects_each_coordinate_without_trim_or_casefold() {
    for index in 0..4 {
        let evidence = evidence();
        let nodes = nodes(&evidence);
        let edges = edges(&evidence);
        let mut model = model(&evidence, &nodes, &edges);
        match index {
            0 => model.scope.model_id = "other_model",
            1 => model.scope.tenant_ref = "other_tenant",
            2 => model.scope.task_type = "other_task",
            _ => model.scope.domain_owner_ref = "other_owner",
        }
        assert_eq!(
            check(&model(&evidence, &nodes, &edges)),
            Err(ProceduralValidationError::ScopeMismatch)
        );
    }
}
#[test]
fn reference_equality_includes_location_and_digest() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    for (digest, location) in [
        ("a".repeat(64), "other_section"),
        ("b".repeat(64), "section_2"),
    ] {
        let other = vec![EvidenceReference::new(
            "source_snapshot",
            format!("sha256:{digest}"),
            location,
        )
        .unwrap()];
        let mut edges = edges(&evidence);
        edges[0].source_evidence = &other;
        assert_eq!(
            check(&model(&evidence, &nodes, &edges)),
            Err(ProceduralValidationError::UnboundEvidence)
        );
    }
}
#[test]
fn node_cannot_cite_evidence_outside_root_inventory() {
    let evidence = evidence();
    let mut nodes = nodes(&evidence);
    let edges = edges(&evidence);
    let other = vec![EvidenceReference::new(
        "other_snapshot",
        format!("sha256:{}", "a".repeat(64)),
        "section_2",
    )
    .unwrap()];
    nodes[0].source_evidence = &other;
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::UnboundEvidence)
    );
}
#[test]
fn source_snapshot_identity_cannot_name_two_digests() {
    let mut evidence = evidence();
    evidence.push(
        EvidenceReference::new(
            "source_snapshot",
            format!("sha256:{}", "b".repeat(64)),
            "section_3",
        )
        .unwrap(),
    );
    let nodes = nodes(&evidence);
    let edges = edges(&evidence);
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::ConflictingSourceRevision)
    );
}
#[test]
fn repeated_evidence_coordinate_is_rejected_not_deduplicated_silently() {
    let mut evidence = evidence();
    evidence.push(evidence[0].clone());
    let nodes = nodes(&evidence);
    let edges = edges(&evidence);
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::DuplicateEvidence)
    );
}
#[test]
fn malformed_or_short_digests_are_not_content_identity() {
    for digest in [
        "main".to_string(),
        "sha256:abc".to_string(),
        format!("sha256:{}\n", "a".repeat(64)),
        format!("sha256:{}", "A".repeat(64)),
    ] {
        let evidence = vec![EvidenceReference::new("source_snapshot", digest, "section_2").unwrap()];
        let nodes = nodes(&evidence);
        let edges = edges(&evidence);
        assert_eq!(
            check(&model(&evidence, &nodes, &edges)),
            Err(ProceduralValidationError::InvalidEvidence)
        );
    }
}
#[test]
fn absent_evidence_at_every_projection_boundary_is_rejected() {
    let evidence = evidence();
    let mut nodes = nodes(&evidence);
    let mut edges = edges(&evidence);
    assert_eq!(
        check(&model(&[], &nodes, &edges)),
        Err(ProceduralValidationError::CollectionLimit)
    );
    nodes[0].source_evidence = &[];
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::CollectionLimit)
    );
    nodes[0].source_evidence = &evidence;
    edges[0].source_evidence = &[];
    assert_eq!(
        check(&model(&evidence, &nodes, &edges)),
        Err(ProceduralValidationError::CollectionLimit)
    );
}
#[test]
fn identifier_rules_match_draft_grammar_and_reject_line_terminators() {
    for identity in ["", " node", "node\n", "절차", "node space"] {
        let evidence = evidence();
        let mut nodes = nodes(&evidence);
        let edges = edges(&evidence);
        nodes[0].procedure_id = identity;
        assert_eq!(
            check(&model(&evidence, &nodes, &edges)),
            Err(ProceduralValidationError::InvalidIdentity)
        );
    }
    let evidence = evidence();
    let mut nodes = nodes(&evidence);
    nodes[0].procedure_id = "_step@run+attempt=1#entry";
    let mut model = model(&evidence, &nodes[..1], &[]);
    model.entry_procedure_id = nodes[0].procedure_id;
    assert!(check(&model).is_ok());
}
#[test]
fn collection_limits_are_checked_before_traversal() {
    let evidence = evidence();
    let nodes = nodes(&evidence);
    let edges = edges(&evidence);
    assert_eq!(
        check(&model(&evidence, &[], &edges)),
        Err(ProceduralValidationError::CollectionLimit)
    );
    assert_eq!(
        check(&model(&evidence, &vec![nodes[0]; 257], &edges)),
        Err(ProceduralValidationError::CollectionLimit)
    );
    assert_eq!(
        check(&model(&evidence, &nodes, &vec![edges[0]; 513])),
        Err(ProceduralValidationError::CollectionLimit)
    );
    assert_eq!(
        check(&model(
            &vec![evidence[0].clone(); 65],
            &nodes,
            &edges,
        )),
        Err(ProceduralValidationError::CollectionLimit)
    );
}
#[test]
fn record_order_does_not_change_validation_counts() {
    let evidence = evidence();
    let mut nodes = nodes(&evidence);
    let mut edges = edges(&evidence);
    let first = check(&model(&evidence, &nodes, &edges)).unwrap();
    nodes.reverse();
    edges.reverse();
    assert_eq!(check(&model(&evidence, &nodes, &edges)).unwrap(), first);
}
#[test]
fn evidence_location_rejects_nul_and_excessive_text() {
    for location in ["section\0private".to_string(), "x".repeat(2049)] {
        let evidence = vec![EvidenceReference::new(
            "source_snapshot",
            format!("sha256:{}", "a".repeat(64)),
            location,
        )
        .unwrap()];
        let nodes = nodes(&evidence);
        let edges = edges(&evidence);
        assert_eq!(
            check(&model(&evidence, &nodes, &edges)),
            Err(ProceduralValidationError::InvalidEvidence)
        );
    }
}

#[test]
fn cumulative_evidence_budget_is_not_reset_per_node() {
    let evidence: Vec<_> = (0..8)
        .map(|i| {
            EvidenceReference::new(
                "source_snapshot",
                format!("sha256:{}", "a".repeat(64)),
                format!("{i}{}", "x".repeat(2047)),
            )
            .unwrap()
        })
        .collect();
    let identities: Vec<_> = (0..64).map(|i| format!("procedure_{i}")).collect();
    let nodes: Vec<_> = identities
        .iter()
        .map(|identity| node(identity, &evidence))
        .collect();
    let mut model = model(&evidence, &nodes, &[]);
    model.entry_procedure_id = &identities[0];
    assert_eq!(
        validate_procedural_model(&model, scope(), ReachabilityRule::AllowPartialDraft),
        Err(ProceduralValidationError::EvidenceBudgetExceeded)
    );
}
#[test]
fn maximum_node_boundary_supports_a_complete_nonrecursive_chain() {
    let evidence = evidence();
    let identities: Vec<_> = (0..256).map(|i| format!("procedure_{i}")).collect();
    let nodes: Vec<_> = identities
        .iter()
        .map(|identity| node(identity, &evidence))
        .collect();
    let edges: Vec<_> = identities
        .windows(2)
        .map(|pair| {
            relation(
                &pair[0],
                ProceduralRelationKind::LeadsTo,
                &pair[1],
                &evidence,
            )
        })
        .collect();
    let mut model = model(&evidence, &nodes, &edges);
    model.entry_procedure_id = &identities[0];
    assert_eq!(check(&model).unwrap().reachable_count, 256);
}
