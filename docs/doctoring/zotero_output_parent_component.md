# Zotero report output parent-component admission

Status: `LOCAL_EXACT_HEAD_GREEN_PENDING_HOSTED_REVIEW`

## Problem

`validate_output_path` had been changed at `8cc40c68f33ccd6da3d46b88d7995abb0f653317` to treat `Path::file_name()` as infallible once the path's parent canonicalized into an allowed temporary directory. That premise is false. Rust's `std::path::Path::file_name` explicitly returns `None` when a path terminates in `..`; therefore an absolute path such as `env::temp_dir().join("..")` can have an allowed, canonicalizable parent and still have no file name.

The result was a process panic at the filesystem admission boundary rather than the existing fail-closed `io::ErrorKind::InvalidInput` contract. The change also made `AGENTS.md` encode the incorrect invariant.

Primary reference: Rust Project Developers. (2026). *std::path::Path::file_name*. Rust standard library documentation. https://doc.rust-lang.org/std/path/struct.Path.html#method.file_name

## Reality RED

`bd847fd1b1a85e701eb0a87be4d965d2bb0eb50f` adds the parent-component case to `output_path_must_be_a_new_direct_temp_child`:

- input: `env::temp_dir().join("..")`;
- required result: `io::ErrorKind::InvalidInput`;
- predecessor behavior: `expect("canonicalized child path has a file name")` panics before an `io::Result` can be returned.

This is a behavioral regression of the earlier fail-closed implementation, not a synthetic coverage-only branch.

## Causal repair

`bdb1b7cce53efb42113685de82aa86d009f54b86` removes the `expect` and restores `Path::file_name().ok_or_else(...)`, returning `InvalidInput` for an admitted absolute path without a normal final component. It does not widen output parents, relax create-new publication, alter temporary-file cleanup, or change Zotero/source-resolution semantics.

`a285db3725e82f87d3872868eedd39f85ced2259` repairs the corresponding `AGENTS.md` invariant: parent canonicalization does not imply that a final file-name component exists.

## Rejected alternatives

- Keeping the `expect` to reduce a raw coverage denominator was rejected because coverage work cannot convert a documented fallible path operation into an invariant.
- Special-casing `/tmp/..` was rejected because the contract is component-based and must remain portable across system temporary directories and Windows path syntax.
- Marking the arm `coverage(off)` was rejected because the path is reachable through ordinary CLI input and is therefore owned production behavior.

## Acceptance

No predecessor execution transferred after the RED/source/docs commits. Frozen local verification at source/test successor `3bccda338a2fc34e57ae6767c1a643cb744c48d5` passes the repository's locked Rust 1.98 workspace tests, `cargo fmt --all -- --check`, all-target strict Clippy, warnings-denied rustdoc/release, LLVM native function coverage 100%, and normalized owned-production source-function/region/branch 100%. The live PR ref remains the authority for later successors. The PR is Ready for review, but applicable hosted checks and qualifying independent approval remain unverified; this lane is not protected-merge GREEN.
