# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

This is the code-current decision surface for the active ConceptWeave Source Observation lane. The preceding converter-kind surface through exact head `09f4b9c9d1be4da2979c0a01958014a1aa129f42` is preserved losslessly at `docs/archive/product-technical-gap-baseline-through-09f4b9c9.md`; its matching changelog is preserved at `docs/archive/CHANGELOG-through-09f4b9c9.md`. Earlier surfaces remain under `docs/archive/`, and focused rationale/TRACEABILITY remains under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The current behavior-bearing structural converter successor remains argument-count public head `c7419477a8538551a573482f78013ad226923899`, built ordinary-forward over converter return-set public head `40fc0e46b8a9e8c1018245540fc788f47a9132c0` and converter-kind authority archived at `09f4b9c9...`. The later volatility lifecycle correction changes the interpretation of already-observed raw `provolatile`; it does not rewrite the structural successor digest chain.

Return-set lineage: review `5251492933` -> RED `e92a8657a20878621da883a4326237f1d899bc89` -> production `c6d11432cbac681a937449ea41e2c49e26f24409` -> public composition `40fc0e46b8a9e8c1018245540fc788f47a9132c0`.

Argument-count lineage: review `5251508944` -> RED `4f81f92f54edb622ba933617ef6a598d7a4a1d16` -> production `65dabb7df539c2a80455f8b4aec5c024117b8a58` -> final public composition `c7419477a8538551a573482f78013ad226923899`. An intermediate composition `cb43e7fc8ba28516dc96a6e3d61a8df9a71c9a24` contained one duplicate re-export outside the intended stanza and was immediately repaired ordinary-forward. The net `40fc0e46... -> c7419477...` compare is ahead-only and leaves only the new argument-count source, its contract, and four intended `index_partition.rs` additions.

All valid predecessor ordinary-EXCLUDE facts remain authoritative without rewriting issued digest domains. The active chain binds exact selected transform converters plus definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, raw `prosecdef`, `proleakproof`, `proisstrict`, raw `provolatile`, `proparallel`, exact `prosupport` absence/identity, exact raw `procost` float4 bits, raw `prokind='f'`, raw `proretset=false`, and raw `pronargs=1`.

Quoted PostgreSQL transform-type identifiers remain collision-safe: schema and type-name components are percent-encoded independently before the canonical `.` separator.

## Transform-converter checker semantics and observation boundary

PostgreSQL 18 `check_transform_function()` performs five checks after resolving a converter function: it rejects `PROVOLATILE_VOLATILE`, requires a normal function (`prokind == PROKIND_FUNCTION`), rejects set-returning functions (`proretset`), requires exactly one argument (`pronargs == 1`), and requires that argument type to be `INTERNALOID`. FROM SQL additionally must return `internal`; TO SQL must return the transform data type.

The four shape predicates are represented by the governed chain: `IndexExclusionConstraintOperatorProcedureTransformConverterKindSnapshot`, `...ReturnSetSnapshot`, `...ArgumentCountSnapshot`, and the base converter's resolved single `pg_catalog.internal` argument boundary. These are exact source facts, not values inferred from a generic function-shaped DTO.

Volatility needs a different lifecycle treatment. `check_transform_function()` rejects a volatile converter during `CREATE TRANSFORM`, but PostgreSQL 18 `ALTER FUNCTION` explicitly allows changing an existing function to `IMMUTABLE`, `STABLE`, or `VOLATILE`. The transform checker is not an ALTER-time revalidation hook. Consequently, an already referenced converter can be observed later with raw `provolatile='v'`, even though the same function would fail fresh transform admission.

ConceptWeave therefore preserves `i|s|v` in `IndexExclusionConstraintOperatorProcedureTransformConverterVolatilitySnapshot`. An observed `v` is governance-relevant drift evidence and changes the successor digest; it is not discarded at Source Observation. A later validation rule may mark `v` as non-admissible for a fresh transform or as a drift finding, but validation must consume the raw fact rather than make it unobservable.

This boundary was reverified in this run. Review `5251665634` initially interpreted the creation-time volatility check as a raw-capture restriction. RED `a4a735d2bda47b5cbad7cdf6df6405c76f499fc6` and production `9de4b4ae2ddaa1339116a03d15ca13e5f4b19cef` temporarily encoded that interpretation. PostgreSQL 18 `ALTER FUNCTION` primary-source follow-up invalidated it. Correction review `5251695984`, ordinary-forward source correction `b4df9bef3c5eeb892a2ac47e208d06f9bb0f5bb0`, and regression `7ebe2a484330d882a8aa857fdb53333f524e09eb` restore raw volatile-drift observability while retaining the superseded attempt in history.

Both structural successors continue to require exact predecessor inventory and converter schema/function binding. Missing/extra coordinates, duplicates, binding drift, blank identifiers, zero positions, invalid structural values, and unknown provenance receipts fail closed. Their digests are separately domain-separated, and their canonical locations retain component-wise percent encoding.

Traceability:

- converter volatility observation: original review `5249791627`, RED `2bf4001dc1cee62df3d826fe585e246528c450cb`, production `85265a29730698c6847a1089357a0a18e70c77d8`, public composition `d875c26cdd47c008015e8dfd0d769fc16ba8b5d0`; lifecycle correction review `5251695984`, source correction `b4df9bef3c5eeb892a2ac47e208d06f9bb0f5bb0`, drift regression `7ebe2a484330d882a8aa857fdb53333f524e09eb`;
- converter kind: review `5251421078`, RED `f60aa85fbf36ffb8ab6354afc4826d347ba8ea44`, production `b75b3f008c0baeeae1b5e5eb3fd421b14e0d1205`, public composition `75028d1c8c18a490f793f8fcde5061ec39c52158`, edge contract `98149497113369b1f8a95c7f00bb0800a854a56f`;
- converter return-set: review `5251492933`, RED `e92a8657a20878621da883a4326237f1d899bc89`, production `c6d11432cbac681a937449ea41e2c49e26f24409`, public composition `40fc0e46b8a9e8c1018245540fc788f47a9132c0`;
- converter argument count: review `5251508944`, RED `4f81f92f54edb622ba933617ef6a598d7a4a1d16`, production `65dabb7df539c2a80455f8b4aec5c024117b8a58`, final public composition `c7419477a8538551a573482f78013ad226923899`;
- doctoring: `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-{volatility,kind,return-set,argument-count}-integrity.md`.

Primary authority is PostgreSQL 18 `pg_proc`, `ALTER FUNCTION`, `CREATE TRANSFORM`, and `src/backend/commands/functioncmds.c::check_transform_function()`.

## Acceptance boundary

**Source repaired is not GREEN.** Repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage must be demonstrated on the final exact head. No predecessor execution result transfers after source movement.

The bounded PostgreSQL 18 live differential must retain all predecessor ordinary-EXCLUDE evidence and resolve every nonzero selected transform converter to the same exact-generation `pg_proc` row. It must independently capture definition/owner/ACL/configuration/security/planner facts plus raw `provolatile`, raw `prokind`, raw `proretset`, raw `pronargs`, and the argument-type vector needed to prove `proargtypes[0] == internal`. A raw `v` volatility value is captured as drift evidence, not normalized away. Non-`f` kind, `proretset=true`, `pronargs != 1`, a non-`internal` argument, missing resolution, mixed-generation joins, or normalization-derived substitutes remain capture failures for the structural chain.

## Residual material gap

The PostgreSQL 18 transform checker is now represented without collapsing observation into validation: the governed source chain carries raw `provolatile` plus the four shape predicates `prokind='f'`, `proretset=false`, `pronargs=1`, and exact `internal` argument type. Creation-time non-volatility is a validation interpretation over raw volatility, while a live post-creation `v` remains observable. This closes the specific lifecycle mismatch found in this source review; it is not a universal claim that every future PostgreSQL/catalog semantic fact is exhausted.

The next Source Observation successor must come from another fresh code/catalog review rather than extending the chain by naming fields speculatively. Publication and immutable semantic release remain unauthorized until exact-head Rust/coverage acceptance, the bounded PostgreSQL 18 differential, central workflow prerequisites, and downstream stack adoption are complete.

## Canonical prerequisite state

Active central owner remains `.github#2040@12c3fa6f3623aa5f2979d3d5ee4ed987002a6c0d`, OPEN / Draft / non-mergeable, against protected `.github/main@64aa08d7fa487deacd41c761c36277ca68cab6c9` until a fresh sweep proves otherwise. The branches are materially diverged and the shared scheduler changed on both sides. #2040 production remains repository-identity RED; protected main is only partially hardened.

The next valid central movement is ordinary/non-force path-wise protected-main reconciliation. The resolved scheduler must preserve #2040's v2 CodeQL producer, removal of source-neutral restamps, repository-scoped Actions credential/selected-token proof, stale-run revalidation, rationale/docstrings/tests; adopt compatible protected-main queue/coalescing/capacity behavior; and enforce the stronger repository-identity invariant.

Queue-health #2268 remains source-repaired at `142e5b2617778e79f665693be1e6f8c04d7533aa` and CodeQL scan-dispatch #2271 remains source-repaired at `2b849c874122961e025c29f7fa0bb697863c3d68`. Their hosted evidence remains independent and does not transfer into #2040.

Product bootstrap #35 remains exact `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`. Its Security Scan and SAST success do not override terminal CodeQL PR failure; no unchanged-head manual rerun or leaf-source workaround is authorized.

## Required order

`.github#2040` path-wise reconciliation with repository-identity repair -> focused scheduler GREEN -> fresh exact-head central terminal checks plus qualifying non-self approval -> fresh compatible #35 acceptance and normal landing -> exact #46 Rust 1.98 native/hosted GREEN -> bounded PostgreSQL 18 differential including raw converter volatility and structural facts (`prokind`, `proretset`, `pronargs`, internal argument type) -> fresh semantic-gap review -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
