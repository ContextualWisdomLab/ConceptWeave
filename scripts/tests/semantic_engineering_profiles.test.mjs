import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { test } from "node:test";
import { fileURLToPath } from "node:url";
import { sha256Digest, verifyManifestSourceProvenance } from "../verify_semantic_engineering_provenance.mjs";

const root = new URL("../../", import.meta.url);
const repositoryRoot = fileURLToPath(root);
const read = path => readFileSync(new URL(path, root));
const manifest = JSON.parse(read("profiles/semantic_engineering/profile_manifest.json"));
const records = manifest.profiles.map(item => ({...item, payload: JSON.parse(read(item.path))}));

// Artifact assertions only: these tests do not implement the Rust admission boundary.
test("object-level semantic authoring and meta-level procedure refinement stay separate", () => {
  assert.deepEqual(records.map(row => row.model_id), ["conceptweave_semantic_authoring", "conceptweave_procedural_refinement"]);
  assert.equal(new Set(records.map(row => row.task_type)).size, 2);
  assert.equal(manifest.loop_separation.self_approval_permitted, false);
  assert.equal(manifest.loop_separation.object_level, records[0].model_id);
  assert.equal(manifest.loop_separation.meta_level, records[1].model_id);
});
test("all checked-in profiles are inferred drafts with activation disabled", () => {
  assert.equal(manifest.runtime_activation, false);
  assert.equal(manifest.source_authentication, "not_established_by_manifest");
  for (const {payload} of records) {
    assert.equal(payload.publication_state, "draft");
    assert.equal(payload.truth_status, "inferred");
    assert.equal(payload.domain_owner_ref, "ContextualWisdomLab/ConceptWeave");
    assert.equal(Object.hasOwn(payload, "activation_authorized"), false);
    assert.ok(payload.procedure_nodes.every(node => node.procedure_kind === "skill_procedure"));
  }
});
test("profile paths and recorded content digests match the exact file bytes", () => {
  assert.deepEqual(records.map(row => row.path), ["profiles/semantic_engineering/semantic_authoring.draft.json", "profiles/semantic_engineering/procedural_refinement.draft.json"]);
  for (const record of records) {
    assert.equal(record.content_digest, `sha256:${createHash("sha256").update(read(record.path)).digest("hex")}`);
    assert.equal(record.procedure_count, record.payload.procedure_nodes.length);
    assert.equal(record.relation_count, record.payload.procedure_edges.length);
  }
});
test("references name pinned source coordinates without claiming operational truth", () => {
  for (const source of manifest.sources) {
    assert.equal(source.repository, "ContextualWisdomLab/ConceptWeave");
    assert.match(source.commit_sha, /^[0-9a-f]{40}$/);
    assert.match(source.git_blob_sha, /^[0-9a-f]{40}$/);
    assert.match(source.content_digest, /^sha256:[0-9a-f]{64}$/);
    assert.equal(source.source_status, "proposed_design_not_operational_evidence");
  }
  for (const {payload} of records) {
    for (const ref of payload.source_evidence) {
      const source = manifest.sources.find(item => item.source_id === ref.source_id);
      assert.ok(source);
      assert.equal(ref.source_digest, source.content_digest);
      assert.ok(ref.location.startsWith(`${source.path}::`));
    }
  }
});
test("manifest source coordinates resolve to the exact immutable Git bytes", () => {
  const result = verifyManifestSourceProvenance(repositoryRoot, manifest);
  assert.equal(result.sources_verified, manifest.sources.length);
  assert.ok(result.source_bytes_verified > 0);
  assert.equal(result.verification_scope, "local_git_commit_path_blob_and_sha256");
  assert.equal(result.source_authentication_established, true);
});
test("coordinated digest tampering cannot self-authorize source provenance", () => {
  const tamperedManifest = structuredClone(manifest);
  const fakeDigest = sha256Digest(Buffer.from("coordinated-source-tamper"));
  tamperedManifest.sources[0].content_digest = fakeDigest;
  const tamperedPayloads = records.map(({payload}) => structuredClone(payload));
  let rewrittenReferences = 0;
  tamperedPayloads.forEach((payload, index) => {
    for (const ref of payload.source_evidence) {
      if (ref.source_id === tamperedManifest.sources[0].source_id) {
        ref.source_digest = fakeDigest;
        rewrittenReferences += 1;
      }
    }
    const bytes = Buffer.from(JSON.stringify(payload));
    tamperedManifest.profiles[index].content_digest = sha256Digest(bytes);
    assert.equal(tamperedManifest.profiles[index].content_digest, sha256Digest(bytes));
  });
  assert.ok(rewrittenReferences > 0);
  assert.throws(
    () => verifyManifestSourceProvenance(repositoryRoot, tamperedManifest),
    /source_content_digest_mismatch:conceptweave_prd/,
  );
});
test("source provenance fails closed on malformed, missing, oversized and duplicate coordinates", () => {
  const duplicate = structuredClone(manifest);
  duplicate.sources[1].source_id = duplicate.sources[0].source_id;
  assert.throws(() => verifyManifestSourceProvenance(repositoryRoot, duplicate), /source_id_invalid_or_duplicate/);

  const malformedBlob = structuredClone(manifest);
  malformedBlob.sources[0].git_blob_sha = "not-a-blob";
  assert.throws(() => verifyManifestSourceProvenance(repositoryRoot, malformedBlob), /source_git_blob_invalid:conceptweave_prd/);

  const missingCommit = structuredClone(manifest);
  missingCommit.sources[0].commit_sha = "0".repeat(40);
  assert.throws(() => verifyManifestSourceProvenance(repositoryRoot, missingCommit), /source_commit_unavailable:conceptweave_prd/);

  const missingPath = structuredClone(manifest);
  missingPath.sources[0].path = "docs/does-not-exist.md";
  assert.throws(() => verifyManifestSourceProvenance(repositoryRoot, missingPath), /source_path_unavailable:conceptweave_prd/);

  assert.throws(
    () => verifyManifestSourceProvenance(repositoryRoot, manifest, {maxSourceBytes: 1}),
    /source_size_limit_exceeded:conceptweave_prd/,
  );
});
test("authoring profile retains the seven canonical phases and a correction path", () => {
  const authoring = records[0].payload;
  assert.deepEqual(authoring.procedure_nodes.map(row => row.procedure_id), ["observe_source_evidence", "discover_semantic_candidates", "propose_semantic_model", "align_domain_concepts", "validate_semantic_model", "request_steward_review", "publish_approved_release", "revise_model_candidate"]);
  const edges = authoring.procedure_edges;
  assert.ok(edges.some(edge => edge.source_procedure_id === "validate_semantic_model" && edge.target_procedure_id === "revise_model_candidate"));
  assert.ok(edges.some(edge => edge.source_procedure_id === "revise_model_candidate" && edge.target_procedure_id === "align_domain_concepts"));
  assert.deepEqual(edges.filter(edge => edge.target_procedure_id === "publish_approved_release").map(edge => edge.source_procedure_id), ["request_steward_review"]);
});
test("refinement profile explicitly sequences independent evaluation and review", () => {
  const edges = records[1].payload.procedure_edges;
  assert.deepEqual(edges.filter(edge => edge.target_procedure_id === "request_revision_review").map(edge => edge.source_procedure_id), ["request_independent_evaluation"]);
  assert.deepEqual(edges.filter(edge => edge.target_procedure_id === "publish_procedural_successor").map(edge => edge.source_procedure_id), ["request_revision_review"]);
  assert.deepEqual(edges.filter(edge => edge.source_procedure_id === "retain_rejected_revision").map(edge => edge.target_procedure_id), ["contrast_observed_outcomes"]);
});
test("training/refinement profiles contain no operational tool or evaluator payload", () => {
  const forbidden = new Set(["tool_contract_ref", "raw_trajectory", "validation_scores", "holdout_answers", "final_test_answers", "provider_key"]);
  const inspect = value => {
    if (value === null || typeof value !== "object") return;
    for (const [key, child] of Object.entries(value)) { assert.ok(!forbidden.has(key)); inspect(child); }
  };
  for (const {payload} of records) inspect(payload);
});
test("checked-in profiles have complete local references and intentional cycles", () => {
  for (const {payload} of records) {
    const ids = new Set(payload.procedure_nodes.map(node => node.procedure_id));
    assert.equal(ids.size, payload.procedure_nodes.length);
    assert.ok(ids.has(payload.entry_procedure_id));
    const triples = new Set();
    const evidence = new Set(payload.source_evidence.map(row => JSON.stringify(row)));
    for (const item of [...payload.procedure_nodes, ...payload.procedure_edges]) {
      assert.ok(item.source_evidence.every(row => evidence.has(JSON.stringify(row))));
    }
    for (const edge of payload.procedure_edges) {
      assert.ok(ids.has(edge.source_procedure_id) && ids.has(edge.target_procedure_id));
      assert.equal(edge.relation_type, "leads_to");
      triples.add(JSON.stringify([edge.source_procedure_id, edge.relation_type, edge.target_procedure_id]));
    }
    assert.equal(triples.size, payload.procedure_edges.length);
  }
});
test("locale state is authoring-only rather than a claim of eight-language validation", () => {
  assert.equal(manifest.label_completeness, "ko_en_authoring_only");
  for (const {payload} of records) {
    for (const node of payload.procedure_nodes) assert.deepEqual(Object.keys(node.locale_labels), ["ko", "en"]);
  }
});
