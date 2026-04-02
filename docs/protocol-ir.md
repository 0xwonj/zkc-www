# Protocol IR

**Status:** canonical research document  
**Owns:** the research thesis, semantic role, stage structure, closure conditions, contracts, protocol profiles, correctness story, backend boundary, and evaluation focus of Protocol IR  
**Does not own:** exact MLIR op/type definitions, verifier op inventories, pass-by-pass implementation details, or long-range extension design  
**Depends on:** `system-and-scope.md`

---

## Abstract

This document defines the current core research contribution of the project: **Protocol IR as the semantic lock-in layer between arithmetic structure and execution structure**.

Arithmetic IR describes the algebraic object being proved. Execution IR describes how a fixed prover is realized efficiently on concrete hardware. Protocol IR describes the cryptographic procedure itself: what prover and verifier do, what enters the transcript, how proof-visible objects are organized, which checks define acceptance, and which security and zero-knowledge obligations must be discharged before execution lowering begins.

The central claim is that a prover compiler should not stop at arithmetic import and should not jump directly to kernel generation. It should first compile an unresolved protocol skeleton into a **closed, verifier-explicit, proof-ABI-aware, cryptographically complete protocol**. A closed protocol is best understood as a semantic triple:

$$
\llbracket P \rrbracket = (\text{proof surface},\ \text{transcript trace},\ \text{verifier relation})
$$

This document makes six commitments. First, protocol instantiation happens before kernelization. Second, Fiat–Shamir is treated as a protocol-compilation problem rather than a library helper. Third, proof ABI is a logical contract, not a serializer afterthought. Fourth, verifier structure is first-class. Fifth, the common protocol core is intentionally small and is extended by explicit family features and protocol profiles. Sixth, the current research scope ends at executable lowering to a `LibraryBackend`; native kernel lowering is future work over the same closed protocol semantics.

The result is not “another ZK circuit IR.” It is a proposal for **compiling protocol structure itself**.

---

## 1. Purpose

This document states the **paper-level thesis and semantics** of Protocol IR.

It answers the following questions:

1. Why a prover compiler needs a protocol layer distinct from arithmetic IR and execution IR.
2. What semantic object a closed protocol is.
3. What stages Protocol IR passes through before backend lowering.
4. Which contracts must be enforced before a protocol is considered closed.
5. How Protocol IR is positioned relative to existing ZK IR/compiler work.
6. What the current paper should claim, and what it should deliberately leave to `implementation-spec.md` and `extensions-and-roadmap.md`.

This document is intentionally not an implementation manual. Exact MLIR objects, dialect packaging, verifier ops, pass names, and Transform control details belong to `implementation-spec.md`. Broader upward and downward extensions belong to `extensions-and-roadmap.md`. The present document owns the **research boundary and semantic commitments**.

---

## 2. Central thesis

The project’s central thesis is:

> A prover compiler should compile **protocol structure**, not merely arithmetic structure and not merely execution structure.

This thesis has four parts.

First, **arithmetic structure is not yet protocol structure**. Constraint systems, polynomial systems, and execution traces describe what must be established, but they do not by themselves define the full prover/verifier procedure.

Second, **protocol structure is not execution structure**. Commitment grouping, transcript order, challenge schedule, decommitment shape, proof layout, and verifier completeness all change the cryptographic object being executed. They are not backend scheduling details.

Third, **protocol closure is a compiler phase**. It is the phase that resolves abstract handles into concrete protocol objects, compiles public-coin structure into transcript structure, injects zero-knowledge machinery, finalizes proof-visible layout, emits verifier checks, and discharges obligations.

Fourth, **only after closure should execution lowering begin**. Library lowering and future kernel lowering must consume the same closed protocol semantics. Otherwise the compiler is optimizing an object whose meaning is still unstable.

This thesis refines the system-level architecture in `system-and-scope.md` into a precise research claim: **verified protocol instantiation before kernelization**.

---

## 3. Position in the compiler architecture

The full architecture is:

```text
Protocol frontend
  Protocol DSL / builder / expert form
    -> zk.scheme

Arithmetic / proving-instance side
  -> zk.r1cs | zk.plonkish | zk.air
  -> zk.sem                         (future / optional)

Protocol compilation
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

The current project ends at:

```text
zk.proto.<class>.opt
  -> LibraryBackend
  -> proof / verify roundtrip
```

The later system extends the same closed protocol semantics to:

```text
zk.proto.<class>.opt
  -> KernelBackend
  -> zk.kernel
  -> zk.plan / zk.memplan
```

Protocol IR therefore is not a side path around the full compiler. It is the boundary that makes the full compiler architecture coherent.

---

## 4. Why Protocol IR is a distinct research layer

A prover implementation usually entangles at least five concerns:

- scheme intent,
- arithmetic structure,
- transcript discipline,
- verifier construction,
- and execution strategy.

That entanglement is a major source of brittleness. Security-relevant protocol choices become hidden inside proving libraries; proof layout becomes accidental; verifier logic becomes partially implicit; and performance engineering is forced to work on structures whose cryptographic meaning is not made explicit.

Protocol IR separates these concerns by giving the compiler an intermediate layer whose subject matter is the **prover/verifier procedure itself**.

The key methodological rule is:

> **Protocol instantiation must happen before kernelization.**

This rule is not aesthetic. It follows from the fact that protocol choices such as transcript order, query schedule, batching, proof-surface structure, decommitment organization, and zero-knowledge masking all change the actual proof system. If these choices remain unresolved during backend lowering, then the backend is not merely optimizing execution. It is silently participating in protocol design.

That is the wrong compiler boundary.

---

## 5. Position relative to related work

Protocol IR is informed by two major literatures: **multi-level compiler architecture** and **ZK proof/compiler systems**.

MLIR provides a natural architectural precedent for multi-level, dialect-based compiler design, staged legality-driven lowering, reusable interfaces, and separation between payload semantics and transformation control ([MLIR], [Dialect Conversion], [Transform]). Recent work on first-class verification dialects strengthens the case for making semantic and verification structure explicit in the IR rather than implicit in external tooling ([Verification Dialects]). HEIR shows that a cryptography compiler can benefit from layered, MLIR-native abstractions even when the application domain differs from ZK ([HEIR]).

On the ZK side, existing systems already show the value of compilerization, but they generally center different layers of the stack. CirC is a compiler for proof systems and is one of the closest precedents for taking proving seriously as a compiler problem ([CirC]). Noir’s ACIR is a deployed backend-facing proof IR that sits between a frontend language and proving backends ([Noir ACIR]). zkInterface/SIEVE is an interoperability-oriented IR and file format for exchanging ZK statements across components ([zkInterface], [SIEVE]). zkLLVM is explicitly a circuit compiler that lowers high-level programs into algebraic-circuit inputs for provable computation protocols ([zkLLVM]).

These systems are important precedents, but they do not make the same layer primary. The present project does **not** claim that no prior system has protocol structure anywhere inside it. The claim is narrower and more precise:

> Existing work strongly validates arithmetic IRs, backend-facing proof IRs, circuit compilers, and interoperability layers; this project centers **verifier-explicit protocol closure itself** as a first-class compiler boundary.

This distinction matters most when transcript semantics, proof ABI, verifier completeness, and family-specific protocol objects must remain explicit and optimizable before execution lowering.

---

## 6. What a closed protocol means

A closed protocol should not be modeled merely as “a list of prover actions.”  
The right semantic unit is:

$$
\llbracket P \rrbracket = (\text{proof surface},\ \text{transcript trace},\ \text{verifier relation})
$$

This triple is the semantic center of Protocol IR.

### 6.1 Proof surface

The **proof surface** is the external proof-visible contract:

- which proof objects exist,
- their kinds and grouping,
- their order and readback discipline,
- and which parts are logically stable across repeated proofs.

This is the logical core of the **proof ABI**.

### 6.2 Transcript trace

The **transcript trace** records:

- which semantic subjects are absorbed,
- in what stage and order,
- under which namespace or domain separator,
- when challenges are sampled,
- and what transcript prefix each challenge depends on.

This is the object that transcript-preserving rewrites must preserve.

### 6.3 Verifier relation

The **verifier relation** is the explicit acceptance condition:

- algebraic checks,
- opening or decommitment checks,
- Merkle/FRI checks where relevant,
- and any protocol-specific consistency conditions.

A protocol is not closed unless the verifier relation is explicit enough to be complete.

This triple is also the basis for the correctness story. Closure constructs it. Optimization preserves it. Backend lowering realizes it.

---

## 7. Stage structure

Protocol IR has three primary stages:

```text
zk.proto.core
  -> zk.proto.<class>.closed
  -> zk.proto.<class>.opt
```

### 7.1 `zk.proto.core`

`core` is an unresolved **protocol skeleton**.

It contains:

- protocol phases and rounds,
- abstract protocol objects,
- initial proof-surface declarations,
- transcript intent,
- pending obligations,
- unresolved family-specific structure.

`core` is deliberately composable and deliberately incomplete. It is the right place to express protocol intent without yet claiming cryptographic completeness.

### 7.2 `zk.proto.<class>.closed`

`closed` is the first **cryptographically complete, verifier-explicit** form.

At this stage:

- all abstract handles have been resolved,
- the protocol class/profile is fixed,
- the proof surface is finalized enough to be meaningful,
- transcript legality has been checked,
- zero-knowledge and security obligations have been discharged,
- verifier structure is explicit and complete,
- witness-independent proof topology has been established.

This is the semantic lock-in point.

### 7.3 `zk.proto.<class>.opt`

`opt` is a **semantics-equivalent optimized protocol**.

This stage exists to preserve an essential distinction:

- **closure legality** answers whether the protocol is admissible and complete;
- **protocol optimization** answers whether the same closed semantics can be reorganized more efficiently.

Examples include commitment regrouping, query replanning, transcript-preserving canonicalization, and proof-shape stabilization.

### 7.4 Internal Fiat–Shamir lowering

Conceptually, the transition from `core` to `closed` contains an internal step:

```text
public-coin protocol skeleton
  -> fs.lower
  -> transcriptized protocol skeleton
```

This does not need to surface as a public top-level stage in the MVP. But semantically it matters. Fiat–Shamir is not an incidental hash call during proving. It is part of **protocol closure**.

---

## 8. Fiat–Shamir belongs in Protocol IR

Modern proof systems make Fiat–Shamir too semantically important to relegate to a helper function.

The theory literature shows that multi-round Fiat–Shamir is not simply the three-move story repeated several times, and that FRI/STARK-style systems require more specialized reasoning than a naive generic theorem would suggest ([Attema–Fehr–Klooß], [FRI FS]). The engineering literature shows that transcript construction errors are common in practice and can invalidate real systems even when the rest of the prover appears to work ([Weak FS Attacks], [FS in the Wild]). Transcript frameworks such as Merlin and Decree further demonstrate that stage structure, domain separation, framing, challenge typing, and verifier replay need explicit discipline, not informal convention ([Merlin], [Decree]).

Recent work on duplex-sponge Fiat–Shamir and the active IRTF draft make explicit a decomposition around an initialization context, a duplex sponge interface, and a codec, while `spongefish` and ArkLib illustrate corresponding implementation and mechanization directions ([Duplex Sponge FS], [IRTF FS Draft], [spongefish], [ArkLib]). Protocol IR should align with these external anchors at the transcript and encoding layers, while remaining distinct in its compiler-level treatment of proof-surface bindings, stage-typed transcript construction, verifier replay, and closure obligations.

The resulting compiler stance is:

> **Fiat–Shamir should be compiled, not called.**

More precisely, Protocol IR should compile a public-coin protocol skeleton into a **stage-typed transcript program** whose correctness is governed by proof-surface bindings, verifier replay, challenge policies, and admissibility obligations.

This has two consequences.

First, transcript correctness is not merely about ordering. It is about **which semantic objects** influence each challenge, and under what stage and namespace discipline.

Second, closure must discharge **two distinct obligation families**:

1. **Transcript well-formedness obligations**  
   Required subjects were absorbed, stage requirements were met, labels and domain separation are valid, verifier replay reconstructs the same challenge schedule, and proof-visible objects were bound before transcript use.

2. **Fiat–Shamir admissibility obligations**  
   The protocol belongs to a theorem-backed family/profile, the declared model is compatible with the protocol class, the challenge schedule matches the theorem’s side conditions, and the target security loss is acceptable for the declared reduction.

This is a conservative and defensible design. The compiler does not invent new cryptographic theorems. It checks that the protocol is being closed under an explicit admissibility profile.

---

## 9. Core design principles

Protocol IR is governed by the following design principles.

### 9.1 Protocol structure is first-class

The compiler should represent commitments, openings, oracle queries, transcript actions, proof-surface bindings, verifier checks, and protocol obligations as semantic protocol objects. It should not treat them as accidental byproducts of backend code generation.

### 9.2 Proof ABI is a logical contract

Proof ABI is not post hoc serialization metadata. It is part of protocol meaning:

- what proof-visible objects exist,
- in what order they appear,
- how transcript binding depends on them,
- how the verifier reads them,
- and what remains stable across repeated proofs.

### 9.3 Verifier structure is explicit

A protocol is not closed unless its verifier relation is represented explicitly enough to be complete. This supports precise correctness statements, clearer proof-surface reasoning, and a cleaner path to recursion or wrapper generation.

### 9.4 Shape matters

Protocol topology must be distinguished from witness values. A witness may change payload values, but it must not change the number, order, or kinds of proof slots, the challenge schedule, or the verifier-visible query/decommitment topology of a closed protocol.

This is the project’s **shape-stability** principle.

### 9.5 Common core, explicit profiles

The common protocol core must stay intentionally small. Richer structure should be carried by explicit feature layers and explicit protocol profiles, not by overfitting the common core to one family.

### 9.6 Closure before execution

Execution lowering must preserve already-closed protocol meaning. It must not complete protocol design by accident.

---

## 10. First-class resources

The most useful abstraction for Protocol IR is a **four-resource model**:

1. **transcript**
2. **entropy**
3. **proof surface**
4. **obligations**

### 10.1 Transcript

The transcript is the public-coin state. It records absorb events, challenge derivation, namespaces, and stage discipline. Treating it as a first-class resource is what makes transcript legality analyzable rather than conventional.

### 10.2 Entropy

Prover-private randomness must be explicit and separate from public transcript-derived challenges. This is the correct place for masking, blinding, and other zero-knowledge randomization decisions.

### 10.3 Proof surface

Proof slots, slot grouping, slot ordering, and verifier readback are semantic objects. This is what turns proof ABI from a serializer concern into a compiler concern.

### 10.4 Obligations

Delayed cryptographic and semantic requirements should be explicit IR objects. Examples include missing hiding, unresolved query plans, verifier incompleteness, transcript-stage violations, unsupported Fiat–Shamir profiles, and shape-instability conditions.

A protocol cannot become `closed` while any such obligations remain pending.

This resource split is intentionally narrower than “all mutable protocol state.” It isolates the objects that matter most for protocol correctness and optimization.

---

## 11. Four contract systems

The protocol layer is held together by four contract families.

### 11.1 Transcript / Fiat–Shamir contract

This contract guarantees:

- correct stage structure,
- absorb-before-challenge legality,
- domain separation and namespace discipline,
- challenge provenance,
- explicit verifier replay,
- and proof-surface-to-transcript consistency.

This is the compiler-level counterpart of disciplined transcript APIs such as Merlin and Decree, and is also compatible with the more explicit duplex-sponge/interface style emerging in recent specification work ([Merlin], [Decree], [IRTF FS Draft]).

### 11.2 Entropy / zero-knowledge contract

This contract guarantees:

- prover-private randomness is explicit,
- hiding obligations are not buried inside backend code,
- masking/blinding occurs before closure completes,
- transcript randomness and prover entropy are not conflated.

### 11.3 Security contract

Security must be represented as a structured protocol profile rather than a single scalar. A useful decomposition is:

- **assumptions**
- **budget vector**
- **capabilities**

This allows different protocol components to require different assumptions or capabilities without collapsing everything into one numeric target.

### 11.4 Verifier / ABI / shape contract

This contract guarantees:

- verifier completeness,
- stable proof ABI,
- explicit proof readback,
- witness-independent proof topology.

This contract is one of the strongest project differentiators because it connects build-once/prove-many compilation to protocol semantics rather than only to backend engineering.

---

## 12. Proof ABI and transcript binding

One of the most important consequences of the previous sections is that **proof-visible objects should enter transcript semantics through proof-surface bindings**.

The core idea is:

> For proof-visible subjects, transcript meaning should be defined over the bound proof subject, not merely over an ephemeral internal value.

This unifies five things that are often treated separately:

- proof layout,
- transcript influence,
- verifier replay,
- proof ABI stability,
- and challenge provenance.

At the implementation level this often corresponds to preferring “absorb the slot” over “absorb the raw value,” but the present document is making a stronger research claim: **proof-surface structure and transcript semantics are coupled by design**.

This is what makes proof ABI a semantic contract instead of an afterthought.

---

## 13. Protocol classes, profiles, and feature layering

A major design constraint emerging from the reference stack is that the common protocol core must remain small.

A universal `commit/open/eval` language would be too PCS-centric. It does not cleanly capture oracle-heavy families, especially STARK-style systems whose natural objects include oracle roots, query surfaces, decommitment bundles, OODS/DEEP reductions, FRI schedules, and Merkle path plans ([Winterfell], [RISC Zero], [Cairo], [AirScript]).

The correct design is therefore:

- a **small common core** that owns transcript, entropy, proof surface, obligations, and verifier scaffolding;
- **feature layers** that encode richer reusable protocol structure;
- an explicit **protocol class/profile** that names the family identity of the assembled protocol.

Conceptually, the structure is:

```text
zk.proto.core
  + feature layers
  + zk.verify
  => zk.proto.<class>.closed
```

Representative feature layers include:

- oracle structure,
- PCS structure,
- FRI,
- Merkle/decommitment structure,
- OODS/DEEP structure,
- lookup/permutation structure.

Representative classes include:

- `stark`
- `plonkish`
- `marlin`
- `sigma`

This design gives the best of both worlds. Internally, the compiler remains compositional. Externally, the family identity stays explicit.

---

## 14. Why STARKs are the hardest and most important stress test

Protocol IR is not a STARK-only project, but STARKs provide the strongest architectural stress test.

In a STARK-oriented pipeline, the proof object is naturally structured around:

- committed execution or auxiliary traces,
- challenge-dependent reductions,
- oracle roots,
- query surfaces,
- decommitments,
- low-degree testing schedules,
- and verifier logic that is visibly oracle-centric.

Winterfell’s AIR interface and randomized multi-stage traces make this clear in an implementation-facing way ([Winterfell]). RISC Zero’s public documentation makes the DEEP-ALI and FRI structure explicit ([RISC Zero]). Cairo and AirScript show the practical importance of trace structure and AIR-level organization ([Cairo], [AirScript]).

This matters for Protocol IR because it rules out a false form of generality. The common core should not pretend that all families reduce naturally to one PCS-flavored surface language. Instead, it should stay small enough that STARK/oracle structure can appear as first-class protocol objects without distorting the rest of the design.

That is why the project treats STARK/oracle specialization as the highest-value family extension after the current paper, not as an afterthought.

---

## 15. Closure

Protocol closure is the compiler phase that turns a composable but incomplete protocol skeleton into a fixed cryptographic object.

At a research level, closure has six responsibilities.

### 15.1 Resolve abstract protocol objects

Arithmetic structure and protocol class information are used to materialize the family-specific objects the protocol actually manipulates: commitment bundles, oracle roots, point/query plans, decommitment bundles, reductions, and so on.

### 15.2 Compile public-coin structure into transcript structure

If the protocol is public-coin and uses Fiat–Shamir, the compiler constructs the corresponding transcript discipline, challenge schedule, and replay structure, subject to transcript and admissibility contracts.

### 15.3 Synthesize zero-knowledge machinery

Hiding requirements become explicit masking or blinding structure before the protocol is considered complete.

### 15.4 Finalize the proof surface

Proof-visible objects are frozen into a proof ABI meaningful enough for verifier replay and backend lowering.

### 15.5 Emit verifier structure

All verifier-relevant checks must be explicit by the end of closure.

### 15.6 Discharge obligations

A protocol cannot become `closed` if unresolved obligations remain. Closure failure is a first-class compiler outcome, not a debugging inconvenience.

The output of closure is a backend-neutral protocol meaning. It is complete enough to execute through a library backend and precise enough to serve as the contract consumed by a future kernel backend.

---

## 16. Protocol optimization

Protocol optimization comes **after** closure.

Its job is not to rescue an incomplete protocol. Its job is to reorganize a closed protocol without changing its semantic triple.

Representative optimization families include:

- commitment regrouping,
- query/decommitment replanning,
- transcript-preserving canonicalization,
- proof-surface regularization,
- family-specific cost-guided rewrites.

The key point is methodological:

> Protocol-level optimization is a real compiler problem, distinct from execution optimization.

This insight is closely aligned with MLIR’s separation between payload semantics and transformation control ([Transform]). However, this document does not define the control plane itself. The current claim is simply that post-closure protocol rewrites are meaningful, legality-constrained, and distinct from backend scheduling.

---

## 17. Backend boundary

The output of Protocol IR should be **backend-neutral closed semantics**.

This allows two backend tiers.

### 17.1 Current backend: `LibraryBackend`

This is the current research boundary and the right first executable target.

It demonstrates that:

- Protocol IR is not merely descriptive,
- closure produces a real prover/verifier object,
- explicit verifier structure and proof ABI are executable,
- and protocol optimization has concrete downstream consequences.

### 17.2 Future backend: `KernelBackend`

This is the longer-term systems path.

It lowers the same closed protocol semantics into explicit kernels, plans, and memory policies. Importantly, it does **not** change protocol meaning. It realizes an already fixed protocol more efficiently.

That is why the current library path is not a detour. It is the reference backend for the semantic layer the long-term system will rely on.

---

## 18. Correctness story

A useful way to state correctness is to give each stage a semantic role.

- `zk.proto.core` denotes an incomplete protocol skeleton with pending obligations and unresolved objects.
- `zk.proto.<class>.closed` denotes a fixed protocol meaning in terms of proof surface, transcript trace, and verifier relation.
- `zk.proto.<class>.opt` denotes the same closed protocol meaning after semantics-preserving protocol rewrites.
- library or kernel backends denote execution strategies for realizing that same fixed meaning.

This suggests three main correctness obligations.

### 18.1 Closure correctness

Closure should preserve intended protocol meaning while making it complete. In practice this means that the closed verifier relation matches the intended protocol, transcript replay matches the intended challenge schedule, and the proof surface is sufficiently fixed for execution lowering.

### 18.2 Optimization correctness

Protocol rewrites preserve the semantic triple of the closed protocol. In later work, some rewrites may be allowed to refine or version the ABI explicitly, but the default regime is semantics preservation.

### 18.3 Backend preservation

Library and kernel backends preserve the same closed protocol meaning. They differ in execution strategy, not in cryptographic content.

The point of Protocol IR is not that the first paper will fully mechanize all of these claims. The point is that the stage boundaries are designed so that these claims can be stated cleanly.

---

## 19. Scope of the current paper

The present paper should be framed as:

> **Verified Protocol Instantiation Before Kernelization**

This framing is strong because it is both narrower and more defensible than “a complete prover compiler” or “a universal ZK optimizer.”

### 19.1 In scope

The current paper should claim:

- Protocol IR as the semantic lock-in layer between arithmetic and execution,
- the `core -> closed -> opt` stage structure,
- explicit verifier structure,
- proof ABI as a logical contract,
- transcript / entropy / security / shape contracts,
- obligation-carrying closure,
- protocol-level optimization as distinct from backend optimization,
- one executable path through a library backend.

### 19.2 Deliberately out of scope

The current paper should not center:

- exact MLIR object definitions,
- full native kernel execution compilation,
- broad search/autotuning over rewrite spaces,
- a complete `zk.sem` layer above arithmetic IR,
- maximal multi-family unification.

Those are important directions, but they belong to later documents and later papers.

---

## 20. Evaluation focus

The first paper should not be judged primarily on prover throughput. The evaluation focus should be:

### 20.1 Safety

Can the compiler reject:

- absorb-after-challenge errors,
- missing statement or context binding,
- missing domain separation,
- unsupported Fiat–Shamir profiles,
- missing hiding,
- missing verifier fragments,
- illegal query/decommitment plans,
- witness-dependent proof shape?

### 20.2 Semantic completeness

Does closure make the right things explicit?

- proof surface,
- transcript trace,
- verifier relation,
- protocol class/profile,
- obligations,
- ABI state,
- closure conditions.

### 20.3 Optimization leverage

Can Protocol IR reduce or regularize:

- commitment count,
- opening or decommitment count,
- transcript footprint,
- proof-surface complexity,
- downstream execution complexity?

### 20.4 Executability

Can one closed protocol drive a real proof/verify roundtrip through the current backend?

### 20.5 Validation discipline

Because recent security and testing work shows that real SNARK/ZK stacks contain numerous implementation vulnerabilities and compiler bugs, evaluation should include negative tests and compiler-validation methodology rather than only positive benchmarks ([SNARK Security SoK], [MTZK]).

---

## 21. What is genuinely novel here

The strongest contribution set is the following.

### 21.1 Protocol IR as a first-class compiler boundary

This work centers a layer that sits after arithmetic structure and before execution structure, and treats that layer as the semantic lock-in point.

### 21.2 Verified protocol instantiation before kernelization

Protocol closure is elevated to a first-class compiler phase with explicit legality conditions.

### 21.3 Typed, effectful, obligation-carrying protocol construction

Transcript, entropy, proof surface, and obligations are modeled as first-class semantic resources rather than as scattered helper logic.

### 21.4 Proof ABI and verifier explicitness

Proof layout and verifier semantics are treated as semantic protocol objects.

### 21.5 Profile-carrying Fiat–Shamir closure

Fiat–Shamir is represented as protocol compilation with transcript well-formedness and admissibility obligations.

### 21.6 Family-explicit, feature-layered design

The common core stays small, while richer family structure remains explicit rather than being hidden in a monolithic “generic” protocol language.

Taken together, these claims distinguish Protocol IR from arithmetic IR proposals, backend-facing proof IRs, and execution-only compiler layers.

---

## 22. Final position

The cleanest one-sentence summary of this document is:

> **Protocol IR is the compiler layer that turns an unresolved protocol skeleton into a closed, verifier-explicit, proof-ABI-aware, cryptographically complete protocol before any execution lowering begins.**

That statement captures the project’s present scientific center.

The current implementation path lowers that protocol to a library backend. The future system will lower the same closed semantics to kernels, plans, and runtime replay. The architectural rule does not change:

> **Protocol meaning is fixed before execution strategy is chosen.**

That is the contribution this document exists to define.

---

## Selected references

### Compiler and IR architecture

- [MLIR] Chris Lattner et al. *MLIR: Scaling Compiler Infrastructure for Domain-Specific Computation*. CGO 2021.
- [Dialect Conversion] LLVM MLIR documentation. *Dialect Conversion*.
- [Transform] Martin Paul Lücke et al. *The MLIR Transform Dialect: Your compiler is more powerful than you think*. 2024.
- [Verification Dialects] Mathieu Fehr et al. *First-Class Verification Dialects for MLIR*. PLDI 2025.
- [HEIR] Asra Ali et al. *HEIR: A Universal Compiler for Homomorphic Encryption*. 2025.

### ZK compilers and IRs

- [CirC] Alex Ozdemir, Fraser Brown, Riad S. Wahby. *CirC: Compiler Infrastructure for Proof Systems, Software Verification, and More*. IACR ePrint 2020/1586.
- [Noir ACIR] Noir documentation. *ACIR and backend-facing compilation*.
- [zkInterface] Daniel Benarroch et al. *zkInterface: A Standard Tool for Zero-Knowledge Interoperability*. ZKProof proposal.
- [SIEVE] SIEVE IR specification and tooling.
- [zkLLVM] Nil Foundation. *zkLLVM Circuit Compiler*.

### Fiat–Shamir, transcript engineering, and security

- [Attema–Fehr–Klooß] Thomas Attema, Serge Fehr, Michael Klooß. *Fiat–Shamir Transformation of Multi-Round Interactive Proofs*. IACR ePrint 2021/1377.
- [FRI FS] Austin R. Block et al. *Fiat–Shamir Security of FRI and Related SNARKs*. IACR ePrint 2023/1071.
- [Weak FS Attacks] *Weak Fiat–Shamir Attacks on Modern Proof Systems*. IACR ePrint 2023/691.
- [FS in the Wild] *Fiat–Shamir in the Wild*. IACR ePrint 2024/1565.
- [Merlin] Merlin transcripts documentation and repository.
- [Decree] Trail of Bits. *Decree Fiat–Shamir transcript specification library*.
- [Duplex Sponge FS] Alessandro Chiesa and Michele Orrù. *A Fiat–Shamir Transformation From Duplex Sponges*.
- [IRTF FS Draft] Michele Orrù. *Fiat-Shamir Transformation* (IRTF CFRG Internet-Draft).
- [spongefish] arkworks-rs. *spongefish: a duplex sponge Fiat–Shamir library*.
- [ArkLib] Verified zkEVM / Nethermind. *ArkLib: Formally Verified Arguments of Knowledge in Lean*.

### STARK / AIR / oracle-protocol systems

- [Winterfell] Winterfell documentation. *AIR trait and randomized multi-stage traces*.
- [AirScript] AirScript repository.
- [RISC Zero] RISC Zero developer documentation. *STARK protocol sequence diagram*.
- [Cairo] Cairo book and prover pipeline references.

### Validation and security engineering

- [SNARK Security SoK] Stefanos Chaliasos et al. *SoK: What Don’t We Know? Understanding Security Risks of SNARKs*. USENIX Security 2024.
- [MTZK] Dongwei Xiao et al. *MTZK: Testing and Exploring Bugs in Zero-Knowledge (ZK) Compilers*. NDSS 2025.

---

## Reference links

[MLIR]: https://dl.acm.org/doi/10.1109/CGO51591.2021.9370308
[Dialect Conversion]: https://mlir.llvm.org/docs/DialectConversion/
[Transform]: https://arxiv.org/abs/2409.03864
[Verification Dialects]: https://users.cs.utah.edu/~regehr/papers/pldi25.pdf
[HEIR]: https://arxiv.org/abs/2508.11095

[CirC]: https://eprint.iacr.org/2020/1586
[Noir ACIR]: https://noir-lang.org/docs/
[zkInterface]: https://docs.zkproof.org/pages/standards/accepted-workshop3/proposal-zkinterface.pdf
[SIEVE]: https://raw.githubusercontent.com/sieve-zk/ir/main/v1.0.1/sieve-ir-v1.0.1.pdf
[zkLLVM]: https://github.com/NilFoundation/zkLLVM

[Attema–Fehr–Klooß]: https://eprint.iacr.org/2021/1377
[FRI FS]: https://eprint.iacr.org/2023/1071.pdf
[Weak FS Attacks]: https://eprint.iacr.org/2023/691
[FS in the Wild]: https://eprint.iacr.org/2024/1565
[Merlin]: https://github.com/dalek-cryptography/merlin
[Decree]: https://github.com/trailofbits/decree
[Duplex Sponge FS]: https://eprint.iacr.org/2025/536
[IRTF FS Draft]: https://datatracker.ietf.org/doc/draft-irtf-cfrg-fiat-shamir/
[spongefish]: https://github.com/arkworks-rs/spongefish
[ArkLib]: https://verified-zkevm.github.io/ArkLib/blueprint/index.html

[Winterfell]: https://docs.rs/winterfell/latest/winterfell/trait.Air.html
[AirScript]: https://github.com/0xMiden/air-script
[RISC Zero]: https://dev.risczero.com/proof-system/proof-system-sequence-diagram
[Cairo]: https://www.starknet.io/cairo-book/ch200-introduction.html

[SNARK Security SoK]: https://www.usenix.org/system/files/usenixsecurity24-chaliasos.pdf
[MTZK]: https://www.ndss-symposium.org/wp-content/uploads/2025-530-paper.pdf
