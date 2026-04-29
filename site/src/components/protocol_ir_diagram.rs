use leptos::prelude::*;

#[component]
pub fn ProtocolIrDiagram() -> impl IntoView {
    const FRAME_WIDTH: i32 = 978;
    const FRAME_HEIGHT: i32 = 358;

    const INPUT_X: i32 = 36;
    const INPUT_WIDTH: i32 = 220;
    const INPUT_TEXT_X: i32 = 58;
    const INPUT_CENTER_X: i32 = INPUT_X + INPUT_WIDTH / 2;

    const PCORE_X: i32 = 320;
    const PCORE_WIDTH: i32 = 100;
    const PCORE_CENTER_X: i32 = PCORE_X + PCORE_WIDTH / 2;

    const PCLOSED_X: i32 = 480;
    const PCLOSED_WIDTH: i32 = 130;
    const PCLOSED_CENTER_X: i32 = PCLOSED_X + PCLOSED_WIDTH / 2;

    const C_X: i32 = 624;
    const C_WIDTH: i32 = 56;
    const C_CENTER_X: i32 = C_X + C_WIDTH / 2;

    const STAGE_CENTER_X: i32 = (PCORE_X + C_X + C_WIDTH) / 2;

    const OUTPUT_X: i32 = 756;
    const OUTPUT_WIDTH: i32 = 176;
    const OUTPUT_TEXT_X: i32 = OUTPUT_X + 20;
    const OUTPUT_CENTER_X: i32 = OUTPUT_X + OUTPUT_WIDTH / 2;

    view! {
        <figure class="protocol-diagram-shell">
            <svg
                class="protocol-diagram-svg"
                viewBox="0 0 980 360"
                role="img"
                aria-labelledby="protocol-diagram-title protocol-diagram-desc"
            >
                <title id="protocol-diagram-title">"Protocol IR closure flow"</title>
                <desc id="protocol-diagram-desc">
                    "A diagram showing relation payload, protocol profile, and backend witness flowing into Protocol IR. The open source P_core is sealed by close into the closed artifact P_closed with an attached certificate C. The verifier face is read as P_obs = (Sigma, T, V). The sealed pair flows to backend realization and audit and verification companion tracks."
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

                <rect
                    class="diagram-frame"
                    x="1"
                    y="1"
                    width=FRAME_WIDTH
                    height=FRAME_HEIGHT
                    rx="18"
                ></rect>

                <text class="diagram-section-label" x=INPUT_CENTER_X y="40">
                    "Inputs"
                </text>
                <text class="diagram-section-label" x=STAGE_CENTER_X y="40">
                    "Closure"
                </text>
                <text class="diagram-section-label" x=OUTPUT_CENTER_X y="40">
                    "Companion tracks"
                </text>

                <g class="diagram-node">
                    <rect
                        class="diagram-node-box"
                        x=INPUT_X
                        y="82"
                        width=INPUT_WIDTH
                        height="50"
                        rx="12"
                    ></rect>
                    <text class="diagram-node-title" x=INPUT_TEXT_X y="103">
                        "relation payload"
                    </text>
                    <text class="diagram-node-meta" x=INPUT_TEXT_X y="121">
                        "AIR, R1CS, Plonkish, LLZK"
                    </text>
                </g>

                <g class="diagram-node">
                    <rect
                        class="diagram-node-box"
                        x=INPUT_X
                        y="155"
                        width=INPUT_WIDTH
                        height="50"
                        rx="12"
                    ></rect>
                    <text class="diagram-node-title" x=INPUT_TEXT_X y="176">
                        "profile Π, scope ρ"
                    </text>
                    <text class="diagram-node-meta" x=INPUT_TEXT_X y="194">
                        "θ, σ, m, ν, κ"
                    </text>
                </g>

                <g class="diagram-node">
                    <rect
                        class="diagram-node-box"
                        x=INPUT_X
                        y="228"
                        width=INPUT_WIDTH
                        height="50"
                        rx="12"
                    ></rect>
                    <text class="diagram-node-title" x=INPUT_TEXT_X y="249">
                        "backend witness B"
                    </text>
                    <text class="diagram-node-meta" x=INPUT_TEXT_X y="267">
                        "B_κ, B_scheme, B_abi"
                    </text>
                </g>

                <path class="diagram-connector" d="M256 107 H286 V180"></path>
                <path class="diagram-connector" d="M256 180 H286"></path>
                <path class="diagram-connector" d="M256 253 H286 V180"></path>
                <path
                    class="diagram-flow"
                    d="M286 180 H320"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage">
                    <rect
                        class="diagram-stage-box"
                        x=PCORE_X
                        y="152"
                        width=PCORE_WIDTH
                        height="56"
                        rx="14"
                    ></rect>
                    <text class="diagram-stage-title" x=PCORE_CENTER_X y="180">
                        "P_core"
                    </text>
                </g>

                <path
                    class="diagram-flow"
                    d="M420 180 H480"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage diagram-stage-closed">
                    <rect
                        class="diagram-stage-box"
                        x=PCLOSED_X
                        y="152"
                        width=PCLOSED_WIDTH
                        height="56"
                        rx="14"
                    ></rect>
                    <text class="diagram-stage-title" x=PCLOSED_CENTER_X y="180">
                        "P_closed"
                    </text>
                </g>

                <path class="diagram-connector" d="M610 180 H624"></path>

                <g class="diagram-stage diagram-stage-closed">
                    <rect
                        class="diagram-stage-box"
                        x=C_X
                        y="160"
                        width=C_WIDTH
                        height="40"
                        rx="10"
                    ></rect>
                    <text class="diagram-stage-title" x=C_CENTER_X y="180">
                        "C"
                    </text>
                </g>

                <text class="diagram-formula" x=STAGE_CENTER_X y="302">
                    "close: P_core × Π × ρ × B → (P_closed, C)"
                </text>
                <text class="diagram-formula" x=STAGE_CENTER_X y="322">
                    "P_obs = obs(P_closed) = (Σ, T, V)"
                </text>

                <path class="diagram-connector" d="M680 180 H720"></path>
                <path class="diagram-connector" d="M720 180 V134"></path>
                <path class="diagram-connector" d="M720 180 V222"></path>
                <path
                    class="diagram-flow"
                    d="M720 134 H756"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>
                <path
                    class="diagram-flow"
                    d="M720 222 H756"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-output">
                    <rect
                        class="diagram-output-box"
                        x=OUTPUT_X
                        y="100"
                        width=OUTPUT_WIDTH
                        height="68"
                        rx="12"
                    ></rect>
                    <text class="diagram-node-title" x=OUTPUT_TEXT_X y="124">
                        "Backend realization"
                    </text>
                    <text class="diagram-node-meta" x=OUTPUT_TEXT_X y="142">
                        "lowering, codegen,"
                    </text>
                    <text class="diagram-node-meta" x=OUTPUT_TEXT_X y="158">
                        "runtime, MLIR carriers"
                    </text>
                </g>

                <g class="diagram-output">
                    <rect
                        class="diagram-output-box"
                        x=OUTPUT_X
                        y="188"
                        width=OUTPUT_WIDTH
                        height="68"
                        rx="12"
                    ></rect>
                    <text class="diagram-node-title" x=OUTPUT_TEXT_X y="212">
                        "Audit & verification"
                    </text>
                    <text class="diagram-node-meta" x=OUTPUT_TEXT_X y="230">
                        "certificate consumers,"
                    </text>
                    <text class="diagram-node-meta" x=OUTPUT_TEXT_X y="246">
                        "mechanized proofs"
                    </text>
                </g>
            </svg>
        </figure>
    }
}
