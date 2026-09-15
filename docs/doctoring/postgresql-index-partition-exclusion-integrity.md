# PostgreSQL 18 index-partition exclusion-semantics integrity

## Decision

A direct PostgreSQL index-partition edge is not definition-equivalent merely because both sides have the same access method, mapped attributes, collations, and operator families. PostgreSQL 18 `CompareIndexInfo()` also requires exclusion-index presence to agree. When exclusion semantics are present, it compares every key's exclusion operator OID, underlying procedure OID, and operator-family strategy number. ConceptWeave therefore needs a governed semantic successor for those arrays rather than treating `pg_index.indisexclusion` or rendered DDL as sufficient evidence.

Review `5200580545` on exact #46 head `226523ec16126db12cf4479ca1e0e286882fce64` records the P1. Behavioral source contract `198414959168b29ee92965576c3c91bd768a12a5` requires one-sided exclusion semantics to fail, separately isolates operator/procedure/strategy mismatches, preserves a matching positive control, and requires complete evidence for every key of every observed exclusion index.

Production `c6c97b2bc545b064c57faafeca67f7bea431235f` adds `IndexExclusionSemanticsSnapshot`, and `bd0e4e54fbc67f5e2ed32a72738dbadc90c91336` exposes it through the relation-partition crate. The successor rebound-validates the exact base → relation-partition → index-partition → operator-family chain, requires observed `indisexclusion` state for the bounded indexes, compares direct parent/child exclusion presence, and retains each exclusion key as a stable operator signature, stable underlying-procedure signature, and positive strategy number. Its digest frames the exact operator-family predecessor digest under a new domain separator, so frozen v3 and earlier successor identities do not change.

## Stable coordinates instead of OIDs

`RelationGetExclusionInfo()` returns operator OIDs, the OIDs of their underlying functions, and the strategy numbers obtained for those operators in the index operator family. OIDs are valid capture-time join coordinates but are not durable governed identity. The successor resolves:

- an operator to exact schema/name plus its binary left/right qualified types, because PostgreSQL operators are overloaded;
- an underlying procedure to exact schema/name plus its binary qualified argument types, because procedures are overloaded;
- the strategy as the positive `uint16` strategy number that PostgreSQL compares directly.

The observation constructor also requires the procedure's argument types to match the operator's operand types. This is a source-coherence invariant, not an inference about the operator family's behavior.

## PostgreSQL partitioning relevance

PostgreSQL 18 supports exclusion constraints on partitioned tables subject to partition-key restrictions: the relevant partition-key columns must be included and compared for equality so potentially conflicting rows are routed to the same partition. The existence of that feature makes exclusion-array equivalence material to an attached partition index rather than a dead catalog case.

## Alternatives rejected

- Comparing only `pg_index.indisexclusion` was rejected because `CompareIndexInfo()` separately compares operator, procedure, and strategy arrays.
- Storing catalog OIDs as release identity was rejected because OIDs are database-local capture coordinates.
- Comparing reconstructed `EXCLUDE ...` or `pg_get_indexdef` text was rejected because presentation text is not the semantic array PostgreSQL compares.
- Folding this evidence into frozen v3 or the operator-family digest was rejected because that would rewrite prior immutable identity meaning.

## Remaining definition-equivalence boundary

After this repair, the unresolved `CompareIndexInfo()` surface is canonical expression-tree equality under the partition attribute map and canonical partial-index predicate equality. PostgreSQL maps child Vars through the partition attribute map, rejects whole-row references that cannot be mapped, and then applies internal node equality. Raw `pg_get_expr` text or reconstructed index DDL is not accepted as a substitute. A future successor must preserve semantic expression structure or an equivalently verifiable canonical form without database-local OIDs or child attribute numbers becoming governed identity.

No executed Rust RED/GREEN or hosted Product acceptance is claimed for this source repair. The available execution host lacks the repository-pinned Rust toolchain and protected ConceptWeave `main` still lacks the Product PR workflow.

## Traceability

- owner PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5200580545` on `226523ec16126db12cf4479ca1e0e286882fce64`
- behavioral source RED: `198414959168b29ee92965576c3c91bd768a12a5`
- production semantics: `c6c97b2bc545b064c57faafeca67f7bea431235f`
- production export: `bd0e4e54fbc67f5e2ed32a72738dbadc90c91336`
- production: `crates/conceptweave-relation-partition/src/exclusion.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_partition_definition_exclusion_contract.rs`

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: `CompareIndexInfo()` in `src/backend/catalog/index.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: `RelationGetExclusionInfo()` in `src/backend/utils/cache/relcache.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/utils/cache/relcache.c

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 documentation: Table partitioning — limitations.* https://www.postgresql.org/docs/18/ddl-partitioning.html
