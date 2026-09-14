# PostgreSQL 18 NOT NULL parent-resolution integrity

Status: source repaired / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-14

## Problem

`pg_constraint.conparentid` is not descriptive text. PostgreSQL 18 defines it as a reference to the corresponding constraint row on the parent partitioned table. ConceptWeave had already replaced the database-local OID with a stable `ParentNotNullConstraintCoordinate` and validated the partition-specific `conislocal=false`, `coninhcount=1`, and parent relation-kind invariants. Aggregate canonicalization then learned to prove that the supplied stable coordinate resolves to an observed parent `contype='n'` row in the same immutable NOT NULL family.

That row-existence repair was necessary but not sufficient. The lookup matched only parent schema, relation kind, relation name, and constraint name. In a parent with more than one NOT NULL column, a child `raw_value` constraint could therefore claim the observed parent `quality_flag` constraint as its `conparentid`. Both rows existed, relation/column completeness still held, and the contradictory edge could enter governed identity.

This is a source-integrity defect rather than a naming preference. PostgreSQL does not use `conparentid` as an arbitrary constraint-to-constraint association; the child row represents the inherited corresponding constraint for the partition column.

## Primary-source basis

PostgreSQL 18 documents `pg_constraint.conparentid` as referencing `pg_constraint.oid` and identifying the corresponding constraint of the parent partitioned table when the row belongs to a partition. The same catalog exposes `conkey` as the constrained relation-column numbers. PostgreSQL 18 `CREATE TABLE` states that a partition has the same column names and types as its parent and that parent constraints are cloned on the partition. The partitioning rules require the parent NOT NULL constraints to be inherited by all partitions.

`REL_18_STABLE` makes the column identity explicit in the catalog implementation. `findNotNullConstraintAttnum(relid, attnum)` scans `pg_constraint`, retains only `CONSTRAINT_NOTNULL`, extracts the sole NOT NULL `conkey`, and returns a row only when that key equals the requested attribute number. `ConstraintSetParentConstraint()` then records the partition dependency on the exact `parentConstrId`. Taken together, a governed parent edge must resolve both the parent row identity and the corresponding constrained column.

## Decision

A nonzero source `conparentid` may enter governed identity only after all of the following hold in one immutable observation family:

- the parent coordinate names a `RelationKind::PartitionedTable`;
- the child row has `conislocal=false` and `coninhcount=1`;
- a NOT NULL observation with the exact parent schema, relation kind, relation name, and constraint name exists in the same canonicalized family;
- the resolved parent observation constrains the same source column name as the partition-child observation;
- normal relation/column completeness validation independently proves that both constraints belong to observed relation columns.

The same-column rule is partition-specific. It does not reinterpret generic inheritance rows with `conparentid=0`, and it does not require child and parent constraint names to be equal. The model continues to exclude raw OIDs from governed identity.

## Repair lineage

Review `5193008264` identified the first unresolved-coordinate authority gap on exact predecessor `445acfff1516a69c9180a06f930ead952f0d9f57`.

RED `b20a54c3da3d2890b7e786fce7535fdd13bf6615` changed `not_null_constraint_parent_contract.rs` so valid partition-parent fixtures include the parent relation and parent NOT NULL row, and added `unresolved_partition_parent_constraint_is_rejected`. Production repair `7d62df1792ebcf4a015741496535dafebbc22148` made `canonicalize_not_null_constraints()` reject a parent coordinate that does not resolve to an exact observation in the same family.

Fresh review `5193201137` then identified the remaining cross-column edge on exact `15a1729949eda1ace3b668768a75b9c7c4c11f02`. RED `212d17e96963cf584576056a3907e8a55adef2f2` adds a two-column partition family in `not_null_constraint_parent_column_contract.rs`: the child `raw_value` row deliberately references the parent's `quality_flag` NOT NULL row while every relation and NOT NULL row is otherwise complete. That predecessor accepts the impossible edge, so the contract expects `not_null_constraint_parent_column`.

Production repair `cfe1851f0807c1d53c2a51c58c913509a40cfd36` resolves the parent row once by its stable coordinate and then requires `parent_observation.column_name() == observation.column_name()`. Missing parents continue to use `not_null_constraint_parent_coordinate`; cross-column parents use the distinct `not_null_constraint_parent_column` failure. Existing partition-kind, locality/count, signed-`int2`, `NO INHERIT`, nullability, duplicate-column, completeness, naming, and digest-domain behavior remains unchanged.

The repair is source-complete but not execution acceptance. Rust 1.98 fmt, strict Clippy, contract/workspace/doc tests, release build, owned coverage, hosted security/Product/review evidence, and the unchanged-head requirement remain separate gates.

## Adapter obligation

The future PostgreSQL transport must resolve `conparentid` while holding the same bounded read-only catalog snapshot used for `pg_constraint`/`pg_attribute` capture. It must join the referenced parent `pg_constraint` row, extract its NOT NULL `conkey`, resolve that attribute against the parent relation, and prove that it is the corresponding partition column before emitting the stable coordinate. A missing, out-of-scope without authorized expansion, contradictory, cross-column, or partially resolved parent row fails closed before immutable snapshot construction.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c
