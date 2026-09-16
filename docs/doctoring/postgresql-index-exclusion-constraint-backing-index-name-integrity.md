# PostgreSQL 18 ordinary EXCLUDE backing-index name integrity

## Decision surface

ConceptWeave already preserves an ordinary `pg_constraint.contype = 'x'` coordinate and its exact resolved `conindid` backing-index coordinate as independent source evidence. Before this repair, those two coordinates were allowed to carry different names. That admits a source state PostgreSQL does not expose for an index-backed `EXCLUDE` constraint and also lets two such constraints bypass the schema relation namespace by using different synthetic index names.

PostgreSQL 18 documents two relevant invariants. First, an exclusion constraint is implemented by an index with the same name as the constraint. Second, index names occupy the schema-scoped relation namespace; the `CREATE TABLE` constraint-naming notes explicitly distinguish index-backed `UNIQUE`, `PRIMARY KEY`, and `EXCLUDE` constraints from relation-local non-index constraint naming. `ALTER INDEX ... RENAME` preserves the same coupling by renaming an associated `UNIQUE`, `PRIMARY KEY`, or `EXCLUDE` constraint as well. These are cross-catalog invariants, not a reason to derive one observation from the other.

The previous base-layer contract remains responsible only for relation-local `pg_constraint.conname` collisions among explicitly observed constraint rows. It must not grow a blanket schema-wide constraint-name rule because PostgreSQL permits non-index CHECK/NOT NULL/domain constraint names to repeat on different owning objects. The new successor owns the narrower index-backed name coupling and `pg_class` namespace check.

## Chosen repair

`IndexExclusionConstraintIndexNameSnapshot` is an ordinary-forward successor after `IndexExclusionConstraintCatalogShapeSnapshot`. It:

- rebinds the exact v3 -> relation-partition -> index-partition -> ordinary-EXCLUDE predecessor through `IndexExclusionConstraintSnapshot::new` and then rebinds the exact catalog-shape predecessor;
- compares the independently observed `IndexExclusionConstraintCoordinate::constraint_name()` with the resolved `conindid` `IndexPartitionCoordinate::index_name()` and fails closed on any difference;
- scans only the authorized bounded `PostgresSchemaSnapshotV3` to require exactly one observed index with that name in the owning schema and to require that occurrence to be the exact `conindid` relation/index coordinate;
- rejects a backing-index name that collides with any separately observed top-level relation name in the same schema;
- frames both the constraint coordinate and backing-index coordinate into a new domain-separated digest and provenance receipt, so equality is validated without erasing the fact that the values came from different catalogs.

No existing v3, relation-partition, index-partition, ordinary-EXCLUDE, catalog-shape, timing, enforcement, validation, `conkey`, `conexclop`, namespace, or backing-index digest domain is changed.

## Rejected alternatives

Changing `IndexExclusionConstraintSnapshot` to derive the constraint name from the index name was rejected because it would turn a cross-catalog validation rule into fabricated source evidence and would silently normalize extractor defects.

Applying schema-wide uniqueness directly to every `pg_constraint.conname` was rejected because it is stronger than PostgreSQL semantics for non-index constraints. The schema-wide restriction arises here from the associated index's `pg_class` relation name, not from `pg_constraint` by itself.

Rewriting the existing ordinary-EXCLUDE digest was rejected because already issued evidence must remain immutable. The successor pattern keeps the repair additive and lets consumers explicitly require the stronger governed contract.

## TRACEABILITY

- Finding review: PR #46 review `5223551886`, exact predecessor `9995a40ae33367ca0cd778740d02b6e55d937bcc`.
- Structural source/compile contract: `41d533d3894a2328df252ce4fe70e2529e92edb2`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_name_contract.rs`.
- Production successor: `8ac1c1ce762f980a15e6369302761e765ba94b02`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_name.rs`.
- Public composition: `49f4a60004531819a994aaa1264c34bc53425d9e`, `crates/conceptweave-relation-partition/src/index_partition.rs`.
- Base-layer contract wording repair: `4cf4f93109876eb528507bbd4669a37eb3ead8ca`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_partition_contract.rs`.
- Focused behavioral cases: matching names + receipt, mismatched constraint/index names, duplicate same-schema backing-index names, backing-index/top-level-relation namespace collision, and unknown receipt.

`41d533d...` is a structural compile-RED contract because the referenced successor type did not yet exist at that commit. It must not be represented as an executed compiler failure. The repaired exact head still requires repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, focused and retained workspace/doc tests, release build, rustdoc/test/edge coverage, and hosted acceptance before GREEN can be claimed.

## Live differential requirement

The PostgreSQL 18 differential fixture must capture `pg_constraint.conname`, `conindid`, the resolved backing `pg_class.relname`, its schema, and the bounded schema's other `pg_class` relation names independently. It must prove that an ordinary EXCLUDE row's constraint and backing-index names are identical and that the backing-index name occurs only once in the schema relation namespace. The extractor must not copy `conname` into index evidence or copy `relname` into constraint evidence to make the check pass.

Controls should include a non-index CHECK constraint whose name repeats on a different relation (allowed), two index-backed constraints attempting the same name in one schema (rejected by PostgreSQL), and an index name attempting to collide with a table/view/sequence name (rejected by the schema relation namespace).

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER INDEX*. https://www.postgresql.org/docs/18/sql-alterindex.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026). *PostgreSQL source: `src/backend/catalog/index.c`, REL_18_STABLE pinned authority `3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`*. `index_constraint_create()` and index relation creation/rename paths.