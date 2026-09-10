import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { test } from "node:test";
import { createProceduralValidators } from "../check_procedural_contracts.mjs";

const draft202012 = "https://json-schema.org/draft/2020-12/schema";

function writeSchema(root, filename, id, body = '"type":"object"') {
  writeFileSync(
    join(root, "contracts", filename),
    `{"$schema":"${draft202012}","$id":"${id}",${body}}`,
  );
}

test("repository schema JSON rejects duplicate decoded members before AJV", () => {
  const root = mkdtempSync(join(tmpdir(), "conceptweave-schema-json-"));
  try {
    mkdirSync(join(root, "contracts"));
    writeSchema(
      root,
      "semantic-candidate.schema.json",
      "urn:cwl:test:semantic-candidate",
      '"type":"object","type":"array"',
    );
    writeSchema(root, "procedural-model-draft.schema.json", "urn:cwl:test:procedural-model");
    writeSchema(root, "procedural-revision-proposal.schema.json", "urn:cwl:test:procedural-revision");

    assert.throws(
      () => createProceduralValidators(root),
      /duplicate_json_key:contracts\/semantic-candidate\.schema\.json:/,
    );
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
