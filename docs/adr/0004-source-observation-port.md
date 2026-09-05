# ADR 0004 — Bounded Source Observation port

- **Status:** Proposed
- **Date:** 2026-09-02
- **Owners:** Source Observation bounded context
- **Related:** Issue #2, PR #6, ADR 0001, `docs/product-technical-gap-baseline.md`

## Problem

ConceptWeave needs to observe PostgreSQL metadata without turning source connectivity into hidden coupling or allowing an adapter to run indefinitely, inspect unauthorized schemas, invent a partial snapshot after source disappearance, or leak credentials into domain contracts. The existing `conceptweave-observation` crate already owns immutable observed facts and provenance receipts, but it intentionally does not own source execution policy.

A snapshot digest is an integrity identity, not an adapter assertion. Accepting an arbitrary syntactically valid digest from the caller allows distinct observed metadata to reuse one immutable identity and lets receipts repeat that unverified assertion. Source Observation therefore also needs one owner-defined deterministic content framing before a snapshot can issue provenance.

## Constraints

- Source systems are read-only inputs; ConceptWeave does not own their business truth.
- Only an opaque source registry key may cross the port: at most 128 bytes, lowercase multiword `snake_case`. An authorized registry lookup must issue the capability accepted by immutable snapshots; syntax alone is not provenance authority. Passwords, tokens, DSNs, URLs, shell-style connection parameters, and provider-specific connection objects may not cross this boundary.
- Every request needs an explicit non-empty exact-schema allowlist and positive operation/statement-timeout, row, byte, and concurrency bounds.
- Caller cancellation and source disappearance must fail closed rather than return a fabricated or partial success.
- Exact source identifiers keep original case/text; canonicalization may order an allowlist but must not normalize identifier meaning.
- Snapshot content identity must be derived from the complete observed metadata owned by this bounded context. Caller-supplied digest syntax is not proof of content identity.
- Source registry identity, extractor revision, and observation time are explicit provenance coordinates. They are not source-content bytes and must not change the content digest for an otherwise identical observation.
- Digest framing must be versioned and domain-separated so later metadata-model changes cannot silently reinterpret an existing digest.
- The port must remain provider-independent and free of PostgreSQL driver, credential, semantic-inference, publication, or LLM responsibilities.
- The concrete PostgreSQL adapter must remain outside `conceptweave-domain`, `conceptweave-observation`, and the port contract.

## Options considered

### Put limits and source execution into `conceptweave-observation`

Rejected. That crate owns immutable observation facts. Mixing driver execution policy into the fact model would collapse the Source Observation aggregate boundary and make deterministic replay depend on live-source concerns.

### Let each PostgreSQL adapter define its own timeout/allowlist/error vocabulary

Rejected. This would make resource safety and cancellation non-portable, weaken conformance tests, and allow downstream adapters to silently diverge on what counts as bounded observation.

### Pass a raw connection string plus arbitrary SQL callback through a generic utility layer

Rejected. Raw credentials would cross the boundary, arbitrary SQL would make read-only enforcement unauditable, and a generic utility bucket would erase the Source Observation ubiquitous language.

### Trust an adapter-supplied SHA-256 string as snapshot identity

Rejected. Canonical `sha256:<64 lowercase hex>` syntax proves only representation shape. It does not prove that the digest was computed from the observed tables, columns, constraints, or their exact source metadata.

### Canonicalize the internal observation model through a general JSON or CBOR wire format

Deferred. RFC 8949 deterministic CBOR is a sound standard when a protocol needs deterministic encoded bytes, and a future cross-language Source Observation artifact may adopt it. The current digest is an internal aggregate identity, however, and making a general serialization format canonical now would introduce a wire-format commitment that the current Rust-only fact model does not otherwise require. JSON canonicalization has the same premature wire-contract problem for this boundary.

### Define a small provider-independent Source Observation port and an owner-computed content digest

Selected. `conceptweave-source-port` owns request budgets, exact schema authorization, bounded opaque source registry keys, caller cancellation, and typed fail-closed outcomes. `conceptweave-observation` owns deterministic observed facts and derives their content identity itself.

## Decision

Introduce the Rust workspace crate `conceptweave-source-port` as a Supporting-domain port contract. `ObservationLimits` requires positive operation/statement-timeout, row, byte, and concurrency limits. `ObservationRequest` requires an opaque source registry key of at most 128 bytes using lowercase multiword `snake_case`, plus a non-empty exact schema allowlist. It rejects raw DSNs/URLs/key-value connection material, one-word/generic keys, malformed registry identifiers, and blank or duplicate schema identifiers, and sorts the allowlist only for deterministic request identity. `SourceConnectionRegistry` resolves the exact key and issues `ResolvedSourceConnection`; `PostgresSchemaSnapshot` accepts only that opaque capability. `ObservationCancellation` carries caller cancellation. `SourceObservationPort` defines the adapter seam. `SourceObservationFailure` distinguishes cancellation, source disappearance, timeout, invalid captured metadata, and row/byte/concurrency-limit exhaustion.

`PostgresSchemaSnapshot` computes its own `sha256:` identity after exact table ordering is canonicalized. The digest input uses a versioned domain separator (`conceptweave.postgres_schema_snapshot.v1`) and an explicit length-prefixed binary framing. Strings are hashed as their exact UTF-8 bytes without Unicode, case, or PostgreSQL-quoting normalization. Collection lengths and string lengths are unsigned 64-bit big-endian values; column ordinals are unsigned 32-bit big-endian values; booleans, options, constraint variants, referential actions, match types, and deferrability states use explicit stable tags. Table order, column order, and constraint order are deterministic; ordered composite-key and foreign-key coordinates remain order-significant because PostgreSQL reports those positions as source evidence.

The v1 content envelope includes exact table identifiers, column names/ordinals/types/nullability/comments, and every owned PK/unique/FK/CHECK field, including optional FK reference behavior, targeted delete columns, validation/enforcement state, CHECK definition, and `NO INHERIT`. It excludes `source_connection_key`, `extractor_revision`, and `observed_at_utc`; those remain separate receipt provenance. Changing any observed source-content field changes the digest, while changing only input collection order or those provenance coordinates does not. Receipts expose only the snapshot's owner-computed digest.

SHA-256 is the current digest primitive under NIST FIPS 180-4. The framing is intentionally ConceptWeave-owned rather than an implicit Rust memory/serde representation, so compiler layout, map iteration, or serializer defaults cannot alter identity. A future framing revision must use a new domain/version and document migration rather than silently changing v1 semantics.

This decision does **not** claim that a production PostgreSQL adapter exists. The next owner-side implementation must select a maintained Rust PostgreSQL driver, resolve the registry key to credentials inside the adapter ACL, establish read-only transaction/session behavior, enforce every port limit in execution rather than configuration only, populate the immutable `conceptweave-observation` contracts, and prove cancellation/source-disappearance behavior against a frozen anonymized reference fixture before live-source readiness is claimed.

## Evidence

- Test-first commit `7cafba262aca070fa6bdccc95284641436a81224` specifies positive resource budgets, exact allowlist behavior, cancellation, and bounded failure outcomes.
- Production commit `016b0aff5a6866d6071e02dd1afa6e116a8ce92b` implements the provider-independent contract.
- Test-first security commit `2f6cd4e6f80b60a0d8118de2162d974bbabde4cc` demonstrates that DSNs, shell-style connection parameters, one-word identifiers, mixed-case identifiers, hyphenated identifiers, and malformed underscore forms must fail before adapter access.
- Production commit `339222cba31f126a5f5f36fe00f890fc82c4aa79` turns `source_connection_key` into the bounded opaque registry-key contract instead of attempting heuristic secret scanning.
- Edge-coverage commit `729820490f7d072d28444432a082d9fae263f194` covers the 128-byte registry-key bound.
- Test-first commits `2194a4ed1b8262d76dca0e7708cfd30114372a2b`, `d073aed`, `a39fa08`, and `38ecdf0` pin targeted foreign-key delete columns and registry-resolved snapshot identity; production commits `eb96251`, `cbfa38a`, and `17c5067` implement those boundaries.
- Test-first digest-integrity commit `5ee0e1edf8a2da527aefd4fe7ad2003d79b87ac6` proves that reusing one caller assertion across changed observed metadata is not an acceptable immutable identity and locks provenance/order invariants for the owner-computed replacement.
- NIST FIPS 180-4 defines the Secure Hash Standard used for SHA-256. RFC 8949 deterministic encoding requirements are retained as the benchmark for any future CBOR-based cross-language observation artifact; v1 does not claim CBOR compatibility.
- `docs/product-technical-gap-baseline.md` records the port as implemented-pending-checks and keeps the concrete PostgreSQL adapter OPEN.
- Exact-head hosted Product evidence remains required; predecessor, local-only, or queued runs are not completion evidence.

## Risks and mitigations

- **Configuration without enforcement:** a concrete adapter could accept limits but ignore them. Mitigation: adapter conformance tests must force timeout, row, byte, concurrency, cancellation, and disappearance failures and verify no snapshot is returned.
- **Credential-shaped caller input:** a caller could otherwise place a DSN or connection parameter string in `source_connection_key` even though the field was documented as non-credential. Mitigation: the port accepts only bounded multiword `snake_case` registry keys; credential lookup remains exclusively inside the adapter ACL.
- **Blocking execution:** a blocking driver could stall an asynchronous product executor. Mitigation: adapter design must isolate blocking work or use an async Rust driver; no blocking database call may run on an async web executor thread.
- **Authorization drift:** a broad or normalized schema selector could observe unintended metadata. Mitigation: exact non-empty allowlists are part of the port and must be applied before catalog results become observations.
- **Partial evidence:** a source can disappear mid-capture. Mitigation: incomplete captures fail with `SourceUnavailable`; immutable snapshot identity is issued only after a complete bounded capture.
- **Digest framing drift:** adding a new observed field without defining its identity semantics could make two implementations disagree. Mitigation: v1 is domain-separated and explicit; future framing changes require a new version/domain plus regression fixtures rather than an in-place reinterpretation.
- **Unicode or identifier normalization drift:** visually similar identifiers can have different source bytes. Mitigation: v1 hashes exact UTF-8 source text and performs no normalization.
- **Cross-language replay:** an ad hoc serializer would be difficult to reproduce safely. Mitigation: v1 specifies primitive tags, byte order, and length framing explicitly; if a published cross-language artifact is required, standard deterministic CBOR is reconsidered at that contract boundary.

## Effects

The Source Observation Context Map now has three explicit layers: caller/application -> `conceptweave-source-port` -> concrete source adapter -> `conceptweave-observation` immutable facts. Semantic Discovery consumes completed observation facts and receipts only; it never receives a live connection handle. Governance & Publication remains downstream and does not gain source execution authority. The caller can reference an approved source connection only through a registry key; adapter-local credential resolution remains an Anti-Corruption Layer concern.

The immutable observation aggregate no longer treats a caller-provided digest as evidence. Content identity is computed inside its canonical owner after deterministic ordering; provenance remains separately inspectable through source registry, extractor, timestamp, and evidence-location coordinates.

## Concrete scenes

- **Data architect:** selects an approved source registry key and exact schemas. A raw PostgreSQL URL, generic one-word key, blank schema name, or duplicate schema name is rejected before source access.
- **Operator:** sets a finite statement timeout plus row/byte/concurrency budgets. A source that exceeds any budget fails explicitly instead of producing a misleading partial model.
- **User cancellation:** cancellation is propagated across the port; the adapter must stop/abort as supported and return `Cancelled`, not a success receipt.
- **Source restart/disappearance:** a connection loss during metadata capture returns `SourceUnavailable`; no immutable snapshot is published from the incomplete capture.
- **Security review:** credentials remain adapter-owned and absent from request/domain objects; the port admits only a bounded opaque registry key while schema authorization and resource limits remain visible, typed, and testable.
- **Evidence replay:** two snapshots with the same observed source metadata produce the same v1 source-content digest regardless of input table order, source registry key, extractor revision, or observation timestamp; changing one observed metadata field changes that digest.

## References

Bormann, C., & Hoffman, P. (2020). *Concise Binary Object Representation (CBOR)* (RFC 8949). Internet Engineering Task Force. https://doi.org/10.17487/RFC8949

National Institute of Standards and Technology. (2015). *Secure Hash Standard (SHS)* (FIPS PUB 180-4). U.S. Department of Commerce. https://doi.org/10.6028/NIST.FIPS.180-4

## Follow-up

1. Obtain exact-head Product/coverage/rustdoc evidence for the owner-computed snapshot digest and keep the digest-integrity review finding unresolved until that current head is verified.
2. Implement the concrete read-only PostgreSQL adapter behind this port with Rust and an explicit dependency/release decision.
3. Add conformance tests for registry-key credential resolution, timeout, cancellation, row/byte/concurrency exhaustion, source disappearance, quoted identifiers, cross-schema collisions, composite keys, nullable FKs, CHECK/FK validation-enforcement state, domains, enums, indexes, and comments.
4. Bind successful adapter output to immutable extractor receipts and the owner-computed deterministic snapshot identity.
5. Freeze an anonymized GRC-shaped reference fixture without copying foreign product source/DB internals.
6. Revisit this ADR for Accepted status only after the adapter and exact-head conformance evidence are integrated; until then it remains Proposed.
