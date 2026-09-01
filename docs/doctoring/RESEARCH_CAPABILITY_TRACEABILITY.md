# Research-to-capability traceability

Snapshot: 2026-09-01

This register turns the current Consensus search set into product decisions. A paper is not considered "used" merely because it appears in the bibliography: it must be tied to a bounded context, an accepted or rejected design implication, and an executable evaluation family. Publication metadata below follows the current Consensus record; preprints are not silently promoted to peer-reviewed evidence.

## Classification rules

- `generation`: Source Observation, Semantic Discovery, or LLM Proposal capabilities that create ontology/semantic-model candidates.
- `client`: Model Alignment or Client Consumption capabilities that match, resolve, compare, explain, or consume governed releases.
- `bridge`: connects generated ontology/schema mappings to a downstream virtual/semantic consumption model.
- `cross_cutting`: evaluation, versioning, governance, reproducibility, or human-validation evidence shared by both tracks.
- `adopt`: becomes a product/test requirement.
- `adapt`: informs the design but is constrained by ConceptWeave authority/security contracts.
- `research_only`: retained as evidence but not adopted as a production rule.

## Current research register

| Study | Class | Product implication | Decision | Implementation owner | Evaluation / test family | Limitation carried into ConceptWeave |
| --- | --- | --- | --- | --- | --- | --- |
| Shimizu & Hitzler (2024), *Accelerating Knowledge Graph and Ontology Engineering with Large Language Models* | generation / cross_cutting | Keep ontology engineering modular: modeling, extension, population, alignment, disambiguation are separate operations rather than one opaque prompt. | adopt | Semantic Discovery, LLM Proposal, Model Alignment | per-operation fixtures; orchestration receipts; module-boundary fitness | Consensus currently records an arXiv publication; use as architecture evidence, not sole production truth. |
| Trajanoska, Stojanov, & Trajanov (2023), *Enhancing Knowledge Graph Construction Using Large Language Models* | generation | Compare LLM-assisted entity/relation extraction and ontology proposals with deterministic/specialized baselines. | adapt | Semantic Discovery, LLM Proposal | entity/relation precision-recall-F1; KG/ontology relevance; abstention | Demonstration domain is not enterprise GRC; no automatic generalization of reported accuracy. |
| Val-Calvo et al. (2025), *OntoGenix* | generation | Treat dataset-to-ontology work as staged preprocessing -> planning -> building -> refinement -> mapping, with explicit failure on complex modeling. | adopt | Source Observation, Semantic Discovery, Model Validation | stage receipts; coherent-model fixtures; complex-model abstention | Human modeling remains stronger for complex cases; model output stays proposed. |
| Lo, Jiang, Li, & Jamnik (2024), *End-to-End Ontology Learning with Large Language Models* | generation | Include end-to-end taxonomy generation as a benchmark strategy, not as the sole architecture. Measure semantic and structural similarity. | adapt | Semantic Discovery, Evaluation | taxonomy-edge P/R/F1; graph structural similarity; domain-transfer fixture | Consensus currently records arXiv; fine-tuned end-to-end generation does not remove governance review. |
| Giglou, D'Souza, & Auer (2023), *LLMs4OL* | generation | Term typing, taxonomy discovery, and non-taxonomic relation extraction are first-class Generation tasks. | adopt | Semantic Discovery, LLM Proposal | LLMs4OL-style task fixtures across heterogeneous domains | Zero-shot results vary by domain; no model family becomes canonical product truth. |
| Giglou, D'Souza, & Auer (2024), *LLMs4OL 2024 Overview* | generation / cross_cutting | Preserve challenge-style standardized task definitions and reusable benchmark splits. | adopt | Evaluation | challenge-compatible term typing / taxonomy / relation suites | Challenge evidence measures tasks, not enterprise governance or source authority. |
| Phuttaamart, Kertkeidkachorn, & Trongratsameethong (2024), *The Ghost at LLMs4OL 2024 Task A* | generation | Track prompt/prompt-tuning sensitivity explicitly for term typing. | adapt | LLM Proposal, Evaluation | term-typing per-domain accuracy/F1; prompt sensitivity | GeoNames degradation is a concrete warning against aggregate-only scores. |
| Zhang et al. (2025), *OLIVE: Ontology Learning With Integrated Vector Embeddings* | generation | Vector/LLM workflows may assist relationship discovery and OWL drafting, but vectors remain candidate evidence rather than semantic authority. | adapt | Semantic Discovery, publication adapters | candidate quality; OWL syntax/shape validation | Prompt-driven retrieval and vector similarity cannot define truth status. |
| Lippolis et al. (2025), *Ontology Generation using Large Language Models* | generation / cross_cutting | Competency questions and user stories can drive ontology drafts; assess multiple structural criteria plus expert qualitative review. | adopt | LLM Proposal, Model Validation | competency-question coverage; structural criteria; expert/steward edit distance | Reported quality varies by model/prompt; generated OWL remains draft/proposed. |
| Giglou et al. (2026), *OntoLearner* | generation / cross_cutting | Add cross-domain standardized benchmarking and measure failure against ontology complexity, not just model size. | adopt | Evaluation | multi-domain term/taxonomy/relation benchmark; complexity-stratified error analysis | Tool/library is research infrastructure, not a required runtime dependency. |
| Hertling & Paulheim (2023), *OLaLa* | client | Ontology matching needs explicit prompt representation, examples, existing correspondences, and candidate-generation choices. | adopt | Model Alignment | OAEI-style matching P/R/F1; zero/few-shot comparison | LLM result is a correspondence candidate, never automatic authoritative alignment. |
| Giglou, D'Souza, Engel, & Auer (2024), *LLMs4OM* | client | Retrieve first, then match; compare concept-only, parent-context, and child-context representations. | adopt | Model Alignment, Client Consumption | retrieval recall; matching P/R/F1 across representation variants | Consensus currently records arXiv; client must remain functional without LLM matching. |
| Sousa, Lima, & Trojahn (2025), *Complex Ontology Matching with Large Language Model Embeddings* | client | Support expressive correspondence proposals using local subgraph/neighborhood evidence, not label similarity alone. | adapt | Model Alignment | complex-correspondence F1; subgraph ablations | Embedding-space/model compatibility must be explicit; reported gains do not authorize cross-model vector comparison. |
| Taboada et al. (2025), MILA | client | Use programmed retrieval/search to prune candidates and reserve LLM calls for uncertain cases. | adopt | Model Alignment, Client Consumption | candidate recall; final P/R/F1; LLM-call reduction vs naive prompting | Consensus currently records arXiv; algorithmic search cannot bypass evidence/truth-state rules. |
| Barcelos, French, & Wu (2025), *KROMA* | client | Targeted knowledge retrieval, structural context, and refinement should precede context-augmented LLM matching. | adopt | Model Alignment | candidate pruning recall; prompt-enrichment ablation; communication cost | Consensus currently records arXiv; RAG context is not source authority. |
| Qiang, Wang, & Taylor (2023), *Agent-OM* | client | Separate retrieval and matching responsibilities and expose bounded matching tools rather than one monolithic agent prompt. | adapt | Model Alignment, contextual-orchestrator ACL | OAEI simple/complex/few-shot tracks; tool-call receipts | Agent autonomy does not include release publication or business authorization. |
| Song, Chen, & Schmidt (2025), *GenOM* | client | Generated concept descriptions can enrich retrieval/matching, but exact lexical evidence remains a useful deterministic precision signal. | adopt | Model Alignment | OAEI Bio-ML; definition-quality criteria; retrieval/matching ablations | Biomedical results require enterprise-domain replication before broader claims. |
| Qiang & Taylor (2024), *OM4OV* | client / cross_cutting | Ontology/release version comparison needs explicit update-entity detection and explanations; do not equate versioning with ordinary matching. | adopt | Release Contract, Client Consumption | release-diff correctness; added/removed/changed entity detection; false-match explanation | Consensus currently records arXiv; ConceptWeave needs its own compatibility semantics. |
| Qiang et al. (2024), *OAEI-LLM* | client / cross_cutting | LLM-specific ontology-matching hallucinations require a dedicated benchmark dimension. | adopt | Evaluation | OAEI-LLM hallucination categories; abstention quality | Benchmark does not replace enterprise GRC golden fixtures. |
| Qiang et al. (2025), *OAEI-LLM-T* | client / cross_cutting | Add TBox/schema hallucination tests for matching and alignment. | adopt | Evaluation | TBox hallucination leaderboard/categories | Duplicate preprint/proceedings variants count as one study in this register. |
| Qiang, Wang, & Taylor (2026), *Crowd-OM* | cross_cutting | Human validation quality needs explicit trust/coherence/history controls when review scales beyond one steward. | adapt | Governance & Publication | inter-reviewer disagreement; coherence; adjudication receipts | Crowdsourcing is optional; domain-owner/steward authority remains product policy. |
| Qiang, Taylor, & Wang (2024), *How Does A Text Preprocessing Pipeline Affect Ontology Syntactic Matching?* | client | Keep deterministic tokenization/normalization as inspectable evidence; avoid assuming stopword/stemming pipelines always improve matching; LLM repair is secondary. | adopt | Model Alignment | OAEI preprocessing ablations; false-mapping regressions | No generic stopword/stemming heuristic is promoted to semantic truth. |
| Khalov & Ataeva (2025), *Automating Ontology Mapping in IT Service Management* | client | Lexical, embeddings, graph structure, and LLM signals may be compared as candidate features. | research_only | Model Alignment research adapter | feature ablation if reproduced | Reported validation uses an LLM surrogate expert and no annotated gold; it cannot ground production acceptance. |
| Xiao et al. (2025), *LLM4VKG* | bridge | Schema analysis + ontology development + mapping creation must flow into a stable downstream consumption contract and tolerate incomplete ontology inputs without inventing truth. | adopt | Generation↔Client seam | RODI-style mapping F1; incomplete-ontology fixtures; GRC round-trip | VKG execution remains in consuming systems; ConceptWeave does not become their query/database authority. |
| Li, Garijo, & Poveda-Villalón (2026), systematic literature review | cross_cutting | Standardize task definitions, datasets, metrics, prompt/model receipts, and human-expert review; disclose reproducibility gaps. | adopt | all bounded contexts / Evaluation | reproducibility manifest; benchmark disclosure; provider/prompt sensitivity | Literature reports heterogeneous protocols; no paper/prompt becomes a universal algorithm. |
| Du, An, Wang, & Liu (2024), ontology-learning review | cross_cutting | Keep shallow/deep/LLM methods comparable rather than treating LLMs as the only valid generation family. | research_only | Evaluation | baseline taxonomy of method families | Secondary review; the 2026 systematic review is the stronger cross-cutting evidence base. |

## GRC reference flow

`ContextualWisdomLab/governance-risk-compliance` is the first enterprise golden/reference scenario, not a special-case algorithm. The same immutable GRC fixture must exercise both tracks:

`GRC source contract -> observed facts -> generation candidates -> validation/steward review -> semantic_release -> client validation/resolution/diff/query-plan -> GRC deterministic calculation`.

Acceptance must prove that ConceptWeave never becomes the GRC system of record, that proposed/inferred relations do not mutate authoritative GRC records, that release validation works offline, and that release upgrades identify affected GRC queries explicitly. Public OAEI/RODI/LLMs4OL-style benchmarks remain necessary because one enterprise fixture cannot establish general matching or learning performance.

## Consensus records used in this snapshot

The following canonical Consensus records were fetched before recording product implications:

- https://consensus.app/papers/accelerating-knowledge-graph-and-ontology-engineering-shimizu-hitzler/82d868ee8f7953108246241e28d5e339/?utm_source=chatgpt
- https://consensus.app/papers/enhancing-knowledge-graph-construction-using-large-trajanoska-stojanov/80ffe83041735fdf94bf4b60dd32ba1a/?utm_source=chatgpt
- https://consensus.app/papers/ontogenix-leveraging-large-language-models-for-enhanced-val-calvo-aranguren/2c2771b0905a5292b6addb6b299bda17/?utm_source=chatgpt
- https://consensus.app/papers/endtoend-ontology-learning-with-large-language-models-lo-jiang/c3543c4051ac5bd7bf6932020e8d5120/?utm_source=chatgpt
- https://consensus.app/papers/llms4ol-large-language-models-for-ontology-learning-giglou-d’souza/971c6331c7cd5e24a3a547d5a938b40d/?utm_source=chatgpt
- https://consensus.app/papers/llms4ol-2024-overview-the-1st-large-language-models-for-giglou-d’souza/3ee443141c0d51bf9a9a8f2257070f04/?utm_source=chatgpt
- https://consensus.app/papers/the-ghost-at-llms4ol-2024-task-a-prompttuningbased-large-phuttaamart-kertkeidkachorn/9d0fce91b8ba550fa308ec889bd7056e/?utm_source=chatgpt
- https://consensus.app/papers/olive-ontology-learning-with-integrated-vector-zhang-dalal/1d96dab8b5c45b9fbbf19ecf9d39bc23/?utm_source=chatgpt
- https://consensus.app/papers/ontology-generation-using-large-language-models-lippolis-saeedizade/f3dd9e0944c253e3962b9ae9f4dc7867/?utm_source=chatgpt
- https://consensus.app/papers/ontolearner-a-modular-python-library-for-ontology-giglou-d’souza/63f55ff320b759d0a5e6e2b79fe4e37a/?utm_source=chatgpt
- https://consensus.app/papers/olala-ontology-matching-with-large-language-models-hertling-paulheim/53331022346755a49bdbb5455ae13b8c/?utm_source=chatgpt
- https://consensus.app/papers/llms4om-matching-ontologies-with-large-language-models-giglou-d’souza/a45561fbd0b25041a04df0f2fa49440b/?utm_source=chatgpt
- https://consensus.app/papers/complex-ontology-matching-with-large-language-model-sousa-lima/7244613a2f595e9d9ded3c8e62300d99/?utm_source=chatgpt
- https://consensus.app/papers/ontology-matching-with-large-language-models-and-taboada-martínez/7cff568231f455de89f26311b6be0d26/?utm_source=chatgpt
- https://consensus.app/papers/kroma-ontology-matching-with-knowledge-retrieval-and-barcelos-french/4669219e2e1c54ea8af442fbc690f922/?utm_source=chatgpt
- https://consensus.app/papers/agentom-leveraging-llm-agents-for-ontology-matching-qiang-wang/1ff1e2abb0f255299ecb808951ceaf6b/?utm_source=chatgpt
- https://consensus.app/papers/genom-ontology-matching-with-description-generation-and-song-chen/8587c3ae332a516d8426504b1f64447c/?utm_source=chatgpt
- https://consensus.app/papers/om4ov-leveraging-ontology-matching-for-ontology-qiang-taylor/bc311ce1e88c52a1ad9485037371b2e0/?utm_source=chatgpt
- https://consensus.app/papers/oaeillm-a-benchmark-dataset-for-understanding-large-qiang-taylor/e71db19036e651e69c2b5cee75d36935/?utm_source=chatgpt
- https://consensus.app/papers/oaeillmt-a-tbox-benchmark-dataset-for-understanding-large-qiang-taylor/168a617397d8509ba9fe67e9889f2cab/?utm_source=chatgpt
- https://consensus.app/papers/crowdom-crowdsourcing-for-ontology-matching-validation-qiang-wang/55e7bc49f40d56c7994ffb1e28d1e0fc/?utm_source=chatgpt
- https://consensus.app/papers/how-does-a-text-preprocessing-pipeline-affect-ontology-qiang-taylor/b21e8fc85c665d108ec0022368151aca/?utm_source=chatgpt
- https://consensus.app/papers/automating-ontology-mapping-in-it-service-management-a-khalov-ataeva/74e0b948f0a8599a8ba223f23eeac3cc/?utm_source=chatgpt
- https://consensus.app/papers/llm4vkg-leveraging-large-language-models-for-virtual-xiao-ren/c6486ba49d125d66ad70bb4f97df5dc7/?utm_source=chatgpt
- https://consensus.app/papers/large-language-models-for-ontology-engineering-a-li-garijo/3087bb8f7cd0500d917f89d8a92559e5/?utm_source=chatgpt
- https://consensus.app/papers/a-short-review-for-ontology-learning-stride-to-large-du-an/ad3e2c6bf660569ca1effb7b6d31a6f7/?utm_source=chatgpt

## Next executable consequences

1. Issue #2 Generation evaluation must expose task-level metrics rather than one aggregate "ontology quality" score.
2. Issue #3 Client work must implement retrieval-before-prompt, structural/neighborhood evidence, deterministic lexical evidence, explicit abstention, release diff/version compatibility, and OAEI-LLM hallucination fixtures.
3. GRC must remain the first enterprise round-trip fixture, while OAEI/RODI/LLMs4OL-style data guards against overfitting the general contract to GRC.
4. LLM calls remain behind `contextual-orchestrator`; model/provider/prompt changes require receipts and sensitivity evidence.
5. Human review remains mandatory before authority promotion; Crowd-OM is evidence for scalable validation mechanics, not permission to replace GRC/domain steward authority.
