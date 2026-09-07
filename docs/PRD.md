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

When a write response is lost, show the affected item as unresolved even if a
later read looks unchanged or matches the requested result. Preserve what was
submitted and what was later observed, keep confirmed earlier results, and do not
automatically repeat or undo the uncertain change.

Research changes must use the evidence actually reviewed. Changed supporting
metadata or incomplete source inventories must stop a change plan before approval
is consumed. Older reviews lacking this evidence binding require fresh independent
approval; displaying a plan does not mean that the library has been changed.

A candidate cannot become authoritative solely because an LLM or automated extractor produced it. The publication lifecycle is Draft -> Proposed -> Validated -> Reviewed -> Published, with explicit rejection and supersession paths.

### FR-6 Publication

Publish versioned artifacts for ontology and semantic-layer consumers while retaining the exact input snapshot and proposal/review receipts that produced the release.

### FR-7 Interoperability

Support stable adapters for `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, and other CWL products without direct cross-service application-table SQL.

### FR-8 LLM assistance

All LLM-backed induction uses released `contextual-orchestrator` contracts. Model output is untrusted proposal data and may not skip deterministic validation or review. A documented API, Draft release proposal or successful maintenance job does not prove the deployed service is ready; missing release and runtime evidence keeps model assistance unavailable without excluding papers from review.

### FR-9 Research evidence intake

Message ancestry, source-data changes and work dependencies may supply different kinds of research evidence, but none automatically establishes a semantic relationship. Reviewers must see missing references, cycles, heuristic grouping and producer limitations. A registry entry or package version does not establish the exact API available for adoption. The [threading/CDC/work-dependency audit](doctoring/cwl_ontology_capability_inventory.md#threading-cdc-and-work-dependency-contracts-2026-09-06) assigns follow-up to existing owners; it neither classifies papers nor creates another utility service.

Repository capability discovery and paper classification are separate measures. A statistical library's model fit, factor structure or linked score is not an ontology label or approval. Before such evidence can inform a candidate, reviewers need its population/design, applicable item or observation versions, unavailable results and complete failure denominator. The [statistical-library audit](doctoring/cwl_ontology_capability_inventory.md#statistical-library-contract-audits-2026-09-06) records cultivation requirements, not adopted scoring rules or completed paper reviews.
Evaluation must reject omitted or inconsistent retained sources before requesting approval. Changing retained source metadata invalidates prior approval, even when paper predictions stay unchanged. Empty or unresolved-only inventories remain auditable evidence and must not acquire reviewed-paper counts.

Preserve every observed source, including standalone files and notes outside the bibliographic proposals. Keep unresolved source relationships visible instead of treating a completed bibliography worksheet as a completed library review. All standalone sources and records without a valid path to a bibliographic parent need explicit reconciliation; notes, files and annotations must not acquire paper labels from their titles. Retraction and correction evidence remains separate from topic classification and approval. The current producer retains this inventory; downstream reconciliation, completion admission and independent governance remain required, not implemented by inventory generation alone.

Library reads must finish within a bounded observation window or fail visibly without returning a partial classification. Slowly arriving pages cannot keep a run open indefinitely, and missing time budget must not be handled by silently dropping papers.

Read a complete Zotero Local API observation with one consistent library version and propose exactly one research disposition for every top-level bibliographic item. This consistency check does not establish an atomic provider snapshot. A record claiming a revision newer than the observed library invalidates the complete read; it must not be omitted or assigned a different revision to make the read pass. Each proposal retains the item key/version, exact matched metadata values, rule revision, linked child records, and any model receipt. Weak evidence and evidence that matches multiple specific disposition families must abstain into steward review. A local abstention retains its nonempty abstract exactly once, as matched evidence when applicable or otherwise as review context; decided items omit the review-only copy. Duplicate DOI/title identities are review candidates only: intake never merges, deletes, or silently mutates Zotero records.

Full-text enrichment must distinguish listed attachments, returned nonempty text, complete or partial indexing, and reviewed meaning. Missing abstracts or unavailable text never remove papers from the campaign denominator or prove irrelevance. Newly retrieved text requires its own immutable evidence capture and renewed review of any changed proposal; it cannot silently replace evidence beneath an earlier approval. The [full-text audit](doctoring/zotero_fulltext_contract_audit.md) establishes availability only. A separate proposed local capture now preserves the observed text for later review, with missing material still visible. Retained text is neither completed classification nor approved meaning.

Stewards must be able to inspect saved text alongside the next pending review rows without rereading Zotero or rewriting the report. A separate private evidence view binds the capture, original report and unchanged proposals, includes only those rows' attachment content, and retains missing and partial material explicitly. If the complete view exceeds its size limit, generation fails instead of truncating papers or text. Generating or reading this view does not fill decisions. Existing metadata-only apply commands must reject it.

A full-text review starts with blank decisions for every paper; previous metadata decisions cannot silently acquire a claim that the text was reviewed. Accept a completed view only while its displayed evidence and pending selection still match the current review. Changed or ambiguous content, incomplete decisions and stale views fail without replacing prior work. Each new worksheet, completed review and verified aggregate result retains the same captured-evidence identity. Finalization and evaluation recheck the capture against the original report; approval must authenticate that identity and every reviewed label. This is review provenance, not proof that a person read a file, and not permission to change Zotero. The offline command-line workflow initializes blank work, shows the next pending text, saves accepted decisions to a new worksheet and prepares a fully decided review using a separately supplied approval input. It neither supplies decisions nor authenticates that input; independent governance verification remains required. A successful private review output must fit the matching reader's size limit, without truncation or overwriting earlier work.

For every connected duplicate component, accept externally verified steward decisions selecting one component-level canonical item. Produce a local-only manifest that binds decisions to the raw snapshot, complete item revisions, exact duplicate membership, current proposals and retained source metadata. Reject missing or inconsistent source inventory and invalid decisions before requesting approval. Changed retained evidence requires fresh independent approval, even when duplicate members are unchanged. Record every component source revision plus before, after, and rollback canonical-key mappings. Classification preserves every Zotero source record.

Reviewed collection and tag changes default to a local dry-run plan. Each operation binds the authority receipt, server/library/item revisions, raw-snapshot digest, and complete before/after/rollback metadata. Execution-critical plan state is immutable outside the owner crate, so callers cannot turn a dry run into execution or alter validated operations. Zotero 9 execute requests fail closed. No plan contains credentials or permits `NeedsStewardReview`, source-record deletion, or attachment deletion.

An invalid or stale change request must be rejected before approval is redeemed, including when an earlier item in the same request is valid. A locally valid request reaches approval verification exactly once. For writes based on full-text review, approval must cover the captured evidence, every reviewed label, the explicitly chosen collections and tags, and whether execution was requested. Reviewing meaning does not choose a destination or grant permission to change it. The same approval and evidence identity must remain attached to partial outcomes and recovery. The local library now admits a complete full-text review only with separately verified explicit changes and execution mode. It preserves that binding through writes, rollback, retries and delayed rollback reconciliation. An unknown original write cannot be called restored without evidence. Independent governance integration, durable recovery after process restart, delayed original-write reconciliation and approved live use remain gaps.

An unresolved original change can now be inspected again without resending it. The private result retains the exact attempted change and the complete earlier outcome, including changes already made and papers not attempted. A matching later value is not proof that the earlier request succeeded or finished; failed, foreign and malformed observations cannot grant recovery permission. Resolution and safe recovery remain separate from inspection.

For execute-mode plans, the runtime must preflight every item before the first write and stop at the first failed or unverifiable response. Follow-up metadata reads are observations only: matching before or after values cannot establish whether the submitted request completed, terminated, or caused the observed change. A secret-free receipt retains the exact reviewed plan coordinates, proposal binding, submitted indeterminate request and optional observation. Dry-run receipts enumerate every planned item as untouched. Only directly verified successful write responses produce applied entries and reverse-ordered inverse operations; an unknown write produces neither retry nor rollback authority. Earlier verified operations remain recorded. Cross-item atomicity is not claimed.

Rollback must retain server-bound expected and restoration metadata, preflight every operation at one current library version, preserve receipt order and directly verified revision advancement. Uncertain original or inverse writes must not become successful recovery or retry authority. Failed-inverse inference is repaired locally: complete submitted requests and observations remain indeterminate. PR #20's operation-slice API still requires authoritative consumer integration preserving original-write scope before approved live use; an empty inverse list cannot establish that an unknown original write was recovered.

PR #21 retains validated delayed reads without writes and complete observed metadata. Metadata-only restored/unchanged and retry inference is removed in `f9c2c03`: the observer always retains indeterminate causal status and emits no retry operation. Eight metadata scenarios and the three-GET adapter fixture remain covered; the latter compares the entire observed state. Legacy enum variants and the optional retry field remain for contract compatibility, not as outputs or approval from this observer. Authoritative successor wrappers still must retain the complete prior rollback receipt, its exact submitted request, binding and untouched tail; an operation-only observation cannot replace that envelope or establish causal completion, termination, or independent retry authority.

The Zotero 10+ adapter can accept a caller-owned API key and server identity at runtime or consume one successful, user-approved Local API authorization. Authorization sends one bounded application name and the expected server identity to the fixed loopback endpoint; only a same-server bounded response that explicitly reports denial is classified as the user's decision. Denial and rate limiting return immediately without another prompt or automatic retry. The private 32-character key is neither serializable nor printable. Authorization, read, and write responses bind to the expected server before status classification; writes name expired authorization and matching-server stale preconditions separately. Thin public execution boundaries connect the adapter to the reviewed write and rollback cores without duplicating mutation logic. Synthetic transport evidence does not satisfy AC6's approved live Zotero 10 authorization, write, and rollback requirement.

A local steward worksheet must bind the library version, rule revision, complete raw-snapshot digest, current proposal-and-retained-source digest, every observed parent/child item revision, and one blank decision slot per classified bibliographic item. It repeats item identity, proposal, and abstention reason only; titles, abstracts, tags, collections, and matched evidence remain in the separate sensitive report. Shared inventory validation rejects omitted source records, hidden pending relationships and inconsistent identity before construction. Valid unresolved sources do not prevent starting review, but prevent claiming completion. Old worksheets without the content binding require regeneration, never automatic approval backfill.
After every decision is filled, worksheet finalization must verify the governance receipt coordinates, unique item identities and revisions, proposal/abstention consistency, and non-abstention truth labels before producing a reviewed golden set. Missing decisions remain incomplete and cannot reach external approval verification.
Operators must be able to finalize the saved report, completed worksheet, and approval receipt offline without rereading mutable Zotero state. Every input must be a distinct owner-only file identity, not merely a differently spelled path, and the new golden-set output must use a separate path; invalid, oversized, linked, or shared inputs fail closed.
During the human review campaign, operators must be able to validate a partially completed worksheet against its original report and persist aggregate progress without an approval receipt. Progress binds current proposal and retained metadata identity and reports bibliographic total, decided and remaining counts alongside unresolved source count. Local preparation is complete only for a nonempty fully decided worksheet with no unresolved sources. Filled paper decisions do not hide pending attachments, notes or disconnected ancestry. Progress never claims correctness, independent approval, applied reclassification or publication authority. An empty campaign is not complete.
The worksheet's own required content identity must match the current report independently of the supplied receipt. Blank identity is invalid; a stale or replaced identity is a snapshot mismatch. Conversion only prepares input for independent verification. Unresolved sources can remain in locally prepared review data, but prevent whole-library completion; refreshing local digests cannot renew an independently issued approval.

Operators must be able to accumulate small steward-reviewed decision sets without hand-merging the complete worksheet. Each patch binds the original library version, classifier revision, snapshot and proposal/retained-content digests, item key and item revision. Regenerating a worksheet after content changes cannot make an older patch valid. Missing content binding requires a new review-bound patch, never automatic backfill. Empty, duplicate, unknown, stale or abstention decisions fail atomically. Identical replay is idempotent; conflicting decisions cannot overwrite review work. Applying a patch does not confer independent approval, full-text review provenance or publication authority.
The offline CLI must read the saved report, current worksheet, and decision patch as three distinct owner-only file identities and create a separate updated worksheet. It must never overwrite the current worksheet, reread Zotero, or emit output after invalid input.
Operators must be able to extract up to 100 blank bibliographic decisions with the exact context needed for review. Batches preserve current content identity and separately show unresolved source count; no blank paper decisions does not mean all sources are resolved. Ordering is deterministic, decided rows are skipped and unchanged inputs reproduce the same batch. Creation is neither assignment nor progress; only an accepted completed patch increases unverified decision coverage, never independent approval or applied reclassification.
Applying a completed review batch must rederive the same pending view from the original report and current worksheet, compare every displayed context field, and reject blank or abstaining decisions. A review batch must not be accepted through the context-free decision-patch parser because silently ignored display-field drift would break the link between the steward decision and the evidence shown during review.

Evaluate classifier quality only against a steward-reviewed local golden set whose governance receipt is externally verified and binds both the complete source/classifier-input snapshot and every current proposal field, in addition to the item-key/item-version coordinates. Same-version changes to unmodeled provider metadata, absent/default fields, classifier inputs, predictions or supporting evidence must invalidate the corresponding binding. Evaluation recomputes proposal identity before contacting governance; a locally changed digest cannot renew an approval. Legacy unbound approvals require reissuance, never automatic backfill. Abstention is a prediction outcome, never an approved truth label. Evaluation emits the verified library revision, rule revision, opaque snapshot and proposal digests, and aggregate counts for exact matches, abstentions, and per-disposition true-positive/predicted/expected totals; it must not copy Zotero keys, reviewer identity, or bibliographic text into the result.

The [live/visual audit](doctoring/zotero_metadata_visual_audit.md) found three standalone PDFs and one standalone note outside the bibliographic worksheet. Their identity, relationship to existing papers and treatment still require evidence-bound reconciliation. Retraction/correction evidence remains distinct from topical classification and must be considered before authoritative use.
A complete metadata-review result additionally requires exactly one non-abstention steward label for every top-level bibliographic item and no unresolved source records. Standalone sources, orphan trees and disconnected cycles must be resolved before completion; clearing their reported list cannot bypass inventory validation. A sampled golden set remains valid for quality measurement but cannot prove completion. Neither result proves full-text approval or an applied Zotero reclassification.
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
