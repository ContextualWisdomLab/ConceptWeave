# Source Observation: `WITHOUT OVERLAPS` backing-index evidence

## Decision

ConceptWeave treats `pg_constraint.conperiod` as the source-authoritative fact that a PostgreSQL PRIMARY KEY or UNIQUE constraint was declared with `WITHOUT OVERLAPS`. Index shape must never be used to invent that fact.

Once `conperiod=true` has been explicitly observed, however, governed evidence must remain coherent with PostgreSQL's implementation contract. PostgreSQL 18 specifies that `WITHOUT OVERLAPS` is enforced with exclusion semantics, that a UNIQUE constraint containing `WITHOUT OVERLAPS` automatically creates a GiST index with the same name as the constraint, and that PRIMARY KEY likewise uses GiST when `WITHOUT OVERLAPS` is specified. `pg_constraint.conexclop` also records per-column exclusion operators for `WITHOUT OVERLAPS` PRIMARY KEY/UNIQUE rows. Therefore an immutable ConceptWeave snapshot must not admit `conperiod=true` for a represented key while the same bounded representation lacks the same-name backing index or lacks the material catalog flags needed to establish its exclusion/GiST coherence.

This is a one-way coherence rule, not inference. `conperiod=true` requires the backing evidence; observing GiST/exclusion evidence does not imply `conperiod=true`.

## Current finding and RED

Review `5185780899` on PR #46 exact `11e40e59f3ab3fa0067c4bf78842166b375d8d2a` found that `canonicalize_constraint_periods()` checks the backing index only inside an optional `if let Some(backing_index) ... && let Some(catalog_flags) ...` branch. Missing index evidence or missing material catalog flags therefore bypasses the temporal-key coherence check.

Behavioral RED `c80d07816863f814e9b8fbb716661310d8b2b150` adds three witnesses in `constraint_period_backing_index_presence_contract.rs`:

- a `conperiod=true` temporal key with no same-name backing index must fail `constraint_period_backing_index`;
- a same-name GiST index without material catalog flags must fail the same invariant;
- an exact same-name GiST index with explicit exclusion catalog evidence remains admissible.

The production repair must be minimal. For PRIMARY KEY/UNIQUE with `conperiod=true`, require same-name backing-index evidence, material catalog flags, `indisexclusion=true`, and observed access method `gist`. Preserve the existing ordinary `conperiod=false` boundary: absence of unrelated index-family evidence must not become a new requirement merely because the `conperiod` family was observed.

## Alternatives rejected

Inferring `conperiod` from a GiST index or `indisexclusion` was rejected because `pg_constraint.conperiod` is the direct catalog truth. Requiring backing-index evidence for every `conperiod=false` key was rejected because it would couple the temporal family to optional index observation for ordinary keys. Accepting a temporal key with absent backing evidence was rejected because the governed snapshot could then encode a PostgreSQL state that cannot represent a valid `WITHOUT OVERLAPS` key.

## Traceability

Owner: Source Observation bounded context, PR #46.

Production seam: `crates/conceptweave-observation/src/lib.rs` → `canonicalize_constraint_periods()`.

Behavioral contract: `crates/conceptweave-observation/tests/constraint_period_backing_index_presence_contract.rs`.

Primary sources:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html

Status: behavioral RED active; production repair and unchanged-head native/Product acceptance are pending.
