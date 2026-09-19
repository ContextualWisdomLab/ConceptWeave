# PostgreSQL index-partition collation catalog identity integrity

Status: source-repaired / execution acceptance pending

## Problem

The issued v3 representation stores an index key collation as `QualifiedCollationName(schema_name, collation_name)`. The historical `IndexPartitionSnapshot` therefore compares attached parent/child key collations by that two-part value.

That is not a complete PostgreSQL 18 catalog-row identity. `pg_collation` declares `pg_collation_name_enc_nsp_index` UNIQUE over `(collname, collencoding, collnamespace)`. Two distinct collation rows can therefore share namespace and name while differing in `collencoding`. PostgreSQL attachment does not compare rendered names: `CompareIndexInfo()` receives the resolved per-key collation OID arrays and requires corresponding OIDs to match.

Consequently, the predecessor could admit two distinct collation OIDs as equivalent when both resolved to the same `schema/name` pair. Rewriting `QualifiedCollationName` or the issued v3/index-partition digest would invalidate immutable predecessor meaning, so the repair must be a new evidence family.

## Decision

Review `5203297768` on exact predecessor `100c1ad75da145f32829ba30735dcfddf249b166` records the P1.

Behavioral compile RED `db761588b9e3a79da4cd6f2839b756ce06992601` introduces a case where parent and child both retain qualified collation `pg_catalog.C`, so the historical index-partition predecessor accepts them, but their resolved catalog identities carry different raw encodings (`-1` versus `6`). The successor must reject that pair.

Production `d1d2423e2f20c328f459bf912f0e7c4e2c4d44ae` adds:

- `CollationCatalogIdentity`, preserving exact schema, name, and raw signed `pg_collation.collencoding` while excluding database-local OID from governed identity;
- `IndexKeyCollationIdentityObservation`, one explicit observation per bounded key-semantic position, including explicit no-collation;
- `IndexPartitionCollationIdentitySnapshot`, which rebound-validates the exact `IndexPartitionSnapshot`, requires complete one-per-key evidence, binds schema/name back to the issued v3 key semantics, compares the complete resolved catalog identity on every direct attached parent/child edge, frames a domain-separated digest, and issues exact receipts.

Export wiring `255f414327d95a680c61e96c80d553f8e07e1137` leaves all earlier modules and digest families unchanged. Focused boundary/receipt coverage `0279cb46e486e8a3c74cb446915ad9c95bc53a61` covers invalid coordinates, binding mismatch, completeness, different-encoding rejection, matching-identity admission, and provenance receipts.

## Constraint and scope

This successor repairs the per-key `pg_index.indcollation` identity seam used by attached-index definition equivalence. It does not retroactively redefine historical `QualifiedCollationName` identity and does not yet prove catalog-exact identity for every collation-bearing expression-node field, relation-`Var`, or column-collation family. Those occurrences must either consume the same catalog-identity concept in a later domain-separated composition or remain fail-closed before an authoritative whole-expression attachment claim.

Concrete PostgreSQL transport is still sequenced after representation acceptance. The future adapter must resolve each observed nonzero collation OID to the exact `pg_collation` row from the same source snapshot and emit its raw `collencoding`; live differential tests must include the same-name/different-encoding alias case where the server permits such rows in the bounded database/encoding context.

## Evidence and traceability

- ConceptWeave predecessor: `crates/conceptweave-observation/src/representation_v3.rs` (`QualifiedCollationName`, `IndexKeySemantics`).
- Historical comparison: `crates/conceptweave-relation-partition/src/index_partition_base.rs` (`validate_attached_index_collations`).
- Successor: `crates/conceptweave-relation-partition/src/collation_identity.rs`.
- Contract: `crates/conceptweave-relation-partition/tests/index_partition_collation_catalog_identity_contract.rs`.
- PostgreSQL 18 source: `src/include/catalog/pg_collation.h` declares `pg_collation_name_enc_nsp_index` over `collname`, `collencoding`, `collnamespace`.
- PostgreSQL 18 source: `src/backend/catalog/index.c`, `CompareIndexInfo()`, compares the supplied resolved per-key collation OIDs after attribute mapping.

## References

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: pg_collation catalog definition* (`REL_18_STABLE`, `src/include/catalog/pg_collation.h`). PostgreSQL Global Development Group.

PostgreSQL Global Development Group. (2025). *PostgreSQL 18 source: index catalog and attachment comparison* (`REL_18_STABLE`, `src/backend/catalog/index.c`). PostgreSQL Global Development Group.
