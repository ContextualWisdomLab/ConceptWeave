# PostgreSQL transform-converter argument-mode integrity

## Decision

ConceptWeave Source Observation preserves nullable raw `pg_proc.proargmodes` for every selected FROM-SQL and TO-SQL transform converter. NULL remains distinct from a non-NULL mode vector and both participate in the governed successor digest.

This is a source-fidelity requirement, not a new transform-admission policy. PostgreSQL 18's `check_transform_function()` validates volatility, normal-function kind, non-set return, one input argument, and `internal` input type, but it does not inspect `proargmodes`. The existing ConceptWeave chain already binds those checked predicates plus FROM/TO return types. It did not, however, distinguish argument-mode catalog shapes that share the same input and return types.

## Primary-source basis

PostgreSQL 18 documents `proargtypes` as the input-argument vector, explicitly including `INOUT` and `VARIADIC`. `proallargtypes` contains input and output arguments, and `proargmodes` records `i` (`IN`), `o` (`OUT`), `b` (`INOUT`), `v` (`VARIADIC`), and `t` (`TABLE`). `proargmodes` is NULL when every argument is ordinary `IN`.

The PostgreSQL 18 parameter-list implementation confirms that catalog representation. It builds `parameterModes` only when there is at least one output or variadic parameter; otherwise the mode array remains absent. The same implementation counts `INOUT` as both input and output, requires a variadic parameter to be an array type, and makes multiple output parameters return `record`.

`check_transform_function()` separately requires `pronargs == 1` and `proargtypes[0] == INTERNALOID`. It does not read `proargmodes`. Therefore a normalized one-`internal` input plus normalized return type is not enough to preserve the raw converter signature shape.

## Contract boundary

`IndexExclusionConstraintOperatorProcedureTransformConverterArgumentModesObservation` accepts the PostgreSQL catalog states that remain coherent with the already-governed predecessor facts:

- `None`: canonical all-`IN` representation for the one input argument;
- one `INOUT` mode for a FROM-SQL converter, where the `internal` input is also the `internal` output;
- one ordinary `IN` plus one `OUT` mode, in either catalog order, with the predecessor return-type binding continuing to determine the direction-specific output type.

It rejects non-NULL all-`IN` vectors because PostgreSQL stores that state as NULL; vectors with zero or multiple input-capable modes because `pronargs=1` is already bound; `VARIADIC` because the bound input is `internal` rather than an array; `TABLE` because the predecessor binds `proretset=false`; multiple output modes because PostgreSQL would derive a `record` result; and TO-SQL `INOUT internal` because TO SQL must return the transform type, not `internal`.

Argument names are deliberately not folded into this successor. They do not alter the input type identity used by transform resolution and require a separate materiality review before becoming governed semantic evidence.

## Traceability

Finding review: `5251993832` on #46 exact head `95e5fb8e19d336b43c6dde3fa7767542b22505c6`.

RED contract: `6ecc6418fce88b3aaf1519e7c422b2a1583b5407`.

Production successor: `39602ea78c4bbff856c6274052918c4c0541e9df`.

Public composition: `a3c61e33fbd84c86e9167d17377072df3e0e6ec0`.

The focused contract distinguishes NULL, FROM-SQL `INOUT`, and TO-SQL `IN`+`OUT` digests; rejects impossible mode shapes; checks exact predecessor inventory and converter binding; preserves collision-safe quoted transform-type provenance; and keeps receipt lookup exact-coordinate bound.

No execution result from an earlier head is transferable. Final acceptance still requires exact-head Rust 1.98 formatting, strict Clippy, focused/retained/workspace/doc tests, release/rustdoc, owned coverage, and a bounded PostgreSQL 18 live differential that reads `proargmodes` from the same `pg_proc` row as the other converter facts.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: src/backend/commands/functioncmds.c* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/functioncmds.c