# Test Strategy

## Foundation gates

- Rust formatting and Clippy with warnings denied;
- unit/integration tests for every owned domain/client behavior branch;
- owned production line/function/region and LLVM branch coverage target of 100%;
- Draft 2020-12 JSON Schema syntax and positive/negative fixture validation for public contracts;
- lockfile freshness and clean-tree verification;
- public Rust documentation with `missing_docs` denied;
- every CI result is valid only for the unchanged exact PR head.

## Current Client Consumption tests

- authoritative + Published release admits offline for the exact supported contract version;
- Reviewed/unpublished and Published/non-Authoritative releases fail closed;
- unsupported contract versions fail closed;
- release/contract/ontology identifiers reject blank values;
- provenance is required;
- concept identifiers reject blanks and duplicates;
- declared digest identity rejects unsupported algorithm, wrong length, and non-hex payloads;
- public error messages identify the rejected admission invariant;
- JSON Schema fixtures mirror Published -> Authoritative, unique concepts, provenance and digest-shape constraints.

Digest identity tests do not claim byte integrity. A future verifier must add golden exact-byte hashing plus one-byte mutation, truncation, serialization/canonicalization, wrong-digest and signature/provenance cases before integrity is release-ready.

## Future product test families

### Source observation

Realistic PostgreSQL schema snapshots, OpenAPI/AsyncAPI fixtures, malformed contracts, deep nesting, invalid encoding, duplicate identifiers, archive bombs, parser cancellation, and exact digest/location provenance.

### Ontology and semantic discovery

Golden concept/type/taxonomy/relation sets; mapping precision/recall; multilingual labels; synonyms/homonyms; false friends; unrelated sources; cross-domain collisions; explicit no-answer cases.

### Client compatibility and alignment

Current, older-supported, unsupported, malformed, partial, conflicting, stale and superseded releases; exact release diff; candidate-retrieval recall; OAEI-style matching precision/recall/F1; deterministic preprocessing ablations; abstention/ambiguity handling; LLM-call reduction against naive full-prompt baselines. Optional model calls use `contextual-orchestrator`; no model judge is sole truth.

### Query-plan seam

Golden semantic query plans preserve governed dimensions/measures/relations while physical execution and tenant/purpose authorization remain in the consuming product. Tests must prove no direct foreign application-table SQL or cross-tenant authorization bypass.

### Semantic measures

Exact deterministic calculations, grain correctness, join/cardinality safety, units, null semantics, time windows, currency/unit conversions through approved deterministic layers, and no LLM arithmetic authority.

### Validation/reasoning

OWL consistency where supported, SHACL conformance, cycle constraints, unsatisfiable classes, contradictory ranges/domains, duplicate measures, and bounded reasoner resources.

### Governance

No bypass of Reviewed before Published, immutable published releases, rejection, supersession, stale-review protection, maker-checker requirements where configured, and exact audit receipts.

### Security

Prompt injection, malicious ontology/source/release content, SSRF, cross-tenant leakage, secret leakage, expression injection, resource exhaustion, replay, malformed source provenance, hostile export values, compatibility downgrade, stale/superseded use, and artifact tampering.

### Evaluation

Model-backed evaluation must include deterministic fixtures and human-reviewed expert cases. Report extraction recall, semantic precision, structural validity, mapping accuracy, citation/provenance completeness, compatibility correctness, and abstention quality separately rather than collapsing them into one opaque score.
