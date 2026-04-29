#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DocEntry {
    pub slug: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub category: &'static str,
    pub status: &'static str,
    pub summary: &'static str,
    pub hackmd_url: Option<&'static str>,
    pub featured: bool,
    pub sort_order: u8,
}

const DOCS: [DocEntry; 3] = [
    DocEntry {
        slug: "grant-proposal",
        title: "Grant Proposal",
        subtitle: "Protocol IR and compiler infrastructure for zero-knowledge proving",
        category: "Project",
        status: "Grant proposal",
        summary: "Frames the project scope, public-good motivation, milestones, and validation criteria for building compiler infrastructure for zero-knowledge proving around Protocol IR.",
        hackmd_url: Some("https://hackmd.io/@wonj/grant-proposal"),
        featured: false,
        sort_order: 1,
    },
    DocEntry {
        slug: "protocol-ir",
        title: "Protocol IR",
        subtitle: "A typed protocol-object layer for zero-knowledge proofs",
        category: "Core Thesis",
        status: "Canonical research document",
        summary: "Defines Protocol IR as a protocol-object layer between arithmetization and backend realization: open source P_core, sealed artifact P_closed, attached certificate C, and the FSAdmissible closure boundary that admits public-coin sources into non-interactive arguments under strong Fiat-Shamir.",
        hackmd_url: Some("https://hackmd.io/@wonj/protocol-ir"),
        featured: true,
        sort_order: 2,
    },
    DocEntry {
        slug: "positioning",
        title: "Positioning",
        subtitle: "Relationship to ArkLib, hax, and Vellvm",
        category: "Positioning",
        status: "Working positioning document",
        summary: "Positions Protocol IR against ArkLib (Lean formal spec) and hax (Rust extraction), with the LLVM IR : Vellvm :: Protocol IR : ArkLib analogy and a three-phase integration vision.",
        hackmd_url: Some("https://hackmd.io/@wonj/positioning"),
        featured: true,
        sort_order: 3,
    },
];

pub fn all_documents() -> Vec<DocEntry> {
    let mut docs = DOCS.to_vec();
    docs.sort_by_key(|doc| doc.sort_order);
    docs
}

pub fn protocol_ir_documents() -> Vec<DocEntry> {
    all_documents()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn slugs_are_unique() {
        let mut seen = HashSet::new();

        for doc in all_documents() {
            assert!(seen.insert(doc.slug), "duplicate slug: {}", doc.slug);
        }
    }

    #[test]
    fn sort_orders_are_unique() {
        let mut seen = HashSet::new();

        for doc in all_documents() {
            assert!(
                seen.insert(doc.sort_order),
                "duplicate sort_order: {}",
                doc.sort_order
            );
        }
    }

    #[test]
    fn titles_and_summaries_are_present() {
        for doc in all_documents() {
            assert!(
                !doc.title.trim().is_empty(),
                "title missing for {}",
                doc.slug
            );
            assert!(
                !doc.summary.trim().is_empty(),
                "summary missing for {}",
                doc.slug
            );
        }
    }

    #[test]
    fn hackmd_links_are_https_when_present() {
        for doc in all_documents() {
            if let Some(url) = doc.hackmd_url {
                assert!(
                    url.starts_with("https://"),
                    "hackmd url should be https: {}",
                    url
                );
            }
        }
    }
}
