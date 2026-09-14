# PostgreSQL 18 expression `Var` equality integrity

Status: source repaired by fail-closed validation / complete `Var` successor open / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-15

## Problem

`IndexExpressionSemanticsSnapshot` deliberately stopped using physical `attnum` as immutable identity and normalized a relation-local PostgreSQL `Var` to `CanonicalExpression::Column(column_name)`. That was useful for preserving the attribute-map result without leaking a child table's physical column order. The later `IndexExpressionNodeSchemaSnapshot`, however, treated that column-name leaf as if it were already a complete PostgreSQL 18 `equal()` representation.

That completeness claim was too strong. PostgreSQL does not reduce a mapped `Var` to its column number before comparing attached-index expression trees. `CompareIndexInfo()` maps the child expression or predicate with `map_variable_attnos()` and then invokes `equal()` on the complete mapped node tree. `map_variable_attnos_mutator()` first copies the complete `Var` and changes the mapped `varattno` (plus the syntactic attribute number when the syntactic referent is the same range-table entry). The remaining equality-participating `Var` state survives into `equal()`.

In PostgreSQL 18, `Var` carries `varno`, `varattno`, `vartype`, `vartypmod`, `varcollid`, `varnullingrels`, `varlevelsup`, and `varreturningtype`. `varnosyn` and `varattnosyn` are explicitly marked `equal_ignore`; parse locations are also ignored by the equality framework. A column name therefore proves only the attribute-map identity. It does not by itself prove type, type modifier, collation, nulling-relation state, query nesting level, RETURNING behavior, or the target-relation role that PostgreSQL still compares.

This is a source-integrity defect rather than a transport nicety. ConceptWeave already carries qualified column type, structured `pg_attribute.atttypmod`, and column-collation evidence in neighboring families, but `IndexExpressionNodeSchemaSnapshot` did not compose those facts into each `Var` leaf. Allowing `Column(String)` to pass `postgresql-18-equal-supported-v1` therefore certified completeness that the successor could not prove.

## Primary-source basis

`REL_18_STABLE` `CompareIndexInfo()` documents that its `attmap` is built with `build_attrmap_by_name(index2, index1)`. For both `ii_Expressions` and `ii_Predicate`, it maps the second index tree through `map_variable_attnos(..., target_varno=1, sublevels_up=0, ...)`, rejects an unpreservable whole-row reference, and compares the resulting tree to the first index with `equal()`.

`REL_18_STABLE` `map_variable_attnos_mutator()` copies the complete matching `Var` with `*newvar = *var` before replacing a positive `varattno` through the supplied attribute map. It does not reconstruct a new `Var` from a name or erase its type/collation/query-level state.

`REL_18_STABLE` `primnodes.h` marks only `varnosyn` and `varattnosyn` as `equal_ignore`. Its `Var` definition retains type OID, type modifier, collation OID, nulling relations, nesting level, and RETURNING semantics alongside the semantic relation/attribute reference. `equalfuncs.c` states the general rule that parse locations are intentionally not compared.

Consequently an immutable successor that claims parity with PostgreSQL `equal()` must either preserve stable equivalents for every equality-participating `Var` field or prove, from the PostgreSQL index-expression boundary, that a field is fixed to one canonical value. Database-local OIDs and physical child attribute numbers cannot be used as governed identity; they must resolve to stable coordinates before publication.

## Decision

The existing expression-semantics predecessor remains unchanged. It can still record `CanonicalExpression::Column` and `CanonicalExpression::WholeRow` as observational facts, and its digest family is not rewritten.

The newer PostgreSQL-18 node-schema validation successor now fails closed on both leaf forms. `validate_postgres18_equal_schema()` admits `FuncExpr` and `OpExpr` only when every nested leaf is itself supported by the complete equality-schema gate. This removes the false completeness claim while preserving the predecessor as historical evidence.

Positive node-schema fixtures no longer rely on an incomplete relation `Var`. They use a complete zero-argument `FuncExpr` leaf so `FuncExpr`/`OpExpr` field-schema behavior remains independently testable while the `Var` successor is open.

A later domain-separated `Var` semantics successor may re-admit relation variables only after it represents or proves the following PostgreSQL equality state:

- target relation role (`varno`) under the stored-index expression boundary;
- attribute-map-normalized column identity replacing physical `varattno`;
- stable qualified value type replacing `vartype` OID;
- exact raw type modifier corresponding to `vartypmod`;
- stable qualified collation replacing `varcollid` OID;
- nulling-relation state;
- `varlevelsup`;
- `varreturningtype`.

The successor must continue to exclude `varnosyn`, `varattnosyn`, and parse location from semantic identity because PostgreSQL equality ignores them. It must not use `pg_get_expr`, `pg_node_tree`/`nodeToString`, reconstructed DDL, rendered type text, or raw database-local OIDs as substitutes.

## Repair lineage

Review `5202734131` identified the incomplete `Var` equality claim on exact predecessor `44013dada8b6a830abe6f51cc0340bd87f21f498`.

Behavioral source RED `60fe5da642bf32db0d6487a9c9a1f998dcfd39fe` adds `relation_var_leaves_fail_closed_until_their_full_equal_schema_is_modeled`. On the predecessor, both `CanonicalExpression::Column("account_email")` and `CanonicalExpression::WholeRow` were accepted by `validate_postgres18_equal_schema()`, contradicting the required fail-closed contract.

Contract-isolation commit `3eac59e2d8d50fe1d154b9d5cedd01f341ce5edd` removes incomplete `Var` leaves from the positive `FuncExpr`/`OpExpr` schema controls. The positive path instead uses a complete zero-argument `FuncExpr`, so the new RED remains specifically about relation-`Var` completeness rather than making the modeled node kinds untestable.

Minimal production repair `afbecf97fb126bfb59edb95911d7214d375849c9` changes only the node-schema gate: `CanonicalExpression::Column` and `CanonicalExpression::WholeRow` now return `canonical_expression_node_schema`, including when nested under otherwise complete `FuncExpr`/`OpExpr` nodes. No predecessor digest family, expression canonicalization, rendered SQL, OID resolution, or attachment topology is changed.

These commits are executable source evidence, not an executed Rust RED→GREEN claim. The current execution host has not provided repository-pinned Rust 1.98 acceptance, and protected ConceptWeave `main` still lacks the repository-owned Product pull-request workflow. One unchanged exact #46 head must still pass native and hosted acceptance before any parent adoption or publication.

## Adapter and successor obligation

The concrete PostgreSQL extractor must read the stored expression/predicate node semantics in the same bounded source observation and resolve every OID-bearing `Var` field to stable ConceptWeave coordinates. It must apply the same parent/child attribute mapping semantics as PostgreSQL before governed comparison. Unsupported or contradictory `Var` states fail closed; they do not degrade to column-name equality.

Differential verification must exercise actual PostgreSQL 18 attached-index acceptance and compare it with ConceptWeave admission. Fixtures must include physical-column-order differences that map successfully, plus type modifier, collation, and other equality-state mismatches that remain unequal after `varattno` mapping. Any oracle disagreement remains a release-blocking Source Observation defect.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: index.c (`CompareIndexInfo`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: rewriteManip.c (`map_variable_attnos`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/rewrite/rewriteManip.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: primnodes.h (`Var`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/nodes/primnodes.h

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: equalfuncs.c (`equal`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/nodes/equalfuncs.c
