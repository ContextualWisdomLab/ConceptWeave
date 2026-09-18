# Product / Technical Gap Baseline

**Snapshot:** 2026-09-18

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding surface through exact source head `7ebb14bb37e7a86c88446a4fad729ee4cdd24607` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-7ebb14bb.md`; its matching changelog is preserved at `docs/archive/CHANGELOG-through-7ebb14bb.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

Current #46 source authority is `ab8d72e98dcc4a3c1270ce265fe6cbf18e3a3d6e`, with behavior-bearing converter-strictness production at parent `a5234386f1d8fa67740b2d1454574e30f824894d` and realistic compile-contract RED `06cd1ea330d439ac87553ac59010306f1df173dc`.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains. The active chain binds exact constraint/backing-index identity, ordered keys and exclusion operators, target-procedure scalar/strictness/volatility/parallel/kind/security/leakproof/definition/owner/configuration/ACL/planner-support/cost/transform-type facts, exact selected `pg_transform` rows, converter definition, converter owner, converter object-level `EXECUTE` ACL, converter nullable `proconfig`, converter raw `prosecdef`, converter raw `proleakproof`, and now converter raw `proisstrict`.

Quoted PostgreSQL transform-type identifiers remain collision-safe: schema and type-name components are percent-encoded independently before the canonical `.` separator. Valid pairs such as (`payload.domain`, `json`) and (`payload`, `domain.json`) therefore cannot collapse to one provenance location.

## Transform-converter strictness integrity

PostgreSQL 18 `CREATE FUNCTION` exposes `STRICT` / `RETURNS NULL ON NULL INPUT` as independent function execution semantics. `CREATE TRANSFORM` constrains converter signatures but does not make strictness part of `(trftype, trflang)` identity. Converter definition, owner, ACL, local configuration, security-definer state and leakproof state therefore cannot stand in for raw same-row `pg_proc.proisstrict`.

`IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot` is an observational successor over `IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot`. Both strict and non-strict states are representable and domain-separated. Every exact predecessor `(constraint coordinate, key position, transform type, direction)` must receive one observation with matching converter schema/function identity. Missing or extra coordinates, duplicate coordinates, binding drift, zero positions, blank converter identifiers and unknown receipt coordinates fail closed.

Traceability:

- finding/RED: `06cd1ea330d439ac87553ac59010306f1df173dc`;
- production successor: `a5234386f1d8fa67740b2d1454574e30f824894d`;
- edge-contract successor: `ab8d72e98dcc4a3c1270ce265fe6cbf18e3a3d6e`;
- finding review: `5249125942`;
- edge review: `5249147930`;
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_strictness.rs`;
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_strictness_contract.rs`;
- doctoring: `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-strictness-integrity.md`.

## Acceptance boundary

**Source repaired is not GREEN.** Exact `ab8d72e...` currently has no pull-request Actions run inventory. Repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc and owned production statement/branch/edge coverage have not been demonstrated on the exact head. No predecessor execution result transfers.

The bounded PostgreSQL 18 live differential must retain all predecessor ordinary-EXCLUDE evidence and resolve every nonzero selected transform converter to the same exact-generation `pg_proc` row, independently capturing definition, owner/ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof` and raw `proisstrict`. Missing resolution, mixed-generation joins or inferred Boolean state is capture failure.

## Residual material gap

Converter volatility, parallel safety, planner support and cost remain independently mutable converter-function facts that are not yet governed by the converter successor chain. They stay review-gated until the current strictness successor obtains exact-head native/hosted acceptance and bounded PostgreSQL 18 differential evidence. No publication or immutable semantic release is authorized from this Draft stack.

## Canonical prerequisite state

Active central owner is `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`. The branches are materially diverged and the shared scheduler changed on both sides. #2040 production remains repository-identity RED; protected main is only partially hardened.

The next valid central movement is ordinary/non-force path-wise protected-main reconciliation. The resolved scheduler must preserve #2040's v2 CodeQL producer, removal of source-neutral restamps, repository-scoped Actions credential/selected-token proof, stale-run revalidation, rationale/docstrings/tests; adopt compatible protected-main queue/coalescing/capacity behavior; and enforce the stronger repository-identity invariant.

Queue-health #2268 is exact `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 is exact `2b849c874122961e025c29f7fa0bb697863c3d68`. Both carry focused source repairs but their exact-head Security/SAST/Python Security/CodeQL workflows remain queued. Source-fix #2175 remains exact `6e5f75dd40a428d174f691e35e210d08a29eb270`; it intentionally cannot self-modify `.github/` or `scripts/ci/` control-plane paths.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its Security Scan and SAST success do not override terminal CodeQL PR failure; no unchanged-head manual rerun or leaf-source workaround is authorized.

## Required order

`.github#2040` path-wise reconciliation with repository-identity repair -> focused scheduler GREEN -> fresh exact-head central terminal checks plus qualifying non-self approval -> fresh compatible #35 acceptance and normal landing -> exact #46 Rust 1.98 native/hosted GREEN -> bounded PostgreSQL 18 differential -> review the next converter auxiliary fact -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
