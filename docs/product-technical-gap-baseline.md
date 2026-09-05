# Product / Technical Gap Baseline

**Snapshot:** 2026-09-05

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot. Because this documentation update creates a Foundation successor, the Foundation SHA below is the exact pre-refresh head; PR metadata must be refreshed to the resulting successor SHA.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

The active roots observed immediately before this baseline refresh are:

1. Foundation PR #1 — pre-refresh exact head `5cdd319b9425989e632149b243a3308dd630c0ae`, Draft/open/mergeable. The current Foundation makes Product CI draft-aware while keeping Ready/non-Draft quality requirements intact. Product CI still cannot materialize from protected `main` because that branch does not yet contain `.github/workflows/product.yml`.
2. Product-CI bootstrap PR #35 — exact head `daa543ce2cc2b2eb6d35a7265abcf2a7466e7381`, open/non-Draft/mergeable. It adds only the pull-request form of Product CI so #1 can later be marked Ready without a no-op commit. Exact-head CodeQL PR, Security Scan and SAST Semgrep remain queued; `Security Scan / Detect changed scope` is pre-runner with no steps and no runner assignment, and no independent submitted review exists yet.
3. Client Consumption PR #5 — exact head `cbb9cda0c93d8b762195423834f1d6a27dbfa613`, Draft/open/mergeable. The current source retains language-neutral semantic-release admission, integrity, compatibility, diff/resolution and supersession validation. Previously valid review findings are source-repaired, but current protected evidence remains independently required.
4. Source Observation PR #6 — exact head `d255f5c08a621024809c7e076989eccf0662a330`, Draft/open/mergeable. PostgreSQL targeted `ON DELETE SET NULL (...)` / `SET DEFAULT (...)` column provenance and registry/ACL-resolved source identity are source-repaired. The next P0 slice is the concrete bounded read-only PostgreSQL adapter.
5. Zotero Research Classification root PR #9 — exact head `cda546672cd95b5f8bed7024f70e4e6b39a134c8`, Draft/open/mergeable. The dependent research/write-back stack remains proposal/review oriented and does not elevate local classifier output to semantic authority.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and the public Draft 2020-12 semantic-candidate schema enforce compatible publication-state/truth-status semantics. Hosted exact-head Product evidence still requires the bootstrap workflow on protected `main`. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, targeted delete-column provenance, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and registry-authorized opaque source identity exist. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Current exact-head protected evidence and prerequisite integration remain outstanding. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | CONSUMER_REVALIDATION_PENDING | The earlier public non-fork exact-range HTTP 403 was traced to an uninitialized repository dependency graph, not to a retryable central workflow defect. `.github#1873` was closed unmerged after enabling Dependabot vulnerability alerts initialized affected graphs and the same exact comparison returned HTTP 200. The hard gate remains fail closed; a current ConceptWeave head must still execute the pinned Dependency Review action successfully before acceptance. |
| Review / runner admission | BLOCKED_OWNER | #35's exact-head central runs are still queued before useful execution; `Detect changed scope` has no runner assignment or steps. Queueing blocks this validation lane only and is not a reason to stop Source Observation or other repository-owned work. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring remains bound to authoritative standards/primary research and exact implementation contracts; hosted exact-head evidence remains independently required after head changes. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Dependency Review incident correction

The prior Foundation predecessor exposed a real hosted failure: the authenticated Dependency Review compare preflight returned HTTP 403 for a public, non-fork ConceptWeave exact range. The initially proposed central repair retried the same token-bound request while retaining fail-closed behavior.

Fresh owner RCA invalidated that causal hypothesis. The same authenticated exact-range request returned HTTP 200 for a repository whose dependency graph was initialized and HTTP 403 for affected repositories whose graph was not initialized. Enabling Dependabot vulnerability alerts initialized the dependency graph in ConceptWeave and pingora-gateway, after which the exact compare endpoint returned HTTP 200. Therefore `.github#1873` was correctly closed without merge: retries would extend queue occupancy but would not establish repository capability.

Acceptance remains stricter than the RCA. HTTP 200 availability alone is not GREEN. A fresh exact ConceptWeave consumer run must reach and complete the pinned Dependency Review action; 403, transport failure, skipped substitution or a sibling scanner cannot satisfy the hard gate.

## Central control-plane evidence

Protected central source is `.github/main@b5efbc2762e472e4a380b0503b1f050f76fbb008` at this snapshot. This is evidence only, not a mutable ConceptWeave dependency.

- The current central source includes queue/admission and changed-scope/review-runtime repairs already integrated through ordinary protected history.
- `.github#1873@41935494aa234eb458f1cc08f006daaa278b9760` is closed/unmerged because repository dependency-graph initialization, not its retry/sleep source delta, was the verified root cause of the observed public-repository 403.
- #35 remains an exact consumer canary for current runner admission and Dependency Review behavior. Its central workflows are queued, so no protected recovery or dependency-review success is inferred from repository settings alone.

## P0 product gaps

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local registry/credential resolution; explicit read-only session/transaction; exact schema allowlist; total operation and statement deadlines; cancellation plus row/byte/concurrency budgets; complete immutable snapshot or fail closed; source-disappearance handling; deterministic replay against a frozen anonymized GRC-shaped fixture.
2. **Observed PostgreSQL surface completion** — domains/enums/indexes/comments, quoted identifiers and cross-schema collisions as generic observed evidence without importing source-system business truth.
3. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
4. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; do not infer business authority from relational structure alone.
5. **LLM Proposal** — every production model call through a released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
6. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
7. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning and explicit unsupported-feature failure.
8. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
9. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
10. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
11. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
12. **CWL integration** — only released/versioned `semantic_release`/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
13. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
14. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
