# PostgreSQL ordinary EXCLUDE backing-index lifecycle integrity

## Decision

ConceptWeave treats `pg_index.indisready`, `indisvalid`, and `indislive` as independently observed lifecycle facts for the exact `pg_constraint.conindid` backing index of an ordinary PostgreSQL `EXCLUDE` constraint. Publication fails closed unless all three facts are present and true.

This is a cross-catalog source-integrity rule, not a derived default. The adapter must read the exact backing `pg_index` row and must not infer lifecycle health from `contype = 'x'`, `convalidated`, `conenforced`, a successful constraint lookup, or the mere existence of the index coordinate.

`pg_index.indcheckxmin` is intentionally not folded into this invariant. PostgreSQL defines it as a query-visibility horizon related to HOT chains, not as the ready/valid/live state that determines whether the index accepts writes, is a valid query index, or is being dropped.

The lifecycle read must also belong to the exact v3 content generation that produced the predecessor chain. Source key, policy binding, extractor revision, and timestamp are provenance coordinates, but they are not a substitute for exact source-content digest identity.

## Problem

The v3 source representation already preserves the three lifecycle columns independently as `Option<bool>` on `IndexObservation`, and the ordinary-EXCLUDE stack already proves the exact `conindid` backing index plus its role, name, namespace, timing, and other catalog semantics. Before this repair, however, no ordinary-EXCLUDE successor required the enforcing index to have complete, healthy lifecycle evidence.

That allowed a governed successor to be constructed when the exact backing index had one of these source states:

- `indisready` unobserved or false;
- `indisvalid` unobserved or false;
- `indislive` unobserved or false.

Those states are materially different from a stable enforcing index. PostgreSQL 18 documents `indisready = false` as an index that must be ignored by `INSERT`/`UPDATE`, `indisvalid = false` as possibly incomplete and unsafe for queries, and `indislive = false` as an index being dropped and ignored for all purposes.

A bounded follow-up review found a second provenance defect in the first source repair: the lifecycle constructor compared source key, policy binding, extractor revision, and observation time against the namespace predecessor, but did not prove that the supplied v3 snapshot had the same content digest as the v3 generation that produced that predecessor. Distinct snapshots can share all four provenance coordinates. Without digest equality, lifecycle state from one source generation could be combined with namespace evidence from another generation.

## PostgreSQL authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_index`*. https://www.postgresql.org/docs/18/catalog-pg-index.html

Relevant catalog semantics:

- `indisvalid`: the index is currently valid for queries when true; false means it is possibly incomplete.
- `indisready`: the index is currently ready for inserts when true; false means `INSERT`/`UPDATE` must ignore it.
- `indislive`: false means the index is in the process of being dropped and should be ignored for all purposes.
- `indcheckxmin`: query use is restricted by a transaction visibility horizon; this is distinct from ready/valid/live lifecycle health.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: REINDEX*. https://www.postgresql.org/docs/18/sql-reindex.html

The documented `REINDEX CONCURRENTLY` phases demonstrate that PostgreSQL models index readiness, validity, constraint rebinding, and retirement as separate catalog transitions: the replacement becomes ready, constraints are later rebound while validity changes, and the old index is subsequently made not-ready before removal. PostgreSQL 18 also explicitly states that exclusion-constraint indexes themselves cannot be reindexed concurrently; this sequence is therefore used only as authoritative evidence for the meaning and independence of the generic `pg_index` lifecycle flags, not as a claimed transition path for an ordinary EXCLUDE backing index.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Constraints*. https://www.postgresql.org/docs/18/ddl-constraints.html

An exclusion constraint automatically creates the index used to enforce that constraint. ConceptWeave therefore binds lifecycle validation to the exact `conindid` index rather than to any same-name or same-relation index.

## Implementation traceability

Initial finding review: `5226246612` on PR #46 predecessor `3296e5e6e735fa8f1c10dadc6387bd2e6682bf32`.

Structural source/compile RED contract: `07fd2aecd176d4b114781d552e2de82894304ed2`, `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_index_lifecycle_contract.rs`. The test imported `IndexExclusionConstraintIndexLifecycleSnapshot` before that type existed. This is structural RED evidence only; no executed compiler failure is claimed.

Initial production repair: `a4f5dff99e58ee1be55a2160dcc7fc1ab0d8dea7`, `crates/conceptweave-relation-partition/src/index_exclusion_constraint_index_lifecycle.rs`.

Public composition: `203d3fe6a7ed730e9884059b441bee80fe7e2196`, `crates/conceptweave-relation-partition/src/index_partition.rs`.

Initial edge/provenance contract currentization: `5a11b4ca161623e231d804b8ee44df1f8cbf5e47`.

Follow-up finding review: `5226362550` on exact `742bef1a9c358e9bab123d4ec894289caa42b110` identified missing exact v3 content-generation binding.

Behavioral source-RED contract: `19fa58e1ba3355a808953e868150af364000e13c`. The contract constructs two v3 snapshots with the same source key, policy binding, extractor revision, timestamp, and exact index coordinate but different content digests and requires cross-generation lifecycle rebinding to fail closed. The execution environment did not contain Rust tooling, so no executed test failure is claimed.

Provenance propagation repair:

- `7d84bdbdfd4789f425acb1dd78ac97573510378d` retains the exact v3 source-content digest as predecessor metadata in `IndexExclusionConstraintIndexNameSnapshot` without changing its issued digest algorithm;
- `c1561475e331fae2fe93ae02383547d7610d05a1` carries the same immutable source digest through `IndexExclusionConstraintIndexNamespaceSnapshot`, again without changing its issued digest algorithm;
- `f7d8dbcf4056c3b1c53cfcffd0bf46d14052f63d` requires exact `base_snapshot.snapshot_digest() == namespace_snapshot.source_snapshot_digest()` before any lifecycle lookup.

The final successor contract therefore:

- uses the current backing-index namespace generation as the exact constraint/index binding authority;
- requires source identity, policy binding, exact v3 content digest, extractor revision, and observation time to match that predecessor generation;
- resolves the exact backing `IndexObservation` from the supplied v3 snapshot;
- rejects missing lifecycle bits instead of assuming PostgreSQL defaults;
- rejects any `ready=false`, `valid=false`, or `live=false` state;
- frames both the exact v3 base digest and namespace-predecessor digest into a new domain-separated lifecycle digest;
- retains exact constraint/index coordinates and all three raw lifecycle booleans in digest/provenance;
- does not modify any issued predecessor digest algorithm.

## Alternatives rejected

### Infer lifecycle health from constraint existence

Rejected. `conindid` identifies the backing index but does not replace the separate `pg_index` lifecycle columns. Treating a resolved constraint as equivalent to ready/valid/live would erase source evidence precisely where PostgreSQL exposes distinct index state.

### Accept missing lifecycle columns as healthy defaults

Rejected. The generic v3 model intentionally distinguishes `None` from explicit `true` or `false`. Publication cannot turn absence of observation into authoritative health.

### Treat matching provenance coordinates as exact source-generation identity

Rejected. Source key, policy binding, extractor revision, and timestamp are necessary provenance metadata but do not cryptographically bind catalog content. Exact v3 source digest equality is required before a downstream successor may reuse source facts from a supplied base snapshot.

### Reject `indcheckxmin = true` as part of the same rule

Rejected. PostgreSQL documents `indcheckxmin` as a query-snapshot/HOT-chain visibility restriction. It is material catalog evidence and is already retained by `IndexCatalogFlags`, but it is not equivalent to the three backing-index lifecycle states validated here.

### Reuse predecessor digest identity

Rejected. Adding a new cross-catalog invariant under an old digest would make previously issued evidence change meaning. The lifecycle repair uses its own domain-separated successor digest; the follow-up adds immutable predecessor metadata needed for exact rebinding without altering previously defined digest calculations.

## Live differential requirement

A PostgreSQL 18 differential must perform one bounded source observation that:

1. reads the ordinary `pg_constraint` row and its exact `conindid`;
2. follows that OID to the exact `pg_index.indexrelid` / index `pg_class` row;
3. reads `indisready`, `indisvalid`, and `indislive` independently from that `pg_index` row;
4. carries those booleans into the same exact v3 source-content generation that feeds the predecessor chain, preserving its source digest;
5. proves the ordinary-EXCLUDE lifecycle successor admits `true/true/true` and fails closed for missing or false lifecycle evidence;
6. proves a different v3 content digest cannot be rebound even when source key, policy, extractor revision, timestamp, and index coordinate all match.

A fixture that copies `true` values from an expected invariant rather than reading `pg_index`, or that mixes catalog evidence from different v3 content generations, is not differential evidence.

## Acceptance boundary

Source repair is not exact-head GREEN by itself. One unchanged final head still needs repository-pinned Rust 1.98 formatting, strict workspace/all-target Clippy, the focused lifecycle contract plus all retained Source Observation/relation-partition tests, workspace/doc tests, release build, rustdoc/coverage obligations, applicable hosted security/Product gates, and qualifying independent review. Hosted or native evidence from an earlier head does not transfer after any source or documentation commit.
