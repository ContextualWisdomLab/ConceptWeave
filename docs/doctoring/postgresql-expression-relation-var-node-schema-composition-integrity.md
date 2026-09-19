# PostgreSQL 18 relation-Var / node-schema composition integrity

## Problem

`IndexExpressionRelationVarSnapshot` fixes the equality state that historical canonical `Column` leaves omitted: stable indexed-relation role, attribute-map-normalized column identity, qualified value type, raw signed `vartypmod`, collation, empty `varnullingrels`, zero `varlevelsup`, and `VAR_RETURNING_DEFAULT`.

That proof deliberately branches from `IndexExpressionSemanticsSnapshot`, because node-schema v2 rejects every relation-`Var` leaf and therefore cannot be a predecessor for the Var-bearing trees being repaired. The branch was necessary, but it left a separate proof-composition gap: relation-Var v1 traversed arbitrary enclosing `CanonicalExpression::Node` values only to enumerate column leaves. It did not prove that the enclosing PostgreSQL node carried every field that PostgreSQL 18 `equal()` compares.

The existing focused relation-Var fixture demonstrated the gap. A `FuncExpr` with only `function` and `arguments`, an `OpExpr` with only `operator` and `arguments`, and a rendered-value `Const` could still receive relation-Var-v1 evidence. Historical node-schema v1 would reject those same trees because its supported `FuncExpr` / `OpExpr` schemas require their complete modeled field sets and it fails closed on `Const` and unknown nodes.

PostgreSQL index attachment does not compare the Vars independently from the surrounding tree. `CompareIndexInfo()` maps child expression/predicate Vars through the partition attribute map and then applies internal `equal()` to the complete mapped expression or predicate. Complete Var evidence therefore cannot, by itself, establish complete attached-index expression equality.

## Evidence and decision

Review `5203122666` records the P1 finding on exact predecessor `fc03c49b42db72dc64c6fa9dcb27ff049ef35919`.

Behavioral compile RED `62635024cf93204d4be447fe9a4859b469a35312` adds a focused contract. It first preserves the historical fact that relation-Var v1 can exist over an incomplete enclosing node while node-schema v1 rejects that tree. Its positive path then requires a new composed successor when the exact expression predecessor satisfies historical node-schema v1 and the exact same source stack satisfies relation-Var v1.

Production `33beb01932d1fdd47d37b8f60c54680d343af84c` adds `IndexExpressionRelationVarNodeSchemaSnapshot` under the domain-separated family
`conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.operator_family.exclusion.expression_semantics.relation_var.node_schema.v1`.
Export wiring is `197a5f7f89caee7d99f4427ac7b028f6e7aba6ab`.

The composed successor does not change node-schema v1, node-schema v2, relation-Var v1, or their existing digests. Instead it:

- rebound-validates `IndexExpressionNodeSchemaSnapshot` from the exact `IndexExpressionSemanticsSnapshot`;
- rebound-validates `IndexExpressionRelationVarSnapshot` from the same complete relation/index/operator-family/exclusion/expression/type-modifier predecessor stack;
- requires both proofs to share exact source, connection-policy binding, extractor revision, and observation time;
- derives a new digest from both exact predecessor digests under a new semantic domain.

This composition is intentional. Node-schema v1 supplies the complete supported `FuncExpr` / `OpExpr` non-Var field proof. Relation-Var v1 supplies the complete equality state for every canonical relation-column leaf and already rejects whole-row leaves. Together they close the proof seam without weakening v2 or pretending that a successful v2 snapshot can exist for a relation-Var-bearing tree.

`Const` and unsupported PostgreSQL node kinds remain fail closed. They must receive their own typed PostgreSQL equality representations before the supported expression surface expands. `pg_get_expr`, `pg_node_tree` / `nodeToString`, rendered SQL, or display type text remain inadmissible substitutes for internal equality semantics.

## Remaining acceptance boundary

This is source repair, not executed acceptance. The current automation execution host has no Rust toolchain, and protected ConceptWeave `main` still lacks the repository-owned Product pull-request workflow. The exact moved #46 head therefore still requires repository-pinned Rust 1.98 formatting, strict Clippy, focused and retained contracts, workspace/doc tests, release build, coverage evidence, and hosted Product/security/review evidence.

The concrete PostgreSQL 18 semantic-expression extractor plus live attached-index differential oracle remains open. The extractor must emit only node kinds whose full equality schema is modeled, populate the relation-Var observations from the same bounded catalog capture, and fail closed rather than downgrade unsupported nodes to text. The raw `pg_attribute.atttypmod` adapter and live rowtype differential oracle also remain open.
