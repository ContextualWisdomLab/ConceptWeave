# Changelog

The preceding active Source Observation surface through strictness head `7ffc577908dd0e87ed950a3bc8c5c2b952788e04` is preserved losslessly at `docs/archive/CHANGELOG-through-7ffc5779.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-7ffc5779.md`. Earlier history remains under `docs/archive/`. This active changelog records the current transform-converter volatility delta.

## Unreleased

### Added

- Added exact PostgreSQL ordinary-`EXCLUDE` transform-converter volatility evidence for raw same-row `pg_proc.provolatile`.
- Added `IndexExclusionConstraintOperatorProcedureTransformConverterVolatilityObservation`, immutable source receipt, and `IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot` as a successor over converter strictness evidence.
- Added focused doctoring/TRACEABILITY at `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-volatility-integrity.md`, grounded in PostgreSQL 18 `CREATE FUNCTION`, `CREATE TRANSFORM`, and function-optimization documentation.

### Correctness

- Converter definition, owner, `EXECUTE` ACL, nullable `proconfig`, security-definer context, leakproof classification and strictness no longer imply equal volatility state. Raw `provolatile` changes produce a distinct successor digest.
- Volatility observations must exactly cover predecessor converter directions and preserve transform type plus converter schema/function binding. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, invalid catalog discriminators and unknown receipts fail closed.
- Only PostgreSQL catalog states `i`, `s`, and `v` are accepted. All three remain representable; ConceptWeave does not infer an `IMMUTABLE`-only transform policy from PostgreSQL examples.
- Volatility provenance preserves the quoted-qualified-type collision repair by percent-encoding transform schema and type-name components independently before the canonical separator.

### Test and repair evidence

- Review `5249791627` identifies missing raw converter `provolatile` at prior exact head `7ffc577908dd0e87ed950a3bc8c5c2b952788e04`.
- RED `2bf4001dc1cee62df3d826fe585e246528c450cb` adds the executable volatility contract before public volatility types exist.
- Production `85265a29730698c6847a1089357a0a18e70c77d8` adds raw volatility observation/snapshot/receipt behavior.
- Public-composition successor `d875c26cdd47c008015e8dfd0d769fc16ba8b5d0` exports the new successor through `index_partition.rs`.
- The focused contract covers raw-state preservation, `i/s/v` digest separation, exact direction completeness, extra/duplicate coordinates, function-binding drift, invalid states, blank identifiers, zero positions, exact receipt lookup and quoted-type provenance collision safety.
- No native/hosted GREEN is claimed. The first exact-head check on `d875c26...` returned no pull-request workflow runs, so repository-pinned Rust 1.98 fmt/Clippy/tests/docs/release/rustdoc/owned coverage and PostgreSQL 18 bounded live differential remain open.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force, including converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, and collision-safe quoted-identifier provenance.
- Converter parallel safety, planner support and cost remain independent review-gated follow-up facts; none is inferred from volatility.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance; partial cherry-pick/reimplementation is not an acceptable successor.

### Canonical-owner coordination

- Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`.
- #2040 requires ordinary/non-force path-wise protected-main reconciliation of the shared scheduler while preserving v2 CodeQL producer semantics, no-source-neutral-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation and rationale/tests; compatible main queue/coalescing/capacity behavior and the stronger repository-identity invariant must be adopted explicitly.
- Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa`; CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their exact-head acceptance evidence is independent and does not transfer to #2040.
- Source-fix #2175 remains `6e5f75dd40a428d174f691e35e210d08a29eb270` and cannot self-modify `.github/` or `scripts/ci/`.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` with terminal CodeQL failure despite Security/SAST success; no unchanged-head manual rerun is authorized.
