# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Added

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Rust-first `conceptweave-client` supporting subdomain with deterministic offline semantic-release admission by contract version, publication state, truth status, provenance, stable concept identity, and declared SHA-256 digest identity.
- Draft 2020-12 `semantic-release` public JSON Schema with valid and fail-closed fixtures for non-authoritative publication, duplicate concept identifiers, and malformed digest identity.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering/matching research.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Client authoritative-use admission rejects incompatible, unpublished, or non-authoritative releases without requiring a network/model call.
- Declared digest syntax validation is explicitly separated from future cryptographic byte verification to prevent false integrity claims.
- Unsafe Rust is forbidden in the core domain and client crates.
