# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline and CHANGELOG through exact `69a674329969977c25e9266d615bec36a2538bf6` are preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-69a67432.md` and `docs/archive/CHANGELOG-through-69a67432.md`. Earlier surfaces remain under `docs/archive/`; focused decision/TRACEABILITY records remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and its released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner. Consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative; no issued predecessor digest domain is rewritten. For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained source identity includes the independent constraint and exact backing index; timing/enforcement/validation/period/no-inherit state; ordered `conkey`/`conexclop`; `pg_operator.oprkind`, independently resolved `oprcom`, exact `oprcode -> pg_proc`, and independent Boolean `oprresult`/`prorettype`; target-function `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prokind`, `prosecdef`, `proleakproof`; implementation language/definition (`prolang`, `prosrc`, `probin`, `prosqlbody`); exact `proowner`; exact nullable `proconfig`; exact `proacl` with same-generation ACL role resolution; exact optional `prosupport`; operator-family/strategy; constraint/index namespace/name coupling; access-method exclusion capability; backing-index role/lifecycle; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain separate owner families and are not reclassified as ordinary EXCLUDE.

## EXCLUDE implementation-function planner-cost integrity

Review `5232151401` on exact predecessor `69a674329969977c25e9266d615bec36a2538bf6` found the next bounded P1: the governed target-function chain omitted independent `pg_proc.procost`.

PostgreSQL 18.6 stores `procost` as `float4`, measured in units of `cpu_operator_cost`; larger values cause the planner to try to avoid evaluating the function more often than necessary. `ALTER FUNCTION ... COST` can change the estimate without changing the function call signature or any already-governed execution/security/definition/planner-support fact. The same operator/function identity and retained predecessor facts therefore do not prove equivalent planner-cost state.

The repair adds `IndexExclusionConstraintOperatorProcedureCostObservation`, `IndexExclusionConstraintOperatorProcedureCostSourceReceipt`, and `IndexExclusionConstraintOperatorProcedureCostSnapshot` over the exact planner-support predecessor. The adapter supplies the exact same-row `float4`; the domain rejects non-finite or non-positive capture, stores the validated `f32` bit pattern, and hashes those bits directly. Distinct positive finite costs produce distinct successor digests without decimal-string normalization. No ConceptWeave cost threshold is invented.

`pg_proc.prorows` is deliberately excluded from this successor. The retained target function is scalar (`proretset=false`), and PostgreSQL 18 documents `prorows=0` when `proretset` is false. Any future set-returning family requires a separately reviewed row-estimate successor.

Planner-cost lineage:

- finding review `5232151401`;
- structural source/compile RED `8b71c5e4c8d7f80e2cb7b4b0c8236d56136252cf`; the public planner-cost types did not yet exist, so no executed compiler failure is claimed;
- production successor `8fd9bda8ce33c5f2ece7b000c8ccdcb6147681c1`;
- public composition `46772a3dd991603a0ef4638ecc000182c8b6e795`;
- PostgreSQL/APA decision record `add2ade6e91323a273cf8bc3dc34c3a74bccb4a0`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-cost-integrity.md`;
- pre-procost decision surfaces preserved losslessly by `5a2fad70b5854ff82a267aba9637005a1c6a4135`.

Focused contract coverage includes exact positive-finite `procost` provenance, cost digest separation, exact operator/target-function binding, complete unique coordinate coverage, one-based positions, exact receipt coordinates, invalid-cost rejection, and public composition.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc`, which defines `procost` as the function's execution-cost estimate; PostgreSQL 18.6 `CREATE FUNCTION`, which defines `COST` as a positive estimate in units of `cpu_operator_cost`; and PostgreSQL 18.6 `ALTER FUNCTION`, which permits that estimate to change independently. The focused rationale and rejected alternatives are recorded in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-cost-integrity.md`.

Current planner-cost traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5232151401`
- structural RED: `8b71c5e4c8d7f80e2cb7b4b0c8236d56136252cf`
- production: `8fd9bda8ce33c5f2ece7b000c8ccdcb6147681c1`
- composition: `46772a3dd991603a0ef4638ecc000182c8b6e795`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_cost.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_cost_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot`
- exact catalog fact: `pg_proc.procost`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new planner-cost contract plus every retained procedure/operator/Source Observation/relation-partition contract, workspace/doc tests, release build, rustdoc, owned statement/branch/edge coverage, and all applicable hosted quality/security/review gates. Any source movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; follow `oprcode` to the exact target `pg_proc` row and independently read `proowner`, `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, `proparallel`, `prolang`, `prosrc`, `probin`, `prosqlbody`, `proconfig`, `proacl`, `prosupport`, and `procost`. It must preserve NULL-default ACL semantics and same-generation ACL role resolution. For nonzero `prosupport`, it must resolve that exact OID to the support `pg_proc` row in the same source-content generation; unresolved nonzero support is a capture failure, not absence. Operator-family/strategy and backing-index namespace/lifecycle/access-method/catalog controls remain in that same v3 generation.

A real ordinary-EXCLUDE implementation function and its actual `procost` state are the positive control. Synthetic cost variants are digest-distinguishability unit controls only.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` remains the workflow-owner landing vehicle. Its owner-qualified source repair is complete; exact-current workflow settlement remains a separate prerequisite. ConceptWeave does not compete with that owner lane or manufacture wake/no-op evidence.

Product bootstrap #35 remains a separate prerequisite; Ready/mergeable is not normal-landing authority while its required CodeQL acceptance debt remains unresolved. Fresh compatible acceptance follows canonical workflow-owner settlement.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_COST_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_COST_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PLANNER_SUPPORT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_ACCESS_CONTROL_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_CONFIGURATION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_OWNER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DEFINITION_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_SOURCE_REPAIRED / CANONICAL_WORKFLOW_OWNER_TERMINAL_SETTLEMENT_OPEN / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable semantic publication, version/tag/package/SBOM/provenance, reproducibility and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` exact-current terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN including planner-cost evidence -> bounded PostgreSQL 18 live differential including exact `procost` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
