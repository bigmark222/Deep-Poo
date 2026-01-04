use std::process::Command;

const WORKSPACE_MANIFEST: &str = "./Cargo.toml";

#[test]
fn runs_headless_datagen_against_registry_crates() {
    let tmp = tempfile::tempdir().expect("tmpdir");

    // Run a tiny datagen capture in headless mode to ensure registry crates link and run.
    let output = Command::new("cargo")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "run",
            "--locked",
            "--quiet",
            "--bin",
            "datagen_headless",
            "--manifest-path",
            WORKSPACE_MANIFEST,
            "--",
            "--max-frames",
            "3",
            "--output-root",
            tmp.path().to_str().unwrap(),
            "--headless",
        ])
        // Prefer software/GL backends so this can run in CI without a GPU.
        .env("RUST_LOG", "warn")
        .env("WGPU_BACKEND", "gl")
        .env("WGPU_ALLOWED_BACKENDS", "gl,vulkan,metal")
        .env("WGPU_POWER_PREF", "low")
        .env("BEVY_WGPU_POWER_PREFERENCE", "low_power")
        .env("BEVY_WGPU_BACKEND", "gl")
        .output()
        .expect("failed to spawn cargo run");

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("Unable to find a GPU") {
            eprintln!("skipping registry smoke: no GPU backend available");
            return;
        }
        panic!(
            "datagen_headless should exit cleanly against registry crates\nstdout:\n{}\nstderr:\n{}",
            stdout, stderr
        );
    }

    // Basic output assertions: one run dir with images/labels/overlays + manifest.
    let runs: Vec<_> = std::fs::read_dir(tmp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        !runs.is_empty(),
        "expected at least one run directory in {:?}",
        tmp.path()
    );
    let run_dir = runs[0].path();
    assert!(run_dir.join("images").is_dir());
    assert!(run_dir.join("labels").is_dir());
    assert!(run_dir.join("overlays").is_dir());
    assert!(run_dir.join("run_manifest.json").is_file());
}
