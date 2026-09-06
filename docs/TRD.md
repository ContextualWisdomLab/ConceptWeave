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

`ClassificationReport.unclassified_items` retains every input record excluded from bibliographic classification, using the existing `ZoteroItem` metadata projection. Bibliographic proposals and this inventory are disjoint and together account for the observed record count on reader-admitted input. The existing child index is consumed once from bibliographic roots; records never reached remain in sorted `pending_source_item_keys`, including standalone roots, their descendants, orphan trees and cycles. The traversal is iterative, uses no new dependency and costs O(n log n) time/O(n) auxiliary space. It does not validate arbitrary offline input or preserve note bodies, attachment-specific fields and unknown provider JSON.

Consumer requirement, still pending forward integration: require both inventory fields rather than defaulting absent legacy fields to empty; validate exact snapshot identity/version/parent complement and recompute pending keys before any review, duplicate evaluation or write verifier. Keep bibliographic progress distinct from whole-library completion. An empty pending list is only ancestry accounting, never semantic approval. Full-text report-digest changes require fresh bound verification, not capture rewriting; metadata proposal-only digests do not implicitly bind the new inventory. See [source-scope evidence and integration map](doctoring/zotero_source_scope.md).

The shared live reader rejects empty or whitespace-only item keys before accumulating a page or requesting another one. It preserves valid keys verbatim; this does not add a new provider key-format restriction or certify arbitrary offline classifier input.

`conceptweave-zotero` reads only the loopback Local API with at most 100 records per page, an 8 MiB page-body limit, a 50,000-item whole-snapshot limit, a 256 MiB cumulative body limit, redirects disabled, and finite connect/response/body/global timeouts. Every request pins `Zotero-API-Version: 3`; every response must report API version 3 and a present, recorded schema revision that stays unchanged during the snapshot, rather than the historical workstation schema 42. Before another request is issued, exhausted whole-snapshot budgets fail closed. Before a parsed page is accumulated, checked item-count and byte arithmetic must remain within both the advertised total and the configured whole-snapshot budgets. `Total-Results`, `Last-Modified-Version`, Zotero version, schema revision and server identity must remain identical across all pages; contract drift, malformed JSON, an empty intermediate page, duplicate keys, or an oversized response fails the run.

One monotonic five-minute budget covers page admission and complete-report acceptance. At or beyond that limit, no new request starts and no late page or completed report is accepted. An already-started request retains the existing per-request timeouts; computation is not forcibly interrupted. Never return a partial classification to satisfy the time budget or discard legitimate short pages. On budget failure the caller receives an error, not a smaller successful denominator.

After count/byte validation and before accumulating each metadata page, every returned object's revision must be less than or equal to that page's library revision. This includes attachments, notes and annotations, not only bibliographic records. A higher revision returns the existing snapshot-consistency error immediately, with no next-page request or partial report. Zero, lower and equal revisions remain valid and are preserved exactly, including the unsigned maximum. Compare only within this metadata read of one local instance: Zotero 9's synced revisions and Zotero 10's local revisions are not interchangeable, and this condition is not a full-text endpoint contract or proof of atomicity. See the [revision admission evidence](doctoring/zotero_item_revision.md).

Every top-level bibliographic record receives exactly one proposed disposition. `NeedsStewardReview` also records a deterministic abstention reason so missing classification metadata, vocabulary unsupported by the current deterministic rules, and present-but-unmatched metadata are distinguishable. DOI duplicate identity normalization treats bare DOI values, `doi:`, `doi.org`, and legacy `dx.doi.org` resolver forms as the same identity when their normalized DOI is equal.

Golden-set evaluation accepts only a governance receipt verified with the complete reviewed set by a caller-owned authorization boundary. Its library version, rule revision, canonical SHA-256 content digest, and every observed parent/child item-key/item-version identity must bind the classification report. The snapshot digest covers every raw Zotero item in canonical key order. A separate required `proposal_digest` binds every field of every proposed item, including predictions, supporting evidence, and proposals outside a reviewed sample. `classification_proposal_digest` computes SHA-256 over compact JSON containing the `conceptweave-classification-proposals-v1` domain marker and proposal records sorted by item key and revision. It uses the current records, not a report's self-declared digest or a second stored source snapshot. Governance must issue and independently verify both digests together with the labels; a locally recomputed replacement digest cannot renew an old approval. Legacy approvals missing this field fail closed and require reissuance, not automatic backfill.

Structural, source, proposal, and label checks precede the external verifier. Blank, duplicate, unknown, stale, content-mismatched, prediction-mismatched, label-mismatched, or abstention-as-truth inputs fail closed. The aggregate result retains the verified library version, rule revision, and opaque snapshot/proposal digests, but contains no item keys, reviewer identity, or bibliographic text. Production authorization remains Keyverse/governance-owned; this crate passes the complete reviewed labels and approval bindings to that boundary instead of minting authority.

Provider deserialization captures each complete JSON object before projecting metadata. Snapshot hashing serializes the domain marker `conceptweave-zotero-snapshot-v2` followed by key-ordered pairs of that canonical source JSON and the actual typed classifier input. Unknown nested fields, array order, and omitted-versus-explicit default fields remain bound; changing a typed input after decoding also changes the digest. Synthetic offline typed items have no captured provider object and bind an explicit absent-source value alongside their typed input. Earlier reduced-content digests remain historical evidence and cannot establish this complete-content contract; regenerate the report and review artifacts and obtain fresh approval before any release or approved write.

The report is local JSON and contains proposals rather than governance decisions. CLI output is restricted to a new direct child of canonical `/tmp` or the operating system temporary directory; relative paths, nested paths, existing paths, and symlinks are rejected, and create-new file semantics prevent overwrite/path-swap writes. Zotero 9 writes are unsupported; no mutation path exists in this slice. A future Zotero 10+ writer requires a separate reviewed change with a Local API key, stable server identity, fresh item/library version preconditions, item-by-item before/after receipts, and rollback evidence.
