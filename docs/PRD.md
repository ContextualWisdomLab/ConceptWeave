# ConceptWeave Product Requirements Document

## 1. Product statement

ConceptWeave converts heterogeneous enterprise evidence into a governed ontology and semantic layer without collapsing observed facts, model inference, and human-approved meaning into the same truth state. It also exposes a stable consumer contract so downstream products can reject incompatible or non-authoritative releases without understanding generation internals.

## 2. Buyer problem

Enterprise teams repeatedly hand-build business glossaries, ontologies, metric definitions, semantic mappings, and data relationships from database schemas, API contracts, documents, and tribal knowledge. The work is slow, inconsistent across tools, difficult to audit, and unsafe to delegate entirely to an LLM because inferred semantics can be plausible but wrong. Even after a model is published, consumers need a deterministic way to determine whether a release is compatible, governed, immutable, superseded by an explicit successor, and safe to use.

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

The first active relational slice defines an immutable PostgreSQL schema-snapshot contract before a live adapter exists. It preserves exact schema/table/column identifiers, source column ordinals, source type/nullability/comment metadata, registry-authorized opaque source capability evidence, owner-computed snapshot digest, extractor revision, observation-time evidence, PK/unique/FK coordinates, and CHECK-constraint evidence. The raw registry key is bounded to at most 128 bytes of lowercase multiword `snake_case`; raw DSNs, URLs, shell-style connection parameters, generic one-word references, and malformed identifiers fail request admission. Syntax alone is not source authority: a validated `ObservationRequest` must resolve through the caller's authorized `SourceConnectionRegistry` into `AuthorizedObservationRequest`, and the canonical `SourceObservationPort` execution seam accepts only that authorized envelope. A syntactically valid but unregistered key therefore fails before adapter execution. The envelope carries no credentials; a concrete adapter resolves its opaque authorized capability to least-privilege credentials only inside its ACL.

Each request also carries caller-selected positive schema-count/total-UTF-8-byte metadata ceilings plus positive operation/statement-timeout, row, byte and concurrency ceilings. These values make the request structurally bounded but are not authority. `ObservationResourceEnvelope` combines them into one provider-independent policy input, and the same local registry that resolves the immutable source binding must explicitly admit that complete envelope against the same `ResolvedSourceConnection`. Resource authorization defaults to deny. A request above any source-policy ceiling fails with `UnauthorizedResourceEnvelope` before adapter/source/snapshot side effects; equal or narrower requests proceed only when policy explicitly grants them. The product must not use arbitrary PostgreSQL-specific global limits as a substitute for this source/purpose policy.

The end-to-end operation deadline includes source lookup, immutable binding, exact-schema authorization, resource-envelope authorization, connection and catalog work; implementation must not silently restart that deadline after authorization. Registry authorization remains bounded local policy, while remote credential/network work belongs in the adapter and consumes only the remaining admitted budget. Exact source identifiers are not normalized or truncated. For foreign keys, observed `ON UPDATE`/`ON DELETE` actions, any local-column subset targeted by `ON DELETE SET NULL (...)` or `SET DEFAULT (...)`, match type, deferrability/initial timing, and PostgreSQL validation/enforcement state are retained as typed source evidence; each metadata family remains explicitly absent if the adapter did not observe it rather than inventing defaults. For CHECK constraints, preserve the PostgreSQL-reconstructed definition together with validation, enforcement, and `NO INHERIT` status; do not infer ordered expression-column coordinates from SQL text.

Source observation must also distinguish whether a unique constraint treats missing values as distinct or equal. If that behavior was not observed, it remains unknown. A change in this behavior must change the evidence identity used by later proposals, even when the constraint name and columns are unchanged. This does not establish a business key or authorize publication.

### FR-2 Candidate discovery

Produce candidates for concepts, taxonomies, non-taxonomic relations, semantic constraints, dimensions, measures, and physical-to-semantic mappings. Each candidate starts as inferred rather than authoritative.

### FR-3 Evidence and provenance

The current v0.1 candidate contract requires every candidate to retain exact source identity, source digest, and source location through `EvidenceReference`. The active Source Observation slice additionally retains snapshot digest, observation time, extractor revision, typed table/column/constraint locations, foreign-key relationship behavior and validation/enforcement state when observed, CHECK definition/status evidence, and the immutable source-policy binding used for authorization. Issue #2 must still add proposal-receipt/discovery-method provenance and bind generated candidates to verified source receipts before the first Generation release. Unsupported candidates fail closed.

### FR-4 Deterministic validation

Validate syntax, identifiers, relationship cardinality, mapping completeness, duplicate/contradictory definitions, ontology consistency where supported, semantic-measure contracts, and publication schema before review.

### FR-5 Governed review

A candidate cannot become authoritative solely because an LLM or automated extractor produced it. The publication lifecycle is Draft -> Proposed -> Validated -> Reviewed -> Published, with explicit rejection and supersession paths.

### FR-6 Publication

Publish versioned immutable artifacts for ontology and semantic-layer consumers while retaining the exact input snapshot and proposal/review receipts that produced the release. A correction must create a distinct successor release rather than overwrite a published artifact in place. Supersession authority belongs to Governance & Publication and must produce an explicit predecessor/successor receipt; version ordering or timestamps alone are never replacement evidence.

### FR-7 Interoperability

Support stable adapters for `semantic-data-portal`, `LineageWeave`, `context-graph-contracts`, and other CWL products without direct cross-service application-table SQL.

### FR-8 LLM assistance

All LLM-backed induction uses `contextual-orchestrator`. Model output is untrusted proposal data and may not skip deterministic validation or review. Optional future client matching/explanation also routes through this boundary and cannot silently promote a correspondence to authority.

### FR-9 Client consumption

A consuming product can inspect a versioned `semantic_release` offline and fail closed before authoritative use. The first Client slice requires stable release identity, contract and ontology versions, truth/publication state, declared SHA-256 digest identity, provenance references, and unique concept identifiers. Admission accepts only the explicit current contract version or an explicitly configured supported-legacy version, plus `Published` and `Authoritative` state. Compatibility is never inferred from version ordering; unknown versions remain unsupported. Consuming products retain their own tenant/purpose authorization and physical data/query execution.

An admitted client can compare two releases deterministically without contacting a model/provider. Release diff applies the same authoritative-use admission policy to both inputs before returning stable previous/current release identity and sorted added/removed concept identifiers. Diff is semantic-contract evidence only; it does not authorize downstream data access, calculate business measures, mutate either release, or infer consuming-domain impact automatically.

The digest value object validates canonical `sha256:<64 lowercase hex>` identity syntax. Cryptographic integrity is a separate operation: `SemanticReleaseClient::verify_detached_artifact` first applies authoritative-use admission, then hashes the exact caller-supplied detached immutable semantic-artifact bytes and requires an exact digest match. The release manifest declares the artifact digest; it is not defined as a self-digest of the manifest bytes that carry that field. Syntax validity alone is never integrity evidence.

A client can also validate an explicit immutable supersession declaration. `SemanticReleaseReference` binds a release id to its exact artifact digest. `ReleaseSupersession` names distinct predecessor/successor references plus a nonblank rationale, rejects self-supersession, and `validate_supersession` requires both releases to pass ordinary authoritative-use admission and both id+digest coordinates to match exactly. This is consumer-side validation only; it does not grant publication authority or infer supersession from version order, time, diff, or semantic similarity. A language-neutral supersession/publication-receipt schema remains required before cross-language client completeness is claimed.

## 6. First Generation ↔ Client vertical

`relational schema request -> structural request admission -> source key/binding resolution -> exact-schema + trusted resource-envelope authorization -> authorized read-only source observation -> immutable observed tables/columns/constraints -> concept/relation/dimension/measure/mapping candidates -> evidence-bound validation -> steward review -> immutable semantic_release -> offline client admission/diff/integrity/supersession validation -> consuming-product ACL/query boundary`.

`ContextualWisdomLab/governance-risk-compliance` is the first reference source/client scenario, not a special-case algorithm. A shared golden fixture must exercise both Generation and Client without copying GRC truth into ConceptWeave or giving ConceptWeave direct GRC application-table access.

## 7. Non-goals for v0.1

- replacing `semantic-data-portal` as the enterprise catalog;
- owning downstream tenant/purpose authorization or physical query execution;
- arbitrary write access to source systems;
- automatic publication without review;
- treating vector similarity as semantic truth;
- copying every external ontology into one CWL namespace;
- building a generic LLM gateway or browser crawler;
- treating digest syntax validation alone as cryptographic integrity evidence;
- treating a syntactically valid source key, caller-selected schema scope, or positive caller-selected resource ceiling as authorization;
- inferring backward compatibility merely because one version number is older;
- inferring supersession from version order, timestamps, semantic similarity, or diff size;
- overwriting a published semantic release in place;
- claiming an emerging draft semantic-layer format is a stable standard.

## 8. Acceptance criteria for the first commercial candidate

- 100% owned production line/function/region and branch coverage where tooling exposes it;
- candidate-to-source provenance completeness of 100%;
- zero publication paths that bypass reviewed state;
- zero silent inferred-to-authoritative promotion;
- deterministic replay of the same immutable source snapshot and extraction configuration;
- raw source requests cannot reach the canonical adapter execution seam without registry-issued capability evidence;
- unknown registry keys fail before adapter invocation and credential material never crosses the Source Observation contract;
- exact schema scope and the complete provider-independent metadata/runtime resource envelope require explicit trusted policy admission against the same immutable source binding;
- source+schema authorization without resource policy fails closed, and a wider-than-policy resource request has zero adapter/source/snapshot side effects;
- equal or narrower policy-admitted resource controls retain their exact requested ceilings in the authorized envelope;
- end-to-end source-operation deadline includes source/binding/schema/resource authorization, connection and catalog work;
- cross-tenant access denial when tenancy is introduced;
- malformed/hostile source contracts rejected with bounded resource use;
- semantic-model release can be reproduced from source receipts and approved proposal receipts;
- consumer can validate release schema/version/governance state offline before authoritative use;
- current, explicitly supported legacy, and unknown contract versions have deterministic fail-closed compatibility outcomes;
- consumer can deterministically diff admitted releases without provider access or bypassing release admission;
- exact detached artifact digest verification succeeds only for matching bytes;
- corrections preserve the immutable predecessor and identify an explicit distinct successor by exact release id plus digest rather than version-order inference;
- a language-neutral supersession/publication receipt is validated before cross-language release consumption is called complete;
- buyer can inspect why each published artifact exists, which evidence supported it, and why/when it was explicitly superseded.
