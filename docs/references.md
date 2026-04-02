# Reference Stack

## Recommended Bibliography Structure

The material is best organized into six main blocks:

1. Compiler and IR architecture foundations
2. Directly relevant ZK compilers and IRs
3. Fiat–Shamir theory, transcript engineering, standardization, and formalization
4. STARK / AIR / oracle-protocol systems
5. Typed / effectful / proof-carrying PL foundations
6. Security, verification, and testing of ZK stacks

---

## 1. Priority Reading List

The following references form the most efficient initial reading set.

- **MLIR: Scaling Compiler Infrastructure for Domain-Specific Computation** — canonical reference for dialect-based, multi-level compiler architecture.  
  [Link](https://dl.acm.org/doi/10.1109/CGO51591.2021.9370308)

- **The MLIR Transform Dialect** — strong reference for separating transformation control from payload semantics; especially relevant for a future `transform.zk` control plane.  
  [Link](https://arxiv.org/abs/2409.03864)

- **HEIR** — an important adjacent example of an MLIR-native cryptography compiler with layered dialects and reusable passes, even though the application domain is FHE rather than ZK.  
  [Link](https://arxiv.org/abs/2508.11095)

- **IRDL: an IR Definition Language for SSA Compilers** — useful for declarative dialect definitions and machine-checkable structural constraints.  
  [Link](https://grosser.science/static/0c315060e8f3d8454de831910fbb6dd6/fehr-2022-irdl.pdf)

- **First-Class Verification Dialects for MLIR** — relevant for verifier IR, proof obligations, and semantics-carrying dialect design.  
  [Link](https://users.cs.utah.edu/~regehr/papers/pldi25.pdf)

- **CirC** — one of the clearest “compiler for proofs” references, rather than merely a front-end language or circuit DSL.  
  [Link](https://eprint.iacr.org/2020/1586)

- **Noir / ACIR** — an important deployed example of a backend-facing proof IR between a language frontend and multiple proving backends.  
  [Link](https://noir-lang.org/docs/)

- **zkInterface / SIEVE IR** — a key interoperability reference for backend-agnostic ZK IR design.  
  [Link](https://docs.zkproof.org/pages/standards/accepted-workshop2/proposal--zk-interop-zkinterface.pdf)

- **Weak Fiat-Shamir Attacks on Modern Proof Systems** — one of the strongest motivation papers for making transcript discipline compiler-visible and statically enforced.  
  [Link](https://eprint.iacr.org/2023/691)

- **Fiat-Shamir in the Wild** — practical evidence that transcript construction errors are an implementation problem, not just a theorem-level concern.  
  [Link](https://eprint.iacr.org/2024/1565)

- **Fiat-Shamir Security of FRI and Related SNARKs** — essential reading for any STARK/FRI-oriented track.  
  [Link](https://eprint.iacr.org/2023/1071.pdf)

- **A Fiat-Shamir Transformation From Duplex Sponges** — a useful bridge between theoretical Fiat–Shamir analyses and the duplex-sponge constructions that better match modern practice.  
  [Link](https://eprint.iacr.org/2025/536)

- **Merlin** — a strong implementation reference for transcripts, framing, domain separation, typed challenge extraction, and transcript-driven RNG.  
  [Link](https://github.com/dalek-cryptography/merlin)

- **Decree** — stage-wise transcript specification and misuse-resistant transcript engineering.  
  [Link](https://github.com/trailofbits/decree)

- **Winterfell** and **AirScript** — strong references for AIR/STARK-oriented structure and engineering.  
  [Winterfell](https://docs.rs/winterfell/latest/winterfell/trait.Air.html) · [AirScript](https://github.com/0xMiden/air-script)

- **RISC Zero proof-system documentation** — a production reference for a modern STARK-oriented system centered around DEEP-ALI and FRI.  
  [Link](https://dev.risczero.com/proof-system/proof-system-sequence-diagram)

---

## 2. Compiler and IR Architecture Foundations

### Core MLIR references

- **MLIR: Scaling Compiler Infrastructure for Domain-Specific Computation** — multi-level IR, dialect modularity, staged lowering.  
  [Link](https://dl.acm.org/doi/10.1109/CGO51591.2021.9370308)

- **MLIR Dialect Conversion** — legality-driven staged lowering; highly relevant to `core -> closed -> opt` style transitions.  
  [Link](https://mlir.llvm.org/docs/DialectConversion/)

- **MLIR Interfaces** — generic extension points for verifier emitters, transcript contributors, and backend-lowering hooks.  
  [Link](https://mlir.llvm.org/docs/Interfaces/)

- **MLIR Symbols and Symbol Tables** — useful for proof slots, named protocol objects, and cross-region references.  
  [Link](https://mlir.llvm.org/docs/SymbolsAndSymbolTables/)

- **MLIR Side Effects and Speculation** — relevant when transcript, entropy, proof surface, or obligations are modeled as explicit resources.  
  [Link](https://mlir.llvm.org/docs/Rationale/SideEffectsAndSpeculation/)

- **MLIR Attributes and Types** — foundational reference for parametric types plus semantic attributes.  
  [Link](https://mlir.llvm.org/docs/DefiningDialects/AttributesAndTypes/)

- **The MLIR Transform Dialect** — transformation control as IR, distinct from payload semantics.  
  [Link](https://arxiv.org/abs/2409.03864)

- **IRDL: an IR Definition Language for SSA Compilers** — declarative constraints over dialect families and transformation pre/postconditions.  
  [Link](https://grosser.science/static/0c315060e8f3d8454de831910fbb6dd6/fehr-2022-irdl.pdf)

- **First-Class Verification Dialects for MLIR** — verifier IR and semantics-aware dialect layering.  
  [Link](https://users.cs.utah.edu/~regehr/papers/pldi25.pdf)

- **HEIR** — evidence that multi-level MLIR architectures are effective for cryptographic compilers beyond traditional program compilation.  
  [Link](https://arxiv.org/abs/2508.11095)

### Scheduling, transformation control, and search

- **Halide** — classic separation of algorithm and schedule.  
  [Link](https://people.csail.mit.edu/jrk/halide-pldi13.pdf)

- **TVM** — build/tune/run split and schedule/search infrastructure.  
  [Link](https://www.usenix.org/system/files/osdi18-chen.pdf)

- **ELEVATE** — rewrite-strategy foundations for schedule/control-language design.  
  [Link](https://arxiv.org/abs/2002.02268)

- **Exo** — programmable scheduling and effect-aware optimization on low-level kernels.  
  [Link](https://dl.acm.org/doi/10.1145/3519939.3523446)

---

## 3. Directly Relevant ZK Compilers and IRs

### Compiler and IR systems

- **CirC** — arithmetic-circuit and proof compilation infrastructure; one of the best direct precedents for “compilerized proving.”  
  [Link](https://eprint.iacr.org/2020/1586)

- **zkLLVM** — LLVM/MLIR-style compilation into proving systems; useful as a systems-oriented baseline.  
  [Link](https://github.com/NilFoundation/zkLLVM)

- **Noir / ACIR** — backend-facing proof IR with real deployment significance.  
  [Link](https://noir-lang.org/docs/)

- **zkInterface / SIEVE IR** — interoperability-oriented IR, useful for contrast with protocol-structural IR design.  
  [Link](https://docs.zkproof.org/pages/standards/accepted-workshop2/proposal--zk-interop-zkinterface.pdf)

- **LLZK** — a recent MLIR-based ZK IR effort worth citing as neighboring work.  
  [Link](https://veridise.com/blog/zero-knowledge/announcing-llzk-a-unified-open-source-intermediate-representation-ir-for-zero-knowledge-languages/)

- **Vamp-IR** — proof-system-agnostic arithmetic/compiler IR.  
  [Link](https://github.com/anoma/vamp-ir)

- **Leo** — statically typed ZK language ecosystem.  
  [Link](https://eprint.iacr.org/2021/651.pdf)

- **ZoKrates** — mainstream compiler/toolchain reference.  
  [Link](https://zokrates.github.io/print.html)

- **ZK-SecreC** — typed-language reference with confidentiality/integrity semantics and secure-computation flavor.  
  [Link](https://arxiv.org/abs/2203.15448)

- **Coda** — refinement-type-based static checking for ZK application compilation.  
  [Link](https://eprint.iacr.org/2023/547)

- **Ou / Lian** — compiler-style decomposition and parallelization of ZK statements.  
  [Link](https://eprint.iacr.org/2023/657)

- **ZØ** — older but still historically relevant as a “proof compilation” reference.  
  [Link](https://www.usenix.org/system/files/conference/usenixsecurity14/sec14-paper-fredrikson-z0.pdf)

### Surveys and SoKs

- **Zero-Knowledge Proof Frameworks: A Survey** — broad survey of frameworks and tooling.  
  [Link](https://arxiv.org/pdf/2502.07063v2)

- **SoK: Understanding zk-SNARKs — The Gap Between Research and Practice** — strong framing reference for the systems gap.  
  [Link](https://www.usenix.org/system/files/usenixsecurity25-liang-sok.pdf)

- **SoK: What Don’t We Know? Understanding Security Risks of SNARKs** — strong motivation for explicit protocol/security contracts.  
  [Link](https://www.usenix.org/system/files/usenixsecurity24-chaliasos.pdf)

---

## 4. Fiat–Shamir Theory, Transcript Engineering, Standardization, and Formalization

### Core theory

- **The Security of the Fiat–Shamir Transformation in the QROM** — a key modern theoretical baseline.  
  [Link](https://www.h2020prometheus.eu/sites/default/files/2021-05/2019-190.pdf)

- **Fiat-Shamir Transformation of Multi-Round Interactive Proofs** — important for showing that multi-round FS is not the trivial repetition of the 3-move case.  
  [Link](https://eprint.iacr.org/2021/1377.pdf)

- **Straight-Line Knowledge Extraction for Multi-Round Fiat-Shamir Protocols** — useful for stronger extraction profiles and admissibility conditions.  
  [Link](https://eprint.iacr.org/2024/1724)

- **Fiat-Shamir Security of FRI and Related SNARKs** — required reading for FRI/STARK systems.  
  [Link](https://eprint.iacr.org/2023/1071.pdf)

### Engineering failures and transcript pitfalls

- **Weak Fiat-Shamir Attacks on Modern Proof Systems** — likely the single most useful motivation reference for making transcript structure explicit in the IR.  
  [Link](https://eprint.iacr.org/2023/691)

- **Fiat-Shamir in the Wild** — strong practical evidence of real-world transcript bugs.  
  [Link](https://eprint.iacr.org/2024/1565)

- **How to Prove False Statements** — modern adversarial reference showing why “hash the transcript” is not enough as a design principle.  
  [Link](https://eprint.iacr.org/2025/118)

- **How Not to Prove Yourself** — older but still useful cautionary protocol-design reference.  
  [Link](https://dl.acm.org/doi/10.1007/978-3-642-34961-4_38)

### Implementations and transcript APIs

- **Merlin** — transcript design patterns: stage labels, framing, domain separation, typed extraction, transcript-based RNG.  
  [Link](https://github.com/dalek-cryptography/merlin)

- **Decree** — stage-wise transcript specification with enforcement of transcript discipline.  
  [Link](https://github.com/trailofbits/decree)

- **spongefish** — a duplex-sponge Fiat–Shamir implementation reference with a generic API for multi-round public-coin protocols and explicit codec support.  
  [Link](https://github.com/arkworks-rs/spongefish)

### Duplex-sponge, standardization, and mechanization

- **A Fiat-Shamir Transformation From Duplex Sponges** — directly relevant for connecting Fiat–Shamir design to duplex-sponge constructions, fixed-size permutation analyses, concrete security bounds, and the `spongefish` implementation.  
  [Link](https://eprint.iacr.org/2025/536)

- **IRTF CFRG draft: Fiat-Shamir Transformation** — useful when discussing transcript initialization, codec structure, and a more specification-oriented vocabulary for duplex-sponge Fiat–Shamir.  
  [Link](https://datatracker.ietf.org/doc/draft-irtf-cfrg-fiat-shamir/)

- **ArkLib** — useful as an ongoing Lean-oriented mechanization line that explicitly plans to formalize the duplex-sponge version of Fiat–Shamir alongside broader BCS/IOR development.  
  [Link](https://github.com/Verified-zkEVM/ArkLib)

- **Ethereum Foundation Q4 2025 allocation update** — best treated as an ecosystem signal rather than a technical authority; useful for pointing to the current interest in Fiat–Shamir specification, Lean formalization, duplex-sponge review work, ArkLib, and LLZK in one place.  
  [Link](https://blog.ethereum.org/2026/01/27/allocation-q4-25)

### Formalization and mechanization

- **Fixing and Mechanizing the Security Proof of Fiat-Shamir with Aborts and Dilithium** — a strong formalization reference.  
  [Link](https://eprint.iacr.org/2023/246)

- **A certifying compiler based on formalized Σ-protocols** — older but highly relevant for theorem-backed proof compilation.  
  [Link](https://eprint.iacr.org/2010/339)

---

## 5. STARK / AIR / Oracle-Protocol / zkVM References

### AIR and STARK building blocks

- **Winterfell documentation (`Air`, `ProofOptions`, randomized AIR / multi-stage trace support)** — strong reference for AIR structure, extension fields, and multi-stage/randomized traces.  
  [Air trait](https://docs.rs/winterfell/latest/winterfell/trait.Air.html) · [ProofOptions](https://docs.rs/winterfell/latest/winterfell/struct.ProofOptions.html)

- **AirScript** — dedicated AIR DSL and normalized AIR pipeline.  
  [Link](https://github.com/0xMiden/air-script)

- **RISC Zero proof-system documentation** — production reference for DEEP-ALI + FRI-oriented architecture.  
  [Link](https://dev.risczero.com/proof-system/proof-system-sequence-diagram)

- **Cairo / Stone prover / Cairo Book** — execution-trace-to-prover pipeline and production STARK engineering.  
  [Link](https://www.starknet.io/cairo-book/ch200-introduction.html)

- **Polygon PIL / zkEVM prover documentation** — industrial reference for AIR-like / arithmetization-level specification and prover structure.  
  [Link](https://docs.polygon.technology/tools/zkevm/spec/pil/simple-example/)

### Broader proving and zkVM systems

- **Plonky3** — modern modular proving stack.  
  [Link](https://github.com/Plonky3/Plonky3)

- **SP1** — current zkVM/compiler/proving ecosystem.  
  [Link](https://docs.succinct.xyz/docs/sp1/introduction)

- **Jolt** — modern zkVM / lookup-heavy proving-system design.  
  [Link](https://eprint.iacr.org/2023/1217)

- **halo2** — one of the most important SNARK-side proof-system engineering references.  
  [Link](https://zcash.github.io/halo2/concepts/proofs.html)

- **gnark** — practical engineering ecosystem and backend/API design reference.  
  [Link](https://github.com/Consensys/gnark)

- **arkworks** — core Rust ecosystem reference.  
  [Link](https://arkworks.rs/)

### Suggested STARK baseline set

A strong STARK-oriented baseline set is:

- Winterfell
- AirScript
- RISC Zero
- Cairo / Stone

That cluster provides a good combination of DSL, AIR, protocol structure, and deployed engineering.

---

## 6. Typed / Effectful / Proof-Carrying PL Foundations

These references are not ZK papers, but they provide strong academic foundations for a typed, effectful, obligation-carrying Protocol IR.

- **Proof-Carrying Code** — canonical reference for discharged obligations and proof-carrying artifacts.  
  [Link](https://homes.cs.washington.edu/~mernst/teaching/6.893/readings/necula-popl97.pdf)

- **Typed Assembly Language** — typed low-level representations and safety after lowering.  
  [Link](https://www.cs.cornell.edu/talc/papers/tal-popl.pdf)

- **SSA is Functional Programming** — useful philosophical and structural foundation for effect/resource reasoning over SSA IR.  
  [Link](https://www.cs.princeton.edu/~appel/papers/ssafun.pdf)

- **Plotkin & Pretnar / algebraic effects and handlers** — strong foundation if transcript, entropy, or proof surface are modeled as effectful resources.  
  [Link](https://arxiv.org/abs/1306.6316)

- **A refinement type system for algebraic effects and handlers** — useful for a richer typed/effectful story beyond plain effects.  
  [Link](https://terauchi.w.waseda.jp/papers/popl24-arm.pdf)

- **Abstracting Effect Systems** — useful for structuring multiple orthogonal effects/contracts in a principled way.  
  [Link](https://dl.acm.org/doi/10.1145/3674641)

- **Unifying graded and parameterised monads** — relevant if budgets/capabilities/effects are made principled rather than ad hoc.  
  [Link](https://arxiv.org/abs/2001.10274)

- **Linear Haskell** — useful if transcript tokens, proof slots, or obligations are treated as linear/affine resources.  
  [Link](https://arxiv.org/pdf/1710.09756)

- **Proof-Carrying Data** — relevant if proof ABI, recursive composition, or verifier IR are pushed toward recursion.  
  [Link](https://eprint.iacr.org/2012/095.pdf)

### Compact foundations trio

A concise but strong foundations trio for this research direction is:

- Proof-Carrying Code
- algebraic effects and handlers
- linearity / uniqueness

---

## 7. Security, Verification, and Testing of ZK Stacks

These references motivate explicit verifier IR, explicit obligations, and validation/testing infrastructure.

- **SoK: Understanding zk-SNARKs — The Gap Between Research and Practice** — useful for the high-level systems motivation.  
  [Link](https://www.usenix.org/system/files/usenixsecurity25-liang-sok.pdf)

- **SoK: What Don’t We Know? Understanding Security Risks of SNARKs** — useful for the “unsafe composition / silent assumptions” motivation.  
  [Link](https://www.usenix.org/system/files/usenixsecurity24-chaliasos.pdf)

- **MTZK** — mutation testing for ZK circuits and compilers.  
  [Link](https://www.ndss-symposium.org/ndss-paper/mtzk-testing-and-exploring-bugs-in-zero-knowledge-zk-compilers/)

- **Circuzz** — fuzzing/testing of arithmetic circuits and compilers.  
  [Link](https://aisychev.github.io/papers/ccs25-circuzz.pdf)

- **Arguzz** — argument/proof-system fuzzing.  
  [Link](https://arxiv.org/abs/2509.10819)

- **Automated Soundness and Completeness Vetting of Polygon zkEVM** — direct “verifier/soundness checking in production systems” reference.  
  [Link](https://www.usenix.org/system/files/usenixsecurity25-peng-xinghao.pdf)

- **Towards Formal Verification of the First RISC-V zkVM** — useful formal-methods reference at the IR/VM boundary.  
  [Link](https://www.nethermind.io/blog/towards-formal-verification-of-the-first-risc-v-zkvm)

- **Formal Verification of the zkSync Verifier** — verifier-side formal verification reference.  
  [Link](https://www.nethermind.io/blog/formal-verification-of-the-zksync-verifier)

- **Verified zkEVMs** — broader formal-methods and verified-systems reference around zkEVM stacks.  
  [Link](https://verified-zkevm.org/)

---

## 8. Industry Implementations Worth Tracking Closely

For implementation-side grounding, the following codebases and documentation sets are especially useful:

- **Noir / ACIR** — frontend plus backend-facing proof IR.  
  [Link](https://noir-lang.org/docs/)

- **zkLLVM** — compiler-oriented production path.  
  [Link](https://github.com/NilFoundation/zkLLVM)

- **RISC Zero** — modern zkVM + STARK engineering.  
  [Link](https://dev.risczero.com/proof-system/proof-system-sequence-diagram)

- **SP1** — current zkVM/compiler stack.  
  [Link](https://docs.succinct.xyz/docs/sp1/introduction)

- **Winterfell** — open STARK/AIR engineering reference.  
  [Link](https://docs.rs/winterfell/latest/winterfell/trait.Air.html)

- **AirScript** — AIR authoring and compilation pipeline.  
  [Link](https://github.com/0xMiden/air-script)

- **Cairo / Stone** — production STARK prover pipeline.  
  [Link](https://www.starknet.io/cairo-book/ch200-introduction.html)

- **halo2** — proof-system engineering reference.  
  [Link](https://zcash.github.io/halo2/concepts/proofs.html)

- **gnark** — practical engineering baseline.  
  [Link](https://github.com/Consensys/gnark)

- **arkworks** — core Rust ecosystem.  
  [Link](https://arkworks.rs/)

- **Plonky3** — modular proving toolkit.  
  [Link](https://github.com/Plonky3/Plonky3)

- **Merlin** — transcript engineering reference.  
  [Link](https://github.com/dalek-cryptography/merlin)

- **Decree** — stage-disciplined transcripts.  
  [Link](https://github.com/trailofbits/decree)
