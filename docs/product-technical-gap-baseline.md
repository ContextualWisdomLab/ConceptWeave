# Product / Technical Gap Baseline

**Snapshot:** 2026-09-13

This file is the code-current authority for the active ConceptWeave Source Observation lane. Exact SHAs, review IDs, runs, and statuses are evidence coordinates only; earlier-head execution evidence never transfers after head movement.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical client release-consumption contract. `semantic-data-portal` owns catalog/governance/consumption, `context-graph-contracts` interop contracts, `enterprise-architecture-core` EA truth, `contextual-orchestrator` production LLM routing, and Keyverse identity trust. Consumers use released/versioned `semantic_release`/contract/ACL coordinates only. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid.

## Live stack

Protected/default ConceptWeave `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425` at this snapshot.

- #6 `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft.
- #45 `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft on #6.
- #46 `codex/pr6-v3-index-evidence`, OPEN Draft/mergeable, is the active Source Observation writer. Its ordinary-forward lineage retains the repaired index/lifecycle/temporal, column-collation, column-identity, identity/nullability, and column-generation contracts. Generation finding `5188836578`, RED `be3adfd0fa49fe3e27d9794c3773993a48235289`, fixture isolation `493e56bc6e985495511ef981e2d765c7d0583b51`, production family `325556329e16a48b20560d0634ffd9ba07bfa34d`, aggregate integration `f4e8295bf86731a4de9c8810ebcec7fdce2ea5f8`, attachment-order regression `8b0c64479ad93bacd7df2c548731eb1c1f14e59f`, and doctoring `d67433cc0caebfc427719f04ed61f5d701b9c51e` are retained. A later P1 now activates column-expression identity: finding `5189444945`, RED `afc9509f021681e6f6a2d0e6d0c386cce46fad12`, doctoring `3a76c46bc4d12fac2b8c3407a1d6a09005ddd7ea`.
- Product bootstrap #35 remains a separate Product acceptance lane; central workflow evidence never transfers to #46.

#45 and #6 must not duplicate or partially cherry-pick the Source Observation slice. They adopt the complete verified child ordinary/non-force only after one unchanged exact #46 head is GREEN.

## PostgreSQL 18 representation-v3 state

The active successor preserves exact relation/type/index/constraint coordinates, true-array identity, type-kind/domain-base/range evidence, request-authorized cross-schema types, relation-scoped index semantics, PK/UNIQUE timing, explicit temporal constraint evidence, optional source-authoritative column-collation evidence, optional source-authoritative column-generation declaration mode, and optional source-authoritative column-identity declaration mode. `pg_constraint.conperiod` remains declaration authority for `WITHOUT OVERLAPS` PK/UNIQUE and PERIOD FK; index/lifecycle/operator shape never invents temporal truth. Temporal final columns resolve to range or multirange through observed type/domain evidence. PERIOD FKs retain exact action, referenced-key, timing, and bounded-reference requirements.

Retained production repairs include literal exclusion-operator inference removal (`5186175514 -> a9065d46... -> e286c352...`), PERIOD referenced-key completeness (`5186323924 -> d692773a... -> 1462105f...`), temporal backing-index ordered-key/static-shape coherence (`5186585545 -> ffe75edd... -> e258b394...`), explicit unusable lifecycle rejection (`5186802339 -> 681e280f... / ed4882e... -> ff842b35...`), and lifecycle completeness (`5187402940 -> 9d9e8d9e... -> 77b12263... -> 384d1305...`). The shared key-constraint backing-index predicate requires `ready() == Some(true)`, `valid() == Some(true)`, and `live() == Some(true)` when an index is promoted as authoritative support for observed PK/UNIQUE timing or positive `conperiod`. Generic standalone index lifecycle remains optional.

The retained test-fixture repair series through `d704bf9254d0f4c98401052ee275f0984039d62a` preserves the intended temporal/index negative cases while supplying the now-required access method, per-key semantics, catalog flags, lifecycle evidence, and `conexclop` controls. Those are test repairs, not new production semantics.

## Column collation identity / FK consistency — source repaired, acceptance pending

Review `5187855669` identified that `ColumnObservationV3` did not preserve `pg_attribute.attcollation`, while PostgreSQL requires collatable FK column pairs to use either two deterministic collations or the exact same collation. RED `8006b24fd4409bc092d80640084a64892190ea4a` established collation materiality, observed `attcollation=0` versus unobserved distinction, nondeterministic FK rejection, and valid controls.

Production lineage `3e495eac... -> 66489478... -> 3d8a7fb7... -> 8a4b7a1b... -> 6fb0c2b6...` implements a complete bounded `ColumnCollationObservation` family with exact qualified `pg_collation` identity, `collisdeterministic`, duplicate/determinism consistency checks, canonical ordering, domain-separated digesting, and bounded FK validation. Index collation, type/domain defaults, locale text, `search_path`, rendered DDL, and OIDs do not substitute for column-collation evidence.

## Column identity declaration mode — source repaired, acceptance pending

Review `5188215648` identified that the successor did not preserve `pg_attribute.attidentity`. RED `85f2eb0cdd80b6de6983895f598efe5e463da887` requires GENERATED ALWAYS/BY DEFAULT digest distinction, explicit not-identity versus family-unobserved distinction, complete bounded coverage, duplicate rejection, and permutation invariance. Production commits `69a86d2a52fc0f75360f6a48a4549154511fa92b` and `95f7f81f9bed07ebd17c8a9d3d27187b0c94fa48` add and integrate the isolated `ColumnIdentityObservation` family without changing legacy-unobserved digest semantics.

Follow-up review `5188419794`, RED `bd6da911262a70a0a6d27ba439848831062813fb`, repair `00b166bfb47473b567bef3c5ab2f800327afda34`, and doctoring `0bba879daec988b15cff63918224d70ff71c7cf6` close the identity/nullability consistency hole: an observed identity mode paired with the same bounded column reporting nullable source evidence fails closed as `column_identity_nullability`. Nullability never invents identity. Identity-sequence options and ownership remain a separate future evidence family.

## Column generation declaration mode — source repaired, acceptance pending

Review `5188836578` on exact predecessor `7386ffb46ac72fb5257ac2ffd50ef8baab672f5e` found a separate material Source Observation gap: no owned evidence preserved PostgreSQL 18 `pg_attribute.attgenerated`. PostgreSQL exposes empty/not-generated, `s`/stored, and `v`/virtual states. `atthasdef` cannot substitute because it covers both defaults and generation expressions and the catalog explicitly directs consumers to `attgenerated` to distinguish them. Stored and virtual generated columns differ in computation/storage semantics, and a generated column cannot also carry an identity definition.

Behavioral/source RED `be3adfd0fa49fe3e27d9794c3773993a48235289` established stored/virtual digest distinction, explicit not-generated versus family-unobserved distinction, complete bounded coverage, duplicate rejection, input-order invariance, and generated/identity contradiction rejection. `493e56bc6e985495511ef981e2d765c7d0583b51` corrected the contradiction fixture so the older identity/nullability invariant could not satisfy the new RED first.

Production commit `325556329e16a48b20560d0634ffd9ba07bfa34d` adds `ColumnGenerationObservation` with exact schema/relation/kind/column coordinates and explicit ordinary/stored/virtual modes, complete coverage, duplicate rejection, canonical ordering, and the domain separator `conceptweave.postgres_schema_snapshot.v3.column_generation.v1`. Aggregate integration `f4e8295bf86731a4de9c8810ebcec7fdce2ea5f8` adds `new_with_column_generations`, `with_observed_column_generations`, `column_generations`, canonical optional-family ordering, and `column_generation_identity` fail-closed validation. `8b0c64479ad93bacd7df2c548731eb1c1f14e59f` adds the reverse-order regression: identity-first snapshots cannot attach generation evidence later and bypass the cross-family invariant; they fail as `column_generation_observation_order`.

`ColumnObservationV3` remains frozen. Generation mode is not inferred from `atthasdef`, `pg_attrdef`, rendered DDL, defaults, types, naming, or OIDs. Doctoring `d67433cc0caebfc427719f04ed61f5d701b9c51e` records the implemented contract and exact source/test trace.

## Column default / generation expression identity — RED active

Review `5189444945` on exact predecessor `fb6b0226fb6c4364f9e241bf01b684e110433386` identified the next material Source Observation gap. PostgreSQL 18 stores explicit column default expressions and generation expressions in `pg_attrdef`. `pg_attribute.atthasdef` only establishes that the row exists and `attgenerated` distinguishes a default from a generated column; neither preserves the expression itself. `ALTER TABLE ... SET DEFAULT` changes future omitted-column write behavior, while `SET EXPRESSION AS` replaces generated-column computation and rewrites stored generated data. PostgreSQL inheritance/partition rules can also permit matching generation status with different generation expressions. Therefore two schemas can remain equal under all currently modeled column facts while differing materially in source behavior.

Source/compile RED `afc9509f021681e6f6a2d0e6d0c386cce46fad12` adds `crates/conceptweave-observation/tests/column_expression_contract.rs`. It requires different ordinary defaults and different generation expressions to produce distinct governed digests, explicit no-expression evidence to differ from family-unobserved state, complete bounded coverage, duplicate-coordinate rejection, input-order invariance, and consistency between expression kind and the already-authoritative column-generation family. Production `ColumnExpressionObservation`, aggregate attachment, and digest integration intentionally do not exist at this checkpoint, so the branch is expected RED.

Doctoring `3a76c46bc4d12fac2b8c3407a1d6a09005ddd7ea` adds `docs/research/postgresql-18-column-expression-evidence.md`. The selected contract leaves frozen `ColumnObservationV3` untouched and places a complete optional expression family after generation evidence. Exact schema/relation/kind/column coordinates plus explicit no-expression/default-expression/generation-expression state and exact server-rendered `pg_get_expr(adbin, adrelid)` text are governed evidence. `pg_attrdef` OIDs and `adbin`'s internal `pg_node_tree` serialization are capture-time/internal details rather than consumer identity. Expression dependency extraction, function volatility/leakproof/security analysis, expression equivalence, and application-domain meaning remain separate later contracts.

Current Source Observation state is **COLUMN_EXPRESSION_RED_ACTIVE**. Native/Product GREEN, source-repaired status, Ready, merge authorization, publication, and release are not claimed.

## Exact-head acceptance

The immediate source task is the minimum causal production repair for the column-expression RED. Once source-repaired, one unchanged exact #46 head must pass repository-pinned Rust 1.98 `cargo fmt --all --check`, strict workspace/all-target Clippy with warnings denied, `column_expression_contract`, `column_generation_contract`, `column_identity_contract`, `column_collation_contract`, lifecycle-completeness and retained temporal/type/index contracts, workspace/doc tests, release build, owned production docstring/test/edge-case coverage, and applicable Product/security/dependency/review terminal evidence. Any head movement restarts exact-head acceptance.

The current execution host has no installed `cargo`, `rustc`, or `rustfmt`; native execution has not yet been established for this head. #46 has not obtained a qualifying exact-head hosted acceptance set. This is not a reason to toggle Draft/Ready, synthesize status, copy central workflows, manually/no-op retrigger, transfer predecessor evidence, or weaken a gate.

## Central Product-CI owner

Central workflow ownership remains outside ConceptWeave. Product bootstrap/review integration evidence never transfers to #46.

The Required Noema Review failure has an executable owner RED. Actual model transport through `contextual-orchestrator/orchestrator/free` succeeded, but the advertised structured-output schema represents `findings[]` and `adversarial_validation.probes[]` as independent arrays while the deterministic validator requires a confirmed probe anchored to a published finding location. Owner review `5188653693` identified the relation mismatch on `.github#2079`. Owner RED `6ca329896a846110ade7182ed6fa0fa7b0fbba7d` adds `tests/test_noema_review_finding_probe_binding_contract.py`; review `5188972300` fixes the causal boundary: the schema must carry an explicit finding/probe relation coordinate, the prompt must explain confirmed versus falsified population, and the validator must check a valid referenced finding plus same changed-side location. Provider/model fallback and validator weakening remain invalid repairs. Fresh owner state must be re-read before any Product acceptance claim. Strix retains its separate terminal-sub-agent negative-control obligation. None of this evidence transfers to ConceptWeave acceptance.

## PostgreSQL adapter boundary

Transport remains blocked until representation exact-head GREEN and ordinary/non-force adoption through #45/#6. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, resolve least-privilege credentials only through the authorized source/policy binding, use bounded `REPEATABLE READ READ ONLY` catalog capture, and never keep an explicit database transaction/lock open while waiting on an LLM or long external computation.

Catalog OIDs are capture-time joins only. Constraint support must bind `pg_constraint.conindid` to the exact same-snapshot `pg_index` row and explicitly capture usable lifecycle, key/static flags, `conexclop`, operator-class/operator-family evidence, timing/action/match state, and temporal type/domain chains. Referenced temporal keys outside the bounded relation set require explicitly authorized evidence expansion or remain fail closed.

For column collation, the adapter captures `attcollation` for every bounded column when claiming that family; zero is explicit uncollatable evidence and nonzero OIDs resolve inside the same catalog snapshot to exact `pg_collation` namespace/name plus `collisdeterministic`.

For column identity, the adapter captures `attidentity` and `attnotnull` for every bounded column when claiming that family and maps only the documented empty/`a`/`d` states. Unexpected values fail closed; sequence options/ownership are separate evidence.

For column generation, the adapter captures `attgenerated` for every bounded column when claiming that family and maps only the documented empty/`s`/`v` states. Unexpected values fail closed. `atthasdef` does not establish expression kind or contents.

For column expressions, a later adapter implementation must join `pg_attribute` to the matching `pg_attrdef` row within the same bounded catalog snapshot. When the expression family is claimed, every bounded column records explicit no-expression, default-expression, or generation-expression state. Present expressions preserve exact server-rendered `pg_get_expr(adbin, adrelid)` text and must agree with the already-observed generation mode. `pg_attrdef` OIDs and internal `adbin` serialization must not become governed consumer coordinates.

## Primary authority

- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_attrdef*. https://www.postgresql.org/docs/18/catalog-pg-attrdef.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: pg_collation*. https://www.postgresql.org/docs/18/catalog-pg-collation.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: Identity columns*. https://www.postgresql.org/docs/18/ddl-identity-columns.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: Generated columns*. https://www.postgresql.org/docs/18/ddl-generated-columns.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: ALTER TABLE*. https://www.postgresql.org/docs/18/sql-altertable.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 documentation: CREATE TABLE*. https://www.postgresql.org/docs/18/sql-createtable.html
- PostgreSQL Global Development Group. (n.d.). *PostgreSQL 18 release notes*. https://www.postgresql.org/docs/18/release-18.html

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Source Observation | COLUMN_EXPRESSION_RED_ACTIVE | `5189444945 -> afc9509f... -> 3a76c46b...`; production family/integration and exact-head native/hosted GREEN are pending. |
| Retained column semantics | SOURCE_REPAIRED / EXECUTION_PENDING | Generation, identity, nullability, collation contracts remain retained and must stay GREEN after the expression repair. |
| Product CI | CENTRAL_OWNER_RED_ACTIVE | `.github#2079@6ca329896a846110ade7182ed6fa0fa7b0fbba7d` finding/probe-binding owner path requires causal GREEN; central evidence never transfers to #46. |
| Quality gate | BLOCKED_ON_SOURCE_REPAIR | Implement the expression family first; then obtain Rust 1.98/native and hosted acceptance on one unchanged repaired head. |
| PostgreSQL adapter | BLOCKED_ON_REPRESENTATION_ACCEPTANCE | No transport before #46 GREEN and parent adoption. |
| Publication | NO_PUBLICATION | No protected immutable semantic release exists. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/semantic release/SBOM/provenance/reproducibility/rollback remain mandatory. |

## Current causal sequence

1. Implement the minimum complete/domain-separated column-expression evidence family on #46, preserving frozen v3 compatibility and enforcing generation/expression consistency; turn `column_expression_contract` from real RED to GREEN.
2. On the resulting unchanged exact head, execute repository-pinned Rust 1.98 and hosted Product/security/dependency/review acceptance. Repair only real failures; any head movement restarts acceptance.
3. Adopt the complete verified #46 delta ordinary/non-force into #45, obtain fresh parent acceptance, then adopt #45 into #6.
4. Repair/settle the central `.github` Noema/Strix review-owner path from fresh owner state; central evidence never transfers to ConceptWeave.
5. Only after representation/Product prerequisites are GREEN may bounded PostgreSQL transport proceed, followed by deterministic validation, independent evaluation, steward review, immutable publication, and release evidence.
