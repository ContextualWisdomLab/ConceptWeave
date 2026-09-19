# PostgreSQL transform-converter planner-cost integrity

## Decision

ConceptWeave Source Observation preserves a transform converter's `pg_proc.procost` as an independent same-row fact after converter planner support. The successor records the exact positive finite PostgreSQL `float4` value by its IEEE-754 bit pattern for every selected FROM-SQL and TO-SQL converter direction. It does not infer cost from converter language, definition, volatility, parallel safety, planner support, or transform membership, and it does not impose a product-specific cost threshold.

The governed coordinate remains `(constraint coordinate, key position, transform type, direction)` with the exact converter schema/function binding inherited from the planner-support predecessor.

## Problem

The previous converter successor ended at `pg_proc.prosupport`. PostgreSQL stores `procost` separately as `float4`, so two same-generation converter rows can agree on all already governed facts while differing only in estimated execution cost. Without an explicit successor those states collapse to one governed observation digest.

That loss is material even when a planner support function is attached. PostgreSQL's declarative `COST` property supplies a constant estimated execution cost, while a support function can optionally provide non-constant estimates for particular calls through `SupportRequestCost`. The existence or identity of `prosupport` therefore does not determine the catalog's `procost` value.

## PostgreSQL authority

PostgreSQL 18 `pg_proc` defines `procost` as `float4`, the estimated execution cost in units of `cpu_operator_cost`; for set-returning functions the value is per returned row. The same catalog lists `prosupport`, `provolatile`, `proparallel`, `proconfig`, and other routine facts separately.

PostgreSQL 18 `CREATE FUNCTION` defines `COST execution_cost` as a positive number. If omitted, PostgreSQL assumes 1 unit for C-language and internal functions and 100 units for functions in other languages. The value is an estimate used by the planner, not a semantic validity threshold for ConceptWeave.

Section 36.11 distinguishes declarative constant execution cost from an optional planner support function's `SupportRequestCost`, which can provide a non-constant estimate for a particular call. That distinction is why converter cost remains a separate Source Observation fact after planner-support identity.

## Chosen representation

`IndexExclusionConstraintOperatorProcedureTransformConverterCostObservation` repeats only the identity necessary to bind raw cost to its predecessor:

- ordinary exclusion-constraint coordinate;
- one-based exclusion-key position;
- exact transform type;
- FROM-SQL or TO-SQL direction;
- exact converter schema and function name;
- exact positive finite `f32` execution cost, retained internally by raw bits.

`IndexExclusionConstraintOperatorProcedureTransformConverterCostSnapshot` accepts `IndexExclusionConstraintOperatorProcedureTransformConverterPlannerSupportSnapshot` as its sole predecessor. It requires exact predecessor coordinate coverage, rejects duplicates and extra/missing directions, and rejects converter-function binding drift. Its digest is domain-separated from the planner-support predecessor and incorporates the raw `float4` bits so distinct catalog values remain distinct.

The canonical provenance location retains the qualified-transform-type collision repair: transform schema and type-name components are percent-encoded independently before the canonical `.` separator.

## Alternatives rejected

Inferring cost from implementation language or PostgreSQL defaults was rejected because `procost` is already materialized catalog state and can be explicitly changed.

Treating planner-support presence as a replacement for `procost` was rejected because PostgreSQL keeps constant `COST` and dynamic `SupportRequestCost` as distinct planner inputs.

Normalizing costs into rounded decimal text was rejected because `pg_proc.procost` is `float4`; exact raw bits provide deterministic observation identity without presentation-dependent rounding.

Imposing a minimum performance threshold beyond PostgreSQL's positive-value invariant was rejected because Source Observation records authoritative database state rather than creating a product policy for acceptable planner cost.

## Invariants and failure behavior

The converter-cost successor fails closed when any predecessor converter direction is missing, an extra coordinate appears, a coordinate is duplicated, key position is zero, converter identifiers are blank, converter binding differs from the planner-support predecessor, or an exact receipt location is unknown. Non-positive and non-finite values are rejected as invalid capture inputs.

The bounded PostgreSQL 18 differential must resolve each selected converter and its `procost` from the same observation generation as the other converter `pg_proc` facts. Mixed-generation joins, language-default inference in place of catalog capture, textual rounding that changes `float4` identity, or a product-specific cost rewrite are capture failures.

## TRACEABILITY

- PR: ConceptWeave #46, Draft Source Observation single writer.
- Finding review: `5250941523` at exact planner-support head `913577072165cb09fbf52bbdb2e1216fa2b21005`.
- RED contract: `c876bb3907f9d33da3973196937fd854ac3aae85`.
- Production successor: `f82ca656ac238c1ce7015cddd327169abd7ce054`.
- Public composition: `5032b9cbeacd784f6b36d1393476a24b00f7529a`.
- Source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_cost.rs`.
- Contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_cost_contract.rs`.
- Public composition: `crates/conceptweave-relation-partition/src/index_partition.rs`.

No exact-head GREEN is implied by this trace. Rust 1.98 formatting, strict Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the bounded PostgreSQL 18 live differential remain acceptance gates until executed on the final exact head.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 36.11. Function optimization information*. https://www.postgresql.org/docs/18/xfunc-optimization.html
