# Source Observation authorization replay and resource governance

**Evidence date:** 2026-09-06  
**Scope:** ConceptWeave Source Observation port; PR #6  
**Decision linkage:** ADR 0004, ADR 0006

## Question

Does a registry-authorized Source Observation envelope represent one bounded operation, or may the same authorization be replayed across multiple sequential or concurrent adapter executions?

The pre-repair Rust seam allowed replay because `AuthorizedObservationRequest` implemented `Clone` and `SourceObservationPort::observe` borrowed `&AuthorizedObservationRequest`. That shape was inconsistent with `ObservationResourceEnvelope`, whose timeout, row, byte, and concurrency ceilings are defined for one Source Observation operation. Reusing one grant could multiply source access and resource consumption without another trusted registry decision.

## External evidence

OWASP API Security Top 10 API4:2023 treats unrestricted resource consumption and unrestricted operation frequency as denial-of-service and economic-abuse risks. Its mitigation guidance includes limiting how often a client can execute an operation and bounding resource consumption. The guidance is API-facing and does not prescribe a Rust ownership model, but it supports the architectural invariant that a resource grant must not silently become unlimited through replay.

MITRE CWE-770 describes allocation of resources without limits or throttling and recommends explicit minimum/maximum resource expectations plus architectural controls over resource use. Again, CWE-770 does not require a linear capability type; ConceptWeave applies the general resource-governance principle at its in-process application/adapter boundary.

The evidence therefore supports the invariant, not a technology-specific implementation mandate. Rust ownership is the narrowest local mechanism available because the current canonical Source Observation capability is an in-process value rather than a remote bearer token.

## Decision traceability

| Evidence / finding | Contract implication | Exact repository trace |
| --- | --- | --- |
| One authorization could be borrowed by multiple `observe` calls | A policy-admitted operation could be replay-amplified | PR #6 predecessor `db209b9b11039ed77cbae246f65b3a83d7589d23`; review `5124482059` |
| Resource guidance requires explicit bounded use rather than unlimited interaction | One authorization should cross the execution seam at most once | ADR 0006; `ObservationResourceEnvelope`; `SECURITY.md` threat/control |
| Rust ownership can enforce single use without runtime state | Make the authorized capability non-`Clone` and consume it by value | RED-spec `2a03a56a5982f9d56e880689a139597aea3ef47d`; production repair `340ded102f18c1c4abebbcf0590e5941b61f6cba` |
| Retry must not inherit stale policy silently | Cancellation/failure/completion consumes the grant; retry re-authorizes | `async_observation_port.rs`, `bounded_observation_port.rs`, `OPERABILITY.md`, `docs/TRD.md` |
| Existing source/binding/deadline controls must remain intact | Linear capability must preserve stale-binding, deadline and zero-side-effect behavior | `connection_policy_binding.rs`, `remaining_operation_budget.rs`, `authorization_side_effects.rs`, `resource_envelope_authorization.rs` |

## Acceptance evidence still required

The committed compile-contract and source repair are not runtime GREEN by existence alone. One unchanged exact PR head must pass repository-owned Rust 1.98 tests, strict formatting/Clippy, warnings-denied rustdoc, release build, owned 100% coverage, and applicable security/dependency workflows. A concrete PostgreSQL adapter must then prove that each attempted live observation/retry obtains a fresh authorization, resolves credentials only for the exact key-and-binding pair, rejects stale bindings before I/O, and enforces the remaining operation/resource envelope through read-only catalog execution.

## References

MITRE. (2026). *CWE-770: Allocation of resources without limits or throttling (Version 4.20).* Common Weakness Enumeration. https://cwe.mitre.org/data/definitions/770.html

OWASP Foundation. (2023). *API4:2023 unrestricted resource consumption.* OWASP API Security Top 10. https://owasp.org/API-Security/editions/2023/en/0xa4-unrestricted-resource-consumption/
