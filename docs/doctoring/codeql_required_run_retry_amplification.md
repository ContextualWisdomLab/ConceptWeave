# CodeQL required-run retry amplification

Status: `OWNER_REPAIR_PENDING`

Date: 2026-09-09

## Problem

ConceptWeave Product bootstrap #35 is blocked by the central `.github` CodeQL required-workflow owner. The existing bootstrap-compatibility defect is already scoped to `.github#2051@558693e0333e48012beea142f739bc634b0674a7`: the successor verdict reader requires `base_ref@base_sha`, while `repository_dispatch` still executes the protected predecessor handler on `.github/main@7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db` and emits the predecessor title schema.

A separate availability/cost defect is now proven by the same live required run. `CodeQL PR` run `34332431435` remained bound to PR #2051, head `558693e0333e48012beea142f739bc634b0674a7`, and base `main@7fd571dbcdbae6acf29d8f4ee704d7ba6297e4db`, but `github-actions[bot]` automatically advanced it through attempt 33. On attempt 33, `Detect CodeQL languages` succeeded; both `CodeQL compatibility analysis (actions)` and `(python)` failed at `Read current-head CodeQL dispatch verdict`; `Dispatch current-head CodeQL scan` then succeeded again. This is not a new CodeQL/SARIF finding and no manual/no-op rerun was involved.

The existing #2056 atomic-wake repair validates the complete failed-job set before one `rerun-failed-jobs` call inside a handler execution. It does not by itself prove that the required run can be woken only once across run attempts. Under the bootstrap schema mismatch, the current behavior can repeat `attempt N -> dispatch -> wake -> attempt N+1`, consuming runner/job capacity without producing new security evidence.

## Owner boundary

The causal source repair belongs to `ContextualWisdomLab/.github`; ConceptWeave must not copy or fork the central workflow. Exact owner review `ContextualWisdomLab/.github#2056` review `5155230095` records the P1 and its acceptance contract. ConceptWeave leaves #35 and dependent heads stable rather than generating leaf no-op evidence.

## Required RED and repair contract

The central owner should add a deterministic workflow-contract RED for the same required run at `run_attempt=2` with no recognized exact terminal dispatch verdict. The fixture must prove that no new repository dispatch and no new `rerun-failed-jobs` call are issued. Attempt 1 must retain the existing complete-failed-job-set verification and exactly one bounded wake.

The least-widening runtime contract is:

- attempt 1 may dispatch unresolved languages and may cause one exact required-run wake after the complete dispatch matrix terminates;
- attempt >1 must consume already-produced terminal dispatch evidence or terminate fail-closed;
- attempt >1 must not create a new identical dispatch or wake the same required run again;
- failed SARIF, cancellation, stale head/base, base-ref mismatch, required-run mismatch, language mismatch, ambiguous dispatch identity, or incomplete failed-job evidence remain non-passing;
- the bounded predecessor-title compatibility path remains transitional and is removed after the corrected handler is protected-main current.

This limits a required run to the original attempt plus at most one owner-authorized wake while preserving security failure semantics.

## ConceptWeave acceptance impact

This evidence does not make #35 GREEN and does not transfer central checks to a ConceptWeave head. Required order remains: central bootstrap compatibility + retry-amplification repair -> unchanged exact central required checks and qualifying independent review -> normal protected `.github/main` integration -> fresh #35 exact-head CodeQL and independent review -> normal #35 integration -> Foundation/dependent hosted acceptance.

No ConceptWeave source, Zotero data, ontology authority, semantic publication state, provider route, coverage threshold, protected branch, or release is changed by this note.
