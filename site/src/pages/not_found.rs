use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <>
            <Title text="Not found — ZK Compiler Research" />

            <article class="paper" data-reveal>
                <header class="paper-header">
                    <h1 class="paper-project">"404"</h1>
                    <p class="paper-title">
                        "This edition currently exposes only the home page and the Protocol IR dossier."
                    </p>
                </header>

                <hr class="paper-rule" />

                <section class="paper-section">
                    <h2 class="paper-heading">
                        <span class="paper-heading-label">"Available pages."</span>
                    </h2>
                    <nav class="paper-toc">
                        <A href="/" attr:class="paper-toc-entry">
                            <span class="paper-toc-num">"1."</span>
                            <span class="paper-toc-label">"Home"</span>
                            <span class="paper-toc-dots" />
                            <span class="paper-toc-desc">"Front page"</span>
                        </A>
                        <A href="/protocol-ir" attr:class="paper-toc-entry">
                            <span class="paper-toc-num">"2."</span>
                            <span class="paper-toc-label">"Protocol IR"</span>
                            <span class="paper-toc-dots" />
                            <span class="paper-toc-desc">"Project dossier"</span>
                        </A>
                    </nav>
                </section>
            </article>
        </>
    }
}
