#![cfg(unix)]

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use stellar_zk_core::backend::BuildArtifacts;
use stellar_zk_core::config::{BackendConfig, ProjectConfig};

const PROOF: &[u8] = b"\0\0\0\0fixture-proof";

struct Fixture {
    _dir: tempfile::TempDir,
    project: PathBuf,
    caller: PathBuf,
    path: OsString,
}

impl Fixture {
    fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path().join("project");
        let caller = dir.path().join("caller");
        let tools = dir.path().join("tools");
        fs::create_dir_all(project.join("circuits")).unwrap();
        fs::create_dir(&caller).unwrap();
        fs::create_dir(&tools).unwrap();
        ProjectConfig::default_for_backend("project", "ultrahonk")
            .save(&project.join("stellar-zk.config.json"))
            .unwrap();
        BackendConfig::default_for_backend("ultrahonk")
            .save(&project.join("backend.config.json"))
            .unwrap();
        fs::write(project.join("input.json"), "{}").unwrap();
        stellar_zk_core::artifacts::save(
            &BuildArtifacts {
                circuit_artifact: project.join("target/circuit.json"),
                verifier_wasm: project.join("target/verifier.wasm"),
                proving_key: None,
                verification_key: project.join("target/vk"),
            },
            &project.join("target"),
        )
        .unwrap();

        // Stub only the external proof tools; exercise the real CLI and backend file I/O.
        write_tool(
            &tools.join("nargo"),
            "#!/bin/sh\ncase \"$1\" in\n  --version) echo 1.0.0 ;;\n  execute) exit 0 ;;\n  *) exit 2 ;;\nesac\n",
        );
        write_tool(
            &tools.join("bb"),
            r#"#!/bin/sh
case "$1" in
  --version) echo 1.0.0 ;;
  prove_ultra_honk)
    shift
    output=
    while [ "$#" -gt 0 ]; do
      case "$1" in
        -o) output=$2 ;;
        --oracle_hash|-b|-w) ;;
        *) exit 2 ;;
      esac
      shift 2
    done
    [ -n "$output" ] || exit 2
    printf '\000\000\000\000fixture-proof' > "$output"
    ;;
  verify_ultra_honk) exit 0 ;;
  *) exit 2 ;;
esac
"#,
        );
        let path = std::env::join_paths([tools]).unwrap();
        Self {
            _dir: dir,
            project,
            caller,
            path,
        }
    }

    fn prove(&self, output: Option<&Path>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_stellar-zk"));
        command
            .current_dir(&self.caller)
            .env("PATH", &self.path)
            .arg("--config")
            .arg(self.project.join("stellar-zk.config.json"))
            .args(["prove", "--input"])
            .arg(self.project.join("input.json"));
        if let Some(output) = output {
            command.arg("--output").arg(output);
        }
        command.output().unwrap()
    }

    fn assert_success(&self, result: &Output, displayed_path: &Path) {
        assert!(
            result.status.success(),
            "prove failed: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            fs::read(self.project.join("proofs/proof.bin")).unwrap(),
            PROOF
        );
        assert!(self.project.join("proofs/public_inputs.json").is_file());
        let stdout = String::from_utf8_lossy(&result.stdout);
        assert!(
            stdout.lines().any(|line| line.contains("Proof file")
                && line.ends_with(&displayed_path.display().to_string())),
            "wrong displayed proof path: {stdout}"
        );
    }
}

fn write_tool(path: &Path, script: &str) {
    fs::write(path, script).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

#[test]
fn prove_writes_relative_output_in_the_callers_directory() {
    let fixture = Fixture::new();
    let destination = Path::new("custom proof.bin");
    let result = fixture.prove(Some(destination));
    fixture.assert_success(&result, destination);
    assert_eq!(fs::read(fixture.caller.join(destination)).unwrap(), PROOF);
    assert!(!fixture.project.join(destination).exists());
}

#[test]
fn prove_writes_absolute_output_and_overwrites_an_existing_file() {
    let fixture = Fixture::new();
    let destination = fixture.caller.join("existing-proof.bin");
    fs::write(&destination, b"old proof with trailing bytes").unwrap();
    let result = fixture.prove(Some(&destination));
    fixture.assert_success(&result, &destination);
    assert_eq!(fs::read(destination).unwrap(), PROOF);
}

#[test]
fn prove_keeps_the_default_output_when_no_destination_is_given() {
    let fixture = Fixture::new();
    let destination = fixture.project.join("proofs/proof.bin");
    let result = fixture.prove(None);
    fixture.assert_success(&result, &destination);
}

#[test]
fn prove_accepts_the_backend_default_as_an_explicit_destination() {
    let fixture = Fixture::new();
    let destination = fixture.project.join("proofs/proof.bin");
    let result = fixture.prove(Some(&destination));
    fixture.assert_success(&result, &destination);
}

#[test]
fn prove_reports_an_output_write_failure() {
    let fixture = Fixture::new();
    for destination in [
        fixture.caller.clone(),
        fixture.caller.join("missing-parent/proof.bin"),
    ] {
        let result = fixture.prove(Some(&destination));
        assert!(!result.status.success());
        assert!(
            String::from_utf8_lossy(&result.stderr).contains(&destination.display().to_string()),
            "write error omits the output path"
        );
        assert!(!String::from_utf8_lossy(&result.stdout).contains("Proof generated"));
    }
}
