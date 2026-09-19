# Changelog

The Source Observation decision surface through the first cross-input direction/function lineage repair (`71fd68c93805d856171653e63c575075a7f5e16c`) is preserved at `docs/archive/CHANGELOG-through-71fd68c9.md`; its matching product/technical surface is preserved at `docs/archive/product-technical-gap-baseline-through-71fd68c9.md`. Earlier history remains under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`.

## Unreleased

### Added

- Retained the ordinary-forward converter-function successor chain, including raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, converter-function `pg_proc.protrftypes`, converter-function extension membership, and transform-object extension membership without rewriting predecessor digest domains or source history.
- Added hostile transform-object lineage contracts for same-generation converter-direction drift, converter-function identity drift, and same-name/same-direction raw converter-definition root drift.
- Extended `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-extension-membership-lineage-integrity.md` to distinguish reconstructed binding equality from immutable raw-root ancestry.

### Correctness

- PostgreSQL transform-object membership remains independent of converter-function membership. PostgreSQL 18 exposes `TRANSFORM FOR type LANGUAGE language` as its own `ALTER EXTENSION ... ADD/DROP` member object, so the `pg_transform` row can be attached to or detached from an extension while its type/language identity and converter functions remain unchanged.
- The successor still records one membership fact per exact same-generation `(constraint, key_position, transform_type, target_language)` transform row rather than duplicating the fact once per FROM SQL / TO SQL converter direction.
- The first cross-input repair requires the complete nonzero raw FROM SQL / TO SQL converter count to match the converter-function membership predecessor and binds each raw converter by exact constraint coordinate, key position, transform type, direction, converter schema and converter function name.
- Fresh review found that equal generation metadata plus equal direction/function identities still did not prove ancestry: a separately supplied raw transform-converter snapshot could preserve those identifiers while differing in converter-definition material.
- The converter-function successor chain now carries the immutable raw transform-converter root as `converter_snapshot_digest` instead of substituting derived successor digests. Transform-object extension-membership construction requires that root to equal the separately supplied raw transform-converter `snapshot_digest()` before direction/function checks run.
- Root mismatch fails closed with `index_exclusion_constraint_operator_procedure_transform_extension_membership_lineage`; the existing exact direction/cardinality/schema/function checks remain as binding validation after ancestry is established.
- Exact standalone state remains distinct from one resolved `pg_extension.extname`. Different extension names produce different successor digests. Missing/extra or duplicate transform coordinates, source-generation mismatch, converter ancestry or binding drift, transform/language drift, blank language or extension identifiers, zero positions, unknown receipts and quoted identifier provenance collisions fail closed.
- The extractor must resolve membership from the exact `pg_transform` object through its same-generation `pg_depend.deptype='e'` edge to `pg_extension`. Converter-function membership, naming conventions, installed packages and application metadata are not proxies.
- Extension-owned truth such as `extversion`, extension configuration, control files and update scripts remains with the extension owner and is not copied into ConceptWeave.

### Test and repair evidence

- Original transform-object extension-membership lineage: finding review `5252885919` -> structural RED `708761cc9ef5a14ea1201aa4c5d785463229e40d` -> production `c055f0a351e0aec4bec6174327a9cd0cf8a9a4b0` -> public composition `0d543dc5cdc2bfe23a7ee11e961bc7510730c0bd` -> focused primary-source traceability `526bb4cf0ad990ba34996139d5ce08055a400b91` -> pre-repair decision head `fcaa330965915a85ea7116a66c04286b79ef5931`.
- Cross-input direction/function lineage: finding review `5253304917` -> hostile direction-set RED `3f3a947e31c3b80e1a66120b89705ec3e515e43c` -> minimum production binding repair `37742d774d7c33a693dc3a0359a6583134bc5cbf` -> same-generation converter-function identity edge contract `276821d5d5fdafbd4776c9856acfc04221e470c1` -> focused doctoring `71fd68c93805d856171653e63c575075a7f5e16c`.
- Stronger immutable-root lineage: finding review `5253398088` -> behavioral RED `8ab7c431cdba2d0ba43a66313253b93b53b9e72a` -> ordinary-forward root-digest propagation across converter-function successors -> exact source repair `7ee7847b629e4f2e5ec5a12f93f7da06f23f366c` -> source/contract coherence COMMENT review `5253528602`.
- Documentation drift at `7ee7847b...` was recorded in review `5253827592`; its branch-name assertion was immediately corrected without review dismissal by review `5253828240`. Only the verified missing root-lineage traceability is carried into this documentation repair.
- No native or hosted GREEN is claimed after source or documentation movement. Rust 1.98 fmt, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, owned production statement/branch/edge coverage, and the PostgreSQL 18 bounded live differential remain acceptance gates on the final exact head.

### Retained

- All valid ordinary-EXCLUDE predecessor authority remains in force: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, exact one-`pg_catalog.internal` input, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, exact return type, implementation language and implementation-material digest, converter-function extension membership, and independent transform-object extension membership.
- #45 and #6 remain source-stable and must adopt #46 only after complete child acceptance. Partial cherry-pick or independent reimplementation is not a successor.

### Canonical-owner coordination

- Central `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d` remains the last documented workflow prerequisite against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until an ending fresh sweep proves otherwise; its shared scheduler still requires ordinary/non-force path-wise reconciliation plus the stronger repository-identity invariant.
- Queue-health #2268 and CodeQL dispatch lanes remain central-owner concerns. Current live central coordination must be read from their PRs; sibling central evidence does not transfer into #2040 or #46.
- Product bootstrap #35 remains a separate prerequisite and previously observed successful security lanes do not override its terminal CodeQL failure evidence.