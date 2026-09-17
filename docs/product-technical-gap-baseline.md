# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-68ee4b2e.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, procedure-volatility, procedure-parallel-safety, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; raw `pg_proc.provolatile`; raw `pg_proc.proparallel`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_proc.prokind` integrity

Review `5229806144` on exact predecessor `68ee4b2e852fb2f41750caa184f944d666bdf8a0` found the next bounded P1. The retained chain proves the exact operator implementation routine and preserves Boolean result identity, binary operator kind, scalar cardinality, raw strictness, raw volatility, and raw parallel safety, but it still omitted `pg_proc.prokind`. A stable namespace/name/input-type signature therefore did not prove that the resolved `pg_proc` row was a normal function.

PostgreSQL 18.6 stores functions, procedures, aggregate functions, and window functions together in `pg_proc`; raw `prokind` distinguishes them as `f`, `p`, `a`, and `w`. PostgreSQL `CREATE OPERATOR` requires the implementation routine to have been defined with `CREATE FUNCTION` and explicitly states that even when the legacy keyword `PROCEDURE` is used, the referenced object must be a function rather than a procedure. Routine kind is therefore an independent source fact and a PostgreSQL operator-validity invariant, not a property that ConceptWeave may infer from normalized signature or other `pg_proc` attributes.

Ordinary-forward lineage:

- structural source/compile contract `7f9d964d27abbfea05b8cbd4761246e8955db1c9`; it referenced the new public procedure-kind types before production existed, so no executed compiler failure is claimed;
- production successor `c9f4d8e719c9c5a4e1d82c150c8b6e942c7fb0a1`, `IndexExclusionConstraintOperatorProcedureKindObservation` / `Snapshot` / `SourceReceipt`;
- public composition `616ce311672f2ab9bd57e2cdbdd6ba1eb9b5e758`;
- focused predecessor-completeness/binding/provenance contract `e7d10cdb3b0877146207e6881099c2d5b008493d`;
- PostgreSQL/APA decision record `42a8cfa89c7fcc88cebb3c22b1bb01b093fd3d24`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-kind-integrity.md`;
- predecessor baseline preserved losslessly at `docs/archive/product-technical-gap-baseline-through-68ee4b2e.md` by `dfd153596c41f70b48386e8b8a0021433fc2316e`.

`IndexExclusionConstraintOperatorProcedureKindSnapshot` derives its exact coordinate/key inventory from `IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot`. Every governed position requires one independently observed raw `prokind` fact bound to the same stable operator and exact `oprcode` routine. Only `f` is admissible because PostgreSQL requires an operator implementation to be a normal function. `p`, `a`, `w`, unknown raw values, missing/duplicate evidence, operator/procedure binding drift, zero positions, and unknown receipt coordinates fail closed. The new digest includes the predecessor digest, exact coordinate/key position, stable operator and routine signatures, and raw discriminator without rewriting any predecessor identity.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc` §52.39, which defines `prokind='f'` normal function, `p` procedure, `a` aggregate, and `w` window function, together with `CREATE OPERATOR`, which requires the referenced implementation routine to be a function even under its historical `PROCEDURE` spelling. Focused rationale and APA 7 references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-kind-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5229806144`
- structural RED: `7f9d964d27abbfea05b8cbd4761246e8955db1c9`
- production: `c9f4d8e719c9c5a4e1d82c150c8b6e942c7fb0a1`
- composition: `616ce311672f2ab9bd57e2cdbdd6ba1eb9b5e758`
- focused contract: `e7d10cdb3b0877146207e6881099c2d5b008493d`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_kind.rs`
- test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_kind_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot`
- exact catalog fact: `pg_proc.prokind`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `prokind`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `prokind` must come from the exact joined `pg_proc` row and equal `f`; it may not be inferred from routine name, normalized argument/result signature, scalar cardinality, strictness, volatility, parallel safety, language, or operator-family conventions.

A real ordinary-EXCLUDE implementation routine with observed `prokind='f'` is the positive control. `p`, `a`, and `w` are negative distinguishability/corrupt-capture controls for the evidence layer, not claims that PostgreSQL accepts them as operator implementations.

The focused source contract covers valid provenance, non-function/unknown raw states, operator and implementation-routine binding drift, missing evidence, duplicate coordinates, zero position, unknown receipt coordinates, and public composition. It does not replace the still-required exact-head Rust suite or PostgreSQL live differential.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `pg_proc.prokind` plus every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.