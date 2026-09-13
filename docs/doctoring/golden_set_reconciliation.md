# Golden-set stale-parent reconciliation record

Date: 2026-09-14

## Evidence boundary

Historical golden-set head `fdf8b8d70c05bcb76c55cb6336c9bf31b5e42ce4` had merge base `51c7df6d03f072449422fd58ca24b2f9d6026f07`. Canonical Research Intake #9 is `a67d9d66b35024d6f2155f50ee5c9fb7d2e1dbe9`. Before repair, comparison was 35 commits ahead / 81 behind and GitHub reported #10 non-mergeable.

Review `5192181682` restated the causal compatibility finding: keep current #9's private `ClassificationReport` and public three-field `ZoteroItem`, move complete raw JSON evidence to an explicit capture/wire boundary, and bind golden approval to every material current-#9 report field without reopening trusted state.

Ordinary merge `c9954e286041c80d08655e092f10007a684ceab1` has parents historical #10 `fdf8b8d70...` and current #9 `a67d9d66...`. Fresh ancestry after that integration is ahead of #9 with zero commits behind; GitHub recomputed #10 as mechanically mergeable. This proves parent adoption only. It does not transfer predecessor checks or authorize merge.

## Semantic reconciliation

The old `ZoteroItem.source_record` design was rejected. Current #9's source shape stays `{ key, version, data }`. `CapturedZoteroItem` instead owns caller-supplied raw JSON and its decoded typed item privately, preventing the two representations from drifting after capture.

Review `5192398634` found a separate provenance-contract flaw in the first reconciliation: `CapturedZoteroItem::try_from(Value)` is public and therefore cannot establish that supplied bytes came from Zotero. Calling that boundary with fabricated JSON could previously mint a digest in the `conceptweave-zotero-provider-snapshot-v3` domain. Domain separation proved which code path produced the digest, not provider origin.

RED `67b5214ce8cd39fddbc1ba6043971b9832b2ef17` fixes the intended semantic contract by requiring caller-constructed raw capture to use a non-authenticating receipt domain. Production repair `2e52972745953fbb5b606226fce2dda2cf61abd3` renames the domain to `conceptweave-zotero-captured-json-snapshot-v3` and makes the public API/rustdoc explicit: complete raw JSON and typed classifier inputs are content-bound, but source/provider provenance requires a separate transport-owned attestation boundary. Typed-fixture receipts remain distinct and likewise make no provider-origin claim.

The old evaluator's direct `ClassificationReport` field access was rejected. `GoldenSnapshot`, `classification_proposal_digest`, structural validation and evaluation all consume current #9 read-only accessors. Existing compile-fail/private aggregate behavior therefore remains in the adopted parent rather than being counteracted by the child.

Historical corruption tests that mutated public report internals were replaced by externally reachable cases: duplicate source identity, changed same-revision source content, changed unreviewed proposal/source evidence, stale or locally rewritten receipts, invalid labels, and malformed raw capture. States that only exist after bypassing the trusted constructor are no longer made reachable solely for testing.

## Receipt compatibility

Caller-captured raw JSON snapshot domain: `conceptweave-zotero-captured-json-snapshot-v3`.

Typed fixture snapshot domain: `conceptweave-zotero-typed-snapshot-v3`.

Proposal domain: `conceptweave-classification-proposals-v3`.

The superseded `conceptweave-zotero-provider-snapshot-v3` label is not provider-authentication evidence and must not be migrated by local digest recomputation. #10 remains Draft and unreleased, so any historical local receipt under that label is invalidated and requires fresh capture plus independent approval if retained as evidence. v1/v2 proposal and v2 snapshot receipts remain historical evidence under the same rule.

If authenticated provider origin becomes a product requirement, the evidence must be minted at a canonical transport adapter that actually observes the provider response and binds repository/library/version/transport identity or equivalent attestation. A public `Value` constructor cannot grant that authority.

## Acceptance status

The ordinary integration and source repairs are not exact-head GREEN. The execution environment used for this reconciliation has no usable Rust toolchain, and repository-owned Product/security/review workflows are still unavailable on this ConceptWeave PR head. CodeRabbit status alone is not sufficient acceptance. Keep #10 Draft until unchanged-head Rust 1.98 and hosted/review evidence are established.
