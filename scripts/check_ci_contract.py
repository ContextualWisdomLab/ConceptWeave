"""Fail closed when Product CI regresses queue admission or pinned validation."""

from __future__ import annotations

from pathlib import Path


WORKFLOW_PATH = Path(".github/workflows/product.yml")
COVERAGE_SCRIPT_PATH = Path("scripts/check_coverage.sh")
RUST_ADOPTION_CHECKER_PATH = Path("scripts/check_rust_workspace_adoption.py")
RUST_ADOPTION_TEST_PATH = Path("scripts/test_rust_workspace_adoption.py")
SEMANTIC_CHECKER_PATH = Path("scripts/check_semantic_candidate_contracts.mjs")
SEMANTIC_ADOPTION_TEST_PATH = Path("scripts/test_semantic_contract_adoption.mjs")
PACKAGE_JSON_PATH = Path("package.json")
PACKAGE_LOCK_PATH = Path("package-lock.json")
RUST_ADOPTION_GUARD = "if: steps.rust_workspace.outputs.adopted == 'true'"
RUST_GUARDED_STEPS = (
    "Install pinned Rust toolchain",
    "Verify pinned Rust toolchain",
    "Format",
    "Clippy",
    "Test",
    "Release build",
    "Public documentation",
    "Install cargo-llvm-cov",
    "Install pinned branch-coverage toolchain",
    "Exact owned coverage",
    "Lockfile freshness",
    "Remove Rust build artifacts",
)
SEMANTIC_STEP_ORDER = (
    "Install pinned Node.js runtime",
    "Verify pinned Node.js runtime",
    "Install pinned JSON Schema validator",
    "Test staged semantic contract adoption",
    "Validate public JSON contracts",
    "Remove installed Node dependencies",
)


def _workflow_step_block(workflow: str, step_name: str) -> str:
    """Return one named workflow step without parsing unrelated YAML."""
    marker = f"      - name: {step_name}\n"
    start = workflow.find(marker)
    if start < 0:
        raise SystemExit(f"Product CI contract missing workflow step: {step_name}")
    next_step = workflow.find("\n      - name: ", start + len(marker))
    return workflow[start:] if next_step < 0 else workflow[start:next_step]


def _require_step_order(workflow: str, step_names: tuple[str, ...]) -> None:
    """Fail when an ordered validation chain is missing or reordered."""
    offsets = []
    for step_name in step_names:
        marker = f"      - name: {step_name}\n"
        offset = workflow.find(marker)
        if offset < 0:
            raise SystemExit(f"Product CI contract missing workflow step: {step_name}")
        offsets.append(offset)
    if offsets != sorted(offsets):
        raise SystemExit(
            "Product CI semantic validation steps must preserve pinned-runtime order: "
            + " -> ".join(step_names)
        )


def main() -> int:
    """Validate Product CI ownership, Draft execution, and deterministic tooling."""
    workflow = WORKFLOW_PATH.read_text(encoding="utf-8")

    required_fragments = (
        "runs-on: ubuntu-24.04",
        "types: [opened, synchronize, reopened, ready_for_review, converted_to_draft, edited, closed]",
        "group: ${{ github.workflow }}-${{ github.repository }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}-${{ github.event.action == 'edited' && github.event.changes.base == null && 'metadata-only' || 'validation' }}",
        "cancel-in-progress: ${{ github.event_name == 'pull_request' && (github.event.action != 'edited' || github.event.changes.base != null) }}",
        "name: ${{ github.event.action == 'edited' && github.event.changes.base == null && 'Product metadata-only' || 'Product acceptance' }}",
        "if: ${{ github.event.action != 'closed' && (github.event.action != 'edited' || github.event.changes.base != null) }}",
        "actions/checkout@9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0",
        "python3 scripts/test_rust_workspace_adoption.py",
        "id: rust_workspace",
        'python3 scripts/check_rust_workspace_adoption.py >> "$GITHUB_OUTPUT"',
        "RUSTUP_TOOLCHAIN: 1.98.1",
        'rustup toolchain install "$RUSTUP_TOOLCHAIN" --profile minimal --component rustfmt --component clippy',
        'test "$(rustc --version | awk \'{print $2}\')" = "$RUSTUP_TOOLCHAIN"',
        'rustup show active-toolchain | grep -Eq "^${RUSTUP_TOOLCHAIN}-"',
        "cargo clippy --workspace --all-targets --locked -- -D warnings",
        "cargo test --workspace --locked",
        "cargo build --workspace --release --locked",
        "cargo doc --workspace --no-deps --locked",
        "actions/setup-node@820762786026740c76f36085b0efc47a31fe5020",
        "node-version: '24.21.0'",
        "package-manager-cache: false",
        "COVERAGE_TOOLCHAIN: nightly-2026-08-20",
        'rustup toolchain install "$COVERAGE_TOOLCHAIN" --profile minimal --component llvm-tools-preview',
        "bash scripts/check_coverage.sh",
        "npm ci --ignore-scripts --no-audit --no-fund",
        "node scripts/test_semantic_contract_adoption.mjs",
        "BASE_SHA: ${{ github.event.pull_request.base.sha }}",
        "npm run check:json-contracts",
        "rm -rf node_modules",
        "rm -rf target",
        "git ls-files --error-unmatch package-lock.json",
    )
    missing = [fragment for fragment in required_fragments if fragment not in workflow]
    if missing:
        raise SystemExit(
            "Product CI contract missing required fragment(s): " + ", ".join(missing)
        )

    for step_name in RUST_GUARDED_STEPS:
        block = _workflow_step_block(workflow, step_name)
        if RUST_ADOPTION_GUARD not in block:
            raise SystemExit(
                f"Product CI Rust step must be staged by workspace adoption: {step_name}"
            )

    _require_step_order(workflow, SEMANTIC_STEP_ORDER)

    if not COVERAGE_SCRIPT_PATH.is_file():
        raise SystemExit("Product CI requires tracked scripts/check_coverage.sh")

    if not RUST_ADOPTION_CHECKER_PATH.is_file():
        raise SystemExit(
            "Product CI requires tracked scripts/check_rust_workspace_adoption.py"
        )

    if not RUST_ADOPTION_TEST_PATH.is_file():
        raise SystemExit(
            "Product CI requires tracked scripts/test_rust_workspace_adoption.py"
        )

    rust_adoption_checker = RUST_ADOPTION_CHECKER_PATH.read_text(encoding="utf-8")
    rust_adoption_required_fragments = (
        'os.environ.get("BASE_SHA")',
        "cat-file",
        "Cargo.toml",
        "Cargo.lock",
        "not_adopted",
        "rust_workspace_incomplete",
        "adopted=true",
        "adopted=false",
    )
    rust_adoption_missing = [
        fragment
        for fragment in rust_adoption_required_fragments
        if fragment not in rust_adoption_checker
    ]
    if rust_adoption_missing:
        raise SystemExit(
            "Rust workspace adoption checker missing staged-adoption fragment(s): "
            + ", ".join(rust_adoption_missing)
        )

    if not SEMANTIC_CHECKER_PATH.is_file():
        raise SystemExit(
            "Product CI requires tracked scripts/check_semantic_candidate_contracts.mjs"
        )

    if not SEMANTIC_ADOPTION_TEST_PATH.is_file():
        raise SystemExit(
            "Product CI requires tracked scripts/test_semantic_contract_adoption.mjs"
        )

    semantic_checker = SEMANTIC_CHECKER_PATH.read_text(encoding="utf-8")
    semantic_required_fragments = (
        "process.env.BASE_SHA",
        "git",
        "cat-file",
        "not_adopted",
        "semantic_contract_incomplete",
    )
    semantic_missing = [
        fragment
        for fragment in semantic_required_fragments
        if fragment not in semantic_checker
    ]
    if semantic_missing:
        raise SystemExit(
            "Semantic contract checker missing staged-adoption fragment(s): "
            + ", ".join(semantic_missing)
        )

    if not PACKAGE_JSON_PATH.is_file() or not PACKAGE_LOCK_PATH.is_file():
        raise SystemExit("Product CI requires tracked package.json and package-lock.json")

    forbidden_fragments = (
        "runs-on: ubuntu-latest",
        "github.event.pull_request.number || github.ref",
        "github.event.pull_request.draft == false",
        "group: ${{ github.workflow }}-${{ github.repository }}-${{ github.event_name == 'pull_request' && github.event.pull_request.number || github.run_id }}\n",
        "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
        "npx --yes",
        "uses: actions/setup-node@v",
        "node-version: lts/",
        "node-version: latest",
        "RUSTUP_TOOLCHAIN: stable",
        "RUSTUP_TOOLCHAIN: 1.98.0",
        "rustup default stable",
        "./scripts/check_coverage.sh",
    )
    forbidden = [fragment for fragment in forbidden_fragments if fragment in workflow]
    if forbidden:
        raise SystemExit(
            "Product CI contract contains forbidden fragment(s): " + ", ".join(forbidden)
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
