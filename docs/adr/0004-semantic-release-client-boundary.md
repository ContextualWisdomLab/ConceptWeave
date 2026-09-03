# ADR 0004 — Semantic-release client boundary

- **Status:** Proposed
- **Date:** 2026-09-02
- **Decision owners:** ConceptWeave Governance & Publication and Client Consumption bounded contexts
- **Related:** Issue #3, PR #5, `docs/product-technical-gap-baseline.md`

## Context

Issue #3 requires downstream CWL products to consume governed ConceptWeave releases without importing generation internals. The foundation separates candidate truth from publication state, but a buyer-facing workflow remains incomplete until a consumer can reject incompatible or non-governed releases, verify exact detached artifact bytes, compare releases, and follow an explicit correction/supersession relation without guessing from version order or timestamps.

A client contract must remain useful offline. LLM/provider availability, generator prompts, persistence state, and foreign application databases cannot be prerequisites for deterministic release admission. Conversely, client-side structural checks must not be confused with publication authority, consuming-product authorization, or cryptographic/signature verification that has not actually occurred.

This ADR remains **Proposed** while PR #5 is Draft and exact-head Product/security/review evidence is incomplete. Implemented code on an unintegrated Draft head is evidence for the decision, not grounds to mark the decision Accepted prematurely.

## Decision

Introduce **Client Consumption** as a Supporting Bounded Context and `conceptweave-client` as its Rust reference implementation.

The current `semantic_release` public contract carries stable release identity, explicit contract and ontology/model versions, truth/publication state, a canonical declared artifact digest identity, provenance references, and unique stable concept identifiers. `SemanticReleaseClient` admits authoritative use only when the release uses the explicit current contract version or an explicitly configured supported-legacy version and is both `Published` and `Authoritative`. Compatibility is never inferred from semantic-version ordering.

`ReleaseDigest` accepts only canonical `sha256:<64 lowercase hex>` identity. `SemanticReleaseClient::verify_detached_artifact` separately hashes the exact caller-supplied detached immutable semantic-artifact bytes and requires an exact digest match after authoritative-use admission. The manifest declares that detached artifact digest; the contract deliberately avoids a self-referential requirement to hash the manifest bytes containing the digest field. Digest syntax and byte-integrity evidence therefore remain distinct.

`SemanticReleaseClient::diff` admits both releases through the same authoritative-use gate and reports deterministic sorted concept additions/removals. Exact concept resolution is deterministic and performs no fuzzy matching or model call.

For corrections, `SemanticReleaseReference` binds one release id to its exact artifact digest. `ReleaseSupersession` names a distinct superseded reference, an exact successor reference, and a nonblank rationale. `validate_supersession` requires both referenced releases to pass ordinary authoritative-use admission and requires both id-and-digest references to match exactly. Supersession is never inferred from version order, timestamp, semantic diff, or ontology similarity, and the prior immutable release is not overwritten.

The generation-to-client seam is a versioned public contract. Client code may use public domain value types such as `TruthStatus`, `PublicationState`, and `EvidenceReference`, but may not import generator-private classes, prompts, provider payloads, persistence tables, Source Observation internals, or orchestration state.

Consuming products keep tenant/purpose authorization, business-domain truth, and physical query execution. ConceptWeave returns semantic contracts/query plans; it does not become a foreign product's data plane.

LLM-assisted future `match`, ambiguity explanation, and candidate ranking operations must use `ContextualWisdomLab/contextual-orchestrator`. Their outputs remain candidate/evidence state. Admission, compatibility, digest verification, supersession validation, publication-state checks, and authorization remain deterministic.

## Consequences

### Positive

- consumers can fail closed before authoritative use without an LLM provider;
- stable release contracts prevent generator-private implementation leakage;
- truth/publication authority remains explicit across repository boundaries;
- digest syntax and actual byte verification cannot be conflated;
- explicit supported-legacy policy avoids accidental version-order heuristics;
- corrections preserve immutable predecessor releases and bind the exact successor by id plus digest;
- GRC and other downstream consumers can build ACLs against one stable seam.

### Costs and deferred work

- the Rust supersession contract does not yet have a finalized language-neutral supersession JSON Schema or generated bindings;
- signature/provenance-chain verification remains deferred until Governance & Publication defines a stable signing contract;
- typed relation/mapping/dimension/measure resolution, match/align/explain, and semantic query-plan operations remain Issue #3 work;
- GRC reference-client fixtures remain required before buyer-facing integration readiness;
- this ADR cannot advance to Accepted until the stacked implementation is integrated and current-head deterministic/security/review evidence is terminal.

## Alternatives rejected

1. **Let consumers import generator internals.** Rejected because it couples downstream products to prompts/adapters/persistence and destroys the reuse boundary.
2. **Require an LLM call to decide release usability.** Rejected because compatibility, governance state, digest verification, and explicit supersession are deterministic security/data-integrity controls.
3. **Treat a well-shaped digest string as proof of artifact integrity.** Rejected because syntax validation does not hash bytes.
4. **Infer compatibility or supersession from version ordering/timestamps.** Rejected because neither proves compatibility nor steward-approved replacement and would create hidden heuristics.
5. **Overwrite a published release in place when corrected.** Rejected because published semantic truth is immutable; correction creates a distinct successor and explicit supersession evidence.
6. **Move downstream authorization into ConceptWeave.** Rejected because tenant/purpose authorization belongs to each consuming product and its identity/control plane.

## Verification evidence on the active branch

- Existing Rust integration tests cover authoritative admission, compatibility, non-Published/non-Authoritative states, provenance/identity requirements, duplicate concepts, digest syntax, exact detached-byte verification, diff, and exact concept resolution.
- Test-first supersession commit `67132eda0e25d23a4185d4b98f0c6dc3b11e17a4` introduced an API that did not yet exist and required immutable id+digest predecessor/successor references, rationale, self-supersession rejection, exact-reference validation, and ordinary authoritative admission.
- Production commit `2c4a7954ad3a4fb0dd0a5482a6870fcc0d2996a3` implements that bounded contract. Follow-up edge coverage binds mismatch checks to digest as well as id and exercises both predecessor and successor admission paths.
- Detached-artifact contract RED was observed on predecessor head `0c32a7b55d3c687ab76cee789962866573496ba1`: Product run `33664177838`, job `100361706615` acquired an Ubuntu 24.04 runner, verified the exact checkout, passed the CI contract/toolchain/fmt steps, then Clippy failed with `E0599` because `verify_detached_artifact` did not yet exist. Production head `9c278598001c502a733100d11e901538c3dc2677` applies only the causal API/rustdoc repair.
- Hosted exact-head GREEN is still required on the final unchanged documentation head; queued/predecessor results are not GREEN.

## Follow-up / acceptance for Accepted status

1. Obtain exact-head fmt/Clippy/tests/rustdoc/100% owned coverage and public-contract validation on the final PR #5 head.
2. Integrate the foundation prerequisite, cleanly restack PR #5, and rerun every then-required exact-head workflow.
3. Resolve all valid current-head review findings and satisfy ordinary governance without self-approval or routine bypass.
4. Define a language-neutral supersession/publication receipt contract before generated bindings or cross-language release claims.
5. Prove the seam with an anonymized GRC-shaped reference-client fixture and no cross-service application-table access.
