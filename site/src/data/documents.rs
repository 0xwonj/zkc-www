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

const DOCS: [DocEntry; 8] = [
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
        slug: "system-and-scope",
        title: "System and Scope",
        subtitle: "Project architecture and current research boundary",
        category: "Overview",
        status: "Canonical project overview",
        summary: "Defines the larger ZK compiler architecture, separates source, arithmetic, protocol, verifier, and execution layers, and fixes the current project scope at protocol instantiation before kernelization.",
        hackmd_url: Some("https://hackmd.io/@wonj/system-and-scope"),
        featured: true,
        sort_order: 2,
    },
    DocEntry {
        slug: "protocol-ir",
        title: "Protocol IR",
        subtitle: "The semantic lock-in layer between arithmetic structure and execution",
        category: "Core Thesis",
        status: "Canonical research document",
        summary: "Defines Protocol IR as the semantic lock-in layer between arithmetic structure and execution structure, with explicit protocol closure before execution lowering.",
        hackmd_url: Some("https://hackmd.io/@wonj/protocol-ir"),
        featured: true,
        sort_order: 3,
    },
    DocEntry {
        slug: "implementation-spec",
        title: "Implementation Spec",
        subtitle: "MLIR realization of Protocol IR, verifier IR, staged closure, and Transform control",
        category: "Implementation",
        status: "Canonical implementation-facing document",
        summary: "Specifies the MLIR object model, pass structure, verifier surface, and backend boundary that realize the current Protocol IR thesis.",
        hackmd_url: Some("https://hackmd.io/@wonj/mlir-spec"),
        featured: true,
        sort_order: 4,
    },
    DocEntry {
        slug: "fiat-shamir-protocol-ir",
        title: "Fiat–Shamir in Protocol IR",
        subtitle: "Research design note",
        category: "Research Note",
        status: "Working internal research document",
        summary: "Argues that Fiat–Shamir belongs inside Protocol IR as a compiler-level protocol-closure problem rather than as a proving-library helper.",
        hackmd_url: Some("https://hackmd.io/@wonj/fiat-shamir"),
        featured: false,
        sort_order: 5,
    },
    DocEntry {
        slug: "extensions-and-roadmap",
        title: "Extensions and Roadmap",
        subtitle: "Direct continuation of Protocol IR and the path to a native prover compiler",
        category: "Roadmap",
        status: "Canonical extension-and-roadmap document",
        summary: "Prioritizes the direct next steps after the current core contribution, including LLZK-backed views, specialization, semantic enrichment, and later execution lowering.",
        hackmd_url: Some("https://hackmd.io/@wonj/extensions-and-roadmap"),
        featured: true,
        sort_order: 6,
    },
    DocEntry {
        slug: "future-research",
        title: "Future Research",
        subtitle: "Beyond Protocol IR: DAG kernels, semantic-aware proving, and new protocol families",
        category: "Future Work",
        status: "Curated future research document",
        summary: "Maps the broader research space beyond the first paper, including distributed lowering, new protocol families, autotuning, and semantic-aware proving.",
        hackmd_url: Some("https://hackmd.io/@wonj/future-research"),
        featured: false,
        sort_order: 7,
    },
    DocEntry {
        slug: "references",
        title: "Reference Stack",
        subtitle: "Recommended bibliography structure",
        category: "References",
        status: "Recommended bibliography",
        summary: "Collects the compiler, ZK, Fiat–Shamir, PL, and verification references that anchor the Protocol IR research program.",
        hackmd_url: Some("https://hackmd.io/@wonj/references"),
        featured: false,
        sort_order: 8,
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
