# Extensions and Roadmap

**Status:** canonical extension-and-roadmap document  
**Owns:** the prioritized extension program beyond the current Protocol IR paper; the dependency-aware medium-term roadmap; the distinction between direct continuation work and broader speculative research; the recommended sequencing of family specialization, semantic enrichment, control-plane work, execution lowering, and compositional extensions  
**Does not own:** the project-level overview and current scope (`system-and-scope.md`), the paper-level semantics of Protocol IR (`protocol-ir.md`), the exact MLIR realization (`implementation-spec.md`), or the broader research opportunity map (`future-research.md`)  
**Depends on:** `system-and-scope.md`, `protocol-ir.md`, `implementation-spec.md`

---

## Abstract

This document explains **what should happen next if the current Protocol IR contribution succeeds**.

The project now has a reasonably sharp center of gravity. Arithmetic structure is not the final object. Execution strategy is not the right place to finish protocol design. Protocol IR is the semantic lock-in layer between them, and the current research contribution ends at **verified protocol instantiation before kernelization**. That center should not be diluted.

The purpose of a roadmap document is therefore not to list every interesting future idea. It is to identify the **direct continuation path** that follows most naturally from the current architecture and that yields the highest research value per layer added.

The main recommendation of this document is the following:

> **The next extension path should first make the arithmetic-to-protocol boundary precise through LLZK-backed family views, then stress-test Protocol IR with one strong family specialization—ideally STARK/oracle protocols—then add proof-relevant semantic enrichment above arithmetic IR, then expose rewrite control and cost models, and only after that move aggressively into native execution compilation and recursive/compositional bundle paths.**

This ordering is deliberate.

- **LLZK-backed arithmetic views** are the enabling interface that keeps the arithmetic side from devolving into ad hoc importers.  
- **STARK/oracle specialization** is the strongest test of whether the current common core is genuinely protocol-generic rather than quietly PCS-centric.  
- **Semantic enrichment above arithmetic IR** has the highest research upside once there is a concrete family whose protocol and AIR structure can actually benefit from it.  
- **Transform-driven control and cost models** only become convincing once the payload semantics are stable enough to support legal post-closure rewrites.  
- **Kernel lowering and DAG execution** should consume a mature closed protocol layer, not compensate for an immature one.  
- **Recursion, accumulation, and certified bundles** become most natural once verifier structure, proof ABI, and protocol profiles are already explicit.

This document is therefore intentionally narrower than `future-research.md`. The future-research document owns the broader opportunity map: distributed lowering, proof-native frontends, large autotuning spaces, alternative-assumption families, service-topology compilation, and other exploratory directions. The present document owns the **critical path** of the project.

---

## 1. Purpose

This document answers five concrete questions.

1. Which extensions are **directly adjacent** to the current Protocol IR core.  
2. In what **order** those extensions should be attempted.  
3. Which extensions are **foundational**, which are **family specializations**, which are **methodology layers**, and which are **later lateral branches**.  
4. How the roadmap should account for the project’s explicit commitment to **LLZK-backed arithmetic integration**.  
5. Which important future directions should remain outside the direct roadmap and stay in `future-research.md` instead.

The document therefore has a different role from the other canonical sources.

- `system-and-scope.md` explains the architecture and current scope.  
- `protocol-ir.md` defines the semantic thesis of the current paper.  
- `implementation-spec.md` defines the MLIR realization of that thesis.  
- `future-research.md` collects broader and more speculative research opportunities.  
- **This document** explains what should be built and published **next**, and in what dependency order.

---

## 2. What this roadmap does not reopen

A good roadmap begins by stating what is already fixed.

The following commitments should be treated as stable.

### 2.1 Protocol IR remains the semantic lock-in layer

Protocol IR is still the layer where an arithmetic object becomes a **closed prover/verifier protocol**. Future work may enrich the inputs flowing into this layer or lower its outputs into richer execution layers, but it should not relocate the semantic lock-in point.

### 2.2 Protocol instantiation still happens before kernelization

No future execution compiler should be allowed to complete transcript structure, verifier structure, proof layout, or zero-knowledge insertion implicitly. Those remain closure-time responsibilities.

### 2.3 A closed protocol still denotes the same semantic triple

The semantic center of the project remains:

$$
\llbracket P \rrbracket = (\text{proof surface},\ \text{transcript trace},\ \text{verifier relation})
$$

Future extensions may refine the object model around this triple, but they should not replace it with a purely operational or backend-centric view.

### 2.4 LLZK remains the preferred arithmetic-side substrate

The architecture now explicitly assumes that the integrated system is **LLZK-first** on the arithmetic side. Protocol IR may still speak conceptually in family terms such as R1CS, Plonkish, and AIR, but the preferred concrete source of arithmetic structure is LLZK or LLZK-derived family views, not bespoke frontends per language.

### 2.5 The current executable boundary remains the library backend

The current project ends at:

```text
zk.proto.<class>.closed | zk.proto.<class>.opt
  -> LibraryBackend
```

Any larger native execution compiler remains future work over the same closed protocol semantics.

### 2.6 The common core should remain small

The roadmap should not drift toward a giant “universal protocol language.” The current design choice remains correct: a small common protocol core, explicit feature layers, and explicit protocol profiles/classes.

These commitments are not obstacles to future work. They are the conditions that make future work scientifically coherent.

---

## 3. How this roadmap differs from the broader future-research document

The project now has two distinct kinds of future work.

### 3.1 Direct continuation work

This is work that should plausibly happen **next** if the current paper succeeds. It is adjacent to the current architecture, depends on the existing objects and invariants, and would likely produce the next one to three papers or major implementation phases.

Examples:

- LLZK-backed family views for Protocol IR import.  
- one strong family specialization, especially STARK/oracle protocols.  
- a proof-relevant semantic layer above arithmetic IR.  
- Transform-driven control for legal protocol rewrites.  
- an initial native execution compiler below Protocol IR.

### 3.2 Broader opportunity-map work

This is work that is important but not part of the immediate critical path. It often requires additional layers, new evaluation methodology, or a broader redesign space than the roadmap should commit to today.

Examples:

- extensive autotuning and search over large rewrite or schedule spaces,  
- distributed and service-topology lowering,  
- proof-native frontend DSLs,  
- very broad alternative protocol families,  
- specialized application-led frontends such as document, parser, database, or graph proof systems.

Those directions matter, but they belong in `future-research.md` because they are broader than the direct next-step roadmap.

A useful shorthand is:

> **The roadmap owns the critical path. The future-research document owns the opportunity frontier.**

---

## 4. Principles used to prioritize extensions

The order recommended here is not arbitrary. It follows five prioritization criteria.

### 4.1 Architectural leverage

An extension should be prioritized if it validates or sharpens a boundary that many later steps depend on.

LLZK-backed family views and STARK specialization score highly here because they directly test the current architecture.

### 4.2 Research differentiation

An extension should be prioritized if it makes the project look less like “yet another arithmetic IR or backend wrapper” and more like a genuinely new compiler layer.

STARK/oracle specialization, verifier-explicit closure, semantic-aware protocol compilation, and recursive-ready bundle contracts score highly here.

### 4.3 Dependency centrality

An extension should be prioritized if several later branches cannot be evaluated cleanly without it.

A stable arithmetic import contract, explicit family profiles, and a first cost-model/control layer are all dependency-central.

### 4.4 Implementation tractability

An extension should not be placed first merely because it is theoretically rich. If it requires too many immature supporting layers, it should come later.

That is why very broad autotuning, distributed lowering, or maximal family unification do not belong at the front of the roadmap.

### 4.5 Evaluation clarity

An extension should be prioritized if it comes with a clear evaluation story that can be tied back to the current claims.

For example, STARK specialization can be evaluated by explicit oracle/query/decommitment structure and verifier completeness. A semantic layer above AIR can be evaluated by trace shape, degree profile, preprocessing extraction, proof ABI stabilization, and verifier-side clarity.

---

## 5. Roadmap at a glance

The direct roadmap is best understood as **six stages plus one lateral branch family**.

| Stage | Main objective | Typical output | Why it comes here |
|---|---|---|---|
| A | Stabilize the current core and the arithmetic import boundary | canonical `Protocol IR` spine plus LLZK-backed family-view contract | every later extension needs a stable boundary |
| B | Prove that the common core is not secretly PCS-centric | one strong STARK/oracle specialization | highest-value family stress test |
| C | Preserve proof-relevant semantics above arithmetic IR | `zk.sem`-style or equivalent semantic layer for a focused family | strongest upward extension once a family target exists |
| D | Expose protocol rewrite control and cost models | `transform.zk`, legality-aware rewrite sequences, limited search | needs stable closed payload semantics |
| E | Build the first native execution compiler path | `KernelBackend -> zk.kernel -> zk.plan / zk.memplan` | should consume mature closed protocol semantics |
| F | Make protocols compositional and deployment-ready | certified bundles, recursive-readiness, accumulation/folding hooks | depends on explicit verifier, ABI, and profile structure |
| G | Expand laterally into additional protocol families | sumcheck/GKR, multilinear, folding/accumulation, alternative transparent/PQ lines | important, but not the first critical-path validation |

This gives the project a clearer shape than the earlier four-wave presentation. The previous wave model was directionally right, but it bundled too many distinct dependencies together. In particular, it underemphasized the need for a **stable LLZK-to-Protocol import layer** and for a separate **compositional/recursive bundle track**.

---

## 6. Stage A — Stabilize the current core and define the arithmetic import contract

This stage has two responsibilities.

First, it finishes the current Protocol IR paper and implementation spine. Second, it makes the arithmetic-to-protocol boundary precise enough that future specialization work does not become a collection of ad hoc importers.

### 6.1 Why this stage is foundational

The system architecture already says that Protocol IR consumes protocol intent plus arithmetic structure plus security/profile information. But that is still a **logical** description. The next extension step is to define the **practical import contract** that feeds Protocol IR from the LLZK side.

Without that contract:

- family specialization becomes frontend-specific,  
- verifier structure risks depending on accidental arithmetic encodings,  
- semantic-aware proving above arithmetic IR becomes hard to phrase precisely,  
- and later execution lowering cannot tell which facts are stable protocol semantics versus importer quirks.

### 6.2 What this stage should produce

The main deliverable is a **protocol-facing family-view interface** extracted from LLZK.

That interface should expose, at minimum:

- public, witness, fixed, and preprocessed artifact schemas;  
- relation inventories relevant to protocol construction;  
- lookup, permutation, table, RAM, or nonlocal-relation metadata where applicable;  
- degree, domain, and grouping summaries where the family requires them;  
- modular component and interface facts that matter to protocol closure;  
- shape/topology facts that may constrain proof ABI or verifier structure.

The key rule is:

> **Protocol IR should consume normalized family views, not raw frontend syntax and not raw LLZK syntax trees.**

### 6.3 Recommended design stance

This stage should preserve the conceptual family vocabulary already used across the project:

```text
LLZK
  -> normalization / validation / family analyses
  -> family-aware arithmetic view
  -> Protocol IR
```

That means the project can still speak in terms such as:

- R1CS-like view,  
- Plonkish view,  
- AIR-like view,

without implying that those views are separate frontends or separate canonical source languages.

### 6.4 AIR and STARK caveat

The current public LLZK story is strongest for circuit- and component-oriented arithmetic flows. AIR/STARK integration remains more subtle because trace semantics, oracle structure, randomized auxiliary traces, and transition locality often require richer summary objects than a circuit-centric importer normally exposes.

That is not a reason to step away from LLZK. It is a reason to make the import contract explicit and to allow the roadmap to add a semantic layer above AIR where needed.

### 6.5 Why this is not “just engineering”

A stable import contract is a research enabler. It fixes what counts as arithmetic information, what must already be normalized before protocol closure, and what can still be optimized later. That boundary influences the correctness story, the evaluation story, and the feasibility of later family specialization.

### 6.6 Recommended priority

This work should start **immediately**, in parallel with the final stabilization of the current Protocol IR core. It is not the flagship next paper by itself, but it is part of the enabling substrate for the next papers.

---

## 7. Stage B — STARK and oracle-protocol specialization

This is the most important direct extension after the current core.

### 7.1 Why STARK is the right first specialization

The current Protocol IR design is strongest if it survives contact with a family whose natural protocol objects are visibly **not** just PCS-flavored commitments and polynomial openings.

STARK-oriented systems expose exactly the objects that test this.

- oracle commitments,  
- query surfaces,  
- decommitment bundles,  
- Merkle path planning,  
- OODS/DEEP-style reductions,  
- FRI plans and low-degree testing schedules,  
- auxiliary-trace commitments and verifier logic.

If the current common core can support these cleanly through feature layering and protocol profiles, the architecture becomes substantially more credible.

### 7.2 The design target

The right target is not a monolithic STARK dialect that swallows the entire protocol layer.

The correct design remains:

```text
zk.proto.core
  + zk.proto.oracle
  + zk.proto.merkle
  + zk.proto.fri
  + zk.proto.oods
  + zk.verify
  => protocol_class = stark
```

In other words, STARK should appear as a **profiled assembly of reusable feature layers**, not as proof that the common core was too small.

### 7.3 What this stage should produce

A serious Stage B should deliver:

1. a focused `stark` profile with explicit closure rules;  
2. oracle-root, query-surface, decommitment, FRI, and OODS objects as first-class protocol components;  
3. STARK-specific verifier fragments and completeness checks;  
4. at least one library-backed vertical slice for a focused STARK family;  
5. one initial cost model for protocol-level choices such as commitment grouping, decommitment structure, and FRI/query configuration.

The goal is not to build a universal STARK compiler immediately. The goal is to show that Protocol IR can represent a real oracle-heavy family without retreating into backend-side conventions.

### 7.4 Relationship to the arithmetic side

This stage should consume the best available arithmetic substrate.

In the near term that likely means one of two things:

- a focused LLZK-backed view where circuit-like arithmetic structure is sufficient, or  
- a deliberately narrow STARK-facing arithmetic/semantic summary where trace structure and randomized AIR facts are explicit enough.

This is precisely why Stage A and Stage B belong next to each other.

### 7.5 Why this stage precedes the semantic layer above arithmetic IR

A semantic layer above AIR is attractive, but its strongest claims are family-sensitive. It becomes much easier to decide what semantics should survive lowering once there is a concrete protocol target whose structure has been made explicit.

For this reason, STARK specialization should come **before** any very broad attempt at a proof-relevant semantic IR.

### 7.6 Recommended priority

This is the **highest-priority next paper/program step** after the current core.

---

## 8. Stage C — Semantic enrichment above arithmetic IR

Once one strong family specialization exists, the next major upward move is to preserve proof-relevant semantics above arithmetic IR.

### 8.1 Why this stage matters

If the compiler starts from raw R1CS, raw Plonkish, or raw AIR alone, much of the structure that could improve protocol construction has already been erased.

That loss is especially expensive in STARK-oriented systems, where trace shaping, auxiliary-trace synthesis, column-role inference, relation selection, and verifier structure all depend on properties that are closer to program or machine semantics than to a fully flattened arithmetic representation.

### 8.2 The right role of `zk.sem`

The semantic layer should remain narrow. It should not try to preserve full source-language meaning.

It should preserve only the semantic facts that can still change:

- arithmetization shape,  
- protocol shape,  
- verifier structure,  
- proof ABI stability,  
- or downstream execution consequences.

Examples include:

- state schema,  
- resource kind,  
- relation kind,  
- transition locality,  
- control regularity,  
- static proof-shape constraints.

### 8.3 The right place in the roadmap

The semantic layer should not be treated as a prerequisite for everything else. It is a **second-wave upward extension**.

The correct formulation is roughly:

```text
source / VM semantics
  -> zk.sem
  -> LLZK-backed arithmetic view or zk.air
  -> zk.proto.<class>
```

with the understanding that the semantic layer exists only where it yields measurable protocol or proving leverage.

### 8.4 What this stage should produce

A strong Stage C should demonstrate at least one family-specific semantic pipeline, ideally STARK-oriented, and show that the retained semantics materially improve:

- trace decomposition,  
- auxiliary-trace construction,  
- relation selection,  
- degree profile,  
- commitment/query structure,  
- proof-surface stability,  
- verifier completeness or clarity.

### 8.5 Why this is one of the best research opportunities in the project

This stage is where the compiler can begin to exploit information that existing arithmetic-only pipelines typically discard. That makes it one of the most research-differentiating parts of the long-term program.

### 8.6 Recommended priority

This should be the **next major upward paper or extension** after the first family specialization.

---

## 9. Stage D — Transform control, cost models, and disciplined protocol search

Once the project has stable closed protocols and at least one rich family profile, it should expose a proper control plane for legal protocol rewrites.

### 9.1 Why this stage comes after specialization and semantic stabilization

A Transform layer is only as meaningful as the payload semantics it controls. If the protocol objects, verifier fragments, ABI contracts, and family closure rules are not yet stable, a transformation control plane becomes an elaborate wrapper around moving targets.

That is why this stage is important but should not come first.

### 9.2 The design rule remains unchanged

`zk.proto` remains the payload semantics.  
`transform.zk` becomes the control plane.

This is the same architectural lesson emphasized by the MLIR Transform Dialect literature: separate the object being transformed from the IR that describes how transformations should be orchestrated.

### 9.3 What this stage should produce

A useful first version of this stage should deliver:

- named legal rewrite sequences over closed protocols;  
- explicit preconditions and postconditions for major rewrite families;  
- ABI-aware rewrite classification;  
- cost models for proof size, prover time, memory pressure, and perhaps recursion-readiness;  
- limited search or autotuning over a bounded rewrite space.

The emphasis should be on **disciplined, certificate-aware protocol optimization**, not on throwing a very large search problem at the compiler immediately.

### 9.4 Relationship to AI-compiler methodology

This stage is exactly where lessons from Halide, TVM, and Exo become useful.

- Halide is valuable because it made the algorithm/schedule separation explicit.  
- TVM is valuable because it turned optimization spaces into objects that can be searched.  
- Exo is valuable because it externalizes hardware mapping and low-level optimization strategy rather than burying them in monolithic compilers.

The project should adopt the lesson, but at the **protocol** layer first: separate protocol semantics from protocol rewrite control before attempting very large search spaces.

### 9.5 What should remain deferred

The roadmap should not yet promise:

- large reinforcement-learning-guided tuning,  
- cluster-wide distributed schedule search,  
- or open-ended multi-objective search over protocol plus kernel plus deployment space.

Those belong in `future-research.md` until there is a stable transform/control substrate and a meaningful cost model.

### 9.6 Recommended priority

This stage should begin once one strong family profile is mature. It is best treated as a **methodology extension** that supports both later execution lowering and later protocol-family expansion.

---

## 10. Stage E — Native execution compiler and DAG-oriented kernel lowering

This stage is where the broader prover-compiler vision becomes real.

### 10.1 Why this stage matters

The current `LibraryBackend` path is correct as a semantic boundary check, but it is not the end state of the project. A serious prover compiler eventually needs a native path:

```text
zk.proto.<class>.opt
  -> KernelBackend
  -> zk.kernel
  -> zk.plan / zk.memplan
  -> bundle / runtime replay
```

The central rule, however, must remain unchanged:

> **The execution compiler consumes an already closed protocol.**

### 10.2 Why this stage does not come earlier

Execution lowering is one of the most engineering-heavy parts of the long-term stack. If introduced too early, it obscures the semantic contribution and tempts the system to finish protocol design “where convenient” inside backend code.

That is the wrong direction.

### 10.3 What this stage should produce

A first serious execution stage should define:

- a `KernelBackend` contract over closed protocols;  
- an initial `zk.kernel` model that distinguishes arithmetic, oracle, FRI, Merkle, transcript, and proof-surface-facing kernels where necessary;  
- an initial DAG-structured scheduling view;  
- plan/memplan objects that reason about memory pressure, staging, and replay;  
- one hardware-aware vertical slice demonstrating that execution lowering preserves the same closed protocol semantics as the library backend.

### 10.4 Why DAG kernels matter here

The future-research document goes much deeper on DAG kernels. The roadmap only needs the following direct point:

- once protocols become explicit graphs of commitments, reductions, transcripts, decommitments, checks, and kernels, a DAG-oriented execution representation is likely more natural than a purely linear “call this library next” model.

That is enough to justify a DAG-shaped execution stage without pulling the roadmap into the full research space of distributed scheduling, service lowering, or cross-proof superbatching.

### 10.5 Recommended priority

This is the **main systems extension**, but it should begin only after the protocol layer has at least one mature specialized family and an initial control/cost framework.

---

## 11. Stage F — Stronger verifier contracts, certified bundles, and recursive/compositional readiness

This stage cuts across several others, but it has enough internal coherence to deserve its own place in the roadmap.

### 11.1 Why it belongs in the direct roadmap

The current project already treats verifier structure and proof ABI as first-class semantic objects. That creates a natural opening for three concrete extensions:

1. **richer verifier contracts**,  
2. **bundle-facing certificates**,  
3. **recursive and compositional protocol interfaces**.

These are not arbitrary add-ons. They are precisely what explicit verifier IR and proof ABI are good for.

### 11.2 Near-term bundle and certificate work

Before full recursion or accumulation is attempted, the roadmap should strengthen bundle artifacts.

Useful near-term outputs include:

- proof-ABI hashes or version commitments;  
- discharged-obligation digests;  
- protocol-profile and security-profile metadata;  
- verifier-ABI fingerprints;  
- bundle-level evidence that a closed protocol passed specified legality checks.

This is the practical compiler analogue of proof-carrying artifacts: not a fully formal proof of correctness, but explicit machine-readable evidence that certain closure and compatibility conditions were discharged.

### 11.3 Why this helps later recursion

Recursion and accumulation are easiest to reason about when:

- verifier structure is explicit,  
- proof readback is typed,  
- transcript replay is well-defined,  
- proof ABI is stable or explicitly versioned.

Those are already first-class design goals of Protocol IR. So the roadmap should not treat recursion as a completely unrelated domain.

### 11.4 What the later recursive/compositional track should produce

A serious later version of this stage should aim for:

- recursion-readiness checking for closed protocols;  
- recursive-friendly verifier/readback forms;  
- proof-ABI discipline for recursive proof objects;  
- protocol interfaces for accumulation or folding profiles;  
- eventually, support for IVC/PCD-oriented bundle composition.

### 11.5 Why this still comes after the first family specialization

Although recursive composition is strategically important, it should not be the first extension after the current paper. Full recursion, folding, and accumulation are themselves rich protocol families. They deserve an explicit place in the roadmap, but not at the cost of skipping the more immediate STARK/oracle validation or the semantic layer above arithmetic.

### 11.6 Recommended priority

- **Bundle certificates and stronger verifier/ABI contracts** can progress in parallel with medium-term roadmap stages.  
- **Full recursion/folding/accumulation integration** should follow only after at least one mature specialized family and a stable verifier/ABI story.

---

## 12. Stage G — Lateral family expansion after the first specialization

Once the project has one mature specialized profile and one mature execution direction, it becomes meaningful to broaden the family axis.

This should be done carefully.

### 12.1 Why this is a lateral branch, not the first critical path

The current architecture must first prove that it can support one demanding protocol family cleanly. Only then does broader family expansion become a strong scientific statement rather than a diffuse aspiration.

### 12.2 The most promising lateral branches

Three branches stand out.

#### A. Sumcheck, GKR, and multilinear systems

This line is strategically important because it tests a different notion of proving structure than the usual R1CS/Plonkish/AIR picture. Systems such as Spartan, Virgo, and Jolt make it clear that multilinear, sumcheck-heavy, and GKR-like flows deserve first-class treatment rather than being treated as awkward encodings of circuit-style families.

#### B. Folding, accumulation, and IVC

This line is important because it turns verifier structure and recursive composition into central compilation objects. Nova, HyperNova, ProtoStar, and later accumulation work suggest that explicit verifier IR and profile-carrying protocol closure could become unusually valuable here.

#### C. Alternative transparent and post-quantum families

This line includes binary-field, code-based, VOLE-based, and MPC-in-the-Head families. It is strategically important for long-term breadth and post-quantum diversity, but it is not the right place to start broadening unless the project already has a convincing family-generic core.

### 12.3 The conceptual lesson

This stage reinforces a key long-term insight that becomes more obvious after the first family expansion:

> **Statement families and protocol families are not the same axis.**

That deeper observation belongs to `future-research.md`, but the roadmap already benefits from acknowledging it. Sumcheck/GKR, folding/accumulation, and STARK/oracle systems do not fit neatly into a single “arithmetic family only” taxonomy.

### 12.4 Recommended ordering inside the lateral branch

If lateral expansion is pursued, the strongest order is:

1. **sumcheck/GKR/multilinear**  
2. **folding/accumulation/IVC**  
3. **alternative transparent / post-quantum families**

This order keeps the broadening path tied to the project’s current semantic strengths rather than jumping immediately to the most exotic families.

---

## 13. Dependencies between roadmap stages

The roadmap is not a strictly linear chain. Some stages can overlap. But the dependencies are still strong enough to state explicitly.

A useful dependency sketch is:

```text
Current core stabilization
  -> LLZK-backed family views
  -> first family specialization (ideally STARK/oracle)
      -> semantic layer above arithmetic IR
      -> transform/cost-model control plane
      -> stronger verifier / bundle certificates
      -> initial native execution compiler
          -> recursion/compositional bundle path
          -> broader lateral family expansion
```

More precisely:

- **Stage A** enables everything else.  
- **Stage B** is the first real stress test of the architecture.  
- **Stage C** depends on having at least one family whose semantics can profit from retention above arithmetic IR.  
- **Stage D** depends on mature closed payload semantics and at least one meaningful optimization target.  
- **Stage E** depends on stable closed protocol objects and at least an initial cost-model/control story.  
- **Stage F** depends on verifier explicitness and proof ABI maturity, though the certificate sub-track can start earlier.  
- **Stage G** should not dominate effort until the project has at least one convincing specialized family and at least an initial execution path.

---

## 14. What should be prioritized next

If the goal is to maximize research value while preserving architectural clarity, the next priorities should be:

### Highest priority

1. **Stabilize the current Protocol IR core and its canonical documentation.**  
2. **Define the LLZK-backed family-view contract that feeds Protocol IR.**  
3. **Push one strong family specialization, ideally STARK/oracle protocol structure.**

These three steps are the foundation. They validate the boundary, the import interface, and the generality of the common core.

### Medium priority

4. **Add a proof-relevant semantic layer above arithmetic IR where it yields real protocol leverage.**  
5. **Strengthen verifier contracts, bundle metadata, and proof-ABI discipline.**  
6. **Prototype `transform.zk` over a bounded space of legal post-closure rewrites.**

These steps deepen the compiler scientifically without yet requiring the full execution stack.

### Later but still direct-roadmap priority

7. **Build the first native `KernelBackend` path and DAG-oriented execution layer.**  
8. **Use the richer verifier/ABI story to support recursive-readiness and compositional bundle work.**

### Lower priority for now

9. **Very broad search or autotuning spaces.**  
10. **Distributed and service-topology lowering.**  
11. **Broad alternative-family expansion without a first mature specialized case.**  
12. **Reopening the common core or collapsing boundaries between arithmetic, protocol, and execution layers.**

This preserves a strong sequence:

> **boundary first, specialization second, semantic enrichment third, methodology fourth, execution fifth, broadening sixth.**

---

## 15. Recommended paper and program sequence

A coherent publication and implementation sequence would look like this.

### Paper / Stage A
**Verified Protocol Instantiation Before Kernelization**

- Protocol IR semantics  
- explicit verifier  
- proof ABI  
- closure contracts and obligations  
- one executable library-backed vertical slice

This is the current paper.

### Paper / Stage B
**LLZK-Backed Family Views and STARK/Oracle Protocol Specialization**

- arithmetic-to-protocol import contract  
- oracle, Merkle, FRI, OODS structure  
- STARK-specific closure and verifier completeness  
- one focused specialized case study

This is the strongest next direct extension.

### Paper / Stage C
**Semantic-Aware Protocol Compilation Above Arithmetic IR**

- proof-relevant semantic layer  
- AIR or arithmetic shaping  
- semantic-guided protocol construction  
- verifier and ABI consequences of retained semantics

This is the strongest upward extension.

### Paper / Stage D
**Protocol Rewrite Control and Initial Native Execution Lowering**

- `transform.zk` control plane  
- cost models and legality-aware rewrite scheduling  
- first `KernelBackend` and `zk.kernel` slice  
- preservation of the closed semantic triple through execution lowering

This is the first major methodology-plus-systems paper.

### Paper / Stage E
**Certified Bundles and Recursive/Compositional Protocol Interfaces**

- stronger verifier and proof-readback contracts  
- bundle-facing certificates  
- recursive-readiness / accumulation-ready profiles  
- initial compositional proof path

This is the natural extension of explicit verifier IR and proof ABI.

### Later lateral papers

After the above stages, the strongest lateral papers are likely:

- **sumcheck/GKR/multilinear protocol families**,  
- **folding/accumulation/IVC as protocol-family extensions**,  
- **broader alternative transparent / post-quantum family support**.

This sequence is both conceptually coherent and tractable. It avoids prematurely turning the project into an unbounded “everything” agenda.

---

## 16. What should remain outside the direct roadmap for now

To keep the project coherent, several important ideas should remain outside the direct roadmap until earlier stages stabilize.

### 16.1 Broad frontend DSL work

Protocol-intent DSLs, proof-native frontends, and domain-specific frontend languages are important, but they should not take priority over getting the protocol boundary and first specialized family right.

### 16.2 Large autotuning and search spaces

Search becomes much more interesting once there is a stable control plane, a clear kernel model, and meaningful cost models. Before that, it risks becoming a large engineering problem with weak semantic grounding.

### 16.3 Multi-machine and service-topology lowering

Distributed lowering, proving services, collaborative proving, and checkpointed service orchestration are important long-term directions, but they depend on a mature execution compiler and a mature bundle model.

### 16.4 Very broad family unification

The project should resist the temptation to unify all families at once. Broad family support becomes convincing only after one hard family, one semantic extension, and one execution path have already succeeded.

### 16.5 Alternative-assumption expansion without a core validation story

Binary-field, code-based, MPCitH, VOLE, and other post-PCS branches are strategically important, but they should come after the architecture has proven itself on a more direct critical path.

These are not rejections. They are sequencing decisions.

---

## 17. Open questions that still matter for the roadmap

The following questions are the most important ones still left open at roadmap level.

### 17.1 Arithmetic-to-protocol boundary

1. What is the smallest but sufficient family-view interface that LLZK should export to Protocol IR?  
2. Which arithmetic facts should remain inside LLZK analyses, and which should become explicit protocol-facing summary objects?  
3. For AIR/STARK-oriented systems, how much structure should be pushed into LLZK-derived views versus a separate `zk.sem` layer?

### 17.2 Protocol-family structure

4. How small can the common protocol core stay while still supporting meaningful generic analyses and rewrites?  
5. Which STARK concepts should be first-class protocol objects, and which should remain derived views or lowered plans?  
6. What is the right balance between protocol-class profiles and family façade dialects?

### 17.3 Semantic enrichment

7. Which semantic facts above arithmetic IR most directly improve verifier structure and proof ABI, not just prover cost?  
8. How should semantic facts flow jointly into arithmetic shaping and protocol closure?  
9. What is the minimal semantic vocabulary that demonstrates real leverage without turning `zk.sem` into a second full frontend language?

### 17.4 Control and cost models

10. Which protocol-level rewrites deserve first-class Transform control earliest?  
11. How should cost models be decomposed between protocol-level meaning and backend-level execution?  
12. What forms of limited search are worth introducing before the project has a mature kernel compiler?

### 17.5 Execution and composition

13. What should the first `zk.kernel` abstraction boundaries be?  
14. Which parts of proof-surface structure should remain protocol-level forever, and which may become execution-planning objects?  
15. What is the right contract for recursive-readiness, accumulation-readiness, or bundle-level certification over a closed protocol?

These questions are not reasons to delay the roadmap. They are the questions the roadmap is supposed to answer incrementally.

---

## 18. Final position

The cleanest one-sentence summary of this document is:

> **The project should now move from “Protocol IR exists” to “Protocol IR has a stable arithmetic import boundary, one demanding family specialization, one upward semantic extension, one control-plane methodology, one downward execution path, and one compositional bundle story.”**

That is the right shape for the next phase.

More concretely, the roadmap should be read as follows:

1. **stabilize the current core**,  
2. **make the LLZK-to-Protocol boundary precise**,  
3. **specialize first for STARK/oracle protocols**,  
4. **add semantic leverage above arithmetic IR**,  
5. **expose disciplined rewrite control**,  
6. **lower to a native execution compiler**,  
7. **then broaden into recursive/compositional and additional family directions**.

That ordering gives the project the best chance of producing a coherent series of results rather than a scattered collection of partially connected ideas.

---

## Selected references

### Canonical project documents

- `system-and-scope.md`
- `protocol-ir.md`
- `implementation-spec.md`
- `future-research.md`
- `Semantic-Aware-Protocol-Compilation-for-STARK-Provers.txt`
- `fiat-shamir-in-protocol-ir-research-note.md`

### Compiler architecture and adjacent MLIR methodology

- [MLIR] Chris Lattner et al. *MLIR: Scaling Compiler Infrastructure for Domain-Specific Computation*. CGO 2021.
- [Dialect Conversion] LLVM MLIR documentation. *Dialect Conversion*.
- [Symbols] LLVM MLIR documentation. *Symbols and Symbol Tables*.
- [Side Effects] LLVM MLIR documentation. *Side Effects & Speculation*.
- [Transform Paper] Martin Paul Lücke et al. *The MLIR Transform Dialect: Your compiler is more powerful than you think*. CGO 2025.
- [Transform Docs] LLVM MLIR documentation. *Transform Dialect*.
- [Verification Dialects] Mathieu Fehr et al. *First-Class Verification Dialects for MLIR*. PLDI 2025.
- [HEIR] Asra Ali et al. *HEIR: A Universal Compiler for Homomorphic Encryption*. 2025.
- [Halide] Jonathan Ragan-Kelley et al. *Halide: A Language and Compiler for Optimizing Parallelism, Locality, and Recomputation in Image Processing Pipelines*.
- [TVM] Tianqi Chen et al. *TVM: An Automated End-to-End Optimizing Compiler for Deep Learning*.
- [Exo] Yu Feng Ikarashi et al. *Exocompilation for Productive Programming of Hardware Accelerators*.

### Arithmetic IRs and LLZK integration

- [LLZK Overview] LLZK documentation.
- [LLZK Repo] LLZK repository.
- [LLZK Blog] Veridise. *Announcing LLZK*.
- [LLZK Grant] Veridise. *Ethereum Foundation grant for LLZK*.
- [CirC] Alex Ozdemir, Fraser Brown, Riad S. Wahby. *CirC: Compiler Infrastructure for Proof Systems, Software Verification, and More*.
- [Noir ACIR] Noir documentation.
- [zkInterface] zkInterface / SIEVE interoperability references.

### STARK, AIR, and oracle-protocol structure

- [Winterfell] Winterfell AIR documentation.
- [RISC Zero] RISC Zero proof-system documentation.
- [AirScript] AirScript repository.

### Compositional and later protocol-family lines

- [Spartan] Srinath Setty. *Spartan: Efficient and General-Purpose zkSNARKs Without Trusted Setup*.
- [Virgo] Jiaheng Zhang et al. *Transparent Polynomial Delegation and Its Applications to Zero Knowledge Proof*.
- [Jolt] Arasu Arun, Srinath Setty, Justin Thaler. *Jolt: SNARKs for Virtual Machines via Lookups*.
- [Nova] Abhiram Kothapalli, Srinath Setty, Ioanna Tzialla. *Nova: Recursive Zero-Knowledge Arguments from Folding Schemes*.
- [HyperNova] Abhiram Kothapalli and Srinath Setty. *HyperNova: Recursive Arguments for Customizable Constraint Systems*.
- [ProtoStar] Benedikt Bünz and Binyi Chen. *ProtoStar: Generic Efficient Accumulation/Folding for Special-Sound Protocols*.
- [Sumcheck Survey] Justin Thaler. *Sum-check Is All You Need: An Opinionated Survey on Fast Provers in SNARK Design*.
- [Accumulation without Homomorphism] Benedikt Bünz et al. *Accumulation Without Homomorphism*.
- [Arc] Benedikt Bünz et al. *Arc: Accumulation for Reed–Solomon Codes*.
- [DeepFold] Yanpei Guo et al. *DeepFold: Efficient Multilinear Polynomial Commitment from Reed-Solomon Code and Its Application to Zero-Knowledge Proofs*.

### Validation and assurance

- [SNARK SoK 2024] Stefanos Chaliasos et al. *SoK: What Don’t We Know? Understanding Security Risks of SNARKs*.
- [SNARK SoK 2025] Junkai Liang et al. *SoK: Understanding zk-SNARKs — The Gap Between Research and Practice*.
- [MTZK] Dongwei Xiao et al. *MTZK: Testing and Exploring Bugs in Zero-Knowledge (ZK) Compilers*.
- [Circuzz] Christoph Hochrainer et al. *Fuzzing Processing Pipelines for Zero-Knowledge Circuits*.

---

## Reference links

[MLIR]: https://research.google/pubs/mlir-scaling-compiler-infrastructure-for-domain-specific-computation/
[Dialect Conversion]: https://mlir.llvm.org/docs/DialectConversion/
[Symbols]: https://mlir.llvm.org/docs/SymbolsAndSymbolTables/
[Side Effects]: https://mlir.llvm.org/docs/Rationale/SideEffectsAndSpeculation/
[Transform Paper]: https://dl.acm.org/doi/10.1145/3696443.3708922
[Transform Docs]: https://mlir.llvm.org/docs/Dialects/Transform/
[Verification Dialects]: https://users.cs.utah.edu/~regehr/papers/pldi25.pdf
[HEIR]: https://arxiv.org/abs/2508.11095
[Halide]: https://people.csail.mit.edu/jrk/halide-pldi13.pdf
[TVM]: https://www.usenix.org/system/files/osdi18-chen.pdf
[Exo]: https://dl.acm.org/doi/10.1145/3519939.3523446

[LLZK Overview]: https://project-llzk.github.io/llzk-lib/
[LLZK Repo]: https://github.com/project-llzk/llzk-lib
[LLZK Blog]: https://veridise.com/blog/zero-knowledge/announcing-llzk-a-unified-open-source-intermediate-representation-ir-for-zero-knowledge-languages/
[LLZK Grant]: https://veridise.com/blog/veridise-announcements/veridise-secures-ethereum-foundation-grant-to-develop-llzk-a-new-intermediate-representation-ir/
[CirC]: https://eprint.iacr.org/2020/1586
[Noir ACIR]: https://noir-lang.org/docs/
[zkInterface]: https://docs.zkproof.org/pages/standards/accepted-workshop3/proposal-zkinterface.pdf

[Winterfell]: https://docs.rs/winterfell/latest/winterfell/trait.Air.html
[RISC Zero]: https://github.com/risc0/risc0/blob/main/website/docs/proof-system/proof-system-sequence-diagram.md
[AirScript]: https://github.com/0xMiden/air-script

[Spartan]: https://eprint.iacr.org/2019/550
[Virgo]: https://eprint.iacr.org/2019/1482
[Jolt]: https://eprint.iacr.org/2023/1217
[Nova]: https://eprint.iacr.org/2021/370
[HyperNova]: https://eprint.iacr.org/2023/573
[ProtoStar]: https://eprint.iacr.org/2023/620
[Sumcheck Survey]: https://eprint.iacr.org/2025/2041
[Accumulation without Homomorphism]: https://eprint.iacr.org/2024/474
[Arc]: https://eprint.iacr.org/2024/1731
[DeepFold]: https://www.usenix.org/system/files/usenixsecurity25-guo-yanpei.pdf

[SNARK SoK 2024]: https://www.usenix.org/system/files/usenixsecurity24-chaliasos.pdf
[SNARK SoK 2025]: https://www.usenix.org/system/files/usenixsecurity25-liang-sok.pdf
[MTZK]: https://www.ndss-symposium.org/ndss-paper/mtzk-testing-and-exploring-bugs-in-zero-knowledge-zk-compilers/
[Circuzz]: https://www.mariachris.github.io/Pubs/CCS-2025.pdf
