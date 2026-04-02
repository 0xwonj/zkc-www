use leptos::prelude::*;

#[component]
pub fn ProtocolIrDiagram() -> impl IntoView {
    const FRAME_WIDTH: i32 = 978;
    const FRAME_HEIGHT: i32 = 358;
    const INPUT_X: i32 = 36;
    const INPUT_WIDTH: i32 = 220;
    const INPUT_TEXT_X: i32 = 58;
    const CORE_X: i32 = 334;
    const CORE_WIDTH: i32 = 104;
    const CLOSED_X: i32 = 474;
    const CLOSED_WIDTH: i32 = 116;
    const OPT_X: i32 = 626;
    const OPT_WIDTH: i32 = 70;
    const OUTPUT_X: i32 = 756;
    const OUTPUT_WIDTH: i32 = 176;
    const OUTPUT_TEXT_X: i32 = OUTPUT_X + 20;
    const INPUT_CENTER_X: i32 = INPUT_X + INPUT_WIDTH / 2;
    const STAGE_CENTER_X: i32 = (CORE_X + OPT_X + OPT_WIDTH) / 2;
    const OUTPUT_CENTER_X: i32 = OUTPUT_X + OUTPUT_WIDTH / 2;
    const CORE_CENTER_X: i32 = CORE_X + CORE_WIDTH / 2;
    const CLOSED_CENTER_X: i32 = CLOSED_X + CLOSED_WIDTH / 2;
    const OPT_CENTER_X: i32 = OPT_X + OPT_WIDTH / 2;

    view! {
        <figure class="protocol-diagram-shell">
            <svg
                class="protocol-diagram-svg"
                viewBox="0 0 980 360"
                role="img"
                aria-labelledby="protocol-diagram-title protocol-diagram-desc"
            >
                <title id="protocol-diagram-title">"Protocol IR compiler flow"</title>
                <desc id="protocol-diagram-desc">
                    "A diagram showing protocol intent, arithmetic objects, and security requirements converging into Protocol IR stages core, closed, and opt. The closed stage is defined by the semantic triple of proof surface, transcript trace, and verifier relation, then flows to library and kernel backend paths."
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
                    "Protocol stages"
                </text>
                <text class="diagram-section-label" x=OUTPUT_CENTER_X y="40">
                    "Outputs"
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
                        "protocol intent"
                    </text>
                    <text class="diagram-node-meta" x=INPUT_TEXT_X y="121">
                        "DSL, builder API"
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
                        "arithmetic object"
                    </text>
                    <text class="diagram-node-meta" x=INPUT_TEXT_X y="194">
                        "R1CS, Plonkish, AIR, LLZK"
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
                        "security profile"
                    </text>
                    <text class="diagram-node-meta" x=INPUT_TEXT_X y="267">
                        "FS profile, assumptions"
                    </text>
                </g>

                <path class="diagram-connector" d="M256 107 H286 V180"></path>
                <path class="diagram-connector" d="M256 180 H286"></path>
                <path class="diagram-connector" d="M256 253 H286 V180"></path>
                <path
                    class="diagram-flow"
                    d="M286 180 H334"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage">
                    <rect
                        class="diagram-stage-box"
                        x=CORE_X
                        y="152"
                        width=CORE_WIDTH
                        height="56"
                        rx="14"
                    ></rect>
                    <text class="diagram-stage-title" x=CORE_CENTER_X y="180">
                        "core"
                    </text>
                </g>

                <path
                    class="diagram-flow"
                    d="M438 180 H474"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage diagram-stage-closed">
                    <rect
                        class="diagram-stage-box"
                        x=CLOSED_X
                        y="152"
                        width=CLOSED_WIDTH
                        height="56"
                        rx="14"
                    ></rect>
                    <text class="diagram-stage-title" x=CLOSED_CENTER_X y="180">
                        "closed"
                    </text>
                </g>

                <text class="diagram-formula" x=STAGE_CENTER_X y="308">
                    "closed = (proof surface, transcript trace, verifier relation)"
                </text>

                <path
                    class="diagram-flow"
                    d="M590 180 H626"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>

                <g class="diagram-stage">
                    <rect
                        class="diagram-stage-box"
                        x=OPT_X
                        y="152"
                        width=OPT_WIDTH
                        height="56"
                        rx="14"
                    ></rect>
                    <text class="diagram-stage-title" x=OPT_CENTER_X y="180">
                        "opt"
                    </text>
                </g>

                <path class="diagram-connector" d="M696 180 H730"></path>
                <path class="diagram-connector" d="M730 180 V134"></path>
                <path class="diagram-connector" d="M730 180 V222"></path>
                <path
                    class="diagram-flow"
                    d="M730 134 H756"
                    marker-end="url(#protocol-diagram-arrow)"
                ></path>
                <path
                    class="diagram-flow"
                    d="M730 222 H756"
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
                        "LibraryBackend path"
                    </text>
                    <text class="diagram-node-meta" x=OUTPUT_TEXT_X y="144">
                        "e.g. Plonky3"
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
                        "KernelBackend path"
                    </text>
                </g>
            </svg>
        </figure>
    }
}
