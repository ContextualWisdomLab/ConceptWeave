# Product / Technical Gap Baseline

**Snapshot:** 2026-09-06

This file records code-current product and technical gaps. Exact PR/check/run coordinates are evidence snapshots, never mutable-head dependencies. Live protected-branch, PR, issue and workflow state wins whenever it advances after this snapshot.

## Protected truth and active stack

Protected/default `main` remains `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; only the bootstrap state is shipped there and no immutable ConceptWeave release exists.

Current active roots observed for this refresh:

1. Foundation PR #1 — `b538470c963e6524ddc0c3f652a46a4fc8265150`, Draft/open/mergeable. Product CI still cannot originate from protected `main` because `.github/workflows/product.yml` has not yet been integrated.
2. Product-CI bootstrap PR #35 — `a31ae0c2df920f2794f7ddb456795b04797ab472`, open/non-Draft/mergeable on the retained exact source head. Security Scan and SAST have terminal success evidence; existing CodeQL/OpenCode/Strix evidence is not merge-valid; Noema has a blocking `CHANGES_REQUESTED`; no qualifying independent APPROVE has been established.
3. Client Consumption PR #5 — `fcf36c8a99f015b963c9f812787df127ac2e2f9e`, Draft/open/mergeable. It retains deterministic generic release admission, integrity, compatibility, diff/resolution and supersession validation.
4. Source Observation PR #6 — replay-amplification repair advanced ordinarily from predecessor `db209b9b11039ed77cbae246f65b3a83d7589d23` through RED-spec `2a03a56a5982f9d56e880689a139597aea3ef47d`, source repair `340ded102f18c1c4abebbcf0590e5941b61f6cba`, by-value fixture propagation, and code-current architecture/TRD/security/test/operability/ADR/changelog successors through `6b68a23ae4559b6329e336abbcd8177016cc2c9f`; this baseline update creates the next successor head. The stack remains Draft on Client #5 and now carries canonical pre-policy structural schema-metadata caps, source-key + immutable policy-binding + exact-schema + trusted resource-envelope authorization, a non-`Clone` single-use authorized operation capability, one non-resetting operation budget, snapshot-side exact-schema containment, stale-binding fail-closed port fixtures, and binding-preserving immutable snapshot/receipt provenance. No live PostgreSQL adapter or exact-head Rust GREEN is claimed.
5. Zotero Research Classification root #9 and its #13→#38 descendants remain a separately coordinated single-writer lane. This Source Observation writer does not mutate their source/ref/PR metadata.

Predecessor reviews/checks never transfer to successor heads. No force-push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op retrigger, mutable supplier dependency, or routine administrator bypass is acceptance evidence.

## Foundation capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define ConceptWeave ownership of `observe -> discover -> propose -> align -> validate -> review -> publish`, governed immutable semantic releases and stable Client contracts. Foreign product truth remains behind released/versioned ports and ACLs. |
| Truth/publication lifecycle | REPAIRED_PENDING_CI | Rust and public contracts preserve observed/inferred/proposed/authoritative/rejected/superseded distinctions. Protected exact-head Product evidence is still unavailable until bootstrap #35 integrates. |
| Source Observation | ACTIVE_CHILD | Immutable PostgreSQL facts, deterministic content digest, provider-independent hard structural schema-metadata caps, exact-schema authorization, source-policy binding, trusted complete resource-envelope admission, single-use authorization capability, non-resetting deadline, cancellation/resource failures, snapshot containment and policy-binding provenance exist in source. ADR 0004 and refining ADR 0006 remain Proposed because production adapter/runtime evidence does not. |
| Client Consumption | ACTIVE_CHILD | Offline Published+Authoritative admission, compatibility, exact resolution/diff, detached artifact verification and explicit supersession validation exist. Current protected evidence and prerequisite integration remain outstanding. |
| Quality gate | BLOCKED_BY_BOOTSTRAP | Rust 1.98.0, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, release build and owned 100% coverage remain required. This execution environment has no Rust toolchain and current #6 has no hosted Product/Rust run, so source commits are not GREEN evidence. |
| Central review plane | OWNER_REPAIR_PENDING | Protected `.github/main` is `fb2ae81dbeaacb0c630e51e9d772c6919fa220cf`. `.github#1929` remains open. Fresh owner evidence preserves multiple producer identities: app-token OpenCode/CodeQL as `opencode-agent[bot]`, a legacy scheduler path as `github-actions[bot]`, and review-fix scheduler dispatches previously observed under human `seonghobae`. The least-widening owner repair remains migration of any human-token producer to a repository-scoped machine principal and then authorization of only intentionally active machine identities, rather than adding a human account to the machine allowlist. |
| Noema | OWNER_REVIEW_REPAIR_PENDING | `.github#1924` remains open for the contradicted external-Cargo-capability `CHANGES_REQUESTED` on #35. Central failure-artifact capture improves diagnosis but is not adjudication repair. |
| Strix | OWNER_RUNTIME_REPAIR_PENDING | `contextual-orchestrator#1049@87612a68b3af1f305bb7b09bd0be860bad1b7fd6` remained the retained open owner path in the latest verified ConceptWeave evidence; a fresh current-owner Strix terminal result is still required before #35 can treat its historical HTTP-500 failure as closed. |
| Release | NOT_STARTED | No immutable ConceptWeave release exists. Version/CHANGELOG/tag/package/semantic_release/SBOM/provenance/reproducibility/rollback are required on the exact protected release head. |

## Source Observation current contract

Client Consumption's existing `SemanticReleaseClient::verify_detached_artifact` remains current: after release admission it hashes the exact caller-supplied detached immutable artifact bytes and compares their declared digest. Digest syntax is not integrity, and the manifest is not its own detached artifact. The initial Rust 1.98.0 execution of `e3c415600300b6c2d5b852c457ea6ab2e5222e08` found this boundary missing only from this baseline's documentation; the existing documentation contract failed before the new UNIQUE repair. Restore the omitted explanation without changing Client runtime or weakening that test.

`ObservationRequestBudget` enforces a canonical provider-independent hard ceiling before trusted source policy runs: at most 4,096 exact schema identifiers and at most 1,048,576 retained UTF-8 bytes across those identifiers. Over-cap caller requests return typed `SchemaCountLimitTooLarge` or `SchemaByteLimitTooLarge`; exact-cap and ordinary narrower budgets remain constructible. These values bound ConceptWeave's retained authorization metadata against pre-policy resource abuse and do not encode PostgreSQL identifier semantics or grant source authority.

`ObservationRequest` accepts only bounded opaque source keys, explicit exact-schema allowlists, a structurally capped caller-requested authorization-metadata budget, and positive operation/statement/row/byte/concurrency limits. Structural admission and request-local bounds are not source policy. `ObservationResourceEnvelope` combines the metadata and runtime ceilings into one provider-independent policy input, and trusted source policy may only admit an equal-or-narrower effective envelope.

The local `SourceConnectionRegistry` must issue a bounded opaque immutable connection-policy binding and authorize both the exact schema scope and complete resource envelope against the resulting `ResolvedSourceConnection`. Schema and resource policy default to fail closed. A known key without a binding cannot execute; connection material such as a PostgreSQL DSN is rejected as an invalid binding; source+schema authorization without a trusted resource decision returns `UnauthorizedResourceEnvelope`. A wider-than-policy resource request must fail before adapter/source/snapshot side effects, while equal or narrower requests proceed only through an explicit policy grant.

`AuthorizedObservationRequest` carries only the validated and policy-admitted request, source key, opaque policy binding, and private monotonic operation-start coordinate. It is intentionally non-`Clone`, and `SourceObservationPort::observe` consumes it by value. One successful registry authorization therefore crosses the canonical execution seam at most once; cancellation, failure, or success consumes the capability and retry must obtain a fresh authorization against current policy. This closes replay amplification of the admitted row/byte/concurrency/deadline/source-access budget without introducing provider/runtime state.

Source lookup, binding, schema policy and resource policy all consume the same operation budget before adapter execution. The adapter receives only `remaining_operation_budget()` rather than a reset timeout. A later adapter ACL may resolve credentials only for the exact key-and-binding pair. A capability authorized for revision A must not silently retarget to revision B after the registry changes; the port fixture requires stale-binding failure before source and snapshot side effects and has an unchanged-binding positive control.

`PostgresSchemaSnapshot::new` requires the complete authorized envelope, rejects locally observed table schemas outside the exact authorized allowlist before digest/receipt construction, and retains the authorized policy binding as immutable provenance. The adapter may borrow the request while it owns the single-use capability inside one `observe` future. Source-content digest identity remains separate from source key and policy revision. Public `SourceObservationReceipt` retains source id, exact policy binding, digest, extractor revision, observation time and verified location. Foreign-key target schemas remain relationship evidence and do not grant read authority for those schemas.

Replay/resource admission now has executable fixtures for five security layers: canonical structural over-cap rejection before registry access; default denial when source+schema authority has no resource policy; wider-than-policy source-envelope rejection before adapter/source/snapshot side effects; exact-ceiling/narrower positive controls; and compile-contract/source fixtures requiring a fresh authorization per execution. These commits remain unexecuted specifications/source repairs in this environment until one unchanged exact head passes the Rust/Product evidence suite.

## Central owner evidence relevant to #35

Protected central source is `.github/main@fb2ae81dbeaacb0c630e51e9d772c6919fa220cf` at this snapshot. `.github#1929` remains open and the issue still records the core mismatch between app-token `opencode-agent[bot]` dispatch and an allowlist historically holding `github-actions[bot]`; later owner evidence also identified review-fix user-token dispatch. ConceptWeave does not widen that authorization boundary or replay stale failed handles.

Owner acceptance remains machine-principal reconciliation followed by fresh current-central-head OpenCode/CodeQL/review-fix canaries where `actor == sender == exact listed machine identity`, exact repository/PR/base/head/wake metadata binds correctly, substantive work begins, and an otherwise equivalent user-account dispatch remains rejected.

#35 also remains blocked by the separate Noema contradicted-capability review and current-owner Strix evidence. These are owner-path blockers for #35 only; they do not justify speculative Source Observation provider fallbacks or weakening ConceptWeave gates.

## P0 product gaps

1. **Exact-head Source Observation verification** — run Rust 1.98 fmt, strict Clippy, tests, warnings-denied rustdoc, release build, owned 100% coverage and applicable security/dependency gates on one unchanged #6 head, including structural-cap, binding/schema/resource/deadline, snapshot, Debug-privacy and single-use capability coverage; repair only observed failures.
2. **Concrete PostgreSQL Source Observation adapter** — maintained patched Rust PostgreSQL driver; exact-binding least-privilege credential resolution; explicit `REPEATABLE READ READ ONLY`; exact-schema `pg_catalog` evidence; one fresh authorization per attempted observation/retry; one remaining-budget clock across connect/transaction/statements/cancellation; policy-admitted row/byte/concurrency limits; stale-binding rejection; complete immutable snapshot or fail closed; source disappearance; frozen anonymized GRC-shaped replay.
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
