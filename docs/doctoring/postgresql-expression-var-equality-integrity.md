# PostgreSQL 18 expression `Var` equality integrity

Status: v1 insufficiency preserved / v2 fail-closed successor source repaired / complete `Var` successor open / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-15

## Problem

`IndexExpressionSemanticsSnapshot` deliberately stopped using physical `attnum` as immutable identity and normalized a relation-local PostgreSQL `Var` to `CanonicalExpression::Column(column_name)`. That was useful for preserving the attribute-map result without leaking a child table's physical column order. The first `IndexExpressionNodeSchemaSnapshot` then treated that column-name leaf as if it were already a complete PostgreSQL 18 `equal()` representation.

That completeness claim was too strong. PostgreSQL does not reduce a mapped `Var` to its column number before comparing attached-index expression trees. `CompareIndexInfo()` maps the child expression or predicate with `map_variable_attnos()` and then invokes `equal()` on the complete mapped node tree. `map_variable_attnos_mutator()` first copies the complete `Var` and changes the mapped `varattno` (plus the syntactic attribute number when the syntactic referent is the same range-table entry). The remaining equality-participating `Var` state survives into `equal()`.

In PostgreSQL 18, `Var` carries `varno`, `varattno`, `vartype`, `vartypmod`, `varcollid`, `varnullingrels`, `varlevelsup`, and `varreturningtype`. `varnosyn` and `varattnosyn` are explicitly marked `equal_ignore`; parse locations are also ignored by the equality framework. A column name therefore proves only the attribute-map identity. It does not by itself prove type, type modifier, collation, nulling-relation state, query nesting level, RETURNING behavior, or the target-relation role that PostgreSQL still compares.

A second defect appeared in the first attempt to repair that problem. Commit `afbecf97fb126bfb59edb95911d7214d375849c9` changed the admission behavior of the already domain-separated `node_schema.v1` / revision `postgresql-18-equal-supported-v1` in place. Although the stricter rule was directionally correct, changing the validator while retaining the same digest domain/revision meant an existing v1 digest could no longer be rebound under the exact contract that had issued it. Tightening a semantic contract does not authorize rewriting its immutable predecessor.

## Primary-source basis

`REL_18_STABLE` `CompareIndexInfo()` documents that its `attmap` is built with `build_attrmap_by_name(index2, index1)`. For both `ii_Expressions` and `ii_Predicate`, it maps the second index tree through `map_variable_attnos(..., target_varno=1, sublevels_up=0, ...)`, rejects an unpreservable whole-row reference, and compares the resulting tree to the first index with `equal()`.

`REL_18_STABLE` `map_variable_attnos_mutator()` copies the complete matching `Var` with `*newvar = *var` before replacing a positive `varattno` through the supplied attribute map. It does not reconstruct a new `Var` from a name or erase its type/collation/query-level state.

`REL_18_STABLE` `primnodes.h` marks only `varnosyn` and `varattnosyn` as `equal_ignore`. Its `Var` definition retains type OID, type modifier, collation OID, nulling relations, nesting level, and RETURNING semantics alongside the semantic relation/attribute reference. `equalfuncs.c` states the general rule that parse locations are intentionally not compared.

Consequently an immutable successor that claims parity with PostgreSQL `equal()` must either preserve stable equivalents for every equality-participating `Var` field or prove, from the PostgreSQL index-expression boundary, that a field is fixed to one canonical value. Database-local OIDs and physical child attribute numbers cannot be used as governed identity; they must resolve to stable coordinates before publication.

## Decision

The expression-semantics predecessor remains unchanged. It can record `CanonicalExpression::Column` and `CanonicalExpression::WholeRow` as observational facts.

The historical `node_schema.v1` family is also preserved under its original behavior. `validate_postgres18_equal_schema()` and `IndexExpressionNodeSchemaSnapshot` continue to admit `Column`/`WholeRow` leaves exactly as v1 did when first introduced. V1 remains useful for rebinding old evidence but is explicitly insufficient for new authoritative PostgreSQL expression-equality claims involving relation Vars.

The stricter rule now lives in a new domain-separated v2 successor:

- digest domain: `conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.node_schema.v2`;
- revision: `postgresql-18-equal-supported-v2-var-safe`;
- `validate_postgres18_equal_schema_v2()` first applies the historical v1 field-schema checks and then rejects every current `Column` or `WholeRow` leaf, including nested occurrences;
- `IndexExpressionNodeSchemaSnapshotV2::new()` takes both the exact expression-semantics predecessor and the exact v1 node-schema snapshot, verifies metadata/predecessor identity, reconstructs v1 and compares its digest, then applies v2 validation and frames the exact v1 digest under the new v2 domain.

This preserves immutable v1 semantics while removing the unsafe completeness claim from the current authoritative path. Positive node-schema fixtures use complete zero-argument `FuncExpr` leaves so `FuncExpr`/`OpExpr` field-schema behavior remains independently testable without relying on an incomplete relation `Var`.

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

Behavioral source RED `60fe5da642bf32db0d6487a9c9a1f998dcfd39fe` demonstrated that both `CanonicalExpression::Column("account_email")` and `CanonicalExpression::WholeRow` were accepted by the then-current complete-schema validator even though they lacked PostgreSQL `Var` equality state. Contract-isolation commit `3eac59e2d8d50fe1d154b9d5cedd01f341ce5edd` removed incomplete Vars from the positive modeled-node controls.

The first production attempt `afbecf97fb126bfb59edb95911d7214d375849c9` made the existing v1 validator reject those leaves. Review `5202777524` then identified that this changed an already domain-separated v1 admission contract without changing its digest family/revision. That repair is therefore superseded as a contract-versioning approach rather than treated as final authority.

Behavioral preservation RED `af93518aa6d4467abe680a4fee225278aad41780` requires the historical v1 validator to continue admitting relation-local `Column` and `WholeRow` leaves. It fails on the in-place-hardening predecessor, proving the immutable-family regression.

Production `969b193f14d37af80d5f9522ab289792e2a2b2f3` restores v1 behavior and introduces the domain-separated `IndexExpressionNodeSchemaSnapshotV2` plus `validate_postgres18_equal_schema_v2()`. Regression commit `bdaa26c9f20627a1275a9163e8aae98ea37fd614` locks both sides of the boundary: v1 preserves its historical admission behavior, while v2 rejects the incomplete relation-`Var` leaves and still accepts modeled `FuncExpr`/`OpExpr` trees that contain no unsupported leaves.

These commits are executable source evidence, not an executed Rust RED→GREEN claim. The current execution host has not provided repository-pinned Rust 1.98 acceptance, and protected ConceptWeave `main` still lacks the repository-owned Product pull-request workflow. One unchanged exact #46 head must still pass native and hosted acceptance before any parent adoption or publication.

## Adapter and successor obligation

The concrete PostgreSQL extractor must read the stored expression/predicate node semantics in the same bounded source observation and resolve every OID-bearing `Var` field to stable ConceptWeave coordinates. It must apply the same parent/child attribute mapping semantics as PostgreSQL before governed comparison. Unsupported or contradictory `Var` states fail closed; they do not degrade to column-name equality.

Differential verification must exercise actual PostgreSQL 18 attached-index acceptance and compare it with ConceptWeave admission. Fixtures must include physical-column-order differences that map successfully, plus type modifier, collation, and other equality-state mismatches that remain unequal after `varattno` mapping. Any oracle disagreement remains a release-blocking Source Observation defect.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: index.c (`CompareIndexInfo`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: rewriteManip.c (`map_variable_attnos`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/rewrite/rewriteManip.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: primnodes.h (`Var`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/nodes/primnodes.h

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: equalfuncs.c (`equal`, REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/nodes/equalfuncs.c
