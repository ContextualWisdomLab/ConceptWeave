# ADR 0004 — Bounded Source Observation port

- **Status:** Proposed
- **Date:** 2026-09-02
- **Owners:** Source Observation bounded context
- **Related:** Issue #2, PR #6, ADR 0001, `docs/product-technical-gap-baseline.md`

## Problem

ConceptWeave must observe relational metadata without turning connectivity into hidden coupling. The canonical boundary has to prevent unauthorized source access, unbounded request metadata, caller-controlled snapshot identity, partial-success evidence, hidden blocking bridges, and a timeout policy that restarts after authorization.

The concrete PostgreSQL adapter is asynchronous. The port therefore needs an awaitable execution seam, but request admission and source authorization must remain provider-independent. The operation timeout is also end-to-end: registry authorization, connection, transaction, catalog queries, cancellation cleanup, and immutable snapshot construction may not each start a fresh copy of the same duration.

## Constraints

- Source systems are read-only inputs; ConceptWeave does not own their business truth.
- Raw DSNs, URLs, credentials, tokens, provider connection objects, and arbitrary SQL callbacks do not cross the port/domain boundary.
- A source key is a bounded opaque multiword `snake_case` registry identifier; syntax is not authority.
- `SourceConnectionRegistry` is an application-owned local authorization boundary. Remote credential/network work belongs in the adapter ACL after authorization.
- Every request carries a non-empty exact-schema allowlist, an explicit schema-count/UTF-8-byte admission budget, and positive operation/statement/row/byte/concurrency bounds.
- Request metadata is rejected before registry/database access when it exceeds policy.
- Exact source identifiers retain source spelling. Ordering may be canonicalized; names are never normalized or truncated for convenience.
- Caller cancellation, source disappearance, malformed captures, timeout, and resource exhaustion fail closed and never create a partial immutable snapshot.
- Snapshot content identity is computed by Source Observation from complete owned observed metadata; caller digest syntax is not content authority.
- Registry identity, extractor revision, observation time, and evidence location are provenance coordinates, not source-content bytes.
- The port crate does not select Tokio or another executor and does not import a PostgreSQL driver.

## Options considered

### Synchronous source port with adapter-local `block_on`

Rejected. It hides scheduling policy in the adapter, risks nested-runtime behavior, and weakens cancellation/deadline reasoning.

### Raw request accepted directly by the adapter

Rejected. A syntactically valid registry key is not proof that the caller is authorized to observe that source.

### Original timeout duration only

Rejected. An adapter entering after slow authorization cannot distinguish a nearly exhausted operation from a fresh one and will over-allocate connection/statement work.

### Wall-clock deadline in the public contract

Rejected. Wall-clock provenance is unnecessary for resource enforcement, adds serialization/clock-domain ambiguity, and leaks execution mechanics into the domain seam.

### Provider-independent authorized envelope with a private monotonic start coordinate

Selected. Authorization begins one monotonic operation budget before the registry lookup. The authorized envelope privately retains that coordinate and exposes only the remaining `Duration` to adapter code.

## Decision

`ObservationRequest` validates a bounded opaque source key, exact schema allowlist, `ObservationRequestBudget`, and `ObservationLimits`. `ObservationRequest::authorize` starts the operation's monotonic budget before the local `SourceConnectionRegistry` lookup. The lookup result is captured first; if elapsed time has exhausted `operation_timeout_ms`, authorization returns `ObservationRequestError::OperationTimeout` before propagating the registry result or admitting an adapter. This gives timeout precedence to an exhausted authorization step and preserves zero adapter/source/snapshot side effects.

A successful authorization returns `AuthorizedObservationRequest`, which binds the validated request to `ResolvedSourceConnection` and privately carries the monotonic start coordinate. `remaining_operation_budget() -> Option<Duration>` is the only timing capability exposed to a concrete adapter. `None` means the end-to-end operation budget has expired. The start coordinate itself is not a public field, serialized timestamp, provider object, or credential.

`SourceObservationPort::observe` accepts only `AuthorizedObservationRequest` and returns a provider-independent `Send` future. Request construction remains deterministic. Authorization is synchronous and local but deadline-aware; it is not described as time-independent. A registry implementation that performs remote I/O would violate this boundary: remote credential/network work belongs inside the concrete adapter and must be capped by the remaining budget.

The concrete adapter must read the remaining budget before potentially blocking connection/transaction/statement/cancellation work and cap each stage accordingly. It must not restart `operation_timeout_ms` at `observe`. A caller-side outer timeout may still bound waiting, but it is not a substitute for passing the remaining budget into driver/server limits.

`PostgresSchemaSnapshot` continues to compute its own domain-separated SHA-256 identity from complete exact observed metadata. Provenance coordinates stay separate from source-content identity.

This ADR remains **Proposed**. The port can now represent and preserve the non-resetting budget, but no production PostgreSQL adapter or exact-head runtime conformance has yet proved the full decision.

## Test and evidence contract

The current Source Observation lineage includes:

- `5ee0e1edf8a2da527aefd4fe7ad2003d79b87ac6` → `301452ae2744080406f4075fe197c16d7c35cd2d`: owner-computed snapshot identity;
- `b7e54ae2b4fe9bea20d42b2d95e8c25c118a1f5f` → `94927ec3c7763c4b53cbcefd01b510030122d1db`, plus `8ed91afcf520efdd53c9103b332d3e277db29a03`: bounded request metadata and checked byte accumulation;
- `a372d6729364347315db1ad9a75efc49c779fbb9` → `5caf10b144b8254946e5d80840b0f200c0d36651`: registry-authorized adapter admission;
- `b2b83c0fdc78af11e3e0df8cf6993216dd9c6004` → `638be096f444fd22755160972285dbb9f0eb0364`: runtime-neutral awaitable source-port seam;
- `1f8f6a5875072f15325c063aa857c6da8e0accc1`: executable specification for partial and exhausted registry-budget consumption;
- `2a77a9012ef2b8323fe61ed3ba9986ee8ecae6b0`: private monotonic coordinate and remaining-budget API;
- `82222c194e974df8f24527ab3e9b0eb579823d2d`: timeout-precedence specification for a slow denied registry lookup;
- `235a892e8a6bd77ac5f33136980eb1fd14f30eaa`: timeout-precedence production repair;
- `1204b35376d739c123668c9eb92868eef1992bb7`: immediate static correction of an accidental enum-variant spelling regression in the preceding commit.

The remaining-budget tests are committed executable specifications, not claimed observed RED→GREEN. The current execution environment has no Rust toolchain, and exact-head GitHub Product/Rust/coverage/rustdoc evidence is still required.

Required runtime acceptance before ADR status can become Accepted:

1. A registry lookup that consumes part of the operation budget leaves the adapter only the remainder.
2. A registry lookup that exhausts the budget returns `OperationTimeout` before adapter/source/snapshot side effects, including the denied-key case.
3. Connection, `REPEATABLE READ READ ONLY` transaction, every catalog statement, cancellation cleanup, and immutable snapshot construction are capped by the same non-resetting remaining budget.
4. Unknown keys, cancellation, source disappearance, malformed/partial metadata, and row/byte/concurrency exhaustion remain typed fail-closed outcomes.
5. Exact-head tests, strict Clippy/fmt/rustdoc, release build, owned coverage, security/dependency gates, and independent review are terminally valid.

## Risks and mitigations

- **Synchronous registry hangs:** the registry boundary is deliberately local and bounded; remote work is prohibited there. Runtime integration must keep that implementation property explicit and test it rather than silently using a network registry.
- **Deadline reset in adapter:** adapter conformance must use `remaining_operation_budget()` at each blocking stage; the original configured duration is a ceiling, not a fresh per-stage allowance.
- **Timing-coordinate leakage:** only remaining `Duration` is part of the adapter-facing API; no wall-clock timestamp or credential is carried.
- **Partial evidence:** immutable snapshot identity is created only after complete construction; failures never return a nominal success snapshot.
- **Authorization bypass:** the canonical adapter seam accepts only `AuthorizedObservationRequest`; a raw/well-formed key cannot invoke it.
- **Provider leakage:** PostgreSQL and runtime types stay in the adapter crate, not the port/domain contract.

## Effects

The Context Map is caller/application → bounded request admission → local registry authorization + shared monotonic budget → authorized awaitable execution envelope → concrete read-only source adapter → immutable Source Observation facts/receipts. Semantic Discovery consumes completed observations only. Governance & Publication gains no source-execution authority.

## References

Bormann, C., & Hoffman, P. (2020). *Concise Binary Object Representation (CBOR)* (RFC 8949). Internet Engineering Task Force. https://doi.org/10.17487/RFC8949

National Institute of Standards and Technology. (2015). *Secure Hash Standard (SHS)* (FIPS PUB 180-4). U.S. Department of Commerce. https://doi.org/10.6028/NIST.FIPS.180-4

## Follow-up

1. Obtain exact-head Rust/Product/coverage/rustdoc/security/dependency evidence for the current port contract.
2. Implement the concrete read-only PostgreSQL adapter in Rust with a maintained patched driver, least-privilege credential resolution, exact `pg_catalog` evidence, explicit `REPEATABLE READ READ ONLY`, cancellation, and the non-resetting remaining budget.
3. Freeze and replay an anonymized GRC-shaped conformance fixture without copying GRC source or querying application tables through hidden coupling.
4. Revisit this ADR for Accepted status only after concrete adapter/runtime conformance and independent exact-head review.