# PostgreSQL 18 relation-partition rowtype integrity

## Decision

A declarative-partition membership edge is source-valid only when the child rowtype can be mapped to its partitioned parent by PostgreSQL's name-based attribute map. Physical attribute order is not semantic identity, but the same non-dropped column set and compatible named-column types are required.

Review `5201119054` on exact #46 head `28236f1f68ab86a621047de5cb547b8db7597514` records the P1: `RelationPartitionSnapshot` previously proved parent kind, graph validity, detach state, and NOT NULL coherence but did not constrain the parent/child rowtypes. That allowed immutable evidence for declarative partitions that PostgreSQL could not create or attach and undermined the later index-expression child→parent attribute-map proof.

Source RED `37062c4b49229ba8f7aae56b7c4ac607f59e0398` covers a missing child column, an extra child column, a same-named column with a different qualified PostgreSQL type, a same type with a different modifier rendering, and a positive control whose physical ordinals differ while the stable name/type map is valid.

Production `ad371e2feddb74c7457e1f28f4fb9992f758ea6b` adds rowtype validation before the relation-partition digest is issued. It compares column sets by exact source name rather than ordinal, requires the frozen-v3 qualified type binding to agree, and retains frozen-v3 exact adapter-rendered type text as the temporary modifier witness because v3 does not structurally expose `pg_attribute.atttypmod`. Different physical order therefore remains admissible.

## PostgreSQL source semantics

PostgreSQL documents that a partition must have the same columns as its partitioned parent and that attaching an existing table requires matching column types. The source helper `build_attrmap_by_name()` is explicitly designed for partition/inheritance rowtypes that may have columns in different physical order. It searches by exact attribute name and rejects a mapped attribute when the PostgreSQL type OID or type modifier differs.

This is why ConceptWeave does not compare `ordinal_position` across a relation-partition edge. The ordinal remains exact source evidence inside frozen v3, while relation-partition validity uses stable name/type mapping.

## Type-modifier limitation

Frozen v3 separates exact server-rendered `data_type` text from the schema-qualified base type binding, but it has no structural `atttypmod` value. Until a future domain-separated successor carries the modifier structurally, the rowtype guard requires the exact adapter-rendered data-type witness to match after the qualified type binding matches. This is deliberately fail-closed and is narrower than pretending that equal base type names prove PostgreSQL rowtype compatibility.

A later structured type-modifier successor may replace this temporary witness only under a new digest domain. Frozen v3 and the current relation-partition digest meaning must not be rewritten.

## Alternatives rejected

- Matching columns by physical ordinal was rejected because PostgreSQL's name-based attribute map permits logically equivalent rowtypes with different physical order.
- Comparing only column names was rejected because PostgreSQL rejects same-named attributes with incompatible types.
- Comparing only the qualified base type was rejected because PostgreSQL also checks type modifiers.
- Adding/removing columns silently was rejected because declarative partitions cannot have a different user-column set from their parent.
- Rewriting frozen v3 to add `atttypmod` was rejected because existing predecessor digests and receipts are immutable.

No executed Rust RED/GREEN or hosted Product acceptance is claimed for this source repair. The current execution host does not provide repository-pinned Rust 1.98, so the committed RED and causal fix remain source evidence pending exact-head native and hosted acceptance.

## Traceability

- owner PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5201119054` on `28236f1f68ab86a621047de5cb547b8db7597514`
- source RED: `37062c4b49229ba8f7aae56b7c4ac607f59e0398`
- production repair: `ad371e2feddb74c7457e1f28f4fb9992f758ea6b`
- production: `crates/conceptweave-relation-partition/src/lib.rs`
- contract: `crates/conceptweave-relation-partition/tests/relation_partition_column_mapping_contract.rs`

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Table partitioning.* https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE.* https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: attribute mapping support in `src/backend/access/common/attmap.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/access/common/attmap.c
