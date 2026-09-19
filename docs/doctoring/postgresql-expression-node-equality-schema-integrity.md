# PostgreSQL 18 expression-node equality-schema integrity

## Decision

ConceptWeave must not treat a generic node tag plus an arbitrary subset of named fields as proof of PostgreSQL 18 internal `equal()` semantics. A canonical node is admissible only when its node kind has an explicit supported equality schema and every equality-participating field in that schema is present with the expected semantic value type. Unsupported node kinds fail closed.

Review `5201557492` on exact #46 head `64e4ef2e77d3b56d68573e2cd5aa19fa8fc6338a` records the P1. The existing generic constructor sorted field names and rejected duplicates, but it could still admit a `FuncExpr` containing only function identity and arguments, an `OpExpr` containing only operator identity and arguments, or a `Const` approximated by type plus rendered value text. Those representations can collapse PostgreSQL trees that `equal()` distinguishes.

Source RED `cbf819f55b1ef54360c1ae47b2a23317d4a2dd23` first demonstrated the missing constructor-level guard. Contract correction `fdc78e63fed2e933cd25ae8105541f7deadcecf3` moved the required check to an explicit versioned validation successor rather than silently changing the already-issued expression-semantics predecessor contract.

Production `0c7e12d05c0da2bf279cc94a13a180043ae6dbd5` adds `validate_postgres18_equal_schema()` and `IndexExpressionNodeSchemaSnapshot`; export `d7767cf93cc02cea684c039ba3c1385ab9143999` makes the successor part of the canonical relation-partition API. The successor frames the exact expression-semantics predecessor digest under a separate domain only after all observed expressions and predicates satisfy the supported PostgreSQL 18 equality schema.

## PostgreSQL 18 equality fields modeled now

PostgreSQL 18 `FuncExpr` stores function OID, result type, set-returning state, variadic state, `CoercionForm`, result collation, input collation, arguments, and parse location. `equal()` intentionally ignores parse locations and `CoercionForm`; the remaining material fields must be represented. ConceptWeave therefore admits `FuncExpr` only with:

- stable resolved function signature, including result type;
- `returns_set`;
- `variadic`;
- result collation or explicit no-collation state;
- input collation or explicit no-collation state;
- ordered arguments.

PostgreSQL 18 `OpExpr` stores operator identity, underlying procedure cache, result type, set-returning state, result/input collation, arguments, and location. ConceptWeave admits it only with stable operator signature, result type, set-returning state, both collation states, and ordered arguments. The cached procedure identity is not duplicated in this successor because direct attachment comparison occurs within one source snapshot and stable operator identity resolves the same catalog operator; PostgreSQL's special `equal_ignore_if_zero` behavior for `opfuncid` is not approximated by a new mutable field.

`Const` is intentionally not admitted yet. PostgreSQL's custom `_equalConst()` compares type, type modifier, collation, type length, null state, by-value state, and—when non-null—the exact Datum via `datumIsEqual()`. Server-rendered SQL or type output is not a sound replacement for that Datum equality. A future typed constant-value contract must preserve the required Datum semantics before `Const` becomes supported.

Unknown node kinds also fail closed. This prevents an adapter from silently emitting an incomplete field subset for a newly encountered PostgreSQL node.

## Alternatives rejected

- Keeping generic `Node { kind, fields }` as self-attesting completeness was rejected because the type cannot prove that an extractor emitted every material field.
- Adding rendered SQL, `pg_get_expr`, raw `pg_node_tree`, or `nodeToString()` as a catch-all was rejected because those are not PostgreSQL's post-attribute-map equality contract.
- Encoding parse location was rejected because PostgreSQL `equal()` intentionally ignores it.
- Encoding `CoercionForm` for `FuncExpr` was rejected because PostgreSQL's equality macro intentionally ignores it.
- Treating `Const` output text as the immutable Datum identity was rejected because `_equalConst()` uses `datumIsEqual()` with the constant's by-value and type-length semantics.
- Rewriting `IndexExpressionSemanticsSnapshot` in place was rejected. The equality-schema check is a new domain-separated validation successor so predecessor digest meaning remains stable.

## Remaining verification boundary

This repair closes the node-schema completeness hole for the currently supported `FuncExpr` and `OpExpr` representation. It does not implement the concrete PostgreSQL adapter extractor and does not claim complete PostgreSQL expression coverage. Production extraction still needs to derive every admitted field from PostgreSQL 18 node/catalog structures, resolve OID-bearing identities to stable coordinates, add typed support for further node kinds only with complete equality schemas, and pass a differential oracle against actual index-partition attachment behavior.

The structured `pg_attribute.atttypmod` successor for relation-partition rowtype identity also remains open. No executed Rust 1.98 RED/GREEN or hosted Product acceptance is claimed for this moved head.

## Traceability

- owner PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5201557492` on `64e4ef2e77d3b56d68573e2cd5aa19fa8fc6338a`
- source RED: `cbf819f55b1ef54360c1ae47b2a23317d4a2dd23`
- contract-boundary correction: `fdc78e63fed2e933cd25ae8105541f7deadcecf3`
- production schema successor: `0c7e12d05c0da2bf279cc94a13a180043ae6dbd5`
- production export: `d7767cf93cc02cea684c039ba3c1385ab9143999`
- production: `crates/conceptweave-relation-partition/src/expression_schema.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_expression_node_schema_contract.rs`

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: primitive expression node definitions in `src/include/nodes/primnodes.h` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/nodes/primnodes.h

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: node equality implementation in `src/backend/nodes/equalfuncs.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/nodes/equalfuncs.c
