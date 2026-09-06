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

A live PostgreSQL adapter must operate read-only behind the Source Observation port. The raw `ObservationRequest` accepts only an opaque source registry key of at most 128 bytes in lowercase multiword `snake_case`; syntax alone is not source authority. Its exact schema allowlist is selection metadata until policy approves it: callers may not turn a recognized source key into authority for arbitrary schemas. `ObservationRequest::authorize` first resolves the exact key through the caller's local `SourceConnectionRegistry` and requires that registry to issue a nonblank opaque immutable connection-policy binding for the current mapping. It then requires the same policy boundary to authorize the exact sorted schema scope against that `ResolvedSourceConnection`, not against the mutable key alone. Both binding resolution and schema authorization default to fail closed. Successful authorization therefore binds the validated request to a source key plus immutable policy revision inside `AuthorizedObservationRequest`. A known key without a binding is rejected, and an unauthorized exact schema scope cannot reach the adapter execution seam.

The connection-policy binding is provider-independent provenance. It must not contain a DSN, credential, token, provider connection object, or wall-clock timestamp. A concrete adapter ACL may resolve least-privilege credentials only for the exact authorized key-and-binding pair. If the registry remaps key K from revision A to revision B after authorization, a capability issued for A must fail before credential/source access rather than silently retarget to B. Exact schema authorization must also have been evaluated against A. This is the port-level defense against mutable-key TOCTOU; the concrete adapter remains responsible for proving the corresponding ACL behavior against real credential/source resolution.

Registry authorization is a synchronous local policy boundary, not remote credential resolution. The operation's monotonic budget starts before key lookup, policy-binding resolution and schema authorization; an exhausted authorization returns `ObservationRequestError::OperationTimeout`, and the authorized envelope privately retains the monotonic start coordinate. The only timing capability exposed to adapter code is `remaining_operation_budget() -> Option<Duration>`; no wall-clock timestamp or runtime-specific type crosses the port contract.

`SourceObservationPort::observe` is an awaitable, `Send` execution seam so an asynchronous source driver can be awaited without a hidden blocking bridge or a runtime dependency in the port crate. Registry implementations at this boundary must remain bounded local authorization lookups; remote credential/network work belongs after authorization in the adapter and is capped by the remaining operation budget.

Each request also carries a caller-selected positive provider-independent authorization-metadata budget: maximum exact-schema count plus maximum total UTF-8 bytes retained across schema identifiers. That admission is enforced before registry/database access and does not assume PostgreSQL's build-time identifier-length default. Exact schema policy is case-sensitive and normalization-free; a differently cased or Unicode-normalized identifier is not implicitly granted. The adapter must then use bounded catalog queries, explicit statement/operation timeout, caller cancellation, row/byte/concurrency limits, exact identifier handling, and immutable extractor receipts. Registry lookup/binding/scope authorization, connection, transaction and catalog work share one non-resetting operation budget. Before each potentially blocking adapter stage, the implementation must read the remaining budget and cap driver/server work accordingly rather than reusing the original duration. It must fail closed on an exhausted budget, cancellation, stale binding, partial or ambiguous catalog evidence, and source disappearance, and must not read another product's application tables through hidden coupling. PostgreSQL catalog reconstruction functions are treated as source rendering, not original DDL text.

Canonical `PostgresSchemaSnapshot::new` remains a second authorization boundary: it accepts the complete `AuthorizedObservationRequest`, retains the exact opaque connection-policy binding as provenance, and rejects every locally observed table whose exact schema name is absent from the already-authorized request scope before digest or receipt issuance. This defense-in-depth check does not replace registry scope authorization. Foreign-key target schema names observed from an authorized local table remain relationship evidence and do not themselves grant authority to read the referenced schema. The source-content digest intentionally excludes source key and policy binding; those are separate immutable provenance coordinates. Every public `SourceObservationReceipt` therefore retains the exact binding alongside source id, digest, extractor revision, observation time and verified location.

The current port repair makes exact source+immutable-policy-binding+schema authorization, remaining budget, stale-binding rejection at the port seam, snapshot-side scope containment, and binding-preserving immutable receipts representable. It does not claim that a concrete PostgreSQL adapter or runtime conformance exists. Exact-head execution must still prove the contract before ADR 0004 can become Accepted.

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

Source artifacts and release payloads are untrusted input. Adapters must enforce source size/type bounds, parser timeouts, archive/decompression limits, SSRF-safe outbound access where external retrieval exists, and prompt-injection isolation for LLM-assisted extraction. Credentials and raw secrets never become semantic evidence. Database adapters must use least-privilege read-only credentials, accept source execution only through an `AuthorizedObservationRequest` whose exact source key, immutable policy binding and exact schema scope were approved by the local registry policy, resolve credentials only from that exact opaque capability, reject stale bindings before source access, reject over-budget schema metadata before registry/database access, preserve the non-resetting remaining operation budget, avoid interpolating source identifiers into SQL, and expose cancellation/resource-limit failure as typed non-success outcomes rather than truncated success. The registry's binding and schema-scope decisions default to deny and must not normalize case or Unicode to broaden access. Snapshot construction independently checks observed local schemas against the authorized request scope and public receipts retain the exact policy binding that produced the observation. Client admission validates governance/compatibility but does not replace consuming-product tenant/purpose authorization. Exact detached artifact integrity must be verified against the declared digest before bytes are trusted as the referenced semantic artifact.

## 11. Evaluation

Evaluation must separate extraction recall, semantic correctness, structural correctness, ontology consistency, mapping accuracy, measure correctness, release compatibility/admission correctness, and governance outcomes. Model-judge scores may supplement but never replace deterministic golden fixtures and human-reviewed expert cases. PostgreSQL extraction tests must include a frozen anonymized fixture covering schema collisions, composite keys, cross-schema FKs, FK behavior, enforced/not-enforced CHECKs, quoted identifiers, nullability/comments, request-metadata admission, source-key authorization, missing/blank connection-policy binding, exact schema-scope denial and positive control, same-binding authorization, stale-binding rejection before source/snapshot side effects, immutable receipt binding propagation, partial and exhausted authorization-budget consumption, timeout precedence after a slow denied registry lookup, awaitable cancellation/execution, and source disappearance/retry boundaries. Client matching later uses OAEI-style precision/recall/F1 and candidate-retrieval recall; release admission and integrity use deterministic malformed/version/state/provenance/digest/tamper fixtures.