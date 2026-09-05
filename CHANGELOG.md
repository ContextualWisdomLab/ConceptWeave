# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Added

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.
- Read-only delayed reconciliation receipts for indeterminate Zotero rollback operations.
- Minimal, nonduplicated local abstract context for Zotero items that require steward classification.
- Owner-only file permissions for sensitive local Zotero classification reports.
- A complete-review evaluator that rejects partial steward labels as full reclassification evidence.

### Security

- Source receipts bind complete captured metadata and actual classifier inputs; earlier report and review artifacts require regeneration under the versioned digest representation.
- Golden-set evaluation rejects changed predictions or evidence under an earlier approval. Proposal-bound approvals must be reissued; aggregate receipts identify the actual evaluated proposal run.
- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in the core domain crate.
