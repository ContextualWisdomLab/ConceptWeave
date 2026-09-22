#!/usr/bin/env python3
"""Apply language-neutral semantic rules to a supersession JSON contract."""

from __future__ import annotations

import json
import sys
from pathlib import Path
from typing import Any


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]
RULES_PATH = REPOSITORY_ROOT / "contracts" / "semantic-release-supersession.rules.json"


def _pointer(document: Any, pointer: str) -> Any:
    value = document
    for token in pointer.removeprefix("/").split("/"):
        if not token:
            continue
        token = token.replace("~1", "/").replace("~0", "~")
        if not isinstance(value, dict) or token not in value:
            raise ValueError(f"missing semantic-rule coordinate: {pointer}")
        value = value[token]
    return value


def validate_semantic_release_supersession(document: Any, rules: Any) -> list[str]:
    """Return deterministic semantic-rule violations for one public contract."""
    violations: list[str] = []
    for rule in rules.get("rules", []):
        if rule.get("operator") != "not_equal":
            violations.append(f"unsupported semantic rule operator: {rule.get('operator')!r}")
            continue
        left = _pointer(document, rule["left"])
        right = _pointer(document, rule["right"])
        if left == right:
            violations.append(rule["id"])
    return violations


def main(argv: list[str]) -> int:
    if len(argv) != 2:
        print("usage: validate_semantic_release_supersession.py CONTRACT.json", file=sys.stderr)
        return 2

    contract_path = Path(argv[1])
    try:
        document = json.loads(contract_path.read_text(encoding="utf-8"))
        rules = json.loads(RULES_PATH.read_text(encoding="utf-8"))
        violations = validate_semantic_release_supersession(document, rules)
    except (OSError, json.JSONDecodeError, KeyError, TypeError, ValueError) as error:
        print(f"semantic supersession validation failed closed: {error}", file=sys.stderr)
        return 2

    if violations:
        for violation in violations:
            print(f"semantic supersession violation: {violation}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
