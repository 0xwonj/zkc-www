use crate::data::DocEntry;
use leptos::prelude::*;

#[component]
pub fn DocumentEntry(doc: DocEntry, #[prop(optional)] compact: bool) -> impl IntoView {
    let entry_class = if doc.hackmd_url.is_some() {
        if compact {
            "paper-toc-entry paper-toc-entry-compact"
        } else {
            "paper-toc-entry"
        }
    } else if compact {
        "paper-toc-entry paper-toc-entry-disabled paper-toc-entry-compact"
    } else {
        "paper-toc-entry paper-toc-entry-disabled"
    };

    view! {
        <li class="paper-doc">
            {match doc.hackmd_url {
                Some(url) => view! {
                    <a
                        class=entry_class
                        href=url
                        target="_blank"
                        rel="noreferrer noopener"
                    >
                        <span class="paper-toc-num">{format!("{}.", doc.sort_order)}</span>
                        <span class="paper-toc-main">
                            <span class="paper-toc-label">{doc.title}</span>
                            <span class="paper-toc-meta">{doc.subtitle}</span>
                        </span>
                        <span class="paper-toc-dots" />
                        <span class="paper-toc-desc">"HackMD"</span>
                    </a>
                }
                    .into_any(),
                None => view! {
                    <div class=entry_class aria-disabled="true">
                        <span class="paper-toc-num">{format!("{}.", doc.sort_order)}</span>
                        <span class="paper-toc-main">
                            <span class="paper-toc-label">{doc.title}</span>
                            <span class="paper-toc-meta">{doc.subtitle}</span>
                        </span>
                        <span class="paper-toc-dots" />
                        <span class="paper-toc-desc">"Pending"</span>
                    </div>
                }
                    .into_any(),
            }}

            {(!compact)
                .then(|| {
                    view! {
                        <p class="paper-entry-summary">
                            {doc.summary}
                            " "
                            <span class="paper-entry-summary-meta">
                                {format!("({} · {})", doc.category, doc.status)}
                            </span>
                        </p>
                    }
                })}
        </li>
    }
}
