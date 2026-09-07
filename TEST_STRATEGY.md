# Test Strategy

## Foundation gates

- Rust formatting and Clippy with warnings denied;
- unit tests for every domain lifecycle branch;
- owned production line/function/region and LLVM branch coverage target of 100%;
- JSON Schema syntax validation;
- lockfile freshness and clean-tree verification;
- public Rust documentation with `missing_docs` denied.

## Local research capture regressions

The full-text suite covers report admission, exact response retention, parent/item revision binding, independent content versions, missing/empty/partial text, duplicate/foreign manifest rejection, bookend drift, byte/deadline boundaries and replay under changed or recomputed digests. It also distinguishes the 256 MiB aggregate source-body budget from the 512 MiB persisted compact-JSON ceiling: an escape-heavy valid capture fixture stays within a proportionally scaled raw-body budget while its outer JSON grows past the corresponding persisted ceiling, and the size validator rejects that representation while accepting the exact serialized-byte boundary. Synthetic HTTP tests cover headers, network/redirect failures, strict encoding and response limits without touching the running Zotero library. Proxy isolation uses fresh subprocess environments for all six supported proxy variable spellings across the three existing local transport paths. Synthetic text is only a unit/integration fixture; live aggregate evidence is separately recorded in doctoring and never reported as approved labels.

The shared restored-capture verifier also has an exact compact-JSON boundary regression: the existing valid report/capture fixture passes at its serialized byte count and returns the budget error with one byte less allowance. The private limit parameter exposes the real verifier error path without allocating a 512 MiB fixture; the public verifier retains its fixed production ceiling and report-before-size validation order. This scaled test is not live-library or approval evidence.

## Future product test families

### Source observation

Realistic PostgreSQL schema snapshots, OpenAPI/AsyncAPI fixtures, malformed contracts, deep nesting, invalid encoding, duplicate identifiers, archive bombs, parser cancellation, and exact digest/location provenance.

### Ontology and semantic discovery

Golden concept/type/taxonomy/relation sets; mapping precision/recall; multilingual labels; synonyms/homonyms; false friends; unrelated sources; cross-domain collisions; explicit no-answer cases.

### Semantic measures

Exact deterministic calculations, grain correctness, join/cardinality safety, units, null semantics, time windows, currency/unit conversions through approved deterministic layers, and no LLM arithmetic authority.

### Validation/reasoning

OWL consistency where supported, SHACL conformance, cycle constraints, unsatisfiable classes, contradictory ranges/domains, duplicate measures, and bounded reasoner resources.

### Governance

No bypass of Reviewed before Published, immutable published releases, rejection, supersession, stale-review protection, maker-checker requirements where configured, and exact audit receipts.

### Security

Prompt injection, malicious ontology/source content, SSRF, cross-tenant leakage, secret leakage, expression injection, resource exhaustion, replay, malformed source provenance, and hostile export values.

### Evaluation

Model-backed evaluation must include deterministic fixtures and human-reviewed expert cases. Report extraction recall, semantic precision, structural validity, mapping accuracy, citation/provenance completeness, and abstention quality separately rather than collapsing them into one opaque score.
