# Zotero metadata read deadline

Status: locally verified source repair; protected integration and forward-stack verification remain required. No actual library or paper artifact was read in this experiment.

## Finding and cause

[PR #9's unresolved review](https://github.com/ContextualWisdomLab/ConceptWeave/pull/9#discussion_r3935157013) identified that one item per page can trigger up to 50,000 requests, each with a fresh timeout. Exact baseline `a2a84884f67dcac6f6892c958d55450aea6d6c88` has item/byte bounds but no total elapsed-time bound. The same body remains in later research descendants. The real call path is `read_local_snapshot` → `read_snapshot_with` → page transport → complete `classify_snapshot`; the CLI receives a report only after that function returns.

## Decision and limits

Reuse that reader, `ReadError::Budget`, and `std::time::Instant`; add a private injected elapsed clock for deterministic tests. Check the five-minute budget before each fetch, after its return and after report classification. Choosing a maximum page count would reject valid short pages without bounding a few slow responses. Replacing transport or adding a cancellation service is unnecessary to deny late results. The trade-off is cooperative admission/completion: an in-flight request keeps its existing timeout and classification is not preempted. System-suspend accounting and monotonic clock implementation are platform-dependent (Rust Project Developers, n.d.). This does not establish provider authentication, source atomicity or any model timeout.

The accepted report still includes the full observed denominator. No late/partial report escapes, no source is deleted or relabeled, and no transport, public signature, dependency, byte/item ceiling, approval or write authority changes. A longer successful observation window would require a separately evidenced operational decision; do not reduce the paper denominator to make a run pass.

## Executed evidence

- Baseline `a2a8488`: 38 tests / 10 unfiltered suites, including two doctests.
- Committed RED `aff539f`: private clock seam plus three regression functions, but no guard. Two rejection tests fail with an actual returned report; the valid short-page control passes. No sleep or real Zotero data is involved.
- GREEN `e6b2a2214b39106ddacc753595b72a699d53d04f`: 41 tests / 10 unfiltered suites, including two doctests; explicit Rust 1.98.0. The tests cover seven individually timely 50-second pages, zero-I/O expired admission, late page, between-page expiry, late classification, late empty library and success one nanosecond below the limit.
- Strict all-target Clippy, warnings-denied rustdoc, release build, formatting and existing CI contract pass. The unchanged coverage script passes 108/108 functions, 703/703 normalized source regions and 100/100 normalized branches. Raw LLVM remains below 100%: 1,051/1,052 lines, 1,600/1,606 regions and 99/100 branches. No threshold or exclusion changed.

Reproduce with `cargo +1.98.0 test --workspace --locked` and `bash scripts/check_coverage.sh`. Logs are `/tmp/conceptweave-pr9-deadline-{baseline,red,green,coverage}-20260906.log`. The initial provider review references API pagination, which permits bounded pages; that does not supply an application-wide read deadline (Zotero, n.d.). The same TRD amendment removes the separately reviewed stale schema-42 requirement and states the implemented API-v3/present-stable-schema contract; no parser behavior changes for that documentation correction.

Next: revalidate the final documentation head, normal-push #9 after a fresh writer/head/base check, merge its delta forward through every dependent research PR without reversing later features, and rerun each changed head's checks. Local success does not resolve protected approval, provider transport security or the other open findings. No predecessor may be closed to hide missing propagation.

## References

Rust Project Developers. (n.d.). *Instant in std::time* [Rust standard-library documentation]. Retrieved September 6, 2026, from https://doc.rust-lang.org/stable/std/time/struct.Instant.html

Zotero. (n.d.). *Zotero Web API v3: Basics*. Retrieved September 6, 2026, from https://www.zotero.org/support/dev/web_api/v3/basics
