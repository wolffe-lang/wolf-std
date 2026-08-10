//! `cargo xtask sync-pin` — the two-pin ritual's verifier. wolf-lang is
//! a DATA pin: `upstream/` is the submodule (sparse: `spec/` + `corpus/`),
//! `vendor/upstream/` is the tracked snapshot (PIN + anchors.json) that
//! keeps CI working where the private submodule cannot clone (org policy
//! disables deploy keys — the lesson ls00 §3 already paid for).
//! Snapshot == submodule whenever the submodule is present; PIN
//! well-formed always.

use crate::anchors::Registry;
use crate::bins::is_full_sha;
use crate::exec;
use crate::repo_root;
use std::process::Command;
use std::time::Duration;

pub fn sync_pin() -> Result<(), String> {
    let repo = repo_root();
    let pin_path = repo.join("vendor/upstream/PIN");
    let pin =
        std::fs::read_to_string(&pin_path).map_err(|e| format!("vendor/upstream/PIN: {e}"))?;
    let pin = pin.trim();
    if !is_full_sha(pin) {
        return Err(format!(
            "vendor/upstream/PIN: `{pin}` is not a 40-hex commit sha"
        ));
    }
    println!("sync-pin: PIN {pin}");

    let snapshot_path = repo.join("vendor/upstream/anchors.json");
    let snapshot =
        std::fs::read(&snapshot_path).map_err(|e| format!("vendor/upstream/anchors.json: {e}"))?;
    Registry::from_json(&String::from_utf8_lossy(&snapshot))
        .map_err(|e| format!("vendor/upstream/anchors.json: corrupt snapshot: {e}"))?;
    println!("sync-pin: anchors.json snapshot parses (registry v1)");

    // The submodule may be absent (CI cannot clone the private repo);
    // that is legal and announced. When present, the snapshot must be
    // byte-identical to the submodule at the PIN.
    let upstream = repo.join("upstream");
    if !upstream.join(".git").exists() {
        println!(
            "notice: upstream/ submodule not initialized — snapshot-only checks \
             (this is CI's normal state until the upstream repo is readable)"
        );
        return Ok(());
    }
    let mut cmd = Command::new("git");
    cmd.arg("-C").arg(&upstream).args(["rev-parse", "HEAD"]);
    let got = exec::run(cmd, Duration::from_secs(10))?;
    if got.status != Some(0) {
        return Err(format!("git rev-parse in upstream/: {}", got.stderr.trim()));
    }
    let head = got.stdout.trim();
    if head != pin {
        return Err(format!(
            "upstream/ is at {head}, PIN records {pin} — checkout the pin or \
             re-vendor deliberately (its own commit)"
        ));
    }
    let sub_anchors = std::fs::read(upstream.join("spec/anchors.json"))
        .map_err(|e| format!("upstream/spec/anchors.json: {e}"))?;
    if sub_anchors != snapshot {
        return Err(
            "vendor/upstream/anchors.json differs from upstream/spec/anchors.json \
             at the pin — NEVER edit the snapshot by hand; re-vendor: \
             cp upstream/spec/anchors.json vendor/upstream/anchors.json"
                .to_string(),
        );
    }
    println!("sync-pin: snapshot == submodule at pin — OK");
    Ok(())
}
