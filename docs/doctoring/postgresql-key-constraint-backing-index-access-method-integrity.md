# PostgreSQL key-constraint backing-index access-method integrity

## Problem

ConceptWeave observes PRIMARY KEY / UNIQUE constraints and their relation-scoped PostgreSQL index evidence independently. Once key-constraint timing is explicitly observed, the aggregate requires a same-name usable backing index and validates key shape, predicate absence, null treatment, catalog role, lifecycle state, and immediacy.

At predecessor `2bd20d586dd5af328ff590fcd2b458a71b16ccfb`, `canonicalize_constraint_timings()` only constrained the access method when `pg_index.indisexclusion = true`:

- exclusion backing index -> `gist`;
- non-exclusion backing index -> any observed access-method identifier.

That admits contradictory source evidence such as an ordinary PRIMARY KEY backed by a synthetic `gin` unique index or an ordinary UNIQUE constraint backed by non-exclusion `gist`.

## PostgreSQL 18 authority

Authority was re-read against `postgres/postgres` `REL_18_STABLE@051db7737c18b1c5d25cdc4ad508608c4b53fafc` on 2026-09-21.

PostgreSQL 18 documents that only B-tree indexes can be declared unique. PRIMARY KEY and ordinary UNIQUE constraints create unique B-tree indexes. PostgreSQL 18 separately defines `WITHOUT OVERLAPS` as temporal key semantics enforced through an exclusion-style GiST index. Those are distinct catalog shapes rather than interchangeable access methods.

The existing ConceptWeave temporal boundary already models that distinction through `IndexCatalogFlags::exclusion()` and requires `gist` when exclusion-backed timing/PERIOD evidence is present. The missing rule is the ordinary non-exclusion side: it must require `btree` rather than treating every access method as admissible.

Primary references:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 11.6. Unique indexes*. https://www.postgresql.org/docs/18/indexes-unique.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL source tree, REL_18_STABLE* [Git repository, commit `051db7737c18b1c5d25cdc4ad508608c4b53fafc`]. https://github.com/postgres/postgres/tree/051db7737c18b1c5d25cdc4ad508608c4b53fafc

## Bounded contract

Review `5266793081` records the finding on predecessor exact head `2bd20d586dd5af328ff590fcd2b458a71b16ccfb`.

Structural RED `6807aa7b45b0e04c6268b1e134c34bc3d988b0c9` adds `constraint_backing_index_access_method_contract.rs` with three boundaries:

1. ordinary PRIMARY KEY + non-exclusion `gin` backing index must fail as `constraint_backing_index`;
2. ordinary UNIQUE + non-exclusion `gist` backing index must fail as `constraint_backing_index`;
3. ordinary PRIMARY KEY and UNIQUE with `btree` backing indexes remain admissible.

The RED deliberately constructs contradictory catalog evidence because `IndexObservation` is an observation representation, not a PostgreSQL DDL executor. The aggregate is responsible for rejecting impossible combinations before they become governed semantic evidence.

## Minimal causal repair

The owning seam is `canonicalize_constraint_timings()` in `crates/conceptweave-observation/src/lib.rs`, adjacent to the existing exclusion-access-method check.

The intended rule is:

- `catalog_flags.exclusion() == false` -> `backing_index.access_method() == Some("btree")`;
- `catalog_flags.exclusion() == true` -> retain the existing `backing_index.access_method() == Some("gist")` requirement.

No change is required to standalone indexes that do not back an observed PRIMARY KEY / UNIQUE constraint. No provider-wide access-method normalization is introduced. No inference from method name to temporal semantics is allowed; `WITHOUT OVERLAPS` remains owned by explicit constraint-period evidence and the observed exclusion flag.

## Rejected alternatives

- **Require B-tree for every PRIMARY/UNIQUE-shaped index.** Rejected because PostgreSQL 18 `WITHOUT OVERLAPS` is the explicit GiST/exclusion path.
- **Validate access method only when PERIOD evidence is attached.** Rejected because ordinary explicitly observed key constraints already have enough source evidence to reject a non-exclusion non-B-tree backing index.
- **Normalize an unexpected method to B-tree.** Rejected because Source Observation preserves source truth and fails closed on contradictory catalog combinations.
- **Move the rule into `IndexObservation::with_access_method()`.** Rejected because standalone indexes may legitimately use many access methods; the invariant belongs to the key-constraint/index aggregate relationship.

## Risk and effect

Without the repair, an impossible key-constraint/index pairing can receive a stable snapshot digest and later be consumed as authoritative semantic metadata. That weakens lineage, migration planning, and schema-diff decisions that depend on constraint enforcement semantics.

The bounded repair narrows only impossible ordinary key-constraint catalog shapes. Temporal GiST/exclusion keys, standalone GiST/GIN indexes, and unobserved timing families retain their existing ownership boundaries.

## Acceptance

This document and the structural RED do not claim execution GREEN. Acceptance requires the production predicate to be repaired on the same exact branch lineage, the focused RED and retained key/temporal/index contracts to execute GREEN, and the resulting exact head to pass the repository's Rust/PostgreSQL acceptance path. Exact-head evidence does not transfer after source or documentation movement.
