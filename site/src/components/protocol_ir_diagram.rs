use leptos::prelude::*;

#[component]
pub fn ProtocolIrDiagram() -> impl IntoView {
    view! {
        <figure class="protocol-diagram-shell">
            <svg
                class="protocol-diagram-svg"
                viewBox="0 0 980 430"
                role="img"
                aria-labelledby="protocol-diagram-title protocol-diagram-desc"
            >
                <title id="protocol-diagram-title">"Protocol IR compiler flow"</title>
                <desc id="protocol-diagram-desc">
                    "A diagram showing protocol intent, arithmetic objects, and security requirements flowing into Protocol IR stages core, closed, and opt. The closed form makes proof surface, transcript trace, and verifier relation explicit before backend lowering."
                </desc>

                <defs>
                    <marker
                        id="protocol-diagram-arrow"
                        viewBox="0 0 10 10"
                        refX="9"
                        refY="5"
                        markerWidth="7"
                        markerHeight="7"
                        orient="auto-start-reverse"
                    >
                        <path class="diagram-arrow-head" d="M 0 0 L 10 5 L 0 10 z"></path>
                    </marker>
                </defs>

                <rect class="diagram-frame" x="1" y="1" width="978" height="428" rx="18"></rect>

                <text class="diagram-section-label" x="32" y="34">
                    "Inputs"
                </text>
                <text class="diagram-section-label" x="286" y="34">
                    "Protocol stages"
                </text>
                <text class="diagram-section-label" x="756" y="34">
                    "Outputs"
                </text>

                <g class="diagram-node">
                    <rect class="diagram-node-box" x="32" y="58" width="186" height="54" rx="10"></rect>
                    <text class="diagram-node-title" x="50" y="81">
                        "protocol intent"
                    </text>
                    <text class="diagram-node-meta" x="50" y="100">
                        "DSL, builder API"
                    </text>
                </g>

                <g class="diagram-node">
                    <rect class="diagram-node-box" x="32" y="145" width="186" height="54" rx="10"></rect>
                    <text class="diagram-node-title" x="50" y="168">
                        "arithmetic object"
                    </text>
                    <text class="diagram-node-meta" x="50" y="187">
                        "R1CS, Plonkish, AIR, LLZK"
                    </text>
                </g>

                <g class="diagram-node">
                    <rect class="diagram-node-box" x="32" y="232" width="186" height="54" rx="10"></rect>
                    <text class="diagram-node-title" x="50" y="255">
                        "security profile"
                    </text>
                    <text class="diagram-node-meta" x="50" y="274">
                        "FS profile, assumptions"
                    </text>
                </g>

                <path class="diagram-connector" d="M218 85 H252 V165"></path>
                <path class="diagram-connector" d="M218 172 H252 V165"></path>
                <path class="diagram-connector" d="M218 259 H252 V165"></path>
                <path
                    class="diagram-flow"
                    d="M252 165 H286"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage">
                    <rect class="diagram-stage-box" x="286" y="58" width="126" height="58" rx="12"></rect>
                    <text class="diagram-stage-title" x="349" y="87">
                        "core"
                    </text>
                </g>

                <path
                    class="diagram-flow"
                    d="M412 87 H442"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage diagram-stage-closed">
                    <rect class="diagram-stage-box" x="442" y="58" width="126" height="58" rx="12"></rect>
                    <text class="diagram-stage-title" x="505" y="87">
                        "closed"
                    </text>
                </g>

                <path
                    class="diagram-flow"
                    d="M568 87 H598"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage">
                    <rect class="diagram-stage-box" x="598" y="58" width="126" height="58" rx="12"></rect>
                    <text class="diagram-stage-title" x="661" y="87">
                        "opt"
                    </text>
                </g>

                <path class="diagram-connector" d="M349 116 V150"></path>
                <path class="diagram-connector" d="M505 116 V150"></path>
                <path class="diagram-connector" d="M661 116 V150"></path>

                <g class="diagram-panel-group">
                    <rect class="diagram-panel" x="270" y="150" width="470" height="210" rx="16"></rect>
                    <text class="diagram-panel-title" x="294" y="180">
                        "Closed protocol semantics"
                    </text>

                    <g class="diagram-card">
                        <rect class="diagram-card-box" x="290" y="202" width="132" height="86" rx="12"></rect>
                        <text class="diagram-card-title" x="306" y="225">
                            "proof surface"
                        </text>
                        <text class="diagram-card-meta" x="306" y="246">
                            "commitments"
                        </text>
                        <text class="diagram-card-meta" x="306" y="264">
                            "openings"
                        </text>
                        <text class="diagram-card-meta" x="306" y="282">
                            "query bundles"
                        </text>
                    </g>

                    <g class="diagram-card">
                        <rect class="diagram-card-box" x="439" y="202" width="132" height="86" rx="12"></rect>
                        <text class="diagram-card-title" x="455" y="225">
                            "transcript trace"
                        </text>
                        <text class="diagram-card-meta" x="455" y="246">
                            "absorbs"
                        </text>
                        <text class="diagram-card-meta" x="455" y="264">
                            "challenge schedule"
                        </text>
                        <text class="diagram-card-meta" x="455" y="282">
                            "domain separation"
                        </text>
                    </g>

                    <g class="diagram-card">
                        <rect class="diagram-card-box" x="588" y="202" width="132" height="86" rx="12"></rect>
                        <text class="diagram-card-title" x="604" y="225">
                            "verifier relation"
                        </text>
                        <text class="diagram-card-meta" x="604" y="246">
                            "algebraic checks"
                        </text>
                        <text class="diagram-card-meta" x="604" y="264">
                            "opening checks"
                        </text>
                        <text class="diagram-card-meta" x="604" y="282">
                            "Merkle / FRI"
                        </text>
                    </g>

                    <rect class="diagram-pill" x="292" y="312" width="82" height="28" rx="14"></rect>
                    <text class="diagram-pill-text" x="333" y="330">
                        "proof ABI"
                    </text>

                    <rect class="diagram-pill" x="389" y="312" width="94" height="28" rx="14"></rect>
                    <text class="diagram-pill-text" x="436" y="330">
                        "Fiat-Shamir"
                    </text>

                    <rect class="diagram-pill" x="498" y="312" width="112" height="28" rx="14"></rect>
                    <text class="diagram-pill-text" x="554" y="330">
                        "verifier replay"
                    </text>

                    <rect class="diagram-pill" x="625" y="312" width="94" height="28" rx="14"></rect>
                    <text class="diagram-pill-text" x="672" y="330">
                        "obligations"
                    </text>
                </g>

                <path class="diagram-connector" d="M724 87 H744 V154"></path>
                <path class="diagram-connector" d="M744 154 V250"></path>
                <path
                    class="diagram-flow"
                    d="M744 154 H756"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>
                <path
                    class="diagram-flow"
                    d="M744 250 H756"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-output">
                    <rect class="diagram-output-box" x="756" y="122" width="192" height="64" rx="12"></rect>
                    <text class="diagram-node-title" x="774" y="147">
                        "LibraryBackend path"
                    </text>
                    <text class="diagram-node-meta" x="774" y="167">
                        "e.g. Plonky3"
                    </text>
                </g>

                <g class="diagram-output">
                    <rect class="diagram-output-box" x="756" y="218" width="192" height="64" rx="12"></rect>
                    <text class="diagram-node-title" x="774" y="253">
                        "KernelBackend path"
                    </text>
                </g>
            </svg>
        </figure>
    }
}
