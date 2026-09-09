# Product / Technical Gap Baseline

**Snapshot:** 2026-09-10

This document records ConceptWeave's code-current product and technical gap baseline. Exact SHA/run coordinates are immutable evidence snapshots, never mutable supplier dependencies. Live protected branch, PR, issue, review, and workflow state supersedes recorded coordinates when it advances. A head movement resets exact-head execution/review evidence unless the evidence was actually produced for that successor.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical release-consumption contract. Source-system business truth stays with its canonical owner.

- `semantic-data-portal`: catalog/governance/consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- consuming products: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and active stack

Protected/default ConceptWeave `main` is `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; it remains repository bootstrap only and no immutable ConceptWeave semantic release exists.

1. Foundation #1 is `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft/mergeable on protected `main`.
2. Product-CI bootstrap #35 is `22709ec9b4d969bf67ec74db402813e74d11f7ca`, OPEN non-Draft/mergeable on protected `main`. Security Scan and SAST Semgrep are terminal success; CodeQL remains terminal failure in the central owner path.
3. Client Consumption #5 has been non-force reconciled onto current Foundation. Exact head before this Source Observation reconciliation is `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, OPEN Draft/mergeable on Foundation `60f14a6...`. The two-parent merge retains the prior Client tree and changes only this gap baseline to the current Foundation version; predecessor checks/reviews do not transfer.
4. Source Observation #6 source head before this reconciliation is `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`. Its production/test tree contains the single-use observation capability, exact source-policy binding, bounded non-resetting operation budget, immutable PostgreSQL table/column/constraint evidence, deterministic digest/receipt construction, and the UNIQUE null-comparison repair. This reconciliation adopts current Client #5 without force; the resulting exact head requires fresh execution/review evidence.
5. Research Intake #9 is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft/mergeable on Foundation. Pending-source resolution #40 remains downstream of #9; neither local predecessor evidence nor research proposal output is semantic authority.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op retrigger, synthetic status, mutable supplier dependency, or routine administrator bypass is acceptance evidence.

## Product-CI and central CodeQL prerequisite

ConceptWeave #35 remains the direct bootstrap prerequisite because protected `main` does not yet contain the repository-owned Product pull-request workflow.

Protected central `.github/main` is `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db` at this snapshot. Central `.github#2051@558693e0333e48012beea142f739bc634b0674a7` is OPEN Draft/mergeable and now explicitly classifies its current combined client+handler form as a rollout/bootstrap defect. Its required CodeQL run `34332431435` reached attempt 50 under the unchanged PR/head/base identity and ended `completed/startup_failure`; additional blind reruns are not acceptance evidence.

The verified mismatch is architectural: the PR-head client expects successor dispatch identity containing `{base_ref, base_sha}`, while `repository_dispatch` executes the handler from protected default branch, whose currently protected run-name schema is the predecessor form. The central owner therefore requires a versioned backward-compatible rollout: first protect a handler version/endpoint that can emit exact `{base_ref, base_sha, head, required_run}` identity without breaking old/in-flight clients; then non-force reconcile #2051 and switch the client; only after protected cutover and drain may the legacy handler be removed.

Stacked `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remains OPEN non-Draft/mergeable on #2051 and preserves complete-failed-job-set validation plus one atomic wake per handler invocation. That delta remains useful but does not itself establish protected rollout compatibility or current ConceptWeave GREEN. Keep #35 stable until the central owner reaches exact terminal GREEN, zero valid unresolved findings, qualifying independent review, and normal protected integration; then obtain fresh #35 exact-head CodeQL/review evidence before normal merge.

## Source Observation current contract

The exact pre-reconciliation source `331f8ed...` locally executed Rust 1.98 evidence: 132 tests across 42 suites including two doctests; fmt; strict Clippy; warnings-denied rustdoc; release build; and normalized owned coverage of 228/228 functions, 2,026/2,026 regions, and 194/194 branches. Raw LLVM diagnostics remained below 100% because source-embedded test/generic instrumentation is reported separately. These results prove `331f8ed...` only and do not transfer to the reconciliation successor.

`ObservationRequestBudget` and the policy-admitted resource envelope bound exact-schema authorization metadata plus runtime row/byte/concurrency/deadline limits. `AuthorizedObservationRequest` is intentionally non-`Clone`, and `SourceObservationPort::observe` consumes it by value, so one authorization crosses the canonical execution seam at most once. Retry after cancellation/failure/success requires a fresh authorization decision. Source lookup, policy binding, schema/resource authorization, adapter execution and cancellation consume one non-resetting operation budget; a concrete adapter may resolve credentials only for the exact authorized key-and-binding pair.

The immutable snapshot preserves exact identifiers and relational evidence without importing source-system business truth. Existing owner digest/receipt vocabulary covers table/column/constraint evidence and the repaired optional UNIQUE NULL-comparison state. This is insufficient for a truthful PostgreSQL 18 adapter because several material catalog facts still have no first-class immutable coordinate.

## PostgreSQL 18 representation prerequisite

The next Source Observation P0 is representation before transport. Do not add the concrete PostgreSQL adapter while the owner model would silently discard source facts.

The minimum representation successor must add deterministic owner value objects, digest identity, and receipt coordinates for:

- relation kind;
- table-level comments;
- first-class indexes, preserving key versus INCLUDE attributes, expression positions, partial predicates, NULL uniqueness semantics, and readiness/validity/liveness;
- qualified domains and enums as schema-scoped objects rather than manufactured table children.

Domain/enum identity must therefore support schema-scoped coordinates independent of a table. Columns may reference qualified type identity, but ConceptWeave must not duplicate a foreign type/business authority that belongs elsewhere. Server-rendered `pg_get_constraintdef`, `pg_get_indexdef`, or `pg_get_expr` text must be labeled as reconstructed source evidence, not original DDL.

Executable RED for this slice is evidence loss: otherwise-identical snapshots differing in one newly required PostgreSQL field must produce different owner digest identity; every admitted evidence kind must have a verifiable receipt coordinate; same-name types in different schemas must remain distinct; a domain/enum in a schema with no table must still be addressable; fake table-scoped type coordinates must fail; and input-order permutations of the same complete evidence must remain digest-identical. Add these fixtures before production representation code and regenerate Rust/doc/coverage evidence on the resulting exact head.

Only after the representation slice is GREEN should the concrete adapter be admitted behind `conceptweave-source-port`: maintained patched Rust PostgreSQL driver, least-privilege credential resolution from exact authorized key+binding, stale-binding rejection before source access, one fresh authorization per attempt, explicit `REPEATABLE READ READ ONLY` catalog transaction, exact-schema `pg_catalog` capture, one remaining-operation budget across connect/transaction/query/cancellation, policy-admitted row/byte/concurrency ceilings, complete-or-fail snapshot construction, source disappearance handling, and deterministic replay against a frozen anonymized GRC-shaped fixture. `governance-risk-compliance` retains its business truth; no cross-service application-table SQL is introduced.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define canonical ConceptWeave ownership and foreign-owner seams. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | Observed/inferred/proposed/authoritative/rejected/superseded distinctions exist; no protected immutable semantic release exists. |
| Client Consumption | RESTACKED_HOSTED_PENDING | #5 now consumes current Foundation through an ordinary two-parent successor; its semantic-release production/test/contract tree was preserved, but exact-head execution/review must regenerate. |
| Source Observation | RESTACK_IN_PROGRESS_THEN_REPRESENTATION_P0 | Current source contract is locally proven at `331f8ed...`; this reconciliation changes exact head, then PostgreSQL representation RED->GREEN precedes the live adapter. |
| Product CI | BLOCKED_OWNER | #35 waits on accepted central CodeQL rollout/bootstrap repair and fresh exact-head evidence. |
| Quality gate | ACTIVE | Rust 1.98, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, owned production coverage, fixture/schema/lock/clean-tree checks; every head movement resets exact-head acceptance. |
| Security / review | PENDING_EXACT_HEAD | Scanner/reviewer status is accepted only when bound to the exact current head and applicable protected policy. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory on the exact protected release head. |

## Remaining commercial/semantic P0 sequence

1. Finish Source Observation PostgreSQL 18 lossless representation and exact-current Rust/coverage evidence.
2. Add the concrete bounded read-only PostgreSQL adapter and frozen anonymized conformance fixture.
3. Build deterministic ontology discovery with source receipts and abstention for unsupported semantics.
4. Build semantic-layer discovery for dimensions/measures/grain/units/relationships/mappings without treating relational structure as business authority.
5. Route every production LLM proposal through a released `contextual-orchestrator`; outputs remain proposed/inferred until steward validation/publication.
6. Add alignment/matching, RDF/OWL/SKOS/SHACL validation, governed persistence/review/publication adapters, client completion, multilingual/evaluation, observability/recovery and immutable release evidence under their canonical owner boundaries.

## DDD and release fitness

Adapters stay outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state. Published semantic truth is immutable; corrections create a new release plus supersession evidence. Production LLM output cannot become authoritative before steward validation and governed publication.

No Foundation, #35, Client, Source Observation, research child, semantic publication, or release is authorized by this snapshot alone. The closest shared infrastructure prerequisite is the central CodeQL versioned rollout/bootstrap repair; the closest ConceptWeave-owned product delta is the PostgreSQL 18 representation RED->GREEN after this non-force stack reconciliation.