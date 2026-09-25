# PostgreSQL 18 index-collation definition/version integrity

## Problem

The existing attached-index collation successors correctly distinguish resolved `pg_collation` catalog coordinates by `(namespace, name, collencoding)` and bind those coordinates to the source database encoding. That is sufficient to avoid qualified-name aliasing when reproducing PostgreSQL `CompareIndexInfo()` collation-OID equality inside one captured catalog, but it is not complete immutable source-content identity for the collation definition itself.

PostgreSQL 18 stores behaviorally material definition state outside that coordinate: `collprovider`, `collisdeterministic`, provider locale fields (`collcollate`/`collctype` or `colllocale`), ICU rules, and recorded `collversion`. PostgreSQL also exposes the current provider version through `pg_collation_actual_version(oid)`. An OS or ICU upgrade can therefore change the effective provider version while the catalog coordinate and stored `collversion` remain unchanged. PostgreSQL explicitly warns that changed collation definitions can make indexes and other stored objects inconsistent with the new sort order and recommends rebuilding affected objects before refreshing the recorded version.

This is a governed-evidence problem rather than an attached-index topology problem. The issued coordinate and database-encoding predecessor digests remain valid for what they claim and are not rewritten.

## Authoritative PostgreSQL 18 evidence

PostgreSQL 18 `CREATE COLLATION` defines locale, provider, deterministic behavior, ICU rules, and version as definition inputs. Nondeterministic collations are supported only by ICU. The documentation also notes that same-name catalog entries may exist for different encodings.

PostgreSQL 18 `ALTER COLLATION ... REFRESH VERSION` documents that the provider-specific version is recorded when the collation is created, compared with the current provider version when used, and may mismatch after operating-system or ICU changes. It states that definition changes can lead to corrupt indexes and that affected objects should be rebuilt before refreshing the recorded version.

The PostgreSQL 18 `pg_collation` catalog defines provider (`d`, `b`, `c`, `i`), deterministic flag, `collencoding`, libc locale fields, provider locale, ICU rules, and `collversion`. Its catalog uniqueness coordinate is `(collname, collencoding, collnamespace)`; that uniqueness key is not a complete representation of the row's behaviorally material definition.

Primary references:

- PostgreSQL Global Development Group. (2026). *CREATE COLLATION (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-createcollation.html
- PostgreSQL Global Development Group. (2026). *ALTER COLLATION (PostgreSQL 18)*. https://www.postgresql.org/docs/18/sql-altercollation.html
- PostgreSQL Global Development Group. (2026). *pg_collation (PostgreSQL 18 system catalog)*. https://www.postgresql.org/docs/18/catalog-pg-collation.html
- PostgreSQL Global Development Group. (2026). *Collation support (PostgreSQL 18)*. https://www.postgresql.org/docs/18/collation.html

## Repair lineage

Review `5204669655` on exact predecessor `291b05c3d2faa0c4121e51cc630fc4f02ba2cd0c` identified the catalog-coordinate-versus-definition gap.

Source RED `542f0543e120ee733ec18136aa83ae4899db9721` requires two observations with the same catalog coordinate but different recorded provider versions or provider semantics to remain distinct. Initial production `a7c53bee4a44ebbc1ef50ea5e1632d3dd7b99ac6` introduced a domain-separated material definition successor, and export `a97d76547850b95f14ddb3cd853baaee36820381` wired it into the relation-partition public contract.

Fresh review `5204679813` found a second P1 in that first repair: stored `collversion` alone cannot detect a provider upgrade before `ALTER COLLATION ... REFRESH VERSION`. Source RED `7fb49690fc4a473917b1d038777cc05d443b1aa4` therefore requires the same coordinate and stored version with a different capture-time actual provider version to produce distinct governed definition identity. Production `0a732e2ab011fa8b0e9ab04c3a1bc9da34e1be99` adds `actual_version` evidence, removes an unused import that would conflict with warning-denied acceptance, preserves stored-versus-actual mismatch instead of normalizing it, and includes both values in item and snapshot digests. Focused contract `7a587443ed2e9b8547b67d6673bed60c8fc71846` checks the mismatch surface explicitly.

## Chosen contract

`CollationDefinitionObservation` preserves:

- the exact predecessor `CollationCatalogIdentity` (`namespace`, `name`, raw `collencoding`);
- PostgreSQL 18 provider token as `PostgresCollationProvider` (`d`, `b`, `c`, `i`);
- raw `collisdeterministic`;
- raw nullable `collcollate`, `collctype`, `colllocale`, and `collicurules` values without provider-default normalization;
- recorded nullable `collversion`;
- capture-time nullable `pg_collation_actual_version(oid)` separately from the recorded version.

`IndexCollationDefinitionSnapshot` consumes the exact `IndexCollationDatabaseEncodingSnapshot`, `IndexPartitionCollationIdentitySnapshot`, and `IndexExpressionCollationIdentitySnapshot`. It rebound-validates the database-encoding predecessor, requires source/policy/extractor/time agreement, derives every distinct non-null collation coordinate actually used by key, expression/predicate, and relation-`Var` evidence, and requires exactly one definition for each coordinate. Missing, duplicate, or unrelated definitions fail closed. The successor digest is domain-separated from all issued predecessor digests.

A stored-versus-actual version mismatch remains observable through `has_version_mismatch()`. Observation does not rewrite the catalog or silently claim that affected indexes were rebuilt. Publication/release policy must treat unresolved mismatch evidence as a validation condition rather than laundering it into a refreshed version.

## Rejected alternatives

Rewriting `CollationCatalogIdentity` was rejected because it would change the meaning of an issued predecessor that remains correct as a catalog-coordinate proof for attached-index equality.

Using only `collversion` was rejected because provider state can change before the recorded version is refreshed.

Using only `pg_collation_actual_version(oid)` was rejected because the stored/current mismatch is itself operationally significant evidence.

Using OID as immutable identity was rejected because OIDs are database-local capture coordinates, not portable governed identity.

Normalizing provider-specific fields into a single synthetic locale string was rejected because it would collapse catalog distinctions and make later PostgreSQL-major changes harder to version explicitly.

Including `collowner` in the semantic-definition digest was rejected for this successor because ownership is authorization metadata rather than ordering/comparison behavior. It remains source-governance evidence elsewhere if required; it is not a substitute for the semantic fields above.

## Remaining boundary

This is still representation source evidence, not executed Rust or live PostgreSQL transport evidence. The concrete PostgreSQL 18 extractor must resolve each used collation OID in the same bounded source observation, capture the full row fields plus `pg_collation_actual_version(oid)`, preserve stored/current version mismatch, and run live differential cases across provider/version changes. Exact-head Rust 1.98 and hosted Product acceptance remain prerequisites before transport work is promoted.
