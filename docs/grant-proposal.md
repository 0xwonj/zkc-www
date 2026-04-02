# Grant Proposal

## Project Abstract

Today many prover stacks are still fragmented and library-centric: protocol logic, transcript discipline, verifier construction, and optimization strategy are often hand-crafted inside prover-specific codebases. The longer-term goal is a standardized ZK compiler stack that makes soundness-critical structure explicit, provides reusable correctness-aware optimization passes, and improves not only performance but also maintenance, implementation reuse, and research iteration speed. This project targets one missing boundary in that stack: the stage where an arithmetic object becomes a closed prover/verifier protocol. It proposes **Protocol IR** as the semantic lock-in layer for that boundary, and will validate it through a focused prototype and one end-to-end proof/verify path.

## Objectives

The main goal of this grant is to define and validate **Protocol IR** as a first-class compiler layer between arithmetic structure and execution structure, following the rule that **protocol instantiation should happen before kernelization**. In practical terms, this means turning protocol closure into an explicit, checkable compiler phase rather than leaving transcript construction, verifier structure, and proof layout embedded inside prover-specific libraries.

Success for this project will mean:

* a clear Protocol IR design centered on the semantic triple **(proof surface, transcript trace, verifier relation)**;
* explicit handling of Fiat–Shamir as protocol compilation rather than a backend helper;
* a staged workflow from unresolved protocol skeleton to closed protocol and optimized protocol form;
* a focused executable path through one library backend, likely a narrow [Plonky3]-backed path;
* public release of design documents, prototype code, and a technical report suitable to serve as the basis for a paper or preprint.

## Outcomes

The main ecosystem benefit is a clearer and more reusable compiler boundary for ZK systems. Instead of treating protocol logic as hand-crafted library code, this project aims to make transcript structure, verifier replay, proof ABI, and closure obligations explicit compiler objects. That makes soundness-critical structure easier to inspect, validate, and optimize.

More broadly, this work supports a healthier ZK tooling ecosystem. A standardized protocol layer can make backend integration less ad hoc, enable reusable optimization passes, reduce implementation ambiguity, and make future research and engineering iteration faster. The outputs are intended to be open and reusable: public design documents, a prototype implementation, and a focused case study that other compiler, proof-system, and formal-methods efforts can build on.

## Grant Scope

### Base scope: 6 months

This grant focuses on the current research center of the project: **verified protocol instantiation before kernelization**. The project will research and prototype:

* a protocol-facing input path from a minimal DSL or protocol-intent form;
* a Protocol IR that represents the prover/verifier procedure itself;
* Fiat–Shamir-aware protocol closure with explicit transcript stages, proof-surface bindings, verifier replay, and admissibility obligations;
* explicit verifier structure and proof ABI as semantic objects;
* a staged protocol path:

  * `zk.proto.core`
  * `zk.proto.<class>.closed`
  * `zk.proto.<class>.opt`
* one focused executable path to a library backend, with an end-to-end proof/verify roundtrip.

The expected outputs for the base grant are:

* public design documents for the architecture and core semantics;
* a working prototype of the core Protocol IR stages and closure machinery;
* one focused backend realization and end-to-end case study;
* an open-source artifact release and technical report / manuscript draft.

### Optional continuation: 3 months

An optional 3-month extension would support **publication-oriented consolidation**. This phase would focus on:

* stronger evaluation and deeper case-study evidence;
* artifact hardening and improved documentation;
* refinement of claims and related-work positioning;
* manuscript completion and submission.

The extension is intended as a continuation subject to progress and mutual agreement, rather than as a hard dependency of the 6-month base scope.

## Related Work

This project sits at the boundary of compiler architecture, ZK compiler systems, Fiat–Shamir theory, and verification-oriented infrastructure. On the compiler side, the most relevant precedents are [MLIR], the [Transform Dialect], [HEIR], and [Verification Dialects]. These works justify a staged, legality-driven compiler architecture in which semantics are explicit in the IR and transformation control is separated from payload semantics.

On the ZK side, important systems include [CirC], [Noir/ACIR], [zkInterface], and [LLZK]. These efforts validate arithmetic IRs, backend-facing proof IRs, circuit compilers, and interoperability layers. However, they do not center the same compiler boundary as this project. The specific gap addressed here is the lack of a compiler layer that makes **protocol closure itself**—including transcript structure, verifier relation, proof ABI, and closure obligations—the primary object of compilation.

On the cryptographic side, recent work such as [Weak Fiat-Shamir Attacks], [Fiat-Shamir in the Wild], and [FRI Fiat-Shamir], together with engineering frameworks like [Merlin] and [Decree], strongly motivates making transcript discipline compiler-visible. This project takes that lesson seriously: Fiat–Shamir is treated as typed protocol closure, not as generic “hash the transcript so far” plumbing.

## Project Team

**Number of people working on this project:** 1

* **[Applicant Name]** — Principal Researcher / Sole Investigator
  PhD student, Seoul National University
  Role: research design, protocol semantics, prototype implementation, backend integration, evaluation, documentation, and manuscript preparation
  Base scope effort: **160 hours/month** for 6 months
  Optional extension effort: **120 hours/month** for 3 months

## Background

I am a PhD student at Seoul National University working on zero-knowledge systems, compiler architecture, and protocol design. This proposal is part of a broader research agenda on ZK compiler structure, but the funded scope here is deliberately narrow and concrete: make the protocol layer explicit and executable.

The project is already backed by a written design corpus, including a system overview, Protocol IR thesis, implementation plan, Fiat–Shamir design note, roadmap, and reference stack. These documents define the current scope, semantic commitments, and implementation direction, and they are intended to be published openly.

Relevant materials to review:

* [System and Scope]
* [Protocol IR]
* [Implementation Spec]
* [Fiat–Shamir in Protocol IR]
* [Extensions and Roadmap]
* [Reference Stack]
* [Project Notes / Blog]

This is an early-stage solo research effort rather than a large existing lab project, so the strongest evidence of readiness is the depth of the existing design work and the narrowness of the proposed execution path. The scope is intentionally sized to one focused vertical slice rather than a full prover compiler.

## Methodology

The methodology follows four steps.

### 1. Fix the protocol boundary explicitly

The project starts from the claim that arithmetic structure is not yet protocol structure, and protocol structure is not yet execution structure. Protocol IR is introduced as the layer that turns an arithmetic object into a closed prover/verifier protocol with explicit semantics. The design is aligned with the project architecture in [System and Scope] and the main thesis in [Protocol IR].

### 2. Compile Fiat–Shamir at the protocol layer

Fiat–Shamir will be modeled as protocol compilation, not as a proving-library helper. The prototype will make transcript stages, proof-surface bindings, challenge classes, verifier replay, and admissibility obligations explicit, following the design in [Fiat–Shamir in Protocol IR]. This is motivated both by the theory of modern multi-round Fiat–Shamir and by practical evidence that transcript construction errors are common in deployed systems ([Weak Fiat-Shamir Attacks], [Fiat-Shamir in the Wild], [FRI Fiat-Shamir]).

### 3. Realize staged closure in a focused prototype

The implementation will realize the staged progression from unresolved protocol skeleton to closed protocol and then optimized protocol form. Internally, the design is organized around four first-class resources: transcript, entropy, proof surface, and obligations, following the implementation direction in [Implementation Spec]. The funded prototype will stay narrow: it will target one focused library-backed vertical slice rather than full native execution compilation.

### 4. Evaluate for semantic completeness and executability

Evaluation will focus on whether the compiler makes the right structures explicit and whether invalid protocol constructions are rejected. This includes negative tests for malformed transcript or verifier structure, proof ABI and replay discipline, and one focused end-to-end proof/verify roundtrip. The design will remain compatible with the longer-term LLZK-backed direction described in [System and Scope] and [Extensions and Roadmap], but the funded scope stays centered on the protocol layer itself.

## Timeline

### Base scope: 6 months

**Hourly rate:** USD 50/hour
**Total effort:** 960 hours
**Base labor request:** USD 48,000
**Project software cost:** USD 1,200

### Milestone 1: Protocol IR design finalization

**Budget:** USD 6,000
**Number of hours (roughly):** 120 hours

Finalize the protocol-layer thesis, closure conditions, proof ABI model, verifier-explicit structure, and Fiat–Shamir compilation strategy.

Subtasks:

* consolidate core design documents;
* refine the closed-protocol semantic model;
* finalize the narrow prototype boundary;
* prepare public-facing architecture and design material.

**Deliverables:** updated design notes, public documentation draft, implementation-ready specification.

### Milestone 2: Core prototype and closure machinery

**Budget:** USD 18,000
**Number of hours (roughly):** 360 hours

Implement the core Protocol IR prototype and staged closure flow.

Subtasks:

* implement the `core -> closed -> opt` skeleton;
* implement proof-surface bindings and transcript flow;
* implement verifier replay structure;
* implement closure checks for transcript legality and verifier completeness.

**Deliverables:** working core prototype, closure machinery, initial rejection tests.

### Milestone 3: Focused backend realization and end-to-end case study

**Budget:** USD 15,000
**Number of hours (roughly):** 300 hours

Build one focused executable path through a library backend and demonstrate an end-to-end proof/verify roundtrip.

Subtasks:

* define a minimal protocol-facing input slice;
* connect one narrow protocol path to a library backend;
* integrate an initial backend realization, likely [Plonky3]-backed;
* document proof ABI, transcript, and verifier behavior for the case study.

**Deliverables:** one focused backend path, end-to-end proof/verify roundtrip, public case-study artifact.

### Milestone 4: Evaluation, artifact release, and technical report

**Budget:** USD 9,000
**Number of hours (roughly):** 180 hours

Evaluate the prototype, package public artifacts, and prepare paper-ready technical documentation.

Subtasks:

* run semantic and safety-oriented validation;
* prepare negative and regression tests;
* harden repository structure and documentation;
* write a technical report / manuscript draft;
* publish the artifact and supporting materials.

**Deliverables:** open-source artifact release, technical report / manuscript draft, evaluation summary.

### Project-wide software line item

**Budget:** USD 1,200

AI subscription costs for research and development support during the 6-month base scope.

**Deliverables / use:** AI-assisted drafting, implementation support, and research workflow acceleration during the funded project period.

### Optional continuation: Months 7–9

**Hourly rate:** USD 50/hour
**Total effort:** 360 hours
**Optional continuation amount:** USD 18,000

### Milestone 5: Publication-oriented consolidation and submission

**Budget:** USD 18,000
**Number of hours (roughly):** 360 hours

Use the additional period to deepen evaluation, improve artifact quality, and complete a paper submission.

Subtasks:

* strengthen case-study evidence and evaluation depth;
* improve documentation and reproducibility materials;
* refine claims and related-work framing;
* complete manuscript polishing and submission.

**Deliverables:** paper-quality manuscript, submission or public preprint, finalized artifact package.

## Budget

### Requested amount

* **Base grant request (6 months): USD 49,200**

  * Principal researcher labor: USD 48,000
  * Software costs: USD 1,200
* **Optional 3-month continuation: USD 18,000**
* **Total including optional continuation: USD 67,200**

### Budget breakdown

#### Base scope (6 months)

* **Principal Researcher Costs:** USD 48,000

  * 960 hours × USD 50/hour
* **Other Staff Costs:** USD 0
* **Hardware Costs:** USD 0
* **Software Costs:** USD 1,200

  * AI subscription costs during the 6-month base scope
* **Data Collection Costs:** USD 0
* **Indirect Costs:** USD 0

#### Optional continuation (3 months)

* **Principal Researcher Costs:** USD 18,000

  * 360 hours × USD 50/hour
* **Other Staff Costs:** USD 0
* **Hardware Costs:** USD 0
* **Software Costs:** USD 0
* **Data Collection Costs:** USD 0
* **Indirect Costs:** USD 0

---

[System and Scope]: https://hackmd.io/@wonj/system-and-scope
[Protocol IR]: https://hackmd.io/@wonj/protocol-ir
[Implementation Spec]: https://hackmd.io/@wonj/mlir-spec
[Fiat–Shamir in Protocol IR]: https://hackmd.io/@wonj/fiat-shamir
[Extensions and Roadmap]: https://hackmd.io/@wonj/extensions-and-roadmap
[Reference Stack]: https://hackmd.io/@wonj/references
[Project Notes / Blog]: https://example.org/protocol-ir/notes
[MLIR]: https://dl.acm.org/doi/10.1109/CGO51591.2021.9370308
[Transform Dialect]: https://arxiv.org/abs/2409.03864
[Verification Dialects]: https://users.cs.utah.edu/~regehr/papers/pldi25.pdf
[HEIR]: https://arxiv.org/abs/2508.11095
[CirC]: https://eprint.iacr.org/2020/1586
[Noir/ACIR]: https://noir-lang.org/docs/
[zkInterface]: https://docs.zkproof.org/pages/standards/accepted-workshop2/proposal--zk-interop-zkinterface.pdf
[LLZK]: https://project-llzk.github.io/llzk-lib/
[Plonky3]: https://github.com/Plonky3/Plonky3
[Weak Fiat-Shamir Attacks]: https://eprint.iacr.org/2023/691
[Fiat-Shamir in the Wild]: https://eprint.iacr.org/2024/1565
[FRI Fiat-Shamir]: https://eprint.iacr.org/2023/1071.pdf
[Merlin]: https://github.com/dalek-cryptography/merlin
[Decree]: https://github.com/trailofbits/decree
