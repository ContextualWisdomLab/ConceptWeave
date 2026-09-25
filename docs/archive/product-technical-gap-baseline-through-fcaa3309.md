# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-function extension-membership surface is preserved at `docs/archive/product-technical-gap-baseline-through-a2ebefec.md`; earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The ordinary-forward converter chain retains all previously established converter definition/owner/ACL/config/security/planner/cost/shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, and converter-function extension membership.

Fresh PostgreSQL 18 review proved that the **`pg_transform` object itself** has a separate extension lifecycle. `pg_transform` has its own row identifier and stores the exact transform type, language, and optional FROM SQL / TO SQL converter OIDs. Independently, PostgreSQL 18 accepts `ALTER EXTENSION name ADD TRANSFORM FOR type LANGUAGE lang` and the matching `DROP` form. ADD makes the existing transform an extension member that can subsequently be dropped only through the extension; DROP disassociates the transform without dropping it. This state can therefore differ while the transform row and both converter functions remain otherwise identical.

Transform-object extension-membership lineage: finding review `5252885919` on `a2ebefecb185f1c1ba0006fc9f118f5e25c4a9cc` -> structural RED `708761cc9ef5a14ea1201aa4c5d785463229e40d` -> production observation/snapshot/receipt `c055f0a351e0aec4bec6174327a9cd0cf8a9a4b0` -> public composition `0d543dc5cdc2bfe23a7ee11e961bc7510730c0bd` -> focused doctoring `526bb4cf0ad990ba34996139d5ce08055a400b91`.

`IndexExclusionConstraintOperatorProcedureTransformExtensionMembershipSnapshot` is digest-layered over the latest converter-function extension-membership snapshot and cross-checks the same-generation transform-converter snapshot. It preserves one fact per exact `(constraint, key_position, transform_type, target_language)` `pg_transform` row rather than copying the same object-level fact into each converter direction. Exact standalone state remains distinct from one resolved `pg_extension.extname`.

The extractor must bind the exact `pg_transform` OID already resolved in the same source generation, inspect `pg_depend` for an extension-membership edge with `deptype='e'` and `objsubid=0`, and resolve the referenced extension OID to exact `pg_extension.extname` in that same snapshot. Zero edges means standalone. An unsupported ambiguous multiple-membership state must fail closed before contract construction.

Converter-function extension membership is not a proxy. Naming conventions, installed extension lists, function namespaces, application metadata or another source generation are not substitutes. The successor also does not copy `pg_extension.extversion`, extension configuration, control-file metadata, update scripts or other extension-owned truth.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.57. pg_transform*. https://www.postgresql.org/docs/18/catalog-pg-transform.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: ALTER EXTENSION*. https://www.postgresql.org/docs/18/sql-alterextension.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: 52.18. pg_depend*. https://www.postgresql.org/docs/18/catalog-pg-depend.html
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: 52.22. pg_extension*. https://www.postgresql.org/docs/18/catalog-pg-extension.html
- PostgreSQL Global Development Group. (2026e). *PostgreSQL 18 documentation: CREATE TRANSFORM*. https://www.postgresql.org/docs/18/sql-createtransform.html

## Observation versus governance

Source Observation records the exact external database state before policy. It does not infer transform or converter-function extension membership. Validation/publish policy may later reject an unexpected member/standalone state, but it must reason over the observed edge instead of rewriting source facts.

The same rule continues to apply to mutable function state. Raw post-creation `provolatile='v'` remains observable drift evidence even though a volatile function is not admissible when creating a fresh transform. Observation and admission remain separate boundaries.

## Acceptance boundary

**Source repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation: preserve the existing converter facts, raw nullable `proargmodes`, raw nullable `proargnames`, raw nullable converter-function `protrftypes`, converter-function extension membership, and the transform object's own `pg_depend -> pg_extension` membership edge. Reconstruction from naming conventions, packages, application metadata, converter-function membership or another source generation is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The fresh `pg_transform` review no longer leaves an obvious uncaptured row field: the catalog exposes its OID, transform type, language, FROM SQL function and TO SQL function, all of which are already used by the transform-converter source contract. The independent extension-membership lifecycle edge is now represented separately. Normal dependency edges to the bound type, language and converter functions are consequences of those already-observed bindings rather than new domain truth.

No further converter/transform successor should be added merely by enumerating catalogs. A new successor now requires proof of an independent buyer, semantic, security, lifecycle or recovery distinction that survives the existing binding and membership facts. Until such a distinction is proven, this sub-chain should stop growing and move to exact-head acceptance plus the same-generation PostgreSQL 18 differential.

## Canonical prerequisite state

Central `.github#2040` remains a separate canonical workflow prerequisite. Its last verified authority before this source movement was `12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9`. It requires ordinary/non-force path-wise reconciliation of the shared scheduler while preserving its v2 producer, no-restamp behavior, repository-scoped credential proof, stale-run revalidation, rationale/tests, compatible current-main queue/coalescing/capacity behavior, and stronger repository-identity invariant. Ending fresh sweeps, not this paragraph, determine whether those coordinates remain current.

Queue-health #2268's SAST failure was traced to shared dynamic-urllib sinks; canonical sink repair is #2272, not a duplicate queue-health patch. #2272's separate Agent Review Runtime Quality failure is tracked by owner issue #2277. Sibling central evidence does not transfer into #2040 or #46.

Product bootstrap #35 remains a separate prerequisite; its previously observed successful security lanes do not override terminal CodeQL failure evidence.

## Required order

fresh central prerequisite verification -> `.github#2040` causal reconciliation/repair if still current -> fresh central exact-head GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> exact #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including converter-function and transform-object extension membership -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
