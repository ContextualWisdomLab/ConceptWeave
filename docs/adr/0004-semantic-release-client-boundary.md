# ADR 0004 — Semantic-release client boundary

- **Status:** Accepted
- **Date:** 2026-09-02
- **Decision owners:** ConceptWeave Governance & Publication and Client Consumption bounded contexts

## Context

Issue #3 requires downstream CWL products to consume governed ConceptWeave releases without importing generation internals. The foundation already separates candidate truth from publication state, but a buyer-facing workflow is incomplete until a consumer can reject an incompatible or non-governed release before it is used.

A client contract must remain useful offline. LLM/provider availability, generator prompts, persistence state, and foreign application databases cannot be prerequisites for deterministic release admission. Conversely, client-side structural checks must not be confused with publication authority, consuming-product authorization, or cryptographic verification that has not actually occurred.

## Decision

Introduce **Client Consumption** as a supporting bounded context and `conceptweave-client` as its first Rust reference implementation.

The initial versioned `semantic_release` public contract carries:

- stable release identity;
- explicit contract and ontology/model versions;
- truth and publication state;
- a declared artifact digest identity;
- provenance references;
- unique stable concept identifiers.

`SemanticReleaseClient` admits authoritative use only when the release contract version exactly matches the supported version and the release is both `Published` and `Authoritative`. The check is deterministic and performs no network, model, database, source-system, or consumer-authorization work.

The public Draft 2020-12 JSON Schema mirrors the structural invariants. `ReleaseDigest` accepts only `sha256:<64 hex>` as the declared digest identity. This is **not** a cryptographic integrity claim: a separate future adapter must hash the exact serialized release bytes and compare that digest before integrity is established.

The generation-to-client seam is a versioned public contract. Client code may use public domain value types such as `TruthStatus`, `PublicationState`, and `EvidenceReference`, but may not import generator-private classes, prompts, provider payloads, persistence tables, or orchestration state.

Consuming products keep tenant/purpose authorization, business-domain truth, and physical query execution. ConceptWeave returns semantic contracts/query plans; it does not become a foreign product's data plane.

LLM-assisted future `match`, ambiguity explanation, and candidate ranking operations must use `ContextualWisdomLab/contextual-orchestrator`. Their outputs remain candidate/evidence state. `validate`, contract compatibility, digest verification, publication-state checks, and authorization remain deterministic.

## Consequences

### Positive

- consumers can fail closed before authoritative use without an LLM provider;
- stable release contracts prevent generator-private implementation leakage;
- truth/publication authority remains explicit across repository boundaries;
- digest syntax and actual integrity verification cannot be accidentally conflated;
- GRC and other downstream consumers can build ACLs against one stable seam.

### Costs and deferred work

- current compatibility is exact-version only; older-supported compatibility and deprecation policy remain #3 work;
- current digest value validates identity syntax only; exact serialized-byte hashing/signature verification remains required before integrity claims;
- release diff, supersession/staleness policy, match/resolve/explain/query-plan operations remain #3 work;
- language-neutral generated bindings remain deferred until the JSON contract is stable enough to justify them.

## Alternatives rejected

1. **Let consumers import generator internals.** Rejected because it couples downstream products to prompts/adapters/persistence and destroys the reuse boundary.
2. **Require an LLM call to decide release usability.** Rejected because compatibility, governance state, and integrity are deterministic security controls.
3. **Treat a well-shaped digest string as proof of artifact integrity.** Rejected because syntax validation does not hash bytes.
4. **Move downstream authorization into ConceptWeave.** Rejected because tenant/purpose authorization belongs to each consuming product and its identity/control plane.

## Verification

- Rust integration tests cover authoritative admission, unsupported versions, non-Published and non-Authoritative states, provenance/identity requirements, duplicate concepts, and digest syntax.
- Error-message tests keep failures actionable.
- JSON Schema fixtures cover valid release, published/non-authoritative rejection, duplicate concept rejection, and malformed digest rejection.
- Product CI validates both Rust and JSON contracts on the exact PR head.
