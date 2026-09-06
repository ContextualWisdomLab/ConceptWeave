# ConceptWeave Technical Requirements Document

## 1. Architectural style

ConceptWeave starts as a Rust-first modular monolith with explicit bounded contexts and ports. Network-service extraction is deferred until independent scaling, trust, or deployment boundaries are demonstrated.

## 2. Bounded contexts

1. **Source Observation** — immutable source snapshots and parser receipts.
2. **Semantic Discovery** — evidence-bound candidate generation.
3. **Model Validation** — deterministic structural, ontology, constraint, and semantic-model validation.
4. **Governance & Publication** — review decisions, immutable releases, supersession.
5. **Interoperability** — import/export adapters and CWL anti-corruption layers.

The Core Domain is **Semantic Model Engineering**, represented by the discovery-to-publication lifecycle. Identity, LLM routing, outbound web access, observability, and catalog consumption are external/generic responsibilities.

## 3. Dependency direction

`domain <- application <- ports/contracts <- adapters <- delivery`

Domain code must not import web frameworks, databases, provider SDKs, LLM SDKs, or another CWL product's internals.

## 4. Source observation contract

Every observed source will eventually carry at least:

- source snapshot identifier;
- source kind;
- immutable content digest;
- source authority;
- observed/recorded time;
- parser/extractor version;
- tenant/workspace scope when tenancy exists;
- bounded source locations for extracted evidence.

## 5. Candidate contract

The initial Rust and JSON contracts cover candidate kind, truth status, publication state, and source evidence. Later revisions add ontology IRIs, language-tagged labels, relation endpoints, cardinality, units, measure expressions, physical mappings, confidence/evaluation receipts, and temporal validity without breaking v0.1 consumers.

## 6. LLM boundary

LLM calls go through `contextual-orchestrator`. The application sends bounded evidence/context and receives structured proposals. LLM output is never a database command, publication decision, validation result, or source-system mutation. Deterministic checks must be able to reject the output without another model call.

## 7. Standards strategy

Stable publication targets use stable recommendations first: RDF 1.1, OWL 2, SKOS, SHACL 1.0, JSON-LD 1.1, and PROV-O as applicable. RDF 1.2 and SHACL 1.2 are tracked as 2026 drafts/candidate work and are not silently treated as final standards. Apache Ossie (incubating; formerly OSI) is tracked as an emerging semantic-model exchange format for metrics, dimensions, relationships, and datasets.

## 8. Persistence

No durable product database is claimed by the foundation slice. When persistence is introduced it must be PostgreSQL, 3NF by default, use descriptive two-or-more-word `snake_case` objects, preserve business/effective time separately from system-recorded time when facts vary over time, enforce tenant-scoped references, and use explicit migration ownership rather than runtime DDL races.

## 9. Security

Source artifacts are untrusted input. Adapters must enforce source size/type bounds, parser timeouts, archive/decompression limits, SSRF-safe outbound access where external retrieval exists, and prompt-injection isolation for LLM-assisted extraction. Credentials and raw secrets never become semantic evidence.

## 10. Evaluation

Evaluation must separate extraction recall, semantic correctness, structural correctness, ontology consistency, mapping accuracy, measure correctness, and governance outcomes. Model-judge scores may supplement but never replace deterministic golden fixtures and human-reviewed expert cases.

## 11. Zotero research intake

Execution receipts retain the verified proposal/source binding in every outcome.
The authenticated-transport regression composes the executor with an ephemeral
loopback HTTP fixture: a failed POST followed by a GET matching the requested
complete state still produces an indeterminate receipt, preserves the entire
submitted request and observed state, and produces no inferred inverse. Exactly
one POST is observed. This is synthetic wire-contract evidence, not an approved
live write, provider peer authentication or real-library recovery.
On a failed or invalid write response, `indeterminate_request` preserves the exact
submitted server/library/item preconditions and complete replacement arrays;
`reconciliation_observation` retains any subsequent read without attributing it
to that request. Matching before/after state never clears uncertainty or grants an
inverse. Only earlier directly verified writes remain applied/reversible. These
serialized fields are audit evidence, not inputs authorizing execution or retry.

Classification write planning also calls the shared report validator before
external authority. Its required `proposal_digest` binds the reviewed v2 proposal,
unclassified-metadata and pending-key payloads and is retained in the plan. Legacy
receipts missing the field fail deserialization; blank bindings fail admission.
All existing local item/metadata errors retain precedence. Independent governance
authenticates the entire set; recomputing the digest cannot renew approval. This
does not replace full-text or duplicate-membership review contracts (ADR 0007).
Mode remains caller-supplied planning input, not receipt-bound execution authority.

`ClassificationReport.unclassified_items` retains every input record excluded from bibliographic classification, using the existing `ZoteroItem` metadata projection. Bibliographic proposals and this inventory are disjoint and together account for the observed record count on reader-admitted input. The existing child index is consumed once from bibliographic roots; records never reached remain in sorted `pending_source_item_keys`, including standalone roots, their descendants, orphan trees and cycles. The traversal is iterative, uses no new dependency and costs O(n log n) time/O(n) auxiliary space. It does not validate arbitrary offline input or preserve note bodies, attachment-specific fields and unknown provider JSON.

Evaluation now calls `validate_classification_report` before governance: counts, disjoint complete key/version partitions, types, direct children and recomputed pending keys must agree. Equal partition count and one successful removal of each unique snapshot coordinate prove completeness. This owner has no original parent/type snapshot coordinates, so consistency is not source authentication. Later restoration, review, duplicate and write consumers must adopt this guard and require both inventory fields without empty legacy defaults. Keep bibliographic progress distinct from whole-library completion. Full-text report-digest changes require fresh bound verification, not capture rewriting. See [source-scope evidence and integration map](doctoring/zotero_source_scope.md).

The shared live reader rejects empty or whitespace-only item keys before accumulating a page or requesting another one. It preserves valid keys verbatim; this does not add a new provider key-format restriction or certify arbitrary offline classifier input.

`conceptweave-zotero` reads only the loopback Local API with at most 100 records per page, an 8 MiB page-body limit, a 50,000-item whole-snapshot limit, a 256 MiB cumulative body limit, redirects disabled, and finite connect/response/body/global timeouts. Every request pins `Zotero-API-Version: 3`; every response must report API version 3 and a present, recorded schema revision that stays unchanged during the snapshot, rather than the historical workstation schema 42. Before another request is issued, exhausted whole-snapshot budgets fail closed. Before a parsed page is accumulated, checked item-count and byte arithmetic must remain within both the advertised total and the configured whole-snapshot budgets. `Total-Results`, `Last-Modified-Version`, Zotero version, schema revision and server identity must remain identical across all pages; contract drift, malformed JSON, an empty intermediate page, duplicate keys, or an oversized response fails the run.

One monotonic five-minute budget covers page admission and complete-report acceptance. At or beyond that limit, no new request starts and no late page or completed report is accepted. An already-started request retains the existing per-request timeouts; computation is not forcibly interrupted. Never return a partial classification to satisfy the time budget or discard legitimate short pages. On budget failure the caller receives an error, not a smaller successful denominator.

After count/byte validation and before accumulating each metadata page, every returned object's revision must be less than or equal to that page's library revision. This includes attachments, notes and annotations, not only bibliographic records. A higher revision returns the existing snapshot-consistency error immediately, with no next-page request or partial report. Zero, lower and equal revisions remain valid and are preserved exactly, including the unsigned maximum. Compare only within this metadata read of one local instance: Zotero 9's synced revisions and Zotero 10's local revisions are not interchangeable, and this condition is not a full-text endpoint contract or proof of atomicity. See the [revision admission evidence](doctoring/zotero_item_revision.md).

Every top-level bibliographic record receives exactly one proposed disposition. `NeedsStewardReview` also records a deterministic abstention reason so missing classification metadata, vocabulary unsupported by the current deterministic rules, and present-but-unmatched metadata are distinguishable. Its nonempty abstract is retained exactly once in the local report: an abstract that triggered conflicting rules remains in matched evidence, while other abstention abstracts use the review-only field. Non-abstained items omit that field. DOI duplicate identity normalization treats bare DOI values, `doi:`, `doi.org`, and legacy `dx.doi.org` resolver forms as the same identity when their normalized DOI is equal.

Duplicate decisions are independent of subject labels but share the complete source-evidence admission boundary. A reviewed set must match the raw-snapshot digest, item revisions, exact candidate membership and required v2 `proposal_digest`. Receipt comparison retains `SnapshotMismatch` precedence; shared structural/audit admission and all component/decision checks then run before the external verifier. The verifier authenticates the entire independently issued set, not a locally recomputed digest. Missing scope bindings fail deserialization and blank bindings fail admission; no legacy default or automatic reapproval exists. The manifest retains the verified proposal digest. Every operation records all component item revisions, before/after identity maps and exact rollback. Zotero records remain unchanged.

Golden-set evaluation accepts only a governance receipt verified with the complete reviewed set by a caller-owned authorization boundary. Its library version, rule revision, canonical SHA-256 content digest, and every observed parent/child item-key/item-version identity must bind the classification report. The snapshot digest covers every raw Zotero item in canonical key order. A separate required `proposal_digest` binds every field of every proposed item, including predictions, supporting evidence, and proposals outside a reviewed sample. `classification_proposal_digest` computes SHA-256 over compact JSON containing the `conceptweave-classification-proposals-v1` domain marker and proposal records sorted by item key and revision. It uses the current records, not a report's self-declared digest or a second stored source snapshot. Governance must issue and independently verify both digests together with the labels; a locally recomputed replacement digest cannot renew an old approval. Legacy approvals missing this field fail closed and require reissuance, not automatic backfill.

The proposal-only v1 format above describes the previous receipt contract. The current source-scope amendment supersedes it with `conceptweave-classification-proposals-v2`: compact JSON binds the marker, sorted proposals, key/version-sorted projected unclassified records, and sorted pending keys. Old v1 receipts fail before governance; locally rewritten digests cannot renew approval. This is not a lossless full-text digest.

Structural, source, proposal, and label checks precede the external verifier. Blank, duplicate, unknown, stale, content-mismatched, prediction-mismatched, label-mismatched, or abstention-as-truth inputs fail closed. The aggregate result retains the verified library version, rule revision, and opaque snapshot/proposal digests, but contains no item keys, reviewer identity, or bibliographic text. Production authorization remains Keyverse/governance-owned; this crate passes the complete reviewed labels and approval bindings to that boundary instead of minting authority.

Provider deserialization captures each complete JSON object before projecting metadata. Snapshot hashing serializes the domain marker `conceptweave-zotero-snapshot-v2` followed by key-ordered pairs of that canonical source JSON and the actual typed classifier input. Unknown nested fields, array order, and omitted-versus-explicit default fields remain bound; changing a typed input after decoding also changes the digest. Synthetic offline typed items have no captured provider object and bind an explicit absent-source value alongside their typed input. Earlier reduced-content digests remain historical evidence and cannot establish this complete-content contract; regenerate the report and review artifacts and obtain fresh approval before any release or approved write.

The complete metadata-review evaluator rejects unequal label cardinality or nonempty `pending_source_item_keys` with `IncompleteReview` before governance. The shared evaluator then recomputes the complete inventory and pending ancestry, so clearing pending keys and rewriting the proposal digest still fails local validation. Because shared validation rejects blank, duplicate, and unknown keys, equal cardinality proves bibliographic label coverage. Sampled evaluation still supports pending sources; completion does not prove a Zotero mutation or full-text approval.
A successful classification report carries an `audit_summary` whose snapshot, bibliographic, proposed-disposition, provenance-complete, abstention, duplicate-candidate, failure, and per-disposition counts are derived from the same in-memory immutable snapshot. Zotero item version zero remains a valid observed coordinate for never-synced Zotero 9 records; provenance completeness rejects a missing item key rather than inventing a positive-only version invariant. Reader failures return an error instead of a partial report; therefore a returned report records `failure_count=0` rather than hiding partial failures.

The review worksheet is a deterministic item-key-ordered projection of the report. It binds library/rule revisions, raw-snapshot digest, required `proposal_digest` from the existing v2 scope hash, complete item coordinates, proposal, abstention reason, and one initially empty decision per bibliographic item. Construction reuses `validate_classification_report`, mapped to `WorksheetError::InvalidReport`, before worksheet-specific nonblank identity and abstention checks. This avoids a second drifting audit implementation while admitting structurally valid pending source evidence. It deliberately omits bibliographic text and matched evidence; stewards consult the owner-only report by item key. Missing proposal binding fails deserialization. Subsequent progress, application and finalization owners must compare this field to the recomputed report binding; present-but-blank or rewritten values are not authority. Legacy worksheets must be regenerated, and independent approval remains separate.

The local report can contain titles, tags, matched metadata, and abstention abstracts. It is sensitive steward-review material, remains outside the repository, and is not a publication artifact.

The report is local JSON and contains proposals rather than governance decisions. On supported Unix platforms, CLI output is restricted to a new owner-readable/writable (`0600`) direct child of canonical `/tmp` or the operating system temporary directory; exact permissions are restored after umask application, and other platforms fail closed. Relative paths, nested paths, existing paths, and symlinks are rejected, and create-new file semantics prevent overwrite/path-swap writes. Reviewed collection/tag changes can produce a pure local plan whose default mode is dry-run. The plan requires exact report and item preconditions, complete before/after/rollback arrays, externally verified authority, and preserved Zotero tag types; its fields are externally read-only after validation. Zotero 9 execute mode fails closed. Every receipt copies the plan's review, authority, server, Zotero version, library, rule, snapshot and proposal coordinates; dry-run reports every operation as not attempted and makes no Local API call. Execute mode preflights every item before the first write, advances the library precondition only from a directly verified write response, stops on the first adapter or response failure, and re-reads that item through the same boundary as observation only. Failed writes remain indeterminate regardless of observed metadata; no inverse is issued for them. Prior directly verified operations retain their inverse coordinates. The API key remains adapter-owned and absent from serializable structures.

PR #20 retains its rollback core and adapter: mixed-server rejection precedes reads; complete current-state checks precede inverse writes; only directly verified responses advance the library version. Every failed or invalid inverse response now remains indeterminate, retaining the complete operation, exact submitted request (including its library precondition), and optional complete readback. Matching restored or unchanged metadata does not prove causal completion or termination, and the failed inverse is absent from remaining work. Earlier directly verified restorations remain recorded; remaining operations are untouched only, not automatic retry authority. The operation-slice API still lacks original-write scope and independent authority; authoritative consumer adoption remains an open gate, including empty-slice and delayed-reconciliation handling.

PR #21 retains validated delayed reads without writes and complete observed metadata. Metadata-only restored/unchanged and retry inference is removed in `f9c2c03`: the observer always retains indeterminate causal status and emits no retry operation. Eight metadata scenarios and the three-GET adapter fixture remain covered; the latter compares the entire observed state. Legacy enum variants and the optional retry field remain for contract compatibility, not as outputs or approval from this observer. Authoritative successor wrappers still must retain the complete prior rollback receipt, its exact submitted request, binding and untouched tail; an operation-only observation cannot replace that envelope or establish causal completion, termination, or independent retry authority.

The Zotero 10+ transport is pinned to loopback, rejects redirects, and uses finite timeouts. A one-shot authorization POST to `/api/local/authorize` sends JSON `{ "appName": ... }`, `Content-Type: application/json`, and the expected `Zotero-Server-ID`. Application names must be nonblank and at most 128 bytes. Every authorization, read, and write response must repeat that exact server identity before its status is interpreted. A bounded `200 OK` authorization response contains a 32-byte visible-ASCII key plus the `remember` decision. A same-server `403` is classified as denial only when its bounded JSON body parses with `denied: true`; missing, malformed, oversized, or false denial evidence fails closed. `429` exposes only a safe integer `Retry-After` delta of at most one day. Neither condition retries or prompts again. The authorization wrapper is neither debug-printable nor serializable, keeps the key private, exposes only the remembered decision, and can be consumed into the existing adapter. Item responses remain capped at 1 MiB. Writes distinguish same-server `401` reauthorization from same-server `412` stale preconditions, while a different-server `412` on library, item, or write paths is a database switch; all errors remain static and secret-free. Narrow adapter functions reuse the generic write and rollback cores. Mock TCP evidence covers the wire contract, but no approved live Zotero 10 authorization, write, partial-failure, or rollback has been performed.

Loopback pinning, redirect rejection, and `Zotero-Server-ID` continuity checks do not encrypt HTTP traffic carrying `Zotero-API-Key` and do not authenticate the local peer before that key is transmitted. `Zotero-Server-ID` is not cryptographic server authentication. Under the currently documented Zotero Local API there is no HTTPS or OS-authenticated IPC write endpoint for ConceptWeave to substitute. A hostile same-host process that can observe, bind, or interpose on the loopback endpoint therefore remains inside the unresolved credential-confidentiality threat boundary. As recorded in `THREAT_MODEL.md`, mock/local orchestration evidence is allowed, but enterprise-secure live write-back remains fail closed until Zotero provides a protected transport or an explicit product-security/governance decision narrows the supported threat model and accepts the residual same-host risk.
