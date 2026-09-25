# Changelog

The Source Observation surface through converter-function extension-membership head `a2ebefecb185f1c1ba0006fc9f118f5e25c4a9cc` is preserved at `docs/archive/CHANGELOG-through-a2ebefec.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-a2ebefec.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the ordinary-forward converter-function successor chain, including raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, converter-function `pg_proc.protrftypes`, and converter-function extension membership, without rewriting predecessor digest domains or source history.
- Added transform-object extension-membership evidence as `IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipObservation`/receipt/snapshot.
- Added focused transform-object extension-membership contract coverage and `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-extension-membership-integrity.md`.

### Correctness

- PostgreSQL transform-object membership is no longer conflated with converter-function membership. PostgreSQL 18 exposes `TRANSFORM FOR type LANGUAGE language` as its own `ALTER EXTENSION ... ADD/DROP` member object, so the `pg_transform` row can be attached to or detached from an extension while its type/language identity and converter functions remain unchanged.
- The successor records one membership fact per exact same-generation `(constraint, key_position, transform_type, target_language)` transform row, rather than duplicating the fact once per FROM SQL / TO SQL converter direction.
- Exact standalone state remains distinct from one resolved `pg_extension.extname`. Different extension names produce different successor digests. Missing/extra or duplicate transform coordinates, source-generation mismatch, transform/language drift, blank language or extension identifiers, zero positions, unknown receipts and quoted identifier provenance collisions fail closed.
- The extractor must resolve membership from the exact `pg_transform` object through its same-generation `pg_depend.deptype='e'` edge to `pg_extension`. Converter-function membership, naming conventions, installed packages and application metadata are not proxies.
- Extension-owned truth such as `extversion`, extension configuration, control files and update scripts remains with the extension owner and is not copied into ConceptWeave.

### Test and repair evidence

- Transform-object extension-membership finding review `5252885919` on exact head `a2ebefecb185f1c1ba0006fc9f118f5e25c4a9cc`.
- Structural RED contract `708761cc9ef5a14ea1201aa4c5d785463229e40d`.
- Production observation/snapshot/receipt `c055f0a351e0aec4bec6174327a9cd0cf8a9a4b0`.
- Public composition `0d543dc5cdc2bfe23a7ee11e961bc7510730c0bd`.
- Focused primary-source traceability `526bb4cf0ad990ba34996139d5ce08055a400b91`.
- No native or hosted GREEN is claimed after source movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, exact one-`pg_catalog.internal` input, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, exact return type, implementation language and implementation-material digest, and converter-function extension membership.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance. Partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d` remains the workflow prerequisite against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise; its shared scheduler still requires ordinary/non-force path-wise reconciliation plus the stronger repository-identity invariant.
- Queue-health #2268's SAST failure was traced to shared dynamic-urllib sinks; canonical sink repair is #2272 rather than a duplicate #2268 change. #2272's separate Agent Review Runtime Quality failure is tracked by #2277 and must not be hidden by unchanged-head reruns.
- Product bootstrap #35 remains a separate prerequisite and its terminal CodeQL failure still blocks normal landing.
