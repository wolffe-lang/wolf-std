//! The wolf-std test rig (sc00): a plain Rust xtask — no build scripts,
//! ever (D33); no source dependency on any `wolf_*` crate or on
//! wolf-interp (binaries and pinned data are the only couplings).
//!
//! Subcommands: `std-test`, `doc-examples`, `ulp`, `doctor`, `sync-pin`,
//! `ledger-check`, `ci`.

mod anchors;
mod bins;
mod directive;
mod doc_examples;
mod doctor;
mod exec;
mod ledger;
mod record;
mod runner;
#[cfg(test)]
mod selftest;
mod stage;
mod sync_pin;
mod tomlite;
mod ulp;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The repository root (the xtask crate lives at `<root>/xtask`).
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask sits one level under the repo root")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("std-test") => runner::std_test(),
        Some("doc-examples") => doc_examples::doc_examples(),
        Some("ulp") => ulp::ulp(),
        Some("ledger-check") => runner::ledger_check(),
        Some("doctor") => doctor::doctor(),
        Some("sync-pin") => sync_pin::sync_pin(),
        Some("ci") => ci(),
        other => {
            eprintln!(
                "usage: cargo xtask \
                 <std-test|doc-examples|ulp|ledger-check|doctor|sync-pin|ci>{}",
                other.map(|o| format!(" (got `{o}`)")).unwrap_or_default()
            );
            std::process::exit(2);
        }
    };
    if let Err(e) = result {
        eprintln!("{e}");
        eprintln!("xtask: RED");
        std::process::exit(1);
    }
}

/// The full local gate, mirroring the CI lanes: fmt, clippy -D warnings,
/// tests, then the rig's own checks. Fail-fast, loud banners, a timeout
/// ceiling on every spawned step.
fn ci() -> Result<(), String> {
    let repo = repo_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let steps: &[(&str, &[&str])] = &[
        ("fmt", &["fmt", "--all", "--check"]),
        (
            "clippy",
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        ("test", &["test", "--workspace", "--quiet"]),
    ];
    for (name, cargo_args) in steps {
        banner(name);
        let mut cmd = Command::new(&cargo);
        cmd.args(*cargo_args).current_dir(&repo);
        let got = exec::run(cmd, Duration::from_secs(600))?;
        if got.timed_out {
            return Err(format!("ci: `cargo {name}` blew the 600s ceiling"));
        }
        if got.status != Some(0) {
            return Err(format!(
                "ci: `cargo {name}` failed\n{}\n{}",
                got.stdout.trim_end(),
                got.stderr.trim_end()
            ));
        }
    }
    banner("sync-pin");
    sync_pin::sync_pin()?;
    banner("doctor");
    doctor::doctor()?;
    banner("ledger-check");
    runner::ledger_check()?;
    banner("std-test");
    runner::std_test()?;
    banner("doc-examples");
    doc_examples::doc_examples()?;
    banner("ulp");
    ulp::ulp()?;
    println!("\nci: GREEN");
    Ok(())
}

fn banner(name: &str) {
    println!("\n==> {name}");
}
