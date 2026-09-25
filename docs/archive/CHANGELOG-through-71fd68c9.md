# Changelog

The Source Observation surface through transform-object extension-membership head `fcaa330965915a85ea7116a66c04286b79ef5931` is preserved at `docs/archive/CHANGELOG-through-fcaa3309.md`; its matching product/technical decision surface is preserved at `docs/archive/product-technical-gap-baseline-through-fcaa3309.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the ordinary-forward converter-function successor chain, including raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, converter-function `pg_proc.protrftypes`, converter-function extension membership, and transform-object extension membership without rewriting predecessor digest domains or source history.
- Added hostile transform-object lineage contracts for same-generation converter-direction drift and converter-function identity drift.
- Added `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-extension-membership-lineage-integrity.md` alongside the existing transform-object extension-membership primary-source traceability.

### Correctness

- PostgreSQL transform-object membership remains independent of converter-function membership. PostgreSQL 18 exposes `TRANSFORM FOR type LANGUAGE language` as its own `ALTER EXTENSION ... ADD/DROP` member object, so the `pg_transform` row can be attached to or detached from an extension while its type/language identity and converter functions remain unchanged.
- The successor still records one membership fact per exact same-generation `(constraint, key_position, transform_type, target_language)` transform row rather than duplicating the fact once per FROM SQL / TO SQL converter direction.
- Fixed a cross-input lineage hole in `IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot::new`: equal source-generation metadata and a shared `(constraint, key_position, transform_type)` were previously sufficient even when the converter-function extension-membership predecessor and the separately supplied raw `pg_transform` snapshot described different converter direction/function sets.
- The constructor now requires the complete nonzero raw FROM SQL / TO SQL converter count to match the converter-function membership predecessor and binds each raw converter by exact constraint coordinate, key position, transform type, direction, converter schema and converter function name before issuing transform-object membership evidence.
- Exact standalone state remains distinct from one resolved `pg_extension.extname`. Different extension names produce different successor digests. Missing/extra or duplicate transform coordinates, source-generation mismatch, converter lineage drift, transform/language drift, blank language or extension identifiers, zero positions, unknown receipts and quoted identifier provenance collisions fail closed.
- The extractor must resolve membership from the exact `pg_transform` object through its same-generation `pg_depend.deptype='e'` edge to `pg_extension`. Converter-function membership, naming conventions, installed packages and application metadata are not proxies.
- Extension-owned truth such as `extversion`, extension configuration, control files and update scripts remains with the extension owner and is not copied into ConceptWeave.

### Test and repair evidence

- Original transform-object extension-membership lineage: finding review `5252885919` -> structural RED `708761cc9ef5a14ea1201aa4c5d785463229e40d` -> production `c055f0a351e0aec4bec6174327a9cd0cf8a9a4b0` -> public composition `0d543dc5cdc2bfe23a7ee11e961bc7510730c0bd` -> focused primary-source traceability `526bb4cf0ad990ba34996139d5ce08055a400b91` -> pre-repair decision head `fcaa330965915a85ea7116a66c04286b79ef5931`.
- Cross-input lineage finding review `5253304917` on exact `fcaa330965915a85ea7116a66c04286b79ef5931`.
- Hostile direction-set structural RED `3f3a947e31c3b80e1a66120b89705ec3e515e43c`.
- Minimum production binding repair `37742d774d7c33a693dc3a0359a6583134bc5cbf`.
- Additional same-generation converter-function identity edge contract `276821d5d5fdafbd4776c9856acfc04221e470c1`.
- Focused lineage doctoring `71fd68c93805d856171653e63c575075a7f5e16c`.
- No native or hosted GREEN is claimed after source movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, exact one-`pg_catalog.internal` input, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, exact return type, implementation language and implementation-material digest, converter-function extension membership, and independent transform-object extension membership.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance. Partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d` remains the workflow prerequisite against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until an ending fresh sweep proves otherwise; its shared scheduler still requires ordinary/non-force path-wise reconciliation plus the stronger repository-identity invariant.
- Queue-health #2268 and CodeQL dispatch lanes remain central-owner concerns. Current live central coordination must be read from their PRs; the previously documented standalone `#2277` issue reference could not be verified in the fresh owner inventory and is not treated as authority.
- Product bootstrap #35 remains a separate prerequisite and its terminal CodeQL failure still blocks normal landing.
