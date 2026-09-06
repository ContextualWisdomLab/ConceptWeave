# Product / Technical Gap Baseline

**Snapshot:** 2026-09-06

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, never mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

Current active roots observed for this refresh:

1. Foundation PR #1 — `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open/mergeable. Product CI still cannot originate from protected `main` because `.github/workflows/product.yml` has not yet been integrated.
2. Product-CI bootstrap PR #35 — `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft/mergeable. Security Scan and SAST are terminal success; existing CodeQL/OpenCode/Strix evidence is terminal failure; Noema has a blocking `CHANGES_REQUESTED`; no qualifying independent APPROVE exists.
3. Client Consumption PR #5 — `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open/mergeable. It retains deterministic generic release admission, integrity, compatibility, diff/resolution and supersession validation.
4. Source Observation PR #6 — this baseline was refreshed from successor source immediately after `21386548889ca1152cfc4dc6dcd3c1f11c658675`; the documentation commit itself creates a newer ordinary forward head. The stack remains Draft/open/mergeable on Client #5 and now carries source-key + immutable policy-binding + exact-schema authorization, one non-resetting operation budget, snapshot-side exact-schema containment, stale-binding fail-closed port fixtures, and binding-preserving immutable snapshot/receipt provenance. No live PostgreSQL adapter or exact-head Rust GREEN is claimed.
5. Zotero Research Classification root #9 and its #13→#38 descendants remain a separately coordinated single-writer lane. This Source Observation writer does not mutate their source/ref/PR metadata.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op retrigger, mutable supplier dependency, or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and public contracts preserve observed/inferred/proposed/authoritative/rejected/superseded distinctions. Protected exact-head Product evidence is still unavailable until bootstrap #35 integrates. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL facts, deterministic content digest, exact-schema authorization, source-policy binding, non-resetting deadline, cancellation/resource failures, snapshot containment and policy-binding provenance exist in source. ADR 0004 remains Proposed because production adapter/runtime evidence does not. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, detached artifact verification and explicit supersession validation exist. Current protected evidence and prerequisite integration remain outstanding. |
| Quality gate | BLOCKED_BY_BOOTSTRAP | Rust 1.98.0, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, release build and owned 100% coverage remain required. This execution environment has no Rust toolchain and current #6 has no hosted Product/Rust run, so source commits are not GREEN evidence. |
| Central review plane | OWNER_REPAIR_PENDING | `.github/main` is `fe827e133e7d867015d088777553e22736344c55`. `.github#1929` remains open: the current app-token dispatcher identity and repository authorization allowlist are not reconciled, so fresh substantive OpenCode/CodeQL evidence for #35 is still unavailable. |
| Noema | OWNER_REVIEW_REPAIR_PENDING | #35 retains a contradicted external-Cargo-capability `CHANGES_REQUESTED`; `.github#1924` is the generic owner path. Failure-artifact capture on central main improves diagnosis but is not adjudication repair. |
| Strix | OWNER_RUNTIME_REPAIR_PENDING | #35 reached the trusted gateway but failed on repeated HTTP 500. Central account-selection bias was repaired, while `contextual-orchestrator#1049` still owns HTTP-500 failover/exhaustion behavior. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Source Observation current contract

`ObservationRequest` admits only bounded opaque source keys, explicit exact-schema allowlists, bounded authorization metadata, and positive operation/statement/row/byte/concurrency limits. The local `SourceConnectionRegistry` must issue a bounded opaque immutable connection-policy binding and authorize the exact schema scope against the resulting `ResolvedSourceConnection`; both additional policy decisions default to fail closed. A known key without a binding cannot execute, and connection material such as a PostgreSQL DSN is rejected as an invalid binding rather than crossing the port seam.

`AuthorizedObservationRequest` carries only the validated request, source key, opaque policy binding, and private monotonic operation-start coordinate. The adapter receives only `remaining_operation_budget()` rather than a reset timeout. A later adapter ACL may resolve credentials only for the exact key-and-binding pair. A capability authorized for revision A must not silently retarget to revision B after the registry changes; the port fixture requires stale-binding failure before source and snapshot side effects and has an unchanged-binding positive control.

`PostgresSchemaSnapshot::new` requires the complete authorized envelope, rejects locally observed table schemas outside the exact authorized allowlist before digest/receipt construction, and retains the authorized policy binding as immutable provenance. Source-content digest identity remains separate from source key and policy revision. Public `SourceObservationReceipt` retains source id, exact policy binding, digest, extractor revision, observation time and verified location. Foreign-key target schemas remain relationship evidence and do not grant read authority for those schemas.

These are source-reviewed executable contracts, not a claimed executed RED→GREEN. The next acceptance is unchanged-head Rust 1.98 Product/test/fmt/Clippy/rustdoc/release/owned-coverage evidence plus observed repair of any real failures. Only then should a concrete maintained Rust PostgreSQL adapter be added.

## Central owner evidence relevant to #35

Protected central source is `.github/main@fe827e133e7d867015d088777553e22736344c55` at this snapshot. `.github#1929` remains open and records that the app-token producer dispatches as `opencode-agent[bot]` while the effective authorization evidence has continued to reject that identity. ConceptWeave does not edit the central allowlist or replay stale failed handles. Owner acceptance requires the intended legitimate producer identities to be reconciled explicitly, followed by a newly emitted current-workflow repository dispatch that passes metadata validation and produces a terminal authenticated verdict.

#35 also remains blocked by the separate Noema contradicted-capability review and contextual-orchestrator/Strix HTTP-500 failover lane. These are owner-path blockers for #35 only; they do not justify speculative Source Observation provider fallbacks or weakening ConceptWeave gates.

## P0 product gaps

1. **Exact-head Source Observation verification** — run Rust 1.98 fmt, strict Clippy, tests, warnings-denied rustdoc, release build, owned 100% coverage and applicable security/dependency gates on one unchanged #6 head; repair only observed failures.
2. **Concrete PostgreSQL Source Observation adapter** — maintained patched Rust PostgreSQL driver; exact-binding least-privilege credential resolution; explicit `REPEATABLE READ READ ONLY`; exact-schema `pg_catalog` evidence; one remaining-budget clock across connect/transaction/statements/cancellation; row/byte/concurrency bounds; stale-binding rejection; complete immutable snapshot or fail closed; source disappearance; frozen anonymized GRC-shaped replay.
3. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers and cross-schema collisions as generic observed evidence without importing source-system business truth.
4. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
5. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; relational structure alone is not semantic authority.
6. **LLM Proposal** — every production model call through a released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
7. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
8. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
9. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
10. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
11. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
12. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
13. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
14. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
15. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
