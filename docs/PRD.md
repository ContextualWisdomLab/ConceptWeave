# ConceptWeave Product Requirements Document

## 1. Product statement

ConceptWeave converts heterogeneous enterprise evidence into a governed ontology and semantic layer without collapsing observed facts, model inference, and human-approved meaning into the same truth state. It also exposes a stable consumer contract so downstream products can reject incompatible or non-authoritative releases without understanding generation internals.

## 2. Buyer problem

Enterprise teams repeatedly hand-build business glossaries, ontologies, metric definitions, semantic mappings, and data relationships from database schemas, API contracts, documents, and tribal knowledge. The work is slow, inconsistent across tools, difficult to audit, and unsafe to delegate entirely to an LLM because inferred semantics can be plausible but wrong. Even after a model is published, consumers need a deterministic way to determine whether a release is compatible, governed, and safe to use.

## 3. Primary buyers and users

- enterprise data architects and semantic-modeling teams;
- data governance and catalog teams;
- analytics/BI platform owners;
- AI/RAG platform teams that require governed machine-readable context;
- risk/compliance and enterprise-architecture teams that need traceable semantic models.

## 4. Core job to be done

Given an enterprise source estate, produce a **reviewable semantic model proposal** in which every concept, relationship, constraint, dimension, measure, and physical mapping is linked to exact evidence and can be validated, rejected, reviewed, published, superseded, reproduced, and then safely admitted by downstream clients through a stable public contract.

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

All LLM-backed induction uses `contextual-orchestrator`. Model output is untrusted proposal data and may not skip deterministic validation or review. Optional future client matching/explanation also routes through this boundary and cannot silently promote a correspondence to authority.

### FR-9 Client consumption

A consuming product can inspect a versioned `semantic_release` offline and fail closed before authoritative use. The first Client slice requires stable release identity, contract and ontology versions, truth/publication state, declared SHA-256 digest identity, provenance references, and unique concept identifiers. Admission requires an explicitly supported contract version plus `Published` and `Authoritative` state. Consuming products retain their own tenant/purpose authorization and physical data/query execution.

The current digest value object validates the declared `sha256:<64 hex>` identity shape. Cryptographic integrity is not claimed until a later verifier hashes the exact serialized artifact bytes and compares the result.

## 6. First Generation ↔ Client vertical

`relational schema snapshot -> observed tables/columns/foreign keys -> concept/relation/dimension/measure/mapping candidates -> evidence-bound validation -> steward review -> immutable semantic_release -> offline client admission -> consuming-product ACL/query boundary`.

`ContextualWisdomLab/governance-risk-compliance` is the first reference source/client scenario, not a special-case algorithm. A shared golden fixture must exercise both Generation and Client without copying GRC truth into ConceptWeave or giving ConceptWeave direct GRC application-table access.

## 7. Non-goals for v0.1

- replacing `semantic-data-portal` as the enterprise catalog;
- owning downstream tenant/purpose authorization or physical query execution;
- arbitrary write access to source systems;
- automatic publication without review;
- treating vector similarity as semantic truth;
- copying every external ontology into one CWL namespace;
- building a generic LLM gateway or browser crawler;
- claiming digest syntax validation is cryptographic byte verification;
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
- consumer can validate release schema/version/governance state offline before authoritative use;
- exact serialized artifact digest verification exists before integrity is claimed;
- buyer can inspect why each published artifact exists and which evidence supported it.
