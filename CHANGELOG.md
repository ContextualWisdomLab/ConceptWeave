# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Fixed

- Failed write responses no longer turn later observations into proof of completion or permission to undo another change; the submitted request and unresolved result remain available for investigation.
- Every execution outcome retains the identity of the reviewed supporting evidence.

- Research change plans reject changed supporting evidence and incomplete inventories before approval, and preserve the identity of the reviewed evidence.

- Duplicate review rejects incomplete source inventories and changed supporting evidence before approval, while retaining reversible identity mappings.
- Research evaluation rejects incomplete source inventories and invalidates prior approvals when retained source metadata changes.
- Research reports retain standalone files and notes that previously disappeared from the classification view, and flag sources whose parent relationships remain unresolved.
- Zotero research intake rejects a read whose records claim revisions newer than the library being observed, without dropping papers or changing their recorded revisions.
- Zotero research intake rejects incomplete or late results after a five-minute read budget, even when individual pages arrive within their request limits.

### Added

- Private, replayable paper-text capture for later research review, preserving unavailable material and leaving earlier reports and approvals unchanged.

- Full-library research-source audit separating available text, incomplete indexing and missing material from reviewed classification; no paper is excluded because its abstract or text is unavailable.
- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.
- Read-only delayed reconciliation receipts for indeterminate Zotero rollback operations.
- Minimal, nonduplicated local abstract context for Zotero items that require steward classification.
- Owner-only file permissions for sensitive local Zotero classification reports.
- A complete-review evaluator that rejects partial steward labels as full reclassification evidence.
- A snapshot-bound steward worksheet with one blank decision per bibliographic item and no duplicated bibliographic text.
- An explicit `--worksheet` CLI mode that writes the live worksheet with owner-only report protections.
- Fail-closed conversion from a fully decided worksheet to the existing externally verified golden-set boundary.
- Lossless owner-only classification-report deserialization for offline review finalization.
- Context-bound validation and owner-only application of completed steward review batches.

### Security

- Local research requests bypass environment-configured proxies. This prevents unintended proxy forwarding; local peer authentication remains an explicit release limitation.

- Source receipts bind complete captured metadata and actual classifier inputs; earlier report and review artifacts require regeneration under the versioned digest representation.
- Golden-set evaluation rejects changed predictions or evidence under an earlier approval. Proposal-bound approvals must be reissued; aggregate receipts identify the actual evaluated proposal run.
- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in the core domain crate.

### Fixed

- Research reads accept valid responses exactly at their documented size limit while still rejecting oversized, incomplete or invalidly encoded responses.
