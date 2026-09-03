# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Added

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Rust-first `conceptweave-observation` contract for immutable PostgreSQL schema snapshots with exact qualified identifiers, deterministic source ordering, canonical lowercase `sha256:<64 hex>` snapshot identity, snapshot/extractor/time evidence, and fail-closed duplicate or blank metadata validation.
- Immutable PostgreSQL primary-key, unique-constraint, and foreign-key observations with exact composite-column order, cross-schema referenced coordinates, deterministic table binding, and fail-closed duplicate/unknown/mismatched constraint evidence.
- Exact optional PostgreSQL foreign-key reference behavior, preserving observed `ON UPDATE`/`ON DELETE` actions, match type, and deferrability/initial timing without inventing defaults when source behavior was not observed.
- Exact optional PostgreSQL foreign-key validation/enforcement evidence, preserving observed `convalidated` and `conenforced` booleans (including explicit `false`) while retaining `None` when the adapter did not observe those catalog fields.
- PostgreSQL 18 `CHECK` constraint observations preserving the reconstructed source definition plus validation, enforcement, and `NO INHERIT` status without guessing expression-to-column dependencies.
- Rust-first `conceptweave-source-port` contract with positive statement-timeout/row/byte/concurrency limits, exact non-empty schema allowlists, bounded opaque source registry keys, caller cancellation, and typed fail-closed source-disappearance/resource-limit outcomes; a live PostgreSQL adapter remains open work.
- Source registry keys now require at most 128 bytes of lowercase multiword `snake_case`, rejecting raw DSNs, URLs, shell-style connection parameters, generic one-word identifiers, and malformed registry identifiers before adapter credential resolution.
- Source Observation timestamps now fail closed unless they use an explicit canonical UTC `Z` form with a valid Gregorian calendar date and clock value; optional fractional seconds are preserved, and numeric/local offsets are not silently normalized into provenance.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering research.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Unsafe Rust is forbidden in owned domain, source-observation, and source-port contract crates.
