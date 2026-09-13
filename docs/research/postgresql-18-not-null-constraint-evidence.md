# PostgreSQL 18 NOT NULL constraint evidence

Status: source repaired / exact-head acceptance pending  
Owner: ConceptWeave / Source Observation  
Decision date: 2026-09-13

## Problem

The v3 relation representation originally stored only the column-level nullable summary. That was insufficient once PostgreSQL 18 moved explicit column `NOT NULL` specifications into `pg_constraint`.

PostgreSQL 18 documents `pg_attribute.attnotnull` as a summary that the column has a **possibly invalid** not-null constraint. The authoritative constraint row is separately represented by `pg_constraint.contype = 'n'`, where PostgreSQL exposes the constraint name, enforcement and validation state, locality/inheritance fields, constrained attribute coordinate, and partition-parent relationship.

Consequently, two PostgreSQL 18 schemas can have the same column name, type, ordinal and `attnotnull` summary while differing in material constraint evidence such as:

- explicit constraint name (`conname`);
- validated versus `NOT VALID` (`convalidated`);
- enforced versus `NOT ENFORCED` (`conenforced`);
- locally defined versus inherited state (`conislocal`, `coninhcount`);
- inheritable versus `NO INHERIT` (`connoinherit`);
- partition-parent constraint linkage (`conparentid`, resolved to a stable source coordinate rather than retained as an OID).

A digest that binds only the nullable boolean therefore cannot be treated as lossless PostgreSQL 18 source identity.

## Primary-source findings

PostgreSQL 18 release notes state that column `NOT NULL` specifications are stored in `pg_constraint`, enabling explicit names, foreign-table not-null constraints and inheritance control. The same release adds `NOT VALID` support and mutable inheritability for not-null constraints.

The PostgreSQL 18 `pg_constraint` catalog documents `contype = 'n'` for not-null constraints and exposes `conname`, `conenforced`, `convalidated`, `conparentid`, `conislocal`, `coninhcount`, `connoinherit`, and `conkey`. The `pg_attribute` catalog describes `attnotnull` only as indicating a possibly invalid not-null constraint.

`coninhcount` is catalog type `int2` and is defined as the number of direct inheritance ancestors. The representation may expose this nonnegative count as an unsigned value, but values above PostgreSQL signed `int2` maximum 32767 are impossible source evidence and must be rejected before governed hashing.

`conparentid` has a narrower meaning than generic inheritance: PostgreSQL defines it as the corresponding constraint of the **parent partitioned table** when the row is a constraint on a partition. The resolved parent relation therefore has `pg_class.relkind = 'p'` (`PartitionedTable`). A stable parent coordinate carrying any other relation kind is impossible source evidence and must fail closed before governed hashing.

`ALTER TABLE` permits not-null constraints to be added `NOT VALID`, later validated, and marked `INHERIT`/`NO INHERIT`. PostgreSQL also permits `NOT ENFORCED` constraint state. The data-definition documentation allows explicit names and states that a column can have at most one explicit not-null constraint.

## Selected representation boundary

Do not modify the frozen `ColumnObservationV3` digest contract and do not infer first-class constraint state from `nullable`/`attnotnull`.

Use a domain-separated optional NOT NULL constraint family whose observations retain, at minimum:

- exact schema, relation kind, relation and column coordinate;
- exact constraint name;
- validation and enforcement state;
- `conislocal`, `coninhcount`, and `connoinherit`;
- resolved partition-parent constraint coordinate when `conparentid != 0`.

Catalog OIDs are join coordinates inside one captured catalog transaction and are not governed identity. A nonzero `conparentid` is resolved to an exact parent schema/relation/constraint coordinate only after verifying that the parent relation kind is `PartitionedTable`; a non-partitioned parent coordinate is rejected as contradictory catalog evidence.

The public model keeps `coninhcount` as a nonnegative count but validates it against the source catalog domain. Values above 32767 are rejected with `not_null_constraint_inheritance_ancestor_count` rather than hashed as if PostgreSQL could have emitted them.

When the family is declared observed, its column set must equal the bounded columns whose source `attnotnull` summary is true. An observed empty set is distinct from an unobserved family. Unknown columns, duplicate not-null constraints for one column, multi-column `conkey`, and contradictions between the constraint family and column nullability fail closed.

Input order is not semantic identity. Canonical ordering precedes domain-separated digest extension.

## Rejected alternatives

### Keep `nullable` as the only identity

Rejected because it loses the PostgreSQL 18 constraint name, validation/enforcement state and inheritance semantics.

### Reconstruct a synthetic not-null constraint from `attnotnull`

Rejected because `attnotnull` explicitly permits invalid state and cannot reconstruct the `pg_constraint` fields. Synthetic reconstruction would turn a summary flag into invented source evidence.

### Bind catalog OIDs

Rejected because OIDs are database-local join coordinates, not stable semantic identity. Partition-parent linkage must be resolved to source coordinates before entering governed evidence.

### Accept arbitrary relation kinds for resolved `conparentid`

Rejected because `conparentid` is specifically the corresponding constraint of a parent partitioned table. Preserving an arbitrary relation kind would make impossible catalog state look authoritative and would create governed identities PostgreSQL cannot produce.

### Permit the full `u16` range for `coninhcount`

Rejected because PostgreSQL 18 stores `coninhcount` as signed `int2`. Accepting 32768 through 65535 would admit source states the catalog cannot represent.

### Reuse generic CHECK evidence

Rejected because PostgreSQL 18 represents explicit not-null constraints as `contype='n'`, not as CHECK constraints, and exposes not-null-specific validation/inheritance behavior.

## Contract and repair lineage

`crates/conceptweave-observation/tests/not_null_constraint_contract.rs` pins first-class `NotNullConstraintObservation` identity, aggregate admission, observed-empty versus unobserved state, completeness against non-nullable bounded columns, contradiction rejection, one-explicit-not-null-per-column, input-order invariance, duplicate-family/order protection, and the signed-`int2` upper bound for `coninhcount`.

`crates/conceptweave-observation/tests/not_null_constraint_parent_contract.rs` additionally pins stable parent-coordinate identity and rejects any resolved `conparentid` coordinate whose parent relation kind is not `PartitionedTable`.

The aggregate family is implemented and source-repaired. Exact-head repository-pinned Rust and hosted acceptance remain separate gates; documentation never upgrades source repair into execution acceptance.

## Adapter obligation

A future PostgreSQL transport must collect `pg_attribute`, `pg_constraint`, and the joined `pg_class` parent relation metadata from the same bounded `REPEATABLE READ READ ONLY` catalog snapshot. It must resolve `conkey` to the exact bounded column and resolve nonzero `conparentid` to a stable parent constraint coordinate whose relation kind is `PartitionedTable` before constructing immutable evidence. `coninhcount` must be parsed as PostgreSQL signed `int2`, rejected if negative, and converted to the nonnegative representation only after that source-domain validation. Missing, duplicate, contradictory, out-of-domain, or partially resolved rows fail the complete-or-fail capture.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18.0 release notes*. https://www.postgresql.org/docs/release/18.0/

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 5.5 Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.7 pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.11 pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.13 pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
