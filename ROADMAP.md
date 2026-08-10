# Roadmap

This document outlines the development roadmap for stellar-zk. Items are organized by phase, with completed phases marked accordingly.

---

## Completed

### Phase 1: Foundation

- [x] Cargo workspace with 5 crates (cli, core, groth16, ultrahonk, risc0)
- [x] `ZkBackend` trait with async methods for init, build, prove, estimate
- [x] CLI with 6 subcommands: `init`, `build`, `prove`, `deploy`, `call`, `estimate`
- [x] Configuration system (`stellar-zk.config.json` + `backend.config.json`)
- [x] Three optimization profiles (development, testnet, stellar-production)
- [x] Template engine (Handlebars) with embedded templates via `include_str!`
- [x] Project scaffolding with directory creation and template rendering
- [x] Static cost estimation models per backend

### Phase 2: Groth16 Backend

- [x] Circom circuit compilation via `circom` CLI
- [x] Powers of Tau generation for development
- [x] Trusted setup via `snarkjs groth16 setup`
- [x] Witness generation via `snarkjs wtns calculate`
- [x] Proof generation via `snarkjs groth16 prove`
- [x] BN254 serializer: snarkjs JSON (decimal strings) to Soroban binary (big-endian bytes)
- [x] VK serialization to binary format (alpha, beta, gamma, delta, IC points)
- [x] Soroban verifier contract with full pairing check
- [x] 9 unit tests for serialization

### Phase 3: Deploy, Call, and Estimate Pipeline

- [x] Build artifact persistence (`target/build_artifacts.json`)
- [x] WASM pipeline: `cargo build` -> `wasm-opt` -> `wasm-strip` -> size validation
- [x] Deploy with VK initialization via `__constructor(vk_bytes)`
- [x] Call with full verification args (proof, public_inputs, nullifier)
- [x] SHA256-based nullifier computation
- [x] Simulate output parsing for Tier 3 cost estimation
- [x] Stellar CLI wrapper (deploy, invoke, simulate)

### Phase 4: UltraHonk Backend

- [x] Noir circuit compilation via `nargo compile`
- [x] VK generation via `bb write_vk`
- [x] Witness generation via `nargo execute`
- [x] Proof generation via `bb prove_ultra_honk`
- [x] Off-chain verification via `bb verify_ultra_honk`
- [x] Oracle hash read from config (cached for prove)
- [x] `Nargo.toml` template for scaffolded projects
- [x] `public_inputs.json` output matching Groth16 pattern
- [x] KZG pairing-check on-chain verifier contract
- [x] 6 unit tests for proof format parsing

### Phase 5: RISC Zero Backend

- [x] `Cargo.toml` templates for guest and host programs
- [x] Host template with structured output (seal.bin, journal.bin, image_id.hex)
- [x] Guest and host binary compilation (`build_guest`, `build_host`)
- [x] Proof generation via shell-out to host binary
- [x] Seal validation (260 bytes, selector check)
- [x] Journal digest computation (SHA256)
- [x] `public_inputs.json` output matching other backends
- [x] Groth16 pairing-check on-chain verifier contract (with selector validation)
- [x] Build config caching for prove step
- [x] 5 unit tests for seal serialization and validation

---

### Phase 6: Testing and Hardening

- [x] Integration tests for the full `init -> build -> prove` pipeline (all 3 backends)
- [x] Edge case handling in serializers (overflow, malformed inputs)
- [x] Error message improvements with actionable recovery suggestions
- [x] CI pipeline (GitHub Actions) with build + test + clippy + fmt checks
- [x] Cross-platform testing (Linux, macOS)
- [x] Prerequisite version checking (minimum versions for circom, snarkjs, nargo, bb, cargo-risczero)
- [x] Graceful handling of missing external tools mid-pipeline (not just at init)

---

## Planned

### Immediate — blocking

Nothing below ships on top of an unmerged core fix.

- [ ] **Merge the Bn254 verifier-contract fix.** All three generated verifier contracts (`groth16_verifier`, `ultrahonk_verifier`, `risc0_verifier`) called a `Bn254` API that never existed in any real `soroban-sdk` release — the pin (`"23.4"`) predates the `bn254` module entirely (introduced in `soroban-sdk` 25.0.0). Fixed against the real API (associated `from_bytes` constructors, `Neg` operator, `g1_add`/`g1_mul`/`pairing_check`), pinned to `soroban-sdk = "26"`, verified via a real `cargo build` of all three scaffolded backends. A new `scaffold-verify` CI job now builds a freshly scaffolded contract per backend so this class of bug can't ship silently again — closes the exact gap that let it through (CI previously only built the 5 workspace crates, never the templates that only materialize when a user runs `init`).

### Phase 7A: Privacy & Compliance Service Templates

Two flagship circuit templates identified through competitive and protocol research (see `docs/briefs/brief-stellar-zk-2026-08-08/`) as the highest-leverage next services to offer — both build on the existing Groth16/nullifier pipeline rather than requiring new infrastructure.

- [ ] **Anonymous voting / private signaling template** (prove group membership + a nullifier against double-signaling, without revealing which member acted). Reuses stellar-zk's existing Groth16 pipeline and nullifier tracking almost unchanged. Poseidon hashing (CAP-0075) is confirmed live at the protocol level; the exact `soroban-sdk` API surface for it still needs verification before this is scoped as "easy," the same way the Bn254 API needed verifying.
- [ ] **Proof-of-Reserve template** (Merkle-sum-tree with range-checked leaves, proving total reserves ≥ total liabilities without revealing individual balances). Directly serves the RWA/institutional-DeFi vertical. Validated as buildable on this exact stack (Circom + Groth16 + BN254) by an existing independent implementation already live on Stellar testnet.

### Phase 7B: Developer Experience

Polish the CLI and make the tool easier to use for newcomers.

- [ ] Interactive `prove` command (prompt for inputs if not provided)
- [ ] `stellar-zk status` command showing project state (built? proved? deployed?)
- [ ] Progress bars for long-running operations (circuit compilation, proof generation)
- [ ] Colored diff output for cost estimation (show changes between runs)
- [ ] `stellar-zk verify` command for local off-chain proof verification
- [ ] `stellar-zk clean` command to remove build artifacts
- [ ] Better error messages when external tools produce unexpected output
- [ ] Shell completions (bash, zsh, fish)

### Phase 8: Production Readiness

Features required for mainnet deployments. The first two items are elevated in priority above the remainder of Phase 7B — a second correctness gap matters more than DX polish, and any service template that ships needs a real trusted setup and real VK behind it.

- [ ] **(elevated)** Production trusted setup support (import community Powers of Tau files for Groth16)
- [ ] **(elevated)** VK extraction from RISC Zero host at build time (replace placeholder with real universal VK)
- [ ] Contract upgrade support (versioned VKs, migration paths)
- [ ] Gas profiling: detailed breakdown of BN254 operation costs per verify() call
- [ ] WASM size optimization reports (which functions contribute most to size)
- [ ] Audit-ready contract templates (formal comments, invariant documentation)
- [ ] Deterministic builds (pinned tool versions, reproducible WASM output)

### Phase 9: Advanced Proving Features

Extend the backends with advanced capabilities.

- [ ] Recursive proof composition (prove verification of a proof)
- [ ] Batch verification (verify multiple proofs in a single transaction)
- [ ] Custom circuit support for UltraHonk (user-defined oracle functions) — the hardened version of this, paired with a generic UltraHonk verifier, is the differentiator worth investing in once a Noir-based service (e.g. a provably-fair shuffle/deal template) makes the case concrete rather than speculative
- [ ] RISC Zero continuation support (segment proofs for long computations)
- [ ] Groth16 proof aggregation (SnarkPack or similar)
- [ ] Witness generation from on-chain data (Stellar ledger queries as circuit inputs)
- [ ] Private input management (encrypted input storage, key derivation)

### Phase 10: Ecosystem Integration

Connect stellar-zk with the broader Stellar and ZK ecosystems.

- [ ] npm/npx package for easy installation (`npx stellar-zk init myapp`)
- [ ] VS Code extension (syntax highlighting for circuit files, inline cost estimates)
- [ ] Soroban SDK integration (Rust helper library for calling verifier contracts)
- [ ] Proof relay service (submit proofs without running a Stellar node)
- [ ] Explorer integration (link verified proofs to Stellar transaction history)
- [ ] Template marketplace (community-contributed circuit templates)
- [ ] Multi-chain support (deploy verifiers to other chains with BN254 support)

### Phase 11: Performance and Optimization

Push the boundaries of what fits within Soroban's resource limits.

- [ ] Custom WASM optimization passes for verifier contracts
- [ ] Precomputed pairing values for fixed VKs (reduce on-chain CPU)
- [ ] Compressed proof formats (where protocol allows)
- [ ] Parallel proof generation (multi-threaded witness computation)
- [ ] Incremental compilation (only rebuild changed circuit components)
- [ ] Proof caching (skip re-proving if inputs haven't changed)

---

## Future Considerations

These are ideas being evaluated but not yet committed to:

- **New backends**: Plonky2, Halo2, SP1 — as Soroban adds new host functions or precompiles
- **zkLogin**: Stellar account abstraction using ZK proofs of OAuth/OIDC tokens
- **zkBridge**: Cross-chain light client verification using ZK proofs of block headers
- **Privacy primitives**: Shielded transfers, confidential assets, private prediction markets/AMMs built *on* stellar-zk's verifier infrastructure — deliberately not a stellar-zk deliverable itself (these need off-chain relayer infrastructure beyond a local CLI's reach), but tracked here as the kind of application this project should enable
- **Formal verification**: Machine-checked correctness proofs for the verifier contract templates
- **Hardware acceleration**: GPU/FPGA support for proof generation via backend plugins

---

## Versioning

stellar-zk follows [Semantic Versioning](https://semver.org/):

- **0.1.x**: Current development. API may change between minor versions.
- **0.2.0**: Target for the Bn254 fix plus Phase 7A (privacy & compliance service templates) and Phase 7B (developer experience polish).
- **1.0.0**: Target for Phase 8 completion (production readiness). Stable API commitment.

---

## Contributing

Want to help? See [CONTRIBUTING.md](CONTRIBUTING.md) for development setup and guidelines. The most impactful areas for contribution right now are:

1. **Privacy & compliance service templates** (Phase 7A) — the anonymous-voting and Proof-of-Reserve templates are the current flagship priority
2. **Developer experience** (Phase 7B) — CLI polish, error messages, shell completions
3. **New backend exploration** — prototype implementations for Plonky2, Halo2, or SP1
