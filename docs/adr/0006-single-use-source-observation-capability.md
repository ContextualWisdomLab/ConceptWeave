# ADR 0006 — Single-use Source Observation authorization capability

- **Status:** Proposed
- **Date:** 2026-09-06
- **Owners:** Source Observation bounded context
- **Refines:** ADR 0004
- **Related:** Issue #2, PR #6, `ARCHITECTURE.md`, `docs/TRD.md`, `SECURITY.md`

## Problem

ADR 0004 binds Source Observation execution to a registry-authorized `AuthorizedObservationRequest` carrying an exact source key, immutable connection-policy binding, schema scope, admitted resource envelope, and the remaining end-to-end operation budget. The prior execution seam still made that capability reusable: `AuthorizedObservationRequest` implemented `Clone` and `SourceObservationPort::observe` borrowed `&AuthorizedObservationRequest`.

That shape allowed one successful registry authorization to start multiple sequential or concurrent adapter executions. Each replay could independently consume the admitted row, byte, concurrency, source-access, and remaining-time budget, so a per-operation resource envelope was not actually bound to one operation. The repository's own async port fixture demonstrated the ambiguity by using one authorized request for a cancelled execution and then reusing it for a successful execution.

This is a resource-governance and authorization-semantics defect rather than a PostgreSQL-driver detail. OWASP API4:2023 treats unrestricted interaction frequency and resource consumption as denial-of-service/cost risks and recommends limiting how often a client can execute an operation. MITRE CWE-770 likewise calls for explicit minimum/maximum capability expectations and architectural resource limits. The ConceptWeave seam needs to enforce that property before a concrete source adapter exists.

## Constraints

- One `ObservationResourceEnvelope` describes one Source Observation operation, not a reusable session budget.
- Registry authorization must remain provider-independent and credential-free.
- The existing non-resetting monotonic deadline, exact schema policy, immutable binding, stale-binding rejection, and snapshot-side scope check must not weaken.
- A concrete adapter must still be able to borrow the owned request while constructing `PostgresSchemaSnapshot` inside one execution.
- Cancellation or failure does not justify replaying stale authorization. Retry must evaluate current registry policy again.
- No Tokio, PostgreSQL, web-framework, DSN, credential, or wall-clock type belongs in the port contract.

## Options considered

### Keep a cloneable/borrowed capability and document “do not replay”

Rejected. The type contract would continue to permit the exact amplification that the resource envelope is supposed to prevent. A comment cannot make a reusable capability linear.

### Keep borrowing but add a mutable consumed flag

Rejected. Interior state would add synchronization and aliasing semantics to a value that can instead be made linear by ordinary Rust ownership. It would also make concurrent replay a runtime error rather than a compile-time ownership constraint.

### Add a process-global replay cache or authorization nonce registry

Rejected for the canonical port. It introduces persistence/lifecycle state and distributed coordination before a need is demonstrated. Provider/runtime-specific anti-replay evidence can be added later if a remote bearer capability is introduced; the current in-process Rust boundary can enforce single use directly.

### Consume a non-`Clone` authorized request by value

Selected. `AuthorizedObservationRequest` no longer implements `Clone`, and `SourceObservationPort::observe` accepts it by value. One authorization can therefore cross the canonical adapter execution seam at most once under safe Rust ownership. Retry constructs or retains a raw `ObservationRequest` and invokes `authorize` again against current source policy.

## Decision

`AuthorizedObservationRequest` is a single-use operation capability.

1. `ObservationRequest` remains cloneable before authorization so callers may intentionally submit independent authorization attempts.
2. `ObservationRequest::authorize(self, registry)` consumes the raw request and issues one non-`Clone` `AuthorizedObservationRequest` after source, immutable binding, exact schema, resource-envelope, and deadline admission.
3. `SourceObservationPort::observe(self-reference, AuthorizedObservationRequest, cancellation)` consumes the authorized capability by value and returns the existing provider-independent `Send` future.
4. The adapter owns the capability for the duration of the future and may borrow it internally for `remaining_operation_budget()`, source/binding inspection, or `PostgresSchemaSnapshot::new(&request, ...)`.
5. Cancellation, `SourceObservationFailure`, or successful completion consumes the capability. Retry requires a fresh authorization decision and therefore observes any changed source-policy binding or resource policy.

The decision does not turn `ResolvedSourceConnection` into a secret or bearer token and does not claim that Rust ownership replaces rate limiting at a future network delivery boundary. It closes replay amplification inside the canonical application/adapter seam where ConceptWeave currently owns the operation capability.

## Test and evidence contract

- Predecessor exact head `db209b9b11039ed77cbae246f65b3a83d7589d23` allowed the same `AuthorizedObservationRequest` to be borrowed by multiple `observe` calls.
- Review `5124482059` records the replay-amplification finding and acceptance criteria.
- Commit `2a03a56a5982f9d56e880689a139597aea3ef47d` changes the async compile-contract fixture first: the port implementation consumes the authorization by value and cancellation/success controls obtain independent authorizations. Against the predecessor trait this is intentionally incompatible and therefore serves as the committed RED specification; it was not executed in the current tool environment.
- Commit `340ded102f18c1c4abebbcf0590e5941b61f6cba` removes `Clone` from `AuthorizedObservationRequest`, makes the public port consume it by value, and documents the single-use invariant.
- Successor fixture commits `72deb9fb043fb85033298f1c31fb6c30c20a9e79`, `6a29cbe193d3dd7b807d936344d783b554f68d2e`, `cd6d999f310f11bc18a5abe59337bcdbba40f15f`, `8ef123997de2eb208d33bac85dae72c65d22c15f`, and `30d253f8c0c35a99d8eb4b2741cc660675bfc30c` preserve zero-side-effect authorization denial, cancellation, stale-binding, remaining-budget, and resource-envelope behavior on the by-value seam.

Acceptance still requires one unchanged exact head to execute repository-owned Rust tests, strict fmt/Clippy, warnings-denied rustdoc, release build, owned coverage, and applicable security/dependency workflows. Source inspection and committed specifications are not GREEN evidence.

## Consequences

The port now matches its own “one bounded operation” vocabulary: authorization and admitted resources cannot be multiplied simply by retaining or cloning a successful capability. Retry becomes intentionally visible because it has to pass policy again. Application code that previously treated `AuthorizedObservationRequest` as a session token must instead retain raw request intent or reconstruct it and re-authorize.

The concrete PostgreSQL adapter remains subsequent work. It must still prove read-only credential resolution for the exact key-and-binding pair, stale-binding rejection before I/O, one remaining operation budget across connection/transaction/statements, cancellation cleanup, bounded rows/bytes/concurrency, and complete-or-fail immutable snapshot construction.

## References

MITRE. (2026). *CWE-770: Allocation of resources without limits or throttling (Version 4.20).* Common Weakness Enumeration. https://cwe.mitre.org/data/definitions/770.html

OWASP Foundation. (2023). *API4:2023 unrestricted resource consumption.* OWASP API Security Top 10. https://owasp.org/API-Security/editions/2023/en/0xa4-unrestricted-resource-consumption/
