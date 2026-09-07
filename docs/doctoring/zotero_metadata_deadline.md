# Zotero metadata read deadline

Status: owner repair and the complete forward chain locally verified; #9 and its 27 descendants through #38 normally pushed, #39 local only pending a fresh remote gate. Protected integration remains required. No actual library or paper artifact was read in this experiment.

## Finding and cause

[PR #9's unresolved review](https://github.com/ContextualWisdomLab/ConceptWeave/pull/9#discussion_r3935157013) identified that one item per page can trigger up to 50,000 requests, each with a fresh timeout. Exact baseline `a2a84884f67dcac6f6892c958d55450aea6d6c88` had item/byte bounds but no total elapsed-time bound. The later research descendants retained the same unbounded body before this repair. The real call path is `read_local_snapshot` → `read_snapshot_with` → page transport → complete `classify_snapshot`; the CLI receives a report only after that function returns.

## Decision and limits

Reuse that reader, `ReadError::Budget`, and `std::time::Instant`; add a private injected elapsed clock for deterministic tests. Check the five-minute budget before each fetch, after its return and after report classification. Choosing a maximum page count would reject valid short pages without bounding a few slow responses. Replacing transport or adding a cancellation service is unnecessary to deny late results. The trade-off is cooperative admission/completion: an in-flight request keeps its existing timeout and classification is not preempted. System-suspend accounting and monotonic clock implementation are platform-dependent (Rust Project Developers, n.d.). This does not establish provider authentication, source atomicity or any model timeout.

The accepted report still includes the full observed denominator. No late/partial report escapes, no source is deleted or relabeled, and no transport, public signature, dependency, byte/item ceiling, approval or write authority changes. A longer successful observation window would require a separately evidenced operational decision; do not reduce the paper denominator to make a run pass.

## Executed evidence

- Baseline `a2a8488`: 38 tests / 10 unfiltered suites, including two doctests.
- Committed RED `aff539f`: private clock seam plus three regression functions, but no guard. Two rejection tests fail with an actual returned report; the valid short-page control passes. No sleep or real Zotero data is involved.
- GREEN `e6b2a2214b39106ddacc753595b72a699d53d04f`: 41 tests / 10 unfiltered suites, including two doctests; explicit Rust 1.98.0. The tests cover seven individually timely 50-second pages, zero-I/O expired admission, late page, between-page expiry, late classification, late empty library and success one nanosecond below the limit.
- Strict all-target Clippy, warnings-denied rustdoc, release build, formatting and existing CI contract pass. The unchanged coverage script passes 108/108 functions, 703/703 normalized source regions and 100/100 normalized branches. Raw LLVM remains below 100%: 1,051/1,052 lines, 1,600/1,606 regions and 99/100 branches. No threshold or exclusion changed.

Reproduce with `cargo +1.98.0 test --workspace --locked` and `bash scripts/check_coverage.sh`. Logs are `/tmp/conceptweave-pr9-deadline-{baseline,red,green,coverage}-20260906.log`. The initial provider review references API pagination, which permits bounded pages; that does not supply an application-wide read deadline (Zotero, n.d.). The same TRD amendment removes the separately reviewed stale schema-42 requirement and states the implemented API-v3/present-stable-schema contract; no parser behavior changes for that documentation correction.

At this owner-only checkpoint, the required follow-up was final-head verification, a fresh writer/head/base check, normal #9 push and full forward propagation with per-head checks. The checkpoint below records the executed work and remaining remote gate. Local success does not resolve protected approval, provider transport security or other open findings. No predecessor may be closed to hide missing propagation.

## Forward integration checkpoint — September 6, 2026

Every integration below passed explicit Rust 1.98.0 locked workspace tests, strict all-target Clippy, warnings-denied rustdoc, formatting, the unchanged CI contract and diff checks. Test counts include doctests and only terminal summaries containing the exact `; 0 filtered out;` suffix. Each merge has the preserved child as its first parent and the verified predecessor as its second parent. A separate ancestry/diff audit confirmed both parents and exactly the six intended source/document files; no Cargo, workflow or script delta was introduced.

#9 was normally pushed at `bb2faccfda9efed55b6759f1bbf7907bf6ec0c3b`. Its 27 descendants through #38 were normally pushed and named remote heads matched the verification ledger. #39's final integration is local only. Conflicts in #10/#34/#37 TRD, #17 adjacent constants, and #22/#39 PRD were resolved by retaining both intended changes: the later schema, abstract retention, source-discovery and full-text contracts were not discarded. No later capability was reverse-merged into #9, no branch was force-pushed, and no predecessor was closed.

| PR | Preserved child | Verified parent | Integration head | Tests / suites |
| --- | --- | --- | --- | ---: |
| #10 | `d4f39b45901ea741baa893a3d6117c5322b7dcdf` | `bb2faccfda9efed55b6759f1bbf7907bf6ec0c3b` | `4bb633305b04a1dd4c4ce526806c9469bcb79fd3` | 58 / 14 |
| #11 | `11d158b105cbd03edc34452358ebb3ff445e388e` | `4bb633305b04a1dd4c4ce526806c9469bcb79fd3` | `1dc032598b41a35d52c09d8690c871e07365d7e3` | 60 / 15 |
| #12 | `ee2c494a3136a4bfa520c29f1c938b621e1ecb9c` | `1dc032598b41a35d52c09d8690c871e07365d7e3` | `a4a7c2d56fc592ef1c7abf64ca6875b0fe10c5ee` | 68 / 17 |
| #13 | `8a684882005085d8b3cb47812e185975084e0475` | `a4a7c2d56fc592ef1c7abf64ca6875b0fe10c5ee` | `b41217b1d38ec8d30e365aac04e68684c09dca7f` | 75 / 18 |
| #15 | `a07dd9a433c7211c2f95065031622d51dadf2cb6` | `b41217b1d38ec8d30e365aac04e68684c09dca7f` | `45e9c4933eae4482b0361473e9f083967182e6cc` | 87 / 19 |
| #16 | `873c46b7dcf2930a98cf7ef7ff8bdcbcf04f17d5` | `45e9c4933eae4482b0361473e9f083967182e6cc` | `044018cef4e5d3e919b278a1cfebe56857863601` | 87 / 19 |
| #17 | `cf93f5323d97e718c8ff986c8e780bfaa26fb765` | `044018cef4e5d3e919b278a1cfebe56857863601` | `c88f9a34c1fc4e72e38cf66b1d2f3fcb305e560a` | 100 / 19 |
| #18 | `5e33981ccb0691e0a24260652c44fe8e28afb8d9` | `c88f9a34c1fc4e72e38cf66b1d2f3fcb305e560a` | `fe2cff4f9fc40496bbb4339ba4242543beacea9b` | 108 / 20 |
| #19 | `de9df48d727fe22a3f1efb881d7d17ef0566b620` | `fe2cff4f9fc40496bbb4339ba4242543beacea9b` | `62c19ee23e3c827bc7db15c79f4755ff040489e9` | 109 / 20 |
| #20 | `4fb1ad073cfbe25d269d063c90914636f27619da` | `62c19ee23e3c827bc7db15c79f4755ff040489e9` | `a03a7248c894a1e0765968ddf58514d98c517da3` | 116 / 21 |
| #21 | `9302f8525aa3b3aba2f68576a97b6d2c853f2819` | `a03a7248c894a1e0765968ddf58514d98c517da3` | `09c84e4cdb1393a5e450f5200b87f292eeea956f` | 119 / 22 |
| #22 | `0b0691ee264264a7c50f894ef0190d12d97dea0d` | `09c84e4cdb1393a5e450f5200b87f292eeea956f` | `7179d13b45d160682e4cce1473c145d465fe657b` | 120 / 23 |
| #23 | `1912c21d9fbe895db62cd9735b44f36fc2b19221` | `7179d13b45d160682e4cce1473c145d465fe657b` | `2a3619f52e1d3e4f699c91be1fc2d0e9a6e234c8` | 121 / 23 |
| #24 | `ba3a691bb246258d2a27bc83c308be21031e310e` | `2a3619f52e1d3e4f699c91be1fc2d0e9a6e234c8` | `1e73e1545de32ae9a349c469a7794c5c3fc2ae9b` | 123 / 23 |
| #25 | `6af51119f434035c9e8fc2743e8327c0199a8c92` | `1e73e1545de32ae9a349c469a7794c5c3fc2ae9b` | `c6b4c17e931951a2e1d4ea79ac79363f6306a5bf` | 126 / 24 |
| #26 | `f60c07ff0dc0b6933bbf5fa097b5bb72b0b24fea` | `c6b4c17e931951a2e1d4ea79ac79363f6306a5bf` | `e2dc6006ed3e56d8388e82912826cf37efed0541` | 128 / 24 |
| #27 | `9c4ecf5fc8bc3e16c3aaffc10ba0498e59128f9d` | `e2dc6006ed3e56d8388e82912826cf37efed0541` | `3df0c124f390797bacaba8ffdf229f502b0e9bf3` | 133 / 25 |
| #28 | `b411b66c2ec34c39bb0cceb27f96221a1fda4416` | `3df0c124f390797bacaba8ffdf229f502b0e9bf3` | `ba6b3dfc71cf89ed4c57b85da0dd9ca5f983efee` | 138 / 26 |
| #29 | `7af6881fc567fbec671f91d3590b4d4d47cf9f50` | `ba6b3dfc71cf89ed4c57b85da0dd9ca5f983efee` | `f73705e15f1236fa8bd34fec032bc78d9b57760c` | 149 / 28 |
| #30 | `a43ddceaf5dfaa4daba5270b954bda9f42a59cdb` | `f73705e15f1236fa8bd34fec032bc78d9b57760c` | `a11e889d1680ab4d91f3565e3debf7ed0f10ba23` | 156 / 32 |
| #31 | `0dad7ca52bd2932e82bbf34eb4b7f4aec6b4f3f2` | `a11e889d1680ab4d91f3565e3debf7ed0f10ba23` | `61072a70f7ec5a1fbd0b477430aacb8e770aa109` | 158 / 33 |
| #32 | `f0bf02a8a600924d254adfa7b4796aa2ef868165` | `61072a70f7ec5a1fbd0b477430aacb8e770aa109` | `1e711728d83efc1e60fc3d43ba0c67c467dd6a43` | 161 / 34 |
| #33 | `18932783aeb336a6d58e8a19f6d7dd6ecfb9ab3a` | `1e711728d83efc1e60fc3d43ba0c67c467dd6a43` | `d82b2f2896bc4cfef5d34ac6f6f83f7cee1072f6` | 166 / 36 |
| #34 | `853517c1c43fdaeea0cef57b2f0e63a34b0fe2db` | `d82b2f2896bc4cfef5d34ac6f6f83f7cee1072f6` | `b9060df2cb1ea02314be429932031fc07de1de30` | 171 / 37 |
| #36 | `87f02a91a9b5ea83ae842ef6b7eb83141aebfd66` | `b9060df2cb1ea02314be429932031fc07de1de30` | `d3f991dcc1b746afed7c36f315e8937c39390c5e` | 201 / 38 |
| #37 | `692cb588b26a9cc878fbaa2b47aa30fd83ea47de` | `d3f991dcc1b746afed7c36f315e8937c39390c5e` | `2688b508e36ac1be15c15717566a0bc165ab962d` | 216 / 39 |
| #38 | `8e057652ee7784b373beeeec865d80dd3db773be` | `2688b508e36ac1be15c15717566a0bc165ab962d` | `e2c3a9fbbe36f44525833d4a94e164c6891a0f94` | 243 / 41 |
| #39 (local only) | `ec1435379e5fb29fbd7842137a2003d8f3363655` | `e2c3a9fbbe36f44525833d4a94e164c6891a0f94` | `e1407d64e67be3556088c36d334427b7de103378` | 258 / 41 |

The final integrated source `e1407d64e67be3556088c36d334427b7de103378` passes **258 tests / 41 unfiltered suites, including seven doctests**, the release build and the unchanged coverage gate: **415/415 functions, 4,465/4,465 source-normalized regions and 762/762 normalized branch outcomes**. Raw LLVM is **5,041/5,115 lines, 7,281/7,438 regions and 707/762 branches**, not 100%. Intermediate-head coverage is not inferred from the owner and final endpoints. Logs: `/private/tmp/conceptweave-deadline-pr{10..38}-20260906.log` for the listed PRs, `/tmp/conceptweave-deadline-root-{tests,clippy,rustdoc,release,coverage}-20260906.log`, and `/private/tmp/conceptweave-deadline-cascade-results-20260906.tsv`. The local guarded runner is `/private/tmp/conceptweave-deadline-cascade-20260906.sh`; it verifies current branch identity, stops on conflicts/failures and supports resumption from the ledger. These temporary paths are execution evidence, not shipped product dependencies.

## Remote acceptance and quota boundary

The #9 deadline review received an [exact-source response](https://github.com/ContextualWisdomLab/ConceptWeave/pull/9#discussion_r3943584189), independently reread. #9 and #10–#37 bodies were updated and reread at their corresponding heads; #38's body refresh and #39's final push/body remain pending. No discussion was resolved as a substitute for independent acceptance.

During the final #39 state read, ordinary GraphQL returned an API-rate-limit failure, confirmed at **10:05:51 UTC on September 6**. The separate quota-status endpoint then reported zero usage/full remaining quota and an 11:06:12 UTC reset; that contradictory summary does not override the actual rejected PR read. It was used only to choose a conservative next check time, not to obtain PR facts through another endpoint. No replacement token, alternate PR endpoint or push after the failed gate was used. Cached PR head/base fields also lagged earlier successful pushes; actual named refs were verified while normal access was available.

Last confirmed #39 remote head remains `ec1435379e5fb29fbd7842137a2003d8f3363655`, OPEN Draft. Its newly pushed named parent is #38 `e2c3a9fbbe36f44525833d4a94e164c6891a0f94`. After the conservative cooldown, obtain one normal fresh head/base/writer read before any push; integrate concurrent deltas normally if present. Then push the local successor, refresh #38/#39 bodies, verify exact heads and all changed-head hosted checks/reviews, and continue canonical root repair. Do not rerun the completed 27 integrations solely to consume a wait.

The last complete audit in this increment found 33 open PRs, 32 Draft and 36 unresolved threads, with no current-head approval among each PR's latest 30 returned reviews; #6 had older review history outside that window. Protected main remains the last verified `f4f440dd58c77d7cd90dff8a1eb2eeb9a9940425`. Fresh ruleset 18156473 requires one independent approval, stale-review dismissal, resolved threads, seven central workflows and deletion/non-fast-forward protection. #9's exact pushed head had zero check-runs/Actions and only CodeRabbit's Draft-skip status, not hosted Product GREEN. No final all-PR snapshot is claimed after quota exhaustion.

Lifecycle capability remains 28, bounded source audits 33/76, source-resolved GitHub releases 7/33, verified adoption zero, and last authentic decisions and independent approvals each 0/3,715. The regression metric improves from one passing/two failing deadline tests to three passing tests; it is not paper reclassification or publication. No actual Zotero/model request, private artifact read, semantic decision, approval issuer, write or rollback occurred. The full goal remains active.

## References

Rust Project Developers. (n.d.). *Instant in std::time* [Rust standard-library documentation]. Retrieved September 6, 2026, from https://doc.rust-lang.org/stable/std/time/struct.Instant.html

Zotero. (n.d.). *Zotero Web API v3: Basics*. Retrieved September 6, 2026, from https://www.zotero.org/support/dev/web_api/v3/basics
