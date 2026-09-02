# ConceptWeave Technical Requirements Document

## 1. Architectural style

ConceptWeave starts as a Rust-first modular monolith with explicit bounded contexts and ports. Network-service extraction is deferred until independent scaling, trust, or deployment boundaries are demonstrated.

## 2. Bounded contexts

1. **Source Observation** — immutable source snapshots and parser/extractor receipts.
2. **Semantic Discovery** — evidence-bound candidate generation.
3. **Model Validation** — deterministic structural, ontology, constraint, and semantic-model validation.
4. **Governance & Publication** — review decisions, immutable releases, supersession.
5. **Interoperability** — import/export adapters and CWL anti-corruption layers.

The Core Domain is **Semantic Model Engineering**, represented by the discovery-to-publication lifecycle. Identity, LLM routing, outbound web access, observability, and catalog consumption are external/generic responsibilities.

## 3. Dependency direction

`domain <- application <- ports/contracts <- adapters <- delivery`

Domain code must not import web frameworks, databases, provider SDKs, LLM SDKs, or another CWL product's internals. `conceptweave-observation` is a provider-independent Source Observation contract crate; live PostgreSQL connectivity belongs in an adapter crate behind an explicit application port.

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

The active PostgreSQL slice already preserves exact schema/table/column identifiers, deterministic column ordinals, source type/nullability/comments, composite PK/unique/FK coordinates, exact optional FK update/delete/match/deferrability behavior, CHECK reconstructed definitions, CHECK validation/enforcement/`NO INHERIT` state, canonical lowercase `sha256:<64 hex>` snapshot identity, extractor revision, observation time, and verified table/column/constraint receipts. CHECK SQL is evidence, not a license to infer ordered expression-column dependencies.

A live PostgreSQL adapter must operate read-only behind a Source Observation port. It must use bounded catalog queries, explicit statement/operation timeout, caller cancellation, row/byte/concurrency limits, exact identifier handling, and immutable extractor receipts. It must fail closed on partial or ambiguous catalog evidence and must not read another product's application tables through hidden coupling. PostgreSQL catalog reconstruction functions are treated as source rendering, not original DDL text.

## 5. Candidate contract

The initial Rust and JSON contracts cover candidate kind, truth status, publication state, and source evidence. Later revisions add ontology IRIs, language-tagged labels, relation endpoints, cardinality, units, measure expressions, physical mappings, confidence/evaluation receipts, and temporal validity without breaking v0.1 consumers. Generated candidates must bind to verified Source Observation receipts plus a discovery/proposal receipt before the first Generation release.

## 6. LLM boundary

LLM calls go through `contextual-orchestrator`. The application sends bounded evidence/context and receives structured proposals. LLM output is never a database command, publication decision, validation result, or source-system mutation. Deterministic checks must be able to reject the output without another model call.

## 7. Standards strategy

Stable publication targets use stable recommendations first: RDF 1.1, OWL 2, SKOS, SHACL 1.0, JSON-LD 1.1, and PROV-O as applicable. RDF 1.2 and SHACL 1.2 are tracked as 2026 drafts/candidate work and are not silently treated as final standards. Apache Ossie (incubating; formerly OSI) is tracked as an emerging semantic-model exchange format for metrics, dimensions, relationships, and datasets.

For the PostgreSQL observation adapter, PostgreSQL 18 `pg_constraint` and `pg_get_constraintdef()` are the current authoritative catalog/rendering contracts. `conenforced`, `convalidated`, `connoinherit`, FK action/match metadata, and reconstructed CHECK definitions are preserved as source evidence rather than normalized into heuristic semantics.

## 8. Persistence

No durable product database is claimed by the foundation slice. When persistence is introduced it must be PostgreSQL, 3NF by default, use descriptive two-or-more-word `snake_case` objects, preserve business/effective time separately from system-recorded time when facts vary over time, enforce tenant-scoped references, and use explicit migration ownership rather than runtime DDL races.

## 9. Security

Source artifacts are untrusted input. Adapters must enforce source size/type bounds, parser timeouts, archive/decompression limits, SSRF-safe outbound access where external retrieval exists, and prompt-injection isolation for LLM-assisted extraction. Credentials and raw secrets never become semantic evidence. Database adapters must use least-privilege read-only credentials, avoid interpolating source identifiers into SQL, and expose cancellation/resource-limit failure as typed non-success outcomes rather than truncated success.

## 10. Evaluation

Evaluation must separate extraction recall, semantic correctness, structural correctness, ontology consistency, mapping accuracy, measure correctness, and governance outcomes. Model-judge scores may supplement but never replace deterministic golden fixtures and human-reviewed expert cases. PostgreSQL extraction tests must include a frozen anonymized fixture covering schema collisions, composite keys, cross-schema FKs, FK behavior, enforced/not-enforced CHECKs, quoted identifiers, nullability/comments, and source disappearance/retry boundaries.
