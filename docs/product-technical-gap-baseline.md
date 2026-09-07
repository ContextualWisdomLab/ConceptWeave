# Product / Technical Gap Baseline

**Snapshot:** 2026-09-08 (current top checkpoint; older exact coordinates below are retained as dated evidence)

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after a dated checkpoint.

## September 8 research lifecycle, abstention, and replay-evidence checkpoint

Research Intake PR #9 remains the canonical Zotero/read-only research-classification owner lane on Foundation `b538470c963e6524ddc0c3f652a46a4fc8265150`. Committed lifecycle RED `0a2d63a32fa782c15eb79970213433efba17f0cd` requires separate serialized `truth_status="proposed"` and `publication_state="proposed"`; minimal source repair `72bf15a4200b6496656ce4faaf86faeff779e2ef` projects those proposal-only fields without adding validation, review or publication transitions, a second lifecycle state machine, or a dependency.

A second current semantic-integrity sequence addresses P2 review `PRRT_kwDOUKg5E86fUr-x`. RED `cd791f9c97dfd6048fff44e67bab0414af2aa819` proves that ordinary accented text (`naïve`) and an otherwise ASCII abstract containing scientific symbol `α` must not be labeled unsupported vocabulary merely because they contain a non-ASCII alphabetic character; a wholly non-ASCII alphabetic control remains explicit unsupported vocabulary under the current English deterministic rule set. Minimal repair `67d0d5e55ed77bfcc65ae83a986a0d7dc690dc61` confines `UnsupportedRuleVocabulary` to alphabetic metadata with no ASCII alphabetic signal, leaving rule phrases, disposition families, lifecycle projection, source denominator and governance authority unchanged.

A third current evidence-integrity sequence now has a committed reality RED at `1c8368707cf8c195eb65e6af4801abb3959a07d3` for P1 review `PRRT_kwDOUKg5E86fT2sS`. `abstention_abstract_evidence.rs` requires a nonempty abstract that leads to `NeedsStewardReview` to remain verbatim as local-only `review_abstract_note`, while deterministic matches and empty abstracts must omit that field. Current production `ClassifiedItem` still has no such projection, so this source/test checkpoint is intentionally RED. The accepted minimum source repair is limited to that optional steward-replay field; it must not reinterpret the abstract as matched evidence, alter rule semantics, or grant validation/review/publication authority.

The lifecycle and vocabulary sequences are committed RED -> source repair only, while the replay-evidence sequence is committed RED awaiting its minimal source repair. The Draft PR still has no exact-head Rust/Product execution in this runtime and the local environment has no Rust toolchain. Rust 1.98 workspace tests, fmt, all-target Clippy, warnings-denied rustdoc/release, owned-production 100% function/normalized-region/branch coverage and applicable hosted checks remain mandatory on one unchanged successor head. Earlier per-tag phrase-boundary, repeated DOI-wrapper, CLI and security/durability repairs remain ancestors but do not inherit executable GREEN after later head changes.

Independent evidence-integrity gaps remain separate: DOI duplicate candidates still need original DOI snapshot evidence; the production CLI/source repairs still require successor-head executable coverage; and the three standalone PDFs plus one standalone note remain governed pending sources until exact-snapshot restoration/identity/digest reconciliation. The new abstract replay RED does not satisfy its review until the source projection and exact-head executable gates pass. Zero pending keys, deterministic classification, or local report serialization never grants semantic or write authority.

## September 6 source inventory checkpoint

The existing #9 owner now retains every nonbibliographic metadata record and derives unresolved ancestry rather than silently discarding standalone sources. [Source-scope doctoring](doctoring/zotero_source_scope.md) binds committed REDs, final source `1e95d6eb979e66ecb7dae4f81f18a6b0a91b7624`, **47 tests / 10 unfiltered suites**, strict checks and the unchanged coverage gate. The earlier inventory executable at `48c3525` genuinely reads 8,326 records into 3,715 unchanged bibliographic proposals plus 4,611 other records, with exactly the four previously audited standalone identities pending. A later shared-reader guard also rejects blank identities; no actual final-guard executable replay is implied.

The earlier source findings are repaired locally, not yet propagated into root #39. Required downstream restoration/identity accounting, pending-source reconciliation, approval binding and full-library completion gates remain open; neither zero pending keys nor successful classification grants semantic or write authority. Historical source scope, authentic worksheet decisions/independent approvals 0/3,715, plus four unresolved sources remain distinct. This checkpoint does not imply protected merge or release.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

Fresh September 8 active roots:

1. Foundation PR #1 — exact head `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open/mergeable. Product CI still cannot materialize from protected `main` because that branch does not yet contain `.github/workflows/product.yml`; #35 remains its direct bootstrap prerequisite.
2. Product-CI bootstrap PR #35 — exact head `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft/mergeable. Security Scan and SAST have retained terminal success while historical CodeQL/OpenCode/Strix failures and Noema `CHANGES_REQUESTED` are not current acceptance. Router schedule materialization recovered; `.github#814` owns the exact review-request traversal/exclusion/claim/receipt evidence gap. Keep the leaf head stable unless a real ConceptWeave defect appears.
3. Client Consumption PR #5 — exact head `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open/mergeable on Foundation. It retains provider-independent semantic-release admission, integrity, compatibility, diff/resolution and supersession validation; current protected exact-head evidence remains independently required.
4. Source Observation PR #6 — exact head `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`, Draft/open/mergeable on Client #5. Single-use authorization and PostgreSQL UNIQUE null-comparison semantics are source-repaired with local exact-head Rust evidence; hosted evidence, protected prerequisites and the concrete bounded read-only PostgreSQL adapter remain outstanding.
5. Zotero Research Classification PR #9 — latest source/test checkpoint `1c8368707cf8c195eb65e6af4801abb3959a07d3` is the committed abstention-abstract replay RED, ordinary successor of docs checkpoint `c5b47d740879029357d2c30b557924546c443895`, non-ASCII abstention repair `67d0d5e...`, lifecycle repair `72bf15a...`, repeated-DOI repair `19ae23f...`, per-tag phrase repair `ab425651...`, and the earlier CLI/security/durability repairs. This baseline is an ordinary docs-only successor of that checkpoint; the RED intentionally has no production fix yet and no predecessor executable evidence transfers.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_CI | The canonical domain and semantic-candidate contract separate epistemic truth from governance publication state. Research Intake now projects proposal-only `proposed` values separately, but exact-head Rust/hosted evidence remains required before the P1 can resolve. |
| Research abstention semantics | SOURCE_REPAIRED_PENDING_CI | Ordinary accented/scientific characters no longer alone imply unsupported vocabulary; wholly non-ASCII alphabetic metadata remains a bounded current-rule-set control. Exact-head Rust/coverage evidence remains required before resolving the review. |
| Research abstention replay evidence | REALITY_RED_SOURCE_FIX_PENDING | Source/test checkpoint `1c836870...` commits a public regression requiring nonempty abstention abstracts to be retained only as local steward replay context. Production still lacks the projection; minimal source repair plus exact-head Rust/coverage/hosted evidence is required. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, targeted delete-column provenance, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and registry-authorized opaque source identity exist. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Current exact-head protected evidence and prerequisite integration remain outstanding. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | CONSUMER_REVALIDATION_PENDING | The earlier public non-fork exact-range HTTP 403 was traced to an uninitialized repository dependency graph, not to a retryable central workflow defect. `.github#1873` was closed unmerged after enabling Dependabot vulnerability alerts initialized affected graphs and the same exact comparison returned HTTP 200. The hard gate remains fail closed; a current ConceptWeave head must still execute the pinned Dependency Review action successfully before acceptance. |
| Review / runner admission | BLOCKED_OWNER | #35's Router schedule now materializes and successful organization sweeps exist, but the exact review request still lacks durable request-level visit/exclusion/claim/receipt evidence. `.github#814` owns that central gap; it blocks that validation lane only and is not a reason to stop repository-owned semantic work. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring remains bound to authoritative standards/primary research and exact implementation contracts; hosted exact-head evidence remains independently required after head changes. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Dependency Review incident correction

The prior Foundation predecessor exposed a real hosted failure: the authenticated Dependency Review compare preflight returned HTTP 403 for a public, non-fork ConceptWeave exact range. The initially proposed central repair retried the same token-bound request while retaining fail-closed behavior.

Fresh owner RCA invalidated that causal hypothesis. The same authenticated exact-range request returned HTTP 200 for a repository whose dependency graph was initialized and HTTP 403 for affected repositories whose graph was not initialized. Enabling Dependabot vulnerability alerts initialized the dependency graph in ConceptWeave and pingora-gateway, after which the exact compare endpoint returned HTTP 200. Therefore `.github#1873` was correctly closed without merge: retries would extend queue occupancy but would not establish repository capability.

Acceptance remains stricter than the RCA. HTTP 200 availability alone is not GREEN. A fresh exact ConceptWeave consumer run must reach and complete the pinned Dependency Review action; 403, transport failure, skipped substitution or a sibling scanner cannot satisfy the hard gate.

## Central control-plane evidence

Protected central source is `.github/main@78a4937c684a54ca8e415822c913742f41c6efc4` at this snapshot. This is evidence only, not a mutable ConceptWeave dependency.

- The current central source contains subsequent queue/admission, review-router and CodeQL compatibility repairs. Historical leaf failures are not current ConceptWeave acceptance.
- `.github#1873` is closed/unmerged because repository dependency-graph initialization, not its retry/sleep source delta, was the verified root cause of the observed public-repository 403.
- #35 remains the exact consumer canary. Review Agent Mention Router schedule materialization has recovered, while `.github#814` owns durable exact-request traversal/exclusion/claim/receipt evidence and the current review publication gap.

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
