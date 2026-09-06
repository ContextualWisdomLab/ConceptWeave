# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Fixed

- Zotero research intake rejects a read whose records claim revisions newer than the library being observed, without dropping papers or changing their recorded revisions.
- Zotero research intake rejects incomplete or late results after a five-minute read budget, even when individual pages arrive within their request limits.

### Added

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in the core domain crate.
