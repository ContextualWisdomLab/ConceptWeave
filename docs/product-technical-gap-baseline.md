# Product / Technical Gap Baseline

**Snapshot:** 2026-09-19

The Source Observation decision surface before converter-function initial-privilege evidence is preserved at `docs/archive/product-technical-gap-baseline-through-71c86b19.md`; its matching changelog is `docs/archive/CHANGELOG-through-71c86b19.md`. Earlier surfaces remain under `docs/archive/`. Focused rationale and primary-source traceability remain under `docs/doctoring/`. Exact-head execution or review evidence never transfers after source or documentation movement.

## Canonical boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and its released client contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` owns interop contracts, `enterprise-architecture-core` owns EA truth, and `contextual-orchestrator` owns production LLM routing. Product-domain truth remains with its owner; consumers use released/versioned semantic contracts rather than source copies, cross-service SQL, or mutable sibling heads.

#46 `codex/pr6-v3-index-evidence` remains the Draft Source Observation single writer stacked on #45. #45 and #6 must not partially adopt or independently reimplement #46. Product bootstrap #35 and canonical reusable-workflow ownership in `ContextualWisdomLab/.github` remain separate prerequisites.

## Current Source Observation authority

The converter chain retains exact definition/owner/current-ACL/config/security-definer/leakproof/strictness/volatility/parallel-safety/planner-support/cost/function-shape facts, raw nullable `pg_proc.proargmodes`, raw nullable `pg_proc.proargnames`, raw nullable converter-function `pg_proc.protrftypes`, immutable raw converter-root lineage, converter-function extension membership, complete auto-extension dependency sets, complete security-label maps, converter-function initial privilege baselines, and independent transform-object extension membership.

Fresh PostgreSQL 18 review found a recovery distinction not represented by the previous chain. `pg_init_privs` records a non-default initial privilege baseline for an object. The baseline can originate from `initdb` (`privtype='i'`) or from GRANT/REVOKE executed while creating an extension (`privtype='e'`). PostgreSQL extension packaging documentation states that these extension-script privilege changes are stored in `pg_init_privs`, and `pg_dump` emits the GRANT/REVOKE operations needed to reproduce the object's current privileges after `CREATE EXTENSION`. Therefore two exact converter functions can have the same current `pg_proc.proacl`, definition, owner, security labels, extension lifecycle, and transform binding while differing in dump/restore privilege reconstruction.

Finding review `5254405649` pinned that lossiness at exact pre-finding head `71c86b196ab1fa2e66143d8f0f708a2606ace724`. Structural RED `6a16b6c314e9ba6f9b8a2038055713c62e4cefab` referenced the public initial-privilege API before that API existed. Production `c94caa8a1da7088c819b802a87588ba303edcf55` added initial privilege type, canonical initial EXECUTE-grant material, observation, immutable receipt, snapshot, exact completeness/binding checks, and immutable raw-root propagation. Public composition followed at `1ba7f86bb62c399f334b823ed51bad651e398f24`.

Each converter direction now carries explicit absence or one exact same-generation `pg_init_privs` row for the selected converter function. Capture is bound to `classoid=pg_proc`, the exact converter function `objoid`, and `objsubid=0`. Row absence and a present row remain distinct. Present rows preserve `privtype` and the complete object-level initial EXECUTE ACL after same-generation grantor/grantee role resolution. ACL ordering is canonicalized because array order is not semantic; PUBLIC versus named-role grantee, exact grantor, and grant option remain identity-bearing. Duplicate ACL entries and blank resolved role names fail closed.

The successor preserves complete converter coordinates, direction, exact schema/function binding, source generation metadata, and the immutable raw `converter_snapshot_digest`. Missing/extra/duplicate converter coordinates, binding drift, zero key positions, unknown receipt coordinates, and qualified-type provenance collisions fail closed.

Transform-object extension membership was ordinary-forward restacked at `50dddc811bacc301fc36f0d7b5bddd3a6a51fd18` so the initial-privilege snapshot is now its digest predecessor. Its retained contract was restacked at `34c8611f0c75723bdebc82e0e84fa67a92856604`, and the same-generation direction/function/raw-root hostile lineage contract at `a1c35b715e56f5a62813ba60e65d0ba6a80ba418`. The transform-object successor still receives the separately supplied raw transform-converter snapshot and first proves equal source generation plus exact immutable raw converter root before validating complete FROM SQL / TO SQL cardinality and exact converter schema/function identity. A change in converter initial privilege state therefore changes the later transform-object lifecycle digest instead of disappearing from the final chain.

Focused doctoring is `docs/doctoring/postgresql-index-exclusion-constraint-operator-procedure-transform-converter-initial-privileges-integrity.md`; current CHANGELOG authority was updated at `faafbf813c9933a4fdb284b80ca55e63fc4c1846`.

Current converter ACL, converter-function `deptype='e'`, complete converter-function `deptype='x'` sets, converter-function security labels, converter-function initial privilege baselines, and transform-object `deptype='e'` remain distinct facts. ConceptWeave does not infer one from another. Extension-owned `extversion`, configuration, control/update scripts, package inventory, role-membership policy, restore orchestration, and application metadata remain outside ConceptWeave.

Primary authority:

- PostgreSQL Global Development Group. (2026a). *PostgreSQL 18 documentation: 52.28. pg_init_privs*. https://www.postgresql.org/docs/18/catalog-pg-init-privs.html
- PostgreSQL Global Development Group. (2026b). *PostgreSQL 18 documentation: 36.17. Packaging related objects into an extension*. https://www.postgresql.org/docs/18/extend-extensions.html
- PostgreSQL Global Development Group. (2026c). *PostgreSQL 18 documentation: GRANT*. https://www.postgresql.org/docs/18/sql-grant.html
- PostgreSQL Global Development Group. (2026d). *PostgreSQL 18 documentation: pg_dump*. https://www.postgresql.org/docs/18/app-pgdump.html

## Observation versus governance

Source Observation records exact external database state before policy. Validation/publication may later reject an initial privilege arrangement or impose enterprise authorization requirements, but it must reason over the observed `pg_init_privs` baseline rather than reconstructing it from current ACL, extension membership, package state, or names. The same boundary continues to apply to security labels, post-creation volatility drift, and extension lifecycle facts.

## Acceptance boundary

**Source and traceability repaired is not GREEN.** The final exact head must independently demonstrate repository-pinned Rust 1.98 `fmt`, strict workspace/all-target Clippy, the new initial-privilege contract, retained security-label/auto-extension/raw-root/direction/function regressions on the new predecessor shape, focused and retained tests, workspace/doc tests, release build, rustdoc, and owned production statement/branch/edge coverage. No predecessor execution result transfers after source or documentation movement.

The bounded PostgreSQL 18 live differential must resolve every selected converter and transform from one source generation. For each exact converter function OID it must independently capture current function facts and ACL, `deptype='e'` membership, the complete `deptype='x'` dependency set, the complete `pg_seclabel` provider/label map, and exact `pg_init_privs` row absence/presence with `privtype` plus the complete initial EXECUTE ACL for `classoid=pg_proc` and `objsubid=0`; preserve immutable raw converter-root identity; and separately resolve the transform object's own `deptype='e'` membership. At least one live differential must hold current `proacl` constant while changing the initial privilege baseline. Reconstruction from current ACL, function name, extension state, package metadata, another source generation, or application metadata is a capture failure.

A source-neutral wake commit or unchanged-head manual rerun is not an acceptable substitute for causal execution evidence.

## Residual material gap

This successor is admitted because PostgreSQL primary authority demonstrates an independent dump/restore recovery distinction; it is not catalog-field enumeration. No further converter/transform successor is authorized merely because another catalog field, dependency code, comment, or metadata row exists.

The next semantic successor requires proof of an independent buyer, semantic, security, lifecycle, recovery, or provenance distinction that is not already represented by the current raw-root, exact function binding, current ACL/function facts, extension lifecycle, security-label, initial-privilege, and transform-object lifecycle chain. Otherwise this sub-chain moves to exact-head acceptance and the PostgreSQL 18 same-generation differential.

## Canonical prerequisite state

Central `.github#2040` and product bootstrap #35 remain separate prerequisites. Their exact heads, mergeability, reviews, required checks, workflow inventory, and protected-main relationship must be read fresh before landing. Sibling or predecessor evidence does not transfer into #2040, #35, or #46.

## Required order

`.github#2040` ordinary/non-force protected-main reconciliation plus repository-identity production repair -> fresh central exact-head required GREEN plus qualifying non-self approval -> fresh compatible #35 acceptance/normal landing -> final #46 Rust 1.98/coverage GREEN -> bounded PostgreSQL 18 same-generation differential including current ACL, converter `deptype='e'`, complete converter `deptype='x'` sets, complete converter security-label maps, exact converter initial privilege baselines, immutable raw converter-root lineage, and transform-object `deptype='e'` -> fresh semantic-gap review only for independently proven distinctions -> complete ordinary/non-force #46 adoption into #45 -> fresh #45 acceptance -> #6 propagation -> immutable governed release only from an accepted protected head.
