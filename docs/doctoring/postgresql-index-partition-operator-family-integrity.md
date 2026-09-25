# PostgreSQL 18 index-partition operator-family integrity

## Decision

ConceptWeave must not use operator-class-name equality as a substitute for PostgreSQL index-partition attachment equivalence. PostgreSQL 18 `CompareIndexInfo()` compares the resolved operator-family OID at every key position after mapped attribute and collation checks. The frozen v3 observation stores the resolved `pg_opclass` coordinate but not the `pg_opfamily` identity, so the missing evidence belongs in a domain-separated successor rather than in a silent v3 digest mutation.

Review `5200439852` on exact #46 head `7c65090c2a90ba11dff575fde15fde37838795f6` records the P1 representation gap. Source contract `89394d73f5a468fcfde958f181a5bf8ba059e14f` requires a direct parent/child index edge with different operator families to fail, preserves different operator classes that resolve to the same family, and requires complete family evidence bound to the exact observed operator class. At the predecessor head the requested successor API did not exist, so this is a representation-level source RED; no executed compiler RED is claimed because the available host has no repository-pinned Rust toolchain.

Production commit `9e0d28324719b59bb6dc03e24b651e86a98250c3` adds a versioned `IndexOperatorFamilySnapshot`. It resolves every bounded key position through an `IndexKeyOperatorFamilyObservation`, repeats the exact v3 operator-class coordinate so construction can verify the `pg_opclass` join, retains the operator family as access-method/schema/name rather than an OID, requires complete key evidence, rebound-validates the exact base → relation-partition → index-partition predecessor stack, compares corresponding direct parent/child families, issues source receipts, and hashes the predecessor index-partition digest plus canonical family evidence under a new domain separator. The frozen v3 and index-partition digest meanings do not change.

A static module-resolution review immediately after that commit found that the new submodule path would be searched below the `index_partition` module directory. Commit `a6e4a4a55ccf6c6b983bdbb95a309e130ed62dc2` corrects the explicit `#[path]` binding. The original 707-line index-partition implementation is preserved byte-for-byte as `index_partition_base.rs`; the small `index_partition.rs` wrapper only composes the unchanged predecessor module with the new successor. This split is mechanical and does not change the predecessor digest or its public re-exported types.

## Why operator class is not operator family

PostgreSQL's built-in catalog data demonstrates the distinction. For B-tree text semantics, `text_ops` and `varchar_ops` are different operator classes but both resolve to the `btree/text_ops` operator family. Conversely, `text_pattern_ops` resolves to the distinct `btree/text_pattern_ops` family. A class-name equality rule would therefore reject at least one family-equivalent case that PostgreSQL's `CompareIndexInfo()` is designed to accept, while omitting family identity would accept a family mismatch PostgreSQL rejects.

The behavioral fixture mirrors that catalog fact: parent `text_ops` versus child `text_pattern_ops` carries different family observations and must fail `index_partition_definition_operator_family`; parent `text_ops` versus child `varchar_ops` carries the same `btree/pg_catalog/text_ops` family and remains admissible. The test also rejects incomplete family evidence and a family row whose repeated operator-class coordinate contradicts v3.

## Alternatives rejected

- Mutating `IndexKeySemantics` in frozen v3 was rejected because it would silently change an already-domain-separated predecessor identity and invalidate existing digest/receipt meaning.
- Comparing `QualifiedOperatorClassName` directly was rejected because PostgreSQL compares operator family, not class name.
- Comparing `pg_get_indexdef` text was rejected because rendered DDL is presentation evidence, not the catalog semantic identity used by `CompareIndexInfo()`.
- Retaining catalog OIDs as governed identity was rejected because OIDs are capture-time join coordinates. The successor resolves exact access-method/schema/family names before governance.

## Remaining definition-equivalence boundary

This repair closes operator-family representation and direct-edge comparison only. Full PostgreSQL 18 `CompareIndexInfo()` parity is not claimed. Canonical expression-tree equality under the partition attribute map, partial-index predicate equality, and exclusion-constraint operator/procedure/strategy semantics remain open. Those must be modeled as semantic evidence; raw expression/predicate text or reconstructed index DDL is not an acceptable shortcut.

Exact-head native and hosted acceptance is also still pending. Until one unchanged #46 head executes repository-pinned Rust 1.98 formatting, strict all-target Clippy, focused and retained tests, rustdoc, release build, coverage, and applicable hosted Product/security/dependency/review checks, this chronology is source repair evidence rather than GREEN acceptance.

## Traceability

- owner PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5200439852` on `7c65090c2a90ba11dff575fde15fde37838795f6`
- source RED contract: `89394d73f5a468fcfde958f181a5bf8ba059e14f`
- production successor: `9e0d28324719b59bb6dc03e24b651e86a98250c3`
- module-path correction: `a6e4a4a55ccf6c6b983bdbb95a309e130ed62dc2`
- production: `crates/conceptweave-relation-partition/src/operator_family.rs`
- predecessor composition: `crates/conceptweave-relation-partition/src/index_partition.rs`, `index_partition_base.rs`
- contract: `crates/conceptweave-relation-partition/tests/index_partition_definition_operator_family_contract.rs`

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: `CompareIndexInfo()` in `src/backend/catalog/index.c` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/catalog/index.c

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 built-in operator classes: `src/include/catalog/pg_opclass.dat` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_opclass.dat

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 built-in operator families: `src/include/catalog/pg_opfamily.dat` (REL_18_STABLE).* https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_opfamily.dat
