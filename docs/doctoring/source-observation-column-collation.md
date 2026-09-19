# Source Observation: column collation evidence

## Decision record

### Problem

The successor Source Observation model preserves qualified type identity for columns and already has qualified collation coordinates for domains and index-key semantics, but `ColumnObservationV3` does not preserve the column's `pg_attribute.attcollation`. This loses a PostgreSQL source fact that changes comparison semantics even when the qualified data type is unchanged.

The omission also prevents governed foreign-key validation. PostgreSQL requires each collatable referencing/referenced column pair to use collations that are either both deterministic or exactly the same. A source representation that cannot distinguish the two column collations, or cannot retain whether the resolved collation is deterministic, cannot prove that rule from immutable evidence.

### Source authority

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html

- `pg_attribute.attcollation` is the defined collation of a column and is zero for a non-collatable data type.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_collation*. https://www.postgresql.org/docs/18/catalog-pg-collation.html

- `pg_collation.collisdeterministic` records whether a collation is deterministic.
- Qualified collation name is sufficient source identity inside a database after the adapter resolves the catalog OID; the OID remains a capture-time join coordinate.

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html

- For every collatable foreign-key column pair, both collations must be deterministic or the two columns must use the same collation. This applies to foreign-key equality semantics, including PERIOD foreign keys.

### Constraints

1. Do not derive effective column collation from rendered `data_type`, a domain default, index `indcollation`, `search_path`, locale strings, or catalog OIDs.
2. Preserve the original v3 compatibility digest when the column-collation catalog family was not observed. New evidence therefore needs a domain-separated observed-family layer or an equivalent compatibility-safe successor framing.
3. Distinguish family-unobserved from explicitly observed `attcollation = 0` for an uncollatable column.
4. For a collatable column, retain the exact qualified collation coordinate and source-authoritative deterministic flag resolved from the same bounded catalog snapshot.
5. The same qualified collation observed more than once must not be allowed to carry contradictory deterministic flags inside one snapshot.
6. Foreign-key validation must compare exact local/referenced column coordinates, not index keys. A referencing column need not have an index.
7. Do not make collation evidence an excuse to infer foreign-key, PERIOD, type, domain, or index truth owned by other observed families.
8. When an FK references a relation outside the bounded relation inventory, this family must not invent the remote column collation. Cross-boundary FK collation adjudication requires a later immutable referenced-column evidence seam; until then only bounded local/reference pairs are validated by this family.

### Alternatives considered

**Reuse index `indcollation`.** Rejected. Index collation belongs to an index key, is optional because an FK does not require a referencing index, and can differ from the effective column collation through explicit index expressions/options.

**Reuse domain collation.** Rejected. A column can have its own defined/effective collation; domain metadata is not a lossless substitute for `pg_attribute.attcollation`.

**Store only collation name on `ColumnObservationV3`.** Rejected as incomplete for FK validation and risky for the existing v3 digest contract. Determinism comes from `pg_collation`, and the original constructor must continue to mean "collation family unobserved" rather than silently defaulting the new field.

**Store OIDs.** Rejected. OIDs are source-instance join coordinates and are not stable governed identity.

### Selected boundary

Use an explicit observed column-collation family rather than mutating frozen `ColumnObservationV3`. Each bounded column coordinate resolves to exactly one observed state when the family is present:

- explicitly uncollatable (`attcollation = 0`), or
- collatable with exact qualified collation identity plus the resolved `collisdeterministic` value.

The family is complete for the bounded relation-column inventory, has its own deterministic digest domain (`conceptweave.postgres_schema_snapshot.v3.column_collations.v1`), and preserves the legacy v3 digest when absent. Repeated observations of one exact qualified collation must agree on determinism. For an FK whose local and referenced relations are both in the bounded inventory, admission enforces PostgreSQL's rule directly on the paired column evidence: a collatable pair is valid when both resolved collations are deterministic, or when the exact qualified collation coordinates are equal. A pair with one or two nondeterministic collations and different coordinates fails closed. Out-of-inventory references are not assigned invented collation truth.

A later PostgreSQL adapter must resolve `attcollation -> pg_collation` inside one bounded read-only catalog snapshot before crossing the Source Observation ACL. It must not substitute type defaults or `search_path` lookup. A later cross-boundary reference-evidence contract, if required by the buyer path, must be immutable/versioned and owned explicitly rather than widening this family by hidden source access.

## RED / repair traceability

- Valid finding review: PR #46 review `5187855669`, predecessor exact head `fcb75659c4c7ffc046c4c7187789ef3b5d53715d`.
- Executable source-level RED: commit `8006b24fd4409bc092d80640084a64892190ea4a`, `crates/conceptweave-observation/tests/column_collation_contract.rs`.
- The RED requires exact collation materiality, observed-uncollatable versus family-unobserved distinction, rejection of differing nondeterministic FK collations, and positive controls for both-deterministic and exact-same nondeterministic pairs.
- Production repair lineage: `3e495eac9e345db40e3009fe7579e6422a46447a` introduced the isolated observed-family implementation; `664894780f2643c00b86165b0b75757ee1d607db` corrected borrowed determinism access before integration; `3d8a7fb7c8f00ebbc7f6994f2b83274a6f3d3bc0` integrated the public aggregate constructor/state/digest/accessor boundary without changing the legacy unobserved digest path.
- Edge-contract hardening: `8a4b7a1b59353289d3cc33f80d852db9116f7096` adds completeness, duplicate-coordinate, conflicting-determinism, permutation-invariance, retained-evidence, and one-sided-nondeterministic FK cases.
- The repair is source-complete but not execution-accepted. No native/Product GREEN is transferred from any predecessor or inferred from static review.

## Acceptance

The repaired exact head must pass the focused column-collation contract plus all retained Source Observation contracts, repository-pinned Rust 1.98 formatting/strict Clippy/workspace and doc tests/release/owned coverage, and applicable Product/security/dependency/review gates. Any head movement invalidates predecessor execution evidence. Until one unchanged head has that terminal evidence, #46 remains Draft and is `SOURCE_REPAIRED / ACCEPTANCE_PENDING`, not GREEN, publishable, or releasable.
