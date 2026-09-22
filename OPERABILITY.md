# Operability Baseline

ConceptWeave has no production network service or durable database in the foundation slice. This document defines requirements before either is introduced.

## Runtime requirements

- explicit startup/readiness/liveness semantics;
- bounded source job queues, deadlines, cancellation, retry classification, and idempotency;
- persistent job receipts before accepting asynchronous work;
- OpenTelemetry sender/receiver ownership documented using the CWL shared observability contract;
- detailed structured error messages with safe identifiers, failure boundary, cause code, retryability, impact, and next action;
- no secrets or unnecessary raw PII in telemetry;
- backup/restore and migration rehearsal before durable persistence is production-ready;
- graceful drain of source parsing, model calls, validation, and publication jobs;
- deterministic replay from immutable source snapshot + extractor/config revisions.

## Degraded modes

- LLM unavailable: deterministic observation/validation remains available; discovery may return a typed `model_assistance_unavailable` result rather than fabricate candidates.
- external research unavailable: internal source modeling remains available and reports the missing evidence channel.
- downstream catalog unavailable: publication retains a durable release/outbox receipt and does not lose the governed release.

Concrete SLO/RPO/RTO values require measured runtime evidence and are not guessed in the foundation.
