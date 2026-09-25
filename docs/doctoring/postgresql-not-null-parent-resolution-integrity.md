# PostgreSQL 18 NOT NULL parent-resolution integrity

Status: source repaired / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-14

## Problem

`pg_constraint.conparentid` is not descriptive text. PostgreSQL 18 defines it as a reference to the corresponding constraint row on the parent partitioned table. ConceptWeave had already replaced the database-local OID with a stable `ParentNotNullConstraintCoordinate` and validated the partition-specific `conislocal=false`, `coninhcount=1`, and parent relation-kind invariants. Aggregate canonicalization then learned to prove that the supplied stable coordinate resolves to an observed parent `contype='n'` row in the same immutable NOT NULL family.

That row-existence repair was necessary but not sufficient. The lookup matched only parent schema, relation kind, relation name, and constraint name. In a parent with more than one NOT NULL column, a child `raw_value` constraint could therefore claim the observed parent `quality_flag` constraint as its `conparentid`. Both rows existed, relation/column completeness still held, and the contradictory edge could enter governed identity.

The partition-parent attachment boundary also remained able to combine a nonzero parent coordinate with `connoinherit=true` on a regular-table partition child. The constructor correctly rejected `NO INHERIT` on `RelationKind::PartitionedTable`, but ordinary `RelationKind::Table` constraints may legitimately use `NO INHERIT` for classic inheritance. Once such a row is attached to a partitioned-table parent through `conparentid`, however, it represents the inherited partition copy, and PostgreSQL's declarative partitioning rules require the parent's NOT NULL constraint to remain inherited by the partition. That tuple is therefore not source-representable partition evidence.

A further graph-level defect remained after the row and column repairs. Each parent coordinate could resolve to another observed partitioned-table NOT NULL row while the family as a whole formed a cycle such as `metric_a -> metric_b -> metric_a`. Before the acyclicity repair, both edges passed exact-coordinate lookup, same-column validation, partition locality/count checks, and completeness. PostgreSQL declarative partitions form a parent/partition hierarchy: a partition has only its partitioned-table parent, and `conparentid` identifies the corresponding constraint of that actual parent. A cyclic `conparentid` graph is therefore not source-representable and must not acquire governed identity.

These are source-integrity defects rather than naming preferences. PostgreSQL does not use `conparentid` as an arbitrary constraint-to-constraint association, and a partition-child copy cannot simultaneously claim a partition parent edge while declaring that the constraint does not inherit or while participating in a cyclic parent graph.

## Primary-source basis

PostgreSQL 18 documents `pg_constraint.conparentid` as referencing `pg_constraint.oid` and identifying the corresponding constraint of the parent partitioned table when the row belongs to a partition. The same catalog exposes `conkey` as the constrained relation-column numbers and `connoinherit` as the non-inheritable flag. PostgreSQL 18 partitioning rules state that CHECK and NOT NULL constraints of a partitioned table are always inherited by all its partitions and that NO INHERIT constraints of those types are not allowed on the partitioned parent. `ALTER TABLE ... ATTACH PARTITION` likewise requires the attached table to carry all parent NOT NULL and CHECK constraints without `NO INHERIT`.

PostgreSQL's declarative partitioning documentation also constrains topology: a partition cannot have any parent other than the partitioned table it belongs to, and declarative partition trees do not share their inheritance hierarchy with regular-table inheritance. Subpartitioning extends that hierarchy downward; it does not turn the parent relation into a general graph. Consequently a sequence of nonzero `conparentid` links must terminate at a constraint with no partition parent and cannot revisit an already traversed constraint.

`REL_18_STABLE` makes the column identity explicit in the catalog implementation. `findNotNullConstraintAttnum(relid, attnum)` scans `pg_constraint`, retains only `CONSTRAINT_NOTNULL`, extracts the sole NOT NULL `conkey`, and returns a row only when that key equals the requested attribute number. `ConstraintSetParentConstraint()` then records the partition dependency on the exact `parentConstrId`. Taken together, a governed parent edge must resolve both the parent row identity and the corresponding constrained column, its child-side inheritance flags must remain compatible with an inherited partition copy, and the complete resolved edge set must remain a valid acyclic partition hierarchy.

## Decision

A nonzero source `conparentid` may enter governed identity only after all of the following hold in one immutable observation family:

- the parent coordinate names a `RelationKind::PartitionedTable`;
- the child row has `conislocal=false` and `coninhcount=1`;
- the child row has `connoinherit=false`;
- a NOT NULL observation with the exact parent schema, relation kind, relation name, and constraint name exists in the same canonicalized family;
- the resolved parent observation constrains the same source column name as the partition-child observation;
- following parent coordinates from every observation never revisits an observation; any cycle fails as `not_null_constraint_parent_cycle` before hashing;
- normal relation/column completeness validation independently proves that both constraints belong to observed relation columns.

The same-column, no-inherit, and acyclicity rules are partition-parent specific. They do not reinterpret classic inheritance rows with `conparentid=0`; ordinary local table inheritance may still carry `NO INHERIT` or multiple inheritance where PostgreSQL permits it. The model continues to exclude raw OIDs from governed identity.

## Repair lineage

Review `5193008264` identified the first unresolved-coordinate authority gap on exact predecessor `445acfff1516a69c9180a06f930ead952f0d9f57`.

RED `b20a54c3da3d2890b7e786fce7535fdd13bf6615` changed `not_null_constraint_parent_contract.rs` so valid partition-parent fixtures include the parent relation and parent NOT NULL row, and added `unresolved_partition_parent_constraint_is_rejected`. Production repair `7d62df1792ebcf4a015741496535dafebbc22148` made `canonicalize_not_null_constraints()` reject a parent coordinate that does not resolve to an exact observation in the same family.

Fresh review `5193201137` then identified the remaining cross-column edge on exact `15a1729949eda1ace3b668768a75b9c7c4c11f02`. RED `212d17e96963cf584576056a3907e8a55adef2f2` adds a two-column partition family in `not_null_constraint_parent_column_contract.rs`: the child `raw_value` row deliberately references the parent's `quality_flag` NOT NULL row while every relation and NOT NULL row is otherwise complete. That predecessor accepts the impossible edge, so the contract expects `not_null_constraint_parent_column`.

Production repair `cfe1851f0807c1d53c2a51c58c913509a40cfd36` resolves the parent row once by its stable coordinate and then requires `parent_observation.column_name() == observation.column_name()`. Missing parents continue to use `not_null_constraint_parent_coordinate`; cross-column parents use the distinct `not_null_constraint_parent_column` failure.

Review `5193490988` found the remaining inheritance-flag contradiction on exact `862d966b5ec8f24908f546d9d413d6c535b497a0`. RED `1bd8e20e9a13574a2c0d4f700d93bd54923451f9` adds `partition_parent_link_rejects_no_inherit_child_constraint`: an ordinary table constraint is first constructed with `NO INHERIT`, which remains valid before any partition-parent edge is attached, then the test requires parent attachment to fail as `not_null_constraint_parent_no_inherit`. Minimal production repair `e1618a20db333cae4facdf1bf80c209785b6b782` places that fail-closed check only in `with_parent_constraint()`. Generic inheritance with `conparentid=0` is therefore unchanged.

Review `5193997785` found the remaining graph-integrity gap on exact `08f9c65d5a7b7276575e1bb17f3c48c5eaae43c6`. Executable regression contract `a8ef446249ef9249d3f893a94b9d3ee7d1b36822` adds a two-node partitioned-table family in which each NOT NULL row resolves to the other as parent. Every local tuple and corresponding-column check is otherwise valid, so the contract isolates graph topology and expects `not_null_constraint_parent_cycle`. Minimal production repair `2a0643952e5f3150c8b27d3a7c11c30525b6871f` builds an exact constraint-coordinate index and walks every parent chain with per-chain visited-state detection. A repeated node fails closed before the family can be hashed.

Existing partition-kind, locality/count, signed-`int2`, parent-coordinate, same-column, nullability, duplicate-column, completeness, naming, and digest-domain behavior remains unchanged. The latest regression is executable source evidence but has not been run in this execution environment; the available host has no repository Rust toolchain, and protected ConceptWeave `main` still lacks the Product PR workflow. Rust 1.98 fmt, strict Clippy, contract/workspace/doc tests, release build, owned coverage, hosted security/Product/review evidence, and the unchanged-head requirement remain separate gates.

## Adapter obligation

The future PostgreSQL transport must resolve `conparentid` while holding the same bounded read-only catalog snapshot used for `pg_constraint`/`pg_attribute` capture. It must join the referenced parent `pg_constraint` row, extract its NOT NULL `conkey`, resolve that attribute against the parent relation, prove that it is the corresponding partition column, and reject a partition-parent-linked child row whose inheritance flags contradict the inherited partition semantics before emitting the stable coordinate. After all coordinates are resolved, the captured parent graph must be proven acyclic before immutable snapshot construction. A missing, out-of-scope without authorized expansion, contradictory, cross-column, cyclic, `NO INHERIT`, or partially resolved parent row fails closed.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
