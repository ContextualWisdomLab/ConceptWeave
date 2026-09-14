# PostgreSQL expression collation catalog identity composition integrity

Status: source-repaired / execution acceptance pending

## Problem

The per-key catalog-identity successor closes `pg_index.indcollation` aliasing, but the modeled whole-expression proof still contained two historical qualified-name-only collation surfaces:

- `CanonicalExpressionValue::Collation`, used for `equal()`-participating expression-node fields such as `FuncExpr` result/input collation;
- `IndexExpressionRelationVarObservation.collation`, representing `Var.varcollid`.

Both surfaces retain `QualifiedCollationName(namespace, name)`. PostgreSQL 18 can contain distinct `pg_collation` rows with the same namespace/name and different `collencoding`, while node equality compares the underlying collation OIDs. Thus two attached trees could remain equal under the issued expression/relation-Var proofs although corresponding OID identities differed.

Rewriting the issued canonical-expression or relation-Var representation would invalidate immutable predecessor meaning. The repair must therefore compose new catalog-row evidence over those exact proofs.

## Decision

Review `5203347945` on exact predecessor `2638cd207e8e97888f8ec7514df755383ab247ab` records the follow-on P1.

Behavioral compile RED `68576447e8f673098d9a94a1fddacc876db1abe0` builds a real direct parent/child index-partition stack in which:

- parent and child canonical expression trees carry identical historical `pg_catalog.default` qualified collation names;
- relation-`Var` leaves likewise carry identical historical qualified names;
- the new per-key catalog-identity predecessor is valid and identical;
- supplied expression or relation-`Var` catalog identities can nevertheless differ only by raw `collencoding`.

The new successor must reject each mismatch independently and admit the all-matching control.

Production `d67f454d796eb9a5d6b01a4dfac8370c90051c45` adds `IndexExpressionCollationIdentitySnapshot` and its typed observations/locations. Export wiring is `ab5e50f772afb9c9db0ee07298db428f613f1d35`.

The successor:

1. rebound-validates `IndexExpressionRelationVarNodeSchemaSnapshot`, retaining the exact node-schema + relation-`Var` whole-tree proof;
2. rebound-validates `IndexPartitionCollationIdentitySnapshot`, retaining exact per-key catalog identity;
3. deterministically enumerates every `CanonicalExpressionValue::Collation` occurrence in canonical expression/predicate field order and requires one catalog-row identity for each;
4. requires one explicit catalog identity (or explicit no-collation state) for every relation-`Var` leaf;
5. binds every supplied identity back to the historical qualified name so the successor cannot relabel prior evidence;
6. compares full `(namespace, name, raw collencoding)` identity for corresponding expression/predicate occurrences and relation-`Var` leaves across every direct attached parent/child index edge;
7. frames the exact whole-tree and key-collation predecessor digests plus the new complete observations under a new digest domain.

This closes the known catalog-identity seam for the currently modeled attached-index equality surface without mutating prior digests.

## Scope and remaining limitations

The successor covers collation identity used by the currently modeled expression/predicate `Collation` values, relation-`Var.varcollid`, and per-key `indcollation` predecessor. It does not generalize the separate Source Observation column-collation family; that historical contract remains a qualified-name family and requires its own versioned successor if future governed semantics depend on distinguishing same-name/different-encoding catalog rows outside this attachment proof.

Concrete PostgreSQL transport is still not part of this representation repair. After the representation head obtains native and hosted acceptance, the adapter must resolve every nonzero collation OID from the same bounded catalog snapshot to `(namespace, name, collencoding)` and the live differential oracle must verify the server attachment outcome without reducing OID identity to rendered names.

## Traceability

- Review: `5203347945`.
- Behavioral compile RED: `68576447e8f673098d9a94a1fddacc876db1abe0`.
- Production: `crates/conceptweave-relation-partition/src/expression_collation_identity.rs` at `d67f454d796eb9a5d6b01a4dfac8370c90051c45`.
- Export: `crates/conceptweave-relation-partition/src/index_partition.rs` at `ab5e50f772afb9c9db0ee07298db428f613f1d35`.
- Contract: `crates/conceptweave-relation-partition/tests/index_expression_collation_catalog_identity_contract.rs`.
- Per-key predecessor: `crates/conceptweave-relation-partition/src/collation_identity.rs`.
- Whole-tree predecessor: `crates/conceptweave-relation-partition/src/expression_relation_var_schema.rs`.
- PostgreSQL 18 catalog source: `src/include/catalog/pg_collation.h`.
- PostgreSQL 18 attachment/equality source: `src/backend/catalog/index.c` and generated node equality functions on `REL_18_STABLE`.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: pg_collation catalog definition* (`REL_18_STABLE`, `src/include/catalog/pg_collation.h`). PostgreSQL Global Development Group.

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: index attachment comparison* (`REL_18_STABLE`, `src/backend/catalog/index.c`). PostgreSQL Global Development Group.
