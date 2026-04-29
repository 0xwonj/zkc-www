# Protocol IR

## §0. About

The Protocol IR is a protocol-object layer between arithmetization and backend realization. It names an open protocol source, checks when that source can be sealed, and records what the sealed artifact is allowed to claim.

The initial active core is public-coin protocols closed into non-interactive arguments by a strong Fiat-Shamir boundary. The mainline is ROM-style closure; other oracle models require an explicit supporting theorem entry. `Strong` means closure checks transcript binding, prior-prefix visibility, domain separation, and construction discipline directly.

The framework distinguishes an open source presentation `P_core`, a closed non-interactive artifact `P_closed`, and the semantic verifier face `P_obs = (Σ,T,V)` read from that artifact. A certificate `C` attaches to `P_closed` and records the closure, preservation, verifier, and ABI evidence that make the artifact accountable.

Backend realization and ABI evidence remain part of closure and certification. Lowering, code generation, runtime scheduling, implementation architecture, and broader protocol families are companion tracks.

---

## §1. Why a Protocol IR

ZK proof systems have an implicit stratification. At the bottom sits **arithmetization** — constraint-level representations such as AIR, R1CS, Plonkish, and custom gates. At the top sits **backend realization** — the code that runs the prover and verifier. Between them lie the protocols themselves: PLONK, Halo2, HyperPlonk on Plonkish; FRI-based STARKs on AIR; Marlin and Spartan on R1CS. Each fixes its own round structure, commitment scheme, challenge schedule, transcript discipline, and proof surface. Most toolchains leave this middle layer implicit.

Without this layer, protocol-level decisions — hash choice, domain separation, absorb order, challenge-derivation rule, proof ABI — live inside the implementation. They are the protocol's semantic face, yet invisible to circuit-layer IRs and only implicit in backend code.

The consequences are on the empirical record.

- **Same protocol, different realization.** The Halo2 ecosystem shows the pattern: zcash, Privacy & Scaling Explorations, Scroll, and Axiom sit in the same broad Halo2 lineage while closing the protocol with different sponges and transcript disciplines (zcash with Blake2b, others with Keccak-based transcripts). The interesting difference is protocol-layer content: hash choice, domain separation, absorb order, and the profile branch those choices imply.
- **Fiat–Shamir application failures.** Trail of Bits' 2022 ["Frozen Heart" disclosure](#ref-frozenheart22) affected seven repositories implementing Girault's proof of knowledge, Bulletproofs, and PLONK — each missing required public-value absorbs into the transcript. The 2024 USENIX [SoK on SNARK vulnerabilities](#ref-chaliasos24) catalogues 141 bugs in implemented SNARK systems; ["Fiat–Shamir in the Wild"](#ref-nguyen24) provides a taxonomy of Fiat–Shamir implementation pitfalls with case studies, naming absorb order, domain separation, and challenge derivation as recurring fault categories.
- **Assurance tied to realized protocol objects.** Knowledge-soundness and zero-knowledge claims transfer through the properties discharged for a realized protocol object. A backend swap (KZG ↔ IPA, Poseidon ↔ Keccak) reopens exactly the protocol-layer rows affected by the changed construction profile.

The common thread in these examples is that protocol-level propositions have no typed place to live. Protocol IR is that place. It is the layer where "same source protocol, different realization" becomes a relation between protocol objects, Fiat-Shamir application becomes a judgment with explicit conditions, and assurance transfer follows the claims discharged in a certificate.

The gap has a familiar compiler-design shape: introduce an IR layer between source and target, fix semantics there, and state transformations as judgments over that semantics. Here the lower input is an arithmetized relation or payload. The upper output is a closed protocol artifact whose realization evidence is checked at closure; backend implementation, lowering, and runtime execution remain companion tracks that realize or expose that artifact. Protocol IR occupies the middle: protocol-level semantics, composition, Fiat-Shamir closure, and certificate accounting.

That middle layer is organized around three reader-facing responsibilities:

- **Describable.** The protocol must have an open form that names rounds, handles, challenge sites, component boundaries, profile dependencies, and local claims before closure erases that structure into a non-interactive artifact.
- **Composable.** Open protocol sources must combine through typed operators whose structural wiring is separate from any theorem-dependent security lift.
- **Verifiable.** Closure must be a checked boundary: live public-coin challenges are replaced by transcript-derived values only when Fiat-Shamir admissibility, theorem dispatch, and certificate obligations line up.

If the layer succeeds, three things follow. Two protocol artifacts that disagree on Fiat-Shamir profile or transcript discipline carry the disagreement in their certificates rather than only in repository diffs. A Frozen Heart-class missing absorb becomes a localized closure failure naming the obligation it broke. And reasoning about a backend swap reduces to inspecting which certificate rows the swap reopens, not re-auditing prover and verifier source.

---

## §2. Lifecycle and Observational Semantics

A protocol needs distinct faces across its stages: an open form in which it can still be composed, a closed form after Fiat-Shamir has sealed the public-coin challenge sites, and a verifier-observable face read from that closed artifact. Protocol IR gives each face a name.

```text
arithmetized relation / payload
        |
        v
P_core
typed public-coin interactive source
        |
        | close(Pi, rho, B)
        v
P_closed
closed non-interactive artifact
        |
        | obs
        v
P_obs = (Σ, T, V)

C attaches to P_closed.
```

Concretely, for PLONK: `P_core` is the gate/copy/opening source; `P_closed` is the artifact a Halo2 prover emits; `P_obs = (Σ, T, V)` is what a verifier reads.

When the current source is clear, `close(Pi,rho,B)` abbreviates `close(P_core,Pi,rho,B)`.

`P_core` is the open typed public-coin interactive source. It is the presentation manipulated by §4 and §5: component structure, source profile, protocol handles, and composition boundaries. AIR, R1CS, Plonkish constraints, tables, relation-side oracle descriptions, and polynomial shapes enter as arithmetized payload or typed component input.

`P_closed` is the sealed non-interactive artifact produced by `close(Pi,rho,B)`. The closure profile, admissibility evidence, ABI commitments, theorem dispatch, and backend realization witness are part of what has been sealed. Two artifacts can expose the same verifier-observable face while differing as closed artifacts.

`P_obs = obs(P_closed) = (Σ,T,V)` is the IR-level verifier face of `P_closed`: the value-level proof surface, semantic schedule, and acceptance relation read under the sealed materialization.

`C` is the certificate attached to `P_closed`. It records the closure evidence needed for a sealed artifact, the property rows exported as claims, and any open rows preserved as visible non-claims. Open rows are for unrequested or stronger claims; the checks needed for `P_closed` itself must be discharged.

The vocabulary is intentionally small: an **obligation** is something closure must check, a **claim** is an assurance the closed artifact is allowed to carry, a **row** (or **certificate row**) is the certificate entry that records the evidence or caveat for that claim, and a **theorem row** is an entry in the FS dispatch table of Appendix C.1.

The closure boundary uses three external records. §6 gives them their closure-time meaning.

```text
close : P_core × Pi × rho × B -> Option(P_closed × C)
```

`Pi` is the accumulated protocol profile, built from source profile, component profile contributions, and composition-operator obligations. Inside `Pi`, `Pi.fs` is the Fiat-Shamir profile: theorem family, security objective, model, visibility discipline, and construction profile. `rho` names the closure context. `B` supplies backend realization evidence for hash/permutation choice, codec, encoding, and backend conformance.

Symbols introduced here and elsewhere are indexed in Appendix E.

### The observational face

The verifier-observable face is:

```text
P_obs = (Σ, T, V)
Event := Meta | Slot | Chal
```

**Semantic verifier interface `Σ`.** The ordered, typed interface of proof slots that a verifier reads — id, semantic kind, value type, representation, and stability policy. `Σ` contains proof slots only; `Chal` events live in `T`, so verifier randomness is never treated as prover-supplied proof material. The same role relation may be realized by KZG in one artifact and IPA in another, but once the PCS-specific slot type or verifier relation changes, two artifacts may differ as `P_obs` even when their open source agrees. Concrete byte framing and codec/API compatibility are recorded by `B` and `c_abi`.

**Semantic event trace `T`.** A dependency schedule over `Meta | Slot | Chal`. `Meta` records protocol metadata that binds interpretation; `Slot` records a proof slot becoming transcript-visible; `Chal` records a verifier-randomness site with its prior-prefix dependency. `T` states the dependency schedule; `Pi.fs` and `B` state how that schedule is materialized at byte level. The Halo2 fork pattern from §1 — the same challenge schedule realized with different sponges or byte layouts — is exactly the case where two artifacts share one `T` but diverge under `Pi.fs` and `B`.

**Verifier relation `V`.** An explicit acceptance relation over public input, proof slots, and the semantic schedule, read under the sealed transcript materialization recorded by `P_closed`, `B`, and `C`. `V` may reference statement or relation digests, parameters, and component inputs already bound through `Meta` and the component surface. A native verifier routine, an in-circuit verifier gadget, and a backend API binding may all realize the same `V`. Changing the PCS-specific acceptance relation changes `V`; changing only the machine that implements it leaves `V` stable. Derived challenges are terms of `V` computed from `T` under the sealed materialization, not proof slots.

In PLONK-shaped examples, `Σ` carries commitments, evaluations, and opening proofs, and `T` contains the challenge sites — permutation challenges, quotient challenge, evaluation challenge, batching challenges — interleaved with the proof slots that make them derivable. Verifier `Query` events default to schedule/state; oracle openings, Merkle paths, and query answers may be slots (`OpeningProof` or `QueryAnswer`) when a concrete profile serializes them.

### Inputs and core invariant

Closure inputs carry initialization context — protocol identifiers, public parameters, setup metadata, and public inputs — that must be bound before any dependent challenge. `Pi.fs` decides the encoding, ordering, and exact transcript material.

**Core invariant.** `P_closed` carries closure evidence; `P_obs` is the verifier face read from it. Two artifacts agreeing on `(Σ, T, V)` under their sealed materialization may still differ as closed artifacts in their theorem profile, transcript discipline, construction profile, realization witness, or certificate. Composition acts on `P_core`; `close` produces `P_closed`; `obs` reads `P_obs`; `C` certifies `P_closed`.

---

## §3. Layer Position

Protocol IR is a protocol-object specification layer:

- **Relation payload.** AIR, R1CS, Plonkish constraints, tables, custom gates, multilinear extensions, and other arithmetization material describe the relation being proved.
- **Protocol object.** The open source, the closed artifact, the verifier face read from it, and the certificate that travels with it describe how that relation is argued about.
- **Backend realization.** Prover/verifier code, transcript APIs, commitment implementations, codecs, field encodings, kernels, and verifier artifacts execute or expose the closed protocol.

The protocol object is the middle artifact often left implicit in papers and backend libraries.

---

## §4. Describable - Open Protocol Form

`P_core` is the author-facing protocol source before Fiat-Shamir closure. It names rounds, handles, challenge sites, component boundaries, and profile dependencies while they are still visible enough to compose and check. The observational triple `P_obs = (Σ,T,V)` is read later from a closed artifact.

The core definition is deliberately small:

```text
P_core := typed public-coin interactive source
```

`typed public-coin interactive source` is the formal identity of the object; presentation through component interfaces is the authoring discipline. Protocol IR gives existing PCS, lookup, sumcheck, transcript, and folding facts a typed place before closure turns live verifier randomness into transcript-derived values and before backend code turns proof roles into implementation details.

`P_core` carries a source profile:

```text
source_kind ::= IP(public-coin)
              | IOP
              | PIOP
              | holographic-IOP
              | special-sound multi-round
              | accumulation / folding
```

`source_kind` guides theorem dispatch. In the active scope, these are public-coin-compatible dispatch classes. A PLONK-shaped source, an IOP/FRI-shaped source, a sumcheck-heavy source, and a folding source expose different obligations to `close`; the source profile tells §6 which theorem family may be relevant.

Relation material enters this source as typed payload consumed by components, with protocol commitments and openings recorded by component events and surfaces. A circuit IR or MLIR dialect may carry those payloads in an implementation; §4 fixes how protocol components consume them.

Every component interface in `P_core` follows one compact skeleton:

```text
Component K:
  role        protocol role
  methods     prover/verifier handles and call surfaces
  events      Meta | Slot | Chal obligations in T
  proof_slots proof slots contributed to Σ after closure
  properties  local refinement claims
  inputs      relation payload or external values consumed
  profile_delta requirements/contributions to Pi, especially Pi.fs
  composes    interfaces accepted by §5 operators
```

The distinction between `events` and `proof_slots` is load-bearing. `events` write obligations into the semantic schedule `T`; a component may declare `Chal` sites there because an open public-coin source still has verifier-randomness positions. `proof_slots` names proof material that can later appear in `Σ`. `Chal` lives only as an event, never as proof material. The `profile_delta` field records requirements or contributions to the accumulated protocol profile `Pi`, especially `Pi.fs`; closure still checks those obligations in §6.

The current mainline uses the five component families needed for the in-scope witnesses.

| Family | What it records in `P_core` |
|---|---|
| `PCS` | committed polynomial handles, opening proof slots, query-point challenge sites, commitment/opening assumptions |
| `LDT` | low-degree or proximity-test rounds, oracle commitments, query-answer slots, verifier challenge sites |
| `Lookup` | table commitments, tagged lookup claims, lookup proof slots, lookup challenge sites |
| `Sumcheck` | round-polynomial slots, per-round challenge sites, final evaluation handoff |
| `FS` | challenge-site declarations, transcript metadata, domain-separation requirements, codec/construction dependencies |

In §4, the `FS` family declares where challenges occur, what transcript material they depend on, and what profile information closure must later check. Actual substitution happens at `close(Pi,rho,B)` in §6.

The `properties` field is equally local. A component may label a commitment as binding, a lookup as relation-sound, a low-degree test as proximity-sound, or a transcript discipline as RO-modeled. §4 records those labels at the source boundary. §5 decides which labels can be lifted through composition, and §6 decides which labels survive Fiat-Shamir closure into `P_closed` and its certificate `C`.

**Shape stability.** Closure fixes protocol shape, not concrete witness values. Slot count, slot order, and slot kinds in `Σ`, the stage structure of `T`, the challenge schedule (sites, classes, prior-prefix dependency), and the structure of the acceptance relation `V` (which slots and schedule positions it reads, which derived terms it computes from `T`) are *shape* — fixed by `close` and identical across runs of the same `P_closed`. Payload values inside proof slots, the bytes that realize a transcript event, and the field element produced at a challenge site by `Sample` under the sealed transcript are *value* — expected to vary across runs and across witnesses. The certificate row `c_shape` (§7) records this shape so that a sealed artifact can be checked against the closed protocol it claims to realize.

---

## §5. Composable - Composition Policy

Composition assembles described open sources before closure. It builds larger `P_core` objects, updates the accumulated profile `Pi`, and leaves obligations for the later certificate. The crossing from open source to closed artifact happens at `close(Pi,rho,B)` in §6.

The lift policy has three parts. An operator checks typed interfaces and resource separation, merges proof slots and semantic event schedules, and assigns each local property label a lift status. The resulting object is still open:

```text
P_core -- link/par/fold/repeat_t --> P_core'
P_core' -- close(Pi,rho,B) --> Option(P_closed × C)
```

| Lift status | Meaning |
|---|---|
| definitional | follows from the typed structure itself |
| theorem-row dependent | needs a named theorem row before it becomes an exported claim |
| scheme-profile dependent | needs evidence from a concrete scheme profile |
| open until closure | remains visible for `close` and `C` when evidence is absent or incomplete |

There are four ways §5 lets an open source grow:

| Operator | Role | Main policy |
|---|---|---|
| `link` | sequential typed handoff | structural handoff is admitted; knowledge-soundness lift remains theorem-row dependent |
| `par` | independent side-by-side composition | requires transcript, public-input, domain-separation, and oracle/resource separation |
| `fold` | accumulation or folding lane | admitted relative to a named scheme profile; preservation remains scheme-profile dependent |
| `repeat_t` | repetition lane | records repeated structure under a regime and operand class; amplification remains theorem-row dependent |

The table separates wiring from claim preservation. Slots can be ordered, handles can be wired, and source profiles can be accumulated before closure. Security labels become protocol-level claims only through a recorded lift status and named evidence. At composition time, §5 leaves behind the composed shape, trace order, slot order, separation obligations, and unresolved lifts for `close`.

When theorem dispatch needs a monolithic view of a composed source, this document calls that view flattening: the operator tree is analyzed as one public-coin source, while `c_shape` preserves the original composition tree.

---

## §6. Verifiable - Fiat-Shamir Closure

Verifiability begins when the composed public-coin source is sealed. `close` turns live `Chal` sites into transcript-derived challenges, ties the schedule to a transcript discipline, and routes exported security claims to named theorem rows.

```text
close : P_core × Pi × rho × B -> Option(P_closed × C)

P_core   : composed typed public-coin interactive source
Pi       : accumulated protocol profile
rho      : scope / context annotation
B        : input-only backend realization witness
P_closed : closed non-interactive artifact
C        : certificate attached to P_closed
```

`close` is partial. Success returns the sealed artifact with its certificate; failure is `None`, accompanied in an implementation by a localized diagnostic.

### The closure profile

The Fiat-Shamir profile lives under `Pi`:

```text
Pi.fs = (theta, sigma, m, nu, kappa)
```

The five axes of `Pi.fs`:

- **theta** — theorem family and extractor discipline (Appendix C.1).
- **sigma** — property-indexed security objectives, conditional-claim policy, default target level `lambda`, and per-property overrides.
- **m** — oracle and security model tag. Active core defaults to ROM-style; QROM, programmable-ROM, and global-RO modes need an explicit theorem row (Appendix C.2).
- **nu** — visibility and binding discipline (statement/public-input binding, prior-prefix visibility, namespace separation).
- **kappa** — construction profile (codec, sponge, absorb/squeeze schedule, challenge extraction, output distribution).

`rho` identifies the scope in which the judgment is interpreted: protocol instance, public statement context, session label, operand namespace, repetition/fold scope, and other composition context. `Pi.fs.nu` says what visibility and binding discipline must hold in the transcript; `rho` says which instance and namespaces that discipline is interpreted over. Any part of `rho` needed for a claim must appear as `Meta` material and be bound before dependent challenges.

`B` is a typed record of closure-time realization evidence, split into three sub-records by the role each plays at closure:

```text
B = (B_kappa, B_scheme, B_abi)
```

| Sub-record | Realization evidence | Witness role |
|---|---|---|
| `B_kappa` | sponge or hash construction; transcript codec and absorb/squeeze framing; field/byte encoding; challenge sampling rule (`Sample`); output-distribution evidence | discharges the construction-profile side of `ConstructOK` against `Pi.fs.kappa`; recorded as construction witnesses in `c_fs` |
| `B_scheme` | scheme-component bindings used by `P_core` — PCS construction with curve/group and SRS source under a named hardness assumption; folding or lookup scheme bindings as relevant | identifies the scheme profile cited by scheme-conditioned rows in `c_fs.theta`; verifier-side conformance for scheme-specific `V` checks recorded in `c_abi` |
| `B_abi` | transcript API binding; proof codec for slots in `Σ`; verifier API binding; library version and replay/initialization conformance | discharges the API-conformance side of `ConstructOK` (initialization, replay); recorded in `c_abi` |

The split mirrors where the witness contributes: `B_kappa` realizes the construction profile shared by all artifacts under the same `Pi.fs.kappa`, `B_scheme` distinguishes scheme-conditioned dispatch (a KZG↔IPA swap reopens entries here, not in `B_kappa`), and `B_abi` separates proof/verifier interface conformance from the cryptographic construction itself.

**Backend evidence vs sealed semantics.** `B` is closure-time input evidence: its sub-records discharge their respective obligations (per the table above) and are recorded across `c_fs` (construction witnesses and scheme-conditioned dispatch entries in `c_fs.theta`) and `c_abi` (serialization and API conformance). It is not part of the sealed semantic object. `obs` does not reference `B`: `P_obs = obs(P_closed) = (Σ, T, V)` is read from `P_closed` independently of which backend realized the closure. The Halo2-ecosystem fork pattern from §1 is exactly this case — deployments share the protocol logic but diverge on sponge, codec, and transcript discipline — with such differences attributed to certificate rows rather than to the verifier face. PCS-class changes that shift slot types or the acceptance relation (KZG↔IPA, per §1 and §2's `V`) are a different case: those *do* change `P_obs`.

### Admissibility — the `FSAdmissible` judgment

For the default target level `lambda` drawn from `Pi.fs.sigma`, closure succeeds only if the following framework judgment is accepted. Per-property budget overrides stay inside `sigma`:

```text
FSAdmissible(P_core, Pi.fs, rho, B, lambda)
```

`FSAdmissible` is the admission test for sealing the open source under `Pi.fs`, `rho`, and `B`. `TraceOK`, `BindOK`, `SepOK`, and `ConstructOK` must pass for any sealed artifact; `TheoremOK` and `BudgetOK` decide which requested properties may be exported. Stronger or unrequested claims can later appear in `C` as open non-claims; a requested export without an accepted theorem/profile route must be removed or downgraded before closure.

| Obligation | Meaning |
|---|---|
| `TraceOK` | The semantic schedule has finite prior-prefix dependencies; every `Chal` follows the material it depends on; composition introduced no slot reuse or future-slot dependency. |
| `BindOK` | Statement/public inputs, verifier parameters, protocol/session labels, and verifier-conditioned metadata are bound before any dependent challenge. |
| `SepOK` | Protocol, component, operand, repetition-copy, fold-step, challenge-class, public-input, and oracle/resource namespaces are separated by disjoint resources or explicit namespace/domain tags. Shared resources are allowed only when profiled. |
| `ConstructOK` | Initialization, codec/framing, sponge or hash construction, field/byte mapping, challenge extraction, output distribution, and replay behavior conform to `Pi.fs.kappa` and `B`. SAFE APIs, CFRG duplex-sponge specifications, and duplex-sponge FS analyses can witness this layer ([SAFE 2023](#ref-safe23), [CFRG FS](#ref-cfrgfs), [Chiesa-Orru 2025](#ref-co25)). |
| `TheoremOK` | The source profile and `Pi.fs.theta` select theorem rows whose side conditions hold. |
| `BudgetOK` | Every exported claim has a theorem/profile-specific bound or explicitly permitted conditional evidence route accepted by `Pi.fs.sigma`. |

### Transcript discipline: `bind` vs `absorb_slot`

The well-formedness obligations above are made operational by separating two transcript actions on a proof slot. `bind(s, v)` fixes the proof-surface value `v` for slot `s`; it does not contribute to the transcript on its own. `absorb_slot(s)` records that the bound slot contributes to the transcript at a particular stage. Initialization context and required public material — statement digests, public inputs, verifier parameters — enter the transcript through `absorb_meta`, distinct from `absorb_slot` because they are not proof-surface objects. Closure rejects any artifact that derives a challenge from a slot that has not been both bound and absorbed in prior-prefix order.

Prover and verifier follow the same schedule (slot identifiers are written `@name`); the verifier reads slot values from the proof rather than computing them:

```text
Prover side:
  tr0       = init(protocol_id)
  bind(@wire_comms, C_w)
  tr1       = absorb_meta(tr0, d_stmt)
  tr2       = absorb_slot(tr1, @wire_comms)
  tr3, beta = challenge(tr2, @beta)

Verifier side:
  tr0       = init(protocol_id)
  C_w       = read(@wire_comms)
  tr1       = absorb_meta(tr0, d_stmt)
  tr2       = absorb_slot(tr1, @wire_comms)
  tr3, beta = challenge(tr2, @beta)
```

The six obligations rule out concrete failure classes:

- `TraceOK` — an `absorb_slot` missing before a dependent `challenge`, or a slot whose prior-prefix list points to material absorbed later in `T`.
- `BindOK` — required public material never `absorb_meta`-ed before challenge derivation; the canonical [Frozen Heart 2022](#ref-frozenheart22) pattern across Dusk, Iden3, and pre-patched gnark.
- `SepOK` — a slot identifier or challenge label reused across composed components (e.g., two component instances under `par` both declaring `@witness_commitments` without an operand-disambiguating prefix).
- `ConstructOK` — a duplex sponge initialized without protocol-instance binding ([SAFE 2023](#ref-safe23) / [CFRG FS](#ref-cfrgfs) non-conformance), or a hash function used outside its specified mode.
- `TheoremOK` — a closed source declaring a special-sound multi-round profile whose actual round structure does not satisfy any theorem family routed by `Pi.fs.theta`.
- `BudgetOK` — an exported soundness claim at level `lambda` whose dispatched theorem yields a knowledge error larger than `Pi.fs.sigma`'s budget at `lambda`.

### Semantic and byte-level transcript layers

`T` is the semantic event trace; closure derives a byte-level transcript from it under `Pi.fs.kappa` and `B`. Let `T_sem ↾ i` denote the prefix of `T` up to position `i`, let `Abs` extract absorbable events (drop `Chal`), and let `Encode` realize the prefix at byte level under the construction profile:

```text
Abs(T_sem ↾ i)   = [e in T_sem ↾ i  |  e = Meta(label, x) or e = Slot(s)]
T_bytes(I, B; i) = Encode(Pi.fs.kappa, I, Abs(T_sem ↾ i), B)
c_j              = Sample(Pi.fs.kappa, chi_j, T_bytes(I, B; i_j))
```

where `I` is the initialization context, `chi_j` is the challenge class at position `i_j` (i.e., `e_{i_j} = Chal(chi_j)`), and `c_j` is the sampled challenge value.

Only `Meta` and `Slot` events contribute bytes; `Chal` events mark sampling points the verifier replays from the same byte-prefix. Operationally, `absorb_meta` and `absorb_slot` extend `T_sem`, which `Abs` then filters; `challenge` consumes `T_bytes(I, B; i_j)` to produce `c_j`. Changing `Pi.fs.kappa` or `B` changes `T_bytes` without changing `T`, so verifier and prover reconstruct the same `Abs(T_sem ↾ i)` even when their byte-level realization differs.

### Theorem dispatch and budget check

Every exported claim needs a property-specific route — completeness, zero knowledge, soundness, knowledge soundness (also Argument of Knowledge, AoK) — dispatched through `Pi.fs.theta` to a theorem row in Appendix C.1. `close` accepts the exported claim only when its bound fits the `Pi.fs.sigma` budget at target level `lambda`, or when `sigma` explicitly permits a conditional evidence route. Plain soundness and AoK are separate verifier-facing certificate tags. Appendix C.2 records theorem-family caveats (weak FS, naive repetition, holographic visibility, ROM/QROM scope).

---

## §7. Certificate and Preservation

`C` records what the closed artifact may claim after `close` succeeds. The certificate carries structural checks, theorem dispatch rows, exported claims that were discharged or accepted as explicitly conditional, and remaining rows preserved as non-claiming caveats.

The certificate surface is:

```text
C = (c_shape, c_tr, c_fs, c_zk, c_ver, c_abi)
c_ver = (c_complete, c_sound)
```

The fields separate structural evidence from theorem-bearing evidence:

| Field | Main role |
|---|---|
| `c_shape`, `c_tr` | composed shape, sealed schedule, visibility, and `obs(P_closed).T` evidence |
| `c_fs` | `Pi.fs` snapshot, construction witnesses, theorem dispatch, extractor/simulator metadata, and loss entries |
| `c_zk` | exported ZK claim when a supported ZK row applies |
| `c_ver` | verifier-facing completeness, plain soundness, and optional AoK conclusions |
| `c_abi` | proof/verifier ABI and backend serialization compatibility for the semantic slots in `Σ` |

Preservation is property-indexed, not one scalar epsilon relation:

```text
Loss : Property -> Bound
Loss_C[p] = theorem/profile-specific bound for property p
```

The active property set (completeness, zero knowledge, soundness, knowledge soundness / AoK, construction loss) and where each bound is stored are listed in Appendix D.1. `Pi.fs.sigma` records requested per-property objectives, the default target `lambda`, optional per-property budget overrides, and policy for conditional scheme rows. `BudgetOK` accepts an exported claim only when `Loss_C[p]` fits the requested budget for that property, or when policy explicitly permits a scheme-conditioned claim backed by named evidence in `C`.

The certificate may contain discharged rows, scheme-conditioned rows, and open rows at the same time, each with distinct claiming force.

| Status | Can transfer as a claim? | Evidence carried in `C` |
|---|---|---|
| discharged | yes | framework-checkable row and bound |
| scheme-conditioned | yes, conditionally | named scheme profile or external evidence reference accepted by `Pi.fs.sigma` |
| open | no | visible reason a stronger or unrequested claim remains unresolved |

Changed construction, backend, or ABI profiles reopen exactly the affected certificate entries.

---

## §8. Worked Witnesses

These examples calibrate the object chain against familiar protocol families. PLONK/KZG is the primary witness, because it is compact enough to show the whole chain. STARK/FRI, Sumcheck-shaped systems, and folding schemes are deltas against that witness.

### 8.1 Primary — PLONK with KZG

A PLONK/KZG-shaped source uses `link` to assemble gate checks, copy/permutation checks, and polynomial openings; lookup variants add matching slots and schedule events under a concrete lookup profile ([PLONK 2019](#ref-gwc19), [KZG 2010](#ref-kzg10), [Plookup 2020](#ref-plookup20)). This witness shows lifecycle, proof surface, challenge schedule, closure profile, and certificate attachment in one place.

Here `K_gate` records gate constraints, `K_copy` records permutation/copy constraints, and `K_opening` records polynomial opening checks. KZG supplies the polynomial commitment/opening layer, so the later slots specialize to commitments, evaluations, and a batched opening proof.

```text
P_core = link(K_gate, link(K_copy, K_opening))

Σ = [ Slot(witness_commitments),
      Slot(permutation_commitment),
      Slot(quotient_commitment),
      Slot(evaluations_at_zeta),
      Slot(batch_opening_proof) ]

T = [ Meta(protocol_id, circuit_hash, public_inputs, vk_or_srs_digest),
      Slot(witness_commitments), Chal(beta), Chal(gamma),
      Slot(permutation_commitment), Chal(alpha),
      Slot(quotient_commitment), Chal(zeta),
      Slot(evaluations_at_zeta), Chal(v),
      Slot(batch_opening_proof) ]

Pi.fs = (theta, sigma, m, nu, kappa)
rho   = { protocol_id, session_id, circuit_hash, public_inputs }
B     = ( B_kappa  = { sponge, codec, field_enc, sample },
          B_scheme = { pcs = KZG over a pairing-friendly curve, SRS source, t-SDH binding },
          B_abi    = { transcript_api, proof_codec, verifier_api, lib_version } )

close(P_core, Pi, rho, B) = Some(P_closed, C)
P_obs = obs(P_closed) = (Σ,T,V)
```

If a lookup component is present, its commitments, evaluations, and proof slots are inserted into `Σ`; its lookup-specific challenge sites and schedule events are inserted into `T` in the order required by that concrete lookup profile.

`V` checks the PLONK-shaped gate/copy quotient relation, the evaluations at `zeta`, and the KZG batch opening verification under the schedule `T`. The verifier reconstructs the same `T` from the same semantic events using the §6 transcript discipline, with `read` replacing the prover's `bind` on each proof slot; `absorb_slot` and `challenge` are identical on both sides. (Below, `tr = ...` is shorthand for §6's `tr_{i+1} = ...(tr_i, ...)` chaining, and pairing `read(@s)` with `absorb_slot(@s)` on the same line is presentational only.)

```text
Verifier side:
  tr  = init(protocol_id)
  tr  = absorb_meta(tr, circuit_hash, public_inputs, vk_or_srs_digest)
  C_W = read(@witness_commitments);    tr = absorb_slot(tr, @witness_commitments)
  tr, beta  = challenge(tr, @beta)
  tr, gamma = challenge(tr, @gamma)
  C_P = read(@permutation_commitment); tr = absorb_slot(tr, @permutation_commitment)
  tr, alpha = challenge(tr, @alpha)
  C_Q = read(@quotient_commitment);    tr = absorb_slot(tr, @quotient_commitment)
  tr, zeta  = challenge(tr, @zeta)
  E   = read(@evaluations_at_zeta);    tr = absorb_slot(tr, @evaluations_at_zeta)
  tr, v     = challenge(tr, @v)
  O   = read(@batch_opening_proof);    tr = absorb_slot(tr, @batch_opening_proof)

  assert KZGBatchVerify(C_W, C_P, C_Q, E, O, zeta, v, vk_or_srs_digest)
  assert PlonkRelationCheck(public_inputs, beta, gamma, alpha, zeta, E)
```

The certificate records the flattened PLONK-shaped source in `c_shape`, the semantic schedule in `c_tr`, FS profile and theorem dispatch in `c_fs`, verifier-facing completeness/soundness rows in `c_ver`, and proof/verifier encoding evidence in `c_abi`. `c_zk` records a ZK claim when a supported ZK theorem row is supplied.

An AFK-style row applies when the flattened source is presented in that theorem family and PCS binding/extractability rows are separately supplied ([AFK 2022](#ref-afk22)).

Scope note: `link` knowledge-soundness and lookup-claim lifting stay theorem-row dependent. KZG, Poseidon, and plookup choices remain profile choices. [Halo2](#ref-bgh19) moves to different profile branches when IPA, halo-style accumulation, or transcript choices change the protocol object.

Closure refuses deployments that omit the discipline. Drop `public_inputs` from the verifier-side `absorb_meta` before `Chal(@beta)`: `public_inputs` is in `rho` and required by `Pi.fs.nu` to be absorbed before any dependent challenge, yet absent from `Abs(T_sem ↾ i)` at the position of `@beta`. `BindOK` returns false; `close` returns `None`, localizing the violation to the missing absorb. This is the [Frozen Heart 2022](#ref-frozenheart22) pattern observed across Dusk, Iden3, and pre-patched gnark — the failure surfaces concretely at `zeta` derivation in Round 4, where the verifier's evaluation check depends on the unbound public input.

### 8.2 Cross-Family Deltas

The other witnesses still close into `P_closed` and expose `P_obs = (Σ,T,V)` through the same `close` / `obs` pattern. Each delta stresses a different object face: LDT proof material, Sumcheck rounds, or accumulator discipline.

| Witness | Open source pressure | Mainline calibration | Certificate pressure |
|---|---|---|---|
| STARK/FRI | `P_core = link(K_AIR, K_FRI)` | Merkle roots, query answers, paths, and final FRI material shape `Σ`; FRI folds and query positions shape `T` | proximity/LDT rows, IOP/round-by-round (RBR) compilation rows, and end-to-end STARK rows stay separately named |
| HyperPlonk / Spartan / Jolt line | `P_core = link(K_sumcheck, K_open)` | each `g_i` is proof material; each `r_i` is a `Chal`; PCS/opening material remains a separate face | Sumcheck special-sound rows, PCS binding/opening rows, and lookup-heavy zkVM rows stay separate |
| Nova / folding | `P_core = fold^n(K_step, A_scheme)` | accumulator state is visible before dependent fold challenges; `close` fires once after the folded source is assembled | fold preservation, recursive FS, and accumulation losses remain scheme-profile dependent |

Detailed traces, citations, and per-family evidence routes are recorded in Appendix A.1 (STARK/FRI), A.2 (Sumcheck line), and A.3 (Nova / folding). `par` and `repeat_t` still need witnesses with the same object faces and theorem accounting.

---

## §9. Positioning and Limits

### 9.1 Active Core and Surrounding Tracks

The **active core** is public-coin protocol sources, composition operators, Fiat-Shamir closure, observable verifier surface, and certificate accounting.

**Extension profiles** add later protocol or theorem extensions through `Pi`, `close`, and `C`. Non-FS NIZK compilation and BCS-style compile paths belong here when the required theorem row or profile is supplied. UC-facing interfaces can enter through profiles, while UC/concurrency reasoning remains a larger layer around the active object model ([Canetti 2020](#ref-canetti20)).

**Companion tracks** build implementation and proof-engineering layers around the protocol object: user-facing DSLs, MLIR dialects or carriers, lowering passes, backend APIs, runtime kernels, verifier packaging, direct backend realization, and mechanized proof artifacts ([SSProve 2023](#ref-ssprove23); [ArkLib](#ref-arklib) and [VCVio 2024](#ref-vcvio), which formalize Interactive Oracle Reductions and duplex-sponge Fiat-Shamir in Lean within the Verified-zkEVM project). In a compiler-infrastructure setting ([MLIR 2021](#ref-mlir21)), a circuit IR can carry relation payload below Protocol IR, a Protocol IR dialect can carry `P_core` and closure metadata, and backend dialects or libraries can realize `P_closed`. Theorem-bearing rows in `Pi.fs.theta` may cite obligations discharged in such mechanized companions.

The three layers connect through narrow interfaces.

- *Active core → Extension profile.* An extension contributes new dispatch rows to `Pi` (axes), to `close` (Appendix C-style admissibility rows), and to `C` (status entries). The framework checks well-formedness of the contributions — model tag, extractor or simulator discipline, side conditions, bound source — but does not validate the underlying theorem itself. Contributions that fail well-formedness are rejected at registration.
- *Active core → Companion track.* A companion consumes `P_closed` and `C` as a read-only contract: it may inspect `Σ`, `T`, `V`, and any certificate row, and it may emit code, carriers, or mechanized proofs whose behavior is constrained by those reads. A companion may not modify `P_closed`, the framework's obligations, or the rules under which `C` was issued.
- *Extension profile → Companion track.* A companion may reference rows added by extensions in `c_fs` dispatch — for example, to specialize verifier code for an extension-provided theorem family — but it consumes those rows on the same read-only basis as the active core's rows. Extensions and companions therefore communicate only through the certificate, never directly.

### 9.2 Open Edges

The main open edges are named rows or companion work:

- Generic modular knowledge-soundness for arbitrary `link` compositions remains an open theorem row; §5 records the structural rule and leaves the theorem lift named.
- Generic fold preservation remains scheme-profile dependent. Nova-family, ProtoStar/ProtoGalaxy-style, BCMS-style, and later folding rows stay distinct unless evidence connects them.
- `par` and `repeat_t` are structurally present; §8 leaves same-density witnesses for later work.
- Mechanized discharge, MLIR dialect design, lowering to backend APIs, and direct runtime realization continue as companion tracks.
- Validation: examples, carriers, and backend bindings must preserve the object faces (`Σ`, `T`, `Pi.fs`, `B`/`c_abi`, `C`) across lowering and verifier realization. A prototype MLIR carrier or backend binding is the natural early check.

Surrounding frameworks follow the same split: theorem rows feed closure, extension profiles attach new reasoning boundaries, and companion tracks build around the closed artifact.

---

## Appendix A. Detailed Worked Witnesses

This appendix records the longer delta traces behind §8.2.

### A.1 STARK with FRI

The STARK/FRI witness changes the component pressure. The open source links an AIR-facing component to an LDT/FRI component. Merkle commitments, query answers, and authentication paths become proof-surface material; FRI folding and query positions remain schedule events ([BBHR 2018](#ref-bbhr18)).

```text
P_core = link(K_AIR, K_FRI)

Σ = [ Slot(trace_root),
      Slot(composition_root),
      Slot(fri_round_root_1), ...,
      Slot(fri_round_root_R),
      Slot(final_fri_value_or_poly),
      Slot(query_answers),
      Slot(merkle_paths) ]

T = [ Meta(protocol_id, air_or_statement_digest, public_inputs, params, domain),
      Slot(trace_root), Chal(air_mix_alpha),
      Slot(composition_root),
      Chal(fri_fold_1), Slot(fri_round_root_1), ...,
      Chal(fri_fold_R), Slot(fri_round_root_R),
      Slot(final_fri_value_or_poly),
      Chal(query_positions),
      Slot(query_answers), Slot(merkle_paths) ]
```

`V` checks the AIR boundary/transition and composition constraints, verifies Merkle openings for query answers, and checks the FRI/proximity conditions under `T`.

The same `close` / `obs` pattern as §8.1 applies. `Pi.fs.theta` selects property-scoped theorem families; `C` records dispatch and status. `B_kappa` witnesses the hash, codec, and field/byte realization; `B_scheme` records the Merkle binding profile and any FRI-scheme-specific bindings; `B_abi` carries proof/verifier API conformance. Query positions are schedule/state by default; query answers, openings, Merkle paths, and final FRI material may be slots when the concrete profile serializes them.

Exported STARK soundness or AoK needs an end-to-end STARK row or the relevant RBR/state-restoration evidence ([BCS 2016](#ref-bcs16), [Chiesa-Yogev 2021](#ref-cy21), [Holmgren 2019](#ref-h19)). Each evidence route contributes separately:

- FRI/proximity rows contribute proximity evidence.
- BCS-style compilation rows contribute compilation evidence and side-condition checks.
- FRI-like families keep separate theorem rows unless a profile imports shared evidence.

Named partial scheme evidence is preserved as a scheme-conditioned row; missing evidence leaves the row open.

### A.2 Sumcheck Line

HyperPlonk and Spartan exercise the `Sumcheck` component more directly than PLONK/KZG ([HyperPlonk 2023](#ref-cbbz23), [Spartan 2020](#ref-setty20), [LFKN 1992](#ref-lfkn92)). The open source is usually a sum-over-domain or multilinear claim linked to a PCS or multilinear opening component. Jolt/Lasso adds adjacent lookup-heavy zkVM pressure when its theorem row is imported ([Lasso 2023](#ref-lasso23), [Jolt 2023](#ref-jolt23)).

```text
P_core = link(K_sumcheck, K_open)

Σ = [ Slot(poly_oracle_commitments),
      Slot(round_poly_g_1), ...,
      Slot(round_poly_g_v),
      Slot(final_eval_claim),
      Slot(opening_values),
      Slot(opening_proof) ]

T = [ Meta(protocol_id, statement_digest, public_inputs, params),
      Slot(poly_oracle_commitments),
      Slot(round_poly_g_1), Chal(r_1),
      ...,
      Slot(round_poly_g_v), Chal(r_v),
      Slot(final_eval_claim),
      Slot(opening_values),
      Slot(opening_proof) ]
```

`V` checks the Sumcheck round identities, derives the challenge sequence from `T`, checks the final evaluation claim, and verifies the linked opening material.

The same `close` / `obs` pattern applies; the delta is the round-by-round proof surface. Each `g_i` is proof material, each `r_i` is verifier randomness, and `close` replaces the `r_i` sites only after prior-prefix and construction checks. A native Sumcheck source may route to an AFK-style multi-round special-sound row with `k_i = deg(g_i)+1`, but only when the source profile and side conditions match. PCS binding, opening extractability, lookup-specific claims, and backend construction losses remain separate rows in `C`.

Sumcheck rounds map cleanly into `Σ/T`: the round polynomials are proof slots, and the verifier randomness sites are schedule events. SNARK knowledge soundness, `link` preservation, and lookup-heavy zkVM claims stay theorem-family or theorem-row dependent.

### A.3 Nova / Folding

The folding witness stresses all three pillars at once. Accumulator state and fold challenges must be visible before closure; `fold` builds the larger open source; and `close` either dispatches to a named scheme row or preserves a conditional row in `C` ([BCMS 2020](#ref-bcms20), [Nova 2022](#ref-nova22), [SuperNova 2022](#ref-supernova22), [HyperNova 2023](#ref-hypernova23), [CycleFold 2023](#ref-cyclefold23), [ProtoStar 2023](#ref-protostar23), [ProtoGalaxy 2023](#ref-protogalaxy23), [Mova 2024](#ref-mova24)).

```text
P_core = fold^n(K_step, A_scheme)

Σ = [ Slot(step_commitment_1), ...,
      Slot(step_commitment_n),
      Slot(persistent_accumulator_final),
      Slot(terminal_opening_or_decider_proof) ]

T = [ Meta(protocol_id, scheme, params_digest),
      Meta(instance = step_relation_digest, z0, n, public_io),
      Meta(accumulator_binding = CRH | FS-RO),
      Meta(accumulator_0),
      Slot(step_commitment_1), Chal(fold_challenge_1),
      ...,
      Meta(accumulator_{n-1}),
      Slot(step_commitment_n), Chal(fold_challenge_n),
      Slot(persistent_accumulator_final),
      Slot(terminal_opening_or_decider_proof) ]
```

`V` checks the fold transitions against the visible accumulator state, verifies the final accumulator or terminal decider proof, and enforces the scheme's accumulator-binding discipline under `T`.

The same lifecycle applies. `close` fires once after the `fold^n` source has been assembled. `c_shape` records the fold tree and scheme profile, `c_tr` records the step-indexed schedule, and `c_fs` records the selected folding row, accumulator binding discipline, construction evidence relevant to `Pi.fs`, and any scheme-conditioned loss entry. `c_abi` carries proof/verifier encoding evidence when needed. The accumulator may be proof material at one step and statement-side input to the next, often with a collision-resistant hash (CRH) binding discipline distinct from the FS transcript. A concrete profile may represent intermediate accumulators as `Meta` context or as `Slot` proof material, but they must be visible before any dependent fold challenge.

This shows the accounting surface around scheme-conditioned folding. Fold preservation, `repeat_t` composed with `fold`, cross-family folding, and recursive FS soundness need scheme-profile or theorem-row evidence. Nova-family rows, ProtoStar/ProtoGalaxy special-sound rows, and BCMS-style accumulation rows stay distinct unless a specific scheme profile imports the evidence. Mova may supply a variant-specific row when that evidence is imported.

## Appendix B. Composition Lift Rules

### B.1 `link`

`link` connects an output interface of one open source to an input interface of the next: commitments feed openings, low-degree claims feed queries, lookup checks feed a larger verifier predicate. The structural lift is straightforward when the handoff uses typed slots and the event schedule has no slot reuse.

**Interface contract.** `link(P_out, P_in)` is well-typed when three faces line up.

- *Producer face* of `P_out`: declares the output slot kinds it makes available to the consumer, the events it exports as a transcript prefix (the schedule the consumer's `T` must inherit), and its profile contributions to `Pi`, especially `Pi.fs` (codec, construction, visibility, and separation entries the consumer may rely on).
- *Consumer face* of `P_in`: declares the input slot kinds it requires (which producer outputs each input is matched against), the expected schedule prefix (which `Meta` and `Slot` events must already be in `T` before the consumer's first dependent `Chal` site), and its profile requirements on `Pi.fs` (entries the producer must already supply or the consumer itself must contribute).
- *Merge rule*: in the composed source, every consumer event appears in `T` after the producer events its prior-prefix lists (`TraceOK` applied at the link boundary). Slot identifiers from the two sources stay in disjoint namespaces by construction or by explicit prefix tags. Profile contributions are merged axis by axis on `Pi.fs = (theta, sigma, m, nu, kappa)`; unresolvable conflicts make `link` ill-typed.

Knowledge-soundness needs a theorem row because extractors live across time: rewinding one component may disturb the transcript prefix of the other, and knowledge-error functions require a theorem-specific composition rule. When a linked source needs a knowledge-soundness claim, the composed source is flattened and analyzed as one source under a named theorem family. Flattening treats the operator tree as one monolithic public-coin source for theorem dispatch while still recording the original tree in `c_shape`.

### B.2 `par`

`par` places sources side by side while keeping transcript slots, challenge namespaces, domain-separation tags, public-input namespaces, and oracle/resource assumptions separated by disjoint resources or explicit namespace/domain tags. A shared resource, such as one random oracle with separated domains, is allowed only when the shared-resource profile is supplied.

Under those separation conditions, `par` records the open structure that later induces combined proof slots and verifier obligations after closure. Quantitative bounds are property-specific: completeness, zero knowledge, verifier-facing soundness, and AoK/extractor losses may cite different theorem rows and budgets. Advantage addition or repeated zero-knowledge loss is citeable only when the selected theorem row accepts the separation profile.

### B.3 `fold`

`fold` is the state-carrying composition lane. A named scheme supplies the rule that combines instances and carries state forward. BCMS-style accumulation rows, Nova-family folding rows, and ProtoStar/ProtoGalaxy-style special-sound rows are related but distinct evidence routes ([BCMS 2020](#ref-bcms20), [Nova 2022](#ref-nova22), [ProtoStar 2023](#ref-protostar23), [ProtoGalaxy 2023](#ref-protogalaxy23)).

Protocol IR admits `fold` only with a named scheme profile. The operator can expose the repeated source shape and the persistent state surface; soundness, zero-knowledge, and closure claims stay scheme-profile dependent until the certificate cites the needed evidence. Multi-fold variants, recursive Fiat-Shamir, and cross-family folding add profile obligations rather than inheriting a generic preservation theorem.

### B.4 `repeat_t`

`repeat_t` duplicates a source under a repetition regime. Repetition changes the protocol surface: it creates repeated slots, repeated challenge sites, and domain-separation obligations for each copy. Public-coin special-sound and standard public-coin regimes dispatch to different theorem families and have different limits ([Attema-Fehr 2022](#ref-af22)). The active operand class covers public-coin regimes; negative results such as [BIN 1997](#ref-bin97) explain the generic private-coin hazard. The operator records the repeated structure, and the theorem family determines what, if anything, improves.

## Appendix C. FS Admissibility and Dispatch Rows

### C.1 Dispatch Rows

| Claim | Active-core precondition | Dispatch record |
|---|---|---|
| Completeness | Honest prover acceptance is preserved under the sealed challenge substitution and representation choices. | `FS.preserve_completeness` row or structural closure evidence, stored in `c_ver.c_complete`. |
| Zero knowledge | The source is in a supported public-coin honest-verifier ZK (HVZK) or special-HVZK family, the oracle/model tag matches a simulator row, and simulator plus strong-FS binding conditions discharge. | `FS.derive_ZK` row: theorem family, model tag, simulator discipline, oracle-programming assumptions, side conditions, property-indexed bound. |
| Soundness | `source_kind` and `Pi.fs.theta` match an interactive soundness-preservation, IOP/state-restoration, special-sound, or scheme-specific verifier-soundness row, and the FS binding/model side conditions hold. | `FS.derive_soundness` row: theorem family, model tag, verifier side conditions, ordering witness, property-indexed bound. |
| Knowledge soundness / AoK | `source_kind` and `Pi.fs.theta` match a special-sound, generalized special-sound, extractor-bearing IOP/state-restoration row if supplied, or scheme-specific extractor row. | `FS.derive_KS` row: theorem family, extractor discipline, oracle/model tag, ordering witness, property-indexed bound. |

### C.2 Theorem and Hazard Notes

- AFK-style and generalized special-sound rows are usable for their source classes when the round structure, challenge arity, extractor discipline, and loss formula match the selected theorem family ([AFK 2022](#ref-afk22), [Gamma-special-sound 2023](#ref-gammafs23), [Generalized special soundness 2023](#ref-gss23)).
- IOP/RBR and state-restoration rows are citeable only for profiles whose oracle access, restoration condition, transcript compilation, and model tag match the theorem statement.
- Weak Fiat-Shamir and missing public-input binding are construction failures: required public material must be bound before dependent challenges, and the construction profile must record the absorb/squeeze, domain-separation, codec, and challenge-extraction discipline ([Bernhard-Pereira-Warinschi 2012](#ref-bpw12), [Weak FS 2023](#ref-weakfs23)).
- Repetition rows are regime-specific. Public-coin special-sound repetition, standard public-coin repetition, and private-coin repetition hazards dispatch to different theorem families; naive repetition stays open without a matching row ([Holmgren-Lombardi-Rothblum 2021](#ref-hlr21)).
- Visibility discipline `Pi.fs.nu` is theorem-side load-bearing in holographic-IOP and rational-verification settings: when the verifier reads only a sublinear portion of the input via oracle access, FS admissibility shifts because the verifier's view of `Abs(T_sem ↾ i)` is partial. `nu` constrains `Abs` membership and prior-prefix order — which `Meta` and `Slot` events the verifier requires before a dependent challenge — not the byte-level `Sample` rule. Holographic-class theorem rows must match `nu` against verifier oracle access ([Fiat-Shamir Goes Rational 2024](#ref-fsrational24)).
- ROM-style FS rows are the active core. Programmable-ROM simulator rows, QROM, and global-RO modes are admissible only when an explicit theorem row supports them; the active core does not carry such rows. Exporting a quantum-soundness or QROM-specific claim requires importing the corresponding theorem row through an extension profile.

## Appendix D. Certificate Row Semantics

### D.1 Field Notes

- `c_shape` records the operator tree, source/component shape, and repetition or fold structure that reached `close`.
- `c_tr` records the sealed semantic schedule, slot/challenge visibility, public-input inclusion, prior-prefix witnesses, and the evidence from which `obs(P_closed).T` is read.
- `c_fs` records the `Pi.fs` snapshot, domain separation, construction witnesses, oracle/model labels, per-claim theorem-row dispatch, extractor or simulator metadata, ordering witnesses, and property-indexed loss entries.
- `c_zk` records a zero-knowledge claim only when such a claim is requested and a supported ZK theorem row applies.
- `c_ver` records verifier-facing claims: `c_complete` for completeness preservation, and `c_sound` as tagged entries such as `plain_soundness` and optional `aok`.
- `c_abi` records proof/verifier ABI stability and backend-facing serialization or codec compatibility. It refines concrete byte/API compatibility for the semantic slots in `Σ`; the observational face remains defined by `obs`.

Per-property `Loss[p]` is sourced and stored as follows:

| Property | Bound source | Stored in |
|---|---|---|
| completeness | structural closure evidence or theorem/profile-specific row | `c_ver.c_complete` |
| zero knowledge | theorem/profile-specific simulator row | `c_zk`, with dispatch metadata in `c_fs` |
| soundness | theorem/profile-specific verifier row, such as `FS.derive_soundness` | `c_ver.c_sound.plain_soundness`, with dispatch metadata in `c_fs` |
| knowledge soundness / extractor behavior | theorem/profile-specific extractor row | `c_fs`; verifier-facing AoK conclusion, if exported, in `c_ver.c_sound.aok` |
| construction loss | codec, hash/sponge, extraction, and output-distribution profile | `c_fs` and `c_abi` |

### D.2 Comparison and Budget Notes

A reader uses `C` to compare artifacts without re-reading backend code. If two artifacts differ only in codec choices, the difference should appear in the ABI or construction evidence. If they differ in theorem family, simulator discipline, extractor discipline, or security budget, the difference should appear in the theorem-bearing fields.

### D.3 Status and Reopening Semantics

- `discharged` rows carry framework-checkable evidence and a bound accepted by the active budget.
- `scheme-conditioned` rows transfer only under the named scheme profile or external evidence reference accepted by `Pi.fs.sigma`.
- `open` rows are visible non-claims. They may record stronger, unrequested, or currently unsupported obligations, but they do not transfer as exported artifact claims.
- Conditional evidence policy lives in `Pi.fs.sigma`; the row in `C` records which condition was accepted and which property budget it affects.
- Construction, backend, or ABI changes reopen the rows whose evidence depends on the changed material. A PCS swap can reopen binding/extractability and ABI rows; a transcript construction swap can reopen `c_fs` construction rows; a proof encoding change can reopen `c_abi` and any verifier row that depends on the encoding.

## Appendix E. Notation

Symbols introduced across §2–§7 and used throughout the document, with role and first-defined section.

| Symbol | Role | Defined |
|---|---|---|
| `P_core` | Open protocol source — typed public-coin interactive source presented through component interfaces | §2, §4 |
| `P_closed` | Closed non-interactive artifact produced by `close` | §2, §6 |
| `P_obs` | Verifier face read from `P_closed` by `obs`; equals `(Σ, T, V)` under the sealed materialization | §2 |
| `Σ` | Proof surface — value-level proof material the verifier reads | §2 |
| `T` | Semantic event trace — schedule over `Meta` / `Slot` / `Chal` | §2 |
| `V` | Verifier acceptance relation over `T` and public input under the sealed materialization | §2 |
| `Pi` | Accumulated protocol profile referenced by `close` | §2, §6 |
| `Pi.fs = (theta, sigma, m, nu, kappa)` | Fiat-Shamir profile axes — theorem dispatch / budget / oracle model / visibility / construction | §6 |
| `rho` | Closure-time relation/session inputs (protocol id, statement digest, public inputs, parameters) | §6 |
| `B = (B_kappa, B_scheme, B_abi)` | Backend realization witness, closure input only; `B_kappa` realizes `Pi.fs.kappa` (sponge, codec, field encoding, sample rule), `B_scheme` records scheme-component bindings (e.g., PCS curve/SRS/assumption), `B_abi` records transcript/proof/verifier API conformance | §6 |
| `C = (c_shape, c_tr, c_fs, c_zk, c_ver, c_abi)` | Certificate attached to `P_closed` | §7 |
| `c_shape`, `c_tr` | Composed source shape; sealed semantic schedule | §7 |
| `c_fs` | FS profile and theorem dispatch evidence | §7 |
| `c_zk` | Zero-knowledge claim row, when an exported ZK theorem applies | §7 |
| `c_ver = (c_complete, c_sound)` | Verifier-facing completeness and soundness rows | §7 |
| `c_sound = (plain_soundness, aok)` | Plain soundness and argument-of-knowledge routes | §7 |
| `c_abi` | Proof/verifier ABI evidence | §7 |
| `close : P_core × Pi × rho × B → Option(P_closed × C)` | Closure — partial; succeeds only when `FSAdmissible` discharges | §6 |
| `obs : P_closed → P_obs` | Observation — reads `(Σ, T, V)` from `P_closed` | §2 |
| `FSAdmissible` | Closure judgment over six obligations: `TraceOK` / `BindOK` / `SepOK` / `ConstructOK` / `TheoremOK` / `BudgetOK` | §6 |
| `Loss : Property → Bound` | Per-property loss assignment used by `BudgetOK` | §7 |
| `bind`, `absorb_meta`, `absorb_slot`, `challenge`, `read` | Transcript discipline operations (prover-side `bind` becomes verifier-side `read`; the rest are identical on both sides) | §6 |
| `Abs`, `T_bytes`, `Sample` | Semantic-to-byte transcript layers — semantic projection, byte materialization, challenge-sampling rule | §6 |
| `PCS`, `LDT`, `Lookup`, `Sumcheck`, `FS` | Active component families in `P_core` | §4 |
| `link`, `par`, `fold`, `repeat_t` | Composition operators on `P_core` | §5 |

## Appendix F. References and Role Notes

### Composition foundations

<a id="ref-ssprove23"></a>**[SSProve 2023]** Abate et al. *SSProve: A Foundational Framework for Modular Cryptographic Proofs in Coq.* TOPLAS 2023.

<a id="ref-canetti20"></a>**[Canetti 2020]** Canetti. *Universally Composable Security.* JACM 67(5), 2020.

<a id="ref-arklib"></a>**[ArkLib]** Verified-zkEVM project. *Formally Verified Arguments of Knowledge in Lean.* In-development Lean 4 / Mathlib formalization of Interactive Oracle Reductions, the interactive BCS transformation, and duplex-sponge Fiat-Shamir; active formalizations target Sum-Check, Spartan, FRI, STIR/WHIR, and Binius. https://github.com/Verified-zkEVM/ArkLib

<a id="ref-vcvio"></a>**[VCVio 2024]** Tuma, Hopper. *VCVio: A Formally Verified Forking Lemma and Fiat-Shamir Transform, via a Flexible and Expressive Oracle Representation.* ePrint 2024/1819. Lean library for oracle-computation monads, the forking lemma, and Fiat-Shamir for sigma protocols; main dependency of ArkLib in the Verified-zkEVM stack. https://github.com/dtumad/VCV-io

### IOPs and Fiat–Shamir foundations

<a id="ref-bcs16"></a>**[BCS 2016]** Ben-Sasson, Chiesa, Spooner. *Interactive Oracle Proofs.* TCC 2016.

<a id="ref-marlin19"></a>**[Marlin 2019]** Chiesa, Hu, Maller, Mishra, Vesely, Ward. *Marlin: Preprocessing zkSNARKs with Universal and Updatable SRS.* EUROCRYPT 2020 (ePrint 2019/1047). AHP-based preprocessing SNARK reference.

<a id="ref-piop23"></a>**[PIOP modular compilation 2023]** Kohlweiss, Pancholi, Takahashi. *How to Compile Polynomial IOP into Simulation-Extractable SNARKs: A Modular Approach.* TCC 2023 (ePrint 2023/1067).

<a id="ref-cy21"></a>**[Chiesa–Yogev 2021]** Chiesa, Yogev. *Subquadratic SNARGs in the Random Oracle Model.* CRYPTO 2021. RBR/state-restoration evidence for ROM compilation.

<a id="ref-h19"></a>**[Holmgren 2019]** Holmgren. *On Round-By-Round Soundness and State Restoration Attacks.* ePrint 2019/1261. Round-by-round soundness and state-restoration reference.

<a id="ref-afk22"></a>**[AFK 2022]** Attema, Fehr, Klooß. *Fiat–Shamir Transformation of Multi-Round Interactive Proofs.* TCC 2022 (ePrint 2021/1377).

<a id="ref-gammafs23"></a>**[Gamma-special-sound FS 2023]** Attema, Fehr, Klooß, Resch. *The Fiat–Shamir Transformation of $(\Gamma_1,\dots,\Gamma_\mu)$-Special-Sound Interactive Proofs.* Journal of Cryptology 2026 (ePrint 2023/1945).

<a id="ref-gss23"></a>**[Generalized special soundness 2023]** Attema, Fehr, Resch. *Generalized Special-Sound Interactive Proofs and their Knowledge Soundness.* TCC 2023 (ePrint 2023/818).

<a id="ref-bpw12"></a>**[Bernhard–Pereira–Warinschi 2012]** Bernhard, Pereira, Warinschi. *How Not to Prove Yourself: Pitfalls of the Fiat–Shamir Heuristic and Applications to Helios.* Asiacrypt 2012 (ePrint 2016/771). Weak/strong Fiat-Shamir reference.

<a id="ref-hlr21"></a>**[Holmgren–Lombardi–Rothblum 2021]** Holmgren, Lombardi, Rothblum. *Fiat–Shamir via List-Recoverable Codes (or: Parallel Repetition of GMW is Not Zero-Knowledge).* STOC 2021 (ePrint 2021/286). Exhibits a 3-round Σ-style interactive proof whose parallel repetition fails to be zero-knowledge under LWE.

<a id="ref-weakfs23"></a>**[Weak FS 2023]** Dao, Miller, Wright, Grubbs. *Weak Fiat-Shamir Attacks on Modern Proof Systems.* IEEE S&P 2023 (ePrint 2023/691).

<a id="ref-safe23"></a>**[SAFE 2023]** Aumasson, Khovratovich, Mennink, Porçu Quine. *SAFE: Sponge API for Field Elements.* ePrint 2023/522. Sponge API and transcript-pattern binding reference.

<a id="ref-cfrgfs"></a>**[CFRG FS]** Orrù. *Fiat-Shamir Transformation.* IRTF/CFRG Internet-Draft, draft-irtf-cfrg-fiat-shamir-02, March 2026. Duplex-sponge Fiat-Shamir specification.

<a id="ref-co25"></a>**[Chiesa-Orru 2025]** Chiesa, Orrù. *A Fiat-Shamir Transformation From Duplex Sponges.* ePrint 2025/536.

<a id="ref-fsrational24"></a>**[Fiat-Shamir Goes Rational 2024]** Campanelli, Datta. *Fiat-Shamir Goes Rational (Or: On the Perils of Sublinear Verification).* ePrint 2024/1645. Studies Fiat-Shamir in the sublinear-verification setting; gives general impossibility results and sufficient conditions under which FS yields insecure non-interactive protocols, covering rational, holographic, and proximity arguments.

### Parallel repetition and knowledge-soundness amplification

<a id="ref-af22"></a>**[Attema–Fehr 2022]** Attema, Fehr. *Parallel Repetition of (k₁,…,k_μ)-Special-Sound Multi-Round Interactive Proofs.* CRYPTO 2022 (ePrint 2021/1259).

<a id="ref-bin97"></a>**[BIN 1997]** Bellare, Impagliazzo, Naor. *Does Parallel Repetition Lower the Error in Computationally Sound Protocols?* FOCS 1997.

### Accumulation / folding schemes

These entries supply folding, accumulation, and recursive-composition scheme references used by §5, §8, and Appendix B.

<a id="ref-bcms20"></a>**[BCMS 2020]** Bünz, Chiesa, Mishra, Spooner. *Proof-Carrying Data from Accumulation Schemes.* TCC 2020 (ePrint 2020/499).

<a id="ref-protostar23"></a>**[ProtoStar 2023]** Bünz, Chen. *ProtoStar.* Asiacrypt 2023 (ePrint 2023/620).

<a id="ref-protogalaxy23"></a>**[ProtoGalaxy 2023]** Eagen, Gabizon. *ProtoGalaxy.* ePrint 2023/1106.

<a id="ref-nova22"></a>**[Nova 2022]** Kothapalli, Setty, Tzialla. *Nova: Recursive Zero-Knowledge Arguments from Folding Schemes.* CRYPTO 2022 (ePrint 2021/370).

<a id="ref-hypernova23"></a>**[HyperNova 2023]** Kothapalli, Setty. *HyperNova: Recursive Arguments for Customizable Constraint Systems.* CRYPTO 2024 (ePrint 2023/573).

<a id="ref-mova24"></a>**[Mova 2024]** Dimitriou, Garreta, Manzur, Vlasov. *Mova: Nova Folding Without Committing to Error Terms.* ePrint 2024/1220.

<a id="ref-cyclefold23"></a>**[CycleFold 2023]** Kothapalli, Setty. *CycleFold: Folding-Scheme-Based Recursive Arguments Over a Cycle of Elliptic Curves.* ePrint 2023/1192.

<a id="ref-supernova22"></a>**[SuperNova 2022]** Kothapalli, Setty. *SuperNova: Proving Universal Machine Executions Without Universal Circuits.* ePrint 2022/1758.

### Component foundations

<a id="ref-kzg10"></a>**[KZG 2010]** Kate, Zaverucha, Goldberg. *Constant-Size Commitments to Polynomials and Their Applications.* Asiacrypt 2010.

<a id="ref-plookup20"></a>**[Plookup 2020]** Gabizon, Williamson. *plookup: A simplified polynomial protocol for lookup tables.* ePrint 2020/315.

<a id="ref-lfkn92"></a>**[LFKN 1992]** Lund, Fortnow, Karloff, Nisan. *Algebraic Methods for Interactive Proof Systems.* JACM 39(4), 1992.

### Worked example schemes (§8)

<a id="ref-gwc19"></a>**[PLONK 2019]** Gabizon, Williamson, Ciobotaru. *PLONK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge.* ePrint 2019/953.

<a id="ref-bbhr18"></a>**[Ben-Sasson et al. 2018]** Ben-Sasson, Bentov, Horesh, Riabzev. *Scalable, transparent, and post-quantum secure computational integrity.* ePrint 2018/046. The foundational STARK construction; FRI is introduced as the low-degree test.

<a id="ref-cbbz23"></a>**[HyperPlonk 2023]** Chen, Bünz, Boneh, Zhang. *HyperPlonk: Plonk with Linear-Time Prover and High-Degree Custom Gates.* EUROCRYPT 2023 (ePrint 2022/1355).

<a id="ref-setty20"></a>**[Spartan 2020]** Setty. *Spartan: Efficient and general-purpose zkSNARKs without trusted setup.* CRYPTO 2020 (ePrint 2019/550).

<a id="ref-lasso23"></a>**[Lasso 2023]** Setty, Thaler, Wahby. *Unlocking the lookup singularity with Lasso.* ePrint 2023/1216.

<a id="ref-jolt23"></a>**[Jolt 2023]** Arun, Setty, Thaler. *Jolt: SNARKs for Virtual Machines via Lookups.* ePrint 2023/1217.

<a id="ref-bgh19"></a>**[Halo 2019]** Bowe, Grigg, Hopwood. *Recursive Proof Composition without a Trusted Setup.* ePrint 2019/1021. Recursive proof composition and Halo-style accumulation reference.

### Compiler foundations

<a id="ref-mlir21"></a>**[MLIR 2021]** Lattner et al. *MLIR: Scaling Compiler Infrastructure for Domain Specific Computation.* CGO 2021.

### Empirical / applied

<a id="ref-chaliasos24"></a>**[Chaliasos et al. 2024]** Chaliasos et al. *SoK: What Don't We Know? Understanding Security Vulnerabilities in SNARKs.* USENIX Security 2024.

<a id="ref-nguyen24"></a>**[Nguyen et al. 2024]** Nguyen et al. *Fiat–Shamir in the Wild.* ePrint 2024/1565.

<a id="ref-frozenheart22"></a>**[Frozen Heart 2022]** Trail of Bits. *Coordinated Disclosure of Vulnerabilities Affecting Girault's PoK, Bulletproofs, and PLONK.* 2022.
