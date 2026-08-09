# CLAUDE.md — stellar-zk

## Current State & Direction

- **Known-fixed, unmerged bug**: all three generated verifier contract templates (`groth16_verifier`, `ultrahonk_verifier`, `risc0_verifier`) called a `Bn254` API (`env.crypto().bn254().g1_from_bytes/.g2_from_bytes/.fr_from_bytes/.g1_neg(...)`) that never existed in any real `soroban-sdk` release — the pin (`"23.4"`) predates the `bn254` module (introduced in `soroban-sdk` 25.0.0), and CI never compiled the *generated* contracts, only the 5 workspace crates. A fix exists (real API: associated `from_bytes` constructors on `Bn254G1Affine`/`Bn254G2Affine`/`Bn254Fr`, `Neg` operator, pin bumped to `soroban-sdk = "26"`) plus a new `scaffold-verify` CI job that builds a freshly scaffolded contract per backend. **Before assuming the templates compile, check whether this fix has been merged to `main`** — if `git log` doesn't show it, the bug is still live.
- **Strategic direction**: stellar-zk is positioned as Stellar's ZK/privacy tooling layer — not a generic multi-chain devkit. This is grounded in SDF's own public strategy (privacy is deliberately application-layer and ecosystem-owned, not a base-protocol feature — see the Meridian 2025 keynote cited in the brief below) and in real competitive research (no SCF-funded project unifies multiple ZK backends into reusable tooling; every funded privacy player hand-builds the pipeline stellar-zk automates).
- **Flagship services being built next** (see `ROADMAP.md` Phase 7A): an anonymous-voting/private-signaling circuit template (Semaphore-style, reuses the existing Groth16 + nullifier pipeline) and a Proof-of-Reserve circuit template (Merkle-sum-tree, targets the RWA/institutional-DeFi vertical). Full rationale, evidence, and an evidence-scoped competitive/protocol analysis: `docs/briefs/brief-stellar-zk-2026-08-08/brief.md` and its `addendum.md`.
- **Do not describe the fixed feature set or the flagship services as already shipped** — they are roadmap items until `ROADMAP.md` checkboxes are actually checked.

## Build & Run

```bash
# Build the workspace
cargo build

# Run CLI
cargo run -- <subcommand>

# Example: init a project
cargo run -- init myapp --backend groth16
```

If `cargo` is not found, source the env first: `source "$HOME/.cargo/env"` or use the absolute path `$HOME/.cargo/bin/cargo`.

## Tests

```bash
# All tests
cargo test

# By crate
cargo test -p stellar-zk-groth16    # 9 tests (serializer)
cargo test -p stellar-zk-ultrahonk   # 6 tests
cargo test -p stellar-zk-risc0       # 5 tests (seal validation)
cargo test -p stellar-zk-core        # core types
```

No external services or Docker needed for unit tests. Tests cover BN254 serialization, seal validation, and proof layout.

## Code Style

- Format: `cargo fmt --all`
- Lint: `cargo clippy --workspace`
- Doc comments: `///` for items, `//!` for module-level
- Error handling: `thiserror` for `StellarZkError`, `anyhow` at the CLI boundary
- Async: `async-trait` for the `ZkBackend` trait
- Serde: derive `Serialize`/`Deserialize` on all config and artifact types

## Architecture

5-crate workspace:

| Crate | Role |
|-------|------|
| `stellar-zk-cli` | Binary. Clap CLI with 6 subcommands. Factory pattern for backend selection. |
| `stellar-zk-core` | Shared library. `ZkBackend` trait, config types, error enum, WASM pipeline, estimator, templates. |
| `stellar-zk-groth16` | Groth16 backend. Shells out to `circom` + `snarkjs`. BN254 serializer. |
| `stellar-zk-ultrahonk` | UltraHonk backend. Shells out to `nargo` + `bb` (Barretenberg). |
| `stellar-zk-risc0` | RISC Zero backend. Shells out to `cargo-risczero`. Seal validation. |

Key patterns:
- **Trait-based backends**: all backends implement `ZkBackend` (in `core/src/backend.rs`)
- **Factory pattern**: `create_backend()` in `cli/src/commands/init.rs` selects backend by name
- **Template embedding**: `include_str!` in `core/src/templates/embedded.rs`, rendered with Handlebars
- **Artifact persistence**: `target/build_artifacts.json` links build → prove → deploy → call

## Key Design Decisions

1. **Shell-out strategy** — Backends call external tools (`circom`, `snarkjs`, `nargo`, `bb`, `cargo-risczero`) instead of linking as Rust deps. Avoids heavy dependency trees and compilation issues (e.g., `wasmer-wasix` doesn't compile on Rust 1.84+).

2. **BN254 serialization** — snarkjs outputs decimal strings; Soroban expects big-endian 32-byte field elements. G2 points swap component order: snarkjs gives `[c0, c1]` but Soroban needs `c1 | c0` (higher-degree first). See `groth16/src/serializer.rs`.

3. **Proof layouts**:
   - Groth16: `A(G1:64) | B(G2:128) | C(G1:64)` = 256 bytes
   - RISC Zero: `selector(4) | Groth16_proof(256)` = 260 bytes
   - UltraHonk: ~14 KB variable-size

4. **VK layout** (Groth16): `alpha(64) | beta(128) | gamma(128) | delta(128) | ic_count(4 BE) | IC[](64 each)`

5. **Templates**: Handlebars templates with `{{contract_name}}` variables. Embedded at compile-time via `include_str!`.

6. **Three profiles**: `development` (fast, no limits), `testnet` (optimized, 64KB limit), `stellar-production` (all limits enforced, 100M CPU).

7. **Soroban limits**: 64KB WASM, 100M CPU instructions per tx. Protocol 25 BN254 host functions.

## Do NOT

- **Do NOT modify templates** in `crates/stellar-zk-core/templates/` without updating the corresponding constants in `crates/stellar-zk-core/src/templates/embedded.rs` — they are loaded at compile-time via `include_str!`.
- **Do NOT add heavy dependencies** (e.g., `risc0-zkvm`, `ark-circom`, `wasmer`). The shell-out strategy is intentional.
- **Do NOT change byte order** in serializers without updating BOTH the Rust serializer AND the corresponding contract template. G2 `c1|c0` ordering is critical.
- **Do NOT use `ark-circom`** — it depends on `wasmer-wasix` which doesn't compile on Rust 1.84+. Use `snarkjs` via shell instead.
- **Do NOT break the artifact chain** — `build_artifacts.json` is consumed by `prove`, `deploy`, `call`, and `estimate`.
- **Do NOT remove overflow_checks** from profiles — ZK verification is security-critical.

## Common Code Patterns

### ZkBackend pipeline (init → build → prove)

Each backend implements the `ZkBackend` trait methods in sequence. The CLI calls them in order, passing artifacts forward:

```rust
// In cli/src/commands/init.rs
let backend = create_backend(backend_name);  // Factory pattern
backend.init_project(&project_dir, &config).await?;

// In cli/src/commands/build.rs
let artifacts = backend.build(&project_dir, &backend_config, &profile).await?;
artifacts.save(&project_dir.join("target/build_artifacts.json"))?;

// In cli/src/commands/prove.rs
let proof = backend.prove(&project_dir, &artifacts, &input_path).await?;
```

### Shell-out pattern (Command::new)

All external tool calls follow this pattern:

```rust
let output = Command::new("snarkjs")
    .args(["groth16", "prove", &zkey_path, &wtns_path, &proof_path, &public_path])
    .current_dir(&project_dir)
    .output()
    .map_err(|e| StellarZkError::ExternalTool { tool: "snarkjs".into(), source: e })?;

if !output.status.success() {
    return Err(StellarZkError::ExternalToolFailed {
        tool: "snarkjs".into(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    });
}
```

### Template rendering

```rust
use crate::templates::{embedded, renderer::TemplateRenderer};

let renderer = TemplateRenderer::new();  // strict_mode(true)
let data = serde_json::json!({
    "contract_name": config.contract.name,
    "project_name": config.project_name,
});
let rendered = renderer.render(embedded::GROTH16_CONTRACT_LIB, &data)?;
std::fs::write(contract_path, rendered)?;
```

## File Coupling Matrix

These groups of files are tightly coupled. When modifying one, check/update the others:

| Group | Files | Relationship |
|-------|-------|-------------|
| **Templates** | `core/templates/**/*.tmpl` ↔ `core/src/templates/embedded.rs` | Templates are loaded at compile-time via `include_str!`. Adding/removing a template requires updating the constant in `embedded.rs`. |
| **Groth16 serialization** | `groth16/src/serializer.rs` ↔ `core/templates/contracts/groth16_verifier/src/lib.rs.tmpl` | Byte layout must match exactly. G2 `c1\|c0` ordering is critical. |
| **UltraHonk serialization** | `ultrahonk/src/serializer.rs` ↔ `core/templates/contracts/ultrahonk_verifier/src/lib.rs.tmpl` | Proof format parsing must match. |
| **RISC Zero serialization** | `risc0/src/serializer.rs` ↔ `core/templates/contracts/risc0_verifier/src/lib.rs.tmpl` | Selector + seal layout must match. |
| **Backend registration** | `cli/src/main.rs` (BackendChoice) ↔ `cli/src/commands/init.rs` (create_backend) ↔ `core/src/config.rs` (BackendConfig) | Adding a backend requires updating all three. |
| **Profiles** | `core/src/profile.rs` ↔ `core/src/pipeline.rs` | Pipeline reads profile settings for opt-level, LTO, wasm-opt flags. |

## Common Workflows

### Add a new backend

1. Create `crates/stellar-zk-<name>/` with `ZkBackend` impl
2. Add circuit template to `crates/stellar-zk-core/templates/circuits/<name>/`
3. Add contract template to `crates/stellar-zk-core/templates/contracts/<name>_verifier/`
4. Register `include_str!` constants in `core/src/templates/embedded.rs`
5. Add variant to `BackendChoice` in `cli/src/main.rs`
6. Add factory case in `cli/src/commands/init.rs::create_backend()`
7. Add cost model in `core/src/estimator.rs`
8. Add `BackendConfig` variant in `core/src/config.rs`

### Modify a template

1. Edit the `.tmpl` file in `crates/stellar-zk-core/templates/`
2. Verify the `include_str!` path in `embedded.rs` still matches
3. Run `cargo build` to confirm the new template compiles into the binary
4. Test with `cargo run -- init testproj --backend <backend>`

### Modify serialization

1. Update the serializer in the relevant backend crate
2. Update the contract template to match the new format
3. Run serializer tests: `cargo test -p stellar-zk-<backend>`
4. Verify proof layout matches contract expectations end-to-end
