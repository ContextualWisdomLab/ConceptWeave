# Product / Technical Gap Baseline

**Snapshot:** 2026-09-06

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, never mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

Current active roots observed for this refresh:

1. Foundation PR #1 — `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open/mergeable. Product CI still cannot originate from protected `main` because `.github/workflows/product.yml` has not yet been integrated.
2. Product-CI bootstrap PR #35 — `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft/mergeable on the latest retained exact source head. Security Scan and SAST have terminal success evidence; existing CodeQL/OpenCode/Strix evidence is not merge-valid; Noema has a blocking `CHANGES_REQUESTED`; no qualifying independent APPROVE has been established.
3. Client Consumption PR #5 — `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open/mergeable. It retains deterministic generic release admission, integrity, compatibility, diff/resolution and supersession validation.
4. Source Observation PR #6 — source and contract docs advanced ordinarily through `ab57a3dc7ed305a7319d81832aafdd296331f7fa` immediately before this baseline refresh; the baseline commit itself creates a newer successor head. The stack remains Draft on Client #5 and now carries source-key + immutable policy-binding + exact-schema + trusted resource-envelope authorization, one non-resetting operation budget, snapshot-side exact-schema containment, stale-binding fail-closed port fixtures, and binding-preserving immutable snapshot/receipt provenance. No live PostgreSQL adapter or exact-head Rust GREEN is claimed.
5. Zotero Research Classification root #9 and its #13→#38 descendants remain a separately coordinated single-writer lane. This Source Observation writer does not mutate their source/ref/PR metadata.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op retrigger, mutable supplier dependency, or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and public contracts preserve observed/inferred/proposed/authoritative/rejected/superseded distinctions. Protected exact-head Product evidence is still unavailable until bootstrap #35 integrates. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL facts, deterministic content digest, exact-schema authorization, source-policy binding, trusted complete resource-envelope admission, non-resetting deadline, cancellation/resource failures, snapshot containment and policy-binding provenance exist in source. ADR 0004 remains Proposed because production adapter/runtime evidence does not. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, detached artifact verification and explicit supersession validation exist. Current protected evidence and prerequisite integration remain outstanding. |
| Quality gate | BLOCKED_BY_BOOTSTRAP | Rust 1.98.0, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, release build and owned 100% coverage remain required. This execution environment has no Rust toolchain and current #6 has no hosted Product/Rust run, so source commits are not GREEN evidence. |
| Central review plane | OWNER_REPAIR_PENDING | Protected `.github/main` is `efb8926923de45245338159a489a1b227e81945f`. `.github#1929` remains open. Fresh owner evidence preserves three observed producer identities: app-token OpenCode/CodeQL as `opencode-agent[bot]`, a legacy scheduler path as `github-actions[bot]`, and review-fix scheduler dispatches under human `seonghobae`. The least-widening owner repair is to migrate the human-token producer to a repository-scoped machine principal and then authorize only intentionally active machine identities, rather than adding the human account to the machine allowlist. |
| Noema | OWNER_REVIEW_REPAIR_PENDING | `.github#1924` remains open for the contradicted external-Cargo-capability `CHANGES_REQUESTED` on #35. Central failure-artifact capture improves diagnosis but is not adjudication repair. |
| Strix | OWNER_RUNTIME_REPAIR_PENDING | `contextual-orchestrator#1049@87612a68b3af1f305bb7b09bd0be860bad1b7fd6` remains open/non-Draft/mergeable and documents retryable 502/network passthrough failover. The ConceptWeave-observed repeated HTTP-500 path still needs explicit owner acceptance evidence before Strix can be treated as repaired for #35. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Source Observation current contract

`ObservationRequest` accepts only bounded opaque source keys, explicit exact-schema allowlists, a positive caller-requested authorization-metadata budget, and positive operation/statement/row/byte/concurrency limits. Structural positivity and request-local bounds are not source policy. `ObservationResourceEnvelope` combines the metadata and runtime ceilings into one provider-independent policy input.

The local `SourceConnectionRegistry` must issue a bounded opaque immutable connection-policy binding and authorize both the exact schema scope and complete resource envelope against the resulting `ResolvedSourceConnection`. Schema and resource policy default to fail closed. A known key without a binding cannot execute; connection material such as a PostgreSQL DSN is rejected as an invalid binding; source+schema authorization without a trusted resource decision returns `UnauthorizedResourceEnvelope`. A wider-than-policy resource request must fail before adapter/source/snapshot side effects, while equal or narrower requests proceed only through an explicit policy grant.

`AuthorizedObservationRequest` carries only the validated and policy-admitted request, source key, opaque policy binding, and private monotonic operation-start coordinate. Source lookup, binding, schema policy and resource policy all consume the same operation budget before adapter execution. The adapter receives only `remaining_operation_budget()` rather than a reset timeout. A later adapter ACL may resolve credentials only for the exact key-and-binding pair. A capability authorized for revision A must not silently retarget to revision B after the registry changes; the port fixture requires stale-binding failure before source and snapshot side effects and has an unchanged-binding positive control.

`PostgresSchemaSnapshot::new` requires the complete authorized envelope, rejects locally observed table schemas outside the exact authorized allowlist before digest/receipt construction, and retains the authorized policy binding as immutable provenance. Source-content digest identity remains separate from source key and policy revision. Public `SourceObservationReceipt` retains source id, exact policy binding, digest, extractor revision, observation time and verified location. Foreign-key target schemas remain relationship evidence and do not grant read authority for those schemas.

Resource admission now has executable fixtures for three security cases: a registry with source+schema authority but no resource policy is denied by default; any requested ceiling above local policy fails before adapter/source/snapshot side effects; exact-ceiling and narrower controls are explicitly admitted. These commits remain unexecuted specifications/source repairs in this environment until one unchanged exact head passes the Rust/Product evidence suite.

## Central owner evidence relevant to #35

Protected central source is `.github/main@efb8926923de45245338159a489a1b227e81945f` at this snapshot. That head also advances the vendored contextual-orchestrator pin to the merged #1081 retry-stacking repair. `.github#1929` remains open; its latest owner-path evidence warns against collapsing the problem to a two-bot allowlist. The measured review-fix producer still uses human `seonghobae`, while app-token OpenCode/CodeQL uses `opencode-agent[bot]` and a legacy path has emitted `github-actions[bot]`.

ConceptWeave does not edit the central allowlist or replay stale failed handles. Owner acceptance is a migration of the human review-fix producer to an intended least-privilege machine identity, a fresh inventory of any still-live legacy `github-actions[bot]` producer, and then fresh current-central-head OpenCode/CodeQL/review-fix canaries where `actor == sender == exact listed machine identity`, exact repository/PR/base/head/wake metadata binds correctly, substantive work begins, and an otherwise equivalent user-account dispatch remains rejected.

#35 also remains blocked by the separate Noema contradicted-capability review and contextual-orchestrator/Strix HTTP-500 failover lane. These are owner-path blockers for #35 only; they do not justify speculative Source Observation provider fallbacks or weakening ConceptWeave gates.

## P0 product gaps

1. **Exact-head Source Observation verification** — run Rust 1.98 fmt, strict Clippy, tests, warnings-denied rustdoc, release build, owned 100% coverage and applicable security/dependency gates on one unchanged #6 head; repair only observed failures.
2. **Concrete PostgreSQL Source Observation adapter** — maintained patched Rust PostgreSQL driver; exact-binding least-privilege credential resolution; explicit `REPEATABLE READ READ ONLY`; exact-schema `pg_catalog` evidence; one remaining-budget clock across connect/transaction/statements/cancellation; policy-admitted row/byte/concurrency limits; stale-binding rejection; complete immutable snapshot or fail closed; source disappearance; frozen anonymized GRC-shaped replay.
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