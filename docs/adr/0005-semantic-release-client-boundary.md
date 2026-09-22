# ADR 0005 — Semantic-release client boundary

- **Status:** Proposed
- **Date:** 2026-09-02
- **Decision owners:** ConceptWeave Governance & Publication and Client Consumption bounded contexts
- **Related:** Issue #3, PR #5, `docs/product-technical-gap-baseline.md`

## Context

Issue #3 requires downstream CWL products to consume governed ConceptWeave releases without importing generation internals. The foundation separates candidate truth from publication state, but a buyer-facing workflow is incomplete until a consumer can reject incompatible or non-governed releases, verify exact detached artifact bytes, compare releases, and follow an explicit correction/supersession relation without guessing from version order or timestamps.

The Client contract must remain useful offline. LLM/provider availability, generator prompts, persistence state, and foreign application databases are not prerequisites for deterministic release admission. Client-side validation also must not be confused with publication authority, tenant/purpose authorization, or cryptographic evidence that was not actually verified.

This ADR remains **Proposed** while PR #5 is Draft and exact-head Product/security/review evidence is incomplete. Code on an unintegrated head is decision evidence, not grounds for premature Accepted status.

## Decision

Introduce **Client Consumption** as a Supporting Bounded Context and `conceptweave-client` as its Rust reference implementation.

The versioned `semantic_release` contract carries stable release identity, explicit contract and ontology/model versions, truth/publication state, canonical declared artifact digest identity, provenance references, and unique stable concept identifiers. The v1 public JSON Schema is bound to `contract_version = 1.0.0`; unknown/future versions cannot validate as v1. `SemanticReleaseClient` admits authoritative use only when the release is explicitly compatible and both `Published` and `Authoritative`. Compatibility is never inferred from semantic-version ordering.

`ReleaseDigest` accepts only canonical `sha256:<64 lowercase hex>`. `SemanticReleaseClient::verify_detached_artifact` separately hashes the caller-supplied detached immutable artifact bytes and requires an exact digest match after authoritative-use admission. Digest syntax and byte-integrity evidence remain distinct.

`SemanticReleaseClient::diff` admits both releases through the authoritative-use gate and reports deterministic sorted concept additions/removals. If one stable `release_id` names conflicting immutable release content, diff fails closed instead of representing the conflict as ordinary evolution. Exact concept resolution remains deterministic and performs no fuzzy matching or model call.

For corrections, `SemanticReleaseReference` binds one release id to its exact artifact digest. `ReleaseSupersession` names a distinct predecessor reference, exact successor reference, and nonblank rationale. `validate_supersession` accepts a predecessor that is still Published+Authoritative while replacement is being governed or one that has already moved to the governed Superseded+Superseded lifecycle state; the successor must pass ordinary Published+Authoritative admission. Both id-and-digest references must match exactly. Supersession is never inferred from version order, timestamp, semantic diff, or ontology similarity, and the prior immutable release is never overwritten.

Draft 2020-12 JSON Schema cannot express sibling-field inequality. Therefore the public supersession seam includes `semantic-release-supersession.rules.json`, whose `distinct_release_id` rule is machine-readable and language-neutral, plus a deterministic reference validator and negative fixture. Structural schema validation and semantic cross-field validation are both required conformance steps.

The generation-to-client seam is a versioned public contract. Client code may use public domain value types such as `TruthStatus`, `PublicationState`, and `EvidenceReference`, but may not import generator-private classes, prompts, provider payloads, persistence tables, Source Observation internals, or orchestration state.

Consuming products retain tenant/purpose authorization, business-domain truth, and physical query execution. ConceptWeave returns governed semantic contracts/query plans; it does not become a foreign product's data plane. Any future LLM-assisted match/explain/ranking operation must use released `contextual-orchestrator`; deterministic admission, compatibility, integrity, supersession, publication-state and authorization checks remain outside model authority.

## Consequences

### Positive

- consumers can fail closed before authoritative use without an LLM provider;
- stable public contracts prevent generator-private implementation leakage;
- truth/publication authority remains explicit across repository boundaries;
- digest syntax and actual byte verification cannot be conflated;
- version admission is explicit and fail-closed;
- corrections preserve immutable predecessor releases and exact successor identity;
- structural JSON conformance and cross-field semantic conformance are explicit rather than pretending JSON Schema can express unsupported invariants.

### Costs and deferred work

- signature/provenance-chain verification remains deferred until Governance & Publication defines a stable signing contract;
- typed relation/mapping/dimension/measure resolution, match/align/explain and semantic query-plan operations remain Issue #3 work;
- GRC-shaped reference-client fixtures remain required before buyer-facing integration readiness;
- this ADR cannot advance to Accepted until the stacked implementation is integrated and current-head deterministic/security/review evidence is terminal.

## Alternatives rejected

1. **Let consumers import generator internals.** Rejected because it couples downstream products to prompts/adapters/persistence and destroys the reuse boundary.
2. **Require an LLM call to decide release usability.** Rejected because compatibility, governance state, digest verification and explicit supersession are deterministic security/data-integrity controls.
3. **Treat a well-shaped digest string as proof of artifact integrity.** Rejected because syntax validation does not hash bytes.
4. **Infer compatibility or supersession from version ordering/timestamps.** Rejected because neither proves compatibility nor steward-approved replacement.
5. **Overwrite a published release in place when corrected.** Rejected because published semantic truth is immutable; correction creates a distinct successor and explicit supersession evidence.
6. **Pretend structural JSON Schema enforces self-supersession inequality.** Rejected because Draft 2020-12 has no general sibling-field inequality operator; the semantic rule must remain explicit.
7. **Move downstream authorization into ConceptWeave.** Rejected because tenant/purpose authorization belongs to each consuming product and its identity/control plane.

## Verification evidence on the active branch

- Predecessor `61776fbf5969ec4f8897f48b7bd410052f83ea9d` recorded a hosted Product RED for the missing public supersession contract.
- Test-only `6cf136b94b76e09c8ec1c15fee809fe5ca791dca` encodes five reviewed fail-closed regressions: immutable release-id conflict, governed Superseded predecessor admission, v1 schema version binding, self-supersession semantic conformance, and LLVM expansion-region/total-region coverage.
- `af1d123c...`, `1ac0758d...`, `e54a0b30...`, `663e52d6...`, and `d6690051...` apply the corresponding minimal schema, coverage, semantic-rule, Rust and edge-coverage repairs.
- `e0ef02ee99375ebef2ce2b815dc5340e45708b24` non-force adopts Foundation #14 as a two-parent merge and preserves the Client-specific public-contract gate while taking the Foundation repository-qualified PR concurrency/CI contract.
- The semantic reference validator was executed locally against its valid and self-supersession fixtures with return codes 0 and 1 respectively. This is focused local evidence, not hosted exact-head GREEN.

## Follow-up / acceptance for Accepted status

1. Obtain exact-head fmt/Clippy/tests/rustdoc/100% owned coverage and public structural+semantic contract validation on one unchanged final PR #5 head.
2. Integrate the Foundation prerequisite, retain the non-force ancestry, and rerun every then-required exact-head workflow.
3. Resolve each valid review thread only after the current implementation is verified; satisfy ordinary governance without self-approval or routine bypass.
4. Add provenance/signature verification only behind an explicit versioned Governance & Publication contract.
5. Prove the seam with an anonymized GRC-shaped reference-client fixture and no cross-service application-table access.
