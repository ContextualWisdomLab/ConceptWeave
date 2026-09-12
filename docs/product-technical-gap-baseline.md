# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This file is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only; earlier-head execution evidence never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, `contextual-orchestrator` production LLM routing, and Keyverse identity trust. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, is the active Source Observation writer.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, and explicit temporal constraint evidence. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

Retained production repairs include literal exclusion-operator inference removal (`5186175514 -> a9065d46... -> e286c352...`), PERIOD referenced-key completeness (`5186323924 -> d692773a... -> 1462105f...`), temporal backing-index ordered-key/static-shape coherence (`5186585545 -> ffe75edd... -> e258b394...`), explicit unusable lifecycle rejection (`5186802339 -> 681e280f... / ed4882e... -> ff842b35...`), and lifecycle completeness (`5187402940 -> 9d9e8d9e... -> 77b12263... -> 384d1305...`). The shared `key_constraint_backing_index_static_shape_matches()` predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional.

## Retained-test fixture integrity — repaired, execution pending

Review `5187787381` identified a separate P1 on exact predecessor `cdb50a49d6abb3101abb9498a88adf7fe0d890e0`: several retained temporal/index tests constructed supporting indexes using older fixture assumptions. `RelationObservation::with_indexes()` now requires a nonblank access method and exactly one `IndexKeySemantics` record per structural key; constraint-support admission additionally requires explicit ready/valid/live evidence. Stale fixtures therefore failed during setup instead of reaching the temporal/type/index assertion they claimed to test.

The ordinary-forward fixture repair series is test-only from `cdb50a49...` through `d704bf9254d0f4c98401052ee275f0984039d62a`. A direct compare is 10 commits ahead / 0 behind and changes only nine contract-test files; no production source or semantic rule changed. The repaired tests add exact per-key opclass/indoption evidence and explicit lifecycle=true to coherent supporting-index controls while preserving deliberately missing/false lifecycle, missing catalog flags, wrong key order, action/timing/type/operator negative cases. `constraint_period_type_contract.rs` also restores complete `conexclop` fixture evidence so its tests reach the intended type-kind assertions. `constraint_period_contract.rs` drops a stale unused import after the restack.

Repair coordinates:

- `aeed9149e0613f11cd206f1273ac8bf8da33c826` — exclusion-operator fixtures;
- `c9bc77b522cc13b508acebea6f38c8bc0ba1b1e5` and `d704bf9254d0f4c98401052ee275f0984039d62a` — period fixtures plus stale-import cleanup;
- `148f68044cbaab30c6827f709dd48b50493b120c` — temporal-index controls;
- `0a8799bd3935d673715cb6a41fea4810678840c6` — timing controls;
- `cd650963a68f97edc9c3d67c628fc6e69dd90f0f` — backing-index presence/lifecycle controls;
- `2d6ab7a21f5d9bf5dfef018a3ed1a9e424327cf1` — temporal FK action controls;
- `b692bc22bfbf2391a1e21bc83c3b4819c4c3c693` — referenced-key timing controls;
- `546d5efda56e151bb316b741e416d9d595c9c78a` — temporal period-type controls;
- `7f8416dfee4fca20db9a027b4aadf60f26260061` — key backing-index shape controls.

Current Source Observation state is **FIXTURE_REPAIRED / EXACT-HEAD EXECUTION ACCEPTANCE PENDING**. It is not native/Product GREEN, Ready, merge-authorized, published, or released.

## Exact-head acceptance

One unchanged exact #46 successor must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, lifecycle-completeness plus all retained temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The available execution host does not provide `cargo`/`rustc`; therefore repository-pinned Rust execution cannot be substituted locally. This is not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, transfer predecessor evidence, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. At this snapshot the canonical owner is `ContextualWisdomLab/.github#2114` exact `5fb9c987c32620da63546fe69335b58f3a230cad`, OPEN/non-Draft/mergeable, based on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`. On that exact head Security Scan `34715843292` is terminal GREEN while CodeQL `34715843331`, SAST `34715843295`, Runtime Quality `34715843461`, and Python Security `34715843306` are still in progress.

Three current, unresolved owner-path review findings also remain in `scripts/ci/opencode_failure_envelope.py`: present-but-non-object `error`/`error.data` containers can be misclassified instead of treated as malformed; gateway HTTP status and last-provider HTTP status are being conflated as one authority; and specific structured terminal reasons can be rejected merely because they refine a generic 5xx status. These are `.github` owner repairs. ConceptWeave must not copy the workflow, bypass providers, synthesize settlement, or treat central evidence as leaf evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, use bounded `REPEATABLE READ READ ONLY` catalog capture, and never keep an explicit database transaction/lock open while waiting on an LLM or long external computation.

Catalog OIDs may only join the captured snapshot. When the adapter claims an index supports PK/UNIQUE timing or a temporal key, it must bind `pg_constraint.conindid` to the exact `pg_index` row inside the same authorized snapshot and explicitly capture `indisready=true`, `indisvalid=true`, and `indislive=true`. It must retain exact key layout/static flags, `conexclop`, operator-class/operator-family compare-type evidence, temporal type/domain chain, timing/action/match facts, and policy-admitted row/byte/concurrency ceilings. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail-closed.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL source: ComputeIndexAttrs()*.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | FIXTURE_REPAIRED / EXECUTION_PENDING | Production lifecycle repair retained; `5187787381` and fixture-only series through `d704bf92...` restore retained-test admission without weakening invariants. |
| Product CI | CENTRAL_OWNER_REPAIR_IN_PROGRESS | `.github#2114@5fb9c987...`: Security GREEN; four hosted suites in progress; three current review findings remain. |
| Quality gate | BLOCKED_ON_EXACT_HEAD_EXECUTION | Rust 1.98/native acceptance must be generated on one unchanged #46 head. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before #46 GREEN and parent adoption. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Keep both the production lifecycle-completeness repair and the retained-test fixture repair intact; repair only newly verified failures ordinary-forward.
2. Reacquire one unchanged exact-head Rust 1.98 and hosted Product/security/dependency/review acceptance. Head movement restarts the acceptance set.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, `ContextualWisdomLab/.github#2114` must resolve its current parser findings and reach terminal owner-path settlement/independent review before the central path is treated as settled. No central evidence transfers to ConceptWeave.
5. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by deterministic validation, independent evaluation, steward review, immutable publication, and release evidence.
