# Product / Technical Gap Baseline

**Snapshot:** 2026-09-04

This file records code-current product and technical gaps. Exact PR/check/run coordinates below are evidence snapshots, not mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot. Because this documentation update itself creates a successor Foundation head, the Foundation SHA below is explicitly the exact pre-refresh head.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists yet.

The active dependency stack observed immediately before this baseline refresh is:

1. Foundation PR #1 — pre-refresh exact head `9a6aa93ed05dd9cc56825258e072b222d80f85de`, open/non-Draft/mergeable. A public-contract mismatch was reproduced locally before the repair: the Draft 2020-12 `semantic-candidate` schema accepted `publication_state=draft` together with `truth_status=authoritative`, although the Rust domain maps Draft/Validated/Reviewed to Inferred, Proposed to Proposed, Published to Authoritative, Superseded to Superseded and Rejected to Rejected. Test-first `18249652...` added an invalid fixture and Product AJV assertion; minimum fix `9a6aa93...` makes the language-neutral schema enforce the same state/truth mapping as Rust. Local exhaustive verification covered all 7 publication states × 6 truth statuses with zero mapping mismatches. This is local exact-contract GREEN, not hosted exact-head GREEN. Product `33871067722` / job `101016914449`, SAST `33871067759`, and Security `33871067702` remain non-terminal; Product has no executed steps yet.
2. Client Consumption PR #5 — pre-refresh exact head `4a771af962306febb4318aed4de48254d96f32f9`, Draft/open. It non-force adopted the Foundation truth-state contract. The first restack accidentally replaced the child-specific Product JSON-contract checks with the narrower Foundation workflow; that repair finding was fixed immediately by `4a771af...`. Old-child `67c104...` → current comparison now changes only the new candidate mismatch fixture, the candidate schema and five added Product-workflow lines, while all pre-existing semantic-release and supersession checks remain intact. Product `33871312459` is non-terminal. The next Client RED remains its missing language-neutral supersession/publication schema and fixtures after inherited Foundation quality is terminal.
3. Source Observation PR #6 — pre-refresh exact head `e72345ac9f407e5732b3e3cc5a2d78b55b10cad2`, Draft/open. It non-force adopted the same Foundation contract; old-child `a0197ba...` → current comparison changes only the candidate mismatch fixture, candidate schema and five Product-workflow lines. Source Observation semantic delta is preserved. Product `33871231617` is queued. The registry-identity test still requires immutable snapshot/source-receipt provenance to obey the Source Observation port's opaque ≤128-byte lowercase multiword `snake_case` key boundary; production `PostgresSchemaSnapshot::new` still validates this field only as nonblank, so that semantic lane remains open.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, fail-open scanner substitution or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust already derives truth status from publication state. The public JSON Schema now enforces the same mapping for every state, preventing pre-publication `authoritative` claims by non-Rust consumers. New invalid fixture proves the previously admitted Draft+Authoritative combination. Hosted exact-head Product evidence is still non-terminal. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, canonical snapshot digest syntax, UTC provenance, receipts, bounded request budgets/cancellation and opaque source registry keys exist. Registry-key consistency at the immutable snapshot boundary is the current semantic TDD lane. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Public supersession/publication schema/fixtures remain the next Client contract lane after Foundation terminal quality. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | BLOCKED_OWNER | Prior Security evidence showed authoritative GitHub Dependency Review availability was not satisfied. `.github#810` owns central repair; scanner substitution and 403-as-success are forbidden. |
| Review / runner admission | BLOCKED_OWNER | Central queue pressure is materially lower than its peak but current ConceptWeave canaries remain non-terminal. `.github#712/#1531/#1796` own central admission/review-amplification. Queue depth alone is not consumer GREEN. |
| Standards / research | REPAIRED_PENDING_CI | Doctoring binds He et al. to CEUR/ISWC 2023 and Amini et al. to the Springer LNCS 15459 version of record published in 2025 while retaining KGSWC 2024 study/conference lineage in traceability. Hosted exact-head evidence remains non-terminal. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Central control-plane evidence

Protected central source is `.github/main@c31d2e5471fc5daf9d72ff67cde6a8874b736deb` at this snapshot, after merged #1852 aligned current-main workflow contracts. This is evidence only, not a ConceptWeave dependency.

- `.github#1821` queue-ownership repair remains integrated: organization sweep no longer owns repository-wide queued/in-progress Actions inventory or broad cancellation; native per-PR concurrency and repository-local exact-head coalescing own supersession.
- Later consolidation removed merge-scheduler required-check fanout, centralized CodeQL PR ownership and consolidated empty-PR/quality lanes.
- Fresh `.github` queued inventory is `245`. This is far below the ~1,900 peak but above some recent lower observations, so it is neither terminal recovery nor consumer acceptance.

## P0 product gaps after the current TDD lanes

### Zotero research classification slice

Local evidence on 2026-09-04 showed Zotero 9.0.6, Local API v3/schema 42, library version 12341, 8,326 total items, and 3,719 top-level items. The corrected read-only run observed all 8,326 records at that single version and classified all 3,715 top-level bibliographic records; four top-level note/attachment/annotation records were correctly excluded. It proposed 56 adjacent-evidence records, 1 semantic-consumption bridge, and 3,658 steward-review abstentions, linked children for 3,287 records, and surfaced 49 reversible duplicate groups (18 DOI, 31 title). No live record matched multiple specific disposition families; the tested conflict path still abstains fail-closed. Token-boundary matching prevents strings such as `knowledge` from becoming false OWL evidence. These are local aggregate observations, not reviewed truth or applied Zotero changes. The report stays outside the repository.

The golden-set evaluation contract now records aggregate precision/recall numerators and denominators, requires an externally verified governance receipt bound to the complete item-key/item-version snapshot, rejects abstention as expected truth, and retains verified revisions plus an opaque snapshot digest so detached metrics remain attributable. Item and reviewer identities stay out of its output. Successful classification reports also carry same-snapshot aggregate coverage, provenance, abstention, duplicate, disposition, and failure evidence. Connected duplicate components now produce a snapshot-bound local review manifest only after external steward verification; every operation retains all component source revisions and before/after/rollback canonical mappings while Zotero records remain unchanged. Reviewed collection/tag changes can produce a default-dry-run plan bound to exact server, library, item, rule, digest, and complete metadata preconditions; it preserves automatic-tag type and rejects Zotero 9 execute mode. Synthetic fixtures verify these contracts. No real precision/recall, duplicate merge, or write claim exists until a steward supplies reviewed local decisions and a production authorization adapter verifies them. AC6 still requires a Zotero 10+ transport plus approved live partial-failure and rollback evidence. Multilingual rule expansion remains a later evidence-driven change and must not reduce abstention safety. A dedicated utility repository remains unnecessary until an independently released cross-product contract exists.

1. **Concrete Source Observation adapter** — maintained Rust PostgreSQL driver behind `conceptweave-source-port`; adapter-local credential resolution; explicit read-only mode; statement timeout, cancellation, row/byte/concurrency budgets; complete immutable snapshot or fail closed; deterministic replay against a frozen anonymized GRC-shaped fixture.
2. **Ontology discovery** — deterministic term/concept/taxonomy/non-taxonomic-relation candidate generation with exact source receipts and abstention for unsupported semantics.
3. **Semantic-layer discovery** — dimensions, measures, grain, units, relationships and physical mappings with deterministic calculation contracts; do not infer business authority from relational structure alone.
4. **LLM Proposal** — every production model call through released `contextual-orchestrator`; outputs remain proposed/inferred and preserve source/model/prompt/provenance evidence.
5. **Alignment / matching** — retrieval/pruning/structural evidence first, bounded optional LLM assistance, OAEI-style evaluation, deterministic reproducibility and steward-visible decisions.
6. **Validation engine** — RDF/OWL/SKOS/SHACL and semantic-layer validation, consistency/conflict/duplicate detection, bounded reasoning, explicit unsupported-feature failure.
7. **Governance persistence** — PostgreSQL 3NF candidates/evidence/validation/review/release/supersession receipts, transactional outbox and temporal history only where domain semantics require it.
8. **Review workflow** — Keyverse identity context, tenant/role/purpose authorization, steward decisions, maker-checker where required, stale-decision protection and immutable publication receipt.
9. **Publication adapters** — versioned OWL/RDFS/SKOS/SHACL/JSON-LD plus explicitly version-bound Apache Ossie export; draft/incubating formats cannot be presented as final standards.
10. **Client completion** — language-neutral release/supersession contract, provenance/signature verification, relation/mapping/dimension/measure resolution, compatibility/deprecation, match/explain/query-plan contracts while downstream products retain physical authorization/execution.
11. **CWL integration** — only released/versioned semantic_release/contract/ACL seams to `semantic-data-portal`, `context-graph-contracts`, GRC, EA and other consumers; no source copying, cross-service SQL or mutable supplier heads.
12. **Evaluation / multilingual** — reviewed golden fixtures, ontology-learning/matching metrics, source-evidence binding, abstention, reproducibility, KO/EN/JA/ZH/VI/ES/DE/FR labels, CJK/font/text-expansion checks where UI or published labels are material.
13. **Observability / recovery / release** — structured telemetry, security evidence, backup/restore, package/SBOM/provenance/signing, reproducible build and rollback proof before immutable release.

## DDD fitness constraints

- No generic `utils/helpers/services/common` domain buckets.
- Adapters remain outside the core domain model; external DTOs cross Anti-Corruption Layers.
- Source Observation facts are not source-system business truth, and relational constraints are not semantic authority by themselves.
- Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state.
- `semantic-data-portal` remains catalog/governance/consumption rather than ConceptWeave persistence; `context-graph-contracts` owns interop contracts; `enterprise-architecture-core` owns EA; `contextual-orchestrator` owns provider routing.
- Consuming products retain tenant/purpose authorization and physical query execution.
- Published semantic truth is immutable; corrections create a new release plus supersession evidence rather than in-place overwrite.
