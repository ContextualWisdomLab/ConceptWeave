# Classification report provenance seal

## Scope and owner

Research Intake PR #9 owns `ClassificationReport` construction and the immutable Zotero snapshot provenance represented by that aggregate. Pending-source resolution may consume a report but must not create a second report authority. This note records the owner-local prerequisite raised as Issue #41.

## Current finding

At predecessor `b98ba39f9efa4c18c1bcdc3bcae4a8ef4f0abd41`, `ClassificationReport` is a public `Serialize`-only type whose trust coordinates and retained inventory are public mutable fields. `prepare_source_resolution_review` validates a resolution against the values currently present in the supplied report. It therefore proves value consistency with that caller-supplied object, not continuity with the object originally returned by `classify_snapshot` or the Local API reader.

Committed case `5212bf1af6b3b54824a5efb1d94b945e81e1bf68` constructs one report, coherently changes `zotero_version`, `server_id`, `library_version`, `rule_revision`, and the retained source revision, then supplies a resolution matching those changed values. Acceptance requires rejection: a caller must not manufacture a new trusted snapshot identity by modifying both sides of the equality checks.

The case is committed but has not been executed in this environment. There is no Rust toolchain here and no pull-request workflow run for that exact commit. It is therefore a pending behavioral RED case, not executable RED/GREEN evidence.

That statement records the original doctoring checkpoint at `63c22bb0dfa2ca13b703928fe4f8aeafa947bba3`. The later execution evidence below supersedes only its local-toolchain limitation; no hosted or protected-branch evidence is implied.

## Repair constraint

Rust visibility is a suitable zero-runtime-cost boundary for preserving constructor invariants: public structs may keep fields private, and external code cannot access private fields directly. The preferred repair is therefore constructor-bound trusted state rather than another database, cross-service lookup, mutable sibling-PR dependency, or a second copy of Zotero truth.

The smallest compatible design should satisfy all of the following:

- an externally supplied caller cannot construct or mutate the trusted snapshot identity/inventory admitted by source-resolution review;
- read-only access remains available for existing report consumers;
- the existing JSON proposal shape remains stable unless an explicit migration is documented;
- serialization alone does not silently mint a new trusted aggregate on restore;
- `prepare_source_resolution_review` accepts only the constructor-bound trusted report type or first verifies an opaque constructor-bound receipt;
- Local API intake stays read-only and all classifications remain proposed/non-authoritative;
- no cryptographic authenticity is claimed unless the receipt is independently anchored by a released signing or evidence authority.

A transparent newtype/view split is one viable implementation: keep the serialized report projection as the readable target, wrap it in a trusted report whose constructor/inner state is not externally writable, and do not expose `DerefMut`, public reconstruction, or trusted `Deserialize`. Serde documents `#[serde(transparent)]` as preserving the representation of a one-field newtype, which can avoid gratuitous JSON nesting. This is a design candidate, not an accepted implementation until the API migration compiles and the regression executes.

## Verification sequence

1. Run the committed coherent-mutation case on its exact pre-repair head and record the actual failure.
2. Apply the minimum #9 owner repair and update affected tests/callers without weakening existing snapshot checks.
3. Re-run the coherent-mutation regression and existing source-resolution, read-only intake, lifecycle, abstention, duplicate-provenance, transport, and output tests on one unchanged head.
4. Run locked Rust 1.98 workspace tests, formatting, all-target strict Clippy, warnings-denied rustdoc/release, and owned-production function/region/branch coverage.
5. Obtain applicable hosted checks and independent review before resolving the finding.
6. Restack #40 normally onto the released/reviewed #9 contract; predecessor evidence does not transfer.

## Executed owner repair checkpoint

The committed regression failed on exact pre-repair head `63c22bb0dfa2ca13b703928fe4f8aeafa947bba3`: a caller could coherently mutate the report identity and retained inventory, then submit matching source-resolution values for acceptance. Local source repair `826987122124c86791e3958b996c8ea871d0964c` closes that path by making all eleven `ClassificationReport` fields private and exposing read-only accessors only. The existing classifier remains the only public constructor, the serialized JSON field names and values remain unchanged, and no trusted deserialization or public reconstruction path was added.

The same source commit passed the focused provenance/source-resolution regressions, the compile-fail documentation case, locked Rust 1.98.0 workspace tests, formatting, all-target/all-feature strict Clippy, warnings-denied rustdoc, release build, and `git diff --check`. The frozen coverage command remained RED: raw LLVM coverage was 155/160 branches, 190/195 functions, 1,738/1,761 lines, and 2,789/2,841 regions; the repository normalization reported 159/160 branches and 1,130/1,187 source regions. Those uncovered predecessor paths remain a protected-readiness gap and were not hidden or excluded by this repair.

This is an intentional Rust source-compatibility break for callers that used field syntax or struct literals. Stacked consumers must migrate to immutable accessors or an owner-defined constructor before restacking. Serialization compatibility alone does not prove peer authentication, cryptographic authenticity, steward approval, or Zotero write authority.

## References

The Rust Project Developers. (n.d.). *Visibility and privacy*. The Rust Reference. Retrieved September 9, 2026, from https://doc.rust-lang.org/reference/visibility-and-privacy.html

Serde Developers. (n.d.). *Container attributes*. Serde. Retrieved September 9, 2026, from https://serde.rs/container-attrs.html
