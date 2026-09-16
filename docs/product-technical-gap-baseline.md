# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-a27c6991.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, procedure-volatility, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; raw `pg_proc.provolatile`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_proc.proparallel` integrity

Review `5229512857` on exact predecessor `a27c69915d1e76af29752ad07b89b3fcdc9af22e` found the next bounded P1. The retained chain proves the exact operator implementation procedure and preserves Boolean result identity, binary operator kind, scalar cardinality, raw strictness, and raw volatility, but it still omitted raw `pg_proc.proparallel`.

PostgreSQL 18.6 stores `proparallel` independently in `pg_proc`: `s` is parallel safe, `r` is parallel restricted to the parallel group leader, and `u` is parallel unsafe and forces a serial plan. PostgreSQL documents that this property is not inferred automatically for user-defined functions and that an incorrectly permissive label can produce errors or wrong answers in parallel queries. Exact procedure identity and volatility therefore do not preserve this planner/execution property.

Ordinary-forward lineage:

- structural source/compile contract `7b1ccf482d6240427f104d8c19ad04874d546e69`; it referenced the new public parallel-safety types before production existed, so no executed compiler failure is claimed;
- production successor `d7fbb00b64b09863577cbaf0110fd48379b6c5b5`, `IndexExclusionConstraintOperatorProcedureParallelSafetyObservation` / `Snapshot` / `SourceReceipt`;
- public composition `be0662a61417ae8e9eaa01bf75d5bdbe001c1265`;
- PostgreSQL/APA decision record `ada074ab52fd90c757cb47c2a03e304d93f90d32`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-parallel-safety-integrity.md`;
- predecessor baseline preserved losslessly at `docs/archive/product-technical-gap-baseline-through-a27c6991.md`.

`IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot` derives the exact coordinate/key inventory from `IndexExclusionConstraintOperatorProcedureVolatilitySnapshot`. Every governed position requires one independently observed raw `proparallel` fact bound to the same stable operator and exact `oprcode` procedure. Missing/duplicate evidence, operator/procedure binding drift, zero position, unknown receipt coordinates, and raw discriminators outside `s|r|u` fail closed. All three legal PostgreSQL states remain representable and enter the new domain-separated digest. This is observational evidence: ConceptWeave does not invent a local EXCLUDE rule requiring implementation procedures to be `PARALLEL SAFE`.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc` §52.39, where `proparallel` is a separate catalog column, and §15.4 Parallel Safety plus `CREATE FUNCTION`, which define `PARALLEL SAFE`, `RESTRICTED`, and `UNSAFE` and explain that overly permissive labels can fail or return wrong answers. Focused rationale and APA references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-parallel-safety-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5229512857`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_parallel_safety.rs`
- focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_parallel_safety_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureVolatilitySnapshot`
- exact catalog fact: `pg_proc.proparallel`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-parallel-safety/volatility/strictness/scalar/kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `proparallel` must come from the exact `pg_proc` row and may not be inferred from normalized signatures, procedure name/language, result type, scalar cardinality, strictness, volatility, or operator-family conventions.

A known catalog value from a live ordinary-EXCLUDE implementation procedure is the positive control. Otherwise identical `s`, `r`, and `u` fixture observations are distinguishability controls for the evidence layer, not claims that PostgreSQL accepts all three states for one particular EXCLUDE definition.

The current focused source contract verifies raw-state admission/rejection, one-based coordinates, canonical evidence location, public composition, and production unit-level digest separation. It does not replace the still-required exact-head Rust suite, full predecessor-binding/completeness execution, or live differential.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `pg_proc.proparallel` plus every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.