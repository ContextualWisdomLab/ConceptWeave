# PostgreSQL index exclusion-constraint timing integrity

## Decision

ConceptWeave must preserve `pg_constraint.condeferrable` and `pg_constraint.condeferred` for every governed PostgreSQL `EXCLUDE` constraint. The existing exclusion-constraint successor remains immutable; timing is added through a domain-separated successor layered on its exact digest.

This decision closes review finding `5217974439` on Source Observation PR #46. The source/compile contract is `c7a1e8f6f0a045493be7bb498cb4033436805d3d`; the causal implementation is `34acbf3afd0df90c6fcd0d1e04fb854afbaf14ab`; public composition is exported by `a310d23328f2d0e9d29fff9084d843ba07ddc67e`.

## Problem

The preceding `IndexExclusionConstraintSnapshot` correctly preserves the independent `pg_constraint.contype = 'x'` object, resolved `conindid`, partition `conparentid`, `conislocal`, and `coninhcount`. It does not preserve the constraint's timing bits.

PostgreSQL 18 permits exclusion constraints to be:

- `NOT DEFERRABLE`;
- `DEFERRABLE INITIALLY IMMEDIATE`; or
- `DEFERRABLE INITIALLY DEFERRED`.

The catalog stores these semantics independently as `condeferrable` and `condeferred`. Therefore two source-reachable exclusion constraints can have the same index shape, operators, backing-index identity, parentage, and inheritance state while differing in when violations are checked. Collapsing those states into one governed digest is loss of source semantics, not presentation normalization.

The existing `ConstraintTimingObservation` does not close this gap: its contract is explicitly scoped to PRIMARY KEY and UNIQUE constraints. `pg_index.indimmediate` is also insufficient as a substitute. It can distinguish immediate index enforcement from a deferrable index, but it cannot preserve the independent default timing distinction between `DEFERRABLE INITIALLY IMMEDIATE` and `DEFERRABLE INITIALLY DEFERRED` that PostgreSQL stores in `pg_constraint.condeferred`.

## Authoritative source

PostgreSQL 18 `CREATE TABLE` includes `EXCLUDE` in the table-constraint grammar followed by `[ DEFERRABLE | NOT DEFERRABLE ] [ INITIALLY DEFERRED | INITIALLY IMMEDIATE ]`. The same manual states that EXCLUDE is one of the constraint kinds that accepts deferrability, and that `INITIALLY DEFERRED` changes the default check point from statement end to transaction end.

PostgreSQL 18 `pg_constraint` defines `contype = 'x'` as exclusion, `condeferrable` as whether the constraint is deferrable, and `condeferred` as whether it is deferred by default. These fields belong to the constraint object, not merely to its backing index.

The repository's existing PostgreSQL source authority remains `REL_18_STABLE@3d2e8573e9cb91bd2b545184f4f9b326d237bcd1`; this change does not revise earlier source pins or issued ConceptWeave digest domains.

## Alternatives considered

### Rewrite `IndexExclusionConstraintSnapshot`

Rejected. That snapshot has already issued a domain-separated digest and receipt contract. Adding timing fields directly would silently change the meaning of an existing governed identity and violate the immutable-successor rule.

### Infer timing from index catalog flags

Rejected. Constraint timing is explicitly represented in `pg_constraint`; inference from `pg_index` would lose `condeferred` and conflate independently observable source states.

### Reuse the PK/UNIQUE timing observation without changing its contract

Rejected. The existing type documents and scopes itself to key constraints. Reusing it for EXCLUDE would make the ubiquitous language and code contract disagree. A separate successor keeps the bounded meaning explicit without rewriting the earlier family.

### Domain-separated EXCLUDE timing successor

Selected. `IndexExclusionConstraintTimingSnapshot` binds complete timing observations to the exact predecessor exclusion-constraint digest. Each observation preserves the exact exclusion-constraint coordinate plus the two catalog bits. The successor digest frames the predecessor digest, deterministic coordinates, `condeferrable`, and `condeferred`.

## Invariants

1. Every exclusion constraint in the predecessor snapshot has exactly one timing observation.
2. Duplicate or missing timing coordinates fail closed.
3. `condeferrable = false` with `condeferred = true` fails closed as contradictory PostgreSQL timing state rather than being normalized.
4. `NOT DEFERRABLE`, `DEFERRABLE INITIALLY IMMEDIATE`, and `DEFERRABLE INITIALLY DEFERRED` produce distinct governed successor identities.
5. Timing receipts retain the same source ID, policy binding, extractor revision, and observation time as the exact predecessor.
6. Parent/child timing equality is not invented. This successor records exact catalog tuples; a cross-edge equality invariant requires separate PostgreSQL source evidence before enforcement.

## Regression contract

`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_timing_contract.rs` requires:

- initially-immediate and initially-deferred exclusion constraints to yield different successor digests;
- provenance receipts to retain exact timing bits;
- timing inventory to cover every predecessor exclusion constraint; and
- impossible `NOT DEFERRABLE` plus initially-deferred state to fail closed.

This is a source/compile contract until an unchanged exact head executes the repository-pinned Rust 1.98 suite. No RED/GREEN execution claim is made merely because the contract and production source exist.

## Live differential requirement

The PostgreSQL 18 transport differential must create real exclusion constraints in all three supported timing modes and read `pg_constraint.contype`, `conindid`, `conparentid`, `conislocal`, `coninhcount`, `condeferrable`, and `condeferred` in the same repeatable-read observation. It must prove that the two deferrable modes remain distinct after extraction and that the corresponding ConceptWeave successor digests remain distinct. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` remains owned by the key-constraint family and must not be double-counted as ordinary EXCLUDE evidence.

## Risk and follow-up

The repair increases the minimum catalog evidence required before a complete exclusion-constraint observation can become governed. That is intentional fail-closed behavior. Adapters that do not yet query the timing bits must remain incomplete rather than filling PostgreSQL defaults locally.

After native and hosted exact-head acceptance, the concrete transport must add these columns to its bounded catalog query and verify live differential evidence before #46 is adopted into #45.

## References

PostgreSQL Global Development Group. (2026). *CREATE TABLE — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/sql-createtable.html

PostgreSQL Global Development Group. (2026). *pg_constraint — PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

PostgreSQL Global Development Group. (2026, August 13). *PostgreSQL 18.6, 17.11, 16.15, 15.19, 14.24 and 19 Beta 3 released*. https://www.postgresql.org/
