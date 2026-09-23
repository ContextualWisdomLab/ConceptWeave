import {copyFileSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join, resolve} from "node:path";
import {fileURLToPath} from "node:url";
import {spawnSync} from "node:child_process";

const repositoryRoot = fileURLToPath(new URL("../", import.meta.url));
const sourceChecker = resolve(repositoryRoot, "scripts/check_semantic_candidate_contracts.mjs");
const sourceNodeModules = resolve(repositoryRoot, "node_modules");
const invalidFixtureNames = [
  "semantic-candidate.invalid-whitespace.json",
  "semantic-candidate.invalid-published-truth.json",
  "semantic-candidate.invalid-state-truth-mismatch.json",
];

const run = (cwd, command, args, extraEnv = {}) =>
  spawnSync(command, args, {
    cwd,
    encoding: "utf8",
    env: {...process.env, ...extraEnv},
  });

const requireSuccess = (result, context) => {
  if (result.status !== 0) {
    throw new Error(`${context}: ${result.stderr || result.stdout}`);
  }
};

const git = (cwd, ...args) => {
  const result = run(cwd, "git", args);
  requireSuccess(result, `git ${args.join(" ")}`);
  return result.stdout.trim();
};

const initializeRepository = root => {
  mkdirSync(join(root, "scripts"), {recursive: true});
  copyFileSync(sourceChecker, join(root, "scripts/check_semantic_candidate_contracts.mjs"));
  symlinkSync(sourceNodeModules, join(root, "node_modules"), "dir");
  git(root, "init", "-q");
  git(root, "config", "user.email", "conceptweave-ci@example.invalid");
  git(root, "config", "user.name", "ConceptWeave CI");
};

const commit = (root, message, allowEmpty = false) => {
  git(root, "add", "-A");
  const args = ["commit", "-q", "-m", message];
  if (allowEmpty) {
    args.splice(1, 0, "--allow-empty");
  }
  git(root, ...args);
  return git(root, "rev-parse", "HEAD");
};

const writeCompleteContract = root => {
  const contracts = join(root, "contracts");
  const fixtures = join(contracts, "fixtures");
  mkdirSync(fixtures, {recursive: true});
  const schema = {
    $schema: "https://json-schema.org/draft/2020-12/schema",
    $id: "https://contextualwisdomlab.invalid/semantic-candidate.schema.json",
    type: "object",
    properties: {allowed: {const: true}},
    required: ["allowed"],
    additionalProperties: false,
  };
  writeFileSync(
    join(contracts, "semantic-candidate.schema.json"),
    `${JSON.stringify(schema, null, 2)}\n`,
  );
  writeFileSync(
    join(fixtures, "semantic-candidate.valid.json"),
    `${JSON.stringify({allowed: true})}\n`,
  );
  for (const fixtureName of invalidFixtureNames) {
    writeFileSync(
      join(fixtures, fixtureName),
      `${JSON.stringify({allowed: false})}\n`,
    );
  }
};

const runChecker = (root, baseSha, extraEnv = {}) =>
  run(
    root,
    process.execPath,
    [join(root, "scripts/check_semantic_candidate_contracts.mjs")],
    {BASE_SHA: baseSha, ...extraEnv},
  );

const expect = (condition, message) => {
  if (!condition) {
    throw new Error(message);
  }
};

const withRepository = callback => {
  const root = mkdtempSync(join(tmpdir(), "conceptweave-semantic-adoption-"));
  try {
    initializeRepository(root);
    callback(root);
  } finally {
    rmSync(root, {recursive: true, force: true});
  }
};

const cases = [
  ["not adopted", root => {
    const baseSha = commit(root, "empty base", true);
    const result = runChecker(root, baseSha);
    expect(result.status === 0, `not-adopted must succeed: ${result.stderr}`);
    expect(result.stdout.includes('"status":"not_adopted"'), "not-adopted status missing");
  }],
  ["first complete adoption", root => {
    const baseSha = commit(root, "empty base", true);
    writeCompleteContract(root);
    const result = runChecker(root, baseSha);
    expect(result.status === 0, `first adoption must validate: ${result.stderr}`);
    expect(result.stdout.includes('"status":"validated"'), "validated status missing");
  }],
  ["explicit candidate root", root => {
    const baseSha = commit(root, "empty base", true);
    writeCompleteContract(root);
    const result = runChecker(root, baseSha, {PRODUCT_CANDIDATE_ROOT: root});
    expect(result.status === 0, `explicit candidate root must validate: ${result.stderr}`);
    expect(result.stdout.includes('"status":"validated"'), "candidate-root status missing");
  }],
  ["partial adoption", root => {
    const baseSha = commit(root, "empty base", true);
    mkdirSync(join(root, "contracts"), {recursive: true});
    writeFileSync(
      join(root, "contracts/semantic-candidate.schema.json"),
      `${JSON.stringify({type: "object"})}\n`,
    );
    const result = runChecker(root, baseSha);
    expect(result.status !== 0, "partial adoption must fail");
    expect(result.stderr.includes("semantic_contract_incomplete"), "partial-adoption reason missing");
  }],
  ["deletion after adoption", root => {
    writeCompleteContract(root);
    const baseSha = commit(root, "adopt contract");
    rmSync(join(root, "contracts/fixtures/semantic-candidate.invalid-whitespace.json"));
    const result = runChecker(root, baseSha);
    expect(result.status !== 0, "post-adoption deletion must fail");
    expect(result.stderr.includes("semantic_contract_incomplete"), "deletion reason missing");
  }],
  ["invalid base SHA", root => {
    commit(root, "empty base", true);
    const result = runChecker(root, "main");
    expect(result.status !== 0, "invalid base SHA must fail");
    expect(result.stderr.includes("semantic_contract_base_sha_invalid"), "invalid-SHA reason missing");
  }],
  ["unavailable base commit", root => {
    commit(root, "empty base", true);
    const result = runChecker(root, "0".repeat(40));
    expect(result.status !== 0, "unavailable base commit must fail");
    expect(result.stderr.includes("semantic_contract_base_commit_unavailable"), "missing-base reason missing");
  }],
];

for (const [name, execute] of cases) {
  withRepository(execute);
  console.log(`PASS ${name}`);
}
