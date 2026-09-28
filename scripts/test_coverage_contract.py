"""Check LLVM source ownership and fail-closed coverage admission."""

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
    {"name": "_RNvCowner20internal_model_tests7fixture", "filenames": [source],
     "branches": [[25, 2, 25, 8, 1, 0, 0, 0, 4]]},
    {"name": "_RNvCforeign7capture", "filenames": [foreign], "branches": [[30, 2, 30, 8, 0, 0, 0, 0, 4]]},
]
report = {"data": [{"functions": functions, "files": [
    {"filename": source, "branches": [[10, 2, 10, 8, 3, 0, 0, 0, 4], [20, 2, 20, 8, 1, 0, 0, 0, 4]]}
]}]}
filter_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).with_name("owned_source_branches.jq")


def extract(data, path=filter_path):
    result = subprocess.run(["jq", "--arg", "root", root, "-f", str(path)],
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

function_filter = Path(__file__).with_name("owned_source_functions.jq")
function_rows = [
    {"name": "capture", "count": 1, "filenames": [source, foreign],
     "regions": [[10, 1, 12, 1, 1, 0, 0, 0]]},
    {"name": "capture", "count": 2, "filenames": [source, foreign],
     "regions": [[10, 1, 12, 1, 2, 0, 0, 0]]},
    {"name": "capture_instance", "count": 0, "filenames": [source],
     "regions": [[13, 1, 15, 1, 0, 0, 0, 0]]},
    {"name": "_RNvCowner5tests7fixture", "count": 0, "filenames": [source],
     "regions": [[20, 1, 22, 1, 0, 0, 0, 0]]},
    {"name": "_RNvCowner20internal_model_tests7fixture", "count": 1, "filenames": [source],
     "regions": [[25, 1, 27, 1, 1, 0, 0, 0]]},
    {"name": "foreign", "count": 0, "filenames": [foreign, source],
     "regions": [[30, 1, 32, 1, 0, 0, 0, 0], [31, 1, 31, 8, 0, 1, 0, 0]]},
]
region_rows = extract({"data": [{"functions": function_rows}]},
                      Path(__file__).with_name("owned_source_regions.jq"))
assert any(row["file"] == source and row["line_start"] == 10 for row in region_rows)
assert all(row["line_start"] not in {20, 25} for row in region_rows)
rows = extract({"data": [{"functions": function_rows}]}, function_filter)
assert rows == [{"file": source, "line_start": 10, "column_start": 1, "count": 3},
                {"file": source, "line_start": 13, "column_start": 1, "count": 0}]
# A different instantiation at the same definition must share its execution count.
instantiation = dict(function_rows[1], name="capture_generic", count=0)
assert extract({"data": [{"functions": [function_rows[0], instantiation]}]},
               function_filter) == [dict(rows[0], count=1)]
# LLVM excludes expanded files when choosing the definition's main file.
expanded = {"name": "capture_macro", "count": 1, "filenames": [foreign, source],
            "regions": [[9, 1, 9, 5, 1, 1, 0, 1],
                        [10, 1, 12, 1, 1, 1, 0, 0]]}
assert extract({"data": [{"functions": [expanded]}]}, function_filter) == [dict(rows[0], line_start=9, count=1)]
function_gate = "length > 0 and all(.[]; .count > 0)"
for rows, accepted in [(rows, False), ([dict(row, count=1) for row in rows], True),
                       ([], False), ([dict(rows[0], count=-1)], False)]:
    result = subprocess.run(["jq", "-e", function_gate], input=json.dumps(rows),
                            text=True, capture_output=True)
    assert (result.returncode == 0) == accepted, "all owned source functions must execute"
assert extract({"data": [{"functions": function_rows[3:]}]}, function_filter) == []
print("owned source functions and 100% admission contract pass")
