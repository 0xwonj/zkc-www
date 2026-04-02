use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <>
            <Title text="ZK Compiler Research" />

            <article class="paper" data-reveal>
                <header class="paper-header paper-header-home">
                    <h1 class="paper-project paper-project-home">"ZK Compiler Research"</h1>
                </header>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <h2 class="paper-heading">
                        <span class="paper-heading-label">"Abstract."</span>
                    </h2>
                    <p class="paper-body">
                        "A useful zero-knowledge compiler stack should separate protocol intent, arithmetic structure, protocol closure, and execution lowering rather than collapsing them into prover-specific libraries. Existing systems already validate arithmetic IRs, backend-facing proof IRs, and execution-oriented implementations, but the boundary where an arithmetic object becomes a closed prover/verifier protocol is still under-specified."
                    </p>
                    <p class="paper-body">
                        "This archive collects a research direction built around that missing layer. Its current center of gravity is Protocol IR, together with supporting documents on MLIR realization, Fiat-Shamir as protocol compilation, roadmap sequencing, future research, and references. The broader aim is a ZK compiler architecture in which soundness-critical structure is explicit, reusable compiler passes become possible, and implementation and research iteration happen against stable semantic objects rather than ad hoc library boundaries."
                    </p>
                    <p class="paper-body">
                        <span class="paper-keyword-label">"Keywords."</span>
                        " Protocol IR, zero-knowledge proofs, compilers, intermediate representations, protocol compilation, verifier construction, Fiat-Shamir transformation."
                    </p>
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <h2 class="paper-heading">
                        <span class="paper-heading-marker">"§"</span>
                        " Contents"
                    </h2>
                    <nav class="paper-toc" aria-label="Site contents">
                        <A href="/protocol-ir" attr:class="paper-toc-entry">
                            <span class="paper-toc-num">"1."</span>
                            <span class="paper-toc-label">"Protocol IR"</span>
                            <span class="paper-toc-dots" />
                            <span class="paper-toc-desc">"Overview, repository, and documents"</span>
                        </A>
                    </nav>
                </section>
            </article>
        </>
    }
}
