# Strix request correlation

Observed September 8, 2026. ConceptWeave PR #35 run [33938445050](https://github.com/ContextualWisdomLab/ConceptWeave/actions/runs/33938445050), attempt 1, remains completed/failure at `a31ae0c2df920f2794f7ddb456795b04797ab472`.

The job-log endpoint for job `101256562088` succeeds even though `gh run view --log-failed` fails while resolving workflow `347237758` (404). This is an observation-path failure, not missing run/log evidence.

At `2026-09-05T12:43:57.2697906Z`, the Chat Completions streaming call ends with `openai.InternalServerError`, HTTP 500, `internal_error`, request ID `175d6d59c5294b0e8a21548193b90482`. The subsequent wrapper reports `STRIX_PROVIDER_UNAVAILABLE` and free-pool exhaustion. That wrapper classification does not establish why the server returned 500 or prove that every eligible free candidate was attempted.

The request ID and exact job/head were delivered to the existing contextual-orchestrator owner task and central workflow coordinator for server-side correlation. Required next evidence: matching server log, root-cause repair at its owner, verified serving revision, and a fresh exact-head completed security verdict. Do not replace the gateway, use paid fallback, dismiss an approval requirement, or interpret this failure as either a product vulnerability or a clean security result.
