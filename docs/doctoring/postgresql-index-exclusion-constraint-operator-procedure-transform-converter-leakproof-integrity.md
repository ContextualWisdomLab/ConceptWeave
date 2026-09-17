# PostgreSQL ordinary EXCLUDE transform-converter leakproof integrity

## Decision

ConceptWeave Source Observation must preserve each selected nonzero transform-converter function's raw same-row `pg_proc.proleakproof` as an independently observed fact after converter definition, owner, object-level `EXECUTE` ACL, nullable `proconfig`, and `prosecdef` identity. The direct predecessor is `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot`; no predecessor digest domain is rewritten.

This is an observational contract, not an admission policy. Both `false` and `true` are representable source states. ConceptWeave does not infer `proleakproof` from `prosecdef`, owner, ACL, function-local configuration, language, implementation material, current caller, or planner behavior.

## Problem and causal finding

Finding review `5235830453` on exact predecessor `1153c0b6f121b5b99c533d73e3dc9e7b4305c276` identified a material source-identity hole. PostgreSQL 18 permits `ALTER FUNCTION ... [NOT] LEAKPROOF` independently of the already-governed converter facts. Consequently two converter rows could remain equal across the existing ConceptWeave successor chain while differing in a security- and planner-relevant catalog property.

PostgreSQL defines `LEAKPROOF` as a trust assertion that a function reveals no information about its arguments except through its return value. The planner may evaluate leakproof functions before security-barrier view predicates or row-level-security policy expressions, whereas non-leakproof user conditions are ordered behind those security predicates. PostgreSQL also uses the underlying operator function's leakproof status when deciding whether otherwise inaccessible statistics may participate in selectivity estimation. The catalog flag therefore cannot be treated as descriptive decoration or derived from neighboring function metadata.

## Alternatives considered

1. **Infer leakproofness from the converter implementation. Rejected.** PostgreSQL stores and changes the catalog assertion independently; static inspection cannot reproduce the server's authoritative flag and would conflate implementation analysis with source observation.
2. **Fold `proleakproof` into the existing security-definer digest. Rejected.** That would mutate an already-issued predecessor domain and erase the one-fact-at-a-time provenance boundary.
3. **Require all converters to be non-leakproof. Rejected.** Source Observation records database truth; product/governance policy belongs downstream of validated semantic facts.
4. **Add a frozen successor over the exact security-definer predecessor. Selected.** It preserves historical digest semantics, exact converter-direction coverage, and a reviewable causal boundary.

## Contract

`IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofObservation` records, for one exact ordinary-EXCLUDE converter direction:

- exact exclusion-constraint coordinate and one-based key position;
- exact selected transform type and FROM-SQL/TO-SQL direction;
- exact converter schema/function identity repeated from the predecessor;
- raw same-row Boolean `pg_proc.proleakproof`.

`IndexExclusionConstraintOperatorProcedureTransformConverterLeakproofSnapshot` requires its observation coordinates to equal the complete converter-direction inventory of `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot`. Duplicate coordinates, missing/extra directions, zero positions, converter-function binding drift, and unknown receipt coordinates fail closed. The successor digest is domain-separated and includes the immutable predecessor digest plus exact coordinates, converter binding, and raw Boolean state.

Synthetic false/true values in the focused contract are distinguishability controls only; they are not empirical claims about a production PostgreSQL instance.

## PostgreSQL 18 bounded differential

For each selected `(trftype, target prolang)` `pg_transform` row and each nonzero converter OID, one bounded capture must resolve the same exact converter `pg_proc` generation and independently read retained converter definition, `proowner`, `proacl`, `proconfig`, `prosecdef`, and `proleakproof`. A mixed-generation join, unresolved converter row, or inferred leakproof value is capture failure rather than an `unknown` placeholder.

The differential must retain all earlier ordinary-EXCLUDE operator, target-function, operator-family/strategy, backing-index, namespace/lifecycle/access-method, and v3 source-content-generation controls. A passing leakproof comparison cannot compensate for a predecessor mismatch.

## Security and assurance interpretation

PostgreSQL is the technical authority for `proleakproof` semantics. NIST least-privilege and assessment guidance supports preserving independently testable security-relevant configuration evidence, but it does not define PostgreSQL planner semantics and is not used to infer the catalog value. In particular, a leakproof flag is not equivalent to authorization, owner trust, `SECURITY DEFINER`, or an organizational approval.

## TRACEABILITY

- owner PR: `ContextualWisdomLab/ConceptWeave#46`
- finding review: `5235830453`
- structural source/compile RED: `ce86f3c7e84054a0c58e38f81a8b230c3e2653b3`
- production successor: `14ebe48407ab448586f68dbce8b2cd55b32ea5ba`
- public module composition: `5a8229d6d04e3d4498cac61b0603eacc2e4398bc`
- predecessor exact head: `1153c0b6f121b5b99c533d73e3dc9e7b4305c276`
- source: `crates/conceptweave-relation-partition/src/index_exclusion_constraint_operator_procedure_transform_converter_leakproof.rs`
- focused contract: `crates/conceptweave-relation-partition/tests/index_exclusion_constraint_operator_procedure_transform_converter_leakproof_contract.rs`
- direct predecessor type: `IndexExclusionConstraintOperatorProcedureTransformConverterSecurityDefinerSnapshot`
- exact catalog fact: each nonzero selected converter function's raw same-row `pg_proc.proleakproof`

No executed compiler RED/GREEN is claimed by this record. Exact-head Rust 1.98 native checks, hosted required checks, PostgreSQL 18 live differential, and release evidence remain separate acceptance stages.

## References

Joint Task Force. (2022). *Assessing security and privacy controls in information systems and organizations* (NIST Special Publication 800-53A Rev. 5; current CPRT release maintained by NIST). National Institute of Standards and Technology. https://csrc.nist.gov/pubs/sp/800/53/a/r5/final

PostgreSQL Global Development Group. (2026). *ALTER FUNCTION*. In *PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/sql-alterfunction.html

PostgreSQL Global Development Group. (2026). *CREATE FUNCTION*. In *PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/sql-createfunction.html

PostgreSQL Global Development Group. (2026). *Planner statistics and security*. In *PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/planner-stats-security.html

PostgreSQL Global Development Group. (2026). *Rules and privileges*. In *PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/rules-privileges.html

PostgreSQL Global Development Group. (2026). *Row security policies*. In *PostgreSQL 18 documentation*. https://www.postgresql.org/docs/18/ddl-rowsecurity.html
