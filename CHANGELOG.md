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
- Rust-first `conceptweave-client` supporting subdomain with deterministic offline semantic-release admission by contract version, publication state, truth status, provenance, stable concept identity, and declared SHA-256 digest identity.
- Deterministic offline semantic-release diff that first applies the same authoritative-use admission policy, then reports stable previous/current release identity and sorted added/removed concept identifiers without network or model calls.
- Exact offline SHA-256 verification of caller-supplied detached immutable semantic-artifact bytes through `verify_detached_artifact`, with typed digest-mismatch evidence and the same fail-closed authoritative-use admission gate.
- Explicit semantic-release compatibility policy that distinguishes the current contract version, caller-declared supported legacy versions, and unknown versions without inferring compatibility from version ordering; supported legacy releases still pass the same Published/Authoritative gate.
- Explicit immutable semantic-release supersession references that bind predecessor and successor release ids to their exact artifact digests, require a rationale, reject self-supersession, and validate both releases through the ordinary authoritative-use gate without inferring replacement from version order or timestamps.
- Draft 2020-12 `semantic-release` public JSON Schema with valid and fail-closed fixtures for non-authoritative publication, duplicate concept identifiers, and malformed digest identity.
- Standards and research doctoring covering stable W3C ontology standards, 2026 RDF/SHACL work in progress, Apache Ossie, and recent LLM ontology-engineering/matching research.
- Executable documentation/API contract preventing the retired `verify_serialized_artifact` name or manifest-self-digest semantics from drifting back into Client Consumption documentation.

### Security

- Model-generated semantics remain non-authoritative until deterministic validation and authorized review.
- Client authoritative-use admission rejects incompatible, unpublished, or non-authoritative releases without requiring a network/model call.
- Legacy compatibility is explicit opt-in policy; unknown versions remain fail-closed and the current version cannot also be configured as legacy.
- Release diff validates both compared releases through the same fail-closed authoritative-use gate so comparison cannot bypass contract-version, publication-state, or truth-status policy.
- Detached-artifact integrity verification first applies authoritative-use admission, then computes SHA-256 over the exact supplied detached semantic-artifact bytes and rejects any mismatch with the declared release digest.
- Digest syntax validation remains distinct from byte verification so a syntactically valid digest is never treated as proof that detached artifact content matches it.
- Supersession validation requires exact predecessor/successor id-and-digest references and leaves the prior published release immutable; a correction is not inferred from ordering, timestamps, or semantic similarity.
- Unsafe Rust is forbidden in the core domain, client, source-observation, and source-port contract crates.
