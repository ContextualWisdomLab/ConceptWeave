# PostgreSQL relation-partition type-modifier integrity

## Decision status

Proposed/source-repaired; exact-head execution and live PostgreSQL adapter acceptance remain pending.

## Problem

`RelationPartitionSnapshot` already checks declarative-partition rowtypes by exact column-name set and qualified PostgreSQL type identity. Because frozen `ColumnObservationV3` does not expose `pg_attribute.atttypmod`, that predecessor also compares adapter-rendered `data_type` text as a temporary fail-closed modifier witness.

That text is not the catalog value PostgreSQL uses for rowtype compatibility. PostgreSQL 18 documents `pg_attribute.atttypmod` as an `int4` carrying type-specific creation-time data and notes that `-1` is common when no modifier is needed. More importantly, `REL_18_STABLE` `build_attrmap_by_name()` matches attributes by name and then rejects the mapping when either `atttypid` or raw `atttypmod` differs. Therefore equal rendered type text cannot prove exact modifier equality, and rendered text must not become governed semantic identity.

Review `5202118195` on exact predecessor `738f668e61a7f0a3bc4e459a3d5416e19bf868c6` records this P1 source-integrity gap.

## Constraints

- Frozen v3 digest/receipt meaning cannot change.
- The already-issued relation-partition predecessor cannot be silently rewritten.
- Database-local OIDs are join coordinates, not governed identity.
- Type-specific decoding of `atttypmod` is unnecessary for PostgreSQL's exact rowtype equality and would invent a cross-type abstraction ConceptWeave does not own.
- Missing evidence is not equivalent to `atttypmod = -1`.

## Alternatives

1. Continue comparing rendered `data_type` text. Rejected: display rendering is not the catalog comparison coordinate and can hide a raw modifier mismatch.
2. Parse rendered type text back into a modifier. Rejected: this reverses presentation into source identity, is type-specific, and can drift from PostgreSQL internals.
3. Add `atttypmod` directly to frozen `ColumnObservationV3`. Rejected: it would change established v3 identity and receipts.
4. Preserve the exact signed catalog value in a domain-separated successor. Selected.

## Source RED

Commit `2523a121214bd03b3e90bd3e45a391f7615f69c5` adds `relation_partition_type_modifier_contract.rs` before the production API exists. Its critical regression keeps parent and child rendered type text identical while supplying raw `atttypmod` 36 versus 68; the structured successor must reject that pair. Positive controls preserve name-based mapping across different physical column ordinals, accept exact `-1`, require complete per-column evidence, change successor digest when modifier evidence changes, and bind receipts to exact column coordinates.

This is committed source RED, not executed Rust evidence. The available automation host does not provide the repository-pinned Rust 1.98 toolchain, and the protected ConceptWeave branch still lacks the repository-owned Product PR workflow.

## Causal repair

Production `ccbea8dc919a13184488002b1b6f274802d93834` adds a domain-separated `RelationPartitionTypeModifierSnapshot` and related observation/location/receipt value objects.

The successor:

- preserves raw signed `i32` `atttypmod` exactly, including `-1`;
- requires one observation for every bounded predecessor column and rejects duplicate/unknown/missing coordinates;
- rebound-validates that the supplied `RelationPartitionSnapshot` is the exact digest produced from the supplied frozen-v3 predecessor;
- compares direct declarative parent/child columns by stable name and requires exact raw modifier equality;
- does not compare physical `ordinal_position`;
- frames the exact relation-partition predecessor digest under `conceptweave.postgres_schema_snapshot.v3.relation_partition.atttypmod.v1`;
- issues exact per-column provenance receipts without copying credentials or database-local OIDs.

The earlier rendered `data_type` check remains part of predecessor history. The structured successor is the authoritative new modifier evidence, but the predecessor's conservative text check can still reject a case before this successor is constructed. Removing that redundant bridge requires an explicit future version transition rather than mutation of issued predecessor semantics.

## Remaining acceptance work

The concrete PostgreSQL Source Observation adapter still has to read raw `pg_attribute.atttypmod`, emit a complete observation for each bounded column, and prove parity against real PostgreSQL partition attachment/rowtype behavior. One unchanged #46 exact head must also pass repository-pinned Rust 1.98 formatting, strict Clippy, focused and retained tests, rustdoc, release build, owned coverage, and hosted Product/security/dependency/review gates before this source repair can be called GREEN.

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html

PostgreSQL Global Development Group. (2025). *attmap.c* (REL_18_STABLE) [Source code]. PostgreSQL. https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/access/common/attmap.c
