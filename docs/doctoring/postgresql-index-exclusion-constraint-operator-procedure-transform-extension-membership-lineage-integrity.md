# PostgreSQL transform extension-membership lineage integrity

## Decision

`IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot` must fail closed when its converter-function extension-membership predecessor and the separately supplied raw transform-converter snapshot describe different converter directions or different converter functions, even when both carry the same source connection, policy binding, extractor revision, observation timestamp, constraint/key coordinate and transform type.

The transform-object membership fact remains one fact per `(constraint, key_position, transform_type, target_language)` `pg_transform` row. The repair does **not** duplicate extension membership per FROM SQL / TO SQL direction; it only proves that the two constructor inputs refer to the same raw transform binding before the object-level membership successor is issued.

## Problem and realistic RED

Exact head `fcaa330965915a85ea7116a66c04286b79ef5931` verified source-generation metadata across the two inputs but then accepted the converter predecessor when any converter-membership observation shared only `(constraint, key_position, transform_type)`. Direction and converter function identity were omitted from that cross-input binding check.

That admits a mixed lineage. A raw same-generation transform snapshot containing only FROM SQL can be paired with a converter-function extension-membership predecessor that attests both FROM SQL and TO SQL. The transform-object observation set is still complete relative to the raw `pg_transform` row, and the old `any(...)` predicate still finds a converter predecessor, so construction succeeds while the alleged predecessor chain and the raw row disagree.

Finding review `5253304917` records the defect. Structural RED commit `3f3a947e31c3b80e1a66120b89705ec3e515e43c` adds a hostile contract that requires this mixed lineage to fail with `index_exclusion_constraint_operator_procedure_transform_extension_membership_binding`.

## Causal repair

Production commit `37742d774d7c33a693dc3a0359a6583134bc5cbf` cross-checks the complete nonzero raw converter direction count against converter-function membership observations and then requires every raw FROM SQL / TO SQL converter to match the predecessor's exact constraint coordinate, key position, transform type, direction, converter schema and converter function name. Commit `276821d5d5fdafbd4776c9856acfc04221e470c1` adds a second hostile contract for same-direction-count function-name drift.

The repair deliberately does not infer extension membership from converter functions, add a new PostgreSQL catalog field, copy foreign extension truth, or alter the object-level digest granularity. It closes only the cross-input lineage invariant required by the existing API.

## PostgreSQL authority

PostgreSQL 18 defines a transform by type and procedural language and allows independent FROM SQL and TO SQL converter functions; either direction may be omitted. Therefore converter direction and exact function identity are material parts of the raw `pg_transform` binding, not interchangeable implementation details.

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html

## Acceptance

Source repair is not execution evidence. The final exact PR head still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage and the bounded PostgreSQL 18 same-generation differential. Hosted or predecessor results do not transfer after source movement.
