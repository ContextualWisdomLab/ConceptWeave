# PostgreSQL ordinary EXCLUDE backing-index lifecycle integrity

## Decision

ConceptWeave treats `pg_index.indisready`, `indisvalid`, and `indislive` as independently observed lifecycle facts for the exact `pg_constraint.conindid` backing index of an ordinary PostgreSQL `EXCLUDE` constraint. Publication fails closed unless all three facts are present and true.

This is a cross-catalog source-integrity rule, not a derived default. The adapter must read the exact backing `pg_index` row and must not infer lifecycle health from `contype = 'x'`, `convalidated`, `conenforced`, a successful constraint lookup, or the mere existence of the index coordinate.

`pg_index.indcheckxmin` is intentionally not folded into this invariant. PostgreSQL defines it as a query-visibility horizon related to HOT chains, not as the ready/valid/live state that determines whether the index accepts writes, is a valid query index, or is being dropped.

## Problem

The v3 source representation already preserves the three lifecycle columns independently as `Option<bool>` on `IndexObservation`, and the ordinary-EXCLUDE stack already proves the exact `conindid` backing index plus its role, name, namespace, timing, and other catalog semantics. Before this repair, however, no ordinary-EXCLUDE successor required the enforcing index to have complete, healthy lifecycle evidence.

That allowed a governed successor to be constructed when the exact backing index had one of these source states:

- `indisready` unobserved or false;
- `indisvalid` unobserved or false;
- `indislive` unobserved or false.

Those states are materially different from a stable enforcing index. PostgreSQL 18 documents `indisready = false` as an index that must be ignored by `INSERT`/`UPDATE`, `indisvalid = false` as possibly incomplete and unsafe for queries, and `indislive = false` as an index being dropped and ignored for all purposes.

## PostgreSQL authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_index`*. https://www.postgresql.org/docs/18/catalog-pg-index.html

Relevant catalog semantics:

- `indisvalid`: the index is currently valid for queries when true; false means it is possibly incomplete.
- `indisready`: the index is currently ready for inserts when true; false means `INSERT`/`UPDATE` must ignore it.
- `indislive`: false means the index is in the process of being dropped and should be ignored for all purposes.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: REINDEX*. https://www.postgresql.org/docs/18/sql-reindex.html

`REINDEX CONCURRENTLY` is especially useful as a transition authority. PostgreSQL first makes the replacement index ready for inserts, later changes constraints to refer to the replacement while making it valid, and only afterward disables readiness on the old index before dropping it. A constraint reference therefore must not be treated as evidence that arbitrary lifecycle flags are healthy; the flags are separate catalog state and must be observed.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html

An exclusion constraint automatically creates the index used to enforce that constraint. ConceptWeave therefore binds lifecycle validation to the exact `conindid` index rather than to any same-name or same-relation index.

## Implementation traceability

Finding review: `5226246612` on PR #46 predecessor `3296e5e6e735fa8f1c10dadc6387bd2e6682bf32`.

Structural source/compile RED contract: `07fd2aecd176d4b114781d552e2de82894304ed2`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_lifecycle_contract.rs`. The test imported `IndexExclusionConstraintIndexLifecycleSnapshot` before that type existed. This is structural RED evidence only; no executed compiler failure is claimed.

Production repair: `a4f5dff99e58ee1be55a2160dcc7fc1ab0d8dea7`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_lifecycle.rs`.

Public composition: `203d3fe6a7ed730e9884059b441bee80fe7e2196`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

Edge/provenance contract currentization: `5a11b4ca161623e231d804b8ee44df1f8cbf5e47`.

The successor:

- uses the current backing-index namespace generation as the exact constraint/index binding authority;
- requires base-snapshot source identity, policy binding, extractor revision, and observation time to match that predecessor generation;
- resolves the exact backing `IndexObservation` from the supplied v3 snapshot;
- rejects missing lifecycle bits instead of assuming PostgreSQL defaults;
- rejects any `ready=false`, `valid=false`, or `live=false` state;
- frames both the v3 base digest and namespace-predecessor digest into a new domain-separated digest;
- retains exact constraint/index coordinates and all three raw lifecycle booleans in digest/provenance;
- does not modify any issued predecessor digest domain.

## Alternatives rejected

### Infer lifecycle health from constraint existence

Rejected. `conindid` identifies the backing index but does not replace the separate `pg_index` lifecycle columns. Treating a resolved constraint as equivalent to ready/valid/live would erase source evidence precisely where PostgreSQL exposes transition state.

### Accept missing lifecycle columns as healthy defaults

Rejected. The generic v3 model intentionally distinguishes `None` from explicit `true` or `false`. Publication cannot turn absence of observation into authoritative health.

### Reject `indcheckxmin = true` as part of the same rule

Rejected. PostgreSQL documents `indcheckxmin` as a query-snapshot/HOT-chain visibility restriction. It is material catalog evidence and is already retained by `IndexCatalogFlags`, but it is not equivalent to the three backing-index lifecycle states validated here.

### Reuse predecessor digest identity

Rejected. Adding a new cross-catalog invariant under an old digest would make previously issued evidence change meaning. This repair uses a new domain-separated successor digest.

## Live differential requirement

A PostgreSQL 18 differential must perform one bounded source observation that:

1. reads the ordinary `pg_constraint` row and its exact `conindid`;
2. follows that OID to the exact `pg_index.indexrelid` / index `pg_class` row;
3. reads `indisready`, `indisvalid`, and `indislive` independently from that `pg_index` row;
4. carries those booleans into the v3 `IndexObservation` without deriving them from constraint state;
5. proves the ordinary-EXCLUDE lifecycle successor admits `true/true/true` and fails closed for missing or false lifecycle evidence.

A fixture that copies `true` values from an expected invariant rather than reading `pg_index` is not differential evidence.

## Acceptance boundary

Source repair is not exact-head GREEN by itself. One unchanged final head still needs repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the focused lifecycle contract plus all retained Source Observation/relation-partition tests, workspace/doc tests, release build, rustdoc/coverage obligations, applicable hosted security/Product gates, and qualifying independent review. Hosted or native evidence from an earlier head does not transfer after any source or documentation commit.
