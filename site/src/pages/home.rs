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
                    <SectionHeading title="Abstract" />
                    <p class="paper-body">
                        "ZKC is a compiler infrastructure for zero-knowledge proving that treats the end-to-end proving pipeline as a systems and compiler problem rather than as a loose combination of frontend languages, arithmetic encodings, proving libraries, and handwritten optimizations. Current zk stacks are fragmented across incompatible abstractions: source programs, arithmetic representations, protocol logic, verifier construction, and execution planning are often coupled in ad hoc ways, making it difficult to preserve semantics, enforce security-relevant invariants, reuse optimizations, or retarget proofs across backends and hardware. That fragmentation limits both research iteration and production reliability as proof systems become more heterogeneous, recursive, hardware-sensitive, and operationally complex."
                    </p>
                    <p class="paper-body">
                        "ZKC addresses this by organizing zk proving as a full compiler stack with explicit layers for intent, arithmetic structure, protocol construction, verifier logic, execution planning, and runtime artifacts. In this view, a zk compiler should not stop at arithmetization, nor should execution be the place where proof-system details are implicitly finalized. The goal is to compile a proving task into a closed, analyzable, and backend-neutral proof artifact with a stable proof interface, explicit verifier behavior, explicit transcript and security discipline, and a clear separation between cryptographic meaning and performance realization."
                    </p>
                    <p class="paper-body">
                        "The broader aim is shared infrastructure in which protocol and verifier construction, optimization, validation, and backend lowering can be treated as compiler activities rather than prover-specific engineering. By making interfaces explicit across the stack, ZKC is meant to support better modularity, portability, assurance, and extensibility across proving systems while remaining compatible with future kernels, planning layers, and runtime execution paths. In that sense, ZKC is a research program in zk compiler infrastructure: a foundation for building, optimizing, validating, and operating proof systems as compiled artifacts rather than monolithic prover implementations."
                    </p>
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
                            <span class="paper-toc-desc">"Overview, repository, and documents"</span>
                        </A>
                    </nav>
                </section>
            </article>
        </>
    }
}
