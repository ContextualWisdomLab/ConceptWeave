# PostgreSQL clustered-index validity integrity

## Problem

ConceptWeave retains `pg_index.indisclustered` in `IndexCatalogFlags::clustered()` and `pg_index.indisvalid` in `IndexObservation::valid()`. A relation aggregate that accepts both `clustered=true` and an explicitly observed `valid=false` would publish catalog evidence that PostgreSQL's CLUSTER ownership path refuses to create.

## PostgreSQL 18 authority

Authority is PostgreSQL `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`.

- `src/backend/commands/tablecmds.c`: `ATExecClusterOn()` calls `check_index_is_clusterable()` before `mark_index_clustered()`.
- `src/backend/commands/cluster.c`: `check_index_is_clusterable()` rejects `!OldIndex->rd_index->indisvalid` with `cannot cluster on invalid index`.
- The same file's `mark_index_clustered()` defensively checks `indexForm->indisvalid` again before setting `indisclustered = true`.
- PostgreSQL 18 `pg_index` documents `indisclustered` as the index on which the table was last clustered and `indisvalid=false` as an index that cannot safely be used for queries.

The earlier partial-predicate guard remains independent: validity and predicate completeness are separate PostgreSQL checks and should remain separately testable.

## Decision

At the final relation aggregate, reject an index as `index_clustered` when all of the following are observed:

1. `catalog_flags.clustered() == true`; and
2. `index.valid() == Some(false)`.

Do not require `valid == Some(true)`. `None` means this optional lifecycle field was not observed and must remain admissible rather than being rewritten into a negative claim. Do not add `ready` or `live` requirements: the cited CLUSTER check is specifically on `indisvalid`. Do not infer PostgreSQL's `amclusterable` capability from access-method names; ConceptWeave does not yet own source-authoritative access-method capability evidence in this aggregate.

Ordinary invalid indexes that do not claim clustered ownership remain admissible catalog observations.

## Contract traceability

Focused structural contract: `crates/conceptweave-observation/tests/index_clustered_validity_contract.rs`.

Required cases:

- `clustered=true`, `valid=Some(false)` -> `InvalidObservationField { field: "index_clustered" }`;
- `clustered=true`, `valid=Some(true)` -> accepted;
- `clustered=true`, `valid=None` -> accepted;
- `clustered=false`, `valid=Some(false)` -> accepted.

The production seam is `validate_schema_relation_invariants()` in `crates/conceptweave-observation/src/lib.rs`, adjacent to the existing clustered cardinality and clustered partial-predicate guards. Exact-head execution evidence remains required after the source repair; structural RED alone is not execution RED/GREEN.
