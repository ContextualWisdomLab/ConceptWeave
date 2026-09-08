# Ontology signal triage (2026-09-08)

A local, read-only aggregate over the fresh private classification report found
title keyword signals among the 3,715 bibliographic proposals. The query emitted
counts only; no title, key, abstract or source text was retained.

| Signal | Proposals |
| --- | ---: |
| `ontology` | 27 |
| `semantic` | 36 |
| `taxonomy` | 2 |
| `knowledge graph` | 1 |
| `vocabulary` stem | 81 |
| union of these signals | 137 |

The union is a discovery queue, not an ontology verdict: title matching can
miss relevant work and can include irrelevant uses. Existing deterministic
classification remains authoritative for its stated dispositions; all 3,658
abstentions and four pending source-resolution keys remain in scope for
steward review. No model call, decision, approval or Zotero write occurred.

Within the 137-signal queue, deterministic dispositions are adjacent evidence
29, semantic consumption bridge 1, and needs steward review 107. The 107
abstentions split into no deterministic rule match 101 and unsupported rule
vocabulary 6. This split prioritizes rule/ontology vocabulary work without
silently promoting any paper.

## Replay confirmation (2026-09-09 00:47 KST)

The same aggregate was recomputed from the exact private report
`/tmp/conceptweave-zotero-live-20260909-004718.json` (SHA-256
`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`). The
signal union remains 137 proposals: 29 adjacent evidence, one semantic
consumption bridge and 107 steward-review abstentions (101 no deterministic
rule match, six unsupported rule vocabulary). Matching policy is explicit:
`ontology`, `semantic`, `taxonomy`, `knowledge graph` and the `vocab` stem are
case-insensitive; exact uppercase `OWL` and `RDF` counts are separate discovery
signals. These counts do not create decisions, approvals or Zotero writes.

## Deterministic evidence-phrase checkpoint (2026-09-09 03:20 KST)

Recomputing classifier evidence fields from the same private replay report
(`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`) found
matched-phrase counts of `ontology` 32, `owl` 24, `rdf` 20, `knowledge graph`
2 and `linked data` 1. These are classifier-evidence signals, not the separate
title-only discovery counts above; they must not be summed into a new KPI or
treated as steward decisions.

## Bound batch priority checkpoint (2026-09-09 03:25 KST)

The first 25-item capture-bound review view contains 25 blank decisions and 25
attachment-evidence entries. None of those titles contains the literal
`ontology` or `semantic` signal, so this first page is not an ontology-priority
queue. It remains a steward batch selected by worksheet order; no rows were
auto-promoted or skipped.

## Read-only ontology discovery queue checkpoint (2026-09-09 04:08 KST)

From the byte-identical live replay, a private queue was projected for steward
triage at `/tmp/conceptweave-ontology-discovery-queue.XXXXXX.json` (`0600`).
The filter is an OR of case-insensitive title or matched-phrase signals for
`ontology`, `semantic`, `taxonomy`, `knowledge graph`, `linked data` and
`vocab`; it produced 142 candidate rows (137 title-signal rows and 35 rows with
at least one matched phrase). Its SHA-256 is
`a096e2a92bdd17f2f426fd058c619ea45b6397c54004641f40af366b08de2dd7`. The
queue is discovery input only: it does not change dispositions, fill blank
full-text decisions, clear pending sources or authorize Zotero writes.

## External implementation-candidate checkpoint (2026-09-09)

An upstream-only scan recorded three Rust candidates for a later compatibility
matrix: [Oxigraph](https://github.com/oxigraph/oxigraph) (RDF/SPARQL storage and
tooling), [Sophia](https://github.com/pchampin/sophia_rs) (RDF/Linked Data data
model and parsers) and [Rudof](https://github.com/rudof-project/rudof) (RDF
shapes validation, including SHACL and ShEx). The scan did not install, copy,
publish or adopt any candidate. Release/license/provenance, conformance,
performance and ACL-boundary checks remain prerequisites; these libraries do
not supply ConceptWeave semantic authority or Zotero steward decisions.

## Signal breakdown continuity checkpoint (2026-09-09)

The same release-mode replay (`8e533d4d4b27d93bce8e1223528b109f07ba69ff19d1e41cf58a3245b02545cc`)
was projected into a separate private aggregate (`0600`, SHA-256
`2edc6d86f9d4ea7ae745340754d7d28aaf9925551803bc2390ad2b23e31d4a0c`).
Case-insensitive title/phrase counts are: `ontology` 27/32, `semantic` 36/0,
`taxonomy` 2/0, `knowledge graph` 1/2, `linked data` 1/1 and `vocab` 81/0.
These are overlapping discovery signals, not additive paper counts or
dispositions; no full-text decision, authority or Zotero mutation occurred.

## Discovery KPI checkpoint (2026-09-09)

The queue rate is `142 / 3,715 = 3.8223%` of observed bibliographic items.
This is a reproducible screening KPI for steward workload, not ontology
precision, recall or approval coverage. The denominator includes every observed
item, while the queue remains an overlapping title/phrase discovery projection.
