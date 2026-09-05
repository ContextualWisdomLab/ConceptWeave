# ConceptWeave Technical Requirements Document

## 1. Architectural style

ConceptWeave starts as a Rust-first modular monolith with explicit bounded contexts and ports. Network-service extraction is deferred until independent scaling, trust, or deployment boundaries are demonstrated.

## 2. Bounded contexts

1. **Source Observation** — immutable source snapshots and parser/extractor receipts.
2. **Semantic Discovery** — evidence-bound candidate generation.
3. **Model Validation** — deterministic structural, ontology, constraint, and semantic-model validation.
4. **Governance & Publication** — review decisions, immutable releases, supersession.
5. **Client Consumption** — release admission, compatibility, diff, exact resolution, detached-artifact integrity and supersession validation; later match/align/explain/query-plan contracts.
6. **Interoperability** — import/export adapters and CWL anti-corruption layers.

The Core Domain is **Semantic Model Engineering**, represented by the discovery-to-publication lifecycle. Client Consumption is a supporting subdomain that protects downstream consumers from incompatible or non-governed releases. Identity, LLM routing, outbound web access, observability, catalog/search and consuming-product authorization are external/generic responsibilities.

## 3. Dependency direction

`domain <- application <- ports/contracts <- adapters <- delivery`

Client Consumption depends only on versioned public release/domain contracts. Domain and client code must not import web frameworks, databases, provider SDKs, LLM SDKs, generator-private adapters, or another CWL product's internals. `conceptweave-observation` is a provider-independent Source Observation contract crate; live PostgreSQL connectivity belongs in an adapter crate behind an explicit application port.

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

The active PostgreSQL slice already preserves exact schema/table/column identifiers, deterministic column ordinals, source type/nullability/comments, composite PK/unique/FK coordinates, exact optional FK update/delete behavior including targeted `SET NULL`/`SET DEFAULT` local-column subsets, match/deferrability behavior, CHECK reconstructed definitions, CHECK validation/enforcement/`NO INHERIT` state, canonical lowercase `sha256:<64 hex>` snapshot identity, extractor revision, observation time, and verified table/column/constraint receipts. CHECK SQL is evidence, not a license to infer ordered expression-column dependencies.

A live PostgreSQL adapter must operate read-only behind the Source Observation port. The port accepts only an opaque source registry key of at most 128 bytes in lowercase multiword `snake_case`; an authorized registry lookup must issue the opaque capability required to construct the immutable snapshot, and the concrete adapter resolves that same entry to least-privilege credentials inside its Anti-Corruption Layer. Raw DSNs, URLs, shell-style connection parameters, unregistered keys, and provider connection objects cannot cross the snapshot boundary. Each request also carries a caller-selected positive provider-independent authorization-metadata budget: maximum exact-schema count plus maximum total UTF-8 bytes retained across schema identifiers. That admission is enforced before registry/database access and does not assume PostgreSQL's build-time identifier-length default. The adapter must then use bounded catalog queries, explicit statement/operation timeout, caller cancellation, row/byte/concurrency limits, exact identifier handling, and immutable extractor receipts. It must fail closed on partial or ambiguous catalog evidence and must not read another product's application tables through hidden coupling. PostgreSQL catalog reconstruction functions are treated as source rendering, not original DDL text.

## 5. Candidate contract

The initial Rust and JSON contracts cover candidate kind, truth status, publication state, and source evidence. Later revisions add ontology IRIs, language-tagged labels, relation endpoints, cardinality, units, measure expressions, physical mappings, confidence/evaluation receipts, and temporal validity without breaking v0.1 consumers. Generated candidates must bind to verified Source Observation receipts plus a discovery/proposal receipt before the first Generation release.

## 6. Semantic-release client contract

The Rust Client Consumption slice and Draft 2020-12 JSON Schema define an immutable consumer-visible contract containing:

- `release_id`;
- `contract_version`;
- `ontology_version`;
- truth and publication state;
- declared artifact digest identity;
- one or more provenance references;
- unique stable concept identifiers.

`SemanticReleaseClient` performs deterministic offline authoritative-use admission. A release is accepted only when its contract version is the explicit current version or one of the caller's explicit supported-legacy versions and its state is both `Published` and `Authoritative`. Compatibility is never inferred from version ordering. Structural construction and client admission do not grant publication authority.

`ReleaseDigest` validates canonical `sha256:<64 lowercase hex>` digest identity. `SemanticReleaseClient::verify_detached_artifact` then verifies cryptographic integrity of the exact detached immutable semantic-artifact bytes supplied by the caller, after applying the same authoritative-use admission gate. The release manifest declares the detached artifact digest; the digest is not specified as a self-referential hash of the manifest bytes containing that field.

The current Client slice also provides deterministic release diff, exact concept resolution, and explicit immutable supersession validation. `ReleaseSupersession` binds predecessor and successor release ids to their exact artifact digests and never infers replacement from ordering, timestamps, semantic diff, or similarity. A language-neutral supersession/publication-receipt schema is still required before cross-language completeness is claimed.

Remaining Issue #3 work includes signature/provenance-chain validation when Governance & Publication stabilizes a signing contract; relation/mapping/dimension/measure resolution; research-backed match/alignment/explanation; semantic query-plan contracts; GRC reference-client fixtures; and generated bindings after the language-neutral seam stabilizes. LLM-assisted client operations are optional and route only through `contextual-orchestrator`; admission, compatibility, diff, exact resolution, integrity and supersession validation remain deterministic and provider-independent.

## 7. LLM boundary

LLM calls go through `contextual-orchestrator`. The application sends bounded evidence/context and receives structured proposals. LLM output is never a database command, publication decision, validation result, source-system mutation, client authorization decision, or automatic authoritative alignment. Deterministic checks must be able to reject the output without another model call.

## 8. Standards strategy

Stable publication targets use stable recommendations first: RDF 1.1, OWL 2, SKOS, SHACL 1.0, JSON-LD 1.1, and PROV-O as applicable. RDF 1.2 and SHACL 1.2 are tracked as 2026 drafts/candidate work and are not silently treated as final standards. Apache Ossie (incubating; formerly OSI) is tracked as an emerging semantic-model exchange format for metrics, dimensions, relationships, and datasets.

For the PostgreSQL observation adapter, PostgreSQL 18 `pg_constraint` and `pg_get_constraintdef()` are the current authoritative catalog/rendering contracts. `conenforced`, `convalidated`, `connoinherit`, FK action/match metadata, and reconstructed CHECK definitions are preserved as source evidence rather than normalized into heuristic semantics.

## 9. Persistence

No durable product database is claimed by the current slices. When persistence is introduced it must be PostgreSQL, 3NF by default, use descriptive two-or-more-word `snake_case` objects, preserve business/effective time separately from system-recorded time when facts vary over time, enforce tenant-scoped references, and use explicit migration ownership rather than runtime DDL races. Published releases are immutable; correction creates a superseding release. Item-level UPSERT behavior must be explicit and idempotency-tested before any mutable pre-publication persistence is introduced.

## 10. Security

Source artifacts and release payloads are untrusted input. Adapters must enforce source size/type bounds, parser timeouts, archive/decompression limits, SSRF-safe outbound access where external retrieval exists, and prompt-injection isolation for LLM-assisted extraction. Credentials and raw secrets never become semantic evidence. Database adapters must use least-privilege read-only credentials, resolve credentials only from approved opaque registry keys, reject over-budget schema authorization metadata before registry/database access, avoid interpolating source identifiers into SQL, and expose cancellation/resource-limit failure as typed non-success outcomes rather than truncated success. Client admission validates governance/compatibility but does not replace consuming-product tenant/purpose authorization. Exact detached artifact integrity must be verified against the declared digest before bytes are trusted as the referenced semantic artifact.

## 11. Evaluation

Evaluation must separate extraction recall, semantic correctness, structural correctness, ontology consistency, mapping accuracy, measure correctness, release compatibility/admission correctness, and governance outcomes. Model-judge scores may supplement but never replace deterministic golden fixtures and human-reviewed expert cases. PostgreSQL extraction tests must include a frozen anonymized fixture covering schema collisions, composite keys, cross-schema FKs, FK behavior, enforced/not-enforced CHECKs, quoted identifiers, nullability/comments, request-metadata admission, and source disappearance/retry boundaries. Client matching later uses OAEI-style precision/recall/F1 and candidate-retrieval recall; release admission and integrity use deterministic malformed/version/state/provenance/digest/tamper fixtures.
