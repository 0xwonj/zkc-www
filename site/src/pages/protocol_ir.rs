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
                    subtitle="The semantic lock-in layer between arithmetic structure and execution"
                />

                <hr class="paper-rule" />

                <section class="paper-section">
                    <SectionHeading title="Overview" />
                    <p class="paper-body">
                        "Protocol IR is the compiler layer at which an arithmetic object becomes a closed, verifier-explicit proof protocol. It is not another arithmetic IR, and it is not an execution IR. Arithmetic IR describes the algebraic object being proved; execution IR describes how a fixed prover is realized efficiently on concrete hardware. Protocol IR sits between them and fixes the protocol-facing structure that must remain visible and analyzable: which proof objects exist, how the transcript is staged, when challenges are derived, how the verifier reads the proof back, and which checks define acceptance."
                    </p>
                    <p class="paper-body">
                        "The semantic center of this layer is a closed protocol, understood as the triple `(proof surface, transcript trace, verifier relation)`. The proof surface defines the external proof contract, the transcript trace records how protocol objects and challenges are bound, and the verifier relation states the exact acceptance condition. Making these three components first-class lets the compiler check closure, preserve protocol meaning under rewrites, and treat proof ABI, transcript discipline, and verifier structure as semantic commitments rather than backend conventions. In this framing, Fiat-Shamir belongs at the protocol layer as part of protocol closure, not as backend plumbing."
                    </p>
                    <p class="paper-body">
                        "The stage structure follows `core -> closed -> opt`. `core` is an unresolved protocol skeleton with pending structure and obligations; `closed` is the first verifier-explicit, cryptographically complete form; `opt` is a semantics-preserving reorganization of that closed protocol. The methodological consequence is that protocol instantiation must be completed before kernelization, so later library or kernel backends consume a stable protocol object rather than silently participating in protocol design."
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
                            <span class="paper-toc-desc">"Example link"</span>
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
