# Operability Baseline

ConceptWeave has no production network service or durable database in the foundation slice. This document defines requirements before either is introduced.

## Runtime requirements

Snapshot framing is v2 after the unique null-comparison extension. Preserve v1 captures and receipts unchanged for historical replay; do not compare a freshly computed v2 digest to a v1 receipt as if they shared an encoding. Wire-version negotiation, migration and concrete PostgreSQL catalog extraction remain explicit adapter/release prerequisites, not implemented operational capabilities.

- explicit startup/readiness/liveness semantics;
- bounded source job queues, deadlines, cancellation, retry classification, and idempotency;
- Source Observation rejects schema-selection metadata budgets above the canonical provider-independent 4,096-schema/1,048,576-byte structural caps before trusted source policy; source-specific policy may only narrow that envelope;
- Source Observation request construction is not runtime/source admission: trusted local source policy must explicitly admit exact schema scope and the complete metadata/runtime `ObservationResourceEnvelope` before adapter execution;
- each successful authorization issues one non-`Clone` operation capability consumed by one `SourceObservationPort::observe`; retry after cancellation/failure obtains a fresh authorization so one policy decision cannot be replayed to multiply source/resource work;
- source/binding/schema/resource authorization and adapter work share one non-resetting monotonic operation budget; live adapters receive only the remaining duration and must cap connect/transaction/statement/cancellation work accordingly;
- wider-than-policy timeout/row/byte/concurrency/schema-metadata requests fail before adapter/source/snapshot side effects, while equal/narrower requests require an explicit policy grant;
- source registry policy remains bounded local work; remote credential/network resolution belongs in the adapter ACL and must use the exact authorized key-and-binding pair;
- persistent job receipts before accepting asynchronous work;
- OpenTelemetry sender/receiver ownership documented using the CWL shared observability contract;
- detailed structured error messages with safe identifiers, failure boundary, cause code, retryability, impact, and next action;
- no secrets or unnecessary raw PII in telemetry;
- backup/restore and migration rehearsal before durable persistence is production-ready;
- graceful drain of source parsing, model calls, validation, and publication jobs;
- deterministic replay from immutable source snapshot + extractor/config/policy-binding revisions; this is evidence replay, not reuse of an already-consumed live source authorization.

## Degraded modes

- a request asks for authorization metadata above the canonical structural cap: reject request-budget construction before registry/database work and expose the typed maximum without attempting source policy or source I/O;
- source policy denies or the observation budget is exhausted: fail closed with typed authorization/resource outcome; do not start source I/O and do not create a partial snapshot;
- source binding becomes stale after authorization: fail before credential/source access and require a fresh authorization rather than silently retargeting the key;
- one live source execution is cancelled or fails: treat its authorization capability as consumed; retry only after a new registry policy decision and never replay the previous envelope;
- LLM unavailable: deterministic observation/validation remains available; discovery may return a typed `model_assistance_unavailable` result rather than fabricate candidates;
- external research unavailable: internal source modeling remains available and reports the missing evidence channel;
- downstream catalog unavailable: publication retains a durable release/outbox receipt and does not lose the governed release.

Concrete SLO/RPO/RTO values require measured runtime evidence and are not guessed in the foundation.
