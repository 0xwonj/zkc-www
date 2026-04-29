use crate::{
    components::{DocumentEntry, PageHeader, ProtocolIrDiagram, SectionHeading},
    data::{PROTOCOL_IR_REPOSITORY_URL, protocol_ir_documents},
};
use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn ProtocolIrPage() -> impl IntoView {
    view! {
        <>
            <Title text="Protocol IR — ZKC" />

            <article class="paper" data-reveal>
                <PageHeader
                    title="Protocol IR"
                    subtitle="A typed protocol-object layer for zero-knowledge proofs"
                />

                <hr class="paper-rule" />

                <section class="paper-section">
                    <SectionHeading title="Abstract" />
                    <p class="paper-body">
                        "Protocol IR is a protocol-object layer between arithmetization and backend realization. It names an open protocol source, checks when that source can be sealed, and records what the sealed artifact is allowed to claim. The initial active core targets public-coin protocols closed into non-interactive arguments by a strong Fiat-Shamir boundary, with ROM-style closure as the mainline; other oracle models require an explicit supporting theorem entry."
                    </p>
                    <p class="paper-body">
                        "The framework distinguishes an open source `P_core`, a closed non-interactive artifact `P_closed`, the verifier-observable face `P_obs = (Σ, T, V)` read from that artifact, and a certificate `C` attached to `P_closed`. Closure is the typed boundary at which live public-coin challenges become transcript-derived values: it succeeds only when six obligations — schedule integrity, public-input binding, namespace separation, construction discipline, theorem dispatch, and budget — discharge together. `C` then records the closure evidence, exported claims, scheme-conditioned rows, and any visible non-claims that travel with the artifact."
                    </p>
                    <p class="paper-body">
                        "Three reader-facing responsibilities organize the layer: open sources must be "
                        <strong>"describable"</strong>
                        " through typed components, "
                        <strong>"composable"</strong>
                        " through typed operators with explicit separation between structural wiring and theorem-dependent claim lifting, and "
                        <strong>"verifiable"</strong>
                        " at the closure boundary where Fiat-Shamir admissibility, theorem dispatch, and certificate accounting line up. Lowering, code generation, runtime scheduling, MLIR carriers, and mechanized proof artifacts are companion tracks that consume `P_closed` and `C` as a read-only contract."
                    </p>
                    <p class="paper-body">
                        <span class="paper-keyword-label">"Keywords."</span>
                        " Fiat-Shamir transformation, interactive oracle proofs, protocol composition, MLIR."
                    </p>
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <SectionHeading title="Structure" />
                    <ProtocolIrDiagram />
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <SectionHeading title="Repository" />
                    <nav class="paper-toc" aria-label="Protocol IR repository">
                        <a
                            class="paper-toc-entry"
                            href=PROTOCOL_IR_REPOSITORY_URL
                            target="_blank"
                            rel="noreferrer noopener"
                        >
                            <span class="paper-toc-num">"1."</span>
                            <span class="paper-toc-label">"Protocol IR Repository"</span>
                            <span class="paper-toc-dots" />
                            <span class="paper-toc-desc">"Placeholder"</span>
                        </a>
                    </nav>
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <SectionHeading title="Documents" />
                    <ol class="paper-doc-list paper-doc-list-compact" aria-label="Protocol IR documents">
                        <For
                            each=protocol_ir_documents
                            key=|doc| doc.slug
                            children=move |doc| view! { <DocumentEntry doc compact=true /> }
                        />
                    </ol>
                </section>
            </article>
        </>
    }
}
