# Changelog

The Source Observation decision surface before dangling-role repair in converter-function initial privileges is preserved at `docs/archive/CHANGELOG-through-5daf2a57.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-5daf2a57.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the converter-function `pg_init_privs` successor: exact row absence/presence, `privtype` (`i`/`e`), complete object-level initial EXECUTE ACL, immutable receipt/snapshot, exact converter binding, and raw converter-root lineage remain authoritative.
- Added hostile dangling-role coverage for PostgreSQL catalog states where an initial ACL still refers to a role OID after the role no longer exists. The contract covers dangling grantee, dangling grantor, both dangling, PUBLIC with dangling grantor, and rejection of unresolved OID zero.
- Added `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-dangling-role-integrity.md` with PostgreSQL 18 BUG #19483/#19513 and pgsql-hackers repair discussion traceability.

### Correctness

- `IndexExclusionConstraintOperatorProcedureTransformConverterInitialExecuteGrant` no longer requires every non-PUBLIC ACL role OID to resolve to a role name before Source Observation can exist. A nonzero dangling grantee or grantor OID is now preserved explicitly as an unresolved raw OID.
- Resolved role names and unresolved OIDs occupy different identity namespaces. A real role named `"16424"` does not alias raw dangling OID `16424`.
- PUBLIC remains a grantee-only identity. OID zero is rejected by unresolved-role constructors and is never treated as a dangling role.
- Existing resolved-role digest framing remains byte-for-byte stable. Unresolved grantees use a dedicated digest tag; unresolved grantors use a reserved framing sentinel plus the raw OID, avoiding collision with length-framed role names.
- Source Observation records the damaged external state rather than silently filtering it or failing solely on missing role lookup. Validation/publication may later flag the state as a recovery/security defect.
- Current `pg_proc.proacl`, converter-function `deptype='e'`, complete `deptype='x'` sets, security labels, exact `pg_init_privs` baseline (including dangling ACL OIDs), immutable raw converter root, and transform-object `deptype='e'` remain separate facts.

### Test and repair evidence

- Previous initial-privilege finding/repaired lineage remains archived through exact head `5daf2a5725ed52cd1b8d884200fc7786eaba54de`.
- Dangling-role finding review: `5254640920`.
- Structural RED: `94e98da8ebb31f1429b44b20cce7b96ea16e1332`; the contract referenced dangling-role constructors before production exposed them.
- Production causal repair: `1d6abe4f11cbc9c50b705ec5458a80ba04719c7f`.
- Pre-repair CHANGELOG archive: `0d91dd7d876e67c8460a9a3ed04d579ab9bf463e`.
- Pre-repair product/technical baseline archive: `b372444bd178c37100ffda070193cf9ef9bd444d`.
- Focused primary-source doctoring: `ff8498187781a65273297913ca2a280c7f157c3d`.
- Source repair itself is not native or hosted GREEN evidence. Exact-head Rust 1.98 fmt, strict workspace/all-target Clippy, focused/retained/workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and PostgreSQL 18 bounded live differential remain gates.

### Retained

- All previously valid ordinary-EXCLUDE authority remains in force, including exact converter definition/owner/current-ACL/config/security/planner/cost/shape facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function `deptype='e'` membership, complete converter-function `deptype='x'` dependency sets, security-label maps, initial-privilege baselines, immutable raw converter-root lineage, and independent transform-object extension membership.
- #45 and #6 remain source-stable and may adopt #46 only after complete child acceptance; partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their live exact heads, checks, reviews, workflow inventories, and protected-main relationships must be read fresh before landing; predecessor or sibling evidence does not transfer into #46.
