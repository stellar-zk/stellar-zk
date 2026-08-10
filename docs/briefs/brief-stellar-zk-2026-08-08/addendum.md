# Addendum — stellar-zk Product Brief

Supporting depth gathered during this brief's Discovery. Referenced from `brief.md`; not required reading for someone who only needs the positioning.

## A. Competitive landscape (full table)

Sourced from the LumenLoop ~728-project Stellar ecosystem DB (SCF-funding-ranked) and an Electric Capital mining pass (~9,000 Stellar-tagged repos) for projects not yet in LumenLoop.

| Project | Category | SCF (total, rounds) | GitHub activity | Description | Overlap with stellar-zk |
|---|---|---|---|---|---|
| Reclaim | Infra & Services | $166,000 (21, 36) | active | zkTLS: ZK proofs of HTTPS sessions for verifiable credentials | None — vertical identity app |
| Fairblock | Developer Tooling | $150,000 (40) | active | Confidential-tx infra via threshold encryption | Low — different technique (threshold encryption, not SNARKs) |
| Sanctum | Developer Tooling | $135,000 (23, 25) | `zkbricks/sanctum`, `zkbricks/mpc-zexe` | ZK+MPC for compute on secret state | Medium — same category, different technique (MPC, not proof-system orchestration) |
| Alterscope | Infra & Services | $137,600 (24, 26) | `solity-network` | ZK proofs for real-time risk monitoring | None — RWA/risk vertical |
| Moonlight | Infra & Services | $134,990 (37) | `moonlight-protocol` | Non-custodial confidential transactions | None — vertical privacy app; named by SDF's own privacy strategy post as an ecosystem example |
| Boundless | Infra & Services | $110,000 (40) | `boundless-xyz`, active | Universal verifiable-compute layer on RISC Zero | Medium — shares a backend, but is a compute marketplace, not a local multi-backend DevKit |
| Nethermind — `stellar-private-payments` | Infra & Services | not SCF-listed via LumenLoop | 55★, pushed within the last 2 days at time of research, WIP | Groth16+Circom privacy pool with ASP compliance controls | High technical overlap (Groth16/Circom/BN254/Soroban) but a hand-built vertical app, not reusable tooling — and per SDF's own blog, built in partnership with SDF specifically to demonstrate this pattern |
| Soroban-ZK-Std (Neslabs) | not in LumenLoop; found via Electric Capital | none | 1★, pushed 2 days before research (very new) | `no_std` Rust crypto-primitives library for BN254/Poseidon (CAP-0074/0075) | High technical adjacency, low product overlap — a primitives library one layer below stellar-zk's CLI/DX layer; potential future dependency rather than competitor |
| Noir (Aztec Labs) | Developer Tooling | none | active | The language stellar-zk's UltraHonk backend already targets | Not a competitor — a dependency |

**Signal, not competition**: ~15-20 abandoned single-commit Stellar hackathon repos (`zk-battleship`, `zk-poker`, `zk-rps-battle-royale`, `zkash`, `minesweeperzk`, etc.) found via Electric Capital, almost all dead after one commit — strong evidence of unmet demand for a DevKit that removes the ZK-circuit-to-Soroban-contract plumbing, not evidence of competition.

## B. SDF's public privacy strategy — full citations

- **Primary source**: [stellar.org/blog/ecosystem/strategy-for-privacy-on-blockchain](https://stellar.org/blog/ecosystem/strategy-for-privacy-on-blockchain), Tomer Weller (SDF Chief Product Officer), dated 2025-09-16, tied to a Meridian 2025 (Rio) keynote.
  - *"Other chains treat privacy as all-or-nothing. Stellar is taking a different path: privacy will be opt-in and configurable at the application layer, like any other asset-specific feature."*
  - *"Our privacy strategy is guided by the principles that blockchains should be open and transparent by default, while privacy should be configurable and compliance-ready from the start."*
  - *"In the long-run, privacy on Stellar will be ecosystem-driven and community-owned."*
  - *"Stellar is investing in core privacy infrastructure by partnering with organizations like Nethermind to add zkVM verifiers... These efforts aim to make privacy primitives as easy to use as any other building block on the network."*
  - Names Moonlight, Amon Privacy, and human.tech as examples of "Stellar's ecosystem-driven approach to privacy."
- **Supporting**: [stellar.org/blog/developers/announcing-stellar-x-ray-protocol-25](https://stellar.org/blog/developers/announcing-stellar-x-ray-protocol-25) (Bri Wylde) frames Protocol 25/X-Ray as *"foundational cryptographic capabilities developers need before higher-level privacy solutions can exist"* — SDF's own framing is primitives-in, solutions-out.
- **Supporting**: [stellar.org/blog/developers/developer-preview-confidential-tokens-on-stellar](https://stellar.org/blog/developers/developer-preview-confidential-tokens-on-stellar) (Maryam Mazraei) — "Confidential Tokens" is explicitly *"a contract suite from OpenZeppelin, wired to an UltraHonk verifier implemented by Nethermind,"* built on Protocol 25 host functions, with SDF stating *"we welcome design partners and community contributions."*
- **Two Meridian 2025 talk titles surfaced** ("The Roadmap for Privacy on Stellar," "Private Transactions for Stellar") — titles only, not transcribed; a lead worth watching directly if deeper grounding is ever needed, not treated as evidence here.
- **CAP status, fetched directly from `github.com/stellar/stellar-protocol`**, not paraphrased from docs that may lag:
  - CAP-0074 ("Host functions for BN254"): `Status: Final`, Protocol version 25, created 2025-09-25, owner Siddharth Suresh.
  - CAP-0075 ("Cryptographic Primitives for Poseidon/Poseidon2 Hash Functions"): `Status: Final`, Protocol version 25, created 2025-10-08, owner Jay Geng.
  - CAP-0080 ("Host functions for efficient ZK BN254 use cases" — MSM/modular-arithmetic additions): `Status: Implemented`, Protocol version 26, created 2026-01-21.
  - All 99 published CAPs (cap-0001 through cap-0087, some numbers reserved/skipped) were searched for confidential/stealth/shielded/ring-signature/encrypted-balance/privacy/nullifier/mixer/homomorphic terms — no CAP for confidential transactions, shielded assets, or stealth addresses exists.
- **SCF signal**: Moonlight (a privacy layer) is confirmed SCF-funded. No active RFP as of the query date (SCF #45, RFP Track, dated 2026-07-23) is themed privacy/confidential-transactions/ZK — a point-in-time fact, not a trend (RFP themes rotate quarterly).

## C. Proof of Reserve — scheme comparison and prior art

**How real implementations work** (Merkle-sum-tree + SNARK is the dominant pattern):

- **Binance** (Feb 2023, still shipping, open-sourced): Merkle tree + zk-SNARK proving all leaves sum to the claimed total and no leaf is negative. [Blog](https://www.binance.com/en/blog/tech/how-zksnarks-improve-binances-proof-of-reserves-system-6654580406550811626) · [github.com/binance/zkmerkle-proof-of-solvency](https://github.com/binance/zkmerkle-proof-of-solvency) (active — v1.1.1, Nov 2025)
- **OKX zk-PoR v2**: Plonky2, batched Sum Proof + Non-negative Proof (1024 accounts/circuit), recursively combined. [Docs](https://www.okx.com/en-us/help/zero-knowledge-proofs-what-are-zk-starks-and-how-do-they-work-v2)
- **Summa** (PSE/Ethereum Foundation-backed): Halo2, total assets ≥ total liabilities with private individual balances, individual inclusion proofs for users. [github.com/summa-dev/summa-solvency](https://github.com/summa-dev/summa-solvency) (243 commits, 99★)
- **Backpack/OtterSec "PoRv2"** (2025): recursive Plonky2, proofs regenerated every ~10 minutes for near-real-time solvency. [Blog](https://osec.io/blog/2025-08-27-how-proof-of-reserves-uses-zk-to-protect-your-funds/)

Common circuit shape: (a) leaf-inclusion in a committed Merkle-sum tree, (b) range-check every leaf ≥ 0 (prevents hiding negative-balance accounts inside an aggregate sum), (c) sum-consistency up the tree, (d) total assets ≥ total committed liabilities — all without revealing individual balances or the full liability list.

**Stellar-specific prior art**: `zkpos.vercel.app` ("Proof-of-Reserves: ZK Solvency on Stellar") — Circom + Groth16 (BN254), deployed to Stellar testnet (Protocol 27), with a live verifier contract (`CBTN433JB2LSFHPLGEZTLEW43IKKXPKN5AYHJVMLKO4NN6EQNBVDRJP4`) that binds reserves to a live on-chain balance via cross-contract read. Not in LumenLoop or SCF history — real but apparently unfunded/unlisted, no identifiable team or license found.

**No dedicated PoR mechanism in Stellar standards**: SEP-1 has an optional field linking to an off-chain reserve attestation/audit document — not a cryptographic proof.

**Demand signal is live, not FTX-era nostalgia**: the US GENIUS Act (July 2025) mandates monthly CPA-examined reserve reports for payment stablecoins (still attestation-based, not cryptographic). The Zondacrypto collapse (April 2026, ~30K users affected, hot-wallet BTC balance down ~99.7% while solvency was claimed) is cited industry-wide as proof that periodic attestation without continuous cryptographic proof is insufficient — pushing the field toward Backpack-style near-real-time zkPoR.

**Reusable circuits, for reference rather than direct reuse** (none are Soroban-targeted):

| Repo | Scheme | Activity |
|---|---|---|
| `binance/zkmerkle-proof-of-solvency` | Go/gnark, Groth16-family | Active |
| `summa-dev/summa-solvency` | Halo2 | Backed, activity strong |
| `summa-dev/circuits-circom` | Circom (predecessor approach) | Likely stale — project moved to Halo2 |
| `okx/proof-of-reserves-v2` | Plonky2 | Exists, activity unverified |

## D. Service feasibility analysis

Candidate service templates, ranked by how unblocked each is against confirmed-live Soroban primitives (BN254 pairing checks, Poseidon/Poseidon2 hashing, both `Final` as of Protocol 25) and the absence of any tool on Soroban that auto-generates a verifier contract per-circuit (unlike the hand-built approach stellar-zk itself already takes).

**Tier 1 — buildable now, near-direct fit**: Anonymous voting / private signaling (Semaphore-style: prove group membership + nullifier against double-signaling). This is Groth16-over-a-pairing-friendly-curve, provable with the same snarkjs-based toolchain stellar-zk already shells out to, and stellar-zk's contracts already carry nullifier tracking and IC-based public-input handling. This is the brief's proposed flagship template (see `brief.md` Scope §2).

**Tier 2 — buildable, but hits a real tooling gap**: Provably-fair shuffle / hidden-state verification (e.g. encrypted-card games — ElGamal shuffle/deal, proof verified on-chain). The client-side crypto is proof-system-agnostic. The gap is that Soroban has no tool that auto-generates a verifier contract for an arbitrary proof system the way some other ecosystems do for UltraHonk-style proofs; stellar-zk's own `ultrahonk_verifier` template is already attempting exactly this by hand. A pragmatic wedge: build the shuffle/decrypt circuits in Circom instead of Noir to ride the already-working Groth16/BN254 path, treating a hardened generic UltraHonk verifier as the harder Phase-9-shaped goal, not the entry point.

**Tier 3 — a real product, not a stellar-zk template**: shielded prediction markets / private AMMs (shielded UTXO pool: commit/insert/spend/claim circuits, Merkle tree, off-chain relayer, automated market maker). Shaped like Nethermind's `stellar-private-payments` — an application built *using* ZK tooling, needing its own off-chain relayer service, out of scope for a local CLI. Reinforces the brief's "infrastructure, not vertical app" scope boundary.

## E. Proposed roadmap reordering (for reconciling against `ROADMAP.md`)

`[ASSUMPTION — sequencing proposal, not yet confirmed against your own constraints/timeline]`

1. **Immediate**: merge the Bn254 fix (blocking, already implemented and verified in a worktree pending review). The fix pins `soroban-sdk = "26"` (minimum version for the `Bn254Fr` rename) — confirmed sufficient for the methods the current templates use (`g1_add`, `g1_mul`, `pairing_check`, `from_bytes`, `Neg`). Newer methods this session found only at 27.0.5 (`g1_msm`, `fr_add/sub/mul/pow/inv`, `g1_is_on_curve`) were not checked at 26.0.0 — before any future template (PoR range-checks, voting Merkle proofs) assumes one of those is available, confirm it against the actually-pinned version first, the same way this fix had to be corrected once already.
2. **Elevate from Phase 8** (production readiness) above the remaining Phase 7 (DX polish) items: production trusted-setup support and real VK extraction for RISC Zero — both matter more once a template ships that people might actually deploy to mainnet, and DX polish (progress bars, shell completions) is lower-stakes than a second correctness gap.
3. **New near-term phase**: the two flagship verticals from Scope §2-3 (private-voting/Semaphore template, Proof-of-Reserve template) — insert ahead of existing Phase 9 (recursive proofs, batch verification, proof aggregation), which is more speculative and less evidence-backed right now.
4. **Phase 10** (ecosystem integration — npm wrapper, VS Code extension, template marketplace) stays valid but is lower urgency than shipping the verticals that would populate a template marketplace in the first place.

## F. Business model — professional services on a free core

The brief through §A-E covers product positioning and technical roadmap; it deliberately said nothing about revenue. This section closes that gap, produced in a dedicated business-model session (Justin/business-analyst persona) after being asked directly "is there a business model here."

**Chosen direction**: professional services layered on top of the free, MIT-licensed CLI and templates — not open-core, not a hosted paid product (yet). The CLI stays fully capable for free; what's sold is human time and accountability.

### Revenue streams, ranked by how directly they're evidenced by this session's own work

1. **Paid implementation/integration engagements.** Take a client's circuit and business requirements to a deployed, working verifier contract using stellar-zk. This directly monetizes the exact gap "The Problem" documents — Nethermind hand-built this pipeline for one vertical, the indie PoR builder built it alone; a paying client gets the same outcome without either path.
2. **Production-readiness / audit review.** This is, concretely, the work already done for free this session: finding that `env.crypto().bn254().g1_neg()` doesn't exist in any real SDK release, reproducing it with an actual `cargo build`, and fixing it against the verified real API. A client with real money behind a mainnet deployment would pay for exactly this level of rigor before shipping. Maps directly to the existing "audit-ready contract templates" line in ROADMAP.md Phase 8.
3. **Protocol-drift retainer.** The `Bn254` API's shape changed materially across `soroban-sdk` 25 → 26 → 27 (scalar type renamed `Fr` → `Bn254Fr`; MSRV jumped to 1.91.0) within this session's own research window. An institutional client with a deployed contract does not want to discover a breaking API change themselves — a monthly retainer to track and pre-empt exactly this class of drift is a recurring-revenue candidate, not a one-off project.
4. **Custom circuit development.** For use cases outside the two flagship templates (voting, PoR) — scoped and billed per project, not a standard offering.

### Who actually pays

Not the free-CLI user base (hobbyists, hackathon builders — evidenced by the ~15-20 abandoned single-commit repos found via Electric Capital). Those are the funnel. Paying customers are the segments already named in "Who This Serves": institutional/RWA teams needing Proof-of-Reserve in production (direct line to the GENIUS Act and Zondacrypto evidence in §C), and governance/DAO teams needing private voting in production (direct line to the Semaphore-pattern evidence in §D).

### Costs and the core tension

Cost structure is almost entirely founder/contractor time — a two-person team (per README), low infrastructure cost since clients self-host the CLI and contracts. The real cost is opportunity: every hour on billable service work is an hour not spent building the OSS differentiation (the two flagship templates, the UltraHonk verifier) that makes the services credible in the first place. This needs a conscious allocation decision, not an implicit one.

### Risks specific to this model

- **Revenue ceiling.** Two people selling hours does not scale like software. Either accept this as a deliberate boutique-expertise business, or treat repeated demand for one service (most likely the protocol-drift retainer) as the signal to productize it instead of continuing to sell it manually.
- **Free-riding.** The core is MIT and already public — nothing stops a better-resourced competitor (including Nethermind, who already has more institutional credibility per SDF's own blog) from offering the identical service on top of the same free tooling. The only real defense is speed and first-hand knowledge of the tool's own failure modes — which is itself a moving target as the SDK keeps changing shape.
- **Conflict-of-interest optics.** Charging for services on top of a tool marketed as an ecosystem public good invites the question of whether the free tier is deliberately left incomplete to force an upsell. Mitigation is a hard rule, not a hope: the CLI and templates must stay fully correct and complete on their own; only human time (implementation, audit, ongoing tracking) is ever sold.

### Explicitly not decided here

`[ASSUMPTION]` No pricing — no comparable was found in this session's research (SCF award amounts are a grant benchmark, not a services-pricing one, and conflating the two would be a fabricated moat). Recommended next step: validate stream 1 (paid implementation) against a single real prospect before pricing the rest of the menu.
