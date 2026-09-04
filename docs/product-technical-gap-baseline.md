# Product / Technical Gap Baseline

**Snapshot:** 2026-09-04

This file records code-current product and technical gaps. Exact PR/check/run coordinates below are evidence snapshots, not mutable-head dependencies. GitHub protected-branch, PR, issue, and workflow state remains authoritative whenever it advances after this snapshot. A documentation commit necessarily creates a successor Foundation head, so the Foundation SHA below is explicitly the pre-repair head observed immediately before this file was refreshed.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap README is shipped there and no ConceptWeave release exists yet.

The active dependency stack observed immediately before this baseline repair is:

1. Foundation PR #1 — pre-repair exact head `707e687bb510d3f1fd5d1bdc94e590b30ffbb641`, open/non-Draft/mergeable. Product `33850379502` / job `100951576248`, SAST `33850379638`, and Security `33850379525` are queued. Product is on explicit `ubuntu-24.04` with `runner_id=0` and no steps, so the strengthened research-reference test has not yet recorded a real RED. The baseline itself was stale and is the causal reason for this documentation-only successor head; predecessor terminal evidence does not transfer.
2. Client Consumption PR #5 — pre-restack exact head `0f878fc796feb93406dedbbad85351282d3ed7bc`, Draft/open/mergeable and stacked on Foundation. Product `33850422999` / job `100951717500` is pre-runner queued. Detached-artifact API and public-document drift have real hosted RED→repair lineage. The next intended Client RED remains the missing language-neutral supersession/publication JSON Schema plus valid/invalid fixtures, only after the inherited Foundation doctoring lane passes and that Client boundary actually executes.
3. Source Observation PR #6 — pre-restack exact head `7edb2e03c6adeabdc7ffc5c5ecdbc8f4362d3ab3`, Draft/open/mergeable and stacked on Foundation. Product `33850442339` / job `100951774004` is pre-runner queued. The prior UTC provenance RED executed and was repaired. The current registry-identity test requires immutable snapshot/source-receipt provenance to respect the Source Observation port's opaque ≤128-byte lowercase multiword `snake_case` registry-key boundary. Predecessor `c9af2255...` reached a real hosted runner but failed at formatting before the semantic test. Production registry-key validation remains intentionally unchanged until the intended semantic RED executes.

Both child PRs have no submitted reviews or inline review threads in the current fresh read. Foundation's returned inline threads are resolved; its Noema APPROVE belongs to predecessor `bba351b...` and is not current-head independent approval.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases, and the stable Client contract. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | ACTIVE_PR | Rust domain lifecycle enforces explicit governance authorization at steward/publication boundaries, immutable publication and supersession semantics, and evidence-bound authoritative state. Current returned inline review threads are resolved. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL table/column/PK/unique/FK/CHECK evidence, exact identifiers, canonical snapshot digest syntax, UTC provenance, exact receipts, bounded request budgets/cancellation and opaque source registry keys exist. Registry-key consistency at the immutable snapshot boundary is the current TDD lane. No live PostgreSQL adapter is claimed; ADR 0004 remains Proposed. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, explicit compatibility, exact resolution/diff, canonical digest verification, detached artifact verification and explicit supersession validation exist. Language-neutral public supersession/publication schema/fixtures remain intentionally absent pending the real current-head RED. |
| Quality gate | ACTIVE_PR | Rust 1.98.0, unsafe forbidden, public docs required, exact checkout, fmt, Clippy, tests, rustdoc, owned 100% coverage, Draft-2020-12 schema fixtures, lock freshness and clean-tree checks. Every head movement requires fresh exact-head evidence. |
| Security / dependency review | BLOCKED_OWNER | Foundation predecessor Security reached a real runner and failed closed because authoritative GitHub Dependency Review availability was not satisfied. `.github#810` owns the central repair; scanner substitution and 403-as-success are forbidden. |
| Review / runner admission | BLOCKED_OWNER | Central queue pressure has fallen materially, but current ConceptWeave Product canaries remain unassigned. `.github#712/#1531/#1796` own the central admission/review-amplification paths. Queue depth alone is not consumer GREEN. |
| Standards / research | ACTIVE_PR | Stable recommendations remain distinct from drafts. The current doctoring RED contract binds adopted Model Alignment evidence to authoritative bibliography records: He et al. to CEUR/ISWC 2023 and Amini et al. to the Springer LNCS version of record published in 2025 while retaining the KGSWC 2024 study/conference lineage in traceability. `REFERENCES.md` remains intentionally unchanged until the test executes and records that RED. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Central control-plane evidence

Protected central source is `.github/main@07db37e5e42c63ba40ac66f22ef74e4f8836ce9a` at this snapshot. This is evidence only, not a ConceptWeave dependency.

- `.github#1796` queue-ownership repair #1821 is integrated. Organization sweep no longer owns repository-wide queued/in-progress Actions inventory or broad cancellation; native per-PR concurrency and repository-local exact-head coalescing own supersession.
- `.github#1840` removed required-check-completion `workflow_run` fanout from the merge scheduler while retaining native PR/review/protected-branch-push/explicit-dispatch events and bounded recovery. GitHub auto-merge owns required-check completion rather than another scheduler run.
- `.github#1841` made uncovered CodeQL repositories self-remediating through idempotent OpenCode-owned setup PRs created from trusted central main.
- `.github#1842` keeps those generated repository-local CodeQL workflows off pull-request heads, leaving central `codeql-pr.yml` as the single PR-head scanner and isolating audit concurrency by trigger.
- Fresh `.github` queued inventory is `271`, materially below earlier `991`, `914`, and ~`1,900` observations. This is diagnostic progress, not acceptance: current ConceptWeave Product jobs still lack runner identity, exact checkout, steps, and terminal evidence.

## P0 product gaps after the current TDD lanes

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
