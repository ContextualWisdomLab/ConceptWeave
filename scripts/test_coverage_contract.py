"""Check LLVM source-branch ownership and fail-closed coverage admission."""

import json
from pathlib import Path
import subprocess
import sys

root = "/fixture/crates/"
source = root + "owner/src/lib.rs"
foreign = "/dependency/src/lib.rs"
branch = [10, 2, 10, 8, 1, 0, 1, 0, 4]
test_branch = [20, 2, 20, 8, 1, 0, 1, 0, 4]
functions = [
    {"name": "_RNvCowner7capture", "filenames": [foreign, source], "branches": [branch]},
    {"name": "_RNvCowner7capture_instance", "filenames": [source], "branches": [[10, 2, 10, 8, 2, 0, 0, 0, 4]]},
    {"name": "_RNvCowner5tests7fixture", "filenames": [foreign, source], "branches": [test_branch]},
    {"name": "_RNvCforeign7capture", "filenames": [foreign], "branches": [[30, 2, 30, 8, 0, 0, 0, 0, 4]]},
]
report = {"data": [{"functions": functions, "files": [
    {"filename": source, "branches": [[10, 2, 10, 8, 3, 0, 0, 0, 4], [20, 2, 20, 8, 1, 0, 0, 0, 4]]}
]}]}
filter_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).with_name("owned_source_branches.jq")


def extract(data):
    result = subprocess.run(["jq", "--arg", "root", root, "-f", str(filter_path)],
                            input=json.dumps(data), text=True, capture_output=True, check=True)
    return json.loads(result.stdout)


rows = extract(report)
assert len(rows) == 1, "test and dependency branches must not count as owned production"
assert rows[0]["file"] == source and rows[0]["line_start"] == 10
assert (rows[0]["true_count"], rows[0]["false_count"]) == (3, 0)
gate = "length > 0 and all(.[]; .true_count > 0 and .false_count > 0)"
for rows, accepted in [(rows, False), ([dict(rows[0], true_count=0, false_count=1)], False),
                       ([dict(rows[0], false_count=1)], True), ([], False)]:
    result = subprocess.run(["jq", "-e", gate], input=json.dumps(rows), text=True, capture_output=True)
    assert (result.returncode == 0) == accepted, "missing production arms or inventory must fail"
assert extract({"data": [{"functions": functions[2:3], "files": []}]}) == []
print("owned branch ownership and 100% admission contract pass")
