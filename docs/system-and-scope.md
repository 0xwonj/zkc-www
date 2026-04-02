# System and Scope

## Abstract

This project studies a **compiler architecture for zero-knowledge proving**.

Today, most proof systems are built and exposed as **family-specific libraries or vertically integrated prover stacks**. That is a reasonable deployment model, but it is a weak model for shared compiler infrastructure. Security-relevant protocol choices remain hidden inside libraries, optimization opportunities are trapped inside each prover implementation, verifier structure is often only partially explicit, and cross-family research iteration becomes slower than it needs to be.

The goal of this project is not to replace proving libraries with one universal monolith. The goal is to build an **MLIR-native compiler framework** in which multiple layers of the proving stack can share reusable IRs, passes, analyses, and contracts. In that larger picture, a serious ZK compiler should not be understood as only a source-language frontend, only an arithmetic lowering pipeline, or only a collection of prover kernels. It should be understood as a stack of semantic layers:

1. source or circuit intent,
2. arithmetic structure,
3. protocol structure,
4. verifier structure,
5. execution structure.

The current project centers the boundary between the middle layers: the point at which an arithmetic object becomes a **closed prover/verifier protocol** with a fixed transcript discipline, proof surface, verifier relation, and security profile.

The architectural rule is therefore:

> **Protocol instantiation happens before kernelization.**

This rule positions **Protocol IR** between arithmetic structure and execution structure. Arithmetic IR describes what is being proved. Protocol IR describes what prover and verifier do as a cryptographic procedure. Execution IR describes how that fixed procedure is run efficiently.

This document defines that big picture, motivates the compiler framing, and marks exactly where the current scope begins and ends.

---

## 1. Why a ZK compiler framework

### 1.1 The current implementation pattern

Much of the ZK ecosystem is organized around individual proof-system implementations: one library for one proving family, another stack for another family, and so on. Even when these systems are sophisticated and high-performance, they are usually optimized as **separate software artifacts**.

That pattern has real strengths. It allows aggressive family-specific engineering and clear end-to-end ownership. But it also has clear limits.

- Security-relevant logic such as transcript discipline, proof layout, verifier construction, and family-specific closure rules often lives inside library code rather than inside a shared compiler layer.
- Performance work often mixes together protocol design, execution planning, and low-level kernel engineering, which makes reuse harder.
- Similar concepts reappear across systems—commitment organization, challenge scheduling, verifier replay, proof readback, batching structure—but with no common intermediate representation.
- Research iteration across families becomes expensive because each experiment tends to require re-entering a different proving codebase.

A compiler framework is valuable precisely because it **separates concerns without erasing family structure**.

### 1.2 What a shared compiler should make possible

A useful ZK compiler framework should improve at least four things.

| Dimension | Why a compiler framework helps |
|---|---|
| Security | Makes transcript discipline, verifier completeness, proof shape, and closure conditions explicit rather than library-local conventions |
| Performance | Separates protocol-level optimization from execution-level optimization and enables reusable analyses and lowering paths |
| Maintainability | Reuses IRs, passes, validators, and backends across multiple proving families instead of re-implementing similar logic per library |
| Research iteration | Makes it easier to compare protocol choices, change boundaries, test alternative lowerings, and evaluate new designs without rebuilding an entire prover stack each time |

In other words, the compiler goal is not merely “better code generation.” It is a better **architectural decomposition** of proving.

### 1.3 Why MLIR is a good fit

MLIR is attractive here because the proving stack is naturally multi-level.

- There are several semantically distinct layers.
- Different layers want different invariants and rewrite spaces.
- Some boundaries are lowering boundaries; others are semantic lock-in boundaries.
- The system benefits from reusable dialect structure, legality-driven lowering, interfaces, validation, and eventually transform/control infrastructure.

This is exactly the sort of setting where an MLIR-style architecture is more natural than a single flat IR or a monolithic prover library.

---

## 2. The larger ZK compiler problem

A zero-knowledge system is often described as if it were a single object: “the circuit,” “the proof system,” or “the prover.” For compiler architecture, that is too coarse. The end-to-end problem naturally decomposes into at least five questions.

| Layer | Core question | Representative systems / objects |
|---|---|---|
| Source / circuit / VM layer | What computation or relation is the user trying to prove? | DSL ASTs, circuit builders, zkVM traces, components, source-level modules |
| Arithmetic layer | What algebraic relation or trace is actually being proved? | R1CS, Plonkish relations, AIR, ACIR, SIEVE, arithmetic summaries |
| Protocol layer | What prover and verifier do as a cryptographic protocol? | commitments, openings, oracle roots, queries, transcript stages, proof slots, verifier checks |
| Verifier layer | What exactly constitutes acceptance? | explicit proof readback, transcript replay, algebraic and protocol checks |
| Execution layer | How is the closed protocol executed efficiently? | kernels, schedules, memory plans, device placement, runtime replay |

Most existing work is strongest at the first two layers. That work is essential, and this project depends on it. But it still leaves a crucial boundary underexposed: the boundary at which an arithmetic object becomes a **closed prover/verifier protocol**.

That boundary is the role of Protocol IR.

---

## 3. Where this project sits in the research landscape

The wider ZK compiler space can be read as a map of partially overlapping agendas.

### 3.1 Frontend and circuit-language agenda

This agenda asks how users express circuits, constraints, or zkVM computations. It includes language design, embedded DSLs, frontend ergonomics, source diagnostics, and authoring abstractions.

### 3.2 Arithmetic IR and interoperability agenda

This agenda asks how to normalize the algebraic object being proved so that analyses, optimizations, and backend lowerings can be reused across languages and frameworks. This is where systems such as ACIR, SIEVE, LLZK, and other arithmetic IR efforts naturally sit.

### 3.3 Protocol-structural agenda

This agenda asks how arithmetic structure becomes a **concrete prover/verifier protocol**: how proof-visible objects are organized, how the transcript is defined, how zero-knowledge masking is inserted, how verifier checks are made explicit, and how family-specific protocol objects are fixed before execution lowering.

This is the agenda centered by the current project.

### 3.4 Execution-compiler agenda

This agenda asks how a fixed protocol is lowered to kernels, scheduled, memory-planned, and replayed efficiently across hardware targets. It is analogous to the schedule/codegen/runtime side of other compiler stacks.

### 3.5 Validation and assurance agenda

This agenda asks whether circuits, compilers, provers, and verifiers are sound, complete, under-constrained, or security-broken. It includes validation passes, formal tooling, fuzzing, mutation testing, and verification-oriented compiler structure.

The present project is **not** trying to collapse all of these agendas into one paper. It is trying to make the architecture honest enough that each agenda has a clear place.

---

## 4. Architectural thesis of this project

The project is organized around one methodological rule and one semantic rule.

### 4.1 Methodological rule

> **Protocol instantiation must happen before kernelization.**

Security choices, transcript discipline, proof shape, verifier completeness, query structure, commitment grouping, and family-specific protocol objects all change the cryptographic procedure itself. They are not merely backend scheduling details.

### 4.2 Semantic rule

> **Protocol IR is the semantic lock-in layer between arithmetic structure and execution structure.**

This means:

- arithmetic IR answers **what is being proved**;
- Protocol IR answers **what prover and verifier do**;
- execution IR answers **how the fixed protocol is run efficiently**.

That separation is the center of gravity of the project. It is also what makes the broader architecture compatible with both a current library-backed path and a future native kernel backend.

---

## 5. End-to-end system overview

The cleanest integrated architecture is the following.

```text
Protocol intent side
  Protocol DSL / builder / expert input
    -> zk.scheme

Arithmetic / circuit side
  ZK DSL / zkVM / circuit frontend
    -> arithmetic IR / normalization / analysis
    -> family-aware arithmetic view
       -> zk.r1cs | zk.plonkish | zk.air     (conceptual family views)
    -> arithmetic summaries for protocol construction

Protocol compilation
  zk.scheme + arithmetic view + SecurityProfile + TargetProfile
    -> zk.proto.core
    -> zk.proto.<class>.closed
    -> zk.proto.<class>.opt
    -> zk.verify

Backend boundary
    -> { LibraryBackend , KernelBackend }

Execution compilation
    -> zk.kernel
    -> zk.plan / zk.memplan
    -> bundle / runtime replay
```

Two points are important.

First, the system is **conceptually family-aware**. Protocol construction still needs family-specific arithmetic facts.

Second, the system should avoid bespoke one-off importers from every frontend directly into protocol code. A reusable arithmetic-side substrate is therefore valuable, but it remains **upstream of** the protocol boundary rather than replacing it.

---

## 6. Role of each major layer

### 6.1 `zk.scheme`: protocol intent

`zk.scheme` is the canonical form of user-facing protocol intent.

It says what sort of proof system is being assembled and what logical objects or constraints the protocol should expose, but it does not yet define a complete prover/verifier procedure.

### 6.2 Arithmetic substrate and family views

The arithmetic side owns the relation or trace being proved. It is the right place for source-language normalization, arithmetic analysis, interoperability structure, and family-aware summaries.

Protocol construction should not depend on raw frontend syntax. It should depend on a **family-aware arithmetic view** such as:

- R1CS-like relation summaries,
- Plonkish table/lookup/permutation summaries,
- AIR-style trace and transition summaries,
- public, witness, fixed, and preprocessed artifact schemas,
- and other arithmetic facts that materially affect protocol closure.

### 6.3 `zk.proto.*`: protocol compilation

This is the center of the current project.

`zk.proto.*` turns protocol intent and arithmetic structure into:

- a protocol skeleton,
- then a closed protocol,
- then an optimized but semantics-preserving protocol form.

At this layer, the compiler fixes the prover/verifier procedure itself: proof-visible objects, transcript discipline, verifier structure, proof ABI, and closure obligations.

### 6.4 `zk.verify`: verifier structure

Verifier structure is made explicit rather than hidden inside a proving library callback or backend convention.

This matters both for correctness and for architecture. A protocol is not closed unless the verifier relation is explicit enough to be complete.

### 6.5 `zk.kernel`, `zk.plan`, `zk.memplan`: execution structure

These layers belong to the future native execution compiler.

They decide how to realize a closed protocol efficiently, not what the protocol means.

---

## 7. Arithmetic-side integration

The architecture benefits from a reusable arithmetic-side substrate, but that substrate is **not** the main subject of this project.

The key principle is simple:

> **Arithmetic IR owns circuit and relation semantics; Protocol IR owns cryptographic protocol semantics.**

In the integrated system, a substrate such as **LLZK** is a good fit on the arithmetic side because it already aims to provide MLIR-native circuit structure, reusable analysis, and shared lowering infrastructure. That makes it a plausible source of the arithmetic facts that Protocol IR needs.

At the same time, the project should resist two opposite mistakes:

1. pushing transcript, proof-ABI, and verifier-relation semantics down into arithmetic IR;
2. bypassing reusable arithmetic infrastructure and forcing Protocol IR to ingest every frontend language directly.

The right boundary is therefore:

```text
frontend / zkVM
  -> arithmetic substrate
  -> family-aware arithmetic view
  -> Protocol IR
```

This keeps the arithmetic side reusable without making it responsible for protocol closure.

---

## 8. Current research scope

The broader architecture is intentionally larger than the current paper. The current scope is narrower and should remain so.

### 8.1 In scope now

The current project centers:

- a shared compiler framing for ZK proving rather than a single family-specific prover implementation;
- `zk.scheme` as a real protocol-intent input;
- arithmetic input and family-aware arithmetic summaries;
- Protocol IR as the semantic lock-in layer;
- the staged protocol path
  `zk.proto.core -> zk.proto.<class>.closed -> zk.proto.<class>.opt`;
- explicit verifier structure;
- proof ABI as a logical contract;
- obligation-carrying closure;
- a current executable boundary at
  `Protocol IR -> LibraryBackend`.

### 8.2 Explicitly not the center now

The current project does **not** center:

- full frontend language design;
- replacement of every arithmetic IR effort with a new one;
- full native kernel and runtime compilation;
- broad autotuning or large search spaces;
- maximal multi-family unification in the first paper;
- or a complete end-to-end proving platform in one step.

Those directions matter, but they are not the current center.

### 8.3 Scope statement in one sentence

The current contribution is:

> **Given protocol intent, arithmetic structure, and a security profile, compile a verifier-explicit closed protocol before any execution lowering begins.**

That is the correct research-sized slice.

---

## 9. Relationship to the other canonical documents

This document owns the **overview and scope**. The other canonical documents own narrower pieces.

### 9.1 `protocol-ir.md`

Owns:
- the core research thesis,
- the semantic role of Protocol IR,
- closure conditions,
- contract systems,
- protocol profiles,
- correctness story,
- and evaluation focus.

### 9.2 `implementation-spec.md`

Owns:
- the exact MLIR object model,
- dialect packaging,
- symbol model,
- types and attributes,
- pass structure,
- verifier IR shape,
- and Transform integration.

### 9.3 `extensions-and-roadmap.md`

Owns:
- future family specialization,
- semantic enrichment above arithmetic IR,
- native execution compilation,
- transform/search extensions,
- and medium-/long-term open questions.

This separation is intentional. The project becomes much clearer when the overall motivation, the semantic thesis, the implementation object model, and the extension roadmap are not blended together.

---

## 10. Final position

The cleanest summary of the architecture is:

> **This project studies a shared MLIR-native compiler framework for zero-knowledge proving, in which arithmetic structure, protocol structure, verifier structure, and execution structure are given distinct places rather than being entangled inside family-specific prover libraries.**

And the cleanest summary of the current scope is:

> **The current paper is about Protocol IR as the layer that closes a prover/verifier protocol before execution lowering begins.**

Those two sentences capture the intended reading of the whole source set.

---

## Selected references

### Compiler architecture and MLIR

- [MLIR] Chris Lattner et al. *MLIR: Scaling Compiler Infrastructure for Domain-Specific Computation*. CGO 2021.
- [Transform] Martin Paul Lücke et al. *The MLIR Transform Dialect*. 2024.
- [Verification Dialects] Mathieu Fehr et al. *First-Class Verification Dialects for MLIR*. 2025.
- [HEIR] Asra Ali et al. *HEIR: A Universal Compiler for Homomorphic Encryption*. 2025.

### ZK compilers and IRs

- [CirC] Alex Ozdemir, Fraser Brown, Riad S. Wahby. *CirC: Compiler Infrastructure for Proof Systems, Software Verification, and More*.
- [Noir ACIR] Noir documentation.
- [zkInterface] zkInterface / SIEVE interoperability references.
- [ZK Survey] *Zero-Knowledge Proof Frameworks: A Survey*.

### Arithmetic-side substrate examples

- [LLZK Overview] official LLZK overview and documentation.
- [LLZK Repo] LLZK repository.
- [LLZK Blog] Veridise announcement of LLZK.

### Validation and assurance

- [MTZK] mutation testing for ZK compilers.
- [Circuzz] fuzzing/testing work for arithmetic circuits and compilers.

---

## Reference links

[MLIR]: https://dl.acm.org/doi/10.1109/CGO51591.2021.9370308
[Transform]: https://arxiv.org/abs/2409.03864
[Verification Dialects]: https://users.cs.utah.edu/~regehr/papers/pldi25.pdf
[HEIR]: https://arxiv.org/abs/2508.11095

[CirC]: https://eprint.iacr.org/2020/1586
[Noir ACIR]: https://noir-lang.org/docs/
[zkInterface]: https://docs.zkproof.org/pages/standards/accepted-workshop3/proposal-zkinterface.pdf
[ZK Survey]: https://arxiv.org/abs/2502.07063

[LLZK Overview]: https://project-llzk.github.io/llzk-lib/
[LLZK Repo]: https://github.com/project-llzk/llzk-lib
[LLZK Blog]: https://veridise.com/blog/zero-knowledge/announcing-llzk-a-unified-open-source-intermediate-representation-ir-for-zero-knowledge-languages/

[MTZK]: https://www.ndss-symposium.org/ndss-paper/mtzk-testing-and-exploring-bugs-in-zero-knowledge-zk-compilers/
[Circuzz]: https://aisychev.github.io/papers/ccs25-circuzz.pdf
