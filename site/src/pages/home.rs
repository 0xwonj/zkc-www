use crate::components::{PageHeader, SectionHeading};
use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <>
            <Title text="ZKC" />

            <article class="paper" data-reveal>
                <PageHeader
                    title="ZKC"
                    subtitle="Compiler infrastructure for zero-knowledge proving"
                />

                <hr class="paper-rule" />

                <section class="paper-section">
                    <p class="paper-body">
                        <span class="paper-keyword-label">"Keywords."</span>
                        " zero-knowledge proofs, compilers, MLIR, protocol compilation."
                    </p>
                </section>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <SectionHeading title="Contents" />
                    <nav class="paper-toc" aria-label="Site contents">
                        <A href="/protocol-ir" attr:class="paper-toc-entry">
                            <span class="paper-toc-num">"1."</span>
                            <span class="paper-toc-label">"Protocol IR"</span>
                            <span class="paper-toc-dots" />
                            <span class="paper-toc-desc">"Abstract, repository, and documents"</span>
                        </A>
                    </nav>
                </section>
            </article>
        </>
    }
}
