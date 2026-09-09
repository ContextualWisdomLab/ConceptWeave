# Product / Technical Gap Baseline

**Snapshot:** 2026-09-10

This document records ConceptWeave's code-current product and technical gap baseline. Exact SHA/run coordinates are immutable evidence snapshots, never mutable supplier dependencies. Live protected branch, PR, issue, review, and workflow state supersedes recorded coordinates when it advances. Any head movement resets exact-head execution/review evidence unless the evidence was actually produced for that successor.

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
3. Client Consumption #5 is `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, OPEN Draft/mergeable on current Foundation after ordinary two-parent non-force reconciliation. Its predecessor execution/review evidence does not transfer.
4. Source Observation #6 is `9fbeddeead41bcec7b786ef56c6135bd251aac47`, OPEN Draft/mergeable on current Client before this baseline-only successor. Its production/test source matches pre-restack `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`; only the gap baseline changed during the Client reconciliation and the later representation/authorization doctoring. Exact-current execution/review evidence remains pending.
5. Research Intake #9 is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft/mergeable on Foundation. Pending-source resolution #40 is `4efe15c6318d8cb65c52a974a2c105363a4c82a5`, OPEN Draft/mergeable on #9.
6. Golden-set evaluation #10 is `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4`, OPEN Draft but non-mergeable. Its historical merge base is #9 `51c7df6d03f072449422fd58ca24b2f9d6026f07`; fresh compare against current #9 `a67d9d66...` is diverged, 35 commits ahead and 81 behind. Review `5158471700` requires ordinary non-force semantic reconciliation that preserves every #10 delta. #11 and later descendants remain open and must adopt the repaired #10 successor rather than flatten or close the stack.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op retrigger, synthetic status, mutable supplier dependency, or routine administrator bypass is acceptance evidence.

## Product-CI and central CodeQL prerequisite

ConceptWeave #35 remains the direct bootstrap prerequisite because protected `main` does not yet contain the repository-owned Product pull-request workflow.

Protected central `.github/main` is `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`. The currently broadest central producer/handler successor is `.github#2040@6706c231ab06a3c91c43fdb5b989cfcd79fff593`, OPEN Ready/non-Draft and mechanically mergeable. It non-force carries the valid #1902/#2004/#2043/#2044 producer, stacked-check, envelope, settlement, scheduler-credential, SARIF and base-bound evidence work. Exact-tree local evidence is 133 focused producer/handler/recovery contracts, 336 scheduler/stacked-security contracts, and 3,088 full-repository tests with one skip and 21 subtests.

The live hosted result does not authorize integration. On exact #2040 head, Security Scan `34251822390`, SAST `34251822314`, Python Security `34251822251`, and Agent Review Runtime Quality `34251822381` are terminal success, while CodeQL PR `34251822255` is terminal failure. Attempt 7 shows both language readers failing `Read current-head CodeQL dispatch verdict`; the coordinator then fails its dispatch step. Current owner findings classify two coupled pre-cutover problems: the protected handler cannot yet produce the new base-bound receipt/direct-evidence shape the successor producer accepts, and a one-shot Jobs API snapshot can observe an incomplete terminal matrix even after the matrix dependency has completed.

The bounded repair is not to restore creator-only `codeql-dispatch/<language>` trust or blind-rerun the same head. Central owner acceptance requires a pre-cutover direct-evidence adapter authenticated to the exact protected handler source plus repository/PR/base/head/required-run/language/job/SARIF/artifact identity, and deterministic bounded rereads of the exact required-run job set until every detected language has one stable terminal rerunnable identity or a fail-closed deadline is reached. The post-cutover base-bound receipt path remains canonical, with an explicit removal condition for the temporary compatibility adapter after the protected caller is live.

`.github#2051@558693e0333e48012beea142f739bc634b0674a7` remains OPEN Draft/mergeable with stricter `{base_ref, base_sha}` client identity and a versioned-rollout proposal; `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remains OPEN non-Draft/mergeable on #2051 with complete-failed-job-set validation and one atomic run-level wake per handler invocation. Those deltas remain useful and must be reconciled rather than discarded, but neither is protected authority and neither supersedes #2040's still-valid pre-cutover direct-evidence/job-snapshot RED.

Keep #35 stable while the central owner resolves that successor chain with exact terminal GREEN, zero valid unresolved findings, qualifying independent review, and normal protected integration. Then obtain fresh #35 exact-head CodeQL/review evidence before normal merge. Manual/no-op reruns or leaf source churn do not repair the owner contract.

## Source Observation current contract

Pre-restack Source Observation `331f8ed...` locally executed Rust 1.98 evidence: 132 tests across 42 suites including two doctests; fmt; strict Clippy; warnings-denied rustdoc; release build; Product CI contract/schema/fixture checks; normalized owned coverage 228/228 functions, 2,026/2,026 regions, and 194/194 branches. Raw LLVM diagnostics remained below 100% because source-embedded test/generic instrumentation is reported separately. Those results prove `331f8ed...` only and do not transfer to #6's current or later docs/source successors.

`ObservationRequestBudget` and the policy-admitted resource envelope bound exact-schema authorization metadata plus runtime row/byte/concurrency/deadline limits. `AuthorizedObservationRequest` is intentionally single-use at the execution seam. Retry after cancellation/failure/success requires a fresh authorization decision. Source lookup, policy binding, schema/resource authorization, adapter execution and cancellation consume one non-resetting operation budget; a concrete adapter may resolve credentials only for the exact authorized key-and-binding pair. Stale binding fails before source access.

The current owner digest/receipt vocabulary preserves deterministic table/column/constraint evidence, exact identifiers, column comments, PK/UNIQUE/FK/CHECK evidence, UNIQUE NULL-comparison state, FK reference behavior, targeted `SET NULL`/`SET DEFAULT` columns, and observed validation/enforcement state. It does not yet losslessly represent all material PostgreSQL 18 catalog evidence required by the planned adapter.

## PostgreSQL 18 representation/version/authorization prerequisite

The next Source Observation P0 is representation before transport. Do not add the concrete PostgreSQL adapter while the owner model would silently discard source facts.

The minimum successor adds deterministic owner value objects and collision-safe receipt coordinates for:

- relation kind and table-level comments;
- first-class indexes preserving key versus INCLUDE attributes, expression positions, partial predicates, NULL uniqueness semantics, and readiness/validity/liveness;
- qualified domains and enums as schema-scoped objects rather than manufactured table children;
- server-rendered `pg_get_constraintdef`, `pg_get_indexdef`, and `pg_get_expr` text labeled as reconstructed source evidence, never original DDL.

Review `5158484289` adds a versioning and authorization prerequisite. Current owner framing is explicitly `conceptweave.postgres_schema_snapshot.v2` and hashes only table/column/constraint evidence. Do not redefine that immutable receipt family by appending the new material fields under the same v2 domain. Introduce a successor digest framing (`v3` or equivalent explicit version), preserve frozen v2 verification for historical receipts, and bind every new material PostgreSQL fact in the successor identity.

Authorization must expand with representation. Current snapshot construction enforces schema scope by iterating observed tables. Schema-scoped domain/enum evidence must therefore be checked directly against the exact `AuthorizedObservationRequest` allowlist, including a schema with zero observed tables. Otherwise a type-only schema could bypass the existing table-driven containment invariant as soon as the new representation is admitted.

Executable RED precedes production representation code:

- a frozen historical v2 fixture reproduces its original v2 digest exactly;
- otherwise-identical successor snapshots differing in one newly required material PostgreSQL fact have distinct v3 identities;
- every new evidence kind has a verified receipt coordinate;
- unauthorized domain/enum-only schema fails before immutable snapshot/receipt side effects;
- authorized type-only schema succeeds;
- same-name types in different allowed schemas remain distinct;
- fake table-scoped type coordinates fail;
- input-order permutations of identical complete evidence remain digest-identical.

Issue #2 comment `5607010535` records the same acceptance boundary.

Only after this representation/version/authorization slice is exact-head GREEN should the concrete adapter be admitted behind `conceptweave-source-port`: maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, cargo-deny/SBOM review, least-privilege credential resolution from exact authorized key+binding, stale-binding rejection before credential/source I/O, one fresh authorization per attempt, explicit `REPEATABLE READ READ ONLY` catalog transaction, exact-schema `pg_catalog` capture, one remaining-operation budget across connect/transaction/query/cancellation, policy-admitted row/byte/concurrency ceilings, complete-or-fail snapshot construction, source disappearance handling, and deterministic replay against a frozen anonymized GRC-shaped fixture. `governance-risk-compliance` retains its business truth; no cross-service application-table SQL is introduced.

## Research stack repair

Current #9 is the canonical Research Intake parent. Golden-set #10 still carries useful source/test/fixture/docs delta, but its branch was not reconciled after #9 adopted current Foundation. The four paths modified by both lineages include `crates/conceptweave-zotero/src/lib.rs`, `crates/conceptweave-zotero/tests/review_contract.rs`, `review_contract_followup.rs`, and this gap baseline. Therefore a whole-tree ours/theirs merge would discard valid work. Repair #10 by ordinary non-force semantic integration, resolve those overlaps causally, preserve all other #10 delta, and regenerate exact-head Rust/coverage/hosted/review evidence. Keep #11+ dependent PRs Draft/open and restack them only after the repaired parent exists.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define canonical ConceptWeave ownership and foreign-owner seams. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | Observed/inferred/proposed/authoritative/rejected/superseded distinctions exist; no protected immutable semantic release exists. |
| Client Consumption | RESTACKED_HOSTED_PENDING | #5 consumes current Foundation through ordinary non-force ancestry; exact-head execution/review must regenerate. |
| Source Observation | REPRESENTATION_V3_P0 | #6 is stack-current, but PostgreSQL representation needs versioned digest + schema-scoped authorization RED->GREEN before transport. |
| Research Intake | RESTACKED_HOSTED_PENDING | #9 is current; predecessor local evidence is historical only. |
| Golden-set evaluation | STALE_PARENT_REPAIR_P1 | #10 is non-mergeable and 81 commits behind current #9; preserve and reconcile rather than close. |
| Product CI | BLOCKED_OWNER | #35 waits on accepted central CodeQL pre-cutover direct-evidence/job-set convergence repair plus downstream exact-head evidence. |
| Quality gate | ACTIVE | Rust 1.98, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, owned production coverage, fixture/schema/lock/clean-tree checks; every head movement resets exact-head acceptance. |
| Security / review | PENDING_EXACT_HEAD | Scanner/reviewer status is accepted only when bound to exact current head and applicable protected policy. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory on the exact protected release head. |

## Remaining commercial/semantic P0 sequence

1. Finish Source Observation PostgreSQL 18 lossless representation with successor digest versioning, schema-scoped authorization, and exact-current Rust/coverage evidence.
2. Add the concrete bounded read-only PostgreSQL adapter and frozen anonymized conformance fixture.
3. Repair #10's stale Research Intake parent without discarding its golden-set delta, then propagate that exact repaired ancestry through #11+.
4. Build deterministic ontology discovery with source receipts and abstention for unsupported semantics.
5. Build semantic-layer discovery for dimensions/measures/grain/units/relationships/mappings without treating relational structure as business authority.
6. Route every production LLM proposal through a released `contextual-orchestrator`; outputs remain proposed/inferred until steward validation/publication.
7. Add alignment/matching, RDF/OWL/SKOS/SHACL validation, governed persistence/review/publication adapters, client completion, multilingual/evaluation, observability/recovery and immutable release evidence under their canonical owner boundaries.

## DDD and release fitness

Adapters stay outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state. Published semantic truth is immutable; corrections create a new release plus supersession evidence. Production LLM output cannot become authoritative before steward validation and governed publication.

No Foundation, #35, Client, Source Observation, research child, semantic publication, or release is authorized by this snapshot alone. The closest shared infrastructure prerequisite is the central CodeQL pre-cutover direct-evidence/job-set convergence repair; the closest ConceptWeave-owned source delta is the PostgreSQL 18 representation/version/authorization RED->GREEN; the closest research-stack repair is #10's non-force reconciliation onto current #9.
