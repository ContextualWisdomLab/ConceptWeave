# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. Earlier surfaces remain under `docs/archive/`; focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

Fresh live state showed #46 had advanced ordinary-forward from prior documented head `95e5fb8e19d336b43c6dde3fa7767542b22505c6` to `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`. That concurrent delta is adopted, not treated as a race: it added raw transform-converter `pg_proc.proargmodes` evidence with RED `6ecc6418fce88b3aaf1519e7c422b2a1583b5407`, production `39602ea78c4bbff856c6274052918c4c0541e9df`, public composition `a3c61e33fbd84c86e9167d17377072df3e0e6ec0`, and doctoring `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312`.

The next material lossiness found in fresh PostgreSQL 18 catalog review was raw `pg_proc.proargnames`. PostgreSQL defines it independently of `proargtypes`: positions correspond to `proallargtypes`, unnamed positions are empty strings, and the whole field is NULL when none of the arguments are named. `CREATE OR REPLACE FUNCTION` can add a name to an input parameter that previously had none without replacing the function object, so two live converter states can differ only in `proargnames` while retaining the same transform binding and function identity.

Argument-name lineage: finding review `5252092881` on `8132dab8ca9d9f7aca02ac6a84af3c0c1257e312` -> RED `2ccf80dd19f7710d14512f2b184d8551142ad4d5` -> production observation/snapshot/receipt `653aa476578cb3e75bf19103c37ffec4e17e9a5e` -> public composition `bbcf388dfd01323da86b25df300ba112be4dc2cb` -> doctoring `6a4c022c8a8eeb1e9b963ed931bac64fc15aac1d`.

`IndexExclusionConstraintOperatorProcedureTransformConverterArgumentNamesSnapshot` is an ordinary-forward successor of the argument-mode snapshot. It preserves NULL separately from a populated name vector, preserves per-position empty-string sentinels, requires exact predecessor coordinate coverage and converter schema/function binding, and domain-separates the successor digest. A populated vector must have the same positional length as the predecessor all-argument shape: one when `proargmodes` is NULL, otherwise the exact mode-vector length. Empty or all-empty populated vectors fail closed because PostgreSQL represents the all-unnamed state as NULL.

Quoted transform-type provenance remains collision-safe because schema and type-name components are percent-encoded independently before the canonical separator.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains: converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, raw `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, raw `pronargs=1`, the exact one-`pg_catalog.internal` input boundary, raw nullable `proargmodes`, and now raw nullable `proargnames`.

## Observation versus admission

The raw catalog layer does not normalize away post-creation drift. PostgreSQL 18 `CREATE TRANSFORM` has structural and volatility admission constraints, but function properties can later change without replacing the transform binding. The established volatility correction therefore remains in force: raw `provolatile='v'` is observable drift evidence even though a volatile converter is not admissible for a fresh transform.

Argument names follow the same boundary principle. `CREATE OR REPLACE FUNCTION` may add a name to a previously unnamed input parameter while preserving the function identity. Source Observation records the exact resulting `proargnames`; it does not infer names from signatures and does not classify naming drift as a fresh-transform admission failure.

Primary authority:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: 52.39. pg_proc*. https://www.postgresql.org/docs/18/catalog-pg-proc.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE FUNCTION*. https://www.postgresql.org/docs/18/sql-createfunction.html
- Existing transform-checker/volatility authority remains documented in the prior focused doctoring surfaces.

## Acceptance boundary

**Source repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must resolve every selected transform converter to the same exact-generation `pg_proc` row and independently capture all predecessor facts plus raw nullable `proargmodes` and raw nullable `proargnames`. It must preserve empty-string unnamed positions and must not reconstruct names from SQL text, signatures, application metadata, or a different catalog generation. Missing resolution, mixed-generation joins, normalization-derived substitutes, coordinate drift, or name/mode positional mismatch remain capture failures.

At the current exact head, no pull-request workflow run has yet established native or hosted GREEN after these source movements. A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

The immediate argument-mode/name catalog lossiness is now represented in source. This is not a claim that `pg_proc` or PostgreSQL transform semantics are exhausted. In particular, `proallargtypes`, `pronargdefaults`/`proargdefaults`, return-type and implementation-language/source facts remain candidates only if a fresh catalog/code review proves that their omission creates an independent governed-semantic distinction not already guaranteed by the transform binding and predecessor chain.

The next successor must therefore come from another fresh source/catalog review rather than field enumeration by habit.

## Canonical prerequisite state

Central `.github#2040` remains the canonical workflow prerequisite until a fresh sweep proves otherwise. Its next valid movement remains ordinary/non-force path-wise reconciliation of the shared scheduler while preserving its owned v2 producer, no-restamp behavior, repository-scoped credential proof, stale-run revalidation, rationale/tests, compatible current-main queue/coalescing/capacity behavior, and the stronger repository-identity invariant.

Queue-health and CodeQL scan-dispatch sibling lanes retain independent evidence; their results do not transfer into #2040. Product bootstrap #35 remains a separate prerequisite and its prior successful Security/SAST lanes do not override terminal CodeQL failure evidence.

## Required order

`.github#2040` path-wise reconciliation plus repository-identity repair -> fresh exact-head central GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> exact #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 differential including raw `proargmodes` and `proargnames` -> fresh semantic-gap review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.