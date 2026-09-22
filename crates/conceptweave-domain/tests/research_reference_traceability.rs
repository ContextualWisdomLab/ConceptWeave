const REFERENCES: &str = include_str!("../../../docs/doctoring/REFERENCES.md");
const TRACEABILITY: &str =
    include_str!("../../../docs/doctoring/RESEARCH_CAPABILITY_TRACEABILITY.md");

#[test]
fn adopted_alignment_studies_have_authoritative_apa_bibliography_records() {
    for (traceability_marker, authoritative_record, publication_year_marker) in [
        (
            "He, Chen, Dong, & Horrocks (2023)",
            "https://ceur-ws.org/Vol-3632/ISWC2023_paper_427.pdf",
            "(2023).",
        ),
        (
            "Amini, Saki Norouzi, Hitzler, & Amini (2024)",
            "https://doi.org/10.1007/978-3-031-81221-7_2",
            "(2025).",
        ),
    ] {
        assert!(
            TRACEABILITY.contains(traceability_marker),
            "an adopted study must remain explicit in research-to-capability traceability: {traceability_marker}"
        );
        assert!(
            REFERENCES.split("\n\n").any(|entry| {
                entry.contains(authoritative_record) && entry.contains(publication_year_marker)
            }),
            "an adopted study must have an authoritative APA publication record with the publication year bound to that record: {authoritative_record} {publication_year_marker}"
        );
    }
}
