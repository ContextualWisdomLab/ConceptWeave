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

## Procedural model engineering extension — Proposed, issue #42

Requirements PG-FR-1 through PG-FR-6 are defined in
[ADR-PG-20260910](adr/pg_20260910_procedural_model_engineering.md): evidence-bound
procedure authoring, concept/tool-contract alignment, independent offline evaluation,
steward publication, released consumer projection and inspectable authoring/review UX.
These identifiers are profile-scoped and do not renumber existing FRs or research work.

The current #44 source slice goes beyond schema fixtures but remains private and
unreleased. It includes bounded raw-byte/UTF-8/strict-JSON admission, canonical Draft
2020-12 transport-to-domain mapping, schema-significant semantic/topology validation,
and canonical revision-envelope/base/candidate-scope comparison against a separately
supplied `ProceduralRevisionExpectation`. That expectation is deliberately not an
authentication receipt, and none of these source-shaped gates grants publication,
runtime, tool, factual, or steward authority.

Production admission still requires fresh native Rust 1.98 and hosted Product evidence,
then an application-layer adapter consuming an immutable released Keyverse trust
contract to obtain authenticated RP/subject/tenant context. ConceptWeave must apply its
own proposal/base resource authorization after identity admission; it must not copy
mutable Keyverse source, mint a local `authenticated=true` substitute, or move JWT/
provider verification into `conceptweave-domain`. Released semantic/tool artifact
authenticity and ACL/capability, independent evaluation, steward decision, immutable
publication and Noema activation remain later gates.

The procedural extension does not replace the initial relational vertical slice,
deliver a graph editor, run a model, or publish/activate procedural knowledge. A
customer must ultimately trace every suggested procedure to an approved model revision
and exact source evidence.
