# PostgreSQL 18 relation-`Var` successor integrity

Status: Source repair committed; native/hosted acceptance and concrete PostgreSQL extraction remain open.

## Problem

`IndexExpressionNodeSchemaSnapshotV2` intentionally rejects every historical `CanonicalExpression::Column` / `WholeRow` relation-`Var` leaf. That is a correct fail-closed contract, but it also means a later relation-`Var` successor cannot literally consume a successful v2 snapshot for a relation-Var-bearing expression: such a v2 snapshot cannot exist. Review `5202984522` on exact predecessor `a69379663b5f69a8c407fd230fb9f6071de9a4e4` identified this successor-composability contradiction. Weakening v2 would redefine an already domain-separated proof, so v1/v2 remain unchanged.

Behavioral compile RED `817f83ce9f3be644134bec5739d7a713cb9fa919` introduced the expected complete relation-`Var` evidence contract. Production `b506f5154865a732244cc0da8df927c64619515e` added the new domain-separated `IndexExpressionRelationVarSnapshot`; export wiring is `03c60171976693830ddc8842035e5ef3256d55e8`; focused successor/source-binding tests are `b6e7adb6dd89a27a58a885ae15498362e9d2a93c`. None of those commits is execution evidence: this environment does not provide the repository-pinned Rust toolchain, and protected ConceptWeave `main` still lacks the repository-owned Product PR workflow.

## PostgreSQL source authority

PostgreSQL 18 `CompareIndexInfo()` maps child index expressions and predicates through `map_variable_attnos(..., varno = 1, sublevels_up = 0, ...)`, rejects an unpreservable whole-row reference, and then applies internal `equal()` to the mapped tree. The comparison therefore does not reduce a `Var` to a rendered column name.

`map_variable_attnos_mutator()` copies the full `Var` before replacing the mapped `varattno`; when the same RTE is retained it may also update `varattnosyn`, which PostgreSQL marks equality-ignored. PostgreSQL's `Var` definition retains material `varno`, `varattno`, `vartype`, `vartypmod`, `varcollid`, `varnullingrels`, `varlevelsup`, and `varreturningtype`; `varnosyn` and `varattnosyn` are equality-ignored. Parse locations are also intentionally excluded from node equality.

At the ordinary stored-index boundary, PostgreSQL's standard `makeVar()` initialization uses `VAR_RETURNING_DEFAULT` and an empty `varnullingrels`. The new ConceptWeave observation does not silently normalize contrary source input: non-index relation role, non-empty nulling relations, nonzero `varlevelsup`, or non-default `varreturningtype` fail closed.

Primary sources:

- PostgreSQL Global Development Group. (2026). `src/backend/catalog/index.c`, `CompareIndexInfo`, REL_18_STABLE.
- PostgreSQL Global Development Group. (2026). `src/backend/rewrite/rewriteManip.c`, `map_variable_attnos_mutator`, REL_18_STABLE.
- PostgreSQL Global Development Group. (2026). `src/include/nodes/primnodes.h`, `Var`, REL_18_STABLE.
- PostgreSQL Global Development Group. (2026). `src/backend/nodes/equalfuncs.c`, REL_18_STABLE.
- PostgreSQL Global Development Group. (2026). `src/backend/nodes/makefuncs.c`, `makeVar`, REL_18_STABLE.

## Chosen contract

The relation-`Var` family is a new branch over the exact `IndexExpressionSemanticsSnapshot`, not a mutation of v1/v2 and not an unreachable child of v2. The constructor rebound-validates the exact expression-semantics predecessor and exact structured `RelationPartitionTypeModifierSnapshot` before issuing a successor digest.

Each canonical `Column` leaf receives exactly one observation at a deterministic one-based leaf coordinate. Governed identity preserves:

- target relation role instead of statement-local raw `varno`;
- attribute-map-normalized exact column name instead of physical child `attnum`;
- qualified PostgreSQL value type instead of database-local `vartype` OID;
- exact signed raw `vartypmod`;
- qualified collation identity or explicit no-collation state instead of database-local `varcollid` OID;
- explicit proof that `varnullingrels` is empty at this source boundary;
- exact `varlevelsup`, admitted only at zero;
- exact `varreturningtype`, admitted only at `DEFAULT`.

The snapshot validates value type against the owning bounded column, raw typmod against the structured type-modifier family, and collation against the complete source-authoritative column-collation family. For direct attached-index edges, child and parent relation-`Var` evidence at the same canonical leaf coordinate must remain semantically equal after the predecessor's stable column-name normalization.

The digest is domain-separated as `...expression_semantics.relation_var.v1` and frames both the exact expression-semantics digest and exact structured type-modifier digest plus deterministic relation-`Var` observations. Exact source receipts bind the new successor digest.

## Rejected alternatives

Changing v2 to admit a richer leaf was rejected because it would change what an already-issued v2 digest/revision proves. Requiring v2 as a concrete predecessor was rejected because v2 cannot be constructed for relation-Var-bearing trees. Persisting raw `varno`, child `attnum`, type/collation OIDs, `pg_get_expr`, `pg_node_tree`/`nodeToString`, reconstructed DDL, or rendered type text was rejected because those are statement/database-local coordinates or presentation forms rather than stable PostgreSQL equality semantics.

## Remaining acceptance and adapter work

The source contract is not yet native/hosted GREEN. One unchanged exact #46 head must run the repository-pinned Rust 1.98 formatting, strict Clippy, focused relation-Var contracts, retained Source Observation/workspace/doc tests, release build, coverage, and hosted Product/security/dependency/review gates.

The concrete PostgreSQL adapter must still extract the actual internal/catalog facts used by this contract and a live differential oracle must compare ConceptWeave admission against PostgreSQL's real attached-index outcome. The existing raw `pg_attribute.atttypmod` adapter/oracle gap also remains open. Unsupported node kinds and typed constant/Datum semantics must continue to fail closed rather than fall back to rendered text.
