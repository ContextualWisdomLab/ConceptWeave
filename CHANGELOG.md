# Changelog

All notable changes to ConceptWeave are documented here.

## Unreleased

### Added

- Initial ConceptWeave product, DDD, security, test, and operability baselines.
- Rust 1.98.0 `conceptweave-domain` foundation with evidence-bound semantic candidate contracts.
- Fail-closed Draft -> Proposed -> Validated -> Reviewed -> Published lifecycle with explicit rejection and supersession.
- Draft 2020-12 JSON Schema for the semantic-candidate public contract.
- Rust-first `conceptweave-client` supporting subdomain with deterministic offline semantic-release admission by contract version, publication state, truth status, provenance, stable concept identity, and declared SHA-256 digest identity.
- Deterministic offline semantic-release diff that first applies the same authoritative-use admission policy, then reports stable previous/current release identity and sorted added/removed concept identifiers without network or model calls.
- Exact offline SHA-256 verification of caller-supplied serialized semantic-release bytes, with typed digest-mismatch evidence and the same fail-closed authoritative-use admission gate.
- Explicit semantic-release compatibility policy that distinguishes the current contract version, caller-declared supported legacy versions, and unknown versions without inferring compatibility from version ordering; supported legacy releases still pass the same Published/Authoritative gate.
- Draft 2020-12 `semantic-release` public JSON Schema with valid and fail-closed fixtures for non-authoritative publication, duplicate concept identifiers, and malformed digest identity.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering/matching research.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Client authoritative-use admission rejects incompatible, unpublished, or non-authoritative releases without requiring a network/model call.
- Legacy compatibility is explicit opt-in policy; unknown versions remain fail-closed and the current version cannot also be configured as legacy.
- Release diff validates both compared releases through the same fail-closed authoritative-use gate so comparison cannot bypass contract-version, publication-state, or truth-status policy.
- Serialized-artifact integrity verification first applies authoritative-use admission, then computes SHA-256 over the exact supplied bytes and rejects any mismatch with the declared release digest.
- Digest syntax validation remains distinct from byte verification so a syntactically valid digest is never treated as proof that serialized content matches it.
- Unsafe Rust is forbidden in the core domain and client crates.
