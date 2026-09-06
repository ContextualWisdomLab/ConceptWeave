# Test Strategy

## Foundation gates

- Rust formatting and Clippy with warnings denied;
- unit/integration tests for every owned domain/client behavior branch;
- owned production line/function/region and LLVM branch coverage target of 100%;
- Draft 2020-12 JSON Schema syntax and positive/negative fixture validation for public contracts;
- lockfile freshness and clean-tree verification;
- public Rust documentation with `missing_docs` denied;
- every CI result is valid only for the unchanged exact PR head.

## Current Source Observation contract tests

- `ObservationRequestBudget` rejects caller-requested schema-count ceilings above 4,096 and retained schema-name bytes above 1,048,576 with typed over-cap errors before registry/database access; exact-cap and ordinary narrower controls remain constructible;
- request metadata rejects blank/malformed source keys, empty/blank/duplicate exact schema names, zero limits, and schema metadata outside the caller-requested narrower structural count/byte envelope before registry/database access;
- source resolution requires a registered key plus bounded opaque immutable connection-policy binding and rejects connection material masquerading as a binding;
- source-key recognition alone cannot authorize schema scope; exact schema policy defaults to deny and is case/normalization preserving;
- source+schema authorization alone cannot authorize resources; complete `ObservationResourceEnvelope` policy defaults to deny;
- wider-than-policy schema-count/schema-byte/operation/statement/row/byte/concurrency requests return `UnauthorizedResourceEnvelope` before adapter/source/snapshot side effects;
- requests equal to or narrower than every local source-policy ceiling are explicitly admitted and preserve the exact requested envelope;
- source/binding/schema/resource local policy work shares one monotonic operation budget; elapsed authorization reduces the adapter remainder and exhaustion wins before side effects;
- a capability for binding A presented after live mapping changes to B fails before source/snapshot side effects, while unchanged A executes the expected control once;
- `AuthorizedObservationRequest` is non-`Clone` and is consumed by `SourceObservationPort::observe`; cancellation and success controls obtain separate authorizations so one policy grant cannot be replayed to multiply source/resource work;
- the awaitable `Send` port preserves cancellation and typed resource/source failures without adding a runtime dependency to the port crate;
- immutable PostgreSQL snapshot construction requires the complete authorized envelope, rejects locally observed schemas outside the exact scope, and keeps foreign-key target schema names as relationship evidence rather than read authority;
- snapshot and receipt provenance retain the exact immutable connection-policy binding separately from deterministic source-content digest identity;
- PostgreSQL observation value objects preserve exact identifiers, ordering, FK action/match/deferrability/validation/enforcement evidence, CHECK reconstruction/status, strict UTC provenance and owner-computed deterministic digest identity.

These contract fixtures are not runtime GREEN by existence alone. A concrete adapter and the first release candidate require one unchanged exact head to pass Rust 1.98 tests, fmt, strict Clippy, warnings-denied rustdoc, release build, owned 100% coverage, applicable security/dependency gates, and independent review.

## Current Client Consumption tests

- authoritative + Published release admits offline for the exact current or explicitly supported legacy contract version;
- Reviewed/unpublished and Published/non-Authoritative releases fail closed;
- unsupported contract versions fail closed without version-order inference;
- release/contract/ontology identifiers reject blank values;
- provenance is required;
- concept identifiers reject blanks and duplicates;
- declared digest identity rejects unsupported algorithm, wrong length, uppercase and non-hex payloads;
- `SemanticReleaseClient::verify_detached_artifact` hashes exact detached immutable semantic-artifact bytes only after authoritative-use admission;
- exact bytes pass only when SHA-256 equals the declared artifact digest; changed/truncated/wrong bytes fail with typed mismatch evidence;
- release diff admits both releases and returns deterministic sorted concept changes;
- exact concept resolution is provider-independent and rejects blank identifiers;
- supersession validation binds distinct predecessor/successor release ids and digests, rejects self-supersession and blank rationale, and applies ordinary admission to both releases;
- public error messages identify the rejected admission/integrity/supersession invariant;
- JSON Schema fixtures mirror Published -> Authoritative, unique concepts, provenance and digest-shape constraints.

Digest identity syntax and detached-byte integrity remain separate controls. The current verifier does not claim signature authenticity or provenance-chain trust. Those cases require a stable Governance & Publication signing contract before they become release gates.

## Future product test families

### Source observation runtime

A frozen anonymized PostgreSQL fixture must exercise real least-privilege exact-binding credential resolution, stale-binding rejection before credential/source access, one fresh authorization per attempted observation/retry, `REPEATABLE READ READ ONLY`, exact-schema `pg_catalog` capture, operation/statement/row/byte/concurrency enforcement from the policy-admitted envelope, cancellation cleanup, source disappearance, complete-or-fail snapshot construction, domains/enums/indexes/comments, quoted identifiers and cross-schema collisions. OpenAPI/AsyncAPI fixtures, malformed contracts, deep nesting, invalid encoding, archive bombs, parser cancellation, and exact digest/location provenance follow behind their own adapters.

### Ontology and semantic discovery

Golden concept/type/taxonomy/relation sets; mapping precision/recall; multilingual labels; synonyms/homonyms; false friends; unrelated sources; cross-domain collisions; explicit no-answer cases.

### Client compatibility and alignment

Malformed, partial, conflicting, stale and superseded releases beyond the currently implemented explicit compatibility/diff/supersession contracts; candidate-retrieval recall; OAEI-style matching precision/recall/F1; deterministic preprocessing ablations; abstention/ambiguity handling; LLM-call reduction against naive full-prompt baselines. Optional model calls use `contextual-orchestrator`; no model judge is sole truth.

### Query-plan seam

Golden semantic query plans preserve governed dimensions/measures/relations while physical execution and tenant/purpose authorization remain in the consuming product. Tests must prove no direct foreign application-table SQL or cross-tenant authorization bypass.

### Semantic measures

Exact deterministic calculations, grain correctness, join/cardinality safety, units, null semantics, time windows, currency/unit conversions through approved deterministic layers, and no LLM arithmetic authority.

### Validation/reasoning

OWL consistency where supported, SHACL conformance, cycle constraints, unsatisfiable classes, contradictory ranges/domains, duplicate measures, and bounded reasoner resources.

### Governance

No bypass of Reviewed before Published, immutable published releases, rejection, supersession, stale-review protection, maker-checker requirements where configured, and exact audit receipts.

### Security

Prompt injection, malicious ontology/source/release content, SSRF, cross-tenant leakage, secret leakage, expression injection, resource exhaustion, over-cap structural request metadata, caller-self-authorized schema/resource requests, stale source binding replay, authorized-capability replay amplification, malformed source provenance, hostile export values, compatibility downgrade, stale/superseded use, and detached-artifact tampering.

### Evaluation

Model-backed evaluation must include deterministic fixtures and human-reviewed expert cases. Report extraction recall, semantic precision, structural validity, mapping accuracy, citation/provenance completeness, compatibility correctness, and abstention quality separately rather than collapsing them into one opaque score.