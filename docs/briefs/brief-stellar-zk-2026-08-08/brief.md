---
title: "Product Brief: stellar-zk"
status: draft
created: 2026-08-08
updated: 2026-08-09
---

# Product Brief: stellar-zk

## Executive Summary

stellar-zk is an open-source CLI that takes a ZK circuit (Circom, Noir, or Rust/RISC Zero) through the full lifecycle — init, build, prove, deploy, call, estimate — and produces a Soroban verifier contract, without the builder hand-rolling BN254 point serialization, verification-key layout, or nullifier plumbing. Five crates are already published on crates.io, and the pipeline is designed to work end-to-end for all three backends (Groth16/Circom, UltraHonk/Noir, RISC Zero) — with one currently-unmerged fix standing between that design and reality today (see [Current State](#current-state)).

Two things changed this week that sharpen what this product should become. First, a verified compile bug (all three generated verifier contracts called a `Bn254` API that never existed in any real `soroban-sdk` release) is fixed and evidenced — see [Current State](#current-state). Second, and more strategically: research grounded in SDF's own public statements confirms that Stellar's protocol team has explicitly and permanently ruled out building native privacy at the base layer — privacy is deliberately, by stated design philosophy, an application-layer, ecosystem-owned problem. That is not a gap stellar-zk is filling opportunistically; it is the space SDF has publicly committed to leaving open for third parties, and is itself partnering with companies (Nethermind, OpenZeppelin) to seed.

stellar-zk is positioned to be Stellar's ZK/privacy tooling layer: the thing a team reaches for instead of hand-building a proof pipeline the way Nethermind, an indie Proof-of-Reserve project, and dozens of abandoned hackathon repos have each done independently.

## Current State

- **Shipped**: 5 crates on crates.io, MIT, CLI with 6 subcommands, 3 working backends, CI (build/test/clippy/fmt) on the workspace.
- **Critical fix in flight**: the contract templates called a `Bn254` API shape that never shipped in any `soroban-sdk` version. Root cause: `soroban-sdk = "23.4"` predates the `bn254` module (introduced in 25.0.0), and CI never compiled the *generated* contracts — only the 5 workspace crates — so the break was invisible. Fixed and verified via a real `cargo build` of all three scaffolded backends, pending review and merge (see `docs/briefs/brief-stellar-zk-2026-08-08/addendum.md` for the full diff summary). **This blocks everything else below** — no roadmap work should ship on top of an unmerged core fix.
- **CAP-0074 (BN254 host functions) and CAP-0075 (Poseidon/Poseidon2)**: both `Status: Final`, Protocol 25. CAP-0080 (additional BN254/MSM primitives) is `Status: Implemented`, Protocol 26. The primitives this product depends on are live, not speculative — confirmed by direct CAP fetch, not inferred from documentation that may lag reality.

## The Problem

Every team that wants a privacy-preserving or ZK-verified Soroban contract today hand-rolls the same pipeline from scratch: circuit → proof → byte-serialize points into the exact layout the host functions expect → contract with pairing check → nullifier/replay logic → deploy. Evidence this is real, not assumed:

- **Nethermind's `stellar-private-payments`** (55 GitHub stars, active) built exactly this pipeline by hand for one vertical use case (private payments) — Groth16, Circom, BN254 — with no reusable artifact another team could `cargo install` for their own circuit. Nethermind is separately named in SDF's privacy strategy as a partner on "zkVM verifiers" — that partnership isn't confirmed to be this specific repo's origin, so treat the two as related evidence of SDF's ecosystem-build stance, not as one fact.
- **An indie Proof-of-Reserve project** (`zkpos.vercel.app`) independently built the same category of pipeline — Circom + Groth16 + BN254, deployed to Stellar testnet — unlisted, no visible team or funding, i.e. someone solved this alone rather than finding existing tooling.
- **Dozens of abandoned hackathon repos** (`zk-battleship`, `zk-poker`, `zk-rps-battle-royale`, and ~15 similar) on Stellar, almost all single-commit and dead, are the residue of builders who wanted ZK-verified game logic on Soroban and hit the same wall without a DevKit to lower it.

## The Solution

stellar-zk generalizes that pipeline into installable tooling: a trait-based backend system (`ZkBackend`) so Groth16, UltraHonk, and RISC Zero share one CLI surface, Handlebars-templated contracts that already implement the correct BN254 host-function calls and nullifier tracking, and three cost-estimation tiers so builders know what a proof will cost on-chain before they deploy it.

## What Makes This Different

Be precise about what the moat actually is, since none of this is a defensible technical secret on its own:

- **Nobody else in the Stellar ecosystem ships this as reusable tooling.** The competitive-landscape pass — a keyword search (zk, groth16, circom, noir, risc0, snark, proof, verifier, privacy) over the LumenLoop ~728-project DB, not an exhaustive audit — found zero SCF-funded projects unifying multiple ZK backends into an installable CLI. Every funded player it did surface (Moonlight, Alterscope, Fairblock, Zarf, Nethermind) is a vertical application sitting on top of a hand-built version of what stellar-zk automates. This is a first-mover-on-execution advantage, not a structural one — it holds only as long as stellar-zk stays correct and maintained (which is precisely what this week's bug undercuts if left unfixed).
- **The gap is structurally durable under SDF's current stated strategy.** SDF's Chief Product Officer, on the record (Meridian 2025 keynote, published on stellar.org, 2025-09-16): *"privacy will be opt-in and configurable at the application layer... privacy on Stellar will be ecosystem-driven and community-owned."* No CAP for confidential transactions, shielded assets, or stealth addresses exists across all 99 published CAPs (SEPs were not exhaustively checked). SDF ships primitives (CAP-0074/0075/0080) and explicitly invites third parties to build the layer stellar-zk occupies — this isn't a window that closes when the protocol "catches up," though it depends on SDF's stated strategy holding, not a permanent architectural guarantee.
- **A working generic UltraHonk verifier would be a real technical moat**, not just an execution one — Soroban has no tool that auto-generates a verifier contract per-circuit, so `stellar-zk`'s hand-built KZG-style verifier is currently the only attempt at this in the ecosystem. This is not yet proven at production hardness — flag it as the differentiator worth investing in, not one already banked.
- **What this is *not***: a claim of "we're the only multi-backend CLI" in the abstract. That framing is unfalsifiable and forgettable. The evidence-backed version — named competitors, named gaps, a named SDF strategy citation — is what should travel in any external-facing version of this document.

## Who This Serves

- **Primary**: teams building a privacy-preserving or ZK-verified Soroban application who would otherwise hand-roll the circuit-to-contract pipeline — the same position Nethermind and the indie PoR builder were in. This is the horizontal-DX audience.
- **Secondary, and the source of the pivot this brief formalizes**: builders in specific verticals where a *template*, not just a CLI, removes most of the remaining work. Two are backed by real evidence gathered this session (see Scope below); this is where "privacy services provider, ZK-focused, Stellar-only" as a positioning frame earns its keep — not as a rebrand, but as a small set of flagship, evidence-grounded verticals shipped on top of the existing generic pipeline.
- `[ASSUMPTION]` Institutional/RWA teams are called out as a specific sub-audience for the Proof-of-Reserve vertical given your own stated interest in that space — correct this if the intended audience is broader (e.g. any stablecoin issuer) or narrower.

## Scope

**In scope, evidence-ranked by how unblocked each one is today:**

1. **Merge the Bn254 fix.** Not a feature — a precondition. Nothing else in this scope matters if the core pipeline doesn't compile.
2. **A private-voting / anonymous-signaling circuit template** (Semaphore-style: prove group membership + nullifier against double-signaling). This is the strongest candidate found: it reuses stellar-zk's *existing* Groth16 pipeline and nullifier infrastructure almost unchanged, and follows a proof-of-membership + nullifier pattern that has already been proven out end-to-end elsewhere using the same snarkjs-based toolchain stellar-zk already shells out to. Zero SCF-funded Stellar competitors offer this as reusable tooling today (same search-methodology caveat as above). Poseidon (CAP-0075) is confirmed `Status: Final` at the protocol level, which should make cheap in-circuit-friendly hashing available rather than a root-comparison workaround — but unlike Bn254 this session, nobody has yet verified the actual `soroban-sdk` API surface for it the way the Bn254 fix required; confirm the real method signatures before scoping this as "easy," so it doesn't repeat this week's mistake.
3. **A Proof-of-Reserve circuit template** (Merkle-sum-tree + range-checked leaves + total-reserves-≥-total-liabilities, following the pattern used by Binance/OKX/Summa's open-sourced implementations). Real, current demand signal (GENIUS Act reporting requirements, the Zondacrypto collapse exposing the gap between periodic attestation and continuous cryptographic proof) and a direct match to your own RWA/institutional-DeFi focus. One existing indie implementation on Stellar testnet means this is validated as buildable on this exact stack, not speculative.

**Deliberately out of scope for stellar-zk itself:**

- **Full vertical privacy applications** (shielded pools with off-chain relayers, AMMs, browser wallets — the Nethermind-private-payments shape: private prediction markets, shielded transfer apps, and similar). These need infrastructure (relayer services, frontends) beyond a local CLI's reach. stellar-zk should be the thing such projects are built *on*, not attempt to become one itself.
- **A generic UltraHonk verifier**, in this scope — real and valuable, but a harder lift than the two templates above and not blocking them. Sequence it after, once demand for a Noir-based vertical (e.g. a provably-fair shuffle/deal verification template) makes the investment concrete rather than speculative.

## Business Model

The CLI and templates stay MIT and free — that's the adoption hook, not the revenue. Revenue comes from professional services sold to teams who need the pipeline working in production rather than just available: paid implementation/integration engagements (taking a client's circuit to a deployed verifier contract), production-readiness/audit reviews (the exact class of work this week's Bn254 fix already demonstrated), and a protocol-drift retainer (tracking `soroban-sdk` API changes like the one that caused this week's bug, so a client's deployed contract doesn't silently break). Paying customers are institutional/RWA teams needing Proof-of-Reserve in production and governance/DAO teams needing private voting in production — not the hobbyist/hackathon users the free CLI mostly reaches, who function as the funnel rather than the customer.

Two risks specific to this model, named rather than hidden: a two-person services business has a hard revenue ceiling (billable hours), so services revenue should be treated as either a deliberate boutique-expertise choice or a bridge to productizing whatever service repeats (e.g., the protocol-drift retainer, if sold more than once, is a signal to build a monitoring product instead of re-selling the same manual service); and because the core is MIT, a better-resourced competitor can legally offer the identical service on top of the same free tooling — the defense is speed and first-hand knowledge of the tool's own failure modes, not exclusivity. `[ASSUMPTION]` No pricing is proposed here — no comparable was found in this session's research, and the recommended next step is validating the first service (paid implementation) against one real prospect before pricing the rest. Full analysis: `addendum.md` §F.

## Success Criteria

- **Correctness (immediate)**: the new `scaffold-verify` CI job stays green — a scaffolded contract for all three backends compiles on every PR, closing the exact blind spot that let this week's bug ship silently.
- **Adoption**: `[ASSUMPTION — needs a number you're comfortable with]` external projects depending on the published crates, or verifier contracts deployed via `stellar-zk deploy` on testnet/mainnet, tracked from a starting point of effectively zero today.
- **Ecosystem validation**: the voting or PoR template referenced, forked, or built upon by a project outside your own team — the same kind of validation Nethermind's private-payments work already got from SDF's public blog.

## Vision

If this works, stellar-zk becomes the default first stop for any team building privacy or ZK-verified logic on Stellar — not because native privacy is guaranteed to never arrive, but because SDF's current, publicly stated strategy commits to leaving that layer to the ecosystem, and reaching for working tooling beats re-deriving the BN254 serialization layout from scratch, the way every project surveyed this session independently had to.

---

*Supporting detail — the full competitive-landscape table, CAP citations, Proof-of-Reserve scheme comparison, the service feasibility analysis, and the full business-model deep dive — lives in `addendum.md` in this folder.*
