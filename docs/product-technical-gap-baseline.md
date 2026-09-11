# Product / Technical Gap Baseline

**Snapshot:** 2026-09-10

This document records ConceptWeave's code-current product and technical gap baseline. Exact SHA/run coordinates are immutable evidence snapshots, never mutable supplier dependencies. Live protected branch, PR, issue, review, and workflow state supersedes recorded coordinates when it advances. Any head movement resets exact-head execution/review evidence unless the evidence was actually produced for that successor.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical release-consumption contract. Source-system business truth stays with its canonical owner.

- `semantic-data-portal`: catalog/governance/consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- consuming products: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and active stack

Protected/default ConceptWeave `main` is `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; it remains repository bootstrap only and no immutable ConceptWeave semantic release exists.

1. Foundation #1 is `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft/mergeable on protected `main`.
2. Product-CI bootstrap #35 is `22709ec9b4d969bf67ec74db402813e74d11f7ca`, OPEN non-Draft/mergeable on protected `main`. Its recorded leaf Security/SAST evidence succeeded; CodeQL remains blocked in the central owner path.
3. Client Consumption #5 is `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, OPEN Draft/mergeable on current Foundation after ordinary two-parent non-force reconciliation. Predecessor execution/review evidence does not transfer.
4. Source Observation #6 was exact `18d882088d95c4f6b6aee77e2f8bdbe5b73586fb`, OPEN Draft/mergeable on current Client, immediately before this baseline-only successor. Ordinary reconciliation at `b614fddc3331365da84733f660a429ed71b83182` preserved pre-restack source `331f8edcd7cebb1719e5cea3187f3848ce7b9e71`; later movement is active-gap documentation. No open PR currently targets #6 as its base, so this documentation successor does not create a dependent-branch restack.
5. Research Intake #9 is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`, OPEN Draft/mergeable on Foundation. Pending-source resolution #40 is `4efe15c6318d8cb65c52a974a2c105363a4c82a5`, OPEN Draft/mergeable on #9.
6. Golden-set evaluation #10 is `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4`, OPEN Draft/non-mergeable from historical #9 merge base `51c7df6d03f072449422fd58ca24b2f9d6026f07`; it is 35 commits ahead and 81 behind current #9. #11 `6dff8c2ee42cfeb7bf8688c1f7e95989b61be266` and later descendants remain dependent on a semantic non-force #10 repair.
7. Later steward/full-text research remains preserved but is not independent root authority. #33 is `93faf6ab750a99469196cf71567498be83c22a6b`; #34 is `c51330e61bf5b3d2b18a561830151ba874b17a4c`, OPEN Draft/mergeable on #33. #34's live ancestry already carries normally merged #36 full-text-capture work. Terminal open #39 remains `aca2fe603477453fee071679a8aefef0cd784dd3`, OPEN Draft/mergeable on #38 `7678236ed3ec467e93b97bb2ad7ad26b3dc0e5b9`.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op retrigger, synthetic status, mutable supplier dependency, or routine administrator bypass is acceptance evidence.

## Product-CI and central CodeQL prerequisite

ConceptWeave #35 remains the direct bootstrap prerequisite because protected `main` does not yet contain the repository-owned Product pull-request workflow.

Protected central `.github/main` has advanced normally to `f578d8d960177ff113c25fd740619b4a483df300`. That intervening protected delta is current owner truth and must be adopted rather than treated as a race or ignored because an older PR remains mechanically mergeable.

The broad CodeQL producer/handler successor remains `.github#2040@6706c231ab06a3c91c43fdb5b989cfcd79fff593`. Fresh ancestry comparison against protected `f578d8d...` is `diverged`: #2040 is 144 commits ahead and 11 commits behind, with merge base `7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`. GitHub may report the PR mergeable when the trees are conflict-free, but that does not make old-base exact-head evidence current. Its valid #1902/#2004/#2043/#2044 producer, stacked-check, settlement, SARIF and base-bound evidence work must be preserved while the complete protected delta is adopted by ordinary non-force semantic integration. Review `5161498951` records this current stale-base repair contract. Do not close, force-push, destructively rebase, or select an entire side of the tree merely because protected main advanced.

The exact hosted generation on `#2040@6706c231...` is historical old-base evidence, not acceptance for a reconciled successor. Security Scan `34251822390`, SAST `34251822314`, Python Security `34251822251`, and Agent Review Runtime Quality `34251822381` were terminal success; CodeQL PR `34251822255` was terminal failure and is explicitly bound to the historical base. Its `Dispatch current-head CodeQL scan` job took one Jobs API snapshot and then failed with `CodeQL coordinator could not bind every pending language to an exact failed job.` This remains a valid reality RED for job-set convergence, but every execution/review lane must regenerate after current-base reconciliation.

The two verified CodeQL owner findings remain repair requirements to preserve and re-evaluate on that successor rather than reasons to bypass ancestry repair:

1. **Pre-cutover evidence seam.** Transitional compatibility may accept only direct evidence authenticated to the exact protected handler source plus repository/PR/base/head/required-run/language/job/SARIF/artifact identity. Do not restore creator-only legacy status trust. Stale, ambiguous, cross-base and cross-run evidence remains non-passing.
2. **Required-run job-set convergence.** Recovery must boundedly reread the exact required run until every detected language has exactly one stable terminal rerunnable job identity, or fail closed at a deterministic deadline. Partial/mixed inventories must not dispatch or wake a run.

`.github#2051@558693e0333e48012beea142f739bc634b0674a7` remains OPEN Draft and is independently `diverged` from protected `f578d8d...`: 18 commits ahead and 11 behind with the same historical merge base `7fd571db...`. Review `5161499908` requires ordinary non-force adoption of current protected truth while preserving the one-coordinator wake, exact PR/head/base/run evidence binding, stricter `{base_ref, base_sha}` identity and versioned backward-compatible handler rollout. `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remains OPEN non-Draft/mergeable on #2051 and preserves complete-failed-job-set validation plus atomic wake logic; review `5161501428` requires it to remain stacked and adopt the repaired exact #2051 successor afterwards rather than independently restacking to protected main.

Canonical central order is: ordinary non-force adoption of current protected `.github/main` by the active CodeQL owner chain; RED->GREEN repair/revalidation of the authenticated pre-cutover evidence seam and bounded required-job-set convergence; semantic reconciliation of the valid #2051/#2056 identity/wake deltas without duplicate ownership; exact-current-base terminal GREEN, zero valid unresolved findings and qualifying independent review; normal protected integration. Keep #35 stable during that owner repair, then obtain fresh #35 exact-head CodeQL/review evidence before normal merge. Manual/no-op reruns or leaf source churn do not repair the owner contract.

## Source Observation current contract

Pre-restack Source Observation `331f8ed...` locally executed Rust 1.98 evidence: 132 tests across 42 suites including two doctests; fmt; strict Clippy; warnings-denied rustdoc; release build; Product CI contract/schema/fixture checks; normalized owned coverage 228/228 functions, 2,026/2,026 regions, and 194/194 branches. Raw LLVM diagnostics remained below 100% because source-embedded test/generic instrumentation is reported separately. Those results prove `331f8ed...` only and do not transfer to #6's current or later documentation/source successors.

`ObservationRequestBudget` and the policy-admitted resource envelope bind exact-schema authorization metadata plus runtime row/byte/concurrency/deadline limits. `AuthorizedObservationRequest` is intentionally single-use at the execution seam. Retry after cancellation/failure/success requires a fresh authorization decision. Source lookup, policy binding, schema/resource authorization, adapter execution and cancellation consume one non-resetting operation budget; a concrete adapter may resolve credentials only for the exact authorized key-and-binding pair. Stale binding fails before source access.

The current owner digest/receipt vocabulary preserves deterministic table/column/constraint evidence, exact identifiers, column comments, PK/UNIQUE/FK/CHECK evidence, UNIQUE NULL-comparison state, FK reference behavior, targeted `SET NULL`/`SET DEFAULT` columns, and observed validation/enforcement state. It does not yet losslessly represent all material PostgreSQL 18 evidence required by the planned adapter.

Current public representation also has two versioning constraints that must not be silently broken:

- digest framing is explicitly `conceptweave.postgres_schema_snapshot.v2`, which currently hashes table/column/constraint evidence only;
- `ObservationLocation` requires `schema_name + table_name`, `ObservationLocationKind` exposes only Table/Column/Constraint, and receipt canonical locations always traverse `/schemas/{schema}/tables/{table}`.

## PostgreSQL 18 representation/version/authorization prerequisite

The next Source Observation P0 is representation before transport. Do not add the concrete PostgreSQL adapter while the owner model would silently discard or ambiguously bind source facts.

The minimum successor adds deterministic owner value objects and collision-safe receipt coordinates for:

- relation kind and relation/table comments;
- first-class indexes preserving key versus INCLUDE attributes, expression positions, partial predicates, NULL uniqueness semantics, and readiness/validity/liveness;
- qualified domains and enums as schema-scoped objects rather than manufactured table children;
- explicit column-to-qualified-type evidence so a column binds to the exact built-in/domain/enum coordinate rather than a search-path-dependent display string;
- domain semantics needed for semantic identity: qualified base type, relevant type modifier/array dimensions, collation, NOT NULL/default state, and domain CHECK constraints with validation/enforcement evidence where exposed;
- enum label membership and ordering;
- server-rendered `pg_get_constraintdef`, `pg_get_indexdef`, and `pg_get_expr` text labeled as reconstructed source evidence, never original DDL.

Authoritative PostgreSQL 18 catalog basis:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_class`*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_index`*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_type`*. https://www.postgresql.org/docs/18/catalog-pg-type.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_enum`*. https://www.postgresql.org/docs/18/catalog-pg-enum.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_constraint`*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: `pg_description`*. https://www.postgresql.org/docs/18/catalog-pg-description.html

`pg_type.typnamespace + typname` identifies qualified types and `typtype` distinguishes domains/enums. Domain material semantics include `typbasetype`, `typnotnull`, `typtypmod`, `typndims`, collation/default data and domain constraints linked by `pg_constraint.contypid`. `pg_enum` stores labels and `enumsortorder`; enum row OIDs are catalog join coordinates and must not be mistaken for stable governed semantic identity. `pg_index.indnkeyatts` distinguishes key from included attributes, zero `indkey` positions denote expressions, and index readiness/validity/liveness and partial predicates are independently material. `pg_description` stores object comments. `pg_class.relkind` distinguishes relation kinds.

### Digest and receipt compatibility

Review `5158484289` established that v2 must remain reproducible. Do not redefine the immutable v2 receipt family by appending new material fields under the same domain. Introduce successor digest framing (`v3` or an explicit equivalent) and bind every newly material source fact there.

Review `5160201671` established that schema-scoped objects need a backward-compatible coordinate seam. Do not make the existing `table_name() -> &str` nullable and do not invent sentinel tables. Historical v2 table/column/constraint locations retain identical meaning. A versioned/tagged successor coordinate can represent genuine schema-scoped locations such as `/schemas/{schema}/domains/{name}` and `/schemas/{schema}/enums/{name}` with the existing escaping guarantees.

Review `5160252371` adds a semantic binding prerequisite: current `ColumnObservation` stores only unqualified `data_type: String`. Creating qualified domain/enum objects without a qualified column type reference leaves the evidence graph ambiguous. The successor must keep presentation text separate from identity and bind each column to the exact immutable built-in/qualified source-type coordinate. Same-named types in two allowed schemas must remain distinguishishable even if existing `data_type()` text is identical. Do not infer a type from `search_path` during validation.

Review `5174202474` establishes that relation-level successor coordinates must carry the exact observed `pg_class.relkind` rather than reusing table vocabulary. A v3 relation location is `/schemas/{schema}/relations/{kind}/{name}` with `/columns/{name}`, `/constraints/{name}`, and `/indexes/{name}` children, so views, materialized views, foreign tables, sequences, and composite types stay distinguishable from tables at the same qualified name. Receipt lookup must fail closed when the observed relation kind differs, and the kind segment must not alter v2 `/tables/` meaning or any digest.

### Authorization

Current snapshot construction enforces schema scope by iterating observed tables. Schema-scoped domain/enum evidence must be checked directly against the exact `AuthorizedObservationRequest` allowlist, including a schema with zero observed tables. Otherwise a type-only schema could bypass the existing table-driven containment invariant.

### Executable RED before production representation code

- a frozen historical v2 fixture reproduces its original v2 digest and table/column/constraint coordinate meaning exactly;
- otherwise-identical successor snapshots differing in one newly required material PostgreSQL fact have distinct v3 identities;
- every new evidence kind has a verified receipt coordinate;
- unauthorized domain/enum-only schema fails before immutable snapshot/receipt side effects;
- authorized type-only schema succeeds;
- same-name types in different allowed schemas remain distinct and columns resolve to their exact qualified type coordinate;
- enum label/order changes alter successor identity;
- material domain base/default/null/collation/check-constraint changes alter successor identity;
- a schema-scoped receipt cannot be satisfied merely because an unrelated table exists in that schema;
- index key/INCLUDE role, attribute position and expression-versus-column form, partial predicate, NULL uniqueness semantics, readiness/validity/liveness, access method, and reconstructed definition changes alter successor identity;
- index receipt coordinates cannot be satisfied by a same-named constraint, an unrelated relation, or a missing index;
- relation-level and relation-child coordinates preserve the exact observed relation kind, so a non-table relation never receives a table-labelled receipt;
- fake table-scoped type coordinates fail;
- input-order permutations of identical complete evidence remain digest-identical.

Issue #2 and #6 reviews `5160201671` / `5160252371` are the live acceptance authority for this slice.

Only after this representation/version/authorization/type-binding slice is exact-head GREEN should the concrete adapter be admitted behind `conceptweave-source-port`: maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate, cargo-deny/SBOM review, least-privilege credential resolution from exact authorized key+binding, stale-binding rejection before credential/source I/O, one fresh authorization per attempt, explicit `REPEATABLE READ READ ONLY` catalog transaction, exact-schema `pg_catalog` capture, one remaining-operation budget across connect/transaction/query/cancellation, policy-admitted row/byte/concurrency ceilings, complete-or-fail snapshot construction, source disappearance handling, and deterministic replay against a frozen anonymized GRC-shaped fixture. `governance-risk-compliance` retains its business truth; no cross-service application-table SQL is introduced.

## Research stack repair

Current #9 is canonical Research Intake. #10 still carries useful golden-set source/test/fixture/docs delta, but its historical parent predates #9's Foundation reconciliation. Both lineages changed `crates/conceptweave-zotero/src/lib.rs`, `crates/conceptweave-zotero/tests/review_contract.rs`, `review_contract_followup.rs`, and this baseline. Whole-tree ours/theirs would discard valid work.

Repair #10 by ordinary non-force semantic integration. Preserve current #9's private/constructor-bound `ClassificationReport` and read-only accessors. Do not reopen trusted aggregate fields to satisfy older #10 tests; corruption cases belong behind an internal test seam or explicitly untrusted wire/fixture. Likewise do not impose #10's mandatory caller-owned `ZoteroItem.source_record` as authenticity evidence; raw provider data needs a backward-compatible capture/wire boundary. Then propagate the repaired parent through #11+ without transferring predecessor GREEN.

Issue #8 has been refreshed to this root authority and to the live later research coordinates. #34's PR body has also been repaired to exact `c51330e...` after live ancestry showed merged #36 full-text capture was already carried into the branch.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define canonical ConceptWeave ownership and foreign-owner seams. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | Observed/inferred/proposed/authoritative/rejected/superseded distinctions exist; no protected immutable semantic release exists. |
| Client Consumption | RESTACKED_HOSTED_PENDING | #5 consumes current Foundation through ordinary non-force ancestry; exact-head execution/review must regenerate. Its existing `SemanticReleaseClient::verify_detached_artifact` remains current: after admission it hashes the exact caller-supplied detached immutable artifact bytes against the declared digest, keeping digest syntax distinct from byte-integrity evidence. |
| Source Observation | REPRESENTATION_V3_P0 | #6 is stack-current; PostgreSQL representation needs versioned digest/receipt + direct schema authorization + qualified column/type binding RED->GREEN before transport. |
| Research Intake | RESTACKED_HOSTED_PENDING | #9 is current; predecessor local evidence is historical only. |
| Golden-set evaluation | STALE_PARENT_REPAIR_P1 | #10 is non-mergeable and 81 commits behind current #9; preserve and semantically reconcile rather than close. |
| Steward/full-text stack | ROOT_PROPAGATION_PENDING | Valid later deltas exist, but current #10 root repair must propagate before independent readiness claims. |
| Product CI | BLOCKED_OWNER_RESTACK | #35 waits on central protected-base reconciliation, preservation/revalidation of the CodeQL causal repairs, exact central GREEN and downstream exact-head evidence. |
| Quality gate | ACTIVE | Rust 1.98, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, owned production coverage, fixture/schema/lock/clean-tree checks; every head movement resets exact-head acceptance. |
| Security / review | PENDING_EXACT_HEAD | Scanner/reviewer status is accepted only when bound to exact current head and applicable protected policy. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory on the exact protected release head. |

## Remaining commercial/semantic P0 sequence

1. Finish Source Observation PostgreSQL 18 lossless representation with successor digest versioning, backward-compatible schema-scoped coordinates, direct type-only-schema authorization, qualified column/type binding, and exact-current Rust/coverage evidence.
2. Add the concrete bounded read-only PostgreSQL adapter and frozen anonymized conformance fixture.
3. Repair #10's stale Research Intake parent without discarding its golden-set delta, then propagate that exact repaired ancestry through #11+ and still-relevant later research/write Drafts.
4. Build deterministic ontology discovery with source receipts and explicit abstention for unsupported semantics.
5. Build semantic-layer discovery for dimensions/measures/grain/units/relationships/mappings without treating relational structure as business authority.
6. Route every production LLM proposal through a released `contextual-orchestrator`; outputs remain proposed/inferred until steward validation/publication.
7. Add alignment/matching, RDF/OWL/SKOS/SHACL validation, governed persistence/review/publication adapters, client completion, multilingual/evaluation, observability/recovery and immutable release evidence under their canonical owner boundaries.

## DDD and release fitness

Adapters stay outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Client Consumption depends only on governed release contracts, never generator-private classes, prompts, persistence tables or orchestration state. Published semantic truth is immutable; corrections create a new release plus supersession evidence. Production LLM output cannot become authoritative before steward validation and governed publication.

No Foundation, #35, Client, Source Observation, research child, semantic publication, or release is authorized by this snapshot alone. The closest shared infrastructure prerequisite is ordinary non-force adoption of central protected `.github/main@f578d8d...` by the active CodeQL successor chain, followed by fresh verification of the pre-cutover direct-evidence/job-set-convergence repairs and preservation of #2051/#2056's valid identity/wake deltas. The closest ConceptWeave-owned source delta is the PostgreSQL 18 representation/version/authorization/qualified-type RED->GREEN; the closest research-stack repair is #10's non-force semantic reconciliation onto current #9.