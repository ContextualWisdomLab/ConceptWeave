# PostgreSQL ordinary EXCLUDE implementation-function planner-cost integrity

## Decision

ConceptWeave Source Observation preserves exact same-row PostgreSQL 18 `pg_proc.procost` as a versioned observational successor after planner-support evidence for every governed ordinary `EXCLUDE` operator implementation function.

`procost` is not inferred from the function language, implementation definition, volatility, parallel-safety state, planner-support identity, operator family, or any product performance heuristic. The adapter must read the exact `float4` value from the same `pg_proc` row already bound through `pg_operator.oprcode` in the same bounded source-content generation.

The domain accepts only positive finite `float4` capture. It stores the IEEE-754 `f32` bits and hashes those bits directly, avoiding locale- or formatting-dependent decimal serialization. Distinct positive finite PostgreSQL `float4` values therefore produce distinct successor identities. No product-specific cost threshold is imposed.

`pg_proc.prorows` is intentionally not added in this successor. The retained predecessor requires `proretset=false`, and PostgreSQL 18 documents `prorows` as zero when `proretset` is false. A future set-returning function family would require a separately reviewed row-estimate successor rather than silently widening this digest domain.

## Problem

The predecessor chain already binds exact `conexclop -> pg_operator -> oprcode -> pg_proc`, result/cardinality, strictness, volatility, parallel safety, routine kind, security-definer/leakproof state, implementation definition, owner, configuration, ACL, and optional planner-support function. It did not preserve the target function's `procost`.

PostgreSQL uses `procost` as the estimated execution cost in units of `cpu_operator_cost`. Larger values influence the optimizer to avoid repeated evaluation where possible. PostgreSQL also permits the property to be changed with `ALTER FUNCTION ... COST` without changing the function call signature. Therefore the same governed operator/function identity and all predecessor facts do not prove equivalent planner-cost semantics.

## Alternatives considered

1. **Ignore `procost` as optimizer-only metadata.** Rejected because ConceptWeave's source identity already preserves planner-affecting `prosupport`; omitting another independently mutable planner fact would make semantically different catalog states collapse.
2. **Normalize cost to decimal text.** Rejected because formatting rules are not the source contract and can create avoidable canonicalization ambiguity. The catalog type is `float4`; the successor preserves the validated `f32` bit pattern.
3. **Impose a preferred or maximum cost.** Rejected. PostgreSQL defines a positive execution estimate, not a ConceptWeave policy threshold. This successor observes source truth rather than inventing an admission policy.
4. **Add `prorows` at the same time.** Rejected for this bounded lineage because the already-governed implementation function is scalar (`proretset=false`) and PostgreSQL documents `prorows=0` in that case.

## Invariants

- every predecessor `(constraint coordinate, key_position)` has exactly one cost observation;
- key positions are one-based;
- operator and `oprcode` function signatures exactly match the planner-support predecessor;
- `execution_cost` is positive and finite;
- digest input contains predecessor digest, exact coordinate, key position, operator, procedure, and raw `f32::to_bits()` cost;
- missing/duplicate coordinates, binding drift, invalid cost, and unknown receipt coordinates fail closed;
- predecessor digest domains are never rewritten.

## TRACEABILITY

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5232151401`
- structural source/compile RED: `8b71c5e4c8d7f80e2cb7b4b0c8236d56136252cf`
- production successor: `8fd9bda8ce33c5f2ece7b000c8ccdcb6147681c1`
- public composition: `46772a3dd991603a0ef4638ecc000182c8b6e795`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_cost.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_cost_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot`
- exact catalog fact: `pg_proc.procost`

No executed Rust RED/GREEN is claimed by these commits. Exact-head acceptance still requires the repository-pinned Rust 1.98 native and hosted gates on one unchanged head.

## Primary references

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html
