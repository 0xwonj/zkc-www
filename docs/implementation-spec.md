# Implementation Spec

**Status:** canonical implementation-facing document  
**Owns:** the exact MLIR-level structure of the current Protocol IR prototype; physical dialect packaging; top-level IR objects; symbol model; type and attribute surface; effect/resource model; verifier IR shape; stage-transition legality; pass inventory; Transform extension design; current backend boundary; testing and implementation order  
**Does not own:** the project-level architecture (`system-and-scope.md`), the paper-level thesis and semantic commitments (`protocol-ir.md`), or the medium-/long-term roadmap (`extensions-and-roadmap.md`)  
**Depends on:** `system-and-scope.md`, `protocol-ir.md`

---

## Abstract

This document specifies **how Protocol IR is implemented in MLIR**.

The research thesis has already been fixed elsewhere: Protocol IR is the semantic lock-in layer between arithmetic structure and execution structure; closure happens before kernelization; a closed protocol denotes the triple **(proof surface, transcript trace, verifier relation)**. The present document is narrower and more concrete. Its job is to define the MLIR object model that realizes those claims.

The implementation is organized around three permanent separations.

First, **payload semantics** live in `zk.proto.*`. These ops describe the prover-side protocol procedure and the family-specific protocol objects manipulated by that procedure.

Second, **verifier semantics** live in `zk.verify.*`. These ops make proof readback, transcript replay, and acceptance checks explicit.

Third, **rewrite control** lives in `transform.zk.*`, implemented as a Transform-dialect extension rather than as payload semantics.

The design uses MLIR in the way MLIR is strongest: a multi-level staged IR architecture with legality-driven lowering, interface-based generic analyses, symbol-backed named protocol objects, and explicit side-effect resources. Stage transitions `core -> closed -> opt` are implemented as a sequence of analyses, rewrites, and legality checks rather than as ad hoc global state changes. Transcript and entropy are modeled as linear-like SSA token flows; the proof surface is modeled via symbols plus explicit bind/read ops; cryptographic requirements are modeled as first-class obligations. The `prover` and `verify` regions remain CFG-compatible because MLIR’s side-effect model is specified for CFG regions, not graph regions. [MLIR], [Dialect Conversion], [Interfaces], [Symbols], [Side Effects]

This document also adopts several design lessons from adjacent PL work. Transcript and entropy are explicit effect families rather than ambient services; slot binding is single-assignment and symbol-addressable; obligations are carried and discharged explicitly rather than buried in comments or pass conventions; and hidden global protocol state is avoided. This is closer in spirit to effect-typed and proof-carrying low-level IR design than to a thin wrapper around proving-library calls. [PCC], [SSA as FP], [Algebraic Effects], [Linear Haskell]

When Fiat–Shamir is present, the intended transcript surface is also chosen to remain compatible with the emerging duplex-sponge view of transcript state, codec discipline, and explicit initialization context. Those external anchors are useful, but they remain subordinate to Protocol IR’s own proof-surface, verifier-replay, and obligation-carrying semantics. [Duplex Sponge FS], [IRTF FS], [spongefish], [ArkLib]

The current executable boundary remains:

```text
zk.proto.<class>.closed | zk.proto.<class>.opt
  -> LibraryBackend
```

A future `KernelBackend` will consume the same closed protocol semantics and lower them to `zk.kernel -> zk.plan / zk.memplan`, but that is outside the current implementation scope. [HEIR]

---

## 1. Purpose

This document answers the implementation questions left intentionally open by `protocol-ir.md`:

1. What concrete MLIR dialects, op families, symbols, types, and attributes exist.
2. How the top-level protocol object is represented.
3. How proof slots, transcript stages, challenge classes, and verifier reads are named and referenced.
4. How transcript, entropy, proof surface, and obligations are represented as MLIR resources.
5. How `core -> closed -> opt` is realized using MLIR legality, analyses, and passes.
6. How `transform.zk` controls rewrites without becoming part of payload semantics.
7. What is required for the current executable `LibraryBackend` path.
8. What remains open.

This document is deliberately not a paper narrative. It assumes the thesis and semantic intent from `protocol-ir.md` and turns them into an MLIR design that is specific enough to implement.

---

## 2. Implementation stance

The implementation follows eight fixed rules.

### 2.1 Use MLIR as a multi-level compiler, not as a pretty printer

The protocol layer is not a flat “custom op set.” It is a staged compiler IR. The implementation therefore relies on:

- dialect-local invariants,
- stage-specific legality,
- interface-based generic analyses,
- and explicit lowering boundaries,

instead of informal phase conventions. [MLIR], [Dialect Conversion]

### 2.2 Use payload IR for semantics and Transform IR for control

The Transform dialect is explicitly designed as a control layer rather than as a replacement for passes or payload semantics. `transform.zk.*` therefore orchestrates legal rewrites over `zk.proto.*`; it does not define protocol meaning. [Transform Paper], [Transform Docs], [Transform Tutorial]

### 2.3 Keep named protocol objects symbol-backed

Proof slots, transcript stages, challenge classes, and ABI artifacts are all named semantic objects. They should be represented as symbols and referenced with `SymbolRefAttr`, not as stringly typed attributes passed around ad hoc. [Symbols]

### 2.4 Keep effectful protocol code in CFG regions

MLIR’s side-effect rationale is defined for operations in CFG regions. The `prover` and `verify` regions therefore remain SSACFG-compatible, even when the MVP uses a single block. Graph-region protocol bodies are out of scope for the current implementation. [Side Effects], [Interfaces]

### 2.5 Make hidden protocol state illegal

Family-specific protocol ops must not silently update the transcript, silently emit proof bytes, or silently consume randomness. Transcript updates, proof-surface writes, verifier reads, and entropy consumption must be explicit ops.
This rule also applies when external transcript libraries are used as implementation references: their API boundaries may inform conformance testing, but they must not determine the Protocol IR object model.

### 2.6 Prefer explicit resources over ambient services

Transcript, entropy, and obligations are modeled explicitly. This follows the same general philosophy as algebraic-effect and proof-carrying designs: important effects and obligations should be part of the IR object model rather than ambient runtime services. [Algebraic Effects], [Abstracting Effects], [PCC]

This also keeps the transcript surface close to current duplex-sponge Fiat–Shamir specifications, which prefer explicit sponge state, codec choice, and initialization context over ambient hashing conventions. [IRTF FS], [Duplex Sponge FS]

### 2.7 Use linear-like discipline where semantics require it

MLIR’s type system is not linear, but transcript and entropy values should be treated as **linear-like**: the compiler enforces single-chain usage modulo CFG block-argument forwarding. This is crucial for a stage-typed transcript calculus and explicit entropy flow. [Linear Haskell]

### 2.8 Keep semantics-lowering to verification tools possible

`zk.verify` is a protocol-facing verifier IR, not yet a full mechanized semantics framework. But its shape should stay simple enough that future lowering into semantics-supporting verification dialects or SMT-oriented pipelines remains straightforward. [Verification Dialects]

---

## 3. Physical packaging and namespaces

### 3.1 Conceptual layering

Conceptually, the protocol layer is split into these subdialects:

- `proto.core`
- `proto.oracle`
- `proto.pcs`
- `proto.fri`
- `proto.merkle`
- `proto.oods`
- `proto.lookup`
- `proto.permutation`
- `verify`
- `abi`

This is the **semantic** split.

### 3.2 Physical MLIR packaging in the current implementation

In the current implementation, use:

- one physical project dialect `zk`,
- one Transform-dialect extension contributing `transform.zk.*` ops,
- and standard MLIR dialects where needed (`arith`, `cf`, `scf`, `func`, `index`, `builtin`).

The public textual families are still:

- `zk.proto.*`
- `zk.proto.oracle.*`
- `zk.proto.pcs.*`
- `zk.proto.fri.*`
- `zk.proto.merkle.*`
- `zk.proto.oods.*`
- `zk.proto.lookup.*`
- `zk.proto.permutation.*`
- `zk.verify.*`
- `zk.abi.*`
- `transform.zk.*`

This is a deliberate compromise. MLIR dialect namespaces are physically simpler when they are single-segment, but the op-family prefixes still preserve semantic separation. Internally, ODS/TableGen definitions and C++ libraries should still be split by conceptual ownership.

### 3.3 ODS and IRDL discipline

The primary definition source should be ODS/TableGen, which is MLIR’s standard path for dialect definitions. The stable user-visible surface should be declared there and kept modular by family. [ODS], [Attributes and Types], [Creating a Dialect]

In parallel, the project should maintain an **optional IRDL mirror** of the stable subset of the dialect for documentation, schema inspection, linting, and future out-of-tree tooling. IRDL is not required for runtime compilation in the MVP, but it is valuable as design meta-tooling and as a check that the dialect surface is not becoming accidentally opaque. [IRDL]

### 3.4 Prefix discipline

The implementation should use the following conventions:

- semantic payload ops keep the `zk.proto.*` or `zk.verify.*` prefixes;
- module-level ABI and auxiliary symbols keep the `zk.abi.*` prefix;
- Transform ops injected into the Transform dialect keep the `transform.zk.*` prefix, as recommended by the Transform dialect extension mechanism. [Transform Docs]

---

## 4. Admissible surrounding dialects inside protocol regions

Protocol regions are not closed worlds. They may contain standard MLIR ops where that improves clarity or avoids redundant redefinition.

### 4.1 Allowed in `prover` and `verify`

The following dialects are admissible inside `prover` and `verify` in the current implementation:

- `builtin`
- `arith`
- `cf`
- `scf`
- `index`
- `math`

These provide scalar computation and control flow without injecting backend-specific execution semantics.

### 4.2 Conditionally allowed

`tensor` may be used for purely symbolic aggregates or compile-time bookkeeping if it does not introduce backend memory/execution semantics.

### 4.3 Disallowed in the current prototype

The following should not appear inside `zk.proto.protocol` regions in the current prototype:

- `memref`
- `vector`
- `async`
- `gpu`
- `spirv`
- `llvm`
- backend-specific library-call dialects

These belong below the current protocol boundary. If they appear before library or kernel lowering, protocol meaning and execution strategy are being conflated.

### 4.4 Temporary conversion artifacts

`builtin.unrealized_conversion_cast` may appear only as a transient artifact during conversion. It is illegal in all user-visible `closed` and `opt` protocols.

---

## 5. Top-level IR container

### 5.1 Overall structure

The top-level payload object is:

```mlir
module {
  zk.abi.public  @pub_v1
  zk.abi.witness @wit_v1

  zk.proto.protocol @example
    attributes {
      class       = #zk.proto.class<plonkish>,
      stage       = #zk.proto.stage<core>,
      fs_profile  = #zk.fs.profile<lemma = "sigma_qrom", model = "qrom">,
      security    = #zk.proto.security<...>,
      public_abi  = @pub_v1,
      witness_abi = @wit_v1
    } {
  ^decls:
    ...
  ^prover(%pub: !zk.abi.public<@pub_v1>, %wit: !zk.abi.witness<@wit_v1>):
    ...
    zk.proto.complete
  ^verify(%pub: !zk.abi.public<@pub_v1>):
    ...
    zk.verify.complete
  }
}
```

### 5.2 Traits and interfaces on `zk.proto.protocol`

`zk.proto.protocol` should carry:

- `SymbolOpInterface`
- `OpTrait::SymbolTable`
- `OpTrait::IsolatedFromAbove`
- `RegionKindInterface`
- `OpAsmOpInterface`

This makes the protocol itself a named symbol-table-bearing container, gives it clean symbol lookup behavior, prevents accidental capture of external SSA state, allows region kind distinctions, and keeps custom assembly readable. [Symbols], [Interfaces]

### 5.3 Regions

`zk.proto.protocol` has three regions.

#### Region 0: `decls`

This region contains protocol-local symbol declarations such as proof slots, stage specs, and challenge classes.

It is a single-block declaration region and should be treated as **graph-like** for `RegionKindInterface` purposes. It has no protocol execution semantics.

This region is an implementation refinement of the research document’s logical `slots` region. The paper-level semantics still center proof slots; the implementation broadens the region to hold all protocol-local symbolic declarations.

#### Region 1: `prover`

This is an **SSACFG** region. It contains prover-side protocol construction, transcript logic, entropy consumption, proof-surface binding, and family-specific protocol object materialization.

#### Region 2: `verify`

This is also an **SSACFG** region. It contains explicit verifier-side proof readback, transcript replay, and acceptance checks.

### 5.4 Terminators

The protocol regions use custom terminators:

- `zk.proto.complete` for the `prover` region
- `zk.verify.complete` for the `verify` region

Intermediate blocks may terminate with standard `cf.br` / `cf.cond_br`, but any block that semantically completes the region must end in the corresponding custom terminator.

### 5.5 Block arguments

The default block arguments are:

- `prover`: `%pub`, `%wit`
- `verify`: `%pub`

The verifier does **not** take an explicit proof SSA value in the MVP. Instead, the proof surface is the ambient proof under verification, and `zk.verify.read` makes proof reads explicit. This keeps verifier IR focused on the logical proof contract rather than on a concrete serialized container. Whether a first-class proof value should be added after proof-ABI freeze is an open question recorded later in this document.

---

## 6. Declaration-region symbols and ABI model

### 6.1 Local protocol symbols

The `decls` region may contain the following symbol ops in the MVP or near-MVP surface:

- `zk.proto.slot`
- `zk.proto.stage_spec`
- `zk.proto.codec`
- `zk.proto.challenge_class`

An optional near-MVP extension is a symbol-backed transcript codec declaration, but the core requirement is only that transcript framing and challenge decoding remain explicit rather than ambient. All declaration-region symbols are immediate children of the enclosing protocol symbol table.

### 6.2 `zk.proto.slot`

A slot declaration has no SSA results and names a proof-visible object.

Representative form:

```mlir
zk.proto.slot @wire_comms
  { kind = "commitment_bundle",
    encoding = #zk.proto.encoding<"canonical">,
    shape = #zk.proto.shape<static> }
  : !zk.proto.commitment<lemma = "pcs">
```

A slot declaration fixes:

- the slot symbol name,
- the logical kind,
- the value type read/written through the slot,
- the canonical proof/transcript encoding policy,
- and the required shape policy.

### 6.3 `zk.proto.stage_spec`

A stage specification symbol names one transcript stage.

Representative form:

```mlir
zk.proto.stage_spec @perm
  { namespace = ["fs", "perm"],
    required_tags = ["statement"],
    required_slots = [@wire_comms, @z_comm],
    canonical_order = [@wire_comms, @z_comm],
    allowed_challenges = [@beta, @gamma] }
```

A stage specification may additionally carry explicit codec or framing policy once the transcript surface is frozen. The important implementation point is that such policy remains part of the protocol declaration layer, not an implicit convention hidden inside transcript helpers.

A stage spec exists to support **stage-typed** transcript checking. It is not merely documentation.

### 6.4 `zk.proto.codec`

A codec declaration names a canonical transcript/challenge codec associated with a Fiat–Shamir profile.

Representative form:

```mlir
zk.proto.codec @canonical_field_codec
  { prover_message = "canonical_absorb",
    verifier_challenge = "field_uniform" }
```

A codec symbol does not itself perform transcript IO. It records the canonical mapping between absorbed proof subjects and transcript state, and between transcript state and typed verifier challenges. This keeps codec choice explicit and symbol-backed while leaving transcript semantics in `zk.proto.tr.*` rather than in a library-side helper.

### 6.5 `zk.proto.challenge_class`

A challenge-class symbol names one derived challenge and its static discipline.

Representative form:

```mlir
zk.proto.challenge_class @beta
  { stage = @perm,
    field = "bn254_fr",
    codec = @canonical_field_codec,
    sampler = #zk.proto.sampler<field_uniform>,
    required_tags = ["statement"],
    required_slots = [@wire_comms, @z_comm] }
```

This allows challenge values to have specific types such as `!zk.proto.challenge<@beta>` rather than generic “field element” types. When a challenge class references a codec, the mapping from transcript state to verifier message is fixed by the protocol definition rather than by backend convention.

### 6.6 Module-level ABI symbols

At module scope, use:

- `zk.abi.public`
- `zk.abi.witness`
- `zk.abi.proof`

Only the **existence and symbol identity** of public and witness ABI definitions are required by this document. Their internal field grammar may be owned by a separate ABI document later.

The proof ABI, however, is central here because it is the frozen projection of the proof surface.

Representative form:

```mlir
zk.abi.proof @proof_v1
  { version = 1,
    ordered_slots = [@wire_comms, @z_comm, @opening],
    encodings = [...],
    compatibility = "frozen" }
```

`AbiFreezePass` synthesizes or finalizes this symbol and writes `proof_abi = @proof_v1` onto the enclosing `zk.proto.protocol`.

### 6.7 Symbol references

Use:

- `FlatSymbolRefAttr` for references to symbols in the nearest enclosing `zk.proto.protocol` symbol table;
- nested `SymbolRefAttr` only when referencing symbols inside nested symbol tables or module-level ABI objects from outside their immediate table. [Symbols]

Ops that reference any of these symbols should implement `SymbolUserOpInterface`. [Symbols], [Interfaces]

### 6.8 Slot and ABI invariants

The implementation must enforce:

- each slot symbol name is unique within the protocol;
- each slot is bound at most once in the prover;
- verifier reads may target only declared slots;
- slot types and read/write types agree exactly;
- codec references target declared codec symbols when the selected Fiat–Shamir profile requires them;
- the logical proof ABI is derived from slot declarations plus explicit version/freeze policy;
- slot declaration order is the default logical order unless an explicit proof-ABI symbol says otherwise.


## 7. Resource, effect, and linearity model

Protocol IR is a **four-resource IR**.

| Resource | Representation | Primary ops | Discipline |
|---|---|---|---|
| Transcript | SSA token values + `TranscriptResource` effects | `tr.init`, `tr.begin_stage`, `tr.absorb_*`, `tr.challenge`, `tr.end_stage` | linear-like |
| Entropy | SSA token values + `EntropyResource` effects | `ent.init`, `ent.take`, `ent.split`, `mask` | linear-like |
| Proof surface | slot symbols + `ProofSurfaceResource` effects | `bind`, `verify.read` | single-assignment / symbol-addressed |
| Obligations | SSA obligation values + `ObligationResource` effects | `require`, `discharge` | affine / must close |

### 7.1 Why both SSA tokens and side effects

Transcript and entropy are represented both as SSA values and as side-effecting resources.

- The SSA values make sequencing and dominance explicit.
- `MemoryEffectsOpInterface` lets generic MLIR infrastructure reason conservatively about reordering, erasure, and motion. [Side Effects]

This dual representation is intentional. The IR should be analyzable both by custom protocol passes and by MLIR’s existing transformation machinery.

### 7.2 Custom side-effect resources

Define at least these resource classes:

- `TranscriptResource`
- `EntropyResource`
- `ProofSurfaceResource`
- `ObligationResource`

`zk.proto.bind` writes the proof surface. `zk.proto.tr.absorb_slot` reads the proof surface and writes the transcript. This is one reason the “bind slot, then absorb slot” design is stronger than absorbing a raw value: the resource graph reflects the intended dependency.

### 7.3 Linear-like transcript discipline

Transcript tokens must not fan out arbitrarily.

A transcript token may have:

- one non-terminator use in a straight-line block,
- or multiple outgoing uses only via CFG branch forwarding into block arguments that re-establish a single token per path.

The same rule applies to stage tokens.

This is **not** enforced by core MLIR typing. It is a verifier/analysis property enforced by protocol passes.

### 7.4 Linear-like entropy discipline

Entropy tokens follow the same rule. If multiple independent streams are required, they must be made explicit with `zk.proto.ent.split`, not by reusing the same token in two places.

### 7.5 Obligation liveness

Obligation values are affine: they may be consumed by discharge, but no live obligation may reach a complete terminator on a successful path.

The IR may retain `require`/`discharge` ops as audit evidence in `closed` or `opt`, but it may not retain *pending* obligations.

---

## 8. Types, attributes, and op properties

### 8.1 Core types

The following types are first-class in the MVP or near-MVP surface:

```mlir
!zk.abi.public<@pub_v1>
!zk.abi.witness<@wit_v1>

!zk.proto.transcript<
  fs_spec = "duplex_sponge",
  hash = "poseidon2",
  model = "qrom">
!zk.proto.stage_token<@perm>
!zk.proto.challenge<@beta>

!zk.proto.entropy<policy = "bundle_rng">
!zk.proto.random<class = "blinder", field = "bn254_fr">

!zk.proto.obligation<kind = "hiding">
!zk.proto.obligation<kind = "verifier_complete">
!zk.proto.obligation<kind = "fs_init_complete">
!zk.proto.obligation<kind = "codec_conformant">

!zk.proto.commitment<lemma = "pcs">
!zk.proto.oracle_root<lemma = "merkle">
!zk.proto.opening<lemma = "ipa_multiopen">
!zk.proto.query_surface<lemma = "stark">
!zk.proto.decommitment<lemma = "merkle_bundle">
!zk.proto.eval_bundle<lemma = "oods">
!zk.proto.fri_plan<lemma = "fri">

!zk.proto.unresolved<kind = "commitment">
```

### 8.2 Type design rule

Use a type when the fact affects **operational meaning** of SSA values or static compatibility checks.

Examples:

- `!zk.proto.challenge<@beta>` should be a type, not just an attribute on an `iN`, because challenge-class identity matters operationally.
- `!zk.proto.oracle_root<lemma = "merkle">` should be a type, not a string tag on bytes, because verifier and transcript behavior depend on it.

### 8.3 Structural attributes

The following attributes should be first-class:

```mlir
#zk.proto.class<stark | plonkish | marlin | sigma | ...>
#zk.proto.stage<core | closed | opt>

#zk.proto.shape<static | runtime_value | secret_value | shape_dependent>

#zk.proto.abi<version = 1, state = "draft" | "frozen">

#zk.proto.security<
  assumptions = ...,
  budgets = ...,
  capabilities = ...>

#zk.fs.profile<
  lemma = "sigma_qrom" | "multi_round" | "rbr_iop" | "manual",
  model = "rom" | "qrom" | "agm">

#zk.proto.fs_init<
  protocol_id = ...,
  session_policy = ...,
  instance_label_policy = ...,
  codec = @...>
```

When a protocol uses an explicit duplex-sponge-style transcript surface, the profile may also carry or reference a fixed initialization context and codec policy. In the current implementation notes this is treated as a compatibility and conformance concern, not as a separate semantic layer beyond `zk.proto` itself. [Duplex Sponge FS], [IRTF FS]

### 8.4 Shape typing policy

Keep the earlier four-way shape vocabulary because it is already consistent with the research documents:

- `static`
- `runtime_value`
- `secret_value`
- `shape_dependent`

For closure purposes, `runtime_value` and `secret_value` are both “value-only” and may vary across proofs. `shape_dependent` indicates that a value has flowed into proof topology or verifier-visible structure in a way that must be rejected or repaired before closure.

### 8.5 Op properties

Use op properties only for **nonsemantic** data such as pass-local caches, debug metadata, or cost summaries.

Do **not** store proof meaning, ABI meaning, transcript meaning, or security meaning solely in op properties. Those must remain explicit in attributes, symbols, or SSA values so they survive printing, serialization, and external inspection.

### 8.6 ODS layering

Types and attributes should be split across `.td` files by conceptual ownership:

- `ZkProtoTypes.td`
- `ZkProtoAttrs.td`
- `ZkVerifyOps.td`
- `ZkAbiOps.td`
- family-specific fragments such as `ZkProtoPCSOps.td` or `ZkProtoFRIOps.td`

This follows MLIR’s recommendation to keep such definitions modular rather than monolithic. [Attributes and Types], [ODS]

---

## 9. Core payload semantics

### 9.1 Pure protocol-object ops

Family-specific ops such as `pcs.commit`, `oracle.commit`, `fri.plan`, or `oods.reduce` should, by default, be **pure constructors of protocol objects**.

They must not:

- update the transcript,
- emit proof bytes,
- read or write ABI slots,
- or consume entropy implicitly.

If a commitment needs hiding, the hiding randomness must be explicit via `mask` or a family-specific blinding op that takes randomness as an explicit operand.

Pure protocol-object ops should implement `NoMemoryEffect` and, when sound, `ConditionallySpeculatable` or an equivalent speculatability contract. [Side Effects]

### 9.2 Proof-surface ops

#### `zk.proto.bind`

Representative form:

```mlir
%cm = zk.proto.pcs.commit %polyset { pcs = "ipa" } : ...
zk.proto.bind @wire_comms, %cm
```

Semantics:

- verifies that `%cm` matches the declared slot type of `@wire_comms`;
- writes `ProofSurfaceResource`;
- establishes the value that verifier readback and transcript absorption will refer to.

There is exactly one successful binding per slot.

### 9.3 Transcript ops

#### Initialization and stage entry

```mlir
%tr0 = zk.proto.tr.init {
         fs_spec = "duplex_sponge",
         hash = "poseidon2",
         model = "qrom",
         protocol_id = "plonk/v1",
         instance_label = "statement",
         codec = @canonical_field_codec }
%st0 = zk.proto.tr.begin_stage %tr0 @perm
```

`tr.begin_stage` returns a stage token typed by the stage symbol. The intent is to keep transcript initialization explicit enough to record protocol identity, instance-label policy, and codec choice without inheriting them from a backend library default. Public instance data is still absorbed explicitly as metadata or slots later in the transcript.

#### Absorbing protocol metadata

```mlir
%st1 = zk.proto.tr.absorb_meta %st0, %stmt_digest
         { tag = "statement", codec = "digest32" }
```

`absorb_meta` is for protocol metadata and public statement data. Witness-dependent prover messages that are proof-visible should use `bind + absorb_slot`, not `absorb_meta`. When present, fields such as `protocol_id`, `session_id`, and `instance_label` are treated as initialization-time metadata rather than as ad hoc absorb events later in the transcript. This keeps initialization context, protocol metadata, and proof-surface bindings distinguishable in the same way that recent duplex-sponge specifications distinguish explicit setup context from later prover messages. [IRTF FS]

#### Absorbing proof-visible objects

```mlir
%st2 = zk.proto.tr.absorb_slot %st1, @wire_comms
```

This reads `ProofSurfaceResource` and writes `TranscriptResource`.

#### Sampling a challenge

```mlir
%st3, %beta = zk.proto.tr.challenge %st2 @beta
```

This returns:

- an updated stage token, and
- a typed challenge `!zk.proto.challenge<@beta>`.

In the current design, external libraries such as `spongefish` are relevant as implementation and conformance references for selected fragments, but they do not define the IR object model or collapse transcript-derived public randomness with prover-private entropy. [spongefish]

#### Leaving the stage

```mlir
%tr1 = zk.proto.tr.end_stage %st3
```

### 9.4 Entropy ops

Representative surface:

```mlir
%rng0 = zk.proto.ent.init { policy = "bundle_rng" }
%rng1, %rho = zk.proto.ent.take %rng0
                { class = "blinder", bits = 128 }
%rng2, %rngA, %rngB = zk.proto.ent.split %rng1
%masked = zk.proto.mask %poly with %rho
```

Design rule:

- entropy is prover-private,
- transcript-derived challenges are public,
- the two must never be conflated.

This is intentionally stricter than user-facing transcript libraries that may expose verifier coins and prover randomness through one API surface. In Protocol IR, public transcript state and prover-private entropy remain distinct resources even if a concrete backend later routes them through the same implementation substrate.

Reference implementations may expose verifier coins and prover randomness through one ergonomic surface; the IR intentionally does not. [spongefish]

This separation is intentional even if some implementation-oriented transcript libraries expose transcript helpers and prover randomness through one ergonomic surface. Protocol IR should keep them distinct because they play different semantic and verification roles.

### 9.5 Obligation ops

Representative surface:

```mlir
%o0 = zk.proto.require "hiding"(%cm) { bits = 128 }
%o1 = zk.proto.require "verifier_complete"(%component)
%o2 = zk.proto.require "shape_stable"(@wire_comms)
%o3 = zk.proto.require "instance_bound"(%tr0)
%o4 = zk.proto.require "codec_conformant"(@canonical_field_codec)

zk.proto.discharge %o0 { by = "mask-injection", evidence = "rho:128" }
zk.proto.discharge %o1 { by = "emit-verifier" }
zk.proto.discharge %o2 { by = "shape-check" }
zk.proto.discharge %o3 { by = "fs-init-check" }
zk.proto.discharge %o4 { by = "encoding-check" }
```

The string-like examples above are for readability. The implementation may use a dedicated obligation-kind enum attribute.

### 9.6 Placeholder and unresolved ops

The `core` stage may contain unresolved protocol objects.

Representative form:

```mlir
%p = zk.proto.placeholder @main_root
       : !zk.proto.unresolved<kind = "oracle_root">
```

All unresolved types and placeholder ops are illegal in `closed`.

### 9.7 Negative design rules

The following are explicitly forbidden:

- no hidden transcript updates in commitment/open/decommit ops;
- no hidden proof emission in family-specific ops;
- no hidden entropy consumption;
- no implicit selection of transcript codecs or initialization context inside backend adaptors;
- no untyped byte-array escape hatches in the semantic core;
- no transform control ops in payload regions;
- no backend scheduling or memory-planning ops in protocol regions.


## 10. Feature layers and protocol families

The common core is intentionally small. Richer families appear through explicit feature layers.

### 10.1 PCS layer

Representative types and ops:

```mlir
!zk.proto.commitment<lemma = "pcs">
!zk.proto.opening<lemma = "ipa_multiopen">

%cm = zk.proto.pcs.commit %polyset { pcs = "ipa" }
%qp = zk.proto.pcs.query_plan %cm, %points
%op = zk.proto.pcs.open %cm at %qp
```

### 10.2 Oracle layer

Representative types and ops:

```mlir
!zk.proto.oracle_root<lemma = "merkle">
!zk.proto.query_surface<lemma = "stark">
!zk.proto.decommitment<lemma = "merkle_bundle">

%root = zk.proto.oracle.commit %oracle { scheme = "merkle" }
%surf = zk.proto.oracle.query_surface %root, %positions
%dec  = zk.proto.oracle.decommit %root, %surf, %plan
```

### 10.3 FRI layer

Representative ops:

```mlir
%fri = zk.proto.fri.plan %composition
         { queries = 42, blowup = 16, schedule = [4, 4, 2] }
```

### 10.4 Merkle layer

Representative ops:

```mlir
%paths = zk.proto.merkle.path_plan %surf { sharing = "maximal" }
```

### 10.5 OODS / DEEP layer

Representative ops:

```mlir
%deep = zk.proto.oods.reduce %trace, %alpha
          : !zk.proto.eval_bundle<lemma = "oods">
```

### 10.6 Lookup and permutation layers

These should remain explicit rather than encoded as generic algebraic afterthoughts.

### 10.7 Class/profile checking

The protocol itself always carries an explicit class:

```mlir
class = #zk.proto.class<stark>
```

Class-specific checkers validate that the expected feature families and verifier checks exist before closure succeeds.

For example, a `stark` class checker expects, at minimum:

- oracle commitments,
- query surface or equivalent,
- decommitment structure,
- low-degree-test plan,
- and the corresponding verifier fragments.

---

## 11. Verifier IR

### 11.1 Role

`zk.verify` exists to make the verifier relation first-class from the start.

It is not:

- a backend call stub,
- a serializer helper,
- or a future-only extension.

It is a semantic part of closure.

### 11.2 Core verifier ops

#### Proof reads

```mlir
%cm = zk.verify.read @wire_comms
        : !zk.proto.commitment<lemma = "pcs">
```

This reads from the implicit proof surface under verification.

#### Transcript replay

```mlir
%tr0 = zk.verify.tr.init {
         fs_spec = "duplex_sponge",
         hash = "poseidon2",
         model = "qrom",
         protocol_id = "plonk/v1",
         instance_label = "statement",
         codec = @canonical_field_codec }
%st0 = zk.verify.tr.begin_stage %tr0 @perm
%st1 = zk.verify.tr.absorb_meta %st0, %stmt_digest
         { tag = "statement", codec = "digest32" }
%st2 = zk.verify.tr.absorb_slot %st1, @wire_comms
%st3, %beta = zk.verify.tr.challenge %st2 @beta
%tr1 = zk.verify.tr.end_stage %st3
```

The verifier transcript must mirror the prover transcript contract exactly. When the protocol chooses to make codec and initialization context explicit, those choices are part of replay equivalence as well. Where external duplex-sponge or codec references are used for conformance or testing, they serve only as reference points; the IR semantics remain the explicit prover/verify replay structure shown here. [IRTF FS]

#### Family-specific checks

```mlir
%ok0 = zk.verify.pcs.check_opening %cm, %op, %points
%ok1 = zk.verify.oracle.check_decommitment %dec, %surf
%ok2 = zk.verify.fri.check %fri_bundle, %fri_plan
%ok3 = zk.verify.merkle.check_paths %paths, %roots
```

#### Acceptance

```mlir
zk.verify.assert %ok0
zk.verify.assert %ok1
zk.verify.complete
```

Use standard `arith` / `cf` / `scf` for ordinary boolean construction and control flow. `zk.verify.assert` is the semantic boundary between local boolean computation and the verifier acceptance relation.

### 11.3 Verifier completeness rule

Any payload op or component that affects verifier acceptance must either:

- emit a verifier fragment directly through `ZkVerifierEmitterInterface`, or
- emit an obligation whose discharge constructs the required fragment.

Closure fails if verifier completeness is not established.

### 11.4 Future semantics-lowering path

`zk.verify` should stay sufficiently deterministic and explicit that a future lowering pass can translate it into semantics-supporting verification dialects or solver backends without reverse engineering prover-library behavior. This follows the core verification-dialect lesson that semantics should not be recovered only from one chosen lowering path. [Verification Dialects]

---

## 12. Interfaces and analyses

### 12.1 Builtin MLIR interfaces and traits that should be used

The implementation should rely aggressively on existing MLIR interfaces and traits.

#### Symbol-related

- `SymbolOpInterface`
- `SymbolUserOpInterface`
- `OpTrait::SymbolTable`

These handle named protocol objects and safe symbol use. [Symbols], [Interfaces]

#### Region and control-flow related

- `RegionKindInterface`
- CFG-compatible regions for `prover` and `verify`

#### Effect-related

- `MemoryEffectsOpInterface`

#### Printing and ergonomics

- `OpAsmOpInterface`

#### Purity / motion / folding

- `ConditionallySpeculatable`
- `NoMemoryEffect`

#### Type-related, where profitable

- `InferTypeOpInterface`

### 12.2 Custom protocol interfaces

The protocol-specific interface set should include at least:

#### `ZkProtocolComponentOpInterface`

For family-specific protocol ops.

Suggested methods:

- `getSupportedClasses()`
- `getLegalStages()`
- `emitInitialObligations(...)`
- `requiredCapabilities(...)`
- `requiredAssumptions(...)`

#### `ZkTranscriptSubjectInterface`

For ops or types that may be absorbed into the transcript.

Suggested methods:

- `getTranscriptSubjectKind(...)`
- `getCanonicalTranscriptEncoding(...)`

#### `ZkCanonicalEncodingInterface`

For encoding-sensitive subjects, slots, or types.

Suggested methods:

- `getProofEncoding(...)`
- `getTranscriptEncoding(...)`
- `getFiatShamirCodec(...)`

#### `ZkVerifierEmitterInterface`

Suggested method:

- `emitVerifierFragment(...)`

#### `ZkAbiContributorInterface`

Suggested methods:

- `declareSlots(...)`
- `abiFootprint(...)`

#### `ZkShapeOpInterface`

Suggested methods:

- `getShapeClassification(...)`
- `getTopologyDependencies(...)`

#### `ZkSecurityNeedInterface`

Suggested methods:

- `securityRequirements(...)`
- `budgetContribution(...)`

#### `ZkBackendLowerableInterface`

Suggested methods:

- `lowerToLibrary(...)`
- `lowerToKernel(...)`

### 12.3 Core analyses

The pass pipeline should include reusable analyses, not only local pattern rewrites.

#### `SlotBindingAnalysis`

Checks:

- each declared slot is bound on all successful paths that require it;
- each slot is bound at most once;
- binding dominates first prover-side `absorb_slot` use.

#### `TranscriptInfluenceAnalysis`

Computes the set and order class of subjects that influence each challenge.

This powers:

- absorb-before-challenge checking,
- stage requirement checking,
- challenge-class required-subject checking,
- and any profile-specific checks that required initialization context and codec discipline are present before challenge derivation.

#### `LinearityAnalysis`

Applies to transcript and entropy tokens. Rejects token fanout that would violate single-chain semantics.

#### `ShapeStabilityAnalysis`

Tracks whether values remain `static`, `runtime_value`, `secret_value`, or have become `shape_dependent` in a topology-sensitive position.

#### `VerifierCompletenessAnalysis`

Checks that every verifier-relevant payload object has a corresponding verifier-side representation.

#### `ObligationInventoryAnalysis`

Tracks live obligations and their discharge sites.

#### `SecurityBudgetAnalysis`

Accumulates explicit budget/capability needs across components.

### 12.4 Dataflow versus symbol walks

When facts are value-propagative, use MLIR’s dataflow framework or equivalent fixed-point analyses.

When facts are symbolic or structural, prefer direct symbol-table walks, dominance/postdominance, and region-specific checks.


## 13. Stage legality and pass pipeline

### 13.1 Stage as a semantic attribute

`#zk.proto.stage<...>` is semantic state, not a cosmetic tag. Generic canonicalization must not change it. Only closure/finalization passes may do so.

### 13.2 `core`

Legal in `core`:

- placeholders and unresolved types,
- draft proof ABI,
- incomplete verifier region,
- live obligations,
- family-generic protocol skeleton structure.

### 13.3 `closed`

Required in `closed`:

- no unresolved types;
- no placeholder ops;
- no illegal transform or backend ops;
- verifier completeness established;
- transcript and entropy linearity established;
- transcript initialization/context completeness established where required by the selected Fiat–Shamir profile;
- stage/challenge legality established;
- no live obligations on successful paths;
- no `shape_dependent` influence on proof topology or verifier-visible structure;
- proof ABI materialized or explicitly versioned.

`require`/`discharge` audit ops may remain, but pending obligation values may not.

### 13.4 `opt`

Required in `opt`:

- all `closed` invariants still hold;
- only semantics-preserving protocol rewrites have been applied;
- proof ABI is either preserved or explicitly version-bumped.

### 13.5 Pass groups

#### A. Core-construction passes

##### `LowerSchemeToProtoCorePass`

Input:
- `zk.scheme`
- arithmetic-family summary

Output:
- `zk.proto.protocol[stage=core]`

Responsibilities:
- create the protocol op;
- emit local declarations (`slot`, `stage_spec`, `challenge_class`, and optionally `codec`);
- insert an initial transcript/obligation skeleton;
- materialize placeholders where family structure is unresolved.

##### `CanonicalizeProtoCorePass`

Responsibilities:
- normalize trivially equivalent spellings of the skeleton;
- remove local syntactic noise;
- leave semantic obligations untouched.

#### B. Closure passes

##### `ResolveToFamilyPass`

Responsibilities:
- eliminate placeholders;
- replace `!zk.proto.unresolved<...>` with concrete family types;
- perform legality-driven conversion to family-specific op sets.

This pass family should use MLIR’s dialect conversion infrastructure. `core -> closed` is the clearest place where `ConversionTarget`, rewrite patterns, and optional `TypeConverter`s directly match the semantic stage model. [Dialect Conversion]

##### `LowerPublicCoinToTranscriptPass`

Responsibilities:
- materialize transcript stages from declared public-coin structure;
- make transcript initialization and codec policy explicit where the profile requires it;
- replace challenge placeholders with explicit `tr.challenge` ops;
- seed verifier transcript replay skeletons.

##### `SecuritySynthesisPass`

Responsibilities:
- resolve structured security parameters;
- instantiate family-specific default budgets/capabilities where policy permits.

##### `QueryPlanningPass`

Responsibilities:
- build concrete point/query/decommitment plans.

##### `ZkMaskingPass`

Responsibilities:
- insert explicit entropy consumption and masking/blinding ops;
- discharge hiding-related obligations.

##### `EmitVerifierPass`

Responsibilities:
- emit or finalize `zk.verify` fragments using `ZkVerifierEmitterInterface`.

##### `FsCheckPass`

Responsibilities:
- verify stage structure;
- verify required influence and canonical order;
- verify challenge-class legality;
- verify transcript-init and codec consistency where configured;
- verify verifier replay equivalence.

##### `ShapeStabilityPass`

Responsibilities:
- reject or repair witness-dependent topology;
- discharge shape obligations.

##### `AbiFreezePass`

Responsibilities:
- synthesize or finalize `zk.abi.proof`;
- record ordered slots, encodings, and version;
- update `proof_abi = @...` on the enclosing protocol.

##### `ObligationClosurePass`

Responsibilities:
- ensure no live obligations remain on any successful path;
- flip `stage` from `core` to `closed` only after all checks succeed.

#### C. Optimization passes

These require `stage = closed`.

##### `RegroupCommitmentsPass`

- protocol-level cost optimization;
- ABI-preserving by default.

##### `ReplanQueriesPass`

- optimize point/query/decommitment structure while preserving the semantic triple.

##### `ProofShapeStabilizationPass`

- canonicalize proof-surface structure;
- may be used to produce a more regular ABI if policy permits.

##### `ProtocolCanonicalizationPass`

- cleanup and normalization that is known to preserve proof surface, transcript trace, and verifier relation.

##### `FinalizeOptStagePass`

- verifies post-optimization invariants;
- flips stage from `closed` to `opt`.

#### D. Backend lowering passes

##### `LowerToLibraryPass` (current)

Consumes:
- `zk.proto.protocol[stage=closed|opt]`

Produces:
- a library-backed executable representation.

##### `LowerToKernelPass` (future)

Consumes:
- `zk.proto.protocol[stage=opt]`

Produces:
- `zk.kernel` or an equivalent execution IR.

### 13.6 Full versus partial conversion policy

Use `applyFullConversion` or equivalent when moving protocol bodies into a truly closed family form.

Use partial conversion only when intentionally preserving a legal mixture of generic and family-specific ops during intermediate development or exploration.

### 13.7 ABI rewrite categories

Every protocol rewrite should be classified as one of:

1. **ABI-preserving**
2. **ABI-refining**
3. **ABI-breaking**

Default policy:

- closure may finalize ABI as `draft -> frozen`;
- post-closure optimization should prioritize ABI-preserving rewrites;
- ABI-breaking rewrites require an explicit policy gate and a version bump in `zk.abi.proof`.

---

## 14. Transform control plane

### 14.1 Design rule

`transform.zk.*` is a **control plane**, not a semantics plane.

This directly follows both the paper-level architecture and the Transform dialect design. [Transform Paper], [Transform Docs]

### 14.2 Injection mechanism

Do not define a separate payload dialect called `transform.zk`. Instead, inject `transform.zk.*` ops into the Transform dialect using `TransformDialectExtension`. [Transform Docs]

Injected transform ops must implement:

- `TransformOpInterface`
- `MemoryEffectsOpInterface`

and any custom handle/parameter types must implement one of the required Transform type interfaces. [Transform Docs]

### 14.3 Top-level organization

Use top-level `transform.named_sequence` or `transform.with_pdl_patterns` blocks under a container with the `transform.with_named_sequence` attribute, as prescribed by the Transform dialect. [Transform Docs], [Transform Tutorial]

### 14.4 Initial handle types

The MVP should primarily traffic in:

- `!transform.op<"zk.proto.protocol">`

Optional later handle types:

- `!transform.op<"zk.proto.slot">`
- `!transform.op<"zk.proto.stage_spec">`
- `!transform.op<"zk.proto.challenge_class">`

Use narrower handle types only when they provide meaningful static hardening. Otherwise prefer the simpler protocol-level handle.

### 14.5 Initial transform surface

Recommended initial ops:

- `transform.zk.require_stage`
- `transform.zk.require_class`
- `transform.zk.canonicalize_transcript`
- `transform.zk.regroup_commitments`
- `transform.zk.replan_queries`
- `transform.zk.finalize_abi`
- `transform.zk.emit_verifier`
- `transform.zk.check_no_pending_obligations`

### 14.6 Pre/post-condition policy

Every `transform.zk.*` op should document:

- required protocol stage,
- required protocol class/profile, if any,
- whether the proof ABI is preserved, refined, or broken,
- whether verifier completeness is preserved,
- whether the operand handles are consumed.

### 14.7 Handle invalidation policy

Follow the Transform dialect’s handle-consumption model strictly. If a `transform.zk.*` op mutates the payload protocol, it should usually consume the protocol handle and return a new one. Nested component handles should be assumed invalidated when their parent protocol handle is consumed. [Transform Docs]

### 14.8 Pattern matching

For structural candidate discovery, use `transform.with_pdl_patterns` and `transform.pdl_match` rather than embedding large amounts of bespoke matching logic into every transform op. [Transform Docs]

### 14.9 Example

```mlir
module attributes {transform.with_named_sequence} {
  transform.named_sequence @zk_proto_opt
      (%p: !transform.op<"zk.proto.protocol">)
      -> !transform.op<"zk.proto.protocol"> {
    %p0 = transform.zk.require_stage %p "closed"
          : !transform.op<"zk.proto.protocol">
    %p1 = transform.zk.require_class %p0 "stark"
          : !transform.op<"zk.proto.protocol">
    %p2 = transform.zk.canonicalize_transcript %p1
          : !transform.op<"zk.proto.protocol">
    %p3 = transform.zk.replan_queries %p2
          { objective = "proof_bytes" }
          : !transform.op<"zk.proto.protocol">
    %p4 = transform.zk.finalize_abi %p3
          { preserve_semantics, version = 1 }
          : !transform.op<"zk.proto.protocol">
    transform.yield %p4 : !transform.op<"zk.proto.protocol">
  }
}
```

### 14.10 Boundary with pass pipelines

Transform IR is not a replacement for the normal pass infrastructure. Coarse global stage transitions remain pass-driven; Transform scripts become the right place to orchestrate legal post-closure rewrite sequences or exploration workflows. [Transform Paper], [Transform Tutorial]


## 15. Backend lowering boundary

### 15.1 Current executable target

The current executable target is:

```text
zk.proto.<class>.closed | zk.proto.<class>.opt
  -> LibraryBackend
```

This is the only backend path this document standardizes.

### 15.2 Lowering contract

`LowerToLibraryPass` may assume:

- no unresolved protocol objects,
- verifier completeness,
- frozen or explicitly versioned proof ABI,
- explicit transcript schedule,
- no pending obligations,
- and stable proof topology.

It may **not** assume:

- any particular kernel decomposition,
- any memory layout,
- any hardware placement plan.

### 15.3 Lowering mechanism

Each family op that survives to the backend boundary should implement `ZkBackendLowerableInterface`. The lowering target may be:

- a thin library adaptor dialect,
- direct `func.call` shims into a proving library,
- or another backend bridge representation.

That choice is below the semantic boundary and may evolve without changing the protocol-level object model.

### 15.4 Future native backend

A future `KernelBackend` should consume the same closed protocol semantics and lower them into `zk.kernel -> zk.plan / zk.memplan`.

This document deliberately does not standardize those lower layers. The critical rule is only that protocol meaning is already fixed before they begin.

---

## 16. Diagnostics, testing, and validation

### 16.1 Verifier strategy

Every op family should have both:

- an ODS/C++ verifier for local structural invariants,
- and stage-level pass checks for semantic invariants that require whole-region or whole-protocol analysis.

### 16.2 Negative tests

The test suite must include rejection tests for:

- absorb-after-challenge;
- missing required absorbed subject;
- missing domain separator or stage namespace;
- missing or inconsistent transcript initialization / codec policy where the selected profile requires it;
- reusing a consumed transcript/entropy token;
- leaving a slot unbound;
- reading an undeclared slot;
- missing transcript initialization context or illegal codec reference for a profile that requires them;
- missing verifier fragment;
- unresolved placeholder in `closed`;
- unsupported `fs_profile`;
- witness-dependent proof topology.

### 16.3 Transform tests

The Transform surface should have tests for:

- handle invalidation discipline;
- stage precondition failure;
- ABI-preserving versus ABI-breaking classification;
- PDL-based candidate matching and rewrite application.

### 16.4 Roundtrip tests

At least one end-to-end vertical test should cover:

```text
zk.scheme
  -> zk.proto.core
  -> zk.proto.<class>.closed
  -> LibraryBackend
  -> prove / verify roundtrip
```

### 16.5 Differential and metamorphic validation

Because real ZK stacks contain subtle security and correctness bugs, the implementation should adopt a validation style closer to compiler testing and proof-stack fuzzing than to ordinary library smoke tests. Differential checking against reference verifiers, metamorphic mutation of transcripts/proof layouts, and safety-oriented negative testing should be standard practice. [SNARK Security SoK], [MTZK], [Circuzz]

### 16.6 IRDL and doc generation

Generate dialect documentation and, when feasible, an IRDL mirror as part of CI. This makes drift in the surface area visible and keeps the implementation legible to readers and tool builders.

---

## 17. Recommended implementation order

If starting from scratch, the best order is:

1. `zk.proto.protocol` container plus declaration region;
2. `zk.proto.slot`, `zk.proto.stage_spec`, `zk.proto.challenge_class`;
3. proof-surface binding and verifier readback;
4. transcript ops plus `FsCheckPass`;
5. obligation types and `ObligationClosurePass`;
6. verifier emission and completeness checking;
7. one family branch:
   - either PCS-heavy for the fastest end-to-end prototype,
   - or a focused oracle/STARK slice for the strongest architectural stress test;
8. `AbiFreezePass`;
9. `LowerToLibraryPass`;
10. minimal `transform.zk` orchestration.

This order yields a usable prototype quickly while keeping the implementation aligned with the research thesis.

---

## 18. Open questions

The questions below are the implementation questions that are still materially open.

### 18.1 Should verifier code receive an explicit proof SSA value after ABI freeze?

The current design intentionally keeps proof readback implicit through `zk.verify.read`. This is semantically clean, but an explicit proof object might become useful for some backends or for later wrapper generation.

### 18.2 Should ABI freeze occur at the end of `closed` or only after specific post-closure rewrites?

The default recommendation in this document is “freeze by the end of closure unless policy explicitly allows a controlled later finalization.” This is a sensible default, but there is still room to refine the exact compiler policy.

### 18.3 How much of ABI schema should live in `zk.abi.*` versus separate external documents?

This document only requires symbol identity plus proof-facing essentials. A richer ABI schema is likely useful, but keeping the current document focused argues for not overcommitting too early.

### 18.4 Should subprotocol composition be first-class in the initial IR?

A nested or callable subprotocol model is attractive, but it would add substantial complexity to symbol scoping, verifier completeness, and ABI composition. The MVP should stay monolithic unless a clear case study requires otherwise.

### 18.5 How far should the project push the IRDL mirror?

IRDL is useful for documentation, schema checking, and out-of-tree tools, but it should not become a second semantics source that diverges from ODS. The right long-term balance remains open.

### 18.6 Should graph regions ever be permitted for protocol bodies?

The current answer is no, because effect modeling and legality reasoning are much clearer in SSACFG regions. If future work finds a compelling graph-region protocol use case, it should be treated as a separate design extension rather than a casual relaxation.

### 18.7 What is the right long-term physical packaging?

The semantic split is stable. The current physical packaging—one project dialect plus Transform extension—is the simplest practical realization. If ownership boundaries or build modularity later demand a different physical split, the textual namespaces and semantics should remain unchanged.

---

## 19. Final position

The implementation should make one architectural separation explicit and permanent:

> **Protocol semantics live in `zk.proto`; verifier semantics live in `zk.verify`; rewrite control lives in `transform.zk`.**

This split is the cleanest way to preserve the research contribution while still building a practical compiler.

The second permanent rule is equally important:

> **Protocol closure is a legality-driven MLIR compilation step, not a convention inside a backend library.**

Everything in this document is organized around making that statement true in code.

---


## Selected references

### Canonical project documents

- `system-and-scope.md`
- `protocol-ir.md`
- `extensions-and-roadmap.md`
- `fiat-shamir-in-protocol-ir-research-note.md`

### MLIR architecture and documentation

- [MLIR] Chris Lattner et al. *MLIR: Scaling Compiler Infrastructure for Domain-Specific Computation*. CGO 2021.
- [Dialect Conversion] LLVM MLIR documentation. *Dialect Conversion*.
- [Interfaces] LLVM MLIR documentation. *Interfaces*.
- [Symbols] LLVM MLIR documentation. *Symbols and Symbol Tables*.
- [Side Effects] LLVM MLIR documentation. *Side Effects & Speculation*.
- [Attributes and Types] LLVM MLIR documentation. *Defining Dialect Attributes and Types*.
- [ODS] LLVM MLIR documentation. *Operation Definition Specification*.
- [Creating a Dialect] LLVM MLIR documentation. *Creating a Dialect*.
- [Transform Paper] Martin Paul Lücke et al. *The MLIR Transform Dialect: Your compiler is more powerful than you think*. CGO 2025.
- [Transform Docs] LLVM MLIR documentation. *Transform Dialect*.
- [Transform Tutorial] Oleksandr Zinenko. *Transform Dialect Tutorial*.
- [IRDL] Mathieu Fehr et al. *IRDL: an IR Definition Language for SSA Compilers*.
- [Verification Dialects] Mathieu Fehr et al. *First-Class Verification Dialects for MLIR*. PLDI 2025.
- [HEIR] Asra Ali et al. *HEIR: A Universal Compiler for Homomorphic Encryption*. 2025.

### Fiat–Shamir specification and mechanization anchors

- [Duplex Sponge FS] Alessandro Chiesa and Michele Orrù. *A Fiat–Shamir Transformation From Duplex Sponges*. 2025.
- [IRTF FS] Michele Orrù. *Fiat-Shamir Transformation*. IRTF CFRG Internet-Draft, active work item, 2026.
- [spongefish] `arkworks-rs/spongefish` — implementation reference for duplex-sponge Fiat–Shamir.
- [ArkLib] `Verified-zkEVM/ArkLib` — Lean-oriented mechanization line that includes duplex-sponge Fiat–Shamir in its formalization agenda.

### PL and systems foundations

- [PCC] George Necula. *Proof-Carrying Code*. POPL 1997.
- [TAL] Greg Morrisett et al. *From System F to Typed Assembly Language*.
- [SSA as FP] Andrew W. Appel. *SSA is Functional Programming*.
- [Algebraic Effects] Gordon Plotkin and Matija Pretnar. *Handling Algebraic Effects*.
- [Abstracting Effects] Jonathan Immanuel Brachthäuser et al. *Abstracting Effect Systems*. PACMPL 2024.
- [Linear Haskell] Jean-Philippe Bernardy et al. *Linear Haskell*.

### Security and validation

- [SNARK Security SoK] Stefanos Chaliasos et al. *SoK: What Don’t We Know? Understanding Security Risks of SNARKs*. USENIX Security 2024.
- [MTZK] Dongwei Xiao et al. *MTZK: Testing and Exploring Bugs in Zero-Knowledge (ZK) Compilers*. NDSS 2025.
- [Circuzz] Arseniy Alekseyev et al. *Circuzz*.

---

## Reference links

[MLIR]: https://dl.acm.org/doi/10.1109/CGO51591.2021.9370308
[Dialect Conversion]: https://mlir.llvm.org/docs/DialectConversion/
[Interfaces]: https://mlir.llvm.org/docs/Interfaces/
[Symbols]: https://mlir.llvm.org/docs/SymbolsAndSymbolTables/
[Side Effects]: https://mlir.llvm.org/docs/Rationale/SideEffectsAndSpeculation/
[Attributes and Types]: https://mlir.llvm.org/docs/DefiningDialects/AttributesAndTypes/
[ODS]: https://mlir.llvm.org/docs/DefiningDialects/Operations/
[Creating a Dialect]: https://mlir.llvm.org/docs/Tutorials/CreatingADialect/

[Transform Paper]: https://doi.org/10.1145/3696443.3708922
[Transform Docs]: https://mlir.llvm.org/docs/Dialects/Transform/
[Transform Tutorial]: https://arxiv.org/abs/2404.19350
[IRDL]: https://grosser.science/static/0c315060e8f3d8454de831910fbb6dd6/fehr-2022-irdl.pdf
[Verification Dialects]: https://users.cs.utah.edu/~regehr/papers/pldi25.pdf
[HEIR]: https://arxiv.org/abs/2508.11095

[Duplex Sponge FS]: https://eprint.iacr.org/2025/536
[IRTF FS]: https://datatracker.ietf.org/doc/html/draft-irtf-cfrg-fiat-shamir-02
[spongefish]: https://github.com/arkworks-rs/spongefish
[ArkLib]: https://github.com/Verified-zkEVM/ArkLib

[PCC]: https://homes.cs.washington.edu/~mernst/teaching/6.893/readings/necula-popl97.pdf
[TAL]: https://www.cs.cornell.edu/talc/papers/tal-popl.pdf
[SSA as FP]: https://www.cs.princeton.edu/~appel/papers/ssafun.pdf
[Algebraic Effects]: https://arxiv.org/abs/1306.6316
[Abstracting Effects]: https://dl.acm.org/doi/10.1145/3674641
[Linear Haskell]: https://arxiv.org/pdf/1710.09756

[SNARK Security SoK]: https://www.usenix.org/system/files/usenixsecurity24-chaliasos.pdf
[MTZK]: https://www.ndss-symposium.org/wp-content/uploads/2025-530-paper.pdf
[Circuzz]: https://aisychev.github.io/papers/ccs25-circuzz.pdf
