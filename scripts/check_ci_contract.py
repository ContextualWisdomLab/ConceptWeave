"""Fail closed when Product CI regresses on queue or quality-gate identity."""

from __future__ import annotations

from pathlib import Path


WORKFLOW_PATH = Path(".github/workflows/product.yml")


def main() -> int:
    """Validate executable PR admission, supersession, and coverage invariants."""
    workflow = WORKFLOW_PATH.read_text(encoding="utf-8")

    required_fragments = (
        "runs-on: ubuntu-24.04",
        "types: [opened, synchronize, reopened, ready_for_review]",
        "group: ${{ github.workflow }}-${{ github.repository }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}",
        "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
        "actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
        "COVERAGE_TOOLCHAIN: nightly-2026-08-20",
        'rustup toolchain install "$COVERAGE_TOOLCHAIN" --profile minimal --component llvm-tools-preview',
    )
    missing = [fragment for fragment in required_fragments if fragment not in workflow]
    if missing:
        raise SystemExit(
            "Product CI contract missing required fragment(s): " + ", ".join(missing)
        )

    prohibited_fragments = (
        "converted_to_draft",
        "types: [opened, synchronize, reopened, ready_for_review, converted_to_draft, closed]",
        "github.event.pull_request.draft == false",
        "github.event.action != 'closed'",
    )
    present = [fragment for fragment in prohibited_fragments if fragment in workflow]
    if present:
        raise SystemExit(
            "Product CI contract contains dead or Draft-suppressing fragment(s): "
            + ", ".join(present)
        )

    if "runs-on: ubuntu-latest" in workflow:
        raise SystemExit(
            "Product CI must not use ubuntu-latest while current organization "
            "evidence demonstrates selective floating-image starvation"
        )

    if "github.event.pull_request.number || github.ref" in workflow:
        raise SystemExit("Product CI must isolate non-PR runs by run_id")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
