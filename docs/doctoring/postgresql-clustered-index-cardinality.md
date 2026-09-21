# PostgreSQL clustered-index cardinality integrity

## Decision

ConceptWeave Source Observation treats `pg_index.indisclustered` as relation-scoped catalog state, not as an unconstrained independent flag on each index. For one observed relation, at most one index may have `IndexCatalogFlags::clustered() == true`. Zero clustered indexes remain admissible.

This invariant belongs at the final relation aggregate in `validate_schema_relation_invariants()`. It must not be moved into `IndexCatalogFlags::new()` or `RelationObservation::with_indexes()`, because neither local object has enough information to decide whether another index of the same relation already carries the flag.

Failure is reported as `ObservationError::InvalidObservationField { field: "index_clustered" }`, keeping the contract distinct from replica-identity cardinality and from index lifecycle evidence.

## PostgreSQL 18 authority

The authority snapshot used for this decision is PostgreSQL `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc`.

`pg_index.indisclustered` is documented as true when the table was last clustered on that index. PostgreSQL's `src/backend/commands/cluster.c::mark_index_clustered()` makes the single-valued relation semantics explicit: it walks the relation's index list, clears an existing `indisclustered` bit, and sets the requested index after rechecking validity. Passing an invalid index OID clears all clustered bits. `CLUSTER <relation>` without an explicit index later searches that relation's index list for the index carrying `indisclustered`.

The relevant consequence for immutable catalog evidence is cardinality, not command execution policy: a coherent same-generation relation snapshot may expose zero or one `indisclustered=true` index, never two.

## Contract boundary

The focused RED is `crates/conceptweave-observation/tests/index_clustered_cardinality_contract.rs`.

It requires three cases:

- two otherwise structurally valid same-relation indexes with `clustered=true` fail closed as `index_clustered`;
- one clustered index plus an ordinary index remains admissible;
- zero clustered indexes remain admissible.

The invariant intentionally does **not** infer that an index is clustered from ordering, physical layout, access method, uniqueness, primary-key status, or comments. It also does not import `CLUSTER` command eligibility or require optional `ready`, `valid`, or `live` evidence merely to establish cardinality. Those are separate observations and lifecycle contracts.

## Alternatives rejected

Treating `indisclustered` as a free per-index Boolean was rejected because it admits catalog states PostgreSQL's own state transition clears before setting a replacement. Requiring exactly one clustered index was rejected because a table may never have been clustered, and PostgreSQL explicitly supports the absence of a previously clustered index. Enforcing the rule inside the index constructor was rejected because cardinality cannot be determined from a single index in isolation.

## Traceability

- Finding review: ConceptWeave PR #46 review `5264637601`, exact predecessor `78a68f1a3a4c7c845bb7fea0d7414a6af50b3ac2`.
- Structural RED: `crates/conceptweave-observation/tests/index_clustered_cardinality_contract.rs`, introduced at `bc5ae3ccf566cbff701024d604478dac4223c964`.
- Production owner: `crates/conceptweave-observation/src/lib.rs::validate_schema_relation_invariants`.
- Catalog representation: `crates/conceptweave-observation/src/representation_v3.rs::IndexCatalogFlags::clustered`.

## References

PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.26. pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html

PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 source: CLUSTER catalog-state transitions, src/backend/commands/cluster.c* (REL_18_STABLE, commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc`). https://github.com/postgres/postgres/blob/051db7737c18b1c5d25cdc4ad508608c4b53fafc/src/backend/commands/cluster.c
