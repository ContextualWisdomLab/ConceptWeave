# PostgreSQL ordinary EXCLUDE implementation-procedure parallel-safety integrity

## Decision

ConceptWeave preserves raw PostgreSQL 18 `pg_proc.proparallel` for every exact ordinary `EXCLUDE` operator implementation procedure selected through `pg_operator.oprcode`. The value is observational evidence, not a new EXCLUDE validity rule.

`IndexExclusionConstraintOperatorProcedureParallelSafetySnapshot` succeeds the exact `IndexExclusionConstraintOperatorProcedureVolatilitySnapshot`. Each constraint/key position must carry exactly one independently observed raw `proparallel` discriminator bound to the same stable operator and exact implementation procedure. The accepted PostgreSQL catalog values are `s` (parallel safe), `r` (parallel restricted), and `u` (parallel unsafe). All three remain representable and enter a new domain-separated digest.

## Problem

The predecessor chain already preserves the exact `conexclop -> pg_operator -> oprcode -> pg_proc` binding plus binary operator kind, Boolean result identity, scalar cardinality, raw strictness, and raw volatility. That is not enough to reconstruct `proparallel`.

PostgreSQL stores `proparallel` as its own `pg_proc` column. It controls whether a routine may run in parallel workers, only in the parallel group leader, or only in a serial plan. PostgreSQL also documents that the planner cannot infer user-defined function parallel safety automatically. A routine incorrectly labeled safer than its behavior permits can fail or return wrong answers in a parallel query. Collapsing `s`, `r`, and `u` under one governed procedure identity would therefore discard material planner/execution semantics.

## Constraints and alternatives

The repair must not change any issued predecessor digest domain. Procedure signature, result type, `proretset`, `proisstrict`, and `provolatile` are insufficient substitutes because PostgreSQL models parallel safety independently.

Three alternatives were considered:

1. infer parallel safety from procedure name, language, volatility, or operator family — rejected because PostgreSQL explicitly does not infer this property from routine behavior;
2. require ordinary EXCLUDE implementation functions to be `PARALLEL SAFE` — rejected because no EXCLUDE-specific PostgreSQL admission rule establishing that requirement was found;
3. preserve raw `proparallel` independently and bind it to the exact predecessor procedure — selected because it retains source truth without broadening PostgreSQL validity semantics.

## Invariants

- Every exact predecessor constraint/key coordinate has exactly one parallel-safety observation.
- Duplicate, missing, or extra coordinates fail closed.
- The repeated stable operator and exact `oprcode` procedure must equal the volatility predecessor binding.
- Only raw `s`, `r`, and `u` are accepted.
- The new digest includes the predecessor digest, exact coordinate/key position, stable operator, exact procedure, and raw `proparallel` byte.
- `proparallel` is never inferred from `provolatile`, `proisstrict`, `proretset`, return type, procedure name, operator family, or strategy.
- The observation layer does not claim that all three states are valid for a specific ordinary EXCLUDE DDL definition; non-production states can serve as distinguishability controls.

## Risk and effect

The repair prevents two captures with identical normalized procedure identity and retained behavior fields but different planner/execution parallel-safety state from collapsing to one semantic identity. It also makes a live-differential adapter prove that it read `pg_proc.proparallel` from the exact procedure row rather than synthesizing it from another field.

The remaining risk is execution evidence. Source-shaped tests do not establish Rust 1.98 GREEN or a PostgreSQL 18 live differential. One unchanged exact head still requires native/hosted acceptance and a bounded catalog capture reading `proparallel` independently with the retained operator, procedure, backing-index, namespace, lifecycle, and source-generation controls.

## TRACEABILITY

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5229512857`
- structural source/compile contract: `7b1ccf482d6240427f104d8c19ad04874d546e69`
- production successor: `d7fbb00b64b09863577cbaf0110fd48379b6c5b5`
- public composition: `be0662a61417ae8e9eaa01bf75d5bdbe001c1265`
- production source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_parallel_safety.rs`
- focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_parallel_safety_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureVolatilitySnapshot`
- source fact: PostgreSQL 18 `pg_proc.proparallel`

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 15.4. Parallel safety*. https://www.postgresql.org/docs/18/parallel-safety.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html
