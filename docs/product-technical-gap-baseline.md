# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-69785169.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in the archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_proc.provolatile` integrity

Review `5229099545` on exact predecessor `fe69a28d229ee35b721e6b852631d07da9e19072` found the next bounded P1. The retained chain proves the exact operator implementation procedure and preserves Boolean result identity, binary operator kind, scalar cardinality, and raw strictness, but it still omits raw `pg_proc.provolatile`.

PostgreSQL 18.6 stores `provolatile` independently in `pg_proc`: `i` means immutable, `s` stable, and `v` volatile. The categories differ in whether results may vary for the same inputs, whether side effects are allowed, optimizer treatment, and MVCC snapshot visibility. Exact procedure identity therefore does not by itself preserve this mutable catalog behavior.

Ordinary-forward lineage:

- structural source/compile RED `3aa7d5a4aeed555cc76fbb0b97ec2c415d104011`; the contract referenced the new public volatility types before production existed, so no executed compiler failure is claimed;
- production successor `9b10d980ec45c9d2e6886e4714ab71bb7f94d32a`, `IndexExclusionConstraintOperatorProcedureVolatilityObservation` / `Snapshot` / `SourceReceipt`;
- public composition `1ca6200860dec62310b87fa4f194b19fe4983af3`;
- PostgreSQL/APA decision record `697851690b60fb7b086bf4c007a0fb3956adace7`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-volatility-integrity.md`;
- predecessor baseline preserved losslessly at `docs/archive/product-technical-gap-baseline-through-69785169.md`.

`IndexExclusionConstraintOperatorProcedureVolatilitySnapshot` derives the exact coordinate/key inventory from `IndexExclusionConstraintOperatorProcedureStrictnessSnapshot`. Every governed position requires one independently observed raw `provolatile` fact bound to the same stable operator and exact `oprcode` procedure. Missing/duplicate evidence, operator/procedure binding drift, zero position, unknown receipt coordinates, and raw discriminators outside `i|s|v` fail closed. All three legal PostgreSQL states remain representable and enter the new domain-separated digest; the contract proves immutable, stable, and volatile observations cannot collapse to one governed identity.

This repair is deliberately observational. It does not add a local requirement that an ordinary EXCLUDE operator implementation be immutable; such a requirement would need its own PostgreSQL authority and validity analysis.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `pg_proc` §52.39, where `provolatile`, `proisstrict`, `proretset`, and `prorettype` are distinct catalog columns, and §36.7 Function Volatility Categories, which defines the behavioral/optimizer/MVCC meaning of `VOLATILE`, `STABLE`, and `IMMUTABLE`. Focused rationale and APA references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-volatility-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5229099545`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_volatility.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_volatility_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureStrictnessSnapshot`
- exact catalog fact: `pg_proc.provolatile`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-volatility/strictness/scalar/kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `prorettype`, `proretset`, `proisstrict`, and `provolatile`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `provolatile` must come from the exact `pg_proc` row and may not be inferred from normalized signatures, function names, result type, scalar cardinality, strictness, or operator-family conventions.

A known immutable implementation is the positive control. Otherwise identical `i`, `s`, and `v` fixture observations are distinguishability controls for the evidence layer, not claims that PostgreSQL accepts all three states for a particular EXCLUDE DDL definition.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github` owner repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `pg_proc.provolatile` plus every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.
