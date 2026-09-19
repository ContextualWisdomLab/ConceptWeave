# PostgreSQL ordinary EXCLUDE operator procedure security-context integrity

## Decision

ConceptWeave must preserve the raw `pg_proc.prosecdef` value for every exact ordinary PostgreSQL `EXCLUDE` operator implementation function bound through `pg_operator.oprcode`. The value is observed after the existing `prokind='f'` predecessor and becomes part of a new immutable, domain-separated successor digest.

This is an observational contract. Both `prosecdef=false` (`SECURITY INVOKER`) and `prosecdef=true` (`SECURITY DEFINER`) are representable source states. ConceptWeave does not add a PostgreSQL rule that ordinary exclusion operators must use one of those states.

## Problem and risk

The predecessor chain already binds the exact constraint/key position, operator, implementation routine, Boolean return semantics, scalar cardinality, strictness, volatility, parallel safety, and normal-function kind. It did not preserve `pg_proc.prosecdef`.

PostgreSQL stores `prosecdef` as an independent Boolean catalog fact. `SECURITY INVOKER` executes with the caller's privileges and is the default; `SECURITY DEFINER` executes with the privileges of the function owner. PostgreSQL also permits `ALTER FUNCTION ... SECURITY INVOKER|SECURITY DEFINER`, so this security boundary can change without changing the input-argument signature used to identify the function.

If the fact is omitted, two source generations with identical stable routine signatures and identical retained auxiliary properties can collapse to the same governed semantic identity while executing under different privilege principals. That is unacceptable for immutable semantic releases, provenance comparison, security review, and downstream client contracts.

`SECURITY DEFINER` additionally carries operational security obligations around `search_path` and execute privileges. Those controls are not inferred or duplicated here; this layer records only the exact source fact it owns.

## Alternatives considered

1. **Ignore `prosecdef` because the routine OID/signature is already bound.** Rejected. PostgreSQL can alter the security mode independently, so identity alone is insufficient evidence of execution privilege semantics.
2. **Require `prosecdef=false`.** Rejected. No authoritative PostgreSQL ordinary-EXCLUDE admission rule was found that forbids a security-definer implementation function. Such a rule would make ConceptWeave stricter than PostgreSQL rather than faithfully describing source truth.
3. **Infer the value from function owner, language, volatility, parallel safety, configuration, or naming conventions.** Rejected. `prosecdef` is an independent catalog field and must come from the exact joined `pg_proc` row.
4. **Preserve the raw Boolean in a successor digest.** Selected. It retains source truth without rewriting predecessor digest meaning or inventing validation semantics.

## Contract

`IndexExclusionConstraintOperatorProcedureSecurityDefinerSnapshot` derives its exact `(constraint coordinate, key_position)` inventory from `IndexExclusionConstraintOperatorProcedureKindSnapshot`.

For every governed position the adapter must provide exactly one `IndexExclusionConstraintOperatorProcedureSecurityDefinerObservation` carrying:

- the same exact ordinary-EXCLUDE coordinate and one-based key position;
- the same stable operator signature as the predecessor;
- the same exact `pg_operator.oprcode` function signature as the predecessor; and
- the raw `pg_proc.prosecdef` Boolean from that exact row.

Missing or duplicate positions, operator/function binding drift, zero positions, and unknown receipt coordinates fail closed. `false` and `true` are both admitted and must produce different successor digests from the same predecessor.

## Live differential requirement

A PostgreSQL 18 live capture must resolve every exact `conexclop` OID to its `pg_operator` row, follow `oprcode` to the exact `pg_proc` row, and read `prosecdef` directly from that row in the same bounded source-content generation as the retained operator, operator-family, access-method, and backing-index controls. Copying or deriving the value from another function property is not evidence.

A positive ordinary control should preserve the observed production value. A distinguishability control may compare identical predecessor evidence with `prosecdef=false` versus `true`; this proves digest separation only and must not be described as evidence that both states are valid for a particular real PostgreSQL DDL fixture.

## Traceability

- Owner module: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_security_definer.rs`
- Focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_security_definer_contract.rs`
- Predecessor: `IndexExclusionConstraintOperatorProcedureKindSnapshot`
- Catalog fact: PostgreSQL 18 `pg_proc.prosecdef`
- Security semantics: PostgreSQL 18 `CREATE FUNCTION`, `SECURITY INVOKER` / `SECURITY DEFINER`
- Independent mutation path: PostgreSQL 18 `ALTER FUNCTION ... SECURITY INVOKER|SECURITY DEFINER`

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html
