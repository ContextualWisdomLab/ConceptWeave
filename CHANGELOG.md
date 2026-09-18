# Changelog

The Source Observation surface through converter argument-name head `3dc7c81c8432f370b54aeac8375482d88b99a0cc` is preserved at `docs/archive/CHANGELOG-through-3dc7c81c.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-3dc7c81c.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the ordinary-forward raw `pg_proc.proargmodes` and `pg_proc.proargnames` successors without rewriting their digest domains or source history.
- Added raw same-generation converter-function `pg_proc.protrftypes` evidence as `IndexExclusionConstraintOperatorProcedureTransformConverterTransformTypesObservation`/receipt/snapshot over the argument-name predecessor.
- Added focused transform-selection contract coverage and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-function-transform-types-integrity.md`.

### Correctness

- A converter function's own `TRANSFORM FOR TYPE` selection is no longer conflated with its implementation language/body or with the `pg_transform` row that binds the converter. PostgreSQL 18 stores that selection independently in nullable `pg_proc.protrftypes`, and function calls can use different language-specific conversions when the selection changes.
- NULL remains distinct from a nonempty selected-type set. Empty sets and duplicate selected types fail closed; nonempty sets are sorted canonically only after duplicate detection.
- Every converter-function transform-selection observation exactly covers the predecessor `(constraint, key_position, transform_type, direction)` inventory and preserves converter schema/function binding. Missing/extra coordinates, duplicate coordinates, binding drift, blank identifiers, zero positions, unknown receipts and quoted transform-type provenance collisions fail closed.
- Selected converter-function transform types are resolved as qualified types from the same source generation. They are not reconstructed from return type, function body, implementation language, target-function `protrftypes`, or application metadata.
- This successor does not recursively copy the selected types' `pg_transform` rows; mutable converter rows remain with the already-established transform binding surface.
- The earlier observation/admission boundary remains authoritative: raw post-creation `provolatile='v'` and other mutable function state remain observable source facts even when a fresh transform would reject that state.

### Test and repair evidence

- Converter-function transform-selection finding review `5252233297` on exact head `3dc7c81c8432f370b54aeac8375482d88b99a0cc`.
- RED contract `20be74cacfce08778d00cb767bf05379eeba6626`.
- Production observation/snapshot/receipt `3e97370dc221738b0d408ed2a19be32f01ebff5c`.
- Public composition `303677ee2e1807aae883f0b752b897dbf2c23f01`.
- Focused contracts cover NULL/nonempty digest distinction, deterministic set order, empty/duplicate selected sets, predecessor completeness, duplicate/extra coordinates, exact converter binding, blank identifiers, zero positions, exact receipts and collision-safe quoted transform-type provenance.
- No native or hosted GREEN is claimed after source movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, exact one-`pg_catalog.internal` input, raw nullable `proargmodes`, raw nullable `proargnames`, exact return type, implementation language and implementation-material digest.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance. Partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d` remains the workflow prerequisite against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`; its shared scheduler still requires ordinary/non-force path-wise reconciliation plus the stronger repository-identity invariant.
- Queue-health #2268's SAST failure was traced to shared dynamic-urllib sinks; canonical sink repair is #2272 rather than a duplicate #2268 change. #2272's separate Agent Review Runtime Quality failure is tracked by #2277 and must not be hidden by unchanged-head reruns.
- Product bootstrap #35 remains a separate prerequisite and its terminal CodeQL failure still blocks normal landing.