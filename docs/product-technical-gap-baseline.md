# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This document is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, reviews, runs, and statuses are evidence coordinates, never mutable dependencies. Evidence from an earlier head does not transfer after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, `contextual-orchestrator` production LLM routing, and Keyverse identity trust. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only; source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, is the active Source Observation writer. Review `5187402940` identified the current P1; behavioral RED `9d9e8d9e35f008edf0e3359188723dd94db512c8` and doctoring successor `77b12263df08f921645e9f9fed66479847228471` are ordinary-forward descendants of the retained explicit-false lifecycle repair.

#46 remains Draft. #45 and #6 must not duplicate or partially cherry-pick this source slice; they adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is GREEN.

Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Central workflow ownership remains outside ConceptWeave. Canonical provider-failure successor `.github#2114` is now exact `bb183e4d73191c019d3470a9900f6d838078430d`, OPEN/Draft/mergeable on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`. On that exact head Runtime Quality `34707881858`, CodeQL PR `34707881849`, Security Scan `34707881812`, Python Security `34707881821`, and SAST Semgrep `34707881800` are all terminal GREEN. No current-head independent `APPROVED` review exists, so normal protected landing is still pending. Central evidence never transfers to this leaf and no workflow copy, synthetic status, manual/no-op retrigger, or leaf-side provider workaround is permitted.

## Source Observation boundary

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content identity, evidence locations, and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority, or foreign product truth. Historical v2 evidence is frozen. PostgreSQL successor facts are additive and domain-separated. Catalog OIDs are capture-local join coordinates, never governed semantic identity.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, and explicit temporal constraint evidence. `pg_constraint.conperiod` remains the declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, reference, referenced-key and timing requirements.

Retained repairs include literal exclusion-operator inference removal (`5186175514 -> a9065d46... -> e286c352...`), PERIOD referenced-key completeness (`5186323924 -> d692773a... -> 1462105f...`), temporal backing-index ordered-key/static-shape coherence (`5186585545 -> ffe75edd... -> e258b394...`), and explicit unusable lifecycle rejection (`5186802339 -> 681e280f... / ed4882e... -> ff842b35...`). These remain valid predecessors but do not constitute current-head GREEN.

## Active P1 — supporting-index lifecycle completeness

The retained `ff842b358106c0adeda97fd2405952ff7be029b1` repair rejects explicit `indisready=false`, `indisvalid=false`, and `indislive=false`, but the shared `key_constraint_backing_index_static_shape_matches()` still admits all three unobserved states through `Option<bool>::is_none_or(...)`.

That is acceptable for generic standalone index evidence, but not when an index is promoted as authoritative backing evidence for explicitly observed PK/UNIQUE timing or a positive `WITHOUT OVERLAPS` key. PostgreSQL 18 exposes the supporting-index join through `pg_constraint.conindid` and the joined `pg_index` row supplies `indisready`, `indisvalid`, and `indislive` as material booleans. A bounded capture that asserts supporting-index authority while omitting those available lifecycle facts cannot prove that the index is usable support.

Review `5187402940` records the finding. Behavioral RED `9d9e8d9e35f008edf0e3359188723dd94db512c8` covers both owned admission paths: lifecycle-unobserved PK/UNIQUE timing and lifecycle-unobserved positive `conperiod` backing must fail closed, while explicit ready+valid+live controls remain admissible. Primary-source doctoring is `docs/doctoring/source-observation-key-backing-index-lifecycle-completeness.md`.

The minimum causal production repair is confined to the shared supporting-index predicate: `ready`, `valid`, and `live` must each equal `Some(true)` when the index is being admitted as constraint support. Do not make lifecycle globally mandatory for every standalone index, duplicate the test in timing/period callers, or infer constraint/temporal semantics from lifecycle state.

Current Source Observation state is **BEHAVIORAL_RED_ACTIVE**. It is not source-repaired, native/Product GREEN, Ready, merge-authorized, or released.

## Acceptance after repair

After the minimum production repair, one unchanged exact #46 successor must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, the new lifecycle-completeness contracts plus retained temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review evidence. Any head movement restarts exact-head acceptance.

The available execution host does not provide a Rust toolchain and cannot substitute for repository-pinned hosted execution. This does not justify a bypass, synthetic status, Draft/Ready toggle, or predecessor-evidence transfer.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, use bounded `REPEATABLE READ READ ONLY` catalog capture, and never keep an explicit database transaction/lock open while waiting on an LLM or long external computation.

Catalog OIDs may only join the captured snapshot. When the adapter claims that an index supports PK/UNIQUE timing or a temporal key, it must bind `pg_constraint.conindid` to the exact `pg_index` row inside the same authorized snapshot and capture `indisready=true`, `indisvalid=true`, and `indislive=true` explicitly rather than defaulting missing fields. It must also retain exact key layout/static flags, `conexclop`, operator-class/operator-family compare-type evidence, temporal type/domain chain, timing/action/match facts, and policy-admitted row/byte/concurrency ceilings. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail-closed.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: REINDEX*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL source: `ComputeIndexAttrs()`*.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | BEHAVIORAL_RED_ACTIVE | `5187402940 -> 9d9e8d9e... -> 77b12263...`; lifecycle-unobserved supporting indexes still pass production and need the shared-predicate repair. |
| Product CI | CENTRAL_REVIEW_PENDING | `.github#2114@bb183e4d...` has Runtime/CodeQL/Security/Python/Semgrep GREEN; independent current-head approval/protected landing remains. |
| Quality gate | BLOCKED_ON_SOURCE_REPAIR | No exact-head acceptance until the active RED is repaired. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before #46 GREEN and parent adoption. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Repair `key_constraint_backing_index_static_shape_matches()` once so supporting-index lifecycle is exactly `Some(true)` for ready/valid/live; keep generic standalone index lifecycle optional.
2. Reacquire one unchanged exact-head Rust 1.98 and hosted Product/security/dependency/review acceptance; repair only real failures ordinary-forward.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, `.github#2114@bb183e4d...` needs current-head independent approval and normal protected landing/reconciliation before its downstream owner path is replayed. No central evidence transfers to ConceptWeave.
5. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by deterministic validation, independent evaluation, steward review, and immutable publication.
