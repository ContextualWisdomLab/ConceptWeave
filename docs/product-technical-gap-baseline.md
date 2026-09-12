# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This file is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only; earlier-head execution evidence never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, `contextual-orchestrator` production LLM routing, and Keyverse identity trust. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, is the active Source Observation writer. Its ordinary-forward lineage retains the repaired column-collation family and now contains the PostgreSQL column-identity source repair through `95f7f81f9bed07ebd17c8a9d3d27187b0c94fa48`; native/Product acceptance remains pending.
- Product bootstrap #35 remains `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mergeable.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal constraint evidence, optional source-authoritative column-collation evidence, and optional source-authoritative column-identity declaration mode. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

Retained production repairs include literal exclusion-operator inference removal (`5186175514 -> a9065d46... -> e286c352...`), PERIOD referenced-key completeness (`5186323924 -> d692773a... -> 1462105f...`), temporal backing-index ordered-key/static-shape coherence (`5186585545 -> ffe75edd... -> e258b394...`), explicit unusable lifecycle rejection (`5186802339 -> 681e280f... / ed4882e... -> ff842b35...`), and lifecycle completeness (`5187402940 -> 9d9e8d9e... -> 77b12263... -> 384d1305...`). The shared `key_constraint_backing_index_static_shape_matches()` predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional.

## Retained-test fixture integrity — repaired, execution pending

Review `5187787381` identified a separate P1 on exact predecessor `cdb50a49d6abb3101abb9498a88adf7fe0d890e0`: several retained temporal/index tests constructed supporting indexes using older fixture assumptions. `RelationObservation::with_indexes()` now requires a nonblank access method and exactly one `IndexKeySemantics` record per structural key; constraint-support admission additionally requires explicit ready/valid/live evidence. Stale fixtures therefore failed during setup instead of reaching the temporal/type/index assertion they claimed to test.

The ordinary-forward fixture repair series is test-only from `cdb50a49...` through `d704bf9254d0f4c98401052ee275f0984039d62a`. A direct compare is 10 commits ahead / 0 behind and changes only nine contract-test files; no production source or semantic rule changed. The repaired tests add exact per-key opclass/indoption evidence and explicit lifecycle=true to coherent supporting-index controls while preserving deliberately missing/false lifecycle, missing catalog flags, wrong key order, action/timing/type/operator negative cases. `constraint_period_type_contract.rs` also restores complete `conexclop` fixture evidence so its tests reach the intended type-kind assertions. `constraint_period_contract.rs` drops a stale unused import after the restack.

Repair coordinates:

- `aeed9149e0613f11cd206f1273ac8bf8da33c826` — exclusion-operator fixtures;
- `c9bc77b522cc13b508acebea6f38c8bc0ba1b1e5` and `d704bf9254d0f4c98401052ee275f0984039d62a` — period fixtures plus stale-import cleanup;
- `148f68044cbaab30c6827f709dd48b50493b120c` — temporal-index controls;
- `0a8799bd3935d673715cb6a41fea4810678840c6` — timing controls;
- `cd650963a68f97edc9c3d67c628fc6e69dd90f0f` — backing-index presence/lifecycle controls;
- `2d6ab7a21f5d9bf5dfef018a3ed1a9e424327cf1` — temporal FK action controls;
- `b692bc22bfbf2391a1e21bc83c3b4819c4c3c693` — referenced-key timing controls;
- `546d5efda56e151bb316b741e416d9d595c9c78a` — temporal period-type controls;
- `7f8416dfee4fca20db9a027b4aadf60f26260061` — key backing-index shape controls.

## Column collation identity / FK consistency — source repaired, acceptance pending

Review `5187855669` verified the column-collation Source Observation gap on predecessor exact `fcb75659c4c7ffc046c4c7187789ef3b5d53715d`. `ColumnObservationV3` preserved exact qualified type identity but not `pg_attribute.attcollation`; PostgreSQL 18 also requires every collatable referencing/referenced FK pair to have collations that are either both deterministic or exactly the same. The representation therefore needed an explicit source-authoritative column-collation family rather than inference from type/domain/index state.

Source-level RED `8006b24fd4409bc092d80640084a64892190ea4a` established exact collation materiality, observed `attcollation=0` versus family-unobserved distinction, nondeterministic FK rejection, and both-deterministic / exact-same nondeterministic controls. Doctoring `313f27c35be2b1bf834ad98c43e162ae874c9107` fixed the source boundary and rejected inference paths.

Production repair is present. `3e495eac9e345db40e3009fe7579e6422a46447a` introduced the isolated family; `664894780f2643c00b86165b0b75757ee1d607db` corrected borrowed determinism access before integration; `3d8a7fb7c8f00ebbc7f6994f2b83274a6f3d3bc0` integrates `ColumnCollationObservation` and the public aggregate boundary. The original v3 constructor still leaves the family unobserved and retains its prior digest. The observed family is complete for every bounded relation column, distinguishes explicit uncollatable state, stores exact qualified `pg_collation` plus `collisdeterministic`, rejects duplicate column coordinates and conflicting determinism for one collation coordinate, canonicalizes input order, and extends identity under `conceptweave.postgres_schema_snapshot.v3.column_collations.v1`.

For an FK whose local and referenced relations are both in the bounded inventory, the family compares the exact paired column evidence directly. A collatable pair is valid when both collations are deterministic or the exact qualified coordinates are equal; a different pair with either side nondeterministic fails closed as `foreign_key_collation`. Index `indcollation`, type/domain defaults, locale text, `search_path`, rendered DDL, and OIDs are not substitutes. An FK that references a relation outside the bounded inventory does not receive invented remote collation truth; a later immutable referenced-column evidence seam is required before claiming cross-boundary collation adjudication.

Edge-contract successor `8a4b7a1b59353289d3cc33f80d852db9116f7096` adds completeness, duplicate-coordinate, conflicting-determinism, permutation-invariance, retained-evidence, and one-sided-nondeterministic FK cases. Doctoring successor `6fb0c2b673bec4784751cf2e2e90da55fc47458c` aligns the decision record with the implemented boundary.

## Column identity generation mode — source repaired, acceptance pending

Review `5188215648` identified a separate material Source Observation gap on exact predecessor `122dfa983ef1c2a9a1f6a68882eb12461243155d`. PostgreSQL 18 exposes identity declaration mode directly in `pg_attribute.attidentity`: empty means not an identity column, `a` means `GENERATED ALWAYS`, and `d` means `GENERATED BY DEFAULT`. The predecessor v3 column contract/public aggregate did not preserve that family, so two otherwise-identical schemas could collapse even though their admissible write behavior differs.

Behavioral/source RED `85f2eb0cdd80b6de6983895f598efe5e463da887` added `column_identity_contract.rs`. It requires GENERATED ALWAYS and GENERATED BY DEFAULT to produce different governed identities, explicitly observed not-identity to remain distinct from family-unobserved evidence, a claimed family to cover every bounded relation column, duplicate exact column coordinates to fail closed, and input order to remain identity-neutral.

Doctoring `9bcc5e66c0eb6e0234ebcdb82ea8d5b5896b7138` recorded the direct catalog authority, compatibility boundary, rejected inference paths, inheritance/partition caveat, and the separate future responsibility for attached identity-sequence options. Identity must not be inferred from integer type, `NOT NULL`, PK/UNIQUE/index shape, rendered default text, `nextval(...)`, sequence naming, or OIDs.

Production repair is now present. `69a86d2a52fc0f75360f6a48a4549154511fa92b` adds the isolated `ColumnIdentityObservation` family with explicit not-identity/generated-always/generated-by-default states, exact bounded coordinates, duplicate/completeness validation, canonical input ordering, public rustdoc, and `conceptweave.postgres_schema_snapshot.v3.column_identity.v1` digest framing. `95f7f81f9bed07ebd17c8a9d3d27187b0c94fa48` integrates the family into `PostgresSchemaSnapshotV3` with explicit constructor, consuming method, accessor, and unobserved initialization on every legacy-compatible constructor path. Canonical optional-family order is type/array -> column collation -> column identity -> constraint timing/period. Identity-sequence options remain outside the family.

A direct compare from RED baseline `89f53ff08c9a1510b04b0207c2f6b10c1af0ed6e` to `95f7f81f9bed07ebd17c8a9d3d27187b0c94fa48` is two commits ahead / zero behind and changes only `src/column_identity.rs` plus intended aggregate integration in `src/lib.rs`. The integration commit diff is 92 additions / 12 deletions in `lib.rs`; deleted lines are documentation/order-guard replacements, not removed semantic families. Structural compare is not Rust execution evidence.

Current Source Observation state is **COLUMN_IDENTITY_SOURCE_REPAIRED / ACCEPTANCE_PENDING**. It is not native/Product GREEN, Ready, merge-authorized, published, or released.

## Exact-head acceptance

One unchanged exact #46 successor containing the column-identity repair must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, `column_identity_contract`, the repaired `column_collation_contract`, lifecycle-completeness and all retained temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The available execution host does not provide `cargo`/`rustc`; therefore repository-pinned Rust execution cannot be substituted locally. Current #46 pushes have not produced pull-request-triggered workflow runs. This is not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, transfer predecessor evidence, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. Fresh owner state is `ContextualWisdomLab/.github#2114` exact `e7c58c04ed7e59c23cbe4a5f38d4c522ae712712`, OPEN/non-Draft/mergeable, based on protected `.github/main@fb17ef556f94f673234aa557254ae52779e9a7b0`. On that exact head SAST Semgrep `34716210489`, Python Security `34716210535`, Security Scan `34716210462`, Runtime Quality `34716210506`, and CodeQL PR `34716210555` are terminal GREEN.

That does not complete the central review gate. Current-head OpenCode review is `CHANGES_REQUESTED` because Required Noema Review failed and the Strix review run was cancelled on the same head; no qualifying independent approval exists. The `.github` review/settlement lane remains central-owner work. ConceptWeave must not copy the workflow, bypass providers, synthesize settlement, or treat central evidence as leaf evidence.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, use bounded `REPEATABLE READ READ ONLY` catalog capture, and never keep an explicit database transaction/lock open while waiting on an LLM or long external computation.

Catalog OIDs may only join the captured snapshot. When the adapter claims an index supports PK/UNIQUE timing or a temporal key, it must bind `pg_constraint.conindid` to the exact `pg_index` row inside the same authorized snapshot and explicitly capture `indisready=true`, `indisvalid=true`, and `indislive=true`. It must retain exact key layout/static flags, `conexclop`, operator-class/operator-family compare-type evidence, temporal type/domain chain, timing/action/match facts, and policy-admitted row/byte/concurrency ceilings. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail-closed.

For column collation, the adapter must capture `pg_attribute.attcollation` for every bounded column when claiming that family. Zero is explicit uncollatable evidence; nonzero OIDs must be resolved within the same bounded catalog snapshot to exact `pg_collation` namespace/name plus `collisdeterministic`. OIDs stay adapter-local. Column collation must not be reconstructed from the type, domain, index, rendered DDL, locale text, or `search_path`. Cross-boundary referenced-column collation, when required, needs explicit immutable/versioned evidence rather than hidden schema widening.

For column identity, the adapter must capture `pg_attribute.attidentity` for every bounded column when claiming that family and map only PostgreSQL's documented empty/`a`/`d` states. Unexpected values fail closed. The attached sequence's options and ownership are not inferred from this flag; they require a separately versioned immutable evidence family.

## Primary authority

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_collation*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: Identity columns*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE TABLE*.
- PostgreSQL Global Development Group. (2026). *PostgreSQL source: ComputeIndexAttrs()*.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | COLUMN_IDENTITY_SOURCE_REPAIRED | `5188215648 -> 85f2eb0c... -> 9bcc5e66... -> 69a86d2a... -> 95f7f81f...`; native and hosted acceptance remain pending. |
| Product CI | CENTRAL_OWNER_REVIEW_BLOCKED | `.github#2114@e7c58c04...`: five core hosted workflows GREEN; Noema/Strix review settlement and independent approval remain unresolved. |
| Quality gate | BLOCKED_ON_EXACT_HEAD_EXECUTION | Generate Rust 1.98/native and hosted acceptance on one unchanged source-repaired head. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before #46 GREEN and parent adoption. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Keep the source-repaired identity/collation families and all retained index/lifecycle/temporal invariants unchanged while obtaining one exact-head Rust 1.98 and hosted Product/security/dependency/review acceptance set. Head movement restarts the set.
2. If execution exposes a real failure, repair only that causal defect ordinary-forward and restart exact-head acceptance. Do not weaken or bypass the gate.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Independently, `ContextualWisdomLab/.github#2114` must resolve Noema/Strix review settlement and obtain qualifying independent approval. No central evidence transfers to ConceptWeave.
5. Only after representation/Product prerequisites are GREEN may the bounded PostgreSQL adapter proceed, followed by deterministic validation, independent evaluation, steward review, immutable publication, and release evidence.
