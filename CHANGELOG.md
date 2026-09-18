# Changelog

The preceding active Source Observation surface through converter-volatility head `f7593e834660efa56e3331c99bcf15d908cd46bc` is preserved losslessly at `docs/archive/CHANGELOG-through-f7593e83.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-f7593e83.md`. Earlier history remains under `docs/archive/`. This active changelog records the current transform-converter parallel-safety delta.

## Unreleased

### Added

- Added exact PostgreSQL ordinary-`EXCLUDE` transform-converter parallel-safety evidence for raw same-row `pg_proc.proparallel`.
- Added `IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetyObservation`, immutable source receipt, and `IndexExclusionConstraintOperatorProcedureTransformConverterParallelSafetySnapshot` as a successor over converter volatility evidence.
- Added focused doctoring/TRACEABILITY at `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-parallel-safety-integrity.md`, grounded in PostgreSQL 18 `pg_proc`, `CREATE FUNCTION`, parallel-safety, and function-optimization documentation.

### Correctness

- Converter definition, owner, `EXECUTE` ACL, nullable `proconfig`, security-definer context, leakproof classification, strictness, and volatility no longer imply equal parallel-safety state. Raw `proparallel` changes produce a distinct successor digest.
- Parallel-safety observations must exactly cover predecessor converter directions and preserve transform type plus converter schema/function binding. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers, invalid catalog discriminators, and unknown receipts fail closed.
- Only PostgreSQL catalog states `s`, `r`, and `u` are accepted. All three remain representable; ConceptWeave does not infer a `PARALLEL SAFE`-only transform policy.
- Parallel-safety provenance preserves the quoted-qualified-type collision repair by percent-encoding transform schema and type-name components independently before the canonical separator.

### Test and repair evidence

- Review `5250289810` identifies missing raw converter `proparallel` at prior exact head `f7593e834660efa56e3331c99bcf15d908cd46bc`.
- RED `0fc0f567386d39b848444879a41c4877e9f996b6` adds the executable parallel-safety contract before public parallel-safety types exist.
- Production `66846c34d668202a6f8fbbdfb3658fba908aec0d` adds raw parallel-safety observation/snapshot/receipt behavior.
- Public-composition successor `6373c77652f63dae874e2b4bdb881b54f5680bfe` exports the new successor through `index_partition.rs`.
- The focused contract covers raw-state preservation, `s/r/u` digest separation, exact direction completeness, extra/duplicate coordinates, function-binding drift, invalid states, blank identifiers, zero positions, exact receipt lookup, and quoted-type provenance collision safety.
- No native/hosted GREEN is claimed. Exact `6373c776...` had no pull-request workflow runs at the first inventory query, so repository-pinned Rust 1.98 fmt/Clippy/tests/docs/release/rustdoc/owned coverage and PostgreSQL 18 bounded live differential remain open.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force, including converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, raw `proisstrict`, raw `provolatile`, and collision-safe quoted-identifier provenance.
- Converter planner support and cost remain independent review-gated follow-up facts; neither is inferred from parallel safety.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance; partial cherry-pick/reimplementation is not an acceptable successor.

### Canonical-owner coordination

- Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise.
- #2040 requires ordinary/non-force path-wise protected-main reconciliation of the shared scheduler while preserving v2 CodeQL producer semantics, no-source-neutral-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation and rationale/tests; compatible main queue/coalescing/capacity behavior and the stronger repository-identity invariant must be adopted explicitly.
- Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa`; CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their exact-head acceptance evidence is independent and does not transfer to #2040.
- Source-fix #2175 remains `6e5f75dd40a428d174f691e35e210d08a29eb270` and cannot self-modify `.github/` or `scripts/ci/`.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` with terminal CodeQL failure despite Security/SAST success; no unchanged-head manual rerun is authorized.
