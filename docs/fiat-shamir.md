# Fiat–Shamir in Protocol IR

## Purpose

The intended role of this document is to answer five questions in one place:

1. **What Fiat–Shamir actually is** in the context of modern proof systems.
2. **Why Fiat–Shamir belongs inside Protocol IR**, rather than in a proving library or backend helper layer.
3. **What semantic objects and invariants** Protocol IR must make explicit in order to model Fiat–Shamir correctly.
4. **What compiler methodology** should be used to close a public-coin protocol into a non-interactive proof system.
5. **Why this is a compiler and PL contribution**, not merely an implementation detail.

The central conclusion of this note is the following:

> **Fiat–Shamir should be treated in Protocol IR as a first-class protocol-compilation problem.**
> The correct design is not “hash the transcript so far” and not a library-side helper function. The correct design is to compile a **public-coin protocol skeleton** into a **stage-typed transcript program** with explicit proof-surface bindings, explicit verifier replay, explicit prover-private entropy, and explicit admissibility obligations.

This conclusion fits the current architecture directly: Protocol IR is already the semantic lock-in layer between arithmetic structure and execution structure; protocol instantiation already occurs before kernelization; and a closed protocol is already best understood as the triple **(proof surface, transcript trace, verifier relation)**. Fiat–Shamir belongs exactly at that boundary.

A secondary purpose of this note is to make the design’s external alignment clear without changing its center of gravity. Recent work on duplex-sponge Fiat–Shamir, active specification work around a duplex-sponge-and-codec interface, and ongoing implementation and mechanization efforts provide useful **semantic anchors** for the transcript and encoding layers considered here. They do **not** remove the compiler problem addressed in this document. The point is not to repackage a transcript library or a standalone formalization, but to place those ingredients inside a compiler-level account of protocol closure.

---

## 1. Research problem

### 1.1 Why Fiat–Shamir is not a minor detail

In many proof-system implementations, Fiat–Shamir is treated operationally: the prover computes some bytes, hashes them, reduces the output to a field element or seed, and continues. At the implementation level this looks small. At the semantic level it is not small at all.

Fiat–Shamir determines:

- which public data a challenge is bound to,
- which prior prover messages influence that challenge,
- in what order transcript inputs are framed,
- how challenge classes are separated,
- how verifier replay reconstructs the same challenge schedule,
- and under what theorem profile the interactive-to-non-interactive compilation is even justified.

If any of these are underspecified, the protocol may remain executable while no longer matching the intended proof system. In other words, Fiat–Shamir sits exactly at the point where the *meaning* of the protocol is fixed.

That is why a prover compiler cannot relegate Fiat–Shamir to a low-level helper.

### 1.2 The real difficulty

The hard part of Fiat–Shamir is not the cryptographic sponge call. The hard part is specifying, preserving, and checking the **semantic boundary** between:

- the **interactive public-coin protocol** one would write in a proof paper,
- and the **non-interactive transcript protocol** one actually executes in software.

The research problem addressed here is therefore:

> **How should a prover compiler represent Fiat–Shamir so that interactive protocol structure, transcript correctness, proof ABI, verifier replay, and cryptographic admissibility are all explicit, checkable, and preserved under protocol rewrites?**

This is a Protocol IR problem, not an arithmetic IR problem and not a backend scheduling problem.

---

## 2. Thesis

The thesis of this document is:

> **A prover compiler should compile Fiat–Shamir at the protocol layer, before kernelization, by translating a public-coin protocol skeleton into a verifier-explicit transcript program whose correctness is governed by typed effects, proof-surface contracts, and admissibility obligations.**

This thesis has five parts.

First, **Fiat–Shamir is protocol compilation**, not transcript plumbing.

Second, **Fiat–Shamir must be modeled over semantic protocol objects**, not over ad hoc byte strings.

Third, **Fiat–Shamir correctness is two-layered**:
- transcript well-formedness, and
- theorem-backed admissibility.

Fourth, **closed protocol semantics** should remain the triple:

\[
\llbracket P \rrbracket = (\text{proof surface},\ \text{transcript trace},\ \text{verifier relation})
\]

Fifth, **protocol optimization remains possible after closure**, but only if rewrites preserve that semantic triple.

This framing gives Fiat–Shamir a precise place inside the compiler architecture and turns a traditionally fragile implementation discipline into a static compiler contract.

A related but deliberately secondary claim is that this framing is compatible with emerging external work on duplex-sponge Fiat–Shamir. In particular, recent analysis and specification efforts sharpen the transcript and encoding story around a duplex sponge interface, an explicit codec layer, and explicit initialization context. Protocol IR should be able to align with that structure without surrendering its own responsibilities: proof-surface binding, stage-typed construction, verifier-explicit replay, and closure obligations.

---

## 3. Fiat–Shamir as a formal object

### 3.1 The classical picture

At the classical level, Fiat–Shamir starts from a public-coin interactive protocol. Very roughly, an interactive public-coin protocol can be presented as a sequence of prover messages and verifier randomness:

\[
P_1,\ r_1,\ P_2,\ r_2,\ \ldots,\ P_t
\]

where each verifier message is public randomness and each later prover message may depend on earlier transcript state. Fiat–Shamir replaces each verifier coin flip \(r_i\) with a deterministic random-oracle output over the protocol prefix.

For three-move Sigma protocols, this picture is relatively clean. For many modern proof systems, it is not.

### 3.2 Why modern proof systems are more subtle

Modern SNARK and STARK families are usually multi-round, often oracle-based, and often involve transcript objects that are richer than simple prover-message strings. Among the complications that matter are:

- multiple challenge rounds with different semantic roles,
- challenge extraction in different domains,
- interaction with proof-surface layout,
- oracle commitments and query/decommitment bundles,
- and theorem profiles that are family-specific rather than universal.

As a result, Fiat–Shamir in modern systems is not well modeled as a single rewrite rule of the form:

> replace all verifier randomness with `H(prefix)`.

Instead, it should be understood as a **family of admissible compilation schemas**, each with its own side conditions.

### 3.3 The three layers of Fiat–Shamir

For compiler design, the most useful decomposition is to view Fiat–Shamir as having three layers.

#### A. Theory layer
This layer answers:

- What theorem or reduction justifies compilation for this protocol class?
- In what model is the result interpreted?
- What structural side conditions are required?

Examples include:
- Sigma-style profiles,
- special-soundness multi-round profiles,
- round-by-round IOP/BCS profiles,
- and unsupported families that should require manual expert certification.

#### B. Transcript layer
This layer answers:

- What semantic objects are bound to each challenge?
- In what stage and in what canonical order?
- Under which namespace or domain separator?
- Which challenge class is being sampled?

#### C. Encoding layer
This layer answers:

- How are semantic protocol objects framed and encoded into bytes?
- How does the verifier replay the same encoding protocol?
- What is fixed by the protocol definition rather than supplied dynamically?

A correct Protocol IR design must represent all three, but at different abstraction levels.

### 3.4 External semantic anchors

Recent work on duplex-sponge Fiat–Shamir provides useful external anchors for the last two layers.

At the theory-and-construction level, recent duplex-sponge analyses narrow the gap between idealized theoretical Fiat–Shamir models and the fixed-size permutation/hash objects that real systems actually use. At the interface level, current specification work describes Fiat–Shamir around a **duplex sponge interface**, a **codec interface**, and an explicit initialization context containing protocol and instance identifiers. At the implementation level, `spongefish` provides a practitioner-oriented Rust reference for multi-round public-coin transcripts, and ongoing Lean-oriented work such as ArkLib suggests an increasingly formal ecosystem around these abstractions.

These developments are important, but they should be read in the right way. They help clarify what the **transcript** and **encoding** layers ought to look like. They do not by themselves specify how a compiler should couple transcript state to proof slots, how verifier replay should be represented in IR, or how admissibility and shape obligations should be tracked during protocol closure. Those remain the subject of this document.

---

## 4. Position in the prover-compiler architecture

The current architecture already adopts the right top-level separation:

- arithmetic IR describes **what is being proved**,
- Protocol IR describes **what prover and verifier do as a protocol**,
- execution IR describes **how the resulting computation is run efficiently**.

The architecture also already adopts the key methodological rule:

> **Protocol instantiation must happen before kernelization.**

Fiat–Shamir is one of the strongest reasons this rule is correct. If transcript binding, challenge classes, proof-surface layout, and verifier replay are still fluid, then lowering to kernels is targeting an object whose protocol semantics are not yet fixed.

The correct place for Fiat–Shamir is therefore the Protocol IR boundary:

```text
zk.scheme
  -> zk.proto.core
  -> zk.proto.<class>.closed
  -> zk.proto.<class>.opt
  -> { LibraryBackend , KernelBackend }
```

Conceptually, however, `zk.proto.core` should itself be understood as containing an internal Fiat–Shamir compilation step:

```text
public-coin skeleton
  -> fs.lower
  -> transcriptized protocol skeleton
```

This does not need to surface as a public stage in the first paper. But it should be explicit in the design.

---

## 5. Semantic model

A closed protocol should not be modeled merely as a sequence of prover actions. The correct semantic unit is:

\[
\llbracket P \rrbracket = (\text{proof surface},\ \text{transcript trace},\ \text{verifier relation})
\]

This triple is not incidental. It is exactly the semantic center that Fiat–Shamir interacts with.

### 5.1 Proof surface
The proof surface is the external proof-visible contract:

- what proof objects exist,
- how they are grouped,
- in what order they appear,
- and how the verifier reads them.

This is the logical core of the proof ABI.

### 5.2 Transcript trace
The transcript trace records:

- which semantic objects are absorbed,
- in what order and under what namespace,
- when challenges are sampled,
- and what prefix each challenge depends on.

### 5.3 Verifier relation
The verifier relation records:

- algebraic checks,
- opening or decommitment checks,
- Merkle or FRI checks where relevant,
- and any protocol-specific consistency conditions.

Fiat–Shamir is not an isolated concern inside this triple.

- It determines how proof-surface objects enter the transcript.
- It determines how the verifier reconstructs transcript state.
- It constrains which rewrites preserve protocol meaning.

This is why Fiat–Shamir belongs in Protocol IR and not below it.

---

## 6. Design principles

The following design principles should remain fixed.

### 6.1 Fiat–Shamir must be compiled, not called

The compiler should not expose Fiat–Shamir as an opaque helper such as:

```text
challenge = hash(transcript_bytes)
```

That representation loses:

- semantic subject identity,
- proof-surface coupling,
- challenge classes,
- verifier replay structure,
- admissibility information,
- and rewrite legality.

Instead, the compiler should expose Fiat–Shamir as a protocol transformation.

### 6.2 Transcript semantics must be stage-typed

A transcript should not merely be a linear sequence of absorb/challenge operations. It should be a **stage-typed transcript program**.

Each stage should define:

- which subject classes are required,
- what canonical ordering is used,
- what namespace/domain separator applies,
- and which challenge classes may be sampled there.

This is stronger than dominance checking alone and is much closer to how actual proof protocols are specified.

### 6.3 Proof-visible objects should enter the transcript via slots

For proof-visible objects, the transcript should primarily absorb **proof slots**, not raw internal values.

That means the dominant operation is not merely:

```text
absorb_val(transcript, value)
```

but:

```text
bind(slot, value)
absorb_slot(transcript, slot)
```

This unifies:

- proof serialization order,
- transcript order,
- verifier readback,
- proof ABI,
- and challenge provenance.

This is one of the most important design decisions in the entire object model.

### 6.4 Verifier replay must be explicit

A protocol is not closed unless the verifier can replay the same transcript discipline explicitly. This makes `zk.verify` a first-class part of the design rather than an optional refinement.

### 6.5 Fiat–Shamir admissibility must be tracked explicitly

Transcript well-formedness is necessary, but not sufficient. Closure should also require an explicit theorem profile or admissibility certificate.

### 6.6 Prover entropy must be separate from public transcript randomness

Prover-private masking/blinding randomness should not be conflated with public transcript-derived challenge material. These are different semantic resources and should remain separate in the IR.

This separation is worth preserving even when implementation libraries choose to package challenge derivation and prover-side randomness under one practical transcript API. Protocol IR should be stricter than that surface when semantic identity matters.

### 6.7 External specifications should inform the transcript layer, not define the whole compiler object model

Emerging duplex-sponge specifications, codec interfaces, and mechanized developments should influence how the transcript layer is shaped. In particular, they make it increasingly natural to represent initialization context, codec choice, and challenge extraction policy explicitly rather than as ambient conventions.

But that influence should remain proportionate. Protocol IR does not need to become a thin wrapper around any one transcript library or draft specification. It should instead absorb the parts of that work that sharpen transcript and encoding discipline, and then integrate them into a compiler-native treatment of proof slots, replay, obligations, and protocol optimization.

---

## 7. What Protocol IR must make explicit

A Fiat–Shamir-aware Protocol IR must make at least the following objects explicit.

### 7.1 Protocol identity and public statement

Challenges should be bindable to:

- protocol class/profile,
- protocol version,
- statement or instance digest,
- verifier-owned public parameters,
- protocol metadata that affects replay,
- and, where useful, finer-grained initialization context such as session or instance labels.

These are not optional implementation details.

### 7.2 Transcript subjects

Transcript inputs should be semantic objects with known subject classes, such as:

- `ProtocolDef`
- `Statement`
- `VerifierParam`
- `Commitment`
- `EvaluationBundle`
- `OracleRoot`
- `DecommitmentBundle`
- `Metadata`
- `ProofSlot`

### 7.3 Challenge classes

Challenges should not be modeled as generic field elements. They should be typed protocol objects with challenge classes such as:

- algebraic randomizers,
- query seeds,
- evaluation points,
- OODS/DEEP points,
- FRI fold coefficients,
- batching weights,
- transcript-derived entropy seeds.

### 7.4 Canonical encoding and framing

The IR should know:

- which codec is used,
- how framing is done,
- what label or namespace applies,
- whether validation is required before absorption,
- and what initialization policy is used for protocol and instance binding.

### 7.5 Proof shape

Because proof-visible objects enter the transcript and are replayed by the verifier, proof topology and ABI are part of Fiat–Shamir semantics. Shape-stability is therefore part of the closure discipline.

### 7.6 Admissibility profile

The protocol must record which theorem schema or reduction profile justifies the Fiat–Shamir transformation.

---

## 8. Certificate-carrying Fiat–Shamir compilation

The strongest research methodology available here is what may be called **certificate-carrying Fiat–Shamir compilation**.

The idea is simple.

Moving from `zk.proto.core` to `zk.proto.<class>.closed` should require discharging two different obligation families.

### 8.1 Transcript well-formedness obligations

These obligations cover the engineering and semantic side:

- required public inputs were absorbed,
- stage requirements were satisfied,
- labels and namespaces are valid,
- challenge prefixes are correct,
- proof-visible objects were bound before absorption,
- verifier replay reconstructs the same transcript,
- proof-shape invariants are preserved,
- codec and initialization policies were respected where the protocol declares them.

### 8.2 Fiat–Shamir admissibility obligations

These obligations cover the cryptographic side:

- the underlying public-coin protocol belongs to a theorem-backed family,
- the declared model is compatible with that family,
- the verifier-input model is compatible with the theorem profile,
- the challenge schedule satisfies the theorem’s side conditions,
- the requested security target is compatible with the reduction loss,
- and any declared Fiat–Shamir construction profile is compatible with the chosen sponge/codec discipline.

The compiler does **not** prove new cryptographic theorems. Instead, it checks that the protocol carries the right certificate of belonging to an admissible theorem family.

That is a much more honest and much more defensible research stance than pretending Fiat–Shamir is a generic hash substitution.

---

## 9. Object model

### 9.1 The four resources

Protocol IR should be organized around four first-class resources.

#### A. Transcript
Public-coin state.

#### B. Entropy
Prover-private randomness.

#### C. Proof surface
Proof slots, slot ordering, verifier reads, ABI state.

#### D. Obligations
Delayed cryptographic conditions.

This is the right resource decomposition because it cleanly separates:

- public challenge derivation,
- private masking,
- proof-visible structure,
- and closure conditions.

### 9.2 Top-level operation

A good top-level structure is:

```mlir
zk.proto.protocol @name
  attributes {
    class = #zk.proto.class<stark | plonkish | ...>,
    stage = #zk.proto.stage<core | closed | opt>,
    fs_profile = #zk.fs.profile<...>,
    security = #zk.proto.security<...>,
    abi = #zk.proto.abi<version = 1, state = "draft" | "frozen">
  } {
^slots:
  ...
^prover(%pub: !zk.pub_abi<@pub_v1>):
  ...
^verify(%pub: !zk.pub_abi<@pub_v1>):
  ...
}
```

This directly mirrors the semantic triple.

#### `slots`
Declares proof-visible slots and their ABI roles.

#### `prover`
Constructs the protocol, binds slots, and runs transcript logic.

#### `verify`
Replays the transcript and states the verifier relation explicitly.

### 9.3 Recommended core types

```mlir
!zk.proto.transcript<model = rom, hash = poseidon2>
!zk.proto.entropy<policy = bundle_rng>
!zk.proto.challenge<class = "beta", field = "bn254_fr">
!zk.proto.slotref<kind = "commitment_bundle">
!zk.proto.obligation<kind = "fs_admissible">
!zk.proto.namespace<path = ["fs", "round1"]>
```

The type system should be strong where semantic identity matters, and lighter elsewhere.

### 9.4 Recommended core operations

#### Proof surface

```mlir
zk.proto.slot @wire_commitments : !zk.proto.slotref<kind = "commitment_bundle">
zk.proto.bind @wire_commitments, %wire_comms
```

#### Transcript

```mlir
%tr0 = zk.proto.tr.init {
  protocol = "plonk/v1",
  codec = "canonical",
  instance_label = %stmt_root
}
%tr1 = zk.proto.tr.absorb_meta %tr0, %stmt_root { label = "statement" }
%tr2 = zk.proto.tr.absorb_slot %tr1, @wire_commitments { label = "wire_comms" }
%tr3, %beta = zk.proto.tr.challenge %tr2 { class = "beta", field = "bn254_fr" }
```

A richer implementation may also make initialization context more explicit, for example through separate protocol identifiers, session information, or codec symbols. The important design point is not the exact surface spelling; it is that such context should be representable at the protocol layer rather than hidden in backend code.

#### Entropy

```mlir
%rng0 = zk.proto.ent.init { policy = "bundle_rng" }
%rng1, %rho = zk.proto.ent.take %rng0 { class = "blinder", bits = 128 }
%masked = zk.proto.mask %poly with %rho
```

#### Obligations

```mlir
%o0 = zk.proto.require "must_influence"(@wire_commitments, %beta)
%o1 = zk.proto.require "canonical_encoding"(@wire_commitments)
%o2 = zk.proto.require "statement_bound"(%beta)
%o3 = zk.proto.require "verifier_replayable"(%beta)
zk.proto.discharge %o0 { by = "fs_checker" }
```

#### Verifier replay

```mlir
%tr0 = zk.verify.tr.init {
  protocol = "plonk/v1",
  codec = "canonical",
  instance_label = %stmt_root
}
%tr1 = zk.verify.tr.absorb_meta %tr0, %stmt_root { label = "statement" }
%tr2 = zk.verify.tr.absorb_slot %tr1, @wire_commitments { label = "wire_comms" }
%tr3, %beta = zk.verify.tr.challenge %tr2 { class = "beta", field = "bn254_fr" }
```

This is the right degree of explicitness for a research-grade protocol IR.

---

## 10. Stage-typed transcript programs

A transcript should not be modeled as a flat list of absorb/challenge events. The right design is a **stage-typed transcript state machine**.

Each stage should specify:

- stage identifier,
- required subject classes,
- canonical order,
- namespace/domain separator,
- permitted challenge classes,
- replay conditions,
- and, where the protocol needs it, declared codec or initialization policy.

A stage specification may be recorded as an attribute such as:

```mlir
#zk.fs.stage_spec<
  id = "perm",
  required = [@wire_commitments, @z_commitment],
  canonical_order = ["wire_commitments", "z_commitment"],
  challenges = ["beta", "gamma"],
  namespace = ["fiat-shamir", "perm"]
>
```

This is stronger than plain dominance. It says not only that something happened earlier, but that it happened in the right logical phase with the right canonical subject set.

This design is also directly aligned with the strongest modern implementation practice, where transcript stages and fixed labels are treated as protocol definitions rather than runtime choices.

---

## 11. Challenge policies

A useful compiler-level design is to associate each challenge class with a **challenge policy**.

A challenge policy states:

- which subject classes must influence the challenge,
- what namespace it belongs to,
- what sampler/decoder is used,
- what theorem profile permits it,
- what codec assumptions apply,
- and what verifier replay conditions apply.

For example:

- `beta` may require statement binding plus all phase-1 commitments,
- `query_seed` may require all oracle roots for the previous stage,
- `fri_alpha_i` may require all prior FRI commitments and low-degree-test metadata.

This is how the compiler moves from “transcript order seems okay” to **strong-FS-by-construction**.

---

## 12. Static contracts

Protocol IR should be held together by four contract systems.

### 12.1 Transcript / Fiat–Shamir contract

This contract ensures:

- stage discipline,
- canonical transcript prefixes,
- challenge provenance,
- required influence,
- namespace and label correctness,
- verifier replay equivalence,
- codec and framing discipline where declared.

### 12.2 Entropy / ZK contract

This contract ensures:

- prover-private randomness is explicit,
- masking/blinding obligations are tracked,
- zero-knowledge randomization is inserted at the protocol layer,
- transcript and entropy are not accidentally conflated.

### 12.3 Security contract

Security should be represented as a structured profile rather than a single scalar. A useful decomposition is:

- **assumption profile**,
- **budget vector**,
- **capability profile**.

This matters because Fiat–Shamir admissibility is inseparable from the security model.

### 12.4 Verifier / ABI / shape contract

This contract ensures:

- explicit verifier completeness,
- fixed proof slots,
- stable transcript/proof coupling,
- witness-independent proof topology.

---

## 13. Fiat–Shamir profiles

The protocol should explicitly record its Fiat–Shamir profile. A useful shape is:

```mlir
#zk.fs.profile<
  lemma = sigma_qrom | special_sound_multi_round | rbr_iop | manual,
  model = rom | qrom | agm,
  verifier_input = full | digest | oracle,
  extraction = rewinding | straight_line,
  reduction_loss = ...
>
```

This makes two important points explicit.

First, Fiat–Shamir is not one theorem.

Second, protocol closure is only justified if the profile is compatible with the concrete protocol class and challenge schedule.

If the implementation also chooses a more concrete transcript construction profile—such as a declared duplex sponge or codec discipline—that should refine rather than replace the theorem-facing profile. This keeps cryptographic admissibility and transcript construction policy related but conceptually distinct.

---

## 14. Family structure

Fiat–Shamir semantics should be family-generic, but the semantic objects it binds are family-specific.

That implies the following design split.

### 14.1 Small common core
The common Protocol IR core should contain:

- transcript,
- entropy,
- proof ABI,
- obligations,
- verifier scaffold.

### 14.2 Feature layers
Rich protocol structure should live in feature layers such as:

- `zk.proto.oracle`
- `zk.proto.pcs`
- `zk.proto.fri`
- `zk.proto.merkle`
- `zk.proto.oods`
- `zk.proto.lookup`
- `zk.proto.permutation`

### 14.3 Explicit protocol class
The protocol should still carry an explicit class/profile such as:

- `stark`
- `plonkish`
- `marlin`
- `sigma`

This gives the best of both worlds:

- the internal IR remains compositional,
- the external protocol identity remains explicit.

### 14.4 Why this matters for STARKs
For STARK-like systems, the absorbed transcript objects are not just commitment/open/eval artifacts. They include:

- oracle roots,
- OODS/DEEP bundles,
- FRI plans,
- query surfaces,
- Merkle decommitment bundles.

This is why a monolithic PCS-flavored `commit/open/eval` core is not sufficient.

---

## 15. MLIR methodology

This design maps well to MLIR if the following methodology is followed.

### 15.1 Use CFG-compatible regions

Since MLIR’s side-effect modeling is defined for CFG regions rather than arbitrary graph regions, `prover` and `verify` should remain CFG-compatible, even if the initial implementation uses only single-block regions.

### 15.2 Use side-effect resources

Transcript, entropy, proof-surface, and obligation state should be modeled through side-effect resources, not hidden mutable state.

### 15.3 Use interfaces aggressively

The following interfaces are especially useful:

- `TranscriptContributorOpInterface`
- `CanonicalTranscriptEncodingInterface`
- `VerifierReplayOpInterface`
- `FSJustificationOpInterface`
- `ChallengeClassOpInterface`
- `ProofABIContributorOpInterface`
- `SecurityNeedOpInterface`

These let analyses and rewrites operate generically without hardcoding dialect-specific operation names.

### 15.4 Use symbols for proof slots and fixed protocol labels

Proof slots, labels, and protocol-level namespace identifiers should be symbol-backed or otherwise statically defined, not runtime-provided strings.

This recommendation extends naturally to other stable transcript artifacts such as named codec choices or fixed initialization policies when the protocol wants those to be explicit.

### 15.5 Use the Transform dialect as a control plane

Transform IR should not define protocol semantics. Instead:

- `zk.proto` remains the payload semantics,
- `transform.zk` becomes the control layer for protocol rewrites.

This is the right place for:

- transcript canonicalization,
- commitment regrouping,
- query replanning,
- ABI finalization,
- and post-closure legality checks.

---

## 16. Pass pipeline

A clean pass structure is the following.

### Phase 0 — build `zk.proto.core`

This phase lowers the scheme specification into a public-coin protocol skeleton.

It creates:

- phase and round structure,
- stage specs,
- draft proof slots,
- abstract protocol objects,
- initial obligations,
- challenge policies,
- protocol/profile metadata.

Output: composable but incomplete protocol structure.

### Phase 1 — `fs.lower`

This is the internal Fiat–Shamir compilation step.

It:

- binds protocol and statement roots,
- introduces stage-typed transcript programs,
- replaces public-coin randomness placeholders with transcript challenge ops,
- emits transcript well-formedness obligations,
- introduces verifier replay skeletons,
- and, where requested, materializes explicit codec or initialization metadata.

Output: transcriptized protocol skeleton.

### Phase 2 — construct `zk.proto.<class>.closed`

This phase performs:

- handle resolution from arithmetic + security,
- family-specific protocol object materialization,
- security synthesis and budgeting,
- commitment grouping and query planning,
- entropy/masking insertion,
- verifier fragment emission,
- transcript checking,
- FS admissibility discharge,
- ABI freezing.

Output: closed, verifier-complete, kernelization-ready protocol.

### Phase 3 — construct `zk.proto.<class>.opt`

This phase performs semantics-preserving protocol rewrites such as:

- commitment regrouping,
- query-surface replanning,
- transcript-preserving canonicalization,
- stage merging/splitting where legal,
- proof-shape stabilization.

Only after this phase should backend lowering be allowed.

---

## 17. Why this is a compiler contribution

This design is not merely a safer implementation of transcript hashing. It contributes at the compiler level in several independent ways.

### 17.1 Typed protocol construction

It treats protocol construction as a typed, effectful, obligation-carrying program rather than a sequence of ad hoc library calls.

### 17.2 Static legality for protocol rewrites

It makes transcript-preserving rewrites statically checkable.

### 17.3 Explicit semantic boundary

It strengthens the claim that protocol closure occurs before kernelization and that backend lowering preserves already-closed protocol meaning.

### 17.4 Proof ABI as a logical contract

It unifies proof layout, transcript binding, and verifier replay.

### 17.5 Admissibility-aware compilation

It moves Fiat–Shamir from “engineering folklore” to an explicit compilation discipline with theorem-backed profiles.

### 17.6 External alignment without semantic collapse

It absorbs useful structure from emerging specification, implementation, and formalization work on duplex-sponge Fiat–Shamir without collapsing Protocol IR into any one external artifact. This is important because it lets the compiler adopt clearer transcript and encoding discipline while retaining its own independent contribution.

This is precisely the sort of compiler-level contribution that distinguishes Protocol IR from circuit IR or backend scheduling infrastructure.

---

## 18. Evaluation focus

A research prototype based on this design should be evaluated on four dimensions.

### 18.1 Safety
Can the compiler reject:

- absorb-after-challenge errors,
- missing statement binding,
- missing domain separation,
- invalid challenge-class usage,
- omitted verifier replay,
- witness-dependent proof-shape changes,
- unsupported Fiat–Shamir profiles,
- declared codec or initialization mismatches where the protocol makes those explicit?

### 18.2 Expressiveness
Can the same transcript core support:

- one PCS-heavy family,
- one oracle-heavy family,
- explicit verifier generation,
- slot-aware proof ABI discipline,
- and modest alignment with external duplex-sponge-and-codec discipline where desired?

### 18.3 Optimization leverage
Can post-closure protocol rewrites improve:

- commitment count,
- opening count,
- transcript regularity,
- proof-surface simplicity,
- downstream kernel-graph structure?

### 18.4 Executability
Can one closed protocol drive a proof/verify roundtrip through the current library backend while preserving explicit verifier replay and ABI semantics?

A useful later validation step is conformance-style testing against external transcript references for small fragments. This should be treated as a secondary check, not as the definition of the IR semantics.

---

## 19. MVP and staging

A strong first implementation should remain narrow.

### MVP

- `zk.proto.protocol`
- proof slots and `bind`
- transcript init / absorb_meta / absorb_slot / challenge
- entropy init / take
- obligations require / discharge
- explicit verifier replay
- one FS checker
- one admissibility profile family
- one protocol family
- one library backend roundtrip

### Recommended family choice

Two reasonable choices exist.

- **Plonkish** is a strong case study for rich challenge structure, proof slots, and commit/open style transcripts.
- **A focused STARK slice** is a strong case study for oracle-centric transcript payloads and challenge classes.

A practical strategy is to implement the first end-to-end slice with the easier family and validate the architecture with the more demanding family next.

A small additional recommendation is to keep one lightweight conformance track in mind from the beginning: for a narrow fragment, compare the compiler’s transcript discipline against a declared external construction profile or implementation reference. This is useful as a sanity check, but it should remain subordinate to the compiler’s own semantic model.

---

## 20. Final position

The final position of this document is:

> **Fiat–Shamir should be compiled as protocol closure.**
> More precisely, Protocol IR should model Fiat–Shamir as a proof-surface-aware, stage-typed transcript calculus with explicit verifier replay, separate prover-private entropy, and certificate-carrying admissibility obligations.

This is the design that is most faithful to the modern cryptographic understanding of Fiat–Shamir, most compatible with the current Protocol IR thesis, and most defensible as a compiler and PL contribution.

It is also a design that can benefit from recent external work on duplex-sponge Fiat–Shamir without being reduced to it. The transcript layer can and should learn from cleaner interface decompositions, explicit codec discipline, better-concretized constructions, and ongoing mechanization efforts. But the compiler’s core job remains the same: to turn protocol structure into a closed, verifier-explicit, proof-surface-aware object before execution lowering begins.

The one-sentence summary is:

> **Do not model Fiat–Shamir as hash plumbing; model it as typed protocol closure.**

---

## Selected references

The following references are the ones most relevant to the research framing of this document.

### Core Fiat–Shamir theory and security

- Don, Fehr, Majenz, Schaffner. *Security of the Fiat–Shamir Transformation in the Quantum Random-Oracle Model.* 2019.
- Attema, Fehr, Klooß. *The Fiat–Shamir Transformation of Multi-Round Interactive Proofs.* 2021.
- Block et al. *Round-by-Round Soundness and the Fiat–Shamir Transformation for FRI and Related SNARKs.* 2023.
- Campanelli et al. *On the Security of Fiat–Shamir for Rational Arguments with Holographic Verification.* 2024.

### Transcript engineering and failure modes

- Merlin transcript documentation.
- Trail of Bits, Decree.
- *Weak Fiat–Shamir Attacks on Modern Proof Systems.*
- *Fiat–Shamir in the Wild.*

### Duplex-sponge, specification, and formalization anchors

- Alessandro Chiesa and Michele Orrù. *A Fiat–Shamir Transformation From Duplex Sponges.* IACR ePrint 2025/536.  
  <https://eprint.iacr.org/2025/536>
- Michele Orrù. *Fiat-Shamir Transformation.* IRTF CFRG Internet-Draft, work in progress.  
  <https://datatracker.ietf.org/doc/draft-irtf-cfrg-fiat-shamir/>
- `arkworks-rs/spongefish`: a duplex-sponge Fiat–Shamir library.  
  <https://github.com/arkworks-rs/spongefish>
- `Verified-zkEVM/ArkLib`: Lean-oriented formalization effort for modern proof systems, including duplex-sponge Fiat–Shamir in its stated scope.  
  <https://github.com/Verified-zkEVM/ArkLib>
