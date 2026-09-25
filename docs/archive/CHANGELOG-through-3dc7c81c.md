# Changelog

The pre-argument-mode Source Observation surface through exact head `95e5fb8e19d336b43c6dde3fa7767542b22505c6` is preserved at `docs/archive/CHANGELOG-through-95e5fb8e.md`. Earlier decision surfaces remain under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Adopted the ordinary-forward converter argument-mode successor: RED `6ecc6418fce88b3aaf1519e7c422b2a1583b5407`, production `39602ea78c4bbff856c6274052918c4c0541e9df`, public composition `a3c61e33fbd84c86e9167d17377072df3e0e6ec0`, doctoring head `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`.
- Added raw same-generation `pg_proc.proargnames` evidence as `IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesObservation`/receipt/snapshot over the argument-mode predecessor.
- Added focused argument-name contract coverage and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-argument-names-integrity.md`.

### Correctness

- `pg_proc.proargmodes` is preserved as NULL versus explicit `IN`/`OUT`/`INOUT` positional evidence instead of being reconstructed from the normalized converter signature. Impossible transform-converter mode shapes fail closed while Source Observation remains separate from fresh `CREATE TRANSFORM` policy.
- `pg_proc.proargnames` is preserved as NULL versus an exact positional text vector. Empty strings inside a populated vector remain unnamed-position sentinels; names are never trimmed, case-folded, or inferred from SQL text or application metadata.
- A populated argument-name vector must match the predecessor all-argument positional shape: one position when `proargmodes` is NULL, otherwise exactly the `proargmodes` vector length. Empty or all-empty populated vectors fail closed because PostgreSQL represents the all-unnamed state as NULL.
- Converter schema/function binding, direction completeness, duplicate/extra coordinate rejection, one-based key position, exact receipts, domain-separated digests, and collision-safe component-wise percent-encoded transform-type provenance remain enforced.
- PostgreSQL 18 `CREATE OR REPLACE FUNCTION` can add a name to an input parameter that previously had none without replacing the function object. Argument-name changes are therefore live drift/source facts and are not collapsed into a fresh-transform admission rule.
- The earlier volatility lifecycle correction remains authoritative: raw `provolatile='v'` can be observed after transform creation and must not be erased by Source Observation.

### Test and repair evidence

- Argument-name finding review `5252092881` on exact head `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`.
- Argument-name RED contract `2ccf80dd19f7710d14512f2b184d8551142ad4d5`.
- Argument-name production observation/snapshot/receipt `653aa476578cb3e75bf19103c37ffec4e17e9a5e`.
- Public composition `bbcf388dfd01323da86b25df300ba112be4dc2cb`.
- Focused contracts cover NULL/named/partially unnamed vectors, digest distinction, impossible all-unnamed populated vectors, vector-length drift, predecessor completeness, duplicate/extra coordinates, binding drift, blank converter identifiers, zero positions, exact receipts, and quoted-type provenance collision safety.
- No native or hosted GREEN is claimed after source movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, raw `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, the exact one-`pg_catalog.internal` input boundary, and collision-safe quoted-identifier provenance.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance. Partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` remains the prerequisite until a fresh sweep proves otherwise. Its shared scheduler requires ordinary/non-force protected-main reconciliation plus the stronger repository-identity invariant; hosted evidence from sibling central lanes does not transfer.
- Product bootstrap #35 remains a separate prerequisite and must not be landed on stale or failed central CodeQL evidence.