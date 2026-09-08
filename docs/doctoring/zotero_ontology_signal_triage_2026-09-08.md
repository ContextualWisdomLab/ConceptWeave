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
