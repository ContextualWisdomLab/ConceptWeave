# Changelog

All notable ConceptWeave changes through exact source head `7ebb14bb37e7a86c88446a4fad729ee4cdd24607` are preserved losslessly at `docs/archive/CHANGELOG-through-7ebb14bb.md`; the matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-7ebb14bb.md`. Earlier history remains under `docs/archive/`. This active changelog records only the current Source Observation delta.

## Unreleased

### Added

- Added exact PostgreSQL ordinary-`EXCLUDE` transform-converter strictness evidence for raw same-row `pg_proc.proisstrict`.
- Added `IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessObservation`, immutable source receipt, and `IndexExclusionConstraintOperatorProcedureTransformConverterStrictnessSnapshot` as a successor over converter leakproof evidence.
- Added focused doctoring/TRACEABILITY at `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-strictness-integrity.md` using PostgreSQL 18 `CREATE FUNCTION` and `CREATE TRANSFORM` as primary authority.

### Correctness

- Converter definition, owner, `EXECUTE` ACL, nullable `proconfig`, security-definer context and leakproof classification no longer imply equal strictness identity. Raw `proisstrict` changes produce a distinct successor digest.
- Strictness observations must exactly cover predecessor converter directions and preserve transform type plus converter schema/function binding. Missing or extra coordinates, duplicates, binding drift, zero positions, blank converter identifiers and unknown receipts fail closed.
- Strictness provenance keeps the quoted-qualified-type collision repair: transform schema and type-name components are percent-encoded independently before the canonical separator.
- Strict and non-strict converter states are both representable. ConceptWeave does not infer a strict-only CREATE TRANSFORM policy from PostgreSQL examples or converter signatures.

### Test evidence

- RED `06cd1ea330d439ac87553ac59010306f1df173dc` pins the missing converter-strictness public contract.
- Production successor `a5234386f1d8fa67740b2d1454574e30f824894d` adds the strictness source module, public composition, and doctoring evidence without changing predecessor digest domains.
- Test-only successor `ab8d72e98dcc4a3c1270ce265fe6cbf18e3a3d6e` adds explicit extra-coordinate and blank converter schema/function-name edge cases.
- Reviews `5249125942` and `5249147930` record the source and edge-coverage boundaries.
- No native/hosted GREEN is claimed: exact `ab8d72e...` has no pull-request Actions run inventory, so Rust 1.98 fmt/Clippy/tests/docs/release/rustdoc/owned coverage and PostgreSQL 18 bounded differential remain open.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force, including exact converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, raw `proleakproof`, and collision-safe quoted-identifier provenance.
- Converter volatility, parallel safety, planner support and cost remain review-gated independent follow-up facts.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance; partial cherry-pick/reimplementation is not an acceptable successor.

### Canonical-owner coordination

- Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`.
- #2040 requires ordinary/non-force path-wise protected-main reconciliation of the shared scheduler while preserving v2 CodeQL producer semantics, no-source-neutral-restamp behavior, repository-scoped Actions credential proof, stale-run revalidation and rationale/tests; compatible main queue/coalescing/capacity behavior and the stronger repository-identity invariant must be adopted explicitly.
- Queue-health #2268 remains `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 remains `2b849c874122961e025c29f7fa0bb697863c3d68`; both carry source repairs while exact-head security workflows remain queued.
- Source-fix #2175 remains `6e5f75dd40a428d174f691e35e210d08a29eb270` and cannot self-modify `.github/` or `scripts/ci/`.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05` with terminal CodeQL failure despite Security/SAST success; no unchanged-head manual rerun is authorized.
