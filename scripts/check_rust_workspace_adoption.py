"""Stage Rust Product checks on the exact pull-request base workspace contract."""

from __future__ import annotations

import os
import re
import subprocess
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parent.parent
REQUIRED_WORKSPACE_PATHS = ("Cargo.toml", "Cargo.lock")


def _git_object_exists(object_name: str) -> bool:
    """Return whether an exact Git object/path exists without reading mutable refs."""
    result = subprocess.run(
        ["git", "cat-file", "-e", object_name],
        cwd=REPOSITORY_ROOT,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        check=False,
    )
    return result.returncode == 0


def main() -> int:
    """Emit a GitHub Actions output for exact-base Rust workspace adoption."""
    base_sha = os.environ.get("BASE_SHA")
    if not base_sha or re.fullmatch(r"[0-9a-f]{40}", base_sha) is None:
        raise SystemExit("rust_workspace_base_sha_invalid")

    if not _git_object_exists(f"{base_sha}^{{commit}}"):
        raise SystemExit("rust_workspace_base_commit_unavailable")

    base_has_workspace_path = any(
        _git_object_exists(f"{base_sha}:{path}") for path in REQUIRED_WORKSPACE_PATHS
    )
    head_has_workspace_path = any(
        (REPOSITORY_ROOT / path).exists() for path in REQUIRED_WORKSPACE_PATHS
    )

    if not base_has_workspace_path and not head_has_workspace_path:
        print("adopted=false")
        return 0

    missing_head_paths = [
        path
        for path in REQUIRED_WORKSPACE_PATHS
        if not (REPOSITORY_ROOT / path).is_file()
    ]
    if missing_head_paths:
        raise SystemExit(
            "rust_workspace_incomplete:" + ",".join(missing_head_paths)
        )

    print("adopted=true")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
