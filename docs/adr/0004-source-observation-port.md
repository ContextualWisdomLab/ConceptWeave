# ADR 0004 — Bounded Source Observation port

- **Status:** Proposed
- **Date:** 2026-09-02
- **Owners:** Source Observation bounded context
- **Related:** Issue #2, PR #6, ADR 0001, `docs/product-technical-gap-baseline.md`

## Problem

ConceptWeave must observe relational metadata without turning connectivity into hidden coupling. The canonical boundary has to prevent unauthorized source access, caller-self-authorized schema scope, caller-self-authorized resource ceilings, mutable source-key retargeting after authorization, out-of-scope schema evidence, unbounded request metadata, caller-controlled snapshot identity, partial-success evidence, hidden blocking bridges, and a timeout policy that restarts after authorization.

The concrete PostgreSQL adapter is asynchronous. The port therefore needs an awaitable execution seam, but request admission and source authorization must remain provider-independent. The operation timeout is end-to-end: source-key lookup, immutable connection-policy binding, exact-schema-scope authorization, trusted resource-envelope admission, connection, transaction, catalog queries, cancellation cleanup, and immutable snapshot construction may not each start a fresh copy of the same duration.

## Constraints

- Source systems are read-only inputs; ConceptWeave does not own their business truth.
- Raw DSNs, URLs, credentials, tokens, provider connection objects, and arbitrary SQL callbacks do not cross the port/domain boundary.
- A source key is a bounded opaque multiword `snake_case` registry identifier; syntax and key recognition are not authority.
- `SourceConnectionRegistry` is an application-owned local authorization boundary. A known key must resolve to a nonblank opaque immutable connection-policy binding, and exact schema scope plus the complete provider-independent resource envelope must be authorized against that resolved key-and-binding pair. Policy decisions default to fail closed. Remote credential/network work belongs in the adapter ACL after authorization.
- The connection-policy binding is provider-independent provenance, not a DSN, credential, token, wall-clock timestamp, or database connection object.
- Every request carries a non-empty exact-schema allowlist, an explicit schema-count/UTF-8-byte request budget, and positive operation/statement/row/byte/concurrency requested bounds.
- Positive or caller-selected values are not authority. The trusted registry policy must explicitly admit the complete `ObservationResourceEnvelope`; wider-than-policy requests fail before adapter/source/snapshot side effects.
- Request metadata that exceeds its own structural envelope is rejected before registry/database access; structural admission does not replace trusted policy admission.
- The canonical immutable snapshot constructor retains the complete authorization envelope and rejects any locally observed table schema absent from the request's exact allowlist before digest or receipt issuance.
- Immutable snapshots and public source receipts retain the exact connection-policy binding that authorized the observation as a provenance coordinate separate from content identity.
- Exact source identifiers retain source spelling. Ordering may be canonicalized; names are never normalized or truncated for convenience or authorization broadening.
- Caller cancellation, stale policy binding, source disappearance, malformed captures, timeout, and resource exhaustion fail closed and never create a partial immutable snapshot.
- Snapshot content identity is computed by Source Observation from complete owned observed metadata; caller digest syntax is not content authority.
- Registry identity, connection-policy binding, extractor revision, observation time, and evidence location are provenance coordinates, not source-content bytes.
- The port crate does not select Tokio or another executor and does not import a PostgreSQL driver.

## Options considered

### Synchronous source port with adapter-local `block_on`

Rejected. It hides scheduling policy in the adapter, risks nested-runtime behavior, and weakens cancellation/deadline reasoning.

### Raw request accepted directly by the adapter

Rejected. A syntactically valid registry key is not proof that the caller is authorized to observe that source.

### Source-only registry authorization plus caller-selected schema allowlist

Rejected. Recognizing an opaque source key does not prove that the caller may widen its own schema scope. Snapshot-side containment only proves that returned tables are within the caller-selected list; without a policy decision over that list, a broadly credentialed source key can turn selection metadata into an application ACL grant.

### Positive caller-selected resource limits as effective policy

Rejected. `ObservationLimits` and `ObservationRequestBudget` can be structurally positive while still being operationally excessive. If those values become effective merely because the caller chose them, a caller can authorize its own timeout, row, byte, concurrency and schema-metadata ceilings. Structural boundedness is therefore separate from trusted resource admission.

### Fixed PostgreSQL-specific global ceilings in the port

Rejected. A hard-coded provider ceiling would conflate deployment policy with a provider-independent domain seam and would not account for source/purpose-specific risk. Trusted local source policy owns the allowed provider-independent envelope; the concrete adapter translates admitted values into driver/server limits.

### Mutable source key as the only adapter credential coordinate

Rejected. If key K is authorized while it maps to physical/policy source A and is later retargeted to B, resolving K again inside the adapter can silently use B under A's earlier authorization. The immutable evidence would still report the same key and could not prove which mapping was actually authorized.

### Provider-specific DSN or credential fingerprint in the port

Rejected. It leaks adapter/provider semantics and may turn secret-derived connection material into domain provenance. The canonical seam needs only an opaque policy revision whose interpretation stays inside the adapter ACL.

### Snapshot constructor accepts only `ResolvedSourceConnection`

Rejected. Source resolution alone does not carry the request's exact schema scope. That shape allowed canonical snapshots and receipts to be created for locally observed schemas outside the authorization request.

### Original timeout duration only

Rejected. An adapter entering after slow authorization cannot distinguish a nearly exhausted operation from a fresh one and will over-allocate connection/statement work.

### Wall-clock deadline in the public contract

Rejected. Wall-clock provenance is unnecessary for resource enforcement, adds serialization/clock-domain ambiguity, and leaks execution mechanics into the domain seam.

### Provider-independent authorized envelope with immutable policy binding, trusted resource admission and private monotonic start coordinate

Selected. Authorization begins one monotonic operation budget before local registry policy work. The registry resolves the exact source key to an opaque immutable connection-policy binding, authorizes the exact requested schema scope against that same `ResolvedSourceConnection`, then explicitly admits the complete `ObservationResourceEnvelope` against the same binding. Schema/resource policy defaults to deny. The authorized envelope privately retains the operation start coordinate and exposes only the remaining `Duration` to adapter code.

## Decision

`ObservationRequest` validates a bounded opaque source key, exact schema allowlist, `ObservationRequestBudget`, and `ObservationLimits`. These positive values establish a structurally bounded request but do not confer policy authority. `ObservationResourceEnvelope` combines the caller-requested metadata and runtime ceilings into one provider-independent value object.

`ObservationRequest::authorize` starts the operation's monotonic budget before local registry policy. It first checks the exact key through `SourceConnectionRegistry::contains_source_connection`, then requires `connection_policy_binding` to issue a nonblank opaque immutable revision for that mapping. A known key with no binding returns `MissingConnectionPolicyBinding`; a malformed binding returns `InvalidConnectionPolicyBinding`.

The same registry receives the resolved key-and-binding capability plus exact sorted `allowed_schema_names` through `authorizes_schema_scope`. The default schema-scope implementation is fail-closed. A source that exists and is bound but whose requested scope is not explicitly authorized returns `ObservationRequestError::UnauthorizedSchemaScope`; the denial does not echo the schema. No case or Unicode normalization may broaden the grant. Implementations granting a scope must compare the supplied binding with the same policy revision that owns that grant.

Only after the exact schema scope is admitted does the same local policy evaluate `authorizes_resource_envelope(resolved_source, request.resource_envelope())`. The default resource policy is fail-closed. A registry that recognizes a source and schema but does not explicitly admit the requested metadata/runtime ceilings returns `ObservationRequestError::UnauthorizedResourceEnvelope`. Policy may accept an equal or narrower request and must reject a wider-than-policy request. The port does not hard-code PostgreSQL deployment ceilings or accept provider-specific settings in this value object.

All local registry decisions are part of the same operation budget. `authorize` checks the same monotonic deadline immediately after source lookup, immutable binding lookup, schema policy, and resource policy. If one stage exhausts `operation_timeout_ms`, `OperationTimeout` takes precedence over that stage's returned policy result and no later registry stage is started. This prevents post-deadline policy side effects while preserving zero adapter/source/snapshot side effects for over-budget authorization. Because `SourceConnectionRegistry` is synchronous, a single in-flight registry call cannot be preempted by this contract and must itself remain bounded local work; the caller-requested timeout is not permission to hide remote I/O inside registry policy.

A successful authorization returns `AuthorizedObservationRequest`, which binds the validated request to `ResolvedSourceConnection { source_connection_key, connection_policy_binding }`, preserves the explicitly authorized schema scope and explicitly admitted resource envelope in the request, and privately carries the monotonic start coordinate. `remaining_operation_budget() -> Option<Duration>` is the only timing capability exposed to a concrete adapter. `None` means the end-to-end operation budget has expired. The start coordinate itself is not a public field, serialized timestamp, provider object, or credential.

`SourceObservationPort::observe` accepts only `AuthorizedObservationRequest` and returns a provider-independent `Send` future. Request construction remains deterministic. Authorization is synchronous and local but deadline-aware; it is not described as time-independent. A registry implementation that performs remote I/O would violate this boundary: remote credential/network work belongs inside the concrete adapter and must be capped by the remaining budget.

A concrete adapter ACL may resolve credentials only for the exact key-and-binding pair carried by the authorization. If the live mapping has advanced from revision A to B, an A capability must be rejected before credential/source access and before snapshot construction. The port-level synthetic adapter fixture models this fail-closed contract; only a later concrete adapter test can prove real credential/source behavior.

The public `PostgresSchemaSnapshot::new` accepts the complete `AuthorizedObservationRequest`, rather than the narrower `ResolvedSourceConnection`. Before owner-computed digest construction it compares every locally observed table's exact `schema_name` with `request().allowed_schema_names()` and fails closed when a table lies outside that scope. This is defense in depth after registry schema/resource authorization. Matching is exact and case-sensitive; no Unicode/case normalization broadens authorization. A foreign key may retain a referenced schema outside the local read allowlist because that name is relationship metadata observed from an authorized local table, not evidence that ConceptWeave read the referenced table.

The public immutable snapshot also retains the authorized opaque connection-policy binding. The source-content SHA-256 digest remains based only on complete exact observed metadata; source key and policy binding are separate provenance coordinates. `SourceObservationReceipt` carries the exact binding alongside source id, source-content digest, extractor revision, observation time, and verified location so later evidence cannot collapse two different registry mappings that reused the same source key.

The concrete adapter must read the remaining budget before potentially blocking connection/transaction/statement/cancellation work and cap each stage according to both the policy-admitted `ObservationLimits` and that remainder. It must not restart `operation_timeout_ms` at `observe`. A caller-side outer timeout may still bound waiting, but it is not a substitute for passing the remaining budget into driver/server limits.

This ADR remains **Proposed**. The port can now represent source-key plus immutable-policy-binding authorization, exact schema-scope authorization, trusted complete resource-envelope admission, stale-binding rejection at the port seam, non-resetting budget with post-stage cutoff, canonical snapshot scope binding, and binding-preserving public provenance. No production PostgreSQL adapter or exact-head runtime conformance has yet proved the full decision.

## Test and evidence contract

The Source Observation lineage includes:

- `5ee0e1edf8a2da527aefd4fe7ad2003d79b87ac6` → `301452ae2744080406f4075fe197c16d7c35cd2d`: owner-computed snapshot identity;
- `b7e54ae2b4fe9bea20d42b2d95e8c25c118a1f5f` → `94927ec3c7763c4b53cbcefd01b510030122d1db`, plus `8ed91afcf520efdd53c9103b332d3e277db29a03`: bounded request metadata and checked byte accumulation;
- `a372d6729364347315db1ad9a75efc49c779fbb9` → `5caf10b144b8254946e5d80840b0f200c0d36651`: registry-authorized adapter admission;
- `b2b83c0fdc78af11e3e0df8cf6993216dd9c6004` → `638be096f444fd22755160972285dbb9f0eb0364`: runtime-neutral awaitable source-port seam;
- `1f8f6a5875072f15325c063aa857c6da8e0accc1` → `2a77a9012ef2b8323fe61ed3ba9986ee8ecae6b0` → `82222c194e974df8f24527ab3e9b0eb579823d2d` → `235a892e8a6bd77ac5f33136980eb1fd14f30eaa` → `1204b35376d739c123668c9eb92868eef1992bb7`: remaining-operation-budget preservation and timeout precedence;
- `1f4fd1a8b969584584d77eb7c440a9b7958aeeac` → `aa087e3154f01a9c914c9533e1ffe703a79e428b`, with fixture propagation through `3b7e4553564627de527d2460e3e23d3beab58230`: canonical snapshot exact-schema containment;
- `fd00dab3335156ebc849697013de693aab7592d9` → `320ab7c8a80faa23515a158598296c898f1f5822`: source-only registry schema-scope regression and fail-closed exact-scope policy;
- review `5123306381`: mutable key-to-source mapping was identified as a pre-adapter TOCTOU gap;
- `d0c848a0f88cbb3ba18bcde26db639906259f8c3`: committed executable stale-binding specification; it was not an executed RED in the tool environment;
- `ca4446ff6fdae1f78491bbf5b9c149b9f936aa46` and ordinary forward successors: provider-independent key+policy binding capability, same-binding schema authorization, stale-binding port control, fixture propagation, and binding-preserving immutable snapshot/receipt provenance;
- review `5123894287`: positive caller-selected metadata/runtime limits were identified as an authorization gap;
- `5ba8cd6244a54359e98cb57c013cf5312153211a`: committed executable resource-envelope specification covering default deny, wider-than-policy denial and equal/narrower controls;
- `3d32a933bc2bc27fa20c22ea48111ccf3f54d7da` and ordinary forward fixture successors: `ObservationResourceEnvelope`, default-denied `authorizes_resource_envelope`, typed `UnauthorizedResourceEnvelope`, same-binding policy admission and explicit fixture policies;
- review `5124035774` and `3fb340e54d4f56c605e0b20941998d9aeb28ba79`: post-deadline registry-stage side effects identified and committed as executable specifications;
- `9d17ab4698f5d89bf4e1cf3939b81f29a18168a1` → `04a63f321508a1bc64bc1c736c76d367cccf0e3c`: stage-boundary monotonic deadline enforcement plus binding-stage edge coverage.

These are committed executable specifications and source repairs, not claimed observed RED→GREEN. The current execution environment has no Rust toolchain, and exact-head GitHub Product/Rust/coverage/rustdoc evidence is still required.

Required runtime acceptance before ADR status can become Accepted:

1. A known source without a policy binding fails closed before adapter execution; a malformed binding is rejected.
2. A registry that binds an exact source but does not explicitly authorize the requested schema scope returns `UnauthorizedSchemaScope` before adapter/source/snapshot side effects; a valid exact source+binding+schema control reaches the next policy gate.
3. A registry that authorizes source+binding+schema but does not implement trusted resource policy returns `UnauthorizedResourceEnvelope` before adapter/source/snapshot side effects.
4. A resource request above any local source-policy ceiling fails closed before adapter/source/snapshot side effects; requests equal to or narrower than every policy ceiling may be admitted explicitly.
5. Exact schema authorization is case-sensitive and normalization-free; a differently cased or Unicode-normalized identifier is not implicitly granted.
6. A capability authorized for binding A and presented after the live mapping changes to B fails before credential/source access and snapshot construction; an unchanged A control performs each expected side effect exactly once.
7. Immutable snapshot and public receipt provenance preserve binding A separately from source-content digest identity.
8. Registry work that consumes only part of the operation budget leaves the adapter only the remainder.
9. If source lookup, binding lookup, schema policy, or resource policy exhausts the operation budget, authorization returns `OperationTimeout`; no later registry stage begins, and adapter/source/snapshot side effects remain zero.
10. A request authorized only for one exact local schema cannot construct an immutable snapshot or receipt containing a different local schema; explicitly authorized multi-schema capture remains valid without case/Unicode normalization.
11. Connection, `REPEATABLE READ READ ONLY` transaction, every catalog statement, cancellation cleanup, and immutable snapshot construction are capped by the same non-resetting remaining budget and admitted resource ceilings.
12. Unknown keys, cancellation, source disappearance, malformed/partial metadata, and row/byte/concurrency exhaustion remain typed fail-closed outcomes.
13. Exact-head tests, strict Clippy/fmt/rustdoc, release build, owned coverage, security/dependency gates, and independent review are terminally valid.

## Risks and mitigations

- **Caller-selected limits become self-authorization:** structural positive bounds are wrapped in `ObservationResourceEnvelope`; trusted local source policy must explicitly admit the complete envelope and defaults to deny.
- **Mutable-key TOCTOU:** authorization captures an opaque immutable policy binding; schema/resource policy is evaluated against it; the adapter ACL must reject stale bindings before source access; public receipts retain the binding.
- **Source-only authorization accidentally broadens schema scope:** schema-scope authorization defaults to deny and must be explicitly implemented by the registry. Snapshot construction independently rejects local table schemas outside the authorized request as defense in depth.
- **Synchronous registry hangs:** the registry boundary is deliberately local and bounded; remote work is prohibited there. Runtime integration must keep that implementation property explicit and test it rather than silently using a network registry. Once one synchronous stage returns, an exhausted deadline prevents every later registry stage from starting.
- **Deadline reset in adapter:** adapter conformance must use `remaining_operation_budget()` at each blocking stage; the original configured duration is an admitted ceiling, not a fresh per-stage allowance.
- **Timing-coordinate leakage:** only remaining `Duration` is part of the adapter-facing API; no wall-clock timestamp or credential is carried.
- **Partial evidence:** immutable snapshot identity is created only after complete construction; failures never return a nominal success snapshot.
- **Authorization bypass:** both the canonical adapter seam and canonical immutable snapshot constructor require `AuthorizedObservationRequest`; a raw/well-formed key or source-only capability cannot mint out-of-scope evidence.
- **Referenced-schema confusion:** foreign-key target schema names are retained as relationship evidence but do not grant local observation authority for those schemas.
- **Provider leakage:** PostgreSQL and runtime types stay in the adapter crate, not the port/domain contract.

## Effects

The Context Map is caller/application → structurally bounded request → local registry source+immutable-policy-binding+exact-schema+resource-envelope authorization within one monotonic budget → authorized awaitable execution envelope → concrete read-only source adapter → authorization-bound immutable Source Observation facts/receipts. Semantic Discovery consumes completed observations only. Governance & Publication gains no source-execution authority.

## References

Bormann, C., & Hoffman, P. (2020). *Concise Binary Object Representation (CBOR)* (RFC 8949). Internet Engineering Task Force. https://doi.org/10.17487/RFC8949

National Institute of Standards and Technology. (2015). *Secure Hash Standard (SHS)* (FIPS PUB 180-4). U.S. Department of Commerce. https://doi.org/10.6028/NIST.FIPS.180-4

## Follow-up

1. Obtain exact-head Rust/Product/coverage/rustdoc/security/dependency evidence for the current port, binding, schema/resource admission and snapshot-provenance contract.
2. Implement the concrete read-only PostgreSQL adapter in Rust with a maintained patched driver, least-privilege exact-binding credential resolution, exact `pg_catalog` evidence, explicit `REPEATABLE READ READ ONLY`, cancellation, admitted resource ceilings, and the non-resetting remaining budget.
3. Freeze and replay an anonymized GRC-shaped conformance fixture without copying GRC source or querying application tables through hidden coupling.
4. Revisit this ADR for Accepted status only after concrete adapter/runtime conformance and independent exact-head review.