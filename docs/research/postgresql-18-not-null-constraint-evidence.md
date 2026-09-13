# PostgreSQL 18 NOT NULL constraint evidence

Status: source repaired / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-14

## Problem

The v3 relation representation originally stored only the column-level nullable summary. That is insufficient in PostgreSQL 18 because column `NOT NULL` specifications are first-class `pg_constraint` rows.

PostgreSQL 18 documents `pg_attribute.attnotnull` as stating that the column **has a (possibly invalid) not-null constraint**. The corresponding first-class constraint is represented by `pg_constraint.contype = 'n'`, where PostgreSQL exposes the constraint name, enforcement and validation state, locality/inheritance fields, constrained attribute coordinate, and partition-parent relationship.

A digest that binds only the nullable boolean therefore loses material source evidence such as:

- explicit constraint name (`conname`);
- validated versus `NOT VALID` (`convalidated`);
- enforced versus `NOT ENFORCED` (`conenforced`);
- locally defined versus inherited state (`conislocal`, `coninhcount`);
- inheritable versus `NO INHERIT` (`connoinherit`);
- partition-parent constraint linkage (`conparentid`, resolved to a stable source coordinate rather than retained as an OID).

## Primary-source findings

PostgreSQL 18 release notes state that column `NOT NULL` specifications are stored in `pg_constraint`, enabling explicit names, foreign-table not-null constraints, and inheritance control. The PostgreSQL 18 `pg_constraint` catalog documents `contype = 'n'` for not-null constraints and exposes `conname`, `conenforced`, `convalidated`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, and `conkey`. The PostgreSQL 18 `pg_attribute` catalog defines `attnotnull` as: the column has a possibly invalid not-null constraint.

PRIMARY KEY handling is material to completeness. An April 2024 development discussion described an intermediate model in which `attnotnull` could be backed directly by either a NOT NULL or PRIMARY KEY constraint. That evidence is **not authoritative for PostgreSQL 18 final semantics**. Later upstream work changed the implementation: Álvaro Herrera's September 2024 review states that `CREATE TABLE t(a int primary key)` creates a separate NOT NULL constraint and that PRIMARY KEY processing should "queue not-null constraints for each column." The PostgreSQL table-command implementation prepares a PRIMARY KEY by adding NOT NULL constraints on all key columns. April 2025 PostgreSQL 18 discussion likewise treats a validated NOT NULL constraint on each key column as a PRIMARY KEY prerequisite. The earlier April 2024 model is therefore retained only as historical development context and must not drive the PostgreSQL 18 contract.

`coninhcount` is catalog type `int2` and is defined as the number of direct inheritance ancestors. The representation may expose this nonnegative count as an unsigned value, but values above PostgreSQL signed `int2` maximum 32767 are impossible source evidence and must be rejected before governed hashing.

`conparentid` has a narrower meaning than generic inheritance: PostgreSQL defines it as the corresponding constraint of the **parent partitioned table** when the row is a constraint on a partition. The resolved parent relation therefore has `pg_class.relkind = 'p'` (`PartitionedTable`). A stable parent coordinate carrying any other relation kind is impossible source evidence and must fail closed before governed hashing.

The PostgreSQL 18 source narrows the partition-parent tuple further. `ConstraintSetParentConstraint()` asserts that a child constraint has `coninhcount == 0` before linkage, then sets `conislocal = false`, increments `coninhcount` exactly once, and finally stores `conparentid`. Consequently, an observed row with nonzero `conparentid` must be represented as a non-local partition child with exactly one direct inheritance ancestor. Generic table inheritance is different: `AdjustNotNullInheritance()` can increment `coninhcount` while `conparentid` remains zero. ConceptWeave therefore validates the `conparentid`-specific locality/count invariants only when attaching the resolved partition-parent coordinate and does not impose them on ordinary inheritance rows.

PostgreSQL 18 table-partitioning documentation further narrows `connoinherit`: CHECK and NOT NULL constraints of a partitioned table are always inherited by all partitions, and PostgreSQL does not allow `NO INHERIT` constraints of those types on a partitioned table. `NOT NULL ... NO INHERIT` remains meaningful for non-partitioned inheritance hierarchies, but `relation_kind = PartitionedTable` together with `connoinherit = true` is impossible PostgreSQL 18 source evidence and must be rejected before governed hashing.

`ALTER TABLE` permits not-null constraints to be added `NOT VALID`, later validated, and marked `INHERIT`/`NO INHERIT` where the relation kind permits that state. PostgreSQL also permits `NOT ENFORCED` constraint state. The data-definition documentation allows explicit names and states that a column can have at most one explicit not-null constraint.

## Selected representation boundary

Do not modify the frozen `ColumnObservationV3` digest contract and do not reconstruct constraint metadata from rendered DDL.

Use a domain-separated optional NOT NULL constraint family whose observations retain, at minimum:

- exact schema, relation kind, relation and column coordinate;
- exact constraint name;
- validation and enforcement state;
- `conislocal`, `coninhcount`, and `connoinherit`;
- resolved partition-parent constraint coordinate when `conparentid != 0`.

Catalog OIDs are join coordinates inside one captured catalog transaction and are not governed identity. A nonzero `conparentid` is resolved to an exact parent schema/relation/constraint coordinate only after verifying that the parent relation kind is `PartitionedTable`; a non-partitioned parent coordinate is rejected as contradictory catalog evidence. Attaching that coordinate additionally requires the observed child row to have `conislocal = false` and `coninhcount = 1`, matching PostgreSQL 18's partition-linkage mutation rather than accepting arbitrary independent field combinations.

The public model keeps `coninhcount` as a nonnegative count but validates it against the source catalog domain. Values above 32767 are rejected with `not_null_constraint_inheritance_ancestor_count` rather than hashed as if PostgreSQL could have emitted them.

The model also validates relation-kind-specific inheritance state. `RelationKind::PartitionedTable` plus `no_inherit = true` is rejected with `not_null_constraint_no_inherit`; the state cannot be produced by PostgreSQL 18 and therefore cannot receive a governed semantic identity. Ordinary-table inheritance remains capable of representing `NO INHERIT`.

When the family is declared observed for bounded user relations, its resolved column set must equal the bounded columns whose source `attnotnull` summary is true. PostgreSQL 18 creates/queues first-class NOT NULL constraints for PRIMARY KEY columns, so a PRIMARY KEY does not justify omitting those rows from the captured NOT NULL inventory. An observed empty set is distinct from an unobserved family. Unknown columns, duplicate not-null constraints for one column, multi-column `conkey`, and contradictions between the constraint family and column nullability fail closed.

Input order is not semantic identity. Canonical ordering precedes domain-separated digest extension.

## Rejected alternatives

### Keep `nullable` as the only identity

Rejected because it loses the PostgreSQL 18 constraint name, validation/enforcement state, and inheritance semantics.

### Reconstruct a synthetic not-null constraint from `attnotnull`

Rejected because `attnotnull` cannot reconstruct `pg_constraint` fields. The adapter captures the actual first-class row instead of inventing source evidence.

### Treat PRIMARY KEY as a substitute for a missing PostgreSQL 18 `contype='n'` row

Rejected. That matched an intermediate April 2024 development model but was superseded before PostgreSQL 18. Later upstream implementation explicitly queues NOT NULL constraints for PRIMARY KEY columns; final PostgreSQL 18 catalog semantics therefore require the actual NOT NULL row to be captured.

### Bind catalog OIDs

Rejected because OIDs are database-local join coordinates, not stable semantic identity. Partition-parent linkage must be resolved to source coordinates before entering governed evidence.

### Accept arbitrary relation kinds for resolved `conparentid`

Rejected because `conparentid` is specifically the corresponding constraint of a parent partitioned table. Preserving an arbitrary relation kind would make impossible catalog state look authoritative.

### Treat `conparentid`, `conislocal`, and `coninhcount` as independent fields

Rejected because PostgreSQL 18 creates a partition-parent link by changing those fields as one source operation: linkage sets the child non-local, advances its direct-ancestor count from zero to one, and records the parent constraint. Hashing `conparentid != 0` together with `conislocal = true` or `coninhcount != 1` would manufacture governed identity for a partition-linkage state PostgreSQL 18 cannot produce. This restriction does not apply to generic inheritance rows whose `conparentid` is zero.

### Permit `NO INHERIT` NOT NULL evidence on a partitioned table

Rejected because PostgreSQL 18 requires partitioned-table NOT NULL constraints to be inherited by every partition and forbids `NO INHERIT` for that case. Hashing the combination would manufacture an authoritative identity for source state PostgreSQL cannot emit.

### Permit the full `u16` range for `coninhcount`

Rejected because PostgreSQL 18 stores `coninhcount` as signed `int2`. Accepting 32768 through 65535 would admit source states the catalog cannot represent.

### Reuse generic CHECK evidence

Rejected because PostgreSQL 18 represents explicit not-null constraints as `contype='n'`, not as CHECK constraints, and exposes not-null-specific validation/inheritance behavior.

## Contract and repair lineage

`crates/conceptweave-observation/tests/not_null_constraint_contract.rs` pins first-class `NotNullConstraintObservation` identity, aggregate admission, observed-empty versus unobserved state, complete bounded non-nullable-column coverage including PRIMARY KEY columns, contradiction rejection, one explicit NOT NULL constraint per column, input-order invariance, duplicate-family/order protection, and the signed-`int2` upper bound for `coninhcount`.

`crates/conceptweave-observation/tests/not_null_constraint_parent_contract.rs` additionally pins stable parent-coordinate identity, rejects any resolved `conparentid` coordinate whose parent relation kind is not `PartitionedTable`, and now rejects partition-child parent linkage when the source row remains local or does not have exactly one direct inheritance ancestor. Review `5192015894` identified this fresh source-domain gap. RED `57c7f6c4d418a7905f50bb0526711afe1cb1c602` adds the two impossible-state regressions; production repair `03bb42acca171fd52d8a3d9d4faea810c6ae317a` validates the partition-linkage tuple at `with_parent_constraint()` while leaving generic inheritance semantics untouched.

`crates/conceptweave-observation/tests/not_null_partition_inheritance_contract.rs` is the PostgreSQL 18 partition-specific RED. Commit `8ef85d714329d4d26b4fa2a1db5a4f9e84b66220` requires `PartitionedTable + NO INHERIT` to fail with `not_null_constraint_no_inherit`. Production repair `7331fe067f1b9876f8dc7d79bb9d182c01e46e52` adds the minimum constructor guard while preserving valid ordinary-table `NO INHERIT` evidence.

Review `5190456512` and commits `4236309b...`, `5a29f0aa...`, and `ca8d8e5a...` temporarily removed the reverse completeness rule based on the superseded April 2024 development model. Fresh source review `5190470810` identified the chronology error. Corrected RED `d4df54d9...` requires a PRIMARY-KEY-backed non-nullable column with an observed-empty NOT NULL inventory to fail `not_null_constraint_completeness`; production repair `0cdd4fd3...` restores complete PostgreSQL 18 NOT NULL-row coverage ordinary-forward without rewriting history.

The aggregate family is source-repaired. Exact-head repository-pinned Rust and hosted acceptance remain separate gates; documentation never upgrades source repair into execution acceptance.

## Adapter obligation

A future PostgreSQL transport must collect `pg_attribute`, all bounded `pg_constraint.contype = 'n'` rows, and joined parent `pg_class` metadata from the same bounded `REPEATABLE READ READ ONLY` catalog snapshot. It must resolve each `conkey` to exactly one bounded column, retain exact `conname`, `convalidated`, `conenforced`, `conislocal`, `coninhcount`, and `connoinherit`, and resolve nonzero `conparentid` to a stable parent coordinate whose relation kind is `PartitionedTable`.

For bounded user relations, the resolved `contype='n'` column set must equal the captured `attnotnull=true` column set. PRIMARY KEY columns are not an exception in PostgreSQL 18 because PRIMARY KEY creation queues first-class NOT NULL constraints. `coninhcount` must be parsed as signed `int2`, rejected if negative, and converted to the nonnegative representation only after source-domain validation. A nonzero `conparentid` must additionally be accompanied by `conislocal=false` and `coninhcount=1`; contradictory partition-linkage tuples fail before immutable snapshot construction. For partitioned-table rows, `connoinherit=true` is also contradictory source state. Missing, duplicate, contradictory, out-of-domain, or partially resolved rows fail the complete-or-fail capture.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18.0 release notes*. https://www.postgresql.org/docs/18/release-18.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 5.5 Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 5.12 Table Partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.7 pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.11 pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.13 pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: pg_constraint.c (REL_18_STABLE)*. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/pg_constraint.c

Herrera, Á. (2024, September 25). *Re: not null constraints, again* [PostgreSQL hackers mailing-list message]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202409252014.74iepgsyuyws%40alvherre.pgsql

Herrera, Á. (2025, April 1). *Re: Support NOT VALID / VALIDATE constraint options for named NOT NULL constraints* [PostgreSQL hackers mailing-list message]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202504012022.wzrtvfhrltud%40alvherre.pgsql

Herrera, Á. (2024, April 12). *Re: Can't find not null constraint, but \\d+ shows that* [PostgreSQL hackers mailing-list message; superseded development-state evidence]. PostgreSQL Global Development Group. https://www.postgresql.org/message-id/202404120752.6ebv4q5zwnfw%40alvherre.pgsql
