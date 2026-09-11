//! The wolf-std test rig (sc00): a plain Rust xtask — no build scripts,
//! ever (D33); no source dependency on any `wolf_*` crate or on
//! wolf-interp (binaries and pinned data are the only couplings).
//!
//! Subcommands: `std-test`, `doc-examples`, `ulp`, `doctor`, `sync-pin`,
//! `ledger-check`, `fmt-lu`, `ci`.
//!
//! `ci` is the twelve-step local gate, and since sc41 it ends by naming
//! which of those twelve `.github/workflows/ci.yml` also runs (F-0113).
//! The twelfth is sc45's `fmt-lu` (F-0117, wolf-std#21): `cargo fmt
//! --all --check` gated the Rust and nothing gated the wolf, so D34 was
//! asserted in commit messages for a dozen sprints and enforced by
//! nobody.

mod anchors;
mod bins;
mod directive;
mod doc_examples;
mod doctor;
mod exec;
mod fmt_lu;
mod genvec;
mod ledger;
mod record;
mod runner;
#[cfg(test)]
mod selftest;
mod stage;
mod sync_pin;
mod tomlite;
mod ulp;
mod workflow;

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
        // `std-test --lint-conventions` (sc06) checks the test-authoring
        // conventions of API-CONVENTIONS §13 without running anything —
        // it reads directives and source text only, so it is instant and
        // is a `ci` step in its own right.
        Some("std-test") if args.iter().any(|a| a == "--lint-conventions") => {
            runner::lint_conventions()
        }
        Some("std-test") => runner::std_test(),
        Some("doc-examples") => doc_examples::doc_examples(),
        Some("ulp") => ulp::ulp(),
        Some("ledger-check") => runner::ledger_check(),
        // sc45: `wolf fmt --check` over every committed `.lu` (D34).
        Some("fmt-lu") => fmt_lu::fmt_lu(),
        // sc16: regenerate the crypto vector tests from `vendor/vectors/`
        // (`--check` verifies the committed files instead of writing).
        Some("gen-vectors") => genvec::gen_vectors(args.iter().any(|a| a == "--check")),
        Some("doctor") => doctor::doctor(),
        Some("sync-pin") => sync_pin::sync_pin(),
        Some("ci") => ci(),
        other => {
            eprintln!(
                "usage: cargo xtask \
                 <std-test [--lint-conventions]|doc-examples|ulp|ledger-check\
                 |gen-vectors [--check]|fmt-lu|doctor|sync-pin|ci>{}",
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
///
/// The step list lives in [`workflow::CI_STEPS`] and is iterated, not
/// re-spelled here (sc41): the same list is diffed against
/// `.github/workflows/ci.yml` at the end of the run and by
/// `selftest::every_ci_step_runs_in_the_ci_workflow`, so a step cannot
/// drift out of CI's reach without saying so on the author's box. F-0113
/// is what that drift costs when nothing says so.
fn ci() -> Result<(), String> {
    let repo = repo_root();
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    for step in workflow::CI_STEPS {
        banner(step.name);
        run_step(step.name, &repo, &cargo)?;
    }
    banner(&format!("workflow coverage ({})", workflow::CI_WORKFLOW));
    workflow::report(&repo)?;
    println!("\nci: GREEN");
    Ok(())
}

/// Run one named step of [`workflow::CI_STEPS`]. An unknown name is a
/// hard error rather than a skip: a step in the list with nothing behind
/// it would be a banner that gates nothing, which is the failure mode
/// this pairing exists to make impossible.
fn run_step(name: &str, repo: &Path, cargo: &str) -> Result<(), String> {
    match name {
        "fmt" => cargo_step(cargo, repo, name, &["fmt", "--all", "--check"]),
        "clippy" => cargo_step(
            cargo,
            repo,
            name,
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--",
                "-D",
                "warnings",
            ],
        ),
        // `--quiet` locally only: the gauntlet's banners are the reading
        // surface here, while a runner wants the full output when a test
        // fails. `workflow::CI_STEPS` carries the runner's spelling.
        "test" => cargo_step(cargo, repo, name, &["test", "--workspace", "--quiet"]),
        "sync-pin" => sync_pin::sync_pin(),
        "doctor" => doctor::doctor(),
        "ledger-check" => runner::ledger_check(),
        "lint-conventions" => runner::lint_conventions(),
        "fmt-lu" => fmt_lu::fmt_lu(),
        "gen-vectors --check" => genvec::gen_vectors(true),
        "std-test" => runner::std_test(),
        "doc-examples" => doc_examples::doc_examples(),
        "ulp" => ulp::ulp(),
        other => Err(format!(
            "ci: `{other}` is in workflow::CI_STEPS with no runner behind it — \
             add a match arm to run_step or take it out of the list"
        )),
    }
}

fn cargo_step(cargo: &str, repo: &Path, name: &str, args: &[&str]) -> Result<(), String> {
    let mut cmd = Command::new(cargo);
    cmd.args(args).current_dir(repo);
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
    Ok(())
}

fn banner(name: &str) {
    println!("\n==> {name}");
}
