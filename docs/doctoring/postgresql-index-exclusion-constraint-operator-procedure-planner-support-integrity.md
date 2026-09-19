# PostgreSQL ordinary-EXCLUDE implementation-function planner-support integrity

## Decision

ConceptWeave Source Observation must preserve the exact `pg_proc.prosupport` state of every implementation function reached through an ordinary `pg_constraint.contype='x'` exclusion operator. `prosupport = 0` is explicit absence. A nonzero `prosupport` must be resolved from the same bounded PostgreSQL source generation to a stable support-function signature and bound to the exact operator/function coordinate already governed by the access-control predecessor.

This is observational evidence, not a new PostgreSQL admission rule. ConceptWeave does not require an ordinary-EXCLUDE function to have a planner support function, and it does not infer `prosupport` from the target function's language, volatility, cost, implementation body, operator family, or any other retained property.

## Problem

The predecessor chain already preserves exact `conexclop -> pg_operator -> oprcode -> pg_proc`, target-function execution flags, implementation definition, owner, local run-time configuration, and ACL state. PostgreSQL 18 nevertheless stores `prosupport` independently in `pg_proc` as a `regproc` reference to a planner support function, or zero if no support function is attached.

A stable target-function signature does not determine planner-support state. PostgreSQL permits a target function to name a support function with the `SUPPORT` clause, and planner support can simplify calls, estimate selectivity, estimate nonconstant execution cost, estimate result rows, and provide index conditions. PostgreSQL's function-optimization documentation explicitly states that simplification also applies to operators based on the target function. Omitting `prosupport` therefore permits two catalog states with different planning semantics to collapse into one governed source identity.

## Constraints

- Preserve all issued predecessor digest domains unchanged.
- Keep the exact `(constraint coordinate, key_position, operator, target function)` binding from the access-control predecessor.
- Treat `prosupport = 0` and a nonzero support reference as distinct states.
- Resolve a nonzero support OID in the same source generation; mutable-head or later-session resolution is not admissible evidence.
- Store a stable resolved support-function signature in the successor digest, not the adapter-local OID.
- Do not claim that a synthetic support-function fixture proves production validity or ordinary-EXCLUDE admission.
- Do not recursively copy foreign or mutable implementation state into this layer. Any later requirement to bind the support function's own implementation row must be a separately reviewed successor rather than a silent expansion of this digest domain.

## Alternatives considered

### Ignore `prosupport` because it is only an optimization hint

Rejected. Planner support can transform target calls and operator expressions and can supply planner selectivity/cost/row/index-condition information. That is observable planner behavior, not merely presentation metadata.

### Infer support from the target function or operator family

Rejected. PostgreSQL stores `prosupport` as an independent catalog reference. None of the retained target-function flags, implementation definition, cost, operator family, or access method proves the referenced support function.

### Require support to be present

Rejected. PostgreSQL explicitly allows zero (`no support function`). ConceptWeave records the source fact and does not invent a stronger EXCLUDE admission rule.

### Persist the raw support OID

Rejected. OIDs are capture-time join coordinates and are not portable semantic identifiers. The adapter resolves the exact OID inside the bounded generation and supplies the stable schema/name/input-type signature to the domain boundary.

## Implementation and invariants

`IndexExclusionConstraintOperatorProcedurePlannerSupportSnapshot` succeeds only when planner-support observations form the exact unique coordinate set inherited from `IndexExclusionConstraintOperatorProcedureAccessControlSnapshot`. The repeated operator and target-function signatures must match the predecessor exactly. Zero key positions, missing evidence, duplicate coordinates, operator drift, target-function drift, and unknown receipt coordinates fail closed.

The successor digest domain includes the predecessor digest, exact constraint coordinate, key position, operator signature, target-function signature, an explicit absence/presence discriminator, and the resolved support-function signature when present. A change from absence to presence or between two resolved support identities therefore changes the governed digest without rewriting any predecessor domain.

The unit fixture models the documented planner-support call signature `supportfn(internal) returns internal` at the input-signature boundary. It is only a digest/binding control; the fixture does not claim that the invented support-function names exist in PostgreSQL.

## Live differential requirement

The PostgreSQL 18 bounded differential must read `pg_proc.prosupport` from the exact same target-function row already joined from `pg_operator.oprcode`. If zero, record explicit absence. If nonzero, resolve that OID to the exact `pg_proc` row in the same source-content generation and reduce its stable schema/name/input-argument identity before entering the domain boundary. A nonzero unresolved reference is a capture failure, not `None`.

The differential must retain all earlier ordinary-EXCLUDE controls in the same v3 source-content generation, including operator kind/commutator/result/implementation binding, target-function properties and access control, operator-family/strategy, backing-index namespace/lifecycle/access-method capability, and constraint catalog state.

## TRACEABILITY

- Owner PR: `ContextualWisdomLab/ConceptWeave#46`
- Finding review: `5231648255`
- Structural source/compile RED: `e77373013fb8340038a426d84ca5af6588da63c1`
- Production successor: `9212b5211ad75e17f8afe43d29dec0a309054b8a`
- Public composition: `0802f62c95adba09f25fe58fdb4df84038940fd3`
- Fixture signature correction: `f2af547d3a95affd79d2cd70c6ddc402bbdfbfb6`
- Source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_planner_support.rs`
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_planner_support_contract.rs`
- Predecessor: `IndexExclusionConstraintOperatorProcedureAccessControlSnapshot`
- Exact catalog fact: `pg_proc.prosupport`

No executed Rust RED or GREEN is asserted by this decision record. Source movement resets exact-head execution and review evidence.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18.6 documentation: 36.11. Function optimization information*. https://www.postgresql.org/docs/18/xfunc-optimization.html
