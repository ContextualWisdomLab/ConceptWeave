# Product CI Draft execution gate

## Decision

ConceptWeave Product CI must execute repository quality gates on Draft pull requests. `closed` pull-request events remain suppressed; Draft state is not an execution-suppression signal.

## Problem

Source Observation PR #46 is governed as a Draft single-writer until unchanged-head Rust/PostgreSQL acceptance exists. The repository Product workflow previously required both `github.event.action != 'closed'` and `github.event.pull_request.draft == false` before `rust-quality` could run. `scripts/check_ci_contract.py` required the same predicate.

That formed a circular gate: the PR could not become Ready before exact-head GREEN, while the repository-owned job capable of producing that GREEN was forbidden while the PR remained Draft.

## Evidence

- predecessor: `7ee8aeb4abbd6e8460713676803a40b62a43baf2`
- review: `5277042429`
- contract-first commit: `4e2c7b1fd0a655aa79367874518e3e2ce1db5713`
- workflow repair: `68e8ff3ad181f883dcdfec20d6d5e0f54e8b042b`
- changed Product predicate: `github.event_name != 'pull_request' || github.event.action != 'closed'`
- CI contract now rejects reintroduction of `github.event.pull_request.draft == false`
- full predecessor-to-repair compare: `.github/workflows/product.yml` +1/-1 and `scripts/check_ci_contract.py` +9/-3 only

At the time of the repair, the exact repaired head still had no PR-triggered workflow inventory. Source repair therefore does not by itself establish hosted acceptance.

## Alternatives

### Mark the PR Ready to obtain CI

Rejected. Ready is downstream of unchanged-head acceptance in the governed stack; using Ready as an execution trigger would invert the admission contract and would make Draft protection ceremonial.

### Keep Draft suppression and use manual workflow execution

Rejected. `product.yml` does not expose `workflow_dispatch`, and a manual run would not establish the normal pull-request event path that must remain reliable for later commits.

### Execute Product CI on Draft pull-request events

Selected. The workflow already declares `opened`, `synchronize`, `reopened`, `ready_for_review`, `converted_to_draft`, and `closed`. Running all non-closed PR events preserves normal exact-head validation while `cancel-in-progress` limits superseded work within the PR concurrency group.

## Risks and controls

Draft pushes can now consume CI capacity. Existing per-PR concurrency with `cancel-in-progress` bounds obsolete work, and `closed` remains suppressed. This change does not widen repository permissions, alter provider/model routing, or weaken any quality step.

## Follow-up

A fresh native Product run on the unchanged owner head is still required. If a substantive commit on the owner branch produces no `pull_request` workflow generation after this source repair, treat that as a separate trigger/admission/runtime defect; do not bypass it with Ready transitions, no-op commits, synthetic statuses, or predecessor evidence.
