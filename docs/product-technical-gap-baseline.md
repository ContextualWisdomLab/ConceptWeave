# Product / Technical Gap Baseline

**Snapshot:** 2026-09-11

This document records ConceptWeave's code-current product and technical gap baseline. Exact SHA/run coordinates are immutable evidence snapshots, never mutable supplier dependencies. Live protected branch, PR, issue, review, and workflow state supersedes a recorded coordinate when it advances. Any head movement resets exact-head execution/review evidence unless that evidence was actually produced for the successor.

## Canonical product boundary

ConceptWeave owns `observe -> discover -> propose -> align -> validate -> review -> publish`, ontology/semantic-layer generation and validation, governed immutable semantic releases, and the canonical release-consumption contract. Source-system business truth stays with its canonical owner.

- `semantic-data-portal`: catalog/governance/consumption.
- `context-graph-contracts`: interop contracts.
- `enterprise-architecture-core`: enterprise-architecture truth.
- `contextual-orchestrator`: production LLM/provider/capability routing.
- `keyverse`: identity/authentication trust evidence; ConceptWeave separately owns authorization of ConceptWeave proposal/base/semantic resources.
- consuming products: tenant/purpose authorization and physical execution.

Consumers use released/versioned `semantic_release`/contract/ACL coordinates. Source copying, cross-service SQL, and mutable sibling-head dependencies are invalid integration mechanisms.

## Protected truth and active prerequisites

Fresh 2026-09-11 authority:

- protected/default ConceptWeave `main`: `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`; repository bootstrap only, with no immutable ConceptWeave semantic release;
- Product-CI bootstrap #35: `9bb82f041483cb4e0cf1aa1f5450b413309f9a05`, OPEN/non-Draft/mechanically mergeable on protected `main`; its unchanged exact head remains the central CodeQL rollout/settlement consumer canary;
- Foundation #1: `60f14a6e85a83d56c2eea43b34d52b3366bb1735`, OPEN Draft; it must ordinary/non-force restack after #35 normally integrates;
- Client Consumption #5: `6873ec0c0a701b2c59f3e0785d48d8739f019d5b`, the current parent of Source Observation #6;
- Source Observation #6: `287165d399c5f54d6c4b4aa3c15497b47de8244b`, OPEN Draft/mechanically mergeable;
- representation-v3 parent #45: `6b2a8f555725dc79f60432afbc492d6005290a4a`, OPEN Draft/mechanically mergeable on #6;
- representation/index successor #46: per-key semantics repair ancestor `173ae2f5a187341421ec5ed94f3fcc7d69c1de35`; operator-class option RED `d87eb560f3c90fd22c8ac669febf0d85b030e1a0` and source repair through `d25a9d5022a483693819663f27b7e1f1f0c2969d`. Re-read the exact branch head after every ordinary forward commit; this documentation commit itself resets exact-head execution evidence.

Protected central `.github/main` is `cb0872c9a20d5584703dffacca65c096fc034c6c`. Its required contexts include CodeQL compatibility analysis for actions/python, queue/security/dependency checks, Noema, required-workflow bootstrap, coverage evidence and OpenCode review. Historical central-base execution evidence does not become current-base acceptance merely because an old PR remains mechanically mergeable.

`.github#2051@558693e0333e48012beea142f739bc634b0674a7` and stacked `.github#2056@69ae472562c93cc17674af5e2085a58947d3fab8` remain owner repair prerequisites at the latest verified ConceptWeave checkpoint. Preserve their base-bound identity, complete terminal-job-set convergence and atomic-wake deltas through ordinary/non-force current-main reconciliation. A backward-compatible protected handler must exist before clients require a newer dispatch identity. Then obtain fresh exact terminal GREEN before unchanged-head #35 acceptance.

No force push, destructive rebase, self-approval, review dismissal, fail-open scanner substitution, no-op/manual retrigger, synthetic status, mutable supplier dependency or routine administrator bypass is acceptance evidence.

## Source Observation bounded-context contract

Source Observation owns bounded request admission, exact source/schema/resource authorization, immutable observed relational facts, source-content digest identity, evidence locations and receipts. It does not own credentials, source-system business truth, semantic inference, provider runtime objects, publication authority or foreign product truth.

`ObservationRequestBudget` and the policy-admitted resource envelope bind exact-schema authorization metadata plus runtime row/byte/concurrency/deadline limits. `AuthorizedObservationRequest` is intentionally single-use at the execution seam. Retry after cancellation/failure/success requires fresh authorization. Source lookup, policy binding, schema/resource authorization, adapter execution and cancellation consume one non-resetting operation budget. A concrete adapter may resolve credentials only for the exact authorized source key plus immutable connection-policy binding; stale binding fails before source access.

Historical v2 evidence is frozen. The v2 digest domain, table/column/constraint representation and `/schemas/{schema}/tables/{table}` receipt vocabulary retain identical meaning and reproducibility. New PostgreSQL facts must never be appended under the v2 identity domain.

Client Consumption verifies exact detached immutable semantic-artifact bytes with `SemanticReleaseClient::verify_detached_artifact` after authoritative-use admission; digest syntax alone is never integrity evidence.

## PostgreSQL 18 representation-v3 current state

The representation-before-transport prerequisite is materially implemented in the #45 -> #46 successor lineage. The previously active per-key PostgreSQL index semantics RED and the later operator-class parameter loss are source-repaired, but exact-head native/Product acceptance is still absent and this documentation successor resets execution evidence again.

The v3 source currently preserves:

- `pg_class.relkind` and relation comments;
- exact qualified column type bindings without `search_path` inference;
- schema-scoped domains and enums, including domain base type, typmod/array/collation/NOT NULL/default/CHECK evidence and enum membership/order;
- direct schema allowlist revalidation for relations/domains/enums before immutable snapshot or receipt issuance;
- first-class indexes with key versus `INCLUDE` roles, expression-versus-column structure, exact positions, uniqueness, `NULLS NOT DISTINCT`, access method, readiness/validity/liveness, partial predicate, reconstructed `pg_get_indexdef` text and comments;
- one structured `IndexKeySemantics` record for every key position, carrying exact qualified collation when present, exact qualified operator-class coordinate, opaque access-method-specific `indoption` bits and canonical exact operator-class option name/value evidence;
- v3 domain-separated deterministic digest framing, including complete per-key semantic records and operator-class options;
- kind-aware successor coordinates `/schemas/{schema}/relations/{kind}/{name}/...` with relation-child column/constraint/index coordinates, while frozen v2 `/tables/...` semantics stay unchanged;
- exact qualified-type resolution shared by relation columns and domain base types; unknown qualified types fail closed.

### Index representation repair lineage

PostgreSQL 18 defines `indnatts` as total key plus included attributes and `indnkeyatts` as key attributes. Key positions precede included payload positions; a zero `indkey` entry denotes a key expression. `INCLUDE` accepts columns, not expressions, and `CREATE INDEX` requires at least one key `index_elem` before optional payload.

The source lineage enforces one canonical structural invariant at `IndexObservation::new`: at least one key; collection/`IndexAttributeKind` agreement; one-based contiguous key -> INCLUDE positions; expression keys permitted; INCLUDE columns only.

Exact structural repair evidence on #46:

- reviews `5174516003`, `5174543856`, `5174551665`: role/order/expression findings;
- `0ec2d1016b931230655eb3536bb40e62d07f8531`: kind-aware relation/child successor coordinates;
- `294a6d06ec4986f51b0cd714868d6b4de8253fc6`: role/collection, contiguous ordinal and expression-INCLUDE constructor repair plus contradictory-fixture correction;
- review `5174770575` -> RED `808faa922c466a586ea3bc8bf9f928f049ec3b95` -> repair `6dd59aef9a69e08aae6e7ede54ed1e204a634d9f`: empty and INCLUDE-only indexes fail closed;
- `033ae72b95e3ef490c47ec84c191b83632601b10` and `b2806d0b2d02d1a635d5a4850a88082c99c38fda`: prior authority/integrity documentation successors adopted as ordinary forward history.

### Per-key PostgreSQL semantics: RED -> source repair

Review `5175066205` identified that key position/source alone lost PostgreSQL 18 `pg_index.indcollation`, `indclass`, and `indoption`. Refinement review `5175346595` fixed the cardinality and interpretation boundary: those arrays have exactly `indnkeyatts` key entries, INCLUDE positions carry none, and `indoption` meaning belongs to the owning access method.

RED `ebac140d8e4e554b5f43603ed167999efa7c739b` requires:

1. changing qualified collation, qualified operator class, or exact per-key access-method option bits changes structured v3 identity without relying on `pg_get_indexdef`;
2. every key position has exactly one semantic record and INCLUDE positions have none;
3. semantic positions match structural key positions exactly;
4. blank/unqualified operator-class or collation coordinates fail closed;
5. relation-attached indexes cannot enter governed snapshot identity with missing key semantics or blank access-method context;
6. the generic domain preserves exact `indoption` bits under the access-method binding rather than decoding them as B-tree-specific ASC/DESC/NULLS semantics.

Ordinary forward repair `173ae2f5a187341421ec5ed94f3fcc7d69c1de35` implements the missing production seam:

- `QualifiedOperatorClassName` uses exact schema/name coordinates and does not use OID or `search_path` as governed identity;
- `IndexKeySemantics` owns one-based key position, optional exact qualified collation, exact qualified operator class, and opaque option bits;
- `IndexObservation::with_key_semantics` requires one record for every structural key position and rejects position/count mismatch;
- `RelationObservation::with_indexes` requires complete key semantics plus a nonblank observed access method before an index can enter governed snapshot identity;
- the v3 digest frames each semantic position, optional collation, operator class and option bit pattern as structured identity;
- current fixtures/tests cover material digest variation, missing semantics, blank access method, position mismatch and operator-class coordinate validation.

This closes the earlier source-level RED at the implementation/fixture boundary, but it is not native/Product GREEN. Review `5175722509` found that PR/baseline authority still described the predecessor RED after this ordinary forward repair; the PR body and baseline were subsequently currentized. Every resulting head movement still requires fresh executable acceptance.

### Operator-class parameters: verified P1 -> RED -> source repair

Review `5176147253` initially suspected that the schema/name operator-class coordinate could collapse same-name classes across access methods. Verification against the actual v3 framing showed that claim was too broad: `encode_index` already frames the parent index access method before its per-key operator-class coordinate. Review `5176183141` supersedes that collision claim rather than forcing a false model change.

The same verification exposed a real adjacent P1. PostgreSQL 18 `CREATE INDEX` permits an operator class with optional per-key `opclass_parameter = value` parameters, exposed as attribute-level option evidence. The existing `IndexKeySemantics` carried collation, operator-class name and `indoption` but no first-class operator-class parameters; optional reconstructed `pg_get_indexdef` text cannot be the only semantic carrier.

RED `d87eb560f3c90fd22c8ac669febf0d85b030e1a0` requires:

- changing an operator-class parameter value changes v3 source-content identity;
- option-array ordering does not create a second identity for the same exact option set;
- duplicate option names fail closed;
- option names are nonblank while option values remain exact source text, including an empty value when observed.

Source repair through `d25a9d5022a483693819663f27b7e1f1f0c2969d` adds `OperatorClassOption`, deterministic `IndexKeySemantics::with_operator_class_options`, public export and digest framing of each exact option name/value pair. A provisional access-method-binding RED was added and then retired by ordinary forward history after finding verification showed the parent index method was already framed; no history was rewritten and no false GREEN was claimed.

The representation must never use catalog OIDs or `search_path` inference as governed identity. `pg_get_indexdef` remains reconstructed provenance text, not the sole semantic carrier. PostgreSQL access-method-specific decoding belongs in the adapter/access-method boundary; ConceptWeave's generic representation owns exact observed facts and deterministic identity.

Authoritative PostgreSQL 18 basis:

- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: CREATE INDEX*. https://www.postgresql.org/docs/18/sql-createindex.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_index*. https://www.postgresql.org/docs/18/catalog-pg-index.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_opclass*. https://www.postgresql.org/docs/18/catalog-pg-opclass.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_am*. https://www.postgresql.org/docs/18/catalog-pg-am.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_attribute*. https://www.postgresql.org/docs/18/catalog-pg-attribute.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_class*. https://www.postgresql.org/docs/18/catalog-pg-class.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_type*. https://www.postgresql.org/docs/18/catalog-pg-type.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_enum*. https://www.postgresql.org/docs/18/catalog-pg-enum.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_constraint*. https://www.postgresql.org/docs/18/catalog-pg-constraint.html
- PostgreSQL Global Development Group. (2026). *PostgreSQL 18 documentation: pg_description*. https://www.postgresql.org/docs/18/catalog-pg-description.html

Server-rendered `pg_get_constraintdef`, `pg_get_indexdef` and `pg_get_expr` text is reconstructed source evidence, never original DDL. Catalog OIDs may be transaction-local join coordinates but are not governed semantic identity.

## Representation acceptance still required

Before #46/#45/#6 can claim the representation prerequisite GREEN, the current exact successor must execute and pass repository-pinned Rust 1.98:

- `cargo fmt --all --check`;
- strict workspace/all-target Clippy with warnings denied;
- workspace tests including v2-frozen and v3/index-layout/per-key semantic/operator-class-option contracts;
- rustdoc/doc tests and release build;
- owned production docstring/test/edge-case coverage requirements;
- Product/security/dependency/review workflows applicable to the exact protected-stack state.

Hosted runs or local execution produced for predecessor heads do not transfer. The current environment does not provide the repository-pinned Rust toolchain, and predecessor PR heads had no pull-request workflow run beyond bot-only status. This documentation successor resets that evidence again. Draft-skipped or bot-only status is not Product/security/native acceptance.

The representation regression set must keep proving:

- historical v2 digest and table/column/constraint coordinate meaning are unchanged;
- otherwise-identical v3 snapshots differing in one material PostgreSQL fact have distinct v3 identity;
- every new evidence kind has a verified receipt coordinate;
- unauthorized domain/enum-only schema fails before immutable snapshot/receipt side effects while an authorized type-only schema succeeds;
- same-name types in different allowed schemas stay distinct and columns resolve to exact qualified type coordinates;
- unknown domain base types fail closed through the same exact qualified-type resolver as columns;
- enum label/order and material domain semantics alter successor identity;
- relation-level and child coordinates preserve exact `RelationKind` and reject same-name wrong-kind receipt lookup;
- index role/position/source/predicate/null-uniqueness/readiness/validity/liveness/access-method/definition changes alter identity;
- per-key collation/operator-class/access-method option semantics and operator-class parameters are structured first-class identity rather than optional text-only provenance;
- operator-class option order canonicalizes, duplicate option names fail closed, and material option-value changes alter identity;
- missing/misaligned key semantics, blank access method, role/collection disagreement, non-contiguous ordinals, expression INCLUDE, empty indexes and INCLUDE-only indexes fail closed;
- input-order permutations of identical complete evidence remain digest-identical.

## Concrete PostgreSQL adapter boundary

Do not attach transport until the representation successor is exact-head GREEN and ordinarily adopted through the stack. The later adapter must use a maintained patched Rust PostgreSQL driver pinned by immutable lock coordinate and passing cargo-deny/SBOM review; resolve least-privilege credentials only for the exact authorized key+binding; reject stale binding before credential/source I/O; use one explicit `REPEATABLE READ READ ONLY` catalog transaction; preserve exact-schema `pg_catalog` evidence, including per-key collations/operator classes/`indoption` and operator-class option arrays; consume one remaining-operation budget across connect/transaction/query/cancellation; enforce policy-admitted row/byte/concurrency ceilings; perform complete-or-fail snapshot construction; handle source disappearance deterministically; and replay a frozen anonymized GRC-shaped conformance fixture without copying `governance-risk-compliance` business truth or querying its application tables through hidden coupling.

## Other product/research lanes

This Source Observation lineage does not rewrite Client Consumption, Research Intake, golden-set evaluation or later steward/full-text deltas. Those lanes retain their canonical owner histories. Before acting on one of them, re-read its live PR/issue/default-branch authority rather than relying on historical coordinates embedded in this baseline.

For the procedural-generation lane, #44 remains a private source-shaped Rust boundary until native/Product acceptance and released Keyverse trust consumption exist. LLM-produced semantics remain proposed/inferred and cannot become authoritative before independent validation, authorized stewardship and immutable publication.

## Capability status

| Area | Status | Evidence / next verification |
| --- | --- | --- |
| Product boundary | ACTIVE_PR | PRD/TRD/ADR/context map define canonical ConceptWeave ownership and foreign-owner seams. |
| Truth/publication lifecycle | SOURCE_REPAIRED_PENDING_PROTECTED_EVIDENCE | No protected immutable semantic release exists. |
| Client Consumption | RESTACKED_HOSTED_PENDING | #5 remains the current Source Observation parent; exact live evidence must be re-read before Client action. |
| Source Observation | REPRESENTATION_V3_SOURCE_REPAIRED_PENDING_EXACT_HEAD_GREEN | `173ae2f...` closes the original per-key RED and `d87eb... -> d25a9d...` closes the operator-class-option source RED; this docs successor resets execution evidence. Rust/Product acceptance and ordinary parent adoption remain. |
| Product CI | BLOCKED_OWNER_RECONCILIATION | #35 waits on central backward-compatible handler/current-main reconciliation and exact terminal GREEN. |
| Quality gate | ACTIVE | Rust 1.98, unsafe forbidden, public docs, fmt, strict Clippy, tests, rustdoc, release, owned production coverage, fixture/schema/lock/clean-tree checks; every head movement resets acceptance. |
| Security / review | PENDING_EXACT_HEAD | Scanner/reviewer status is accepted only when bound to the exact current head and applicable protected policy. |
| Release | NOT_STARTED | Version/CHANGELOG/tag/package/immutable semantic release/SBOM/provenance/reproducibility/rollback remain mandatory on the exact protected release head. |

## Current causal sequence

1. Re-read #46 after this documentation successor, review the exact current head, and obtain exact-current Rust 1.98 plus applicable hosted Product/security/dependency/review acceptance without flattening #45/#6.
2. Ordinary/non-force adopt the verified #46 delta into #45 and then #6; do not transfer predecessor GREEN.
3. In parallel prerequisite order, central owner lands a backward-compatible handler, reconciles #2051/#2056 onto protected `.github/main`, obtains exact terminal GREEN, then unchanged #35 obtains fresh acceptance and merges normally.
4. Foundation ordinary/non-force restacks after #35 and repairs its generic owner text-contract discrepancy under its own authority before descendants inherit a fresh parent.
5. Only after Source Observation representation and protected prerequisites are current, add the bounded least-privilege PostgreSQL adapter and frozen conformance fixture.
6. Continue ontology/semantic discovery, alignment, deterministic validation, independent evaluation, steward review and immutable publication under canonical owner boundaries; all production LLM calls remain behind released `contextual-orchestrator` contracts.

Adapters stay outside the core domain model and external DTOs cross explicit Anti-Corruption Layers. Source Observation facts are evidence, not source-system business truth. Published semantic truth is immutable; corrections create a new release plus supersession evidence. No Foundation, #35, Client, Source Observation, research child, semantic publication or release is authorized by this baseline alone.