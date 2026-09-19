# PostgreSQL transform-converter planner-support integrity

## Decision

ConceptWeave Source Observation preserves a transform converter's `pg_proc.prosupport` as an independent same-row fact after converter parallel safety. The successor represents either explicit absence (`prosupport = 0`) or the exact resolved planner-support function identity. It does not infer support from converter definition, volatility, parallel safety, cost, operator semantics, or transform membership, and it does not impose a policy that converters must have a support function.

This decision applies to every nonzero FROM-SQL and TO-SQL converter selected by the ordinary-`EXCLUDE` transform chain. The governing coordinate remains `(constraint coordinate, key position, transform type, direction)` with exact converter schema/function binding inherited from the parallel-safety predecessor.

## Problem

The prior converter successor ended at raw `pg_proc.proparallel`. PostgreSQL stores `prosupport` independently as a `regproc` reference to another `pg_proc` row, or zero when no support function is attached. Two exact converter rows can therefore agree on all previously governed facts while differing only in planner-support absence or support-function identity. Without an explicit successor those states collapse to the same governed observation digest.

That loss is material to semantic provenance. PostgreSQL can invoke a planner support function while planning calls to its target function, and support functions can simplify calls, estimate selectivity or cost, estimate rows, or derive index conditions. PostgreSQL also states that the support function is attached to the target through the `SUPPORT` clause rather than derived from the target's other attributes.

## PostgreSQL authority

PostgreSQL 18 `pg_proc` defines `prosupport` as `regproc`, referencing `pg_proc.oid`, with zero meaning no planner support function. The same catalog documents `procost`, `provolatile`, `proparallel`, `proconfig`, `proacl`, and other routine facts as separate columns. This separation is the catalog-level reason not to infer `prosupport` from an existing successor.

PostgreSQL 18 `CREATE FUNCTION` exposes `SUPPORT support_function` independently from volatility, leakproofness, strictness, security mode, parallel safety, cost, rows, and configuration. Section 36.11 states that a planner support function is attached to an SQL-callable target function, has SQL signature `supportfn(internal) returns internal`, and may supply planner knowledge that cannot be expressed by declarative annotations.

## Chosen representation

`IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportObservation` repeats only the identity needed to bind the new fact to its predecessor:

- ordinary exclusion-constraint coordinate;
- one-based exclusion-key position;
- exact transform type;
- FROM-SQL or TO-SQL direction;
- exact converter schema and function name;
- optional `QualifiedProcedureSignature` resolved from converter `prosupport`.

`IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot` accepts `IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot` as its sole predecessor. It requires exact predecessor coordinate coverage, rejects duplicates and extra/missing directions, and rejects converter-function binding drift. The snapshot digest is domain-separated from the predecessor and distinguishes absence from each exact support-function signature.

The canonical provenance location continues the qualified-transform-type collision repair: transform schema and type-name components are percent-encoded independently before the canonical `.` separator. A dotted schema name and a dotted type name therefore remain distinct locations.

## Alternatives rejected

Treating missing support and an attached support function as equivalent was rejected because it erases a first-class `pg_proc` reference that can affect planner behavior.

Requiring every converter to have `SUPPORT` was rejected because PostgreSQL explicitly permits absence and treats planner support as an optional advanced feature. Source Observation records database state; it does not invent a planner-support admission policy.

Folding converter cost into this successor was rejected because `pg_proc.procost` is another independently mutable catalog fact with its own units and defaults. Cost remains the next review-gated successor.

Resolving support by unqualified name was rejected because PostgreSQL functions are overloaded and schema-scoped. The observation stores a qualified call signature so support identity does not collapse across namespaces or argument lists.

## Invariants and failure behavior

The planner-support successor fails closed when any predecessor converter direction is missing, an extra coordinate appears, a coordinate is duplicated, key position is zero, converter identifiers are blank, converter binding differs from the parallel-safety predecessor, or an exact receipt location is unknown. Support absence is valid and distinct from support presence.

The bounded PostgreSQL 18 differential must resolve each selected converter and its `prosupport` from the same observation generation. A nonzero `prosupport` must resolve to the exact support-function call identity. Mixed-generation joins, name-only resolution, or inferring support from another function property are capture failures.

## TRACEABILITY

- PR: ConceptWeave #46, Draft Source Observation single writer.
- Finding review: `5250887046` at exact predecessor head `1fbbfadec507999710a8ca5c00378685d017326a`.
- RED contract: `f7a92af4d18e964680f514514020b712942ffc49`.
- Production successor: `3b048dae7cefed4c1615d5bfe5659be57d044eb9`.
- Public composition: `9bbb668767ddaa24f6934568a7340ef1cbc504ad`.
- Source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_planner_support.rs`.
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_planner_support_contract.rs`.
- Public composition: `crates/conceptweave-relation-partition/src/index_partition.rs`.

No exact-head GREEN is implied by this trace. Exact `9bbb668...` had no pull-request workflow runs at the first inventory query. Rust 1.98 formatting, strict Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 live differential remain acceptance gates.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 36.11. Function optimization information*. https://www.postgresql.org/docs/18/xfunc-optimization.html
