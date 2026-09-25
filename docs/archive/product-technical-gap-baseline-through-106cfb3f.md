# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before converter-function auto-extension dependency evidence is preserved at `docs/archive/product-technical-gap-baseline-through-fb6b8e6e.md`; its matching changelog is `docs/archive/CHANGELOG-through-fb6b8e6e.md`. Earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains the previously established exact definition/owner/ACL/config/security/planner/cost/shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, and converter-function extension membership.

Fresh PostgreSQL 18 review found a separate lifecycle/recovery fact that survived those bindings. `pg_depend.deptype='e'` is extension membership, whereas `deptype='x'` is an auto-extension dependency. An `x` dependent routine is not an extension member: it remains independently droppable, but `DROP EXTENSION` removes it and dump/restore treatment differs from extension-owned membership. `ALTER FUNCTION ... DEPENDS ON EXTENSION` / `NO DEPENDS ON EXTENSION` can mutate this state without changing the exact converter schema/name or `pg_transform` binding, and one function may depend on multiple extensions.

Finding review `5254049432` pinned the lossiness at the pre-finding head. Structural RED `3347a4c8fe2dab338c138cb4acd0ca0d7fc0e498` references the new public auto-extension API before that API exists. Production `f52c34dbd385b8d2e50b54a67b125c1a35a777fa` adds `IndexExclusionConstraintOperatorProcedureTransformConverterAutoExtensionDependencyObservation`, immutable receipt, and snapshot. Public composition follows at `55b0baf3bbb34d0ef24534d29c12a3814fa97176`.

Each converter direction now carries the complete canonical sorted set of exact `pg_extension.extname` values reached through same-generation `pg_depend.deptype='x'` edges. Zero, one, and multiple dependencies are distinct states. Blank or duplicate names fail closed; ordering does not affect identity because row order is not semantic. Complete converter coordinates and exact converter schema/function binding remain mandatory, and the immutable `converter_snapshot_digest` propagates unchanged from the preceding membership chain.

The transform-object extension-membership contract was ordinary-forward restacked at `458bd76f193906999a81c583da30dff152d0f9f8`, and production at `fcb0e2cdd8700ed56ee4d2a0b96bdf89c5bd4532` now takes the converter auto-extension dependency snapshot as its digest predecessor. It still receives the separately supplied raw transform-converter snapshot and first proves equal source generation plus exact immutable raw converter root. Only then does it validate complete FROM SQL / TO SQL cardinality and exact converter schema/function identity. Consequently a change in converter `deptype='x'` state changes the later transform-object successor digest instead of disappearing from the final chain.

Fresh source review after that restack found the retained transform-object direction/function/raw-root lineage contract still constructed the removed membership-only predecessor helper. Ordinary-forward `60d90c3ed0b0793378707a39cd27397dba6ed3bb` restacks those hostile regressions on `converter_function_auto_extension_dependency_snapshot()` so the existing direction drift, function-name drift, and same-name/different-definition raw-root cases exercise the current production boundary rather than a stale predecessor shape. Focused doctoring was currentized at `98417ed8d92863fe3fddf97560b87bc37001a676`.

Converter-function `deptype='e'`, converter-function `deptype='x'`, and transform-object `deptype='e'` remain three different facts. ConceptWeave does not infer one from another and does not copy extension-owned `extversion`, configuration, control/update scripts, package inventory, or application metadata.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: ALTER FUNCTION*. https://www.postgresql.org/docs/18/sql-alterfunction.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: DROP EXTENSION*. https://www.postgresql.org/docs/18/sql-dropextension.html
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html

## Observation versus governance

Source Observation records exact external database state before policy. Validation or publication may later reject an unexpected extension membership or auto-extension dependency, but it must reason over the observed state rather than rewrite it. Raw post-creation `provolatile='v'` follows the same boundary: observation preserves drift even when fresh transform admission would reject it.

## Acceptance boundary

**Source and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new zero/one/multiple auto-extension dependency contract, retained immutable-root and direction/function regressions on the new predecessor shape, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently resolve `deptype='e'` membership and the complete `deptype='x'` dependency set from `pg_depend`, resolve extension OIDs to exact `pg_extension.extname`, preserve existing function facts and raw converter-root identity, and separately resolve the transform object's own `deptype='e'` membership. Reconstruction from naming conventions, installed packages, extension membership, another source generation, or application metadata is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

This successor was admitted because it has independently documented DROP/pg_dump/recovery semantics; it is not a continuation of catalog-field enumeration. After this repair, no further converter/transform successor is authorized merely because another catalog field or dependency code exists.

The next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction that survives the existing raw-root, function-binding, membership, auto-extension, and transform-object lifecycle facts. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their exact heads, mergeability, reviews, required checks and protected-main relationship must be read fresh before landing. Sibling central evidence does not transfer into #2040, #35 or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including converter `deptype='e'`, complete converter `deptype='x'` sets, immutable raw converter-root lineage, and transform-object `deptype='e'` -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.