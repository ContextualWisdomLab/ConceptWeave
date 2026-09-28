"""Verify generated workspace SBOMs cover the exact Cargo.lock package set."""

import json
import tomllib
from pathlib import Path


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    workspace = tomllib.loads((root / "Cargo.toml").read_text())
    members = workspace["workspace"]["members"]
    locked = tomllib.loads((root / "Cargo.lock").read_text())["package"]
    expected = {(package["name"], package["version"]) for package in locked}
    if len(expected) != len(locked):
        raise SystemExit("ambiguous package name/version in Cargo.lock")

    files = sorted(root.glob("crates/**/*.cdx.json"))
    if len(files) != len(members):
        raise SystemExit("one SBOM per workspace crate is required")
    observed = set()
    for member in members:
        manifest = tomllib.loads((root / member / "Cargo.toml").read_text())
        name = manifest["package"]["name"]
        path = root / member / f"{name}.cdx.json"
        if path not in files:
            raise SystemExit(f"missing SBOM for {name}")
        bom = json.loads(path.read_text())
        if bom["bomFormat"] != "CycloneDX" or bom["specVersion"] != "1.5":
            raise SystemExit(f"invalid CycloneDX format for {name}")
        if bom["metadata"]["component"]["name"] != name:
            raise SystemExit(f"invalid root component for {name}")
        if bom["metadata"]["tools"][0]["name"] != "cargo-cyclonedx" or bom["metadata"]["tools"][0]["version"] != "0.5.9":
            raise SystemExit(f"unexpected SBOM generator for {name}")
        components = [bom["metadata"]["component"], *bom["components"]]
        observed.update((component["name"], component["version"]) for component in components)
    if observed != expected:
        raise SystemExit(f"SBOM/lock mismatch: missing={expected - observed}, extra={observed - expected}")
    print(f"{len(files)} SBOMs cover {len(expected)} locked packages")


if __name__ == "__main__":
    main()
