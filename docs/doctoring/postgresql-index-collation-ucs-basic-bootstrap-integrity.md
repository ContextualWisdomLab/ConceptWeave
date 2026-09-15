# PostgreSQL 18 `ucs_basic` bootstrap collation integrity

## Problem

`CollationDefinitionObservation::validate_provider_encoding()` previously modeled every built-in `C` collation as encoding-independent (`collencoding = -1`). That rule matches PostgreSQL 18's ordinary `builtin_locale_encoding("C")` creation path, but it rejects one canonical bootstrap catalog row: `pg_catalog.ucs_basic` is built-in provider `b`, locale `C`, UTF8 `collencoding = 6`, with stored collation version `1`.

Treating the ordinary creation rule as a universal catalog invariant made a legitimate PostgreSQL 18 source state unrepresentable. Broadly permitting arbitrary built-in `C/6` rows would be the opposite error because ordinary built-in `C` creation still resolves to `-1`.

## Authority and decision

PostgreSQL 18 `src/include/catalog/pg_collation.dat` is the catalog authority for the bootstrap exception. `src/backend/utils/adt/pg_locale.c::builtin_locale_encoding()` is the authority for ordinary built-in locale encoding derivation: `C -> -1`, `C.UTF-8 -> PG_UTF8`, and `PG_UNICODE_FAST -> PG_UTF8`.

The selected repair admits UTF8 built-in `C` only for the exact bootstrap coordinate `(pg_catalog, ucs_basic, 6)`. All other built-in `C` definitions continue to require `collencoding = -1`. This preserves raw catalog evidence instead of inferring or normalizing it.

Rejected alternatives:

- allow all built-in `C` definitions with either `-1` or `6`: too broad and would admit catalog states not produced by PostgreSQL 18's ordinary creation path;
- rewrite `ucs_basic` to encoding-independent evidence: destroys the exact source catalog coordinate and breaks governed evidence identity;
- special-case in transport only: leaves the canonical representation unable to accept the authoritative bootstrap row.

## Traceability

- PR: `ContextualWisdomLab/ConceptWeave#46`
- Review finding: `5207042221`
- Source/compile RED: `934f0d97af17ed8c804168d1be3ec4d307deef8b`
- Production repair: `5af83839a4be145f395664a2cac00f0f6743fa9c`
- Production seam: `crates/conceptweave-relation-partition/src/collation_definition.rs::validate_provider_encoding`
- Focused contract: `crates/conceptweave-relation-partition/tests/index_collation_provider_encoding_contract.rs::postgresql18_ucs_basic_bootstrap_collation_is_utf8_builtin_c`

The focused contract also retains an arbitrary `pg_catalog.builtin` + built-in `C` + UTF8 negative control, so the bootstrap exception cannot silently widen into a generic `C/6` rule.

## Risk and follow-up

This source repair has not received native Rust 1.98 or hosted Product acceptance. Concrete PostgreSQL transport must read raw `pg_collation.collencoding` and include a live differential for `pg_catalog.ucs_basic`, while retaining rejection of fabricated non-bootstrap built-in `C/6` observations. Any later extractor must not derive `collencoding` from locale text.
