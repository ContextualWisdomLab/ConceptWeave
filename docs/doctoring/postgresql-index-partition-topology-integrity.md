# PostgreSQL 18 index-partition topology integrity

## Problem

The relation-partition successor introduced on Source Observation PR #46 preserved `pg_class.relispartition`, direct `pg_inherits` table parentage, detach state, and table/NOT NULL coherence for `PostgresSchemaSnapshotV3::relations()`. It did not preserve the same catalog facts for indexes because v3 represents indexes as nested `IndexObservation` values rather than top-level `RelationKind` values.

That omission is material. PostgreSQL 18 stores ordinary indexes as `pg_class.relkind = 'i'`, partitioned indexes as `relkind = 'I'`, and defines `relispartition` as true when a **table or index** is a partition. `pg_inherits` likewise stores each direct parent-child **table or index** relationship and carries `inhdetachpending`. A partitioned-table index is virtual; PostgreSQL creates or attaches child indexes on the table partitions. Therefore two v3 snapshots with byte-identical nested index definitions can have different source topology: one child index may be attached to the partitioned parent index while another remains local and unattached.

Without a successor evidence family, those two source states collapse to the same relation-partition governed identity.

A second source-integrity boundary appears once topology is represented. PostgreSQL 18 permits `CREATE INDEX ON ONLY` a partitioned table as a staged operation: the partitioned index is initially invalid, child indexes can be created independently and attached one partition at a time, and the parent becomes valid automatically only after every table partition has a matching attached child index. A governed state with an explicitly valid partitioned index but a direct table partition lacking any child-index attachment is therefore source-impossible.

## Constraint and rejected alternatives

The frozen v3 digest and the existing relation-partition digest are historical contracts. Adding index `relkind`, `relispartition`, or `pg_inherits` fields directly to either predecessor would silently change existing digest meaning and receipt semantics. Treating table partition membership as proof of index attachment is also invalid: PostgreSQL permits indexes to be created independently on partitions and attached later with `ALTER INDEX ... ATTACH PARTITION`.

The repair therefore uses another domain-separated immutable successor rather than changing either predecessor family. It remains Source Observation evidence; it does not confer semantic authority, publication authority, or execution authority.

The validity rule also does **not** infer attachment from index-name or definition similarity. It uses the already explicit direct index-parent evidence owned by this successor. When parent validity is false or unobserved, a local child index remains representable because it can be part of a legitimate staged attachment workflow.

## Decision

`conceptweave-relation-partition` now exposes an index-partition evidence family layered over one exact `PostgresSchemaSnapshotV3` and one exact `RelationPartitionSnapshot`.

The family is complete over every nested observed index and preserves:

- ordinary index versus partitioned-index `pg_class.relkind`;
- index `pg_class.relispartition`;
- exact relation-scoped child and parent index coordinates;
- direct `pg_inherits` parentage;
- `inhdetachpending`, which fails closed before immutable identity;
- deterministic domain-separated digest and exact source receipts;
- parent-index resolution to an observed partitioned index;
- consistency between the index parent owner and the independently observed direct table-partition parent;
- the PostgreSQL validity lifecycle: an explicitly valid partitioned index must have an attached child index on every observed direct table partition.

A local/unattached child index remains legal when its owning table is a partition and the parent partitioned index is invalid or its validity was not observed. Attachment is asserted only when the index evidence itself carries the exact direct parent.

## Review, RED, repair, and chronology

Initial topology repair:

- Finding review: PR #46 review `5199149942` on exact predecessor `18c19281260f98d450b399a12c469fe86d9acbb5`.
- Staged source: `209f2a5832396bba420ede11a52d54406340e3eb` created `index_partition.rs`, but the module was not referenced from the crate root and therefore did not change reachable production behavior.
- Behavioral RED source: `1dd392936aed8103c4ebb6879f923343ab733818` added the external contract while the module was still unreachable. It requires attached and local/unattached child-index topology to produce different governed identity and covers wrong relation-parent, incomplete-family, detach-pending, and receipt boundaries.
- Active production wiring: `7865809a5dc090679a1b34d3c8074193c652f395` exposed the successor from the crate root.
- Static source correction: `2dc8c91535acc99550e0760166451bf31631880f` replaced an invalid derived ordering dependency on `RelationKind` with canonical coordinate ordering by its stable token.

Partitioned-index validity repair:

- Finding review: PR #46 review `5199344556` on exact predecessor `d156ad01cd6bc7b964952076a1bab71af5c4d416`.
- Behavioral RED source: `d819639b3f8973b2d3bca166c9a091b6868b92e1` requires `valid parent + local/unattached direct child` to fail while preserving the legitimate `invalid parent + staged local child` state.
- Minimal production repair: `664afcc1684ead7569d9d6dd070afca4ef847ceb` resolves the exact predecessor `IndexObservation.valid()` state and, only for `Some(true)`, requires an explicit child-index parent edge for every direct table partition before hashing.

The initial chronology is intentional evidence: the implementation text existed before the first RED commit, but it was not reachable through the production crate API. The executable external contract preceded the commit that made the production path active. The validity repair follows the ordinary review → behavioral RED source → minimal causal production change sequence. No claim is made that Rust RED or GREEN was executed on this host.

## Invariants

An admitted `IndexPartitionSnapshot` must satisfy all of the following:

1. the supplied relation-partition snapshot recomputes to the same digest from the supplied v3 base;
2. every nested v3 index has exactly one index-partition observation and no unknown index coordinate is admitted;
3. indexes owned by partitioned tables are represented as partitioned-index relations; indexes on other currently indexable relation kinds are ordinary indexes;
4. `relispartition=true` has exactly one direct parent index and detach-pending state is rejected;
5. an attached child index is owned by a table partition and its parent index is owned by that table's exact direct partition parent;
6. the parent index is observed and is a partitioned index;
7. an explicitly valid partitioned index has at least one explicitly attached child index for every direct table partition; an invalid or unobserved-validity parent may retain unattached local child indexes during staged construction;
8. the parent graph is acyclic before digesting;
9. input order does not affect canonical identity; topology does.

## Risk and follow-up

The current branch still lacks exact-head native execution because the available execution host has no repository-pinned Rust toolchain, and protected ConceptWeave `main` still lacks the repository-owned Product pull-request workflow. These commits are source repair, not acceptance evidence.

The PostgreSQL adapter must eventually capture index `pg_class.relkind`, index `relispartition`, `pg_index.indisvalid`, and direct index `pg_inherits` rows in the same bounded catalog snapshot used for relation and index evidence. Catalog OIDs may be used only for capture-time joins; governed identity uses resolved coordinates. Publication remains blocked until one unchanged exact head has native Rust and applicable hosted acceptance.

PostgreSQL also requires `ALTER INDEX ... ATTACH PARTITION` targets to have an equivalent definition. The direct `pg_inherits` edge remains the authoritative evidence that PostgreSQL accepted the attachment; this successor does not reconstruct server attachment eligibility from names or partial client-side heuristics. The adapter must resolve that exact edge, not synthesize it from similar index definitions.

## Primary references

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.11. pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.27. pg_inherits*. https://www.postgresql.org/docs/18/catalog-pg-inherits.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 5.12. Table partitioning*. https://www.postgresql.org/docs/18/ddl-partitioning.html

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: ALTER INDEX*. https://www.postgresql.org/docs/18/sql-alterindex.html
