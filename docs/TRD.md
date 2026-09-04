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

Every top-level bibliographic record receives exactly one proposed disposition. `NeedsStewardReview` also records a deterministic abstention reason so missing classification metadata, vocabulary unsupported by the current deterministic rules, and present-but-unmatched metadata are distinguishable. DOI duplicate identity normalization treats bare DOI values, `doi:`, `doi.org`, and legacy `dx.doi.org` resolver forms as the same identity when their normalized DOI is equal.

Golden-set evaluation accepts only a governance receipt verified with the complete reviewed set by a caller-owned authorization boundary. Its library version, rule revision, canonical SHA-256 content digest, and every observed parent/child item-key/item-version identity must bind the classification report. The digest covers every raw Zotero item in canonical key order. Blank, duplicate, unknown, stale, content-mismatched, label-mismatched, or abstention-as-truth inputs fail closed. The output retains the verified library version, rule revision, and opaque snapshot digest, but contains no item keys, reviewer identity, or bibliographic text. Production authorization remains Keyverse/governance-owned; this crate passes the complete reviewed labels to that boundary instead of minting authority.

The report is local JSON and contains proposals rather than governance decisions. CLI output is restricted to a new direct child of canonical `/tmp` or the operating system temporary directory; relative paths, nested paths, existing paths, and symlinks are rejected, and create-new file semantics prevent overwrite/path-swap writes. Zotero 9 writes are unsupported; no mutation path exists in this slice. A future Zotero 10+ writer requires a separate reviewed change with a Local API key, stable server identity, fresh item/library version preconditions, item-by-item before/after receipts, and rollback evidence.
