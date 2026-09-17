# Product / Technical Gap Baseline

**Snapshot:** 2026-09-17

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The immediately preceding active baseline is preserved byte-for-byte at `docs/archive/product-technical-gap-baseline-through-e2284292.md`; earlier detailed decision surfaces remain in `docs/archive/`, and focused authority/TRACEABILITY records remain in `docs/doctoring/`. Exact-head execution evidence never transfers after branch movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology and semantic-layer generation/validation, governed immutable semantic releases, and the canonical released client-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth stays with its canonical owner; consumers use released/versioned semantic contracts and must not copy source truth, perform cross-service SQL, or depend on mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45 `codex/pr6-v3-representation`. #45 and #6 must not partially cherry-pick or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Retained ordinary-EXCLUDE authority

All valid predecessor repairs preserved in archived baselines remain authoritative. No issued v3, relation-partition, index-partition, key-constraint, ordinary-EXCLUDE, operator-family, backing-index exclusion-semantics, operator, commutator, operator-procedure, operator-result, operator-kind, procedure-scalar, procedure-strictness, procedure-volatility, procedure-parallel-safety, procedure-kind, procedure-security-definer, index-name, index-namespace, or index-lifecycle digest domain is rewritten by the current repair.

For ordinary `pg_constraint.contype='x'` EXCLUDE constraints, retained evidence includes the independent constraint object; exact `conindid` backing index; parentage/inheritance/timing/enforcement/validation/period state; ordered raw `conkey`; ordered resolved `conexclop`; raw `pg_operator.oprkind`; independently resolved `pg_operator.oprcom`; exact `pg_operator.oprcode -> pg_proc`; independent `pg_operator.oprresult` and `pg_proc.prorettype` with exact `pg_catalog.bool` identity; raw `pg_proc.proretset=false`; raw `pg_proc.proisstrict`; raw `pg_proc.provolatile`; raw `pg_proc.proparallel`; raw `pg_proc.prokind='f'`; raw `pg_proc.prosecdef`; constraint namespace and catalog-family shape; exact constraint/backing-index name coupling; access-method exclusion capability; independently resolved backing-index `pg_class.relnamespace`; exact index role/lifecycle flags; and exact v3 source-content-generation binding.

Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` and FOREIGN KEY `PERIOD` remain with their own owner families rather than being reclassified as ordinary EXCLUDE.

## EXCLUDE raw `pg_proc.proleakproof` integrity

Review `5230464081` on exact predecessor `e2284292ed83c0c957bbcdc5745ca15585f56438` found the next bounded P1. The retained chain proves the exact ordinary-EXCLUDE implementation function and preserves routine kind, execution privilege mode, result/cardinality, strictness, volatility, and parallel-safety semantics, but it still omitted raw `pg_proc.proleakproof`.

PostgreSQL 18.6 treats leakproofness as independent function metadata. A leakproof function reveals no information about its arguments except through the return value. PostgreSQL may evaluate leakproof operators/functions before `security_barrier` or row-level-security predicates, and leakproofness also affects whether planner statistics may be consulted when the user lacks direct table/column privilege. `ALTER FUNCTION ... LEAKPROOF|NOT LEAKPROOF` can change that property without changing the function's input-argument identity. Therefore otherwise identical stable operator/function bindings can carry different security/planner semantics and must not collapse to one governed digest.

This repair is observational rather than prescriptive. No authoritative ordinary-EXCLUDE rule was found that requires one particular `proleakproof` value. Both raw Boolean states are representable, but they are domain-separated and bound to the exact predecessor position.

Ordinary-forward lineage:

- finding review `5230464081` on predecessor `e2284292ed83c0c957bbcdc5745ca15585f56438`;
- structural source/compile contract `cd76aebfa29aa8ad19bfc9dd2c3d9d5447070112`; the contract referenced the new public leakproofness types before production existed, so no executed compiler failure is claimed;
- production successor `fe9d58d450863ffe5fb4316befe8b03d93c7085f`, `IndexExclusionConstraintOperatorProcedureLeakproofObservation` / `Snapshot` / `SourceReceipt`;
- public composition `513e858ff01a179f0e4275c972ff1a2bd45b13ea`;
- PostgreSQL/APA decision record `499330f9157ff372c28cc5de68e00eb6fa59f175`, `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-leakproof-integrity.md`;
- predecessor baseline preserved losslessly at `docs/archive/product-technical-gap-baseline-through-e2284292.md` by `2b976ecddedad059490d884408467b1b0c36e5f3`.

`IndexExclusionConstraintOperatorProcedureLeakproofSnapshot` derives the exact `(constraint coordinate, key_position)` inventory from `IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot`. Each governed position requires one independently observed raw `proleakproof` value bound to the same stable operator and exact `oprcode` function. Missing/duplicate evidence, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. The successor digest includes the predecessor digest, exact coordinate/key position, operator/function signatures, and raw Boolean. From the same predecessor, `false` and `true` must produce distinct digests.

## PostgreSQL 18 authority and TRACEABILITY

Primary authority is PostgreSQL 18.6 `CREATE FUNCTION` for `LEAKPROOF`, `ALTER FUNCTION` for independent mutation of leakproofness, Rules and Privileges §39.5 for security-barrier ordering, and Planner Statistics and Security §69.3 for statistics-access consequences. Focused rationale and APA 7 references are in `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-leakproof-integrity.md`.

Traceability:

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5230464081`
- structural RED: `cd76aebfa29aa8ad19bfc9dd2c3d9d5447070112`
- production: `fe9d58d450863ffe5fb4316befe8b03d93c7085f`
- composition: `513e858ff01a179f0e4275c972ff1a2bd45b13ea`
- doctoring: `499330f9157ff372c28cc5de68e00eb6fa59f175`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_leakproof.rs`
- test: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_leakproof_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot`
- exact catalog fact: `pg_proc.proleakproof`

## Acceptance boundary

**Source repaired does not mean GREEN.** One unchanged exact #46 head must pass repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the dedicated procedure-leakproof/security-definer/kind/parallel-safety/volatility/strictness/scalar/operator-kind/result/procedure/commutator contracts plus every retained Source Observation/relation-partition contract, workspace/doc tests, release build, owned rustdoc/test/edge-case coverage, and all applicable hosted quality/security/dependency/review gates. Any head movement resets exact-head acceptance.

The bounded PostgreSQL 18 live differential must resolve each exact `conexclop` OID to one `pg_operator` row and independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`; resolve `oprcode` to the exact `pg_proc` row; independently read `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel`; and retain operator-family/strategy plus backing-index namespace/lifecycle/access-method/catalog controls in the same v3 source-content generation. `proleakproof` must come from the exact joined `pg_proc` row and may not be inferred from routine name, owner, language, security mode, result/cardinality, strictness, volatility, parallel safety, or routine kind.

A real ordinary-EXCLUDE implementation function with its actually observed leakproofness is the positive control. A synthetic `false`/`true` pair is only a digest-distinguishability unit control; it is not evidence that both states are accepted by a particular real DDL fixture.

## Canonical prerequisite state

Canonical `ContextualWisdomLab/.github#2106` has advanced ordinary-forward to exact `c81e39ce040bb174c6ffb12670cadc3d0e0878a9` on protected `.github/main@a9c6477d326b84411f492ba4cac29f52deebcac3`, OPEN / Draft / mergeable. Its owner-local source RED remains exactly two owner-qualified repository-identity corrections in `docs/product-technical-gap-baseline.md`; its new exact-head security/quality workflow generation is nonterminal. ConceptWeave does not compete with that owner lane or manufacture wake/no-op evidence.

Product bootstrap #35 remains source-stable at `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` until the canonical workflow owner reaches compatible terminal acceptance and #35 receives a fresh current-generation acceptance.

## Current state

`INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_LEAKPROOF_DIFFERENTIAL_OPEN / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SECURITY_DEFINER_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_PARALLEL_SAFETY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_VOLATILITY_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_STRICTNESS_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SCALAR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_KIND_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_RESULT_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_PROCEDURE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_OPERATOR_COMMUTATOR_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_LIFECYCLE_SOURCE_REPAIRED / INDEX_EXCLUSION_CONSTRAINT_INDEX_NAMESPACE_SOURCE_REPAIRED / EXACT_HEAD_NATIVE_GREEN_OPEN / EXACT_HEAD_HOSTED_GREEN_OPEN / POSTGRESQL_18_LIVE_DIFFERENTIAL_OPEN`.

No release readiness is asserted while those obligations remain open. Immutable publication, semantic release, SBOM/provenance, reproducibility, and rollback evidence follow only after one unchanged protected head reaches terminal acceptance.

## Required causal order

Canonical `.github#2106` owner source repair/terminal settlement -> fresh compatible #35 acceptance and normal landing -> one unchanged #46 native+hosted terminal GREEN -> bounded PostgreSQL 18 live differential including independent raw `pg_proc.proleakproof` plus every retained ordinary-EXCLUDE control -> fresh terminal GREEN -> continue bounded material-catalog review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation.

Force-push, destructive rebase, self-approval, review dismissal, administrator bypass, synthetic status, copied central workflows, manual/no-op reruns, gate weakening, partial parent adoption, predecessor-evidence transfer, and premature publication/release remain prohibited.