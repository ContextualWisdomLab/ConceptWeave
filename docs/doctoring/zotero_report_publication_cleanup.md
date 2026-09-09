# Zotero local-report publication cleanup

Status: `SOURCE_TEST_REPAIRED_PENDING_CI` on Research Intake PR #9. This note records a canonical owner repair only; it is not executable GREEN, semantic approval, protected integration, or Zotero write authority.

## Problem boundary

The Research Intake CLI publishes a private local proposal report by writing a create-new temporary file and then creating the final pathname with `hard_link`. On predecessor `e1fd933bd11eb3c0cf3cf26776a9d357fca40713`, a failure while removing the temporary pathname after that successful link entered a rollback branch that called `remove_file(output)`.

That rollback is unsafe after publication. The final pathname can be removed and replaced by another actor between the successful link and cleanup recovery. A cleanup failure must therefore never authorize deletion of the final pathname; doing so can remove unrelated data. This is a local artifact-integrity boundary, not a Zotero mutation concern.

The dependent source-resolution lane had already reproduced the same defect with committed RED `1b9b0960b2d4a8a56900062f6b1c9200db0a3869`. Research Intake independently verified the vulnerable code still existed on its live exact head and adopted only the causal invariant rather than reverse-merging descendant source-resolution semantics.

## Canonical owner repair

Owner repair `4b142fcc4330afbc4bb7115667cbd39daf9c0eb9` changes only `crates/conceptweave-zotero/src/main.rs` around report publication and its deterministic tests:

- after `hard_link` succeeds, cleanup is confined to the private temporary pathname;
- a cleanup error keeps its original `io::ErrorKind` and is annotated as a post-publication cleanup failure;
- the recovery path never receives or unlinks the final output pathname;
- serialization, flush-before-publication, create-new semantics, output-parent admission, Unix `0600` mode, Zotero read/classification, proposal lifecycle, and governance authority are unchanged;
- deterministic unit regressions assert that cleanup attempts only the temporary path and that the returned error identifies the post-publication state while preserving its I/O kind.

The dependent #40 lane was then ordinarily restacked with a non-force two-parent integration so the canonical #9 repair is ancestry. Equivalent descendant cleanup history remains dated evidence, not ownership reversal.

## Acceptance and traceability

Current owner path: `run_with` → `write_report` → `hard_link` → `cleanup_published_report` in `crates/conceptweave-zotero/src/main.rs`.

Required GREEN is one unchanged exact #9 successor passing all of the following without exclusions or threshold weakening:

- `cargo +1.98.0 test --workspace --locked`;
- `cargo +1.98.0 fmt --all -- --check`;
- `cargo +1.98.0 clippy --workspace --all-targets --locked -- -D warnings`;
- `RUSTDOCFLAGS='-D warnings' cargo +1.98.0 doc --workspace --no-deps --locked`;
- warnings-denied release build;
- owned production function / normalized-region / branch coverage at 100%;
- applicable hosted checks and independent review evidence.

No pull-request workflow run existed for exact source repair `4b142fcc...` when this note was created. CodeRabbit Draft status is not executable or independent-review evidence. Predecessor Rust/coverage results do not transfer.

## Residuals

`docs/product-technical-gap-baseline.md` still contains older Research Intake/Product-bootstrap coordinates and needs a safe reconciliation that preserves its dated evidence while moving this cleanup lane into the current checkpoint. The three standalone PDFs plus one standalone note remain governed pending sources; successful local-report publication does not resolve, classify, approve, rebind, publish, or mutate them.
