import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";

const repositoryRoot = process.env.PRODUCT_CANDIDATE_ROOT
  ? resolve(process.env.PRODUCT_CANDIDATE_ROOT)
  : fileURLToPath(new URL("../", import.meta.url));
const readJson = path => JSON.parse(readFileSync(resolve(repositoryRoot, path), "utf8"));
const SEMANTIC_SCHEMA_PATH = "contracts/semantic-candidate.schema.json";
const cases = [
  ["contracts/fixtures/semantic-candidate.valid.json", true],
  ["contracts/fixtures/semantic-candidate.invalid-whitespace.json", false],
  ["contracts/fixtures/semantic-candidate.invalid-published-truth.json", false],
  ["contracts/fixtures/semantic-candidate.invalid-state-truth-mismatch.json", false],
];
const requiredContractPaths = [SEMANTIC_SCHEMA_PATH, ...cases.map(([path]) => path)];

const baseSha = process.env.BASE_SHA;
if (!baseSha || !/^[0-9a-f]{40}$/.test(baseSha)) {
  throw new Error("semantic_contract_base_sha_invalid");
}

const gitObjectExists = object => {
  const result = spawnSync("git", ["cat-file", "-e", object], {
    cwd: repositoryRoot,
    stdio: "ignore",
  });
  return result.status === 0;
};

if (!gitObjectExists(`${baseSha}^{commit}`)) {
  throw new Error("semantic_contract_base_commit_unavailable");
}

const baseHasContractPath = requiredContractPaths.some(path =>
  gitObjectExists(`${baseSha}:${path}`),
);
const headHasContractPath = requiredContractPaths.some(path =>
  existsSync(resolve(repositoryRoot, path)),
);

if (!baseHasContractPath && !headHasContractPath) {
  console.log(JSON.stringify({checked_cases: 0, status: "not_adopted"}));
  process.exit(0);
}

const missingHeadPaths = requiredContractPaths.filter(
  path => !existsSync(resolve(repositoryRoot, path)),
);
if (missingHeadPaths.length > 0) {
  throw new Error(`semantic_contract_incomplete:${missingHeadPaths.join(",")}`);
}

const schema = readJson(SEMANTIC_SCHEMA_PATH);
const validator = new Ajv2020({allErrors: true, strict: true}).compile(schema);

for (const [fixturePath, expectedValid] of cases) {
  const actualValid = validator(readJson(fixturePath));
  if (actualValid !== expectedValid) {
    const details = validator.errors ? JSON.stringify(validator.errors) : "[]";
    throw new Error(`semantic_candidate_contract_validation_failed:${fixturePath}:${details}`);
  }
}

console.log(JSON.stringify({checked_cases: cases.length, schema_id: schema.$id, status: "validated"}));
