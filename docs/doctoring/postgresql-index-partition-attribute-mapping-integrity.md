# PostgreSQL index-partition attribute-mapping integrity

Status: source RED committed; production repair pending exact-head implementation and acceptance.

## Problem

ConceptWeave's index-partition successor currently validates direct parent topology plus uniqueness, `NULLS NOT DISTINCT`, and access method before admitting an attached child index. That is necessary but not sufficient for PostgreSQL 18 attachment semantics.

`REL_18_STABLE` `CompareIndexInfo()` also requires the two indexes to have the same total number of index attributes and the same number of key attributes. It then compares each index slot through the partition attribute map because parent and child attribute numbers can differ. A simple-column slot must map to the same logical partition column. Column/expression slot shape must also agree before PostgreSQL compares expression trees. For key positions it additionally compares collation and operator family.

The current v3 representation already preserves ordered key versus `INCLUDE` roles and exact simple-column names. Therefore the currently modeled subset is sufficient to reject two source-impossible states without inventing new evidence:

- a parent key on one partition column attached to a child key on a different partition column;
- a parent/child pair with different key versus `INCLUDE` cardinality/role layout.

The representation is not yet sufficient for full PostgreSQL parity. In particular, it records operator class, while `CompareIndexInfo()` compares operator family; operator-class equality must not be used as a substitute. Expression-tree, predicate, and exclusion semantic equality also remain separate representation work. Raw `pg_get_indexdef` text is reconstructed display evidence and is not a semantic-equality shortcut.

## Traceability

- Canonical owner PR: ConceptWeave #46.
- Reviewed predecessor: `10b3dbb8ed58ecea065fbe5fab8bb1aa1039f2bf`.
- Review finding: `5199853671`.
- Behavioral source RED: `3a8fbb9b023d260ecda658f6685d6ccac88574e2`.
- RED contract: `crates/conceptweave-relation-partition/tests/index_partition_definition_attribute_mapping_contract.rs`.
- Production seam: `crates/conceptweave-relation-partition/src/index_partition.rs::canonicalize_index_partitions`.

## Required repair

At the exact parent/child definition resolution seam, fail closed before digest construction when:

1. key counts differ;
2. total key+`INCLUDE` counts differ;
3. corresponding modeled slots differ between simple-column and expression shape;
4. corresponding simple-column slots do not identify the same partition column.

Expression-vs-expression slots may remain admitted only under the explicit broader definition-equivalence gap until canonical expression-tree evidence exists; this repair must not claim full `CompareIndexInfo()` parity.

## Acceptance

The committed test is executable source evidence, not an executed RED claim in this run. No GREEN, Ready transition, merge, publication, or release is authorized until a minimal causal production repair is present and one unchanged exact head passes the repository-pinned Rust 1.98 and hosted acceptance gates.

## Primary source

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `src/backend/catalog/index.c`, `CompareIndexInfo()`* (`REL_18_STABLE`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c
