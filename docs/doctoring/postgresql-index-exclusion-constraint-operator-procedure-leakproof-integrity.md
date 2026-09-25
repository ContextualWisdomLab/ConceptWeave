# PostgreSQL ordinary-EXCLUDE implementation-function leakproofness integrity

## Decision

ConceptWeave Source Observation preserves raw PostgreSQL 18 `pg_proc.proleakproof` for every exact ordinary-`EXCLUDE` `pg_operator.oprcode -> pg_proc` implementation function. The fact is layered after the existing procedure-kind/security-context chain and receives its own digest domain.

This is an observational contract, not a new PostgreSQL admission rule. Both `proleakproof=false` and `proleakproof=true` are representable when actually observed. The governed identity must distinguish them, but ConceptWeave does not claim that ordinary `EXCLUDE` requires one particular value.

## Problem

Before this repair, the bounded chain retained the exact exclusion operator, binary operator kind, commutator, exact implementation function, Boolean result, scalar cardinality, strictness, volatility, parallel safety, normal-function kind, and `SECURITY INVOKER`/`SECURITY DEFINER` mode. It still omitted `pg_proc.proleakproof`.

PostgreSQL stores leakproofness independently. A leakproof function reveals no information about its arguments except through its return value; the planner may evaluate leakproof operators/functions before security-barrier view or row-level-security conditions. Planner statistics access is also affected when user privileges would otherwise prevent applying a user-defined operator to protected statistics. `ALTER FUNCTION ... LEAKPROOF|NOT LEAKPROOF` can change this property without changing the function's input-argument identity.

Therefore two otherwise identical `oprcode -> pg_proc` bindings can have different security/planner semantics while collapsing to the same governed digest if raw `proleakproof` is absent.

## Contract

`IndexExclusionConstraintOperatorProcedureLeakproofSnapshot` derives the exact `(constraint coordinate, key_position)` inventory from `IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot`.

For every predecessor position it requires exactly one raw `proleakproof` observation bound to the same stable operator and exact implementation function. Missing or duplicate evidence, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. The successor digest includes the predecessor digest, exact coordinate/key position, stable operator/function signatures, and the raw Boolean.

The unit contract uses synthetic `false`/`true` pairs only to prove digest distinguishability. Such pairs are not evidence that both states are accepted by a particular real ordinary-EXCLUDE DDL fixture.

## Live differential obligation

A PostgreSQL 18 differential must resolve each exact `conexclop` to one `pg_operator` row, independently read `oprkind`, `oprcom`, `oprresult`, and `oprcode`, follow `oprcode` to the exact `pg_proc` row, and independently read `prokind`, `prosecdef`, `proleakproof`, `prorettype`, `proretset`, `proisstrict`, `provolatile`, and `proparallel`. `proleakproof` may not be inferred from function name, owner, language, security mode, strictness, volatility, parallel safety, result/cardinality, or routine kind.

Retained operator-family/strategy, backing-index namespace/lifecycle/access-method/catalog controls, and exact v3 source-content-generation binding remain required in the same bounded capture.

## TRACEABILITY

- PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5230464081`
- structural source/compile RED: `cd76aebfa29aa8ad19bfc9dd2c3d9d5447070112`
- production successor: `fe9d58d450863ffe5fb4316befe8b03d93c7085f`
- public composition: `513e858ff01a179f0e4275c972ff1a2bd45b13ea`
- production: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_leakproof.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_leakproof_contract.rs`
- predecessor: `IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot`
- exact catalog fact: `pg_proc.proleakproof`

No executed compiler RED or GREEN is claimed by the structural RED commit. Exact-head Rust 1.98 and hosted acceptance remain separate obligations.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: Rules and privileges*. https://www.postgresql.org/docs/18/rules-privileges.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: Planner statistics and security*. https://www.postgresql.org/docs/18/planner-stats-security.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: System catalogs*. https://www.postgresql.org/docs/18/catalogs.html
