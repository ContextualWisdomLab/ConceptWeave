# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-30d7fb8b.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in `docs/archive/product-technical-gap-baseline-through-30d7fb8b.md` remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; `conparentid`; `conislocal`; `coninhcount`; raw `connoinherit`; `condeferrable`; `condeferred`; `conenforced`; `convalidated`; `conperiod`; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independently resolved `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; independently resolved `pg_constraint.connamespace`; catalog-family shape; exact constraint/backing-index name coupling; independently observed access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_proc.proisstrict` integrity

Review `5228673154` on exact predecessor `a1c23696f6db6faf6780446ecfac965ae309948d` found the next bounded P1. The retained operator procedure chain proves the exact scalar Boolean `oprcode` implementation but omits raw `pg_proc.proisstrict`. PostgreSQL 18 stores `proisstrict` independently from procedure identity, `prorettype`, and `proretset`: strict functions return null for null input without being invoked; non-strict functions may inspect null arguments.

This is material to ordinary EXCLUDE observation because PostgreSQL 18 `src/backend/executor/execIndexing.c::index_recheck_constraint()` explicitly states `Assume the exclusion operators are strict` before short-circuiting an existing NULL index value. However, `src/backend/commands/indexcmds.c::ComputeIndexAttrs()` validates commutativity and operator-family compatibility without establishing raw `proisstrict=true` as a separate creation-time gate. ConceptWeave therefore preserves strictness as independent catalog evidence but deliberately does **not** invent a stronger PostgreSQL validity rule.

Ordinary-forward lineage:

- structural source/compile RED `413971a472e4393a4b0e6089569ba2b991c3ae65`; the contract referenced the new public types before production existed, so no executed compiler failure is claimed;
- production successor `b3348b9f62942432b5f3ecb376511677c47092fb`, `IndexExclusionConstraintOperatorProcedureStrictnessObservation` / `Snapshot` / `SourceReceipt`;
- public composition `4e9b5dd2257ac314e68998a9c5be9291d04376e1`;
- focused edge/provenance contract `2ccd5b10889c2f081a182c24341e75161fe0c61e`;
- PostgreSQL/APA decision record `30d7fb8bcca2e1f06c59e9863c0a520993d167a6`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-strictness-integrity.md`;
- predecessor baseline preserved losslessly at `docs/archive/product-technical-gap-baseline-through-30d7fb8b.md`.

`IndexExclusionConstraintOperatorProcedureStrictnessSnapshot` derives the exact coordinate/key inventory from `IndexExclusionConstraintOperatorProcedureScalarSnapshot`. Every governed position requires one independently observed raw `proisstrict` fact bound to the same stable operator and exact `oprcode` procedure. Missing/duplicate evidence, operator/procedure binding drift, zero position, and unknown receipt coordinates fail closed. Both `true` and `false` are admitted and included in the new domain-separated digest so source states cannot collapse. The contract explicitly proves that a non-strict row remains representable and has a different digest from an otherwise identical strict row.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc`, where `proisstrict`, `proretset`, and `prorettype` are independent catalog columns, plus `REL_18_STABLE@1ac292cb1436c788fb6ea29551b0fe459e2cb340` source paths `src/backend/executor/execIndexing.c` and `src/backend/commands/indexcmds.c`. Focused rationale and APA references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-strictness-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5228673154`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_strictness.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_strictness_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureScalarSnapshot`
- exact catalog fact: `pg_proc.proisstrict`
- executor behavior: NULL recheck short-circuit under an explicit strictness assumption

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-strictness/scalar/kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `prorettype`, `proretset`, and `proisstrict`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `proisstrict` must come from the exact `pg_proc` row and may not be inferred from normalized signatures, scalar cardinality, Boolean return type, operator family, or executor behavior.

A strict implementation is the positive control. A separately observed non-strict procedure state is a distinguishability control, not a fabricated claim that PostgreSQL DDL rejects or endorses that state for EXCLUDE validity.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `proisstrict` and all retained ordinary-EXCLUDE controls -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
