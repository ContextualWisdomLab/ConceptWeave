# PostgreSQL 18 copied `ucs_basic` collation integrity

**Status:** source repaired; exact-head execution acceptance pending  
**Owner:** ConceptWeave Source Observation / relation-partition semantics  
**PostgreSQL authority:** `REL_18_STABLE@6567f3f1f21353b5b0e9538d058998ab76c5a37c`

## Problem

The provider/catalog-encoding guard treated built-in locale `C` as `collencoding = -1` except for the one literal bootstrap identity `pg_catalog.ucs_basic`, whose PostgreSQL 18 bootstrap row is provider `b`, locale `C`, and UTF8 encoding ID `6`.

That rule was too narrow. PostgreSQL 18 `CREATE COLLATION ... FROM existing_collation` copies the source row's `collprovider`, `collisdeterministic`, `collencoding`, `collcollate`, `collctype`, `colllocale`, and `collicurules` before creating the new row. Copying `pg_catalog.ucs_basic` therefore produces another source-reachable built-in `C` collation with `collencoding = 6` under the caller's target identity. Only the database-default provider is explicitly forbidden from this copy path.

A governed observation that rejects such a copied row is not lossless PostgreSQL 18 evidence: a valid attached index using the copy could not enter the semantic snapshot.

## Constraints

- Preserve the raw catalog coordinate; do not normalize the copied collation back to `pg_catalog.ucs_basic`.
- Keep direct built-in `C` with `collencoding = -1` valid.
- Keep built-in `C.UTF-8` and `PG_UNICODE_FAST` fixed to UTF8 encoding ID `6`.
- Keep ICU fixed to `collencoding = -1` and provider `d` fixed to the bootstrap default row.
- Do not encode schema privilege assumptions into the semantic provider/encoding invariant. The invariant describes source-reachable row shape, not who may create the target namespace.
- Do not change issued digest domains.

## RED

Review `5209588784` identified the false-negative boundary on exact predecessor `733e7280330c48978b6b76746b9031ed83c821e1`.

Source/compile RED `980b95cab7a6baa9e2cdc2f86277a91963f5d23a` changes `index_collation_provider_encoding_contract.rs` so that:

- built-in `C/-1` remains valid;
- built-in `C/6` is valid because it is reachable by copying `pg_catalog.ucs_basic`;
- a concrete `public.ucs_basic_copy/6` built-in `C` observation is a positive witness;
- built-in `C` at an unrelated encoding such as LATIN1 (`8`) remains invalid;
- `C.UTF-8` and `PG_UNICODE_FAST` remain UTF8-only;
- ICU remains encoding-independent (`-1`).

The current execution host has no Rust toolchain, so this is source/compile RED evidence rather than an executed failing test result.

## Causal repair

Production `ee8acd0424bf67b07b4d6c0f4eee21889d59f67c` changes only the built-in `C` branch of `validate_provider_encoding()`:

- accepted catalog encodings are now `-1` and PostgreSQL 18 UTF8 ID `6`;
- the prior schema/name special case for only the bootstrap row is removed;
- all other provider/locale encoding rules remain unchanged.

This matches both PostgreSQL creation paths: direct built-in `C` creation derives `-1`, while `CREATE COLLATION ... FROM pg_catalog.ucs_basic` preserves the source row's `6`.

## Alternatives rejected

**Keep the literal `pg_catalog.ucs_basic` exception.** Rejected because it contradicts PostgreSQL's `FROM` copy semantics and rejects valid copied rows.

**Allow built-in `C` at every database encoding.** Rejected because PostgreSQL 18 has only two source-reachable catalog encodings for this built-in locale through ordinary supported creation paths: direct `C/-1` and copies descending from `ucs_basic/6`.

**Normalize copied rows to the bootstrap identity.** Rejected because namespace/name/encoding is a real catalog coordinate and index collation OID resolution must preserve the actual row used by the source database.

## Risk and effect

The repair widens one previously over-restricted source-shape invariant. It does not weaken completeness, database-encoding usability, provider shape, provider-version, or effective database-default checks. A copied `C/6` row still has to be a collation actually used by the bounded predecessor and must satisfy the source database-encoding binding; in practice encoding `6` is usable only in a UTF8 database.

The semantic effect is that two distinct catalog rows can legitimately share provider `b`, locale `C`, and `collencoding = 6`; their namespace/name coordinates remain distinct and are preserved in governed identity.

## Acceptance and live differential

Exact-head GREEN remains pending repository-pinned Rust 1.98 `fmt`, strict Clippy, focused/workspace/doc tests, release build, rustdoc/coverage, and hosted Product/security/review evidence.

The later PostgreSQL transport differential must include an actual `CREATE COLLATION public.ucs_basic_copy FROM pg_catalog.ucs_basic` case, read the copied row's raw `collencoding`, and prove that the extractor emits the copied catalog identity rather than normalizing it to the bootstrap source row.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- review finding: `5209588784`
- RED: `980b95cab7a6baa9e2cdc2f86277a91963f5d23a`
- production: `ee8acd0424bf67b07b4d6c0f4eee21889d59f67c`
- production seam: `crates/conceptweave-relation-partition/src/collation_definition.rs::validate_provider_encoding`
- focused contract: `crates/conceptweave-relation-partition/tests/index_collation_provider_encoding_contract.rs`
- upstream source: PostgreSQL `src/backend/commands/collationcmds.c::DefineCollation`
- bootstrap catalog: PostgreSQL `src/include/catalog/pg_collation.dat`

## References

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 source: `DefineCollation`* (REL_18_STABLE, commit `6567f3f1f21353b5b0e9538d058998ab76c5a37c`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/backend/commands/collationcmds.c

PostgreSQL Global Development Group. (2026). *PostgreSQL 18 bootstrap collation catalog* (REL_18_STABLE, commit `6567f3f1f21353b5b0e9538d058998ab76c5a37c`). https://github.com/postgres/postgres/blob/REL_18_STABLE/src/include/catalog/pg_collation.dat
