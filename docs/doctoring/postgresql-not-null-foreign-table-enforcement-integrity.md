# PostgreSQL 18 foreign-table NOT NULL enforcement integrity

## Problem

Source Observation models PostgreSQL 18 first-class `NOT NULL` rows from `pg_constraint` and retains `conenforced` in governed identity. The previous constructor rejected `conenforced = false` for every relation kind admitted by the model. That rule was correct for ordinary and partitioned tables, but it incorrectly erased a valid foreign-table source state.

PostgreSQL 18 documents separate grammars for ordinary tables and foreign tables. `CREATE TABLE` states that `NOT ENFORCED` is currently supported only for foreign-key and `CHECK` constraints. `CREATE FOREIGN TABLE`, however, permits `[ ENFORCED | NOT ENFORCED ]` after both column-level and table-level `NOT NULL` constraints. The foreign-table documentation also explains that PostgreSQL core does not itself enforce foreign-table `CHECK` or `NOT NULL` constraints and that such declarations are expected to describe guarantees supplied by the remote system.

Treating the ordinary-table restriction as universal therefore makes ConceptWeave unable to represent a PostgreSQL-18-valid foreign-table `NOT NULL ... NOT ENFORCED` catalog observation.

## Decision

`NotNullConstraintObservation::new` applies enforcement admissibility by owning relation kind:

- `RelationKind::Table` and `RelationKind::PartitionedTable` continue to reject `enforced = false` for `NOT NULL`;
- `RelationKind::ForeignTable` accepts both `enforced = true` and `enforced = false` and retains the observed bit in the existing NOT NULL digest framing;
- no caller-selected default or inferred enforcement state is introduced.

This is a source-representation correction, not a change in semantic authority. The foreign-data wrapper or remote server remains responsible for whatever guarantee the foreign-table constraint is intended to describe; ConceptWeave records the catalog fact and does not assert that PostgreSQL core enforced it.

## Rejected alternatives

Applying the `CREATE TABLE` restriction to every table-like `relkind` was rejected because PostgreSQL publishes a distinct `CREATE FOREIGN TABLE` grammar. Dropping `conenforced` from foreign-table identity was rejected because it would collapse two materially different catalog states. Accepting `NOT ENFORCED` for ordinary or partitioned-table `NOT NULL` constraints was rejected because PostgreSQL 18 does not support that form there.

## Executable traceability

- Finding: PR #46 review `5196207955`, exact predecessor `1e30ed3ba3a017db1309e3f10011bc5543c0d267`.
- RED contract: `crates/conceptweave-observation/tests/not_null_constraint_enforcement_contract.rs`, commit `6d2b22891aecdc6be6375a0ca56c648fc853b64b`.
  - ordinary-table `NOT NULL NOT ENFORCED` remains rejected;
  - foreign-table `NOT NULL NOT ENFORCED` must be constructible and preserve `enforced = false`.
- Minimal production repair: `crates/conceptweave-observation/src/not_null_constraint.rs`, commit `41de3e390905299c3b77e7ba9dbd950c2fa90d46`.
- Governed-identity regression: `crates/conceptweave-observation/tests/not_null_constraint_enforcement_contract.rs`, commit `5f786846a03974dbe8a6eb085fb9e151d20202d2`, proves foreign-table enforced and not-enforced observations produce distinct snapshot digests.
- The production delta itself remains one file, four additions and four deletions relative to the RED head; parent topology, completeness, validation asymmetry, constraint-name integrity, and digest framing are unchanged.

The executable contracts are source-level evidence only until one unchanged exact head runs the repository-pinned Rust 1.98 and hosted acceptance gates.

## Adapter obligation

The PostgreSQL adapter must capture `pg_constraint.conenforced` from the same bounded catalog snapshot as the rest of the first-class NOT NULL row and pass the observed bit unchanged. It must not normalize a foreign-table row to `true` merely because ordinary-table NOT NULL constraints are enforced, and it must not synthesize `false` merely because PostgreSQL core does not enforce foreign-table constraints. Relation kind and `conenforced` are separate source facts and both remain bound to governed identity.

## Primary sources

PostgreSQL Global Development Group. (2026). *CREATE FOREIGN TABLE*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/sql-createforeigntable.html

PostgreSQL Global Development Group. (2026). *CREATE TABLE*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *pg_constraint*. PostgreSQL 18 documentation. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
