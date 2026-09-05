# ADR 0004 — Bounded Source Observation port

- **Status:** Proposed
- **Date:** 2026-09-02
- **Owners:** Source Observation bounded context
- **Related:** Issue #2, PR #6, ADR 0001, `docs/product-technical-gap-baseline.md`

## Problem

ConceptWeave needs to observe PostgreSQL metadata without turning connectivity into hidden coupling or allowing an adapter to inspect unauthorized schemas, run without bounds, fabricate partial snapshots after source disappearance, leak credentials into domain contracts, or let callers assert immutable snapshot identity.

Three independent admission/integrity gaps are material to this boundary. First, a syntactically valid caller-supplied digest is not proof that the digest was computed from the observed metadata. Second, captured catalog row/byte limits do not bound the caller-owned exact-schema allowlist retained before source access. Third, a registry capability is not an authorization boundary if the primary adapter method can still accept a raw request and succeed without registry resolution.

A fourth execution-seam gap appears when the concrete adapter is asynchronous: a synchronous source port forces an async database adapter either to hide a nested executor/blocking bridge or to push scheduling workarounds into every caller. That would make cancellation and the single end-to-end operation deadline harder to prove at the canonical boundary.

## Constraints

- Source systems are read-only inputs; ConceptWeave does not own their business truth.
- Only a bounded opaque registry key may appear in request/domain objects: at most 128 bytes, lowercase multiword `snake_case`. Passwords, tokens, DSNs, URLs, shell-style connection parameters and provider connection objects do not cross this boundary.
- Key syntax is admission hygiene, not authorization. An authorized `SourceConnectionRegistry` must issue the opaque capability before a request can reach `SourceObservationPort` execution.
- Every request has a non-empty exact-schema allowlist, positive caller-selected schema-count/total-UTF-8-byte admission budget, and positive operation/statement-timeout, row, byte and concurrency bounds.
- Request authorization metadata is rejected before registry or database access when it exceeds policy.
- Exact source identifiers preserve original source text. Ordering may be canonicalized; identifier meaning is never normalized or truncated.
- Caller cancellation, source disappearance, malformed captures and resource exhaustion fail closed and do not produce a partial snapshot.
- Snapshot content identity is computed by Source Observation from complete owned observed metadata. Caller digest syntax is not content authority.
- Source registry identity, extractor revision and observation time remain provenance coordinates, not source-content bytes.
- Digest framing is versioned and domain-separated.
- Request construction and registry authorization remain deterministic pre-adapter operations. Live adapter execution is awaitable and returns a `Send` future; the port crate does not select or depend on an async runtime.
- The port remains provider-independent and free of PostgreSQL drivers, credentials, semantic inference, publication and LLM responsibilities.
- The concrete PostgreSQL adapter remains outside `conceptweave-domain`, `conceptweave-observation` and the port contract. Credential resolution stays inside its Anti-Corruption Layer.

## Options considered

### Put execution policy into `conceptweave-observation`

Rejected. That crate owns immutable observation facts. Driver execution policy would collapse fact identity and live-source concerns into one aggregate boundary.

### Let every adapter define its own authorization, timeout and failure vocabulary

Rejected. Resource safety and authorization would become adapter convention rather than a reusable product contract, weakening conformance and allowing silent divergence.

### Keep `SourceObservationPort::observe` synchronous and bridge async adapters internally

Rejected. The maintained PostgreSQL adapter line is asynchronous. A synchronous canonical port would either hide `block_on`/nested-runtime policy in the adapter or force callers to wrap a logically asynchronous source operation as blocking work. Both choices leak scheduling policy across the boundary and make cancellation plus one end-to-end deadline less auditable.

### Treat a well-formed registry key as sufficient source authority

Rejected. Syntax cannot establish whether the caller is allowed to observe the named source. The earlier `SourceObservationPort::observe(&ObservationRequest, ...)` shape demonstrated the problem: an implementation could succeed without ever consulting `SourceConnectionRegistry` while satisfying the trait.

### Hard-code PostgreSQL's identifier-length default as request-memory policy

Rejected. PostgreSQL build defaults are provider implementation details, not ConceptWeave authorization-memory policy. Exact identifiers are source evidence and may not be truncated to fit a convenience constant.

### Pass a raw connection string or arbitrary SQL callback

Rejected. It would cross credential boundaries, make read-only enforcement unauditable and erase Source Observation ubiquitous language.

### Trust an adapter-supplied SHA-256 string as snapshot identity

Rejected. Canonical `sha256:<64 lowercase hex>` syntax proves representation shape only, not binding to tables, columns or constraints.

### Canonicalize through a general JSON or CBOR wire format now

Deferred. RFC 8949 deterministic CBOR is a suitable benchmark for a future cross-language artifact, but the current digest is an internal Rust aggregate identity. Introducing a general wire format now would create a serialization commitment the current boundary does not need.

### Provider-independent request + authorized awaitable execution envelope + owner-computed digest

Selected. `conceptweave-source-port` owns request admission, authorization capability binding, cancellation and fail-closed execution outcomes. `conceptweave-observation` owns immutable facts and source-content identity. The live execution method returns a runtime-neutral `Send` future while deterministic admission and registry authorization stay outside the adapter await point.

## Decision

`ObservationLimits` requires positive operation/statement-timeout, row, byte and concurrency limits. `ObservationRequestBudget` separately requires positive maximum schema count and total retained UTF-8 schema bytes. Caller/application policy selects these values explicitly; no provider-derived default is embedded.

`ObservationRequest` accepts a bounded opaque source registry key plus a non-empty exact schema allowlist. It rejects raw connection material, malformed/generic keys, blank or duplicate schema identifiers and over-budget authorization metadata before registry or database access. It sorts only the allowlist order for deterministic request identity.

`SourceConnectionRegistry` is the authorization boundary for the opaque key. `ObservationRequest::authorize` consumes a validated request, resolves its exact key through that registry and returns `AuthorizedObservationRequest`, which privately binds the request to the resulting `ResolvedSourceConnection`. `SourceObservationPort::observe` accepts only `AuthorizedObservationRequest`; a raw request or unknown key therefore cannot reach the adapter execution seam through the canonical port API. The authorization envelope contains no credential material. The concrete adapter resolves its already-authorized opaque capability to least-privilege credentials inside its own ACL.

`SourceObservationPort::observe` is awaitable and returns `impl Future<Output = Result<Snapshot, SourceObservationFailure>> + Send`. `SourceObservationPort` and `ObservationCancellation` are `Sync`, allowing their shared references to cross an await point without introducing a runtime dependency. This is an execution-shape contract only: request validation, source-registry authorization, snapshot authority, and typed failure semantics are unchanged.

Registry authorization remains part of the same end-to-end operation policy as connection and catalog work. The concrete application/adapter integration must demonstrate that the configured operation deadline covers authorization, connection and all catalog work rather than treating authorization as an unbounded pre-step or restarting the budget when the awaitable adapter begins. ADR 0004 remains Proposed until that runtime conformance is implemented and verified.

`PostgresSchemaSnapshot` computes its own `sha256:` identity after exact table ordering is canonicalized. Digest input uses domain separator `conceptweave.postgres_schema_snapshot.v1` and explicit length-prefixed binary framing. Strings use exact UTF-8 bytes without Unicode/case/quoting normalization. Lengths are unsigned 64-bit big-endian, column ordinals unsigned 32-bit big-endian, and booleans/options/constraint variants/FK actions/match types/deferrability use explicit stable tags. Ordered composite-key and FK coordinates remain order-significant source evidence.

The v1 envelope includes exact table identifiers; column name/ordinal/type/nullability/comment; PK/unique/FK/CHECK fields; optional FK reference behavior and targeted delete columns; validation/enforcement state; CHECK definition; and `NO INHERIT`. It excludes `source_connection_key`, extractor revision and observation time. Receipts expose only the owner-computed digest plus separate provenance coordinates.

SHA-256 follows NIST FIPS 180-4. A future framing revision must use a new domain/version and migration contract rather than silently reinterpret v1. A future published cross-language observation artifact may adopt deterministic CBOR under RFC 8949; v1 does not claim CBOR compatibility.

This decision does **not** claim a production PostgreSQL adapter exists. The next implementation must select a maintained Rust driver, resolve credentials only from the authorized opaque capability, establish explicit read-only session/transaction behavior, enforce all budgets and cancellation in execution, produce complete-or-fail immutable observations, and prove source-disappearance behavior against a frozen anonymized reference fixture.

## Evidence

- `7cafba262aca070fa6bdccc95284641436a81224` — test-first bounded resource/allowlist/cancellation contract.
- `016b0aff5a6866d6071e02dd1afa6e116a8ce92b` — provider-independent port implementation.
- `2f6cd4e6f80b60a0d8118de2162d974bbabde4cc` / `339222cba31f126a5f5f36fe00f890fc82c4aa79` — credential-shaped key rejection and bounded opaque registry-key production contract.
- `729820490f7d072d28444432a082d9fae263f194` — 128-byte registry-key edge coverage.
- `2194a4ed1b8262d76dca0e7708cfd30114372a2b`, `d073aed`, `a39fa08`, `38ecdf0` plus production successors — targeted FK delete coordinates and registry-resolved snapshot identity.
- `5ee0e1edf8a2da527aefd4fe7ad2003d79b87ac6` — test-first digest-integrity predicate.
- `301452ae2744080406f4075fe197c16d7c35cd2d` — owner-computed deterministic snapshot digest.
- `b7e54ae2b4fe9bea20d42b2d95e8c25c118a1f5f` / `94927ec3c7763c4b53cbcefd01b510030122d1db` — request authorization-metadata budget RED/production repair.
- `8ed91afcf520efdd53c9103b332d3e277db29a03` — checked fail-closed schema-byte accumulation.
- Review `5120378921` — raw request could reach the source execution seam without registry capability evidence.
- `a372d6729364347315db1ad9a75efc49c779fbb9` — test-first contract requiring an authorized execution request.
- `5caf10b144b8254946e5d80840b0f200c0d36651` — `AuthorizedObservationRequest` and authorized-only `SourceObservationPort::observe` production repair.
- `b2b83c0fdc78af11e3e0df8cf6993216dd9c6004` — compile-contract RED requiring a provider-independent async adapter to return a `Send` future without an async-runtime dependency.
- `638be096f444fd22755160972285dbb9f0eb0364` — awaitable `SourceObservationPort` production seam.
- `04c0ded682607bb43f5e9b08b6767e113b8221d8`, `f82efca04fb897d5bc0ac78de83555239952016b`, `03b0b0d4cf7236f9bd86145b35d21b8be5b7c360` — existing port/cancellation and zero-side-effect fixtures adapted without weakening their assertions.
- NIST FIPS 180-4 — SHA-256 primitive. RFC 8949 — deterministic encoding benchmark for a future cross-language artifact.
- Exact-head hosted Product evidence remains required; predecessor, local-only, queued or superseded evidence is not completion evidence.

## Risks and mitigations

- **Configuration without enforcement:** a concrete adapter can accept budgets but ignore them. Conformance must force timeout, row, byte, concurrency, cancellation and source-disappearance failures and prove no snapshot is returned.
- **Authorization bypass:** an implementation could otherwise use a syntactically valid raw key directly. Canonical adapter execution now requires `AuthorizedObservationRequest`; unknown keys fail before that value exists.
- **Credential-shaped caller input:** request keys are bounded multiword `snake_case`; actual credential lookup remains adapter-local.
- **Authorization deadline gap:** moving authorization ahead of adapter execution can accidentally exclude it from the total deadline. Runtime integration must prove one end-to-end operation budget across authorization, connection and catalog work before ADR acceptance.
- **Unbounded authorization metadata:** explicit schema count/byte ceilings apply before registry/database access.
- **Async scheduling leakage:** the canonical source operation is awaitable and runtime-neutral; adapters must not hide nested executors, and callers must not reclassify it as blocking work merely to satisfy the port.
- **Authorization drift:** exact schema allowlists remain exact, non-empty and unnormalized.
- **Partial evidence:** incomplete capture fails; immutable snapshot identity is issued only after complete construction.
- **Digest framing drift:** new observed identity fields require a new framing version/domain and regression fixtures.
- **Unicode/identifier normalization drift:** v1 hashes exact UTF-8 source bytes.
- **Cross-language replay:** if published replay becomes a requirement, adopt a standard deterministic representation rather than implicit Rust/serde layout.

## Effects

The Context Map is now caller/application → request admission + registry authorization (`conceptweave-source-port`) → authorized awaitable execution envelope → concrete source adapter → immutable `conceptweave-observation` facts. Semantic Discovery consumes completed observations/receipts only and never sees a live connection handle. Governance & Publication gains no source-execution authority.

The request-memory budget and source authorization are separate invariants. A key can be syntactically valid yet unauthorized; an allowlist can be authorized in principle yet rejected because its retained metadata exceeds policy. Neither condition is silently converted into source access.

Snapshot identity is owner-computed. Provenance remains separately inspectable through source registry, extractor, timestamp and evidence-location coordinates.

## Concrete scenes

- **Data architect:** selects an approved opaque source key, exact schemas and explicit request-metadata budgets. Raw URLs, malformed keys, unknown keys, blank/duplicate schemas or over-budget allowlists fail before adapter execution.
- **Operator:** sets finite operation/statement timeouts plus row/byte/concurrency budgets. Runtime conformance must include the authorization step in the end-to-end deadline.
- **Async application runtime:** awaits the authorized source operation directly; the port does not prescribe Tokio or another executor and does not permit a hidden blocking bridge to become the canonical behavior.
- **User cancellation:** the adapter propagates cancellation and returns `Cancelled`, not a success receipt.
- **Source restart/disappearance:** incomplete capture returns `SourceUnavailable`; no immutable snapshot is published.
- **Security review:** source execution requires registry-issued opaque capability evidence while credentials remain exclusively adapter-owned.
- **Evidence replay:** same observed source content yields the same v1 digest independent of table input order or provenance-only source/extractor/time values; a material metadata change changes the digest.

## References

Bormann, C., & Hoffman, P. (2020). *Concise Binary Object Representation (CBOR)* (RFC 8949). Internet Engineering Task Force. https://doi.org/10.17487/RFC8949

National Institute of Standards and Technology. (2015). *Secure Hash Standard (SHS)* (FIPS PUB 180-4). U.S. Department of Commerce. https://doi.org/10.6028/NIST.FIPS.180-4

## Follow-up

1. Obtain exact-head Product/coverage/rustdoc evidence for digest, request-metadata admission, registry-authorized execution and the awaitable source-port contract; keep the findings acceptance-gated until current-head verification exists.
2. Implement the concrete read-only PostgreSQL adapter in Rust with explicit dependency/release decision and least-privilege credential resolution from `AuthorizedObservationRequest`.
3. Prove one end-to-end operation deadline across authorization, connection and catalog work; do not leave registry resolution as an unbounded pre-step or reset the deadline at adapter await.
4. Add conformance tests for unknown registry keys before adapter invocation, request admission, timeout, cancellation, row/byte/concurrency exhaustion, source disappearance, quoted identifiers, cross-schema collisions, composite keys, nullable FKs, CHECK/FK state, domains, enums, indexes and comments.
5. Bind successful adapter output to immutable extractor receipts and owner-computed snapshot identity.
6. Freeze an anonymized GRC-shaped reference fixture without copying foreign product source/DB internals.
7. Revisit this ADR for Accepted status only after adapter implementation and exact-head conformance evidence; until then it remains Proposed.
