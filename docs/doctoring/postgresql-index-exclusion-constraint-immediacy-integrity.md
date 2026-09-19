# PostgreSQL EXCLUDE constraint timing/index-immediacy integrity

## Decision

ConceptWeave must preserve `pg_constraint.condeferrable` and `pg_index.indimmediate` as independently observed source facts and fail closed when an ordinary `EXCLUDE` constraint disagrees with its exact `conindid` backing index.

For PostgreSQL 18 index-backed constraints, `indexcmds.c` maps `stmt->deferrable` to `INDEX_CONSTR_CREATE_DEFERRABLE`. In `index.c`, `index_create()` passes the `immediate` argument to `UpdateIndexRelation()` as false when that deferrable flag is present and true otherwise. `UpdateIndexRelation()` persists that value in `pg_index.indimmediate`. Executor arbiter validation also rejects a non-immediate index with the error that `ON CONFLICT` does not support deferrable unique constraints/exclusion constraints. These are independent primary-source confirmations that ordinary EXCLUDE constraint deferrability and its supporting index's immediacy are one cross-catalog invariant, not two unrelated booleans.

The PostgreSQL catalog documentation describes `indimmediate` in uniqueness terms and marks it irrelevant when `indisunique` is false. That wording is not sufficient to discard the field for ordinary EXCLUDE evidence: PostgreSQL's own creation path writes the flag from constraint deferrability, and executor code explicitly uses the flag for exclusion-constraint arbiter eligibility. ConceptWeave therefore observes the raw flag exactly and validates it against the independently observed constraint row instead of inferring or normalizing either side.

## Finding and scope

Review `5220488022` on exact predecessor `3e57508fe488aa059122206a7f21a3576babbe63` found that `IndexExclusionConstraintTimingSnapshot` governed `condeferrable` and `condeferred`, while the v3 backing-index observation already governed `indimmediate`, but no successor reconciled the two. A contradictory tuple such as `condeferrable=true` with `indimmediate=true` could therefore become governed evidence.

This repair is limited to ordinary `contype='x'` EXCLUDE constraints already identified by `IndexExclusionConstraintSnapshot`. Temporal PRIMARY KEY/UNIQUE `WITHOUT OVERLAPS` remains in the key-constraint owner family. The repair does not derive `condeferrable` from `indimmediate`, does not derive `indimmediate` from the constraint, and does not collapse `DEFERRABLE INITIALLY IMMEDIATE` with `DEFERRABLE INITIALLY DEFERRED`.

## Implementation

- `4ce488cdaf0e07a2406087867e153c59cc0b2a5d` adds `IndexExclusionConstraintImmediacySnapshot` and immutable per-coordinate coherence receipts.
- `f5b9638cabf3ab18311e7f5dc0893d0aa874ab11` adds the source/compile regression contract. At that exact commit the module is not yet publicly composed, so the new contract is an executable compile RED rather than an inferred failure.
- `d363119e32a1d33486c0b0812a60b9b41bafd195` publicly composes the successor through `index_partition.rs`.

The successor rebinds the exact v3 → relation-partition → index-partition → ordinary-EXCLUDE → timing predecessor chain before reading either fact. For every ordinary EXCLUDE coordinate it resolves the exact `conindid` backing index, requires observed `IndexCatalogFlags`, and accepts only:

- `condeferrable=false` with `indimmediate=true`;
- `condeferrable=true` with `indimmediate=false`.

Both `condeferred=false` and `condeferred=true` remain legal under the second case and remain distinct because the timing predecessor digest is part of the new domain-separated coherence digest.

The new digest domain is `conceptweave.postgres_schema_snapshot.v3.relation_partition.index_partition.exclusion_constraint.immediacy.v1`. No existing v3, partition, EXCLUDE identity, timing, enforcement, validation, `connoinherit`, period, key, or operator digest is rewritten.

## Regression contract

`crates/conceptweave-relation-partition/tests/index_exclusion_constraint_immediacy_contract.rs` fixes four boundaries:

1. deferrable EXCLUDE + immediate backing index fails closed;
2. non-deferrable EXCLUDE + non-immediate backing index fails closed;
3. both deferrable initial-timing modes are admitted with `indimmediate=false` and retain distinct governed digests;
4. non-deferrable EXCLUDE + `indimmediate=true` is admitted and produces exact receipt evidence.

The contract deliberately does not synthesize either raw bit from the other. The fixture supplies both catalog observations separately and asks the owner successor to validate their relationship.

## Primary sources and traceability

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_index`*. PostgreSQL documentation. The catalog defines `indimmediate` as the immediate-enforcement flag.

PostgreSQL Global Development Group. (2026). `src/backend/commands/indexcmds.c`, `REL_18_STABLE`. `DefineIndex()` maps `stmt->deferrable` to `INDEX_CONSTR_CREATE_DEFERRABLE` before calling `index_create()`.

PostgreSQL Global Development Group. (2026). `src/backend/catalog/index.c`, `REL_18_STABLE`. `index_create()` supplies `UpdateIndexRelation()` an immediate value that is false when `INDEX_CONSTR_CREATE_DEFERRABLE` is set and persists that value in `pg_index.indimmediate`.

PostgreSQL Global Development Group. (2026). `src/backend/executor/execIndexing.c`, `REL_18_STABLE`. Arbiter validation treats `!indimmediate` as a deferrable unique/exclusion index and rejects it for `ON CONFLICT` arbitration.

Traceability: review `5220488022` → contract `f5b9638c...` → production `4ce488cd...` → public composition `d363119e...` → `IndexExclusionConstraintImmediacySnapshot::new()` → `index_exclusion_constraint_immediacy_contract.rs`.

## Acceptance and live differential

Source commits are not GREEN evidence. Acceptance requires one unchanged exact head to pass repository-pinned Rust 1.98 formatting, strict all-target/workspace Clippy, the focused contract, all retained relation-partition/Source Observation tests, workspace/doc tests, release build, rustdoc and owned coverage gates, plus hosted Product/security/review gates.

The PostgreSQL 18 live differential must read `pg_constraint.condeferrable`, `condeferred`, `conindid` and backing `pg_index.indimmediate` in one bounded observation. It must exercise NOT DEFERRABLE, DEFERRABLE INITIALLY IMMEDIATE, and DEFERRABLE INITIALLY DEFERRED ordinary EXCLUDE constraints and prove the expected true/false/false `indimmediate` pattern without deriving one catalog fact from the other. Partition parent/child examples must preserve local constraint/index coordinates and still satisfy the same per-constraint invariant.
