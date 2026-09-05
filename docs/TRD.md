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

`conceptweave-zotero` reads only the loopback Local API with at most 100 records per page, an 8 MiB page-body limit, a 50,000-item whole-snapshot limit, a 256 MiB cumulative body limit, redirects disabled, and finite connect/response/body/global timeouts. Every request pins `Zotero-API-Version: 3`; every response must report API version 3, while its schema version is recorded and must remain stable across the snapshot. Before another request is issued, exhausted whole-snapshot budgets fail closed. Before a parsed page is accumulated, checked item-count and byte arithmetic must remain within both the advertised total and the configured whole-snapshot budgets. `Total-Results`, `Last-Modified-Version`, Zotero version, and server identity must remain identical across all pages; contract drift, malformed JSON, an empty intermediate page, duplicate keys, or an oversized response fails the run.

Every top-level bibliographic record receives exactly one proposed disposition. `NeedsStewardReview` also records a deterministic abstention reason so missing classification metadata, vocabulary unsupported by the current deterministic rules, and present-but-unmatched metadata are distinguishable. Its nonempty abstract is retained exactly once in the local report: an abstract that triggered conflicting rules remains in matched evidence, while other abstention abstracts use the review-only field. Non-abstained items omit that field. DOI duplicate identity normalization treats bare DOI values, `doi:`, `doi.org`, and legacy `dx.doi.org` resolver forms as the same identity when their normalized DOI is equal.

Duplicate review is independent of subject classification. A reviewed decision set must match the exact raw-snapshot digest and complete item-key/item-version coordinates, cover every duplicate candidate exactly once, select one retained key from the connected duplicate component, and pass an external governance verifier. Every operation records all component item revisions, identity mappings before and after canonicalization, and the exact rollback mapping. These mappings affect only downstream identity resolution; Zotero records are neither mutated nor deleted.

Golden-set evaluation accepts only a governance receipt verified with the complete reviewed set by a caller-owned authorization boundary. Its library version, rule revision, canonical SHA-256 content digest, and every observed parent/child item-key/item-version identity must bind the classification report. The snapshot digest covers every raw Zotero item in canonical key order. A separate required `proposal_digest` binds every field of every proposed item, including predictions, supporting evidence, and proposals outside a reviewed sample. `classification_proposal_digest` computes SHA-256 over compact JSON containing the `conceptweave-classification-proposals-v1` domain marker and proposal records sorted by item key and revision. It uses the current records, not a report's self-declared digest or a second stored source snapshot. Governance must issue and independently verify both digests together with the labels; a locally recomputed replacement digest cannot renew an old approval. Legacy approvals missing this field fail closed and require reissuance, not automatic backfill.

Structural, source, proposal, and label checks precede the external verifier. Blank, duplicate, unknown, stale, content-mismatched, prediction-mismatched, label-mismatched, or abstention-as-truth inputs fail closed. The aggregate result retains the verified library version, rule revision, and opaque snapshot/proposal digests, but contains no item keys, reviewer identity, or bibliographic text. Production authorization remains Keyverse/governance-owned; this crate passes the complete reviewed labels and approval bindings to that boundary instead of minting authority.

Provider deserialization captures each complete JSON object before projecting metadata. Snapshot hashing serializes the domain marker `conceptweave-zotero-snapshot-v2` followed by key-ordered pairs of that canonical source JSON and the actual typed classifier input. Unknown nested fields, array order, and omitted-versus-explicit default fields remain bound; changing a typed input after decoding also changes the digest. Synthetic offline typed items have no captured provider object and bind an explicit absent-source value alongside their typed input. Earlier reduced-content digests remain historical evidence and cannot establish this complete-content contract; regenerate the report and review artifacts and obtain fresh approval before any release or approved write.

The full-reclassification evaluator checks label cardinality before that boundary and additionally requires the reviewed label count to equal the unique classified bibliographic-item count; because the base evaluator rejects blank, duplicate, and unknown keys, equality proves complete coverage. A sampled golden set can measure quality but cannot satisfy this completion gate.
A successful classification report carries an `audit_summary` whose snapshot, bibliographic, proposed-disposition, provenance-complete, abstention, duplicate-candidate, failure, and per-disposition counts are derived from the same in-memory immutable snapshot. Zotero item version zero remains a valid observed coordinate for never-synced Zotero 9 records; provenance completeness rejects a missing item key rather than inventing a positive-only version invariant. Reader failures return an error instead of a partial report; therefore a returned report records `failure_count=0` rather than hiding partial failures.

The review worksheet is a deterministic item-key-ordered projection of the report. `conceptweave-zotero --worksheet /tmp/REPORT.json /tmp/WORKSHEET.json` reads one live snapshot and creates both owner-only outputs as a bound pair; failed output removes partial files so the same paths can be retried. It binds the library version, rule revision, raw-snapshot digest, complete parent/child item coordinates, item proposal, abstention reason, and an initially empty decision for every bibliographic item. Construction rejects blank or duplicate snapshot identity, mismatched item revisions, and inconsistent observed, bibliographic, proposal, provenance, abstention, duplicate, failure, or disposition counts. It deliberately omits bibliographic text and matched evidence; stewards consult the paired owner-only report by item key.

The local report can contain titles, tags, matched metadata, and abstention abstracts. It is sensitive steward-review material, remains outside the repository, and is not a publication artifact.

The report is local JSON and contains proposals rather than governance decisions. On supported Unix platforms, CLI output is restricted to a new owner-readable/writable (`0600`) direct child of canonical `/tmp` or the operating system temporary directory; exact permissions are restored after umask application, and other platforms fail closed. Relative paths, nested paths, existing paths, and symlinks are rejected, and create-new file semantics prevent overwrite/path-swap writes. Reviewed collection/tag changes can produce a pure local plan whose default mode is dry-run. The plan requires exact report and item preconditions, complete before/after/rollback arrays, externally verified authority, and preserved Zotero tag types; its fields are externally read-only after validation. Zotero 9 execute mode fails closed. Every receipt copies the plan's review, authority, server, Zotero version, library, rule, and snapshot coordinates; dry-run reports every operation as not attempted and makes no Local API call. Execute mode preflights every item before the first write, advances the library precondition only from verified state, stops on the first adapter or response failure, and re-reads that item through the same boundary. A proven applied state receives reverse-ordered rollback evidence containing server identity, post-write item revision, expected post-write metadata, and the complete restoration state even when the write response was lost. An unexpected state stays indeterminate, but receives an inverse operation only when the same server/item and newer library/item versions prove a safe rollback target. The generic rollback core rejects operations spanning server identities before any read, then reads every receipt item at one current library version and verifies that evidence before writing. It follows receipt order, advances the library version only after a verified inverse write, and on failure re-reads the item to classify restored, unchanged, or indeterminate state. Its secret-free receipt separates restored, failed, indeterminate, not-attempted, and remaining work. Automatic retry evidence includes a failed current operation only when it is proven unchanged; an indeterminate operation and its complete metadata are retained separately for operator reconciliation. The delayed reconciliation boundary performs exactly one server-bound read and no write. It treats an exact item revision plus expected metadata as unchanged even if unrelated library changes advanced the library version, and treats restoration metadata as restored only at a newer item revision; every identity, metadata, or version ambiguity remains indeterminate. Already consumed evidence fails preflight on reuse.

The Zotero 10+ transport is pinned to loopback, rejects redirects, and uses finite timeouts. A one-shot authorization POST to `/api/local/authorize` sends JSON `{ "appName": ... }`, `Content-Type: application/json`, and the expected `Zotero-Server-ID`. Application names must be nonblank and at most 128 bytes. Every authorization, read, and write response must repeat that exact server identity before its status is interpreted. A bounded `200 OK` authorization response contains a 32-byte visible-ASCII key plus the `remember` decision. A same-server `403` is classified as denial only when its bounded JSON body parses with `denied: true`; missing, malformed, oversized, or false denial evidence fails closed. `429` exposes only a safe integer `Retry-After` delta of at most one day. Neither condition retries or prompts again. The authorization wrapper is neither debug-printable nor serializable, keeps the key private, exposes only the remembered decision, and can be consumed into the existing adapter. Item responses remain capped at 1 MiB. Writes distinguish same-server `401` reauthorization from same-server `412` stale preconditions, while a different-server `412` on library, item, or write paths is a database switch; all errors remain static and secret-free. Narrow adapter functions reuse the generic write and rollback cores. Mock TCP evidence covers the wire contract, but no approved live Zotero 10 authorization, write, partial-failure, or rollback has been performed.

Loopback pinning, redirect rejection, and `Zotero-Server-ID` continuity checks do not encrypt HTTP traffic carrying `Zotero-API-Key` and do not authenticate the local peer before that key is transmitted. `Zotero-Server-ID` is not cryptographic server authentication. Under the currently documented Zotero Local API there is no HTTPS or OS-authenticated IPC write endpoint for ConceptWeave to substitute. A hostile same-host process that can observe, bind, or interpose on the loopback endpoint therefore remains inside the unresolved credential-confidentiality threat boundary. As recorded in `THREAT_MODEL.md`, mock/local orchestration evidence is allowed, but enterprise-secure live write-back remains fail closed until Zotero provides a protected transport or an explicit product-security/governance decision narrows the supported threat model and accepts the residual same-host risk.
