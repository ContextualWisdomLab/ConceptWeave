# PostgreSQL 18 index-partition expression/predicate integrity

## Decision

A direct PostgreSQL index-partition edge is not definition-equivalent merely because its structural expression slots, access method, mapped simple attributes, collations, operator families, and exclusion semantics agree. PostgreSQL 18 `CompareIndexInfo()` separately compares `ii_Expressions` and `ii_Predicate`. For each family it maps the child tree through the partition attribute map, rejects an unpreservable child whole-row reference, and then requires internal node-tree equality.

Review `5201012106` on exact #46 head `a97c25eb087c1aeda0bc5e534e40c1ce51dfc994` records the P1. Source contract `660f1adda178ac3ba0436cda14003f414e0a0e2b` requires semantic expression mismatch, semantic partial-predicate mismatch, and a direct child whole-row reference to fail while preserving equality when parent and child physical attribute ordinals differ but stable column identity is the same. It also requires semantic evidence for every frozen-v3 zero-`indkey` expression and every observed partial predicate.

Production `697597edb4f40e1dffedd7fcfd2f54cfaea91c87` adds the domain-separated `IndexExpressionSemanticsSnapshot`; `09028cc1945e078aee0f661b15bb01a11ec80365` exports it through the relation-partition crate. The successor rebound-validates the exact base → relation-partition → index-partition → operator-family → exclusion chain, requires complete expression/predicate semantics for the frozen-v3 raw evidence, validates stable column references against their exact owning relation, compares direct parent/child canonical trees, fails closed on child whole-row references, issues exact receipts, and frames the exact exclusion predecessor digest.

## Stable semantic representation

The frozen v3 representation continues to preserve server-rendered index expressions and predicates as source display evidence. This repair does not rewrite that predecessor identity. The successor adds a separate semantic tree whose governed identity cannot contain a relation-local attribute number or a raw catalog OID.

- relation-local user-column `Var` identity is represented by the exact column name, matching the stable result of PostgreSQL's child→parent attribute mapping rather than either table's physical attribute number;
- PostgreSQL types and collations use exact schema-qualified coordinates;
- overloaded binary operators use schema/name plus qualified operand types;
- unary operators use schema/name plus their operand type;
- functions use schema/name plus ordered input argument types and result type;
- ordered child expression lists remain ordered because PostgreSQL node equality is order-sensitive;
- generic node fields are sorted by semantic field name so extractor emission order is not identity;
- whole-row references remain explicit and are rejected on the direct child side of an attached index.

The canonical node contract is intentionally versioned as `postgresql-18-equal-v1`. Its extractor must emit the complete set of fields that PostgreSQL 18 internal `equal()` considers for the corresponding node kind. Parse locations and display-only rendering are not semantic fields.

## Alternatives rejected

- Raw `pg_get_expr` comparison was rejected because it compares a rendering rather than the node semantics PostgreSQL compares after variable remapping.
- Raw `pg_node_tree` / `nodeToString()` comparison was rejected because child relation attribute numbers and database-local OIDs are capture-specific even where PostgreSQL considers the mapped trees equal.
- Reconstructed `pg_get_indexdef`/DDL comparison was rejected because presentation text is not `CompareIndexInfo()`'s equality contract.
- Persisting catalog OIDs or child `attnum` values in immutable release identity was rejected because those are source-local join coordinates.
- Mutating frozen v3 expression/predicate fields was rejected because it would change the meaning of already-issued predecessor digests and receipts.

## Remaining verification boundary

This commit repairs the representation and relation-partition comparison seam; it does not claim that a live PostgreSQL adapter already produces the canonical tree. The adapter must derive this representation from the PostgreSQL 18 catalog/node structure, resolve every OID-bearing semantic field to stable coordinates, and prove that no `equal()`-participating field is omitted. A differential oracle against real PostgreSQL attachment behavior is therefore still required before this successor can be treated as end-to-end source truth.

In particular, future extractor work must cover every admissible expression node carrying type, collation, operator, function, operator-family, or other catalog identity rather than falling back to raw SQL or raw node serialization. Any expression form that cannot be represented completely must fail closed instead of being normalized heuristically.

No executed Rust RED/GREEN or hosted Product acceptance is claimed for this source repair. The available execution host does not provide the repository-pinned Rust 1.98 toolchain, and protected ConceptWeave `main` has not yet supplied the repository-owned Product PR acceptance path at this checkpoint.

## Traceability

- owner PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5201012106` on `a97c25eb087c1aeda0bc5e534e40c1ce51dfc994`
- source RED: `660f1adda178ac3ba0436cda14003f414e0a0e2b`
- production semantics: `697597edb4f40e1dffedd7fcfd2f54cfaea91c87`
- production export: `09028cc1945e078aee0f661b15bb01a11ec80365`
- production: `crates/conceptweave-relation-partition/src/expression_semantics.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_partition_definition_expression_predicate_contract.rs`

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: `CompareIndexInfo()` in `src/backend/catalog/index.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: `map_variable_attnos()` in `src/backend/rewrite/rewriteManip.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/rewrite/rewriteManip.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_index`.* https://www.postgresql.org/docs/18/catalog-pg-index.html
