# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Fixed

- Golden-set evaluation now ordinary/non-force adopts the current Research Intake parent without reopening its private constructor-bound `ClassificationReport` aggregate.
- Raw JSON snapshot binding uses an explicit `CapturedZoteroItem` boundary instead of adding a mandatory caller-owned `source_record` field to the stable public `ZoteroItem` shape.
- Caller-constructible raw JSON receipts no longer use a provider-authentication label; `conceptweave-zotero-captured-json-snapshot-v3` binds complete raw content plus typed classifier input while reserving source authentication for a transport-owned attestation boundary.
- Research evaluation rejects incomplete source inventories and invalidates prior approvals when retained source metadata changes.
- Research reports retain standalone files and notes that previously disappeared from the classification view, and flag sources whose parent relationships remain unresolved.
- Zotero research intake rejects a read whose records claim revisions newer than the library being observed, without dropping papers or changing their recorded revisions.
- Zotero research intake rejects incomplete or late results after a five-minute read budget, even when individual pages arrive within their request limits.

### Added

- Golden-set source/proposal receipts use versioned SHA-256 domains and aggregate-only evaluation evidence; raw-captured and typed-fixture snapshot receipts are deliberately distinct without implying provider origin.
- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.

### Security

- Captured raw-source receipts bind complete supplied metadata and actual classifier inputs through a private capture object but do not authenticate where caller-supplied bytes originated; typed offline fixtures remain in a separate domain.
- Golden-set evaluation rejects changed predictions or evidence under an earlier approval. Proposal-bound approvals must be reissued; aggregate receipts identify the actual evaluated proposal run.
- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in the core domain crate.
