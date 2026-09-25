# PostgreSQL replica-identity index cardinality

## Problem

ConceptWeave observes `pg_index.indisreplident` per index. Per-index eligibility checks are necessary but not sufficient: a relation containing two otherwise eligible indexes with `indisreplident = true` is not a coherent PostgreSQL replica-identity snapshot. The relation-level aggregate must therefore preserve the cardinality of the chosen index, not merely validate each flagged row in isolation.

## PostgreSQL 18 authority

Authority is PostgreSQL `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`.

- `pg_index.indisreplident` means that the index was chosen as replica identity by `ALTER TABLE ... REPLICA IDENTITY USING INDEX`.
- `pg_class.relreplident = 'i'` denotes an index with `indisreplident` set; the relation-level state names one index-mode identity rather than a set of identities.
- `RelationData` stores one `rd_replidindex` OID. `RelationGetIndexList()` scans the relation's `pg_index` rows, remembers the explicitly chosen replica index as one `candidateIndex`, and publishes one OID to `rd_replidindex` when `relreplident == REPLICA_IDENTITY_INDEX`.
- PostgreSQL's `replica_identity.sql` regression changes the selected index repeatedly and then explicitly counts `pg_index` rows with `indisreplident`; the expected result is `1`. Switching to `REPLICA IDENTITY DEFAULT` makes the expected count `0`.

Primary sources:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_index`*. `https://www.postgresql.org/docs/18/catalog-pg-index.html`
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_class`*. `https://www.postgresql.org/docs/18/catalog-pg-class.html`
- PostgreSQL Global Development Group. (2026). `src/backend/utils/cache/relcache.c`, `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`.
- PostgreSQL Global Development Group. (2026). `src/test/regress/sql/replica_identity.sql` and `src/test/regress/expected/replica_identity.out`, `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`.

## ConceptWeave invariant

For each observed relation, the number of indexes whose catalog flags have `replica_identity() == true` must be at most one.

This rule belongs in the final relation/index aggregate validation seam because an individual `IndexCatalogFlags` value cannot observe sibling indexes. It is deliberately independent from the existing per-index eligibility checks: the single flagged index, when present, must still satisfy relation kind, uniqueness, immediacy, non-partial shape, simple key-column identity, and key-column NOT NULL requirements.

Zero flagged indexes remains admissible. Source Observation does not infer `DEFAULT`, `FULL`, `NOTHING`, publication membership, subscriber state, or a missing `pg_class.relreplident` observation from the absence of an explicitly selected index.

The cardinality rule also does not import `indisready`, `indisvalid`, or `indislive` into the base seam. PostgreSQL 18 regression coverage explicitly permits replica identity to be attached to a not-yet-valid partitioned-table index during the supported dump/restore sequence.

## Executable contract

`crates/conceptweave-observation/tests/index_replica_identity_cardinality_contract.rs` pins both sides of the boundary:

- two otherwise eligible indexes with `replica_identity = true` on one table must fail closed as `index_replica_identity`;
- one eligible replica-identity index plus an unrelated ordinary unique index remains admissible.

Review authority: ConceptWeave #46 review `5263136719`. Structural RED commit: `a4e4eb57e83462ef4258c77af443a7c34ca5991c`.

The structural RED is not execution evidence. Exact-head GREEN requires the minimal relation-level cardinality repair and then the same unchanged-head Rust/PostgreSQL acceptance matrix required by #46.
