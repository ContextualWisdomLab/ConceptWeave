import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { test } from "node:test";
import {
  materializeContractCases,
  createProceduralValidators,
  validateContractCases,
} from "../check_procedural_contracts.mjs";

const repositoryRoot = fileURLToPath(new URL("../../", import.meta.url));
const readFixture = name => JSON.parse(readFileSync(resolve(repositoryRoot, "contracts/fixtures", name), "utf8"));
const fixtureBases = {model: readFixture("procedural-model.base.json"), revision: readFixture("procedural-revision.base.json")};
const caseManifest = readFixture("procedural-authoring.cases.json");

// These tests cover the fixture protocol and real in-process JSON Schema validation only.
test("materializes every registered case without changing retained fixture bytes", () => {
  const beforeBytes = JSON.stringify({fixtureBases, caseManifest});
  const outputCases = materializeContractCases(caseManifest, fixtureBases);
  assert.equal(outputCases.length, 46);
  assert.equal(outputCases.filter(item => !item.shape_valid).length, 33);
  assert.equal(outputCases.filter(item => item.semantic_expectation !== "not_evaluated").length, 4);
  assert.equal(JSON.stringify({fixtureBases, caseManifest}), beforeBytes);
  outputCases[0].payload.procedure_nodes[0].locale_labels.en = "Modified test instance";
  assert.equal(fixtureBases.model.procedure_nodes[0].locale_labels.en, "Read review evidence");
});

test("keeps semantic gap witnesses distinct from ordinary shape-positive inputs", () => {
  const outputCases = materializeContractCases(caseManifest, fixtureBases);
  const gapCase = outputCases.find(item => item.case_name === "known_gap_dangling_endpoint");
  assert.equal(gapCase.shape_valid, true);
  assert.equal(gapCase.semantic_expectation, "must_reject_dangling_endpoint");
  assert.notEqual(gapCase.payload.procedure_edges[0].target_procedure_id, fixtureBases.model.procedure_edges[0].target_procedure_id);
});

for (const [caseName, mutation] of [
  ["wrong scope", value => {value.validation_scope = "runtime_approval";}],
  ["empty corpus", value => {value.case_rows = [];}],
  ["unknown base", value => {value.case_rows[0].base_name = "../secret";}],
  ["duplicate name", value => {value.case_rows[1].case_name = value.case_rows[0].case_name;}],
  ["unsafe filename", value => {value.case_rows[0].case_name = "../outside";}],
  ["nonboolean expectation", value => {value.case_rows[0].shape_valid = "true";}],
  ["missing gap disposition", value => {delete value.case_rows[0].semantic_expectation;}],
  ["unknown operation", value => {value.case_rows[0].changes = [{operation: "execute", field_path: ["model_id"]}];}],
  ["empty field path", value => {value.case_rows[0].changes = [{operation: "remove", field_path: []}];}],
  ["prototype traversal", value => {value.case_rows[0].changes = [{operation: "set", field_path: ["__proto__", "polluted"], field_value: true}];}],
  ["absent intermediate", value => {value.case_rows[0].changes = [{operation: "set", field_path: ["absent", "child"], field_value: true}];}],
  ["absent removal", value => {value.case_rows[0].changes = [{operation: "remove", field_path: ["absent"]}];}],
  ["missing replacement", value => {value.case_rows[0].changes = [{operation: "set", field_path: ["model_id"]}];}],
]) test(`rejects invalid fixture protocol: ${caseName}`, () => {
  const changedManifest = structuredClone(caseManifest);
  mutation(changedManifest);
  assert.throws(() => materializeContractCases(changedManifest, fixtureBases), /invalid_fixture_protocol/);
});

test("validates all shape cases in process through the locked AJV library", () => {
  const outputCases = materializeContractCases(caseManifest, fixtureBases);
  const summary = validateContractCases(createProceduralValidators(repositoryRoot), outputCases);
  assert.equal(summary.checked_cases, 46);
  assert.equal(summary.semantic_gap_witnesses, 4);
  assert.equal(summary.publication_authorized, false);
  assert.equal(summary.activation_authorized, false);
});

test("keeps JSON Schema validation off dynamic package execution paths", () => {
  const packageManifest = JSON.parse(readFileSync(resolve(repositoryRoot, "package.json"), "utf8"));
  const packageLock = JSON.parse(readFileSync(resolve(repositoryRoot, "package-lock.json"), "utf8"));
  const workflow = readFileSync(resolve(repositoryRoot, ".github/workflows/product.yml"), "utf8");
  const checker = readFileSync(resolve(repositoryRoot, "scripts/check_procedural_contracts.mjs"), "utf8");
  assert.equal(packageManifest.devDependencies.ajv, "8.20.0");
  assert.equal(packageLock.packages["node_modules/ajv"].version, "8.20.0");
  assert.match(workflow, /npm ci --ignore-scripts --no-audit --no-fund/);
  assert.doesNotMatch(workflow, /\bnpx\b|ajv-cli/);
  assert.doesNotMatch(checker, /spawnSync|\bnpx\b|ajv-cli/);
});
