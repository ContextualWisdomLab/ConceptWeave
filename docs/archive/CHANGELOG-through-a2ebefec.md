# Changelog

The Source Observation surface through converter-function transform-selection head `e2e257d1c498a906dc844060d1e3b8cf2303967a` is preserved at `docs/archive/CHANGELOG-through-e2e257d1.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-e2e257d1.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the ordinary-forward raw `pg_proc.proargmodes`, `pg_proc.proargnames`, and converter-function `pg_proc.protrftypes` successors without rewriting their digest domains or source history.
- Added converter-function extension-membership evidence as `IndexExclusionConstraintOperatorProcedureTransformConverterExtensionMembershipObservation`/receipt/snapshot over the transform-selection predecessor.
- Added focused extension-membership contract coverage and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-extension-membership-integrity.md`.

### Correctness

- A converter function's PostgreSQL extension membership is no longer conflated with function identity, implementation, ACL/config/security/planner state, argument metadata, function transform selection, or the `pg_transform` binding. The successor preserves exact absence or the same-generation `pg_extension.extname` reached through the converter function's `pg_depend.deptype='e'` edge.
- Extension membership is a governed lifecycle/operability distinction: PostgreSQL permits `ALTER EXTENSION ... ADD/DROP FUNCTION` on an existing function; extension members can only be dropped through their owning extension and are treated as extension-owned objects by `pg_dump`.
- Every membership observation exactly covers the predecessor `(constraint, key_position, transform_type, direction)` inventory and preserves converter schema/function binding. Missing/extra coordinates, duplicate coordinates, binding drift, blank identifiers, blank extension names, zero positions, unknown receipts and quoted transform-type provenance collisions fail closed.
- The successor does not copy `pg_extension` version/configuration/control/update-script truth. It records only the exact membership edge. The transform object's own extension-membership edge is independent and remains a separate semantic review rather than being inferred from converter-function membership.
- The earlier observation/admission boundary remains authoritative: raw catalog/lifecycle drift is observed first; validation or publication policy may reason over it later without rewriting source facts.

### Test and repair evidence

- Converter-function extension-membership finding review `5252657912` on exact head `e2e257d1c498a906dc844060d1e3b8cf2303967a`.
- RED contract `ddc04f03c97ae78cccddefe34bd656aec32bd688`.
- Production observation/snapshot/receipt `30976b643aa96203a203705a75100e6374ff4977`.
- Public composition `64080cdc8188ca951f73faa95d6f9f656e6860dc`.
- Focused primary-source traceability `441eadaa35d2fa8606a9c8a1bd9763cd517d2451`.
- No native or hosted GREEN is claimed after source movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, exact one-`pg_catalog.internal` input, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, exact return type, implementation language and implementation-material digest.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance. Partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d` remains the workflow prerequisite against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise; its shared scheduler still requires ordinary/non-force path-wise reconciliation plus the stronger repository-identity invariant.
- Queue-health #2268's SAST failure was traced to shared dynamic-urllib sinks; canonical sink repair is #2272 rather than a duplicate #2268 change. #2272's separate Agent Review Runtime Quality failure is tracked by #2277 and must not be hidden by unchanged-head reruns.
- Product bootstrap #35 remains a separate prerequisite and its terminal CodeQL failure still blocks normal landing.