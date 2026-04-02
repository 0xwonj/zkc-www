use crate::{
    components::{DocumentEntry, ProtocolIrDiagram},
    data::{protocol_ir_documents, PROTOCOL_IR_REPOSITORY_URL},
};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn ProtocolIrPage() -> impl IntoView {
    view! {
        <>
            <Title text="Protocol IR — ZK Compiler Research" />

            <article class="paper" data-reveal>
                <header class="paper-header">
                    <h1 class="paper-project">"Protocol IR"</h1>
                    <p class="paper-title">"Project overview and materials."</p>
                </header>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <h2 class="paper-heading">
                        <span class="paper-heading-label">"Overview."</span>
                    </h2>
                    <p class="paper-body">
                        "Protocol IR is not another arithmetic IR and not an execution IR. Arithmetic IR describes the algebraic object being proved; execution IR describes how a fixed prover is realized efficiently on concrete hardware. Protocol IR sits between them and makes the proof system itself explicit: what prover and verifier do, what enters the transcript, which proof-visible objects exist, how the verifier reads them back, and which obligations must be discharged before lowering can continue."
                    </p>
                    <p class="paper-body">
                        "The semantic center of the design is a closed protocol understood as the triple `(proof surface, transcript trace, verifier relation)`. That means proof ABI is treated as a logical contract rather than a serializer detail, transcript construction is represented explicitly rather than hidden behind helper calls, and verifier structure is first-class from the start. In this framing, Fiat-Shamir belongs at the protocol layer as part of protocol closure, not as backend plumbing."
                    </p>
                    <p class="paper-body">
                        "The stage structure follows `core -> closed -> opt`. `core` is an unresolved protocol skeleton; `closed` is the first verifier-explicit, cryptographically complete form; `opt` is a semantics-preserving reorganization of that closed protocol. The point of the project is that protocol instantiation should happen before kernelization, so later library or kernel backends consume a stable protocol object instead of silently participating in protocol design."
                    </p>
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <h2 class="paper-heading">
                        <span class="paper-heading-marker">"§"</span>
                        " Structure"
                    </h2>
                    <ProtocolIrDiagram />
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <h2 class="paper-heading">
                        <span class="paper-heading-marker">"§"</span>
                        " Repository"
                    </h2>
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
                    <h2 class="paper-heading">
                        <span class="paper-heading-marker">"§"</span>
                        " Documents"
                    </h2>
                    <ol class="paper-doc-list paper-doc-list-compact" aria-label="Protocol IR documents">
                        <For
                            each=protocol_ir_documents
                            key=|doc| doc.slug
                            children=move |doc| view! { <DocumentEntry doc compact=true /> }
                        />
                    </ol>

                    <p class="paper-body">
                        <A href="/" attr:class="paper-inline-link">
                            "Return home."
                        </A>
                    </p>
                </section>
            </article>
        </>
    }
}
