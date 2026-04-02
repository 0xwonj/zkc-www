# Future Research

**Status:** curated future-research document
**Owns:** medium- and long-term research directions beyond the current Protocol IR paper; speculative and exploratory extensions; cross-cutting research hypotheses; candidate future paper/program themes
**Does not own:** the canonical overview and current scope (`system-and-scope.md`), the current Protocol IR thesis (`protocol-ir.md`), the current MLIR realization (`implementation-spec.md`), or the prioritized roadmap (`extensions-and-roadmap.md`)
**Depends on:** `system-and-scope.md`, `protocol-ir.md`, `implementation-spec.md`, `extensions-and-roadmap.md`, `Semantic-Aware-Protocol-Compilation-for-STARK-Provers.txt`

---

## Abstract

This document expands the project’s future research space **beyond the current Protocol IR paper**.

The current contribution is intentionally narrow and should remain narrow: Protocol IR is the semantic lock-in layer between arithmetic structure and execution structure; protocol instantiation happens before kernelization; and the current executable boundary is a library backend. That is the correct first paper.

But once that boundary is established, the larger research program becomes much broader. The most important next questions are no longer only about protocol closure itself. They are about:

1. **how to lower a closed protocol into a native execution compiler**, especially a DAG-structured kernel system;
2. **how to preserve and exploit program semantics not only during arithmetization but also during proving**, especially in STARK-oriented systems;
3. **how to extend the architecture beyond the familiar arithmetic families** of R1CS, Plonkish, and AIR into multilinear/sumcheck/GKR, folding/accumulation/IVC, code-based and binary-field systems, MPC-in-the-Head and VOLE-style proofs, and structured-data proof systems;
4. **how to build typed frontends, autotuning methods, and distributed lowering strategies** that remain security-aware and semantics-preserving.

The central proposal of this document is that future work should move from a **one-dimensional view** of ZK compilation to a **multi-axis view**:

- **statement family**: what kind of proving object we start from,
- **protocol family**: what kind of proof protocol/reduction we instantiate,
- **execution family**: what kind of machine, cluster, or service runs the closed protocol.

That shift matters because many of the most important emerging systems do not fit cleanly into a single arithmetic bucket. Sumcheck, GKR, folding, accumulation, binary-field code-based systems, structured document proofs, and delegated or collaborative proving all cut across the simple R1CS/Plonkish/AIR taxonomy. A serious long-term ZK compiler should acknowledge that explicitly.

This document therefore proposes a broad future program around **DAG kernels, semantic-aware proving, protocol-family expansion, typed DSLs, autotuning, heterogeneous and distributed lowering, and certifying compilation**. It also adds several novel hypotheses not yet fixed in the main documents: a two-axis statement/protocol taxonomy, challenge-barrier-aware DAG scheduling, proof-surface-aware memory planning, recursion-aware proof ABI, and certificate-constrained autotuning.

---

## 1. Purpose

This document exists for a different reason than `extensions-and-roadmap.md`.

The roadmap document answers: **what should be prioritized next** and **what the medium-term wave structure looks like**. This document answers a broader question:

> **If Protocol IR becomes stable, what are the deepest and highest-value research directions that open up around it?**

That includes directions that are:
- too broad for the current paper,
- too speculative for the canonical roadmap,
- or orthogonal enough that they deserve their own conceptual home.

The intended role of this document is to be the project’s **research opportunity map**. It should help answer:

1. what new compiler layers might exist below Protocol IR;
2. what new semantic layers might exist above arithmetic IR;
3. what new protocol families the architecture should eventually support;
4. what methodologies from AI compilers, typed/effectful languages, formal methods, and distributed systems should inform the design;
5. what future papers could be written without destabilizing the current contribution.

---

## 2. Position relative to the current architecture

The current architecture already fixes one crucial boundary:

```text
arithmetic structure
-> Protocol IR
-> backend lowering
```

That boundary should remain fixed.

Future work should therefore avoid two opposite mistakes.

First, it should avoid pushing execution planning, kernel fusion, distributed scheduling, and hardware mapping *upward* into Protocol IR itself.

Second, it should avoid pushing transcript discipline, proof ABI, verifier structure, or closure obligations *downward* into arithmetic IR or execution backends.

The right long-term program is instead:

- **enrich upward** where proof-relevant semantics can still change arithmetization or protocol shape;
- **specialize sideways** where protocol families require richer protocol objects;
- **lower downward** where a fixed closed protocol can become kernels, schedules, memory plans, and runtime artifacts;
- **expand outward** where the system must target clusters, proving services, or collaborative/offloaded proving models.

This document is organized around those four movements.

---

## 3. A key conceptual refinement: statement families are not protocol families

A major future refinement is to stop treating all important distinctions as if they lived on one axis.

Today it is natural to speak in terms of arithmetic families such as:
- R1CS,
- Plonkish,
- AIR.

That is still correct for the current paper. But future work needs a sharper taxonomy, because many important systems do **not** fit cleanly into a single arithmetic bucket.

The better long-term view is a **two-axis taxonomy**.

### 3.1 Statement families

A **statement family** answers: what kind of proving object is being established?

Representative examples include:
- R1CS-like relations,
- Plonkish relation systems,
- AIR / execution traces,
- CCS-like customizable constraint systems,
- multilinear relations and zero-check claims,
- codeword/proximity claims,
- RAM and table relations,
- structured document / parser claims,
- graph/database operator claims,
- recursive accumulator states.

These are mostly “what is being proved” categories.

### 3.2 Protocol families

A **protocol family** answers: what kind of proving reduction or interaction structure is used?

Representative examples include:
- polynomial IOP + PCS,
- oracle/FRI/Merkle protocols,
- sumcheck / zerocheck / GKR,
- folding / accumulation / IVC / PCD,
- lookup and memory-check arguments,
- MPC-in-the-Head / VOLE / OT-based protocols,
- sigma / IPA / discrete-log interactive arguments,
- recursive wrapper or aggregation protocols.

These are “how proving proceeds” categories.

### 3.3 Why this distinction matters

The distinction matters because many modern systems are hybrid.

Examples:
- an AIR-like or trace-like statement may use FRI today, but could later be wrapped by accumulation or recursion;
- a layered computation graph may be most naturally handled by GKR and sumcheck rather than by flattening to R1CS;
- a Plonkish or CCS statement may become the *inner* relation of a folding or accumulation scheme;
- a code-based or binary-field system may share accumulation or recursion structure with systems that do not share its arithmetic representation;
- a document proof such as regex or CFG parsing is not naturally introduced by “R1CS versus AIR”, even though it can eventually be encoded arithmetically.

This suggests an important compiler hypothesis:

> **The long-term compiler should represent statement-family structure and protocol-family structure as separate but composable layers.**

That is a stronger and more future-proof architecture than any single universal “arithmetic family” taxonomy.

### 3.4 Compiler consequence

In the long run, the project may need interfaces or dialect layers such as:

- statement-side:
- `zk.r1cs`
- `zk.plonkish`
- `zk.air`
- `zk.ccs`
- `zk.mle`
- `zk.code`
- `zk.doc`
- `zk.mem`

- protocol-side:
- `zk.proto.pcs`
- `zk.proto.oracle`
- `zk.proto.fri`
- `zk.proto.sumcheck`
- `zk.proto.gkr`
- `zk.proto.fold`
- `zk.proto.accum`
- `zk.proto.mpcit`
- `zk.proto.vole`

The current project should not build all of these now. But future work should think in these terms.

---

## 4. DAG kernels and the native execution compiler

One of the highest-value future directions is the development of a **native execution compiler** below Protocol IR.

The current backend path is:

```text
Protocol IR -> LibraryBackend
```

The long-term path should be:

```text
Protocol IR
-> DAG kernel IR
-> kernel IR
-> plan / memplan
-> runtime replay
```

### 4.1 Why a DAG kernel layer is likely necessary

A future `zk.kernel` could in principle be a flat sequence of kernels. But a flat sequence is unlikely to be the best long-term abstraction.

Modern proving systems are naturally **DAG-shaped**:
- commitments depend on witness or trace materialization,
- openings depend on query plans,
- FRI or folding stages depend on prior commitments and challenges,
- recursive wrappers depend on verifier fragments,
- large proofs are often assembled from semi-independent subgraphs,
- batch proving introduces cross-instance parallelism and shared preprocessing.

This strongly suggests a graph-like intermediate layer—call it `zk.kgraph` or a DAG-kernel sublayer—between closed protocol semantics and low-level device kernels.

### 4.2 Why AI compiler frameworks are relevant

AI compiler work is not directly about cryptography, but it is highly relevant methodologically.

- **Halide** showed the power of separating algorithm from schedule and of making scheduling a first-class optimization object.
- **TVM**, **Ansor**, and **MetaSchedule** showed how to expose large tuning spaces and search them automatically.
- **Exo** showed the value of externalizing low-level mapping and optimization policy rather than hiding it in a monolithic compiler.
- **IREE** and **OpenXLA/GSPMD** showed how MLIR-native systems can represent execution flow, partitioning, and heterogeneous runtime concerns.
- **FlexFlow** and **Alpa** showed that graph partitioning and distributed planning can themselves become compiler problems.

The key lesson is not “copy an AI compiler”. The lesson is:

> **treat proving as a structured compute graph with domain-specific constraints, then build an explicit control/schedule/search layer around it.**

### 4.3 What is different about proof DAGs

A proof DAG is not an ordinary tensor graph.

At least four extra semantic features matter:

1. **challenge barriers**
 After transcript challenges are sampled, all downstream work is semantically constrained by those sampled values. This introduces hard anti-speculation boundaries not found in ordinary pure numeric graphs.

2. **proof-surface sinks**
 Some intermediate results are not just temporary values; they are proof-visible objects that must be emitted, preserved, versioned, and sometimes replayed by the verifier.

3. **verifier-coupled structure**
 Certain subgraphs correspond directly to verifier obligations, recursive wrappers, or proof readback discipline.

4. **protocol legality constraints**
 A schedule that changes the visible transcript or proof shape is not merely “a different execution”. It may be a different protocol.

This suggests a future execution compiler should reason about more than dataflow alone.

### 4.4 Candidate object model for a DAG kernel layer

A future DAG-kernel layer should probably make explicit:

- node kinds:
- NTT / INTT
- MSM
- field-vector and polynomial kernels
- hash and Merkle kernels
- encoder / codeword kernels
- sumcheck round kernels
- FRI fold kernels
- decommitment packing kernels
- transcript barrier nodes
- proof-surface emission nodes
- verifier-wrapper kernels

- edge kinds:
- value dependencies
- challenge dependencies
- proof-surface dependencies
- memory-flow dependencies
- cross-proof batch dependencies
- device-transfer dependencies

- resource annotations:
- field / domain
- expected arithmetic intensity
- reuse profile
- proof visibility
- stage identifier
- batchability
- streaming capability

### 4.5 High-value DAG-kernel research problems

#### A. Challenge-barrier-aware scheduling

The scheduler should understand that transcript stages induce partial orders stricter than ordinary dataflow. This is one of the clearest places where ZK execution differs from AI execution.

#### B. Proof-surface-aware memory planning

Memory planning should account not only for liveness of computation values, but also for values that must survive as proof objects or verifier-facing artifacts. This is a natural extension of the current proof-ABI story.

#### C. Cross-proof superbatching

When many proofs share a closed protocol shape, the compiler should ask whether certain stages can be fused or pipelined across proofs. This is analogous to batch execution in inference compilers, but with additional transcript and proof-shape constraints.

#### D. Out-of-core and streaming proving

Many proof systems are bottlenecked by memory rather than arithmetic. A DAG-kernel layer is the natural place to represent chunking, streaming commitments, staged decommitments, and selective rematerialization.

#### E. Device-specialized kernel mapping

Different proving kernels favor different hardware. MSMs, NTTs, hash trees, and sumcheck rounds need not share the same optimal device. The compiler should be able to express and search such heterogeneous mappings.

### 4.6 Directly novel directions

The following ideas appear especially promising.

**Challenge-Barrier DAGs.**
Represent transcript-stage boundaries directly in the execution DAG so that schedulers, planners, and autotuners cannot silently violate protocol order.

**Proof-Surface-Aware Liveness.**
Generalize liveness analysis to track not only computational necessity but proof-surface necessity.

**Stage-Synchronous Superbatching.**
For “build once / prove many” workloads, superbatch the same protocol stage across many instances before the transcript diverges.

**Transcript-Respecting Graph Transformations.**
Develop graph rewrites that are legal only if they preserve stage order and proof ABI.

**Kernelized Recursion Trees.**
Model fold trees, recursion trees, and aggregation trees as execution graphs, not merely as recursive library calls.

These ideas would make the execution compiler distinctly ZK-native rather than a borrowed tensor runtime.

---

## 5. Semantic-aware proving beyond arithmetic lowering

The next broad direction is to preserve and exploit **program semantics during proving itself**, not only while lowering into arithmetic IR.

This is especially important for STARKs, but not limited to them.

### 5.1 The main idea

Today, semantics is often used only once:
- high-level program semantics are compiled down to arithmetic;
- after that point, the prover mostly treats the result as raw algebra.

Future work should ask a different question:

> **Which semantic facts remain valuable after arithmetic import, because they can still change proof construction cost, memory behavior, or protocol shape?**

The answer is not “all source semantics”. It is “proof-relevant semantics”.

### 5.2 Why STARKs are the clearest opportunity

STARK-style systems make semantic leverage unusually visible because they operate over structured traces, auxiliary traces, oracle commitments, decommitments, and query schedules.

The compiler can potentially use semantics to improve:
- trace segmentation,
- column-role assignment,
- periodic/preprocessed extraction,
- local vs nonlocal relation placement,
- auxiliary trace generation,
- commitment grouping,
- Merkle layout and path sharing,
- query locality,
- proof-shape stability.

Once those decisions are frozen, later kernels mostly realize them. So the proving pipeline has a genuine “semantic optimization window” above or around Protocol IR.

### 5.3 STARK-specific research directions

#### A. Semantic-guided trace shaping

The compiler should infer which state components belong in:
- main trace,
- auxiliary trace,
- periodic columns,
- preprocessed or fixed tables,
- streamed or rematerialized regions.

This was already motivated in the STARK note; future work should take it from AIR design into actual proving behavior and execution planning.

#### B. Semantic-guided trace streaming

Instead of fully materializing all traces in memory, the prover could stream semantically independent or phase-separated regions, committing and discarding where legal. This is especially attractive for large traces and memory-heavy zkVMs.

#### C. Locality-aware decommitment layout

Semantics can influence not only what gets committed but how committed oracles are laid out. If columns that tend to be queried together are placed with locality in mind, Merkle/decommitment cost and runtime behavior may improve.

#### D. Late materialization of challenge-dependent structures

Some auxiliary or interaction-derived structures need not exist before specific transcript stages. The compiler should delay them when that reduces memory pressure or allows better scheduling.

#### E. Semantics-guided hybridization

Within one proving system, different parts of a program may favor different subarguments. For example:
- regular trace evolution may favor STARK-style arguments,
- memory or lookup-heavy regions may favor sumcheck or specialized lookup arguments,
- repeated step relations may favor folding or accumulation wrappers.

Future work should explore whether the compiler can assemble such hybrid proofs safely.

### 5.4 Directly novel hypotheses

**Semantic query locality.**
STARK prover cost is partly a layout and decommitment problem, not just a low-degree-constraint problem. The compiler should search for layouts that improve expected query locality under fixed soundness parameters.

**Proof-time semantic caches.**
The compiler should produce compact semantic summaries—trace partition metadata, locality classes, periodicity descriptors, role tags—that are reused by the prover at runtime.

**Late auxiliary synthesis.**
Auxiliary trace materialization should be staged with transcript barriers rather than always emitted monolithically before proving.

**Proof-shape-stable semantic scheduling.**
Semantic-driven rewrites should be allowed only if they preserve the proof-shape and verifier interface promised by closure.

### 5.5 Broader consequence

This direction suggests that the project’s future semantic layer should not stop at “semantic-aware AIR generation”. It should grow into a broader notion of **semantic-aware proving**:
- semantics affects arithmetic generation,
- semantics affects protocol construction,
- semantics affects execution scheduling.

That is a larger and more distinctive compiler agenda.

---

## 6. Sumcheck, zerocheck, GKR, and multilinear systems

One of the most important future branches is the family of systems centered around **multilinear structure, sumcheck, zerocheck, and GKR-like reductions**.

This branch matters because it is increasingly central to fast provers, lookup-heavy zkVMs, and folding-friendly systems, yet it is not naturally captured by a purely R1CS/Plonkish/AIR view.

### 6.1 Why this needs a distinct compiler treatment

A sumcheck/GKR pipeline is not just “another backend for R1CS”.

Its natural semantic objects are different:
- multilinear polynomials,
- round claims,
- evaluation claims,
- zero-checks and sum-checks,
- layer wiring,
- reduction chains,
- table decompositions,
- memory-check subprotocols.

This strongly suggests future layers such as:
- `zk.mle`
- `zk.proto.sumcheck`
- `zk.proto.zerocheck`
- `zk.proto.gkr`
- possibly `zk.proto.lookup_mem`

### 6.2 Why this line is strategically important

Recent systems and papers reinforce that this is not a niche direction:
- **Spartan**, **Virgo**, **Libra**, and related systems made multilinear/sumcheck pipelines important for general-purpose SNARKs.
- **Jolt** and **Lasso** showed that zkVMs can be built around lookup- and sumcheck-heavy constructions.
- recent work on **sumcheck optimization**, **packed sumcheck**, and **streaming/space-efficient proving** shows that prover performance in these families is still very actively improving.
- recent survey work argues that a large fraction of the fastest modern systems can be understood through the lens of sumcheck and its reductions.

### 6.3 High-value future projects

#### A. GKR-from-DAG compilation

Instead of flattening everything to R1CS and only then rediscovering layered structure, the compiler should ask whether it can lower directly from program or dataflow DAGs into layered GKR-style claims.

This is especially attractive for:
- data-parallel kernels,
- ML/dataflow computations,
- structured arithmetic pipelines,
- certain database and graph operators.

#### B. Sumcheck as a first-class kernel family

If a future execution compiler exists, sumcheck rounds should be first-class kernel objects, not hidden inside a proving library. This will matter for scheduling, batching, recursion, and hardware mapping.

#### C. Zero-check as a reusable protocol interface

Recent folding work increasingly reduces relations to zero-check. The compiler should consider zero-check not just as a proof trick, but as a reusable protocol interface or target.

#### D. Streaming and memory-aware sumcheck

Streaming zero-knowledge proofs, low-memory sumcheck provers, and recent sumcheck optimizations suggest a strong connection between protocol structure and execution-memory planning. This is exactly where a future compiler can contribute.

#### E. Tower-field and binary-field sumcheck

The emerging binary-field and tower-field literature suggests that multilinear systems over small fields deserve dedicated compiler attention rather than being forced through a large-prime-field-only viewpoint.

### 6.4 Directly novel directions

**GKR-from-DAG instead of GKR-from-flattened-circuit.**
Preserve computational graph structure long enough that the compiler can choose layered interactive proofs without first collapsing everything into generic constraints.

**Zero-check as a mid-level semantic contract.**
Between statement IR and concrete protocol IR, represent a “zero-check contract” that multiple protocol families can target.

**Layer-aware semantic extraction.**
Use source or LLZK-side structure to identify natural computational layers, fan-in patterns, and memory boundaries for GKR compilation.

**Hybrid STARK + sumcheck compilation.**
Investigate mixed systems where trace-like regions use STARK machinery while dense algebraic subroutines use sumcheck/GKR subarguments.

This branch is likely to become one of the most important areas of future differentiation.

---

## 7. Folding, accumulation, recursion, IVC, and PCD

Another major branch is the family of systems built around **folding, accumulation, recursion, incrementally verifiable computation, and proof-carrying data**.

This area is no longer a niche extension. It is becoming a central organizing principle for scalable proofs, recursive proofs, and on-chain-friendly verification.

### 7.1 Why this branch deserves its own compiler line

The current Protocol IR documents center a single closed protocol. Future recursive systems will need to represent:
- accumulator state,
- foldable claims,
- recursion wrappers,
- compression proofs,
- recursive verifier interfaces,
- proof-carrying state transitions.

These are not well modeled as ordinary “proof objects plus one verifier”.

They suggest future layers such as:
- `zk.proto.fold`
- `zk.proto.accum`
- `zk.proto.ivc`
- `zk.proto.pcd`

### 7.2 Core family references and why they matter

This line now spans a large design space:
- **Nova**, **SuperNova**, and **HyperNova** established folding-based recursive arguments as a practical path for IVC.
- **ProtoStar** and **ProtoGalaxy** generalized folding and accumulation across broader proof families.
- **CycleFold** and **MicroNova** made recursion more practical for specific verifier and on-chain settings.
- **NeutronNova** emphasized zero-check-centric folding.
- **Nebula** added memory and switchboard structure to folding-oriented zkVM design.
- **Proof-Carrying Data without Succinct Arguments** and **Accumulation without Homomorphism** broadened the conceptual space beyond the standard recursive-SNARK picture.
- **Arc** suggested that hash-based and Reed–Solomon-oriented accumulation can be made first-class.
- **Neo** and **LatticeFold** show that folding itself is also moving into post-quantum directions.

The compiler implication is clear: recursion and accumulation are not “just wrappers”. They are protocol families with their own semantic objects.

### 7.3 High-value future research problems

#### A. Recursion-aware proof ABI

The current proof-ABI story is already strong, but recursive systems require more. The compiler should know:
- which verifier fragments are exported for recursion,
- which proof objects become recursive inputs,
- which parts of the verifier are stable enough to be wrapped,
- how ABI versions interact with recursive composition.

#### B. Accumulation as a scheduling problem

A folding/accumulation system typically has design choices such as:
- fold arity,
- fold tree shape,
- when to compress,
- when to wrap,
- when to checkpoint,
- how to batch instances before accumulation.

These are compiler optimization decisions, not just protocol definitions.

#### C. Foreign arithmetic minimization

Recursive systems often pay heavily for non-native or “foreign” arithmetic. Future work should connect recursion-aware lowering with techniques that offload or minimize foreign arithmetic rather than blindly embedding it in circuits.

#### D. Transparent and post-quantum recursive paths

Hash-based, Reed–Solomon-based, code-based, lattice-based, and binary-field-based accumulation paths open new design space for recursive and post-quantum systems. The compiler architecture should remain broad enough to model them.

#### E. PCD for distributed computation graphs

Proof-carrying data suggests a distributed and possibly indefinite notion of computation, not only a single finite proof. This could become highly relevant for:
- distributed proving services,
- asynchronous workflows,
- verifiable pipelines,
- proof-producing state machines.

### 7.4 Directly novel directions

**Recursion-aware verifier trace export.**
A closed protocol should eventually be able to export a verifier trace specifically meant for recursive wrapping, not only a human-facing verifier relation.

**Fold-tree autotuning.**
Choose fold arity, fold order, and compression cadence automatically based on the workload, hardware, and verifier target.

**Hybrid inner/outer proof synthesis.**
Let the compiler choose a different outer recursive or accumulation protocol than the inner base protocol.

**Transparent recursive STARK paths.**
Explore the interaction between STARK/oracle protocols and hash-based accumulation schemes such as Arc, rather than assuming recursion must be pairing- or PCS-centric.

**Distributed PCD compilation.**
Compile long-running or distributed workflows into proof-carrying state transitions rather than repeatedly recompiling monolithic proofs.

This entire line is likely to become one of the most visible long-term contributions of the broader project.

---

## 8. Protocol families beyond R1CS, Plonkish, and AIR

The previous sections focused on sumcheck/GKR and folding because they are central. But future work should also explicitly address several other protocol clusters that do not fit neatly into the current arithmetic taxonomy.

### 8.1 Code-based and binary-field systems

Systems such as **Brakedown**, **BaseFold**, **Blaze**, **Binius**, **DeepFold**, and related code-based or binary-field constructions suggest a distinct family cluster with natural objects such as:
- codewords,
- encoders,
- proximity claims,
- code switches,
- tower-field representations,
- low-query linear-code oracles.

For the compiler, this suggests future objects and feature layers such as:
- `zk.code`
- `zk.proto.code`
- `zk.proto.iopp`
- `zk.proto.rs_accum`
- tower-field-aware type systems and cost models.

This is strategically important because:
- it broadens the arithmetic assumptions and performance trade-offs,
- it interacts well with accumulation and post-quantum directions,
- and it is likely to matter for systems seeking transparent proofs with better hardware fit.

### 8.2 MPC-in-the-Head, VOLE, and OT-based systems

MPC-in-the-Head, VOLE-based, and OT-based proof systems are structurally different from PCS/FRI-centric systems.

Their natural semantic objects include:
- simulated views,
- seed trees,
- correlation packs,
- OT transcripts,
- VOLE shares,
- opening policies,
- cut-and-choose or challenge-reveal sets.

This suggests that future protocol layers may need concepts like:
- `zk.proto.view`
- `zk.proto.seedtree`
- `zk.proto.ot`
- `zk.proto.vole`

These systems are important because they open:
- post-quantum paths,
- very different concrete efficiency trade-offs,
- and proof-system architectures where “proof objects” are closer to views and correlated randomness than to polynomial openings.

A long-term compiler should not assume every protocol is fundamentally a polynomial IOP plus transcript.

### 8.3 Structured-data proofs: strings, parsers, documents, graphs

Recent systems such as **Reef** and **Coral** make a broader point: some important proof applications have natural frontends that are not “circuits first”.

Regex proofs, CFG proofs, structured document proofs, graph/database operator proofs, and similar systems suggest that future frontends may expose semantic objects such as:
- finite automata,
- parse trees,
- segmented memory,
- document schemas,
- graph traversal operators,
- structured lookup interfaces.

This implies a family of future frontends or semantic layers that are neither generic source-language DSLs nor ordinary arithmetic IRs. They are *proof-native application frontends*.

That matters because it broadens the project from “compiler for generic arithmetic relations” toward “compiler for proof-native computation domains”.

### 8.4 Post-quantum and alternative-assumption directions

The future architecture should also remain compatible with:
- lattice-based folding and commitments,
- Galois-ring or small-characteristic proof systems,
- symmetric-key or hash-based accumulation,
- binary-field and code-based approaches.

This is not merely a matter of adding more backends. It may change:
- field/type assumptions,
- verifier capabilities,
- accumulation mechanisms,
- recursive wrapper choices,
- and the hardware cost model.

---

## 9. Frontend DSL and language research

The project will eventually need more than one frontend story.

The current documents already give one frontend role to `zk.scheme`, but future research should treat frontend design as a substantial research area rather than just a syntax problem.

### 9.1 A typed protocol-intent DSL

The current `zk.scheme` idea can be pushed further into a genuinely typed DSL for:
- protocol families and profiles,
- proof-surface intent,
- recursion or accumulation structure,
- security assumptions and budgets,
- rewrite permissions,
- ABI constraints,
- backend objectives.

This DSL should not become a theorem prover in the first step, but it should be rich enough to avoid turning protocol construction into an untyped builder API.

### 9.2 A semantic DSL above arithmetic IR

The STARK note already motivates a proof-relevant semantic layer above AIR. Future work should generalize that idea.

Such a DSL or IR should preserve only the semantic facts that still affect proving:
- state schema,
- memory and table kinds,
- control regularity,
- proof-shape invariants,
- relation kinds,
- preprocessing opportunities,
- recursion boundaries,
- distributed partitioning opportunities.

This could appear as `zk.sem`, `zk.exec`, `zk.trace`, or a family of domain-specific semantic dialects.

### 9.3 Application-specific proof frontends

Future frontends may also exist for domains where the natural object is not a generic program:
- structured documents,
- parsers,
- regex and grammars,
- ML graphs,
- database operators,
- graph traversals,
- state-machine traces.

This is an especially promising direction because it can yield proofs that are both more expressive and more efficient than a naive “compile everything through a general circuit frontend” approach.

### 9.4 Language-design principles that should inform this work

Frontend and semantic DSL design should draw on several PL ideas already relevant to the project:

- **effect systems** for transcript, entropy, proof surface, distributed communication, and perhaps recursion;
- **linearity or affinity** for transcript tokens, accumulators, and one-shot protocol resources;
- **refinement types** for shape stability, challenge influence, memory discipline, and domain-specific invariants;
- **proof-carrying artifacts** for certificates, discharged obligations, and backend contracts;
- **semantics extraction** from formal semantics frameworks where appropriate.

### 9.5 Directly novel directions

**Proof-relevant effect systems.**
Generalize the current four-resource story into an effect system that can also express distributed proving, recursion resources, and correlation resources.

**Witness-shape separation types.**
Expose to the frontend the difference between value variation and proof-topology variation.

**Theorem-carrying protocol templates.**
Allow frontend protocol templates to carry explicit admissibility or theorem-profile metadata rather than hardcoding such facts in backends.

**A semantic interface language between LLZK and Protocol IR.**
Instead of a single ad hoc importer, define a stable interface describing the proof-relevant arithmetic facts exported from LLZK or other arithmetic substrates.

**Frontend DSLs for proof-native domains.**
Treat parsing, document structure, table semantics, memory traces, and graph queries as first-class proof frontend targets.

This direction is broader than “design a nice syntax”. It is about discovering the right semantic contracts between users, arithmetic compilers, protocol compilers, and execution backends.

---

## 10. Autotuning and search as first-class methodology

Once the project has:
- a stable protocol layer,
- a future DAG-kernel layer,
- and a Transform-style control plane,

the next obvious step is **autotuning**.

This is not optional future polish. It is likely necessary because the design space is too large for one fixed pass pipeline to dominate everywhere.

### 10.1 Three distinct search spaces

The future compiler should distinguish at least three search levels.

#### A. Protocol-level tuning

Examples:
- commitment grouping,
- query/decommitment planning,
- FRI schedule choices,
- Merkle arity and path-sharing policy,
- lookup and memory-check strategy,
- fold arity and compression cadence,
- recursion wrapper choice,
- proof-size versus prover-time trade-offs.

#### B. Kernel-level tuning

Examples:
- thread/block mapping,
- tiling,
- fusion,
- register/shared-memory trade-offs,
- code generation strategy,
- polynomial layout,
- NTT staging,
- hash-kernel specialization.

#### C. System-level tuning

Examples:
- batch size,
- proof batching policy,
- multi-device placement,
- CPU/GPU overlap,
- pipeline depth,
- checkpoint placement,
- cluster partitioning,
- communication topology.

### 10.2 What AI compilers teach here

Again, AI compilers are a methodology reference, not a direct blueprint.

- Halide and its autoschedulers show how to separate schedule space from algorithmic meaning.
- Ansor shows how to search large spaces via learned cost models and evolutionary search.
- MetaSchedule shows how probabilistic or composable search spaces can be exposed to experts.
- Exo suggests that user-extensible or domain-extensible schedule spaces are often preferable to hardcoded global optimizers.
- FlexFlow, Alpa, and GSPMD show how system- and cluster-level planning can also be compiler territory.

The lesson for ZK is:

> **tuning should happen over explicit legal transformation spaces, not by mutating opaque prover-library knobs.**

### 10.3 Why ZK tuning is harder than ordinary performance tuning

A ZK autotuner must optimize subject to stronger constraints:

- it must preserve protocol semantics,
- it must preserve or explicitly version the proof ABI,
- it must respect transcript and challenge barriers,
- it must not violate theorem-backed admissibility or security budgets,
- it may need to preserve verifier complexity targets,
- it may need to preserve recursion-friendliness or on-chain constraints.

This suggests that ZK autotuning cannot simply inherit the “search over whatever compiles” attitude. It needs a stronger notion of legality.

### 10.4 High-value future projects

#### A. Certificate-constrained autotuning

Every candidate transform should carry legality and security guards. The search engine explores only transformations with discharged certificates or readily checkable legality conditions.

#### B. Proof-shape-stable tuning

For many deployments, the most valuable tuning regime is not arbitrary optimization but optimization under a fixed proof shape and stable verifier interface.

#### C. Transfer learning across protocol families

The compiler should try to learn cost models from structural fingerprints:
- relation counts,
- trace width/height,
- sumcheck arity,
- folding tree size,
- decommitment topology,
- batch size,
- hardware target.

This is likely more robust than trying to learn only from one protocol implementation.

#### D. Bundle-stored tuning profiles

“Tune once, prove many” should become a first-class artifact story: store the discovered tuning plan in the proving bundle and replay it across many instances that share protocol shape.

#### E. Multi-objective optimization

The optimizer should explicitly support Pareto trade-offs among:
- prover time,
- proof size,
- verifier time,
- peak memory,
- preprocessing size,
- recursion overhead,
- on-chain cost,
- cluster cost.

### 10.5 Directly novel directions

**Protocol-phase autotuning.**
Tune not only kernels, but the protocol phase structure itself where legal.

**Challenge-aware search spaces.**
Do not allow the search engine to reorder across transcript barriers unless a proof of equivalence exists.

**Recursive-strategy autotuning.**
Search over accumulation/folding tree shape and compression cadence.

**Locality-aware STARK tuning.**
Search over trace layout, query locality, and Merkle layout while holding soundness parameters fixed.

This could become one of the most practically impactful long-term branches.

---

## 11. Lowering to multiple machines, clusters, and proving services

Another essential future direction is lowering not just to “a machine” but to **many machines** and even to **proving services**.

### 11.1 Why this needs compiler support

Large proving deployments increasingly involve:
- CPU + GPU heterogeneous nodes,
- multi-GPU systems,
- clusters,
- collaborative proving,
- private delegation,
- proving marketplaces or community provers,
- fault-tolerant orchestration layers.

It would be a mistake to treat this as purely operational infrastructure. The partitioning and orchestration choices often depend on protocol structure itself.

### 11.2 Levels of distributed lowering

It is useful to distinguish four levels.

#### A. Single-node heterogeneous lowering

Map kernels across CPU, GPU, FPGA, ASIC, or specialized accelerators.

#### B. Multi-device intra-node lowering

Partition kernels and memory across multiple GPUs or accelerators in one node.

#### C. Multi-node distributed lowering

Plan data movement, partial commitments, partial proof objects, and synchronization across a cluster.

#### D. Service-topology lowering

Compile for outsourced or collaborative proving topologies:
- trusted local prover + untrusted helpers,
- multiple mutually distrustful workers,
- private delegation to one or more servers,
- queue-based orchestration and failover services.

### 11.3 ZK-specific distributed concerns

Distributed proving is not just a matter of parallelizing arithmetic.

The compiler must eventually reason about:
- transcript-stage synchronization,
- proof-share assembly,
- commitment consistency,
- witness privacy across workers,
- partial-proof checkpoints,
- deterministic replay,
- failure recovery,
- verifier trust model,
- cost of network transport for prover artifacts.

### 11.4 Relevant research threads

Several external threads make this direction timely:
- collaborative zk-SNARKs and proof delegation,
- private outsourcing of provers,
- batch and pipelined GPU proving systems,
- multi-GPU acceleration for NTT and other core kernels,
- cloud-native orchestration systems for real-time block proving.

These systems are not yet a compiler architecture, but they clearly motivate one.

### 11.5 High-value future projects

#### A. Transcript-consistent SPMD lowering

Develop a distributed lowering model where workers execute under a shared stage/challenge discipline, synchronizing only where transcript barriers require it.

#### B. Partitioning dimensions specialized for proving

General SPMD frameworks partition tensors. A ZK compiler may need to partition by:
- polynomial batch,
- trace rows,
- trace columns,
- Merkle levels,
- query batches,
- fold-tree nodes,
- proof batch,
- recursive wrapper layer.

#### C. Shard-aware proof ABI

If proofs are assembled from distributed workers, the ABI may need to distinguish internal shard outputs from final proof slots.

#### D. Resumable and checkpointable proving

Distributed and real-time proving benefits from checkpoints and resumable tasks. The compiler should identify safe checkpoint boundaries, probably aligned with transcript or accumulation stages.

#### E. Proof-service lowering

Compile not just to hardware, but to service graphs: queue topology, worker specialization, artifact storage, and collection strategy become part of the target.

### 11.6 Directly novel directions

**Prover-topology as a compiler target.**
Treat cluster and service topology as a first-class compilation target, not a deployment afterthought.

**Transcript-consistent distributed scheduling.**
Use the same stage/closure discipline that governs single-machine proving to govern distributed proving.

**Community-proving compilation.**
Compile a closed protocol into many partially independent tasks suitable for unreliable or commodity workers, with assembly and verification policies.

**Hybrid local/outsourced proving.**
Allow the compiler to decide which protocol stages remain local and which are offloaded.

This branch connects directly to real-time proving, proving services, and decentralized prover markets.

---

## 12. Assurance, certification, and validation as a long-term layer

Future work should also strengthen the **assurance story** around the compiler.

The current documents already emphasize verifier explicitness, obligations, and testing. But the long-term system can go further.

### 12.1 Certifying compilation

A future compiler should be able to emit not only code and proof bundles, but also:
- obligation discharge records,
- proof-ABI hashes,
- verifier-interface hashes,
- admissibility-profile certificates,
- protocol-profile digests,
- cost-model provenance,
- recursion-wrapper compatibility records.

This would make the bundle more than an optimized artifact. It would make it a **certified proving object**.

### 12.2 Formal-methods directions

Promising longer-term directions include:
- lowering `zk.verify` to verification-oriented dialects or solver backends,
- certifying equivalence for selected protocol rewrites,
- mechanizing transcript or proof-ABI properties,
- producing machine-checkable certificates for parts of closure,
- proving selected backend-preservation properties.

The goal should not be to formalize everything immediately. The goal is to design the layers so that formal statements are natural.

### 12.3 Validation and testing research

Recent work on ZK security, mutation testing, fuzzing, and verifier vetting strongly suggests that validation itself should be a research area.

A future compiler could support:
- transcript mutation testing,
- proof-shape differential testing,
- metamorphic testing across equivalent rewrites,
- backend differential checking,
- recursive-wrapper equivalence tests,
- verifier fragment audits.

### 12.4 Directly novel directions

**Proof-carrying bundles.**
A deployed artifact could carry a concise certificate of closure, ABI state, and backend-preservation assumptions.

**Transform-level proof obligations.**
The transform language could require explicit proof or certificate attachments for dangerous rewrite classes.

**Security-regression testing for compiler passes.**
Treat protocol passes the way optimizing compilers treat miscompilation risk: with regression suites, differential checking, and negative tests.

This direction will matter both scientifically and operationally.

---

## 13. Application-led research fronts

A final set of future directions is driven by application domains that stress different parts of the architecture.

These are not merely “applications”. They can shape the compiler itself.

### 13.1 zkML and dataflow-heavy proving

zkML and verifiable AI inference stress:
- large dataflow graphs,
- mixed numeric representations,
- hardware heterogeneity,
- batching,
- recursion for verification compression,
- sometimes nontrivial approximate or foreign arithmetic.

This makes them a natural testbed for:
- DAG kernels,
- GKR/dataflow compilation,
- autotuning,
- distributed lowering.

### 13.2 Structured document and parser proofs

Regex, CFG, JSON, and document proofs show that not all proof frontends are arithmetic-language frontends. They motivate:
- parser-aware DSLs,
- structured memory abstractions,
- proof-native application IRs.

### 13.3 Databases, graphs, and stateful services

Database operators, graph queries, and verifiable state machines stress:
- memory consistency,
- concurrency,
- repeated transitions,
- state and log structure,
- incremental or recursive proof production.

These may become ideal targets for:
- STARK semantic layers,
- GKR/dataflow compilation,
- PCD and state-transition compilers,
- distributed proving services.

### 13.4 Foreign arithmetic, floating point, and crypto subroutines

Some domains stress the interaction between the underlying proof field and external arithmetic:
- floating point,
- binary fields,
- elliptic curve subroutines,
- AES and other non-native cryptographic operations.

These domains motivate future work on:
- semantic selection of the most appropriate protocol family,
- offloading or minimizing foreign arithmetic,
- hybrid proofs rather than monolithic circuit embeddings.

### 13.5 Proving services and real-time proving

Real-time L2 proving, proving marketplaces, and outsourced provers stress:
- service topology,
- fault tolerance,
- checkpointing,
- queueing and orchestration,
- resource-aware batch formation,
- privacy during delegation.

These domains are likely to make the distributed-lowering story concrete faster than purely academic benchmarks will.

---

## 14. Recommended long-term research program

This document is broader than the prioritized roadmap, but some ordering still helps.

### 14.1 Highest-value next wave

After the current core paper, the strongest next directions are:

1. **STARK/oracle specialization plus semantic-aware proving**
 This validates that the architecture is not secretly PCS-centric and opens the biggest semantic optimization space.

2. **DAG kernels and native execution compilation**
 This turns Protocol IR into a full compiler stack rather than only a semantic layer plus library backend.

3. **sumcheck/GKR/folding branch**
 This is the best way to break out of the “all important families are arithmetic-family variants” mental model.

### 14.2 Medium-term wave

After those, the strongest medium-term branches are:

4. **autotuning and search over protocol, kernel, and cluster plans**
5. **distributed/service-topology lowering**
6. **recursive / accumulation-aware ABI and bundle design**

### 14.3 Broadening wave

A later broadening wave should tackle:

7. **alternative protocol families** such as code-based, binary-field, MPCitH, and VOLE-style systems
8. **proof-native frontends** for documents, databases, graphs, and domain-specific workflows
9. **certifying compilation and stronger formal assurance**

### 14.4 Candidate future-paper sequence

A coherent publication sequence might look like this:

**Paper A — STARK Semantic-Aware Proving**
Proof-relevant semantics above AIR and into protocol/execution choices.

**Paper B — DAG Kernels for Provers**
Challenge-barrier-aware execution graphs, memory planning, and kernel lowering.

**Paper C — Sumcheck/GKR/Folding as Protocol Families**
A two-axis statement/protocol compiler architecture.

**Paper D — Autotuned Protocol and Kernel Compilation**
Certificate-constrained search over protocol and execution spaces.

**Paper E — Distributed and Service-Topology Lowering**
Compilation to clusters, collaborative proving, and proving orchestration.

**Paper F — Certifying ZK Compilation**
Closure certificates, ABI certificates, and stronger backend-preservation evidence.

This would preserve a clear program arc without collapsing everything into one monolith.

---

## 15. High-value open questions

The questions below are, in my view, the most important ones to keep active.

### 15.1 Architectural questions

1. What is the right boundary between Protocol IR and a future DAG-kernel IR?
2. Should the long-term stack explicitly separate statement-family IR and protocol-family IR?
3. How much protocol-family structure should be explicit before execution lowering?

### 15.2 Semantic questions

4. Which semantic facts most improve prover memory behavior, not just arithmetic compactness?
5. How much of STARK proving can be improved by semantic locality and layout choices?
6. Can the compiler choose between STARK, sumcheck/GKR, and folding-oriented subprotocols compositionally?

### 15.3 Execution questions

7. What are the right primitives for proof-DAG scheduling under transcript barriers?
8. What should be batchable across proofs, and what should remain per-proof?
9. How should checkpointing and resumability interact with transcript and recursion boundaries?

### 15.4 Recursion and accumulation questions

10. What is the right IR contract for accumulators and recursive verifier traces?
11. When should the compiler recurse, fold, accumulate, or simply batch?
12. Can transparent and post-quantum recursive paths become first-class compilation targets?

### 15.5 Language and tooling questions

13. What should the frontend expose as protocol intent versus arithmetic intent versus application semantics?
14. What effect/type system is rich enough to express transcript, entropy, recursion, and distributed resources without becoming unusable?
15. How should autotuners interact with proof certificates, ABI stability, and security constraints?

These questions are sufficiently deep that they can anchor multiple future papers.

---

## 16. Final position

The current project is strongest when it remains disciplined:

> **Protocol IR closes protocol meaning before execution lowering begins.**

Future work should preserve that center while expanding in three directions:

- **upward**, into proof-relevant semantics that still matter for proving;
- **sideways**, into new protocol families such as sumcheck/GKR, folding/accumulation, code-based and VOLE/MPCitH systems;
- **downward**, into DAG kernels, autotuning, heterogeneous devices, clusters, and proving services.

The most important long-term conceptual move is this:

> **A ZK compiler should not be modeled only as “frontend → arithmetic IR → backend”. It should eventually become a compiler over statement families, protocol families, and execution families, with Protocol IR remaining the semantic hinge that keeps those axes coherent.**

That is the broad research direction this document recommends.

---

## Selected references

### Canonical project documents

- `system-and-scope.md`
- `protocol-ir.md`
- `implementation-spec.md`
- `extensions-and-roadmap.md`
- `Semantic-Aware-Protocol-Compilation-for-STARK-Provers.txt`

### AI compiler, scheduling, and execution planning

- [Halide] Jonathan Ragan-Kelley et al. *Halide: A Language and Compiler for Optimizing Parallelism, Locality, and Recomputation in Image Processing Pipelines*.
- [Halide Autoscheduler] Anderson et al. *Learning to Optimize Halide with Tree Search and Random Programs*.
- [TVM] Tianqi Chen et al. *TVM: An Automated End-to-End Optimizing Compiler for Deep Learning*.
- [Ansor] Lianmin Zheng et al. *Ansor: Generating High-Performance Tensor Programs for Deep Learning*.
- [MetaSchedule] Junru Shao et al. *Tensor Program Optimization with Probabilistic Programs*.
- [Exo] Yu Feng Ikarashi et al. *Exocompilation for Productive Programming of Hardware Accelerators*.
- [GSPMD] Yuanzhong Xu et al. *GSPMD: General and Scalable Parallelization for ML Computation Graphs*.
- [Alpa] Lianmin Zheng et al. *Alpa: Automating Inter- and Intra-Operator Parallelism for Distributed Deep Learning*.
- [FlexFlow] Zhihao Jia et al. *Beyond Data and Model Parallelism for Deep Neural Networks*.
- [IREE] IREE documentation.

### STARK, semantic-aware proving, and prover systems

- [Winterfell] Winterfell AIR documentation.
- [AirScript] AirScript repository.
- [RISC Zero] RISC Zero proof-system documentation.
- [BatchZK] Tao Lu et al. *BatchZK: A Fully Pipelined GPU-Accelerated System for Batch Generation of Zero-Knowledge Proofs*.
- [UniZK] Cheng Wang and Mingyu Gao. *UniZK: Accelerating Zero-Knowledge Proof with Unified Hardware and Flexible Kernel Mapping*.
- [ZKPoG] Mingyang Li et al. *ZKPoG: Accelerating WitGen-Incorporated End-to-End Zero-Knowledge Proof on GPU*.
- [UniNTT] Zhuoran Ji et al. *Accelerating Number Theoretic Transform with Multi-GPU Systems for Efficient Zero Knowledge Proof*.
- [ZKProphet] Tarunesh Verma et al. *Understanding Performance of Zero-Knowledge Proofs on GPUs*.

### Sumcheck, multilinear, and GKR lines

- [Spartan] Srinath Setty. *Spartan: Efficient and General-Purpose zkSNARKs without Trusted Setup*.
- [Libra] Tiancheng Xie et al. *Libra: Succinct Zero-Knowledge Proofs with Optimal Prover Computation*.
- [Virgo] Jiaheng Zhang et al. *Transparent Polynomial Delegation and Its Applications to Zero Knowledge Proof*.
- [Jolt] Arasu Arun, Srinath Setty, Justin Thaler. *Jolt: SNARKs for Virtual Machines via Lookups*.
- [Lasso] Srinath Setty, Justin Thaler, Riad Wahby. *Unlocking the Lookup Singularity with Lasso*.
- [Twist and Shout] Srinath Setty et al. *Twist and Shout: Faster Memory Checking Arguments via One Write-Read Grand Product*.
- [Sumcheck Optimizations] Quang Dao and Justin Thaler. *More Optimizations to Sum-Check Proving*.
- [Packed Sumcheck] Yuanju Wei et al. *Packed Sumcheck over Fields of Small Characteristic*.
- [Sumcheck Survey] Justin Thaler. *Sumcheck Is All You Need: An Opinionated Survey on Fast Provers*.
- [Streaming ZK Proofs] Graham Cormode et al. *Streaming Zero-Knowledge Proofs*.

### Folding, accumulation, recursion, and PCD

- [Nova] Abhiram Kothapalli, Srinath Setty, Ioanna Tzialla. *Nova: Recursive Zero-Knowledge Arguments from Folding Schemes*.
- [SuperNova] Abhiram Kothapalli and Srinath Setty. *SuperNova: Proving Universal Machine Executions without Universal Circuits*.
- [HyperNova] Abhiram Kothapalli and Srinath Setty. *HyperNova: Recursive Arguments for Customizable Constraint Systems*.
- [ProtoStar] Benedikt Bünz et al. *ProtoStar: Generic Efficient Accumulation/Folding for Special-Sound Protocols*.
- [ProtoGalaxy] Liam Eagen and Ariel Gabizon. *ProtoGalaxy: Efficient ProtoStar-style Folding of Multiple Instances*.
- [CycleFold] Abhiram Kothapalli and Srinath Setty. *CycleFold: Folding-Scheme-Based Recursive Arguments over a Cycle of Elliptic Curves*.
- [MicroNova] Jiaxing Zhao et al. *MicroNova: Folding-Based Arguments with Efficient (On-Chain) Verification*.
- [NeutronNova] Abhiram Kothapalli and Srinath Setty. *NeutronNova: Folding Everything that Reduces to Zero-Check*.
- [Nebula] Arasu Arun et al. *Nebula: Efficient Read-Write Memory and Switchboard Circuits for Folding Schemes*.
- [PCD without Succinct Arguments] Benedikt Bünz et al. *Proof-Carrying Data without Succinct Arguments*.
- [Accumulation without Homomorphism] Benedikt Bünz et al. *Accumulation without Homomorphism*.
- [Arc] Benedikt Bünz et al. *Arc: Accumulation for Reed–Solomon Codes*.
- [Neo] Wilson Nguyen and Srinath Setty. *Neo: Lattice-Based Folding Scheme for CCS over Small Fields and Pay-Per-Bit Commitments*.
- [LatticeFold] Dan Boneh et al. *LatticeFold: A Lattice-based Folding Scheme and its Applications*.

### Code-based, binary-field, and alternative proof families

- [Brakedown] Alexander Golovnev et al. *Brakedown: Linear-Time and Field-Agnostic SNARKs for R1CS*.
- [Orion] Tiancheng Xie and Yupeng Zhang. *Orion: Zero Knowledge Proof with Linear Prover Time*.
- [BaseFold] Henry Zeilberger et al. *BaseFold: Efficient Field-Agnostic Polynomial Commitment Scheme from Foldable Linear Codes*.
- [Blaze] Matthias Brehm et al. *Blaze: Fast SNARKs from Interleaved RAA Codes*.
- [Binius] Benjamin Diamond and Jim Posen. *Succinct Arguments over Towers of Binary Fields*.
- [DeepFold] Yanpei Guo et al. *DeepFold: Efficient Multilinear Polynomial Commitment Scheme from FRI-Style Folding*.

### MPC-in-the-Head, VOLE, and OT-style systems

- [MPCitH + OT] Delpech de Saint Guilhem et al. *Zero-Knowledge Systems from MPC-in-the-Head and Oblivious Transfer*.
- [VOLE Disjunctions] Carmit Hazay et al. *Optimizing Proofs of Disjunctive Statements in VOLE-Based ZK*.
- [JesseQ] Meng Liu et al. *Efficient Zero-Knowledge Proofs for Circuits over Any Field*.

### Structured-data and proof-native frontends

- [Reef] Sebastian Angel et al. *Fast Succinct Non-Interactive Zero-Knowledge Regex Proofs*.
- [Coral] Sebastian Angel et al. *Coral: Fast Succinct Non-Interactive Zero-Knowledge CFG Proofs*.
- [Traversable Languages] Assimakis Kattis and Joseph Bonneau. *A Framework for Compiling Custom Languages as Efficiently Verifiable zkVMs*.

### Assurance, validation, and benchmarking

- [Verification Dialects] Mathieu Fehr et al. *First-Class Verification Dialects for MLIR*.
- [MTZK] Dongwei Xiao et al. *MTZK: Testing and Exploring Bugs in Zero-Knowledge (ZK) Compilers*.
- [Circuzz] Arseniy Alekseyev et al. *Circuzz*.
- [SNARK SoK 2024] Stefanos Chaliasos et al. *SoK: What Don’t We Know? Understanding Security Risks of SNARKs*.
- [SNARK SoK 2025] Jiaheng Liang et al. *SoK: Understanding zk-SNARKs — The Gap Between Research and Practice*.

### Distributed proving and proving services

- [Collaborative zkSNARKs] Xuanming Liu et al. *Scalable Collaborative zk-SNARK: Fully Distributed Proof Generation and Malicious Security*.
- [Collaborative Delegation] Xuanming Liu et al. *Scalable Collaborative zk-SNARK and Its Application to Fully Distributed Proof Delegation*.
- [Private Outsourcing] Kasra Abbaszadeh et al. *Single-Server Private Outsourcing of zk-SNARKs*.
- [push0] Mohsen Ahmadvand et al. *push0: Scalable and Fault-Tolerant Orchestration for Zero-Knowledge Proof Generation*.
- [CrowdProve] John Stephan et al. *CrowdProve: Community Proving for ZK Rollups*.

---

## Reference links

[Halide]: https://people.csail.mit.edu/jrk/halide-pldi13.pdf
[Halide Autoscheduler]: https://halide-lang.org/papers/autoscheduler2019.html
[TVM]: https://www.usenix.org/system/files/osdi18-chen.pdf
[Ansor]: https://www.usenix.org/system/files/osdi20-zheng.pdf
[MetaSchedule]: https://arxiv.org/abs/2205.13603
[Exo]: https://dl.acm.org/doi/10.1145/3519939.3523446
[GSPMD]: https://arxiv.org/abs/2105.04663
[Alpa]: https://www.usenix.org/system/files/osdi22-zheng-lianmin.pdf
[FlexFlow]: https://arxiv.org/abs/1807.05358
[IREE]: https://iree.dev/

[Winterfell]: https://docs.rs/winterfell/latest/winterfell/trait.Air.html
[AirScript]: https://github.com/0xMiden/air-script
[RISC Zero]: https://dev.risczero.com/proof-system/proof-system-sequence-diagram
[BatchZK]: https://eprint.iacr.org/2024/1862
[UniZK]: https://dl.acm.org/doi/10.1145/3669940.3707228
[ZKPoG]: https://eprint.iacr.org/2025/765
[UniNTT]: https://dl.acm.org/doi/10.1145/3669940.3707241
[ZKProphet]: https://arxiv.org/abs/2509.22684

[Spartan]: https://eprint.iacr.org/2019/550
[Libra]: https://eprint.iacr.org/2019/317
[Virgo]: https://eprint.iacr.org/2019/1482
[Jolt]: https://eprint.iacr.org/2023/1217
[Lasso]: https://eprint.iacr.org/2023/1216
[Twist and Shout]: https://eprint.iacr.org/2025/105
[Sumcheck Optimizations]: https://eprint.iacr.org/2024/1210
[Packed Sumcheck]: https://eprint.iacr.org/2025/719
[Sumcheck Survey]: https://eprint.iacr.org/2025/2041
[Streaming ZK Proofs]: https://arxiv.org/abs/2301.02161

[Nova]: https://eprint.iacr.org/2021/370
[SuperNova]: https://eprint.iacr.org/2022/1758
[HyperNova]: https://eprint.iacr.org/2023/573
[ProtoStar]: https://eprint.iacr.org/2023/620
[ProtoGalaxy]: https://eprint.iacr.org/2023/1106
[CycleFold]: https://eprint.iacr.org/2023/1192
[MicroNova]: https://eprint.iacr.org/2024/2099
[NeutronNova]: https://eprint.iacr.org/2024/1606
[Nebula]: https://eprint.iacr.org/2024/1605
[PCD without Succinct Arguments]: https://eprint.iacr.org/2020/1618
[Accumulation without Homomorphism]: https://eprint.iacr.org/2024/474
[Arc]: https://eprint.iacr.org/2024/1731
[Neo]: https://eprint.iacr.org/2025/294
[LatticeFold]: https://eprint.iacr.org/2024/257

[Brakedown]: https://eprint.iacr.org/2021/1043
[Orion]: https://eprint.iacr.org/2022/1010
[BaseFold]: https://eprint.iacr.org/2023/1705
[Blaze]: https://eprint.iacr.org/2024/1609
[Binius]: https://eprint.iacr.org/2023/1784
[DeepFold]: https://www.usenix.org/system/files/usenixsecurity25-sec25cycle1-prepub-813-guo-yanpei.pdf

[MPCitH + OT]: https://eprint.iacr.org/2023/1470
[VOLE Disjunctions]: https://eprint.iacr.org/2024/1427
[JesseQ]: https://eprint.iacr.org/2025/533

[Reef]: https://eprint.iacr.org/2023/1886
[Coral]: https://eprint.iacr.org/2025/1420
[Traversable Languages]: https://eprint.iacr.org/2025/1110

[Verification Dialects]: https://users.cs.utah.edu/~regehr/papers/pldi25.pdf
[MTZK]: https://www.ndss-symposium.org/ndss-paper/mtzk-testing-and-exploring-bugs-in-zero-knowledge-zk-compilers/
[Circuzz]: https://aisychev.github.io/papers/ccs25-circuzz.pdf
[SNARK SoK 2024]: https://www.usenix.org/system/files/usenixsecurity24-chaliasos.pdf
[SNARK SoK 2025]: https://www.usenix.org/system/files/usenixsecurity25-liang-sok.pdf

[Collaborative zkSNARKs]: https://eprint.iacr.org/2024/143
[Collaborative Delegation]: https://eprint.iacr.org/2024/940
[Private Outsourcing]: https://eprint.iacr.org/2025/2113
[push0]: https://arxiv.org/abs/2602.16338
[CrowdProve]: https://arxiv.org/abs/2501.03126
