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

## Local paper-text capture

The proposed `--capture-full-text` command uses an existing private metadata report and creates a separate owner-only file. Keep the original report: the new file cannot replace it, renew review, or prove that all text came from one atomic snapshot. No Zotero authorization prompt, mutation or model request is part of this command.

A changed library, missing provider identity, unexpected response, malformed text or exhausted budget rejects the run. Preserve earlier artifacts; do not disable the checks or overwrite an old file to retry. If metadata has changed, capture a new report and start a separately bound review campaign. Otherwise investigate the reported boundary and rerun to a new temp path. An expected missing-text response is retained. Admission failures create no output, while a write or flush failure may leave a private partial file at the create-new path; the writer deliberately does not unlink or retry buffered bytes against a pathname that may have been replaced.

Responses are limited to 8 MiB each and 256 MiB total. The persisted compact JSON capture has a separate 512 MiB ceiling because nested source JSON can expand when its text is escaped by the outer capture envelope. Capture verification counts the exact serialized representation before a newly acquired capture can be returned, so the CLI does not create an artifact that the bounded restore path is specified to reject solely because of JSON-encoding expansion. This preflight streams into a counting writer and does not allocate a second encoded copy. The sweep also has a five-minute admission/completion limit and finite local request timeouts. Source text stays in memory until capture completes; hashing and final writing stream. The CLI deliberately does not delete prior reports or schedule private-file cleanup. Review retention according to the research library's policy, and never attach these files to a public PR.
