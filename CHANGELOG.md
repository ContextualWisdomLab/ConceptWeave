# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Added

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Rust-first `conceptweave-observation` contract for immutable PostgreSQL schema snapshots with exact qualified identifiers, deterministic source ordering, canonical lowercase `sha256:<64 hex>` snapshot identity, snapshot/extractor/time evidence, and fail-closed duplicate or blank metadata validation.
- Immutable PostgreSQL primary-key, unique-constraint, and foreign-key observations with exact composite-column order, cross-schema referenced coordinates, deterministic table binding, and fail-closed duplicate/unknown/mismatched constraint evidence.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in owned domain and source-observation contract crates.
