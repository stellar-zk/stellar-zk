use std::process::Command;

use stellar_zk_core::config::ProjectConfig;

#[test]
fn init_persists_the_requested_profile_for_each_backend() {
    let dir = tempfile::tempdir().unwrap();

    for backend in ["groth16", "ultrahonk", "risc0"] {
        for profile in ["development", "testnet", "stellar-production"] {
            let name = format!("{backend}-{profile}");
            let output = Command::new(env!("CARGO_BIN_EXE_stellar-zk"))
                .current_dir(dir.path())
                .args(["init", &name, "--backend", backend, "--profile", profile])
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "init failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );

            let config =
                ProjectConfig::load(&dir.path().join(name).join("stellar-zk.config.json")).unwrap();
            assert_eq!(config.backend, backend);
            assert_eq!(config.profile, profile);
        }
    }
}

#[test]
fn init_defaults_to_the_development_profile() {
    let dir = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_stellar-zk"))
        .current_dir(dir.path())
        .args(["init", "default-profile", "--backend", "groth16"])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "init failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let config =
        ProjectConfig::load(&dir.path().join("default-profile/stellar-zk.config.json")).unwrap();
    assert_eq!(config.profile, "development");
}
