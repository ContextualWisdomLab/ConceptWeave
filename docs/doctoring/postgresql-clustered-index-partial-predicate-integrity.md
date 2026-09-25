# PostgreSQL clustered-index partial-predicate integrity

## Decision

ConceptWeave must reject a relation snapshot that reports `pg_index.indisclustered = true` for an index that also carries a partial predicate. The owner-level error remains `InvalidObservationField { field: "index_clustered" }` because the contradiction concerns the relation's persisted clustered-index selection, not the independent validity of partial indexes.

This rule is relation-local and intentionally narrower than a general index-access-method policy. Zero clustered indexes, one non-partial clustered index, and ordinary partial indexes remain admissible.

## PostgreSQL 18 authority

Primary source generation: PostgreSQL `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`.

The canonical `ALTER TABLE ... CLUSTER ON` path in `src/backend/commands/tablecmds.c` resolves the target index, calls `check_index_is_clusterable(rel, indexOid, lockmode)`, and only then calls `mark_index_clustered(rel, indexOid, false)`.

`src/backend/commands/cluster.c::check_index_is_clusterable()` rejects incomplete indexes by checking `pg_index.indpred`; a non-null predicate is rejected because a partial index does not index every row of the relation. Only after that eligibility check can PostgreSQL persist `indisclustered` for the selected index. The same function separately checks access-method clusterability and `indisvalid`.

Reference documentation: PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CLUSTER*. https://www.postgresql.org/docs/18/sql-cluster.html

Source:

- `src/backend/commands/tablecmds.c`, `ATExecClusterOn()` at `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`
- `src/backend/commands/cluster.c`, `check_index_is_clusterable()` and `mark_index_clustered()` at the same revision

## ConceptWeave traceability

Observed representation:

- `crates/conceptweave-observation/src/representation_v3.rs`
- `IndexObservation::predicate()` preserves the server-rendered partial predicate.
- `IndexCatalogFlags::clustered()` preserves `pg_index.indisclustered`.

Aggregate owner seam:

- `crates/conceptweave-observation/src/lib.rs`
- `validate_schema_relation_invariants()` already enforces relation-level clustered-index cardinality, but at the finding predecessor it does not reject a clustered index whose `predicate()` is present.

Focused contract:

- `crates/conceptweave-observation/tests/index_clustered_partial_predicate_contract.rs`
- clustered + partial predicate: reject as `index_clustered`
- clustered + no predicate: preserve
- ordinary + partial predicate: preserve

Finding lineage:

- predecessor: `2da027ed2a362d75ab59d4d32d0e009e19903c8a`
- finding review: `5268119238`
- structural RED: `b371a51642eb4a7f114bc7142e38e2cf7bf1df55`

## Alternatives considered

### Reject every partial index

Rejected. PostgreSQL supports ordinary partial indexes. The contradiction exists only when the same observed index claims persistent clustered-index ownership.

### Hard-code clusterable access-method names

Rejected for this repair. PostgreSQL asks the access method capability `amclusterable`; ConceptWeave currently stores the observed method name but not source-authoritative AM capability evidence. Treating a static list of built-in method names as the full semantic contract would incorrectly constrain extension access methods and would duplicate provider truth.

### Require lifecycle evidence to be present

Rejected for this repair. `indisvalid` is separately modeled as optional evidence. This contract does not turn an unobserved lifecycle family into a fabricated value. A separately observed `indisvalid = false` contradiction can be governed by its own explicit contract without making absence mean false or true.

## Risk and effect

Without this invariant, ConceptWeave can publish an immutable semantic release that claims PostgreSQL persisted a clustered index selection through a catalog state the canonical PostgreSQL 18 owner path rejects. Downstream catalog, governance, and client consumers would then receive authoritative-looking evidence that cannot correspond to that source contract.

The repair is deliberately causal: validate only the contradiction already present in the bounded relation aggregate. No foreign domain truth, physical clustering operation, access-method capability inference, or runtime maintenance policy is introduced.
