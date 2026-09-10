import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const repositoryRoot = fileURLToPath(new URL("../", import.meta.url));
const readJson = path => JSON.parse(readFileSync(resolve(repositoryRoot, path), "utf8"));
const schema = readJson("contracts/semantic-candidate.schema.json");
const validator = new Ajv2020({allErrors: true, strict: true}).compile(schema);

const cases = [
  ["contracts/fixtures/semantic-candidate.valid.json", true],
  ["contracts/fixtures/semantic-candidate.invalid-whitespace.json", false],
  ["contracts/fixtures/semantic-candidate.invalid-published-truth.json", false],
  ["contracts/fixtures/semantic-candidate.invalid-state-truth-mismatch.json", false],
];

for (const [fixturePath, expectedValid] of cases) {
  const actualValid = validator(readJson(fixturePath));
  if (actualValid !== expectedValid) {
    const details = validator.errors ? JSON.stringify(validator.errors) : "[]";
    throw new Error(`semantic_candidate_contract_validation_failed:${fixturePath}:${details}`);
  }
}

console.log(JSON.stringify({checked_cases: cases.length, schema_id: schema.$id}));
