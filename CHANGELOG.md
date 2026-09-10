# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Added

- Proposed procedural-model engineering profile: local draft/revision input schemas, structural conformance fixtures and existing Product AJV wiring. ConceptWeave owns authoring/validation/publication; Noema retains runtime guidance. No Rust semantic validator, model call, release or activation is delivered by this input-contract slice. Issue #42; ADR-PG-20260910.

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in the core domain crate.
