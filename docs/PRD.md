# ConceptWeave Product Requirements Document

## 1. Product statement

ConceptWeave converts heterogeneous enterprise evidence into a governed ontology and semantic layer without collapsing observed facts, model inference, and human-approved meaning into the same truth state.

## 2. Buyer problem

Enterprise teams repeatedly hand-build business glossaries, ontologies, metric definitions, semantic mappings, and data relationships from database schemas, API contracts, documents, and tribal knowledge. The work is slow, inconsistent across tools, difficult to audit, and unsafe to delegate entirely to an LLM because inferred semantics can be plausible but wrong.

## 3. Primary buyers and users

- enterprise data architects and semantic-modeling teams;
- data governance and catalog teams;
- analytics/BI platform owners;
- AI/RAG platform teams that require governed machine-readable context;
- risk/compliance and enterprise-architecture teams that need traceable semantic models.

## 4. Core job to be done

Given an enterprise source estate, produce a **reviewable semantic model proposal** in which every concept, relationship, constraint, dimension, measure, and physical mapping is linked to exact evidence and can be validated, rejected, reviewed, published, superseded, and reproduced.

## 5. Functional requirements

### FR-1 Source observation

Accept immutable snapshots or versioned contracts for relational schema, OpenAPI, AsyncAPI/event models, documents/glossaries, source-code structure, existing ontology/vocabulary, and lineage/provenance. Raw source authority remains with its owning system.

### FR-2 Candidate discovery

Produce candidates for concepts, taxonomies, non-taxonomic relations, semantic constraints, dimensions, measures, and physical-to-semantic mappings. Each candidate starts as inferred rather than authoritative.

### FR-3 Evidence and provenance

The current v0.1 candidate contract requires every candidate to retain exact source identity, source digest, and source location through `EvidenceReference`. Issue #2 must add immutable Source Observation and proposal-receipt contracts that also retain observation time, parser/extractor revision, and discovery method before the first Generation release. Until those receipt contracts exist, the Rust `SemanticCandidate` and `contracts/semantic-candidate.schema.json` must not be described as already carrying those deferred coordinates. Unsupported candidates fail closed.

### FR-4 Deterministic validation

Validate syntax, identifiers, relationship cardinality, mapping completeness, duplicate/contradictory definitions, ontology consistency where supported, semantic-measure contracts, and publication schema before review.

### FR-5 Governed review

A candidate cannot become authoritative solely because an LLM or automated extractor produced it. The publication lifecycle is Draft -> Proposed -> Validated -> Reviewed -> Published, with explicit rejection and supersession paths.

### FR-6 Publication

Publish versioned artifacts for ontology and semantic-layer consumers while retaining the exact input snapshot and proposal/review receipts that produced the release.

### FR-7 Interoperability

Support stable adapters for `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, and other CWL products without direct cross-service application-table SQL.

### FR-8 LLM assistance

All LLM-backed induction uses `contextual-orchestrator`. Model output is untrusted proposal data and may not skip deterministic validation or review.

### FR-9 Research evidence intake

Read one immutable Zotero Local API library-version snapshot and propose exactly one research disposition for every top-level bibliographic item. Each proposal retains the item key/version, exact matched metadata values, rule revision, linked child records, and any model receipt. Weak evidence and evidence that matches multiple specific disposition families must abstain into steward review. A local abstention retains its nonempty abstract as review context; decided items omit that extra copy. Duplicate DOI/title identities are review candidates only: intake never merges, deletes, or silently mutates Zotero records.

For every connected duplicate component, accept externally verified steward decisions selecting one component-level canonical item. Produce a local-only manifest that binds the decisions to the raw snapshot, its complete item-key/item-version coordinates, and exact duplicate-candidate membership, and records every component source revision plus before, after, and rollback canonical-key mappings. Classification preserves every Zotero source record.

Reviewed collection and tag changes default to a local dry-run plan. Each operation binds the authority receipt, server/library/item revisions, raw-snapshot digest, and complete before/after/rollback metadata. Execution-critical plan state is immutable outside the owner crate, so callers cannot turn a dry run into execution or alter validated operations. Zotero 9 execute requests fail closed. No plan contains credentials or permits `NeedsStewardReview`, source-record deletion, or attachment deletion.

For execute-mode plans, the runtime must preflight every item before the first write, stop at the first failed or unverifiable response, reconcile that item through the same server before declaring its state, and emit a secret-free receipt. The receipt identifies verified writes, the failed item, any indeterminate item, untouched items, and reverse-ordered rollback operations bound to the server identity, proven post-write item revision, and complete expected post-write metadata. Rollback must reject mixed-server evidence before reading, preflight every receipt item at one current library version before its first inverse write, consume the existing receipt order, advance the library precondition only from a verified response, and reconcile a failed response as restored, unchanged, or indeterminate. Only proven unchanged and untouched operations remain eligible for automatic retry; an indeterminate operation is retained separately with complete reconciliation evidence for an operator. Delayed reconciliation performs one read and no write, preserves the observed state, ignores unrelated library-version advancement, and emits retry evidence only when the exact item revision and expected metadata remain unchanged. A second use of consumed evidence must fail before writing. Cross-item atomicity is not claimed.

The Zotero 10+ adapter can accept a caller-owned API key and server identity at runtime or consume one successful, user-approved Local API authorization. Authorization sends one bounded application name and the expected server identity to the fixed loopback endpoint; only a same-server bounded response that explicitly reports denial is classified as the user's decision. Denial and rate limiting return immediately without another prompt or automatic retry. The private 32-character key is neither serializable nor printable. Authorization, read, and write responses bind to the expected server before status classification; writes name expired authorization and matching-server stale preconditions separately. Thin public execution boundaries connect the adapter to the reviewed write and rollback cores without duplicating mutation logic. Synthetic transport evidence does not satisfy AC6's approved live Zotero 10 authorization, write, and rollback requirement.

Evaluate classifier quality only against a steward-reviewed local golden set whose governance receipt is externally verified and bound to the canonical SHA-256 digest of the complete Zotero classification report plus its item-key/item-version coordinates. Abstention is a prediction outcome, never an approved truth label. Evaluation emits the verified library revision, rule revision, opaque snapshot digest, and aggregate counts for exact matches, abstentions, and per-disposition true-positive/predicted/expected totals; it must not copy Zotero keys, reviewer identity, or bibliographic text into the result.
Every successful classification report includes aggregate evidence for snapshot coverage, proposal coverage, provenance completeness, abstentions, duplicate candidates, disposition totals, and zero unreported failures.

## 6. First vertical slice

Relational schema snapshot -> observed tables/columns/foreign keys -> concept/relation/dimension/measure/mapping candidates -> evidence-bound validation report -> reviewable proposal package.

## 7. Non-goals for v0.1

- replacing `semantic-data-portal` as the enterprise catalog;
- arbitrary write access to source systems;
- automatic publication without review;
- treating vector similarity as semantic truth;
- copying every external ontology into one CWL namespace;
- building a generic LLM gateway or browser crawler;
- claiming an emerging draft semantic-layer format is a stable standard.

## 8. Acceptance criteria for the first commercial candidate

- 100% owned production line/function/region and branch coverage where tooling exposes it;
- candidate-to-source provenance completeness of 100%;
- zero publication paths that bypass reviewed state;
- zero silent inferred-to-authoritative promotion;
- deterministic replay of the same immutable source snapshot and extraction configuration;
- cross-tenant access denial when tenancy is introduced;
- malformed/hostile source contracts rejected with bounded resource use;
- semantic-model release can be reproduced from source receipts and approved proposal receipts;
- buyer can inspect why each published artifact exists and which evidence supported it.
