"""Fail closed when Product CI regresses on runner or coverage toolchain identity."""

from __future__ import annotations

from pathlib import Path


WORKFLOW_PATH = Path(".github/workflows/product.yml")


def main() -> int:
    """Validate queue-admission, supersession, and branch-coverage invariants."""
    workflow = WORKFLOW_PATH.read_text(encoding="utf-8")

    required_fragments = (
        "runs-on: ubuntu-24.04",
        "cancel-in-progress: true",
        "actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
        "COVERAGE_TOOLCHAIN: nightly-2026-08-20",
        'rustup toolchain install "$COVERAGE_TOOLCHAIN" --profile minimal --component llvm-tools-preview',
    )
    missing = [fragment for fragment in required_fragments if fragment not in workflow]
    if missing:
        raise SystemExit(
            "Product CI contract missing required fragment(s): " + ", ".join(missing)
        )

    if "runs-on: ubuntu-latest" in workflow:
        raise SystemExit(
            "Product CI must not use ubuntu-latest while current organization "
            "evidence demonstrates selective floating-image starvation"
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
