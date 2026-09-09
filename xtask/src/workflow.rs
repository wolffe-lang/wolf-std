//! What the WORKFLOW runs, read from the workflow file (sc41, F-0113,
//! wolf-std#13).
//!
//! For four sprints `cargo xtask ci` ran eleven steps and
//! `.github/workflows/ci.yml` ran seven, and nothing said so. A broken
//! `net.write` doc example (F-0112) was green on ubuntu, macOS and
//! windows for the commit that carried it, because no runner had ever
//! executed a `wolf-doc-example`. The drift was invisible in both
//! directions: CI never mentioned the steps it was missing, and the
//! local gauntlet never mentioned that its extra strictness was local.
//!
//! So the gauntlet reads the workflow. [`CI_STEPS`] is the ONE list —
//! `ci()` iterates it to decide what to run, and this module diffs it
//! against the cargo invocations parsed out of `ci.yml`. A step added to
//! one side and not the other is a test failure (`selftest.rs`) and a
//! line of output on the author's box the day it happens, instead of a
//! finding four sprints later.

use std::path::Path;

/// The workflow the gauntlet mirrors. `nightly.yml` is deliberately NOT
/// read: it is a scheduled drift watch that gates nothing
/// (`continue-on-error` at the job level), so counting its steps as
/// coverage would let a step be "in CI" while no merge ever waited on
/// it — the exact confusion F-0113 records.
pub const CI_WORKFLOW: &str = ".github/workflows/ci.yml";

/// One step of the local gate.
pub struct Step {
    /// The banner `ci()` prints, and the name every message uses.
    pub name: &'static str,
    /// The cargo invocation `ci.yml` must carry, verbatim modulo runs of
    /// whitespace, for this step to count as covered. It is the
    /// WORKFLOW's spelling, which is not always the local one: `test`
    /// runs `--quiet` locally so the gauntlet's banners stay readable,
    /// and the runner wants the full output when it fails.
    pub command: &'static str,
}

/// The eleven steps, in the order `ci()` runs them. This list is the
/// gauntlet: `ci()` iterates it, so a step cannot exist here without
/// running, or run without being here.
pub const CI_STEPS: &[Step] = &[
    Step {
        name: "fmt",
        command: "cargo fmt --all --check",
    },
    Step {
        name: "clippy",
        command: "cargo clippy --workspace --all-targets -- -D warnings",
    },
    Step {
        name: "test",
        command: "cargo test --workspace",
    },
    Step {
        name: "sync-pin",
        command: "cargo xtask sync-pin",
    },
    Step {
        name: "doctor",
        command: "cargo xtask doctor",
    },
    Step {
        name: "ledger-check",
        command: "cargo xtask ledger-check",
    },
    Step {
        name: "lint-conventions",
        command: "cargo xtask std-test --lint-conventions",
    },
    Step {
        name: "gen-vectors --check",
        command: "cargo xtask gen-vectors --check",
    },
    Step {
        name: "std-test",
        command: "cargo xtask std-test",
    },
    Step {
        name: "doc-examples",
        command: "cargo xtask doc-examples",
    },
    Step {
        name: "ulp",
        command: "cargo xtask ulp",
    },
];

/// Every `cargo …` invocation the workflow actually runs, whitespace
/// normalized, in file order.
///
/// The parse is deliberately dumb — no YAML crate, by the same
/// dependency charter that keeps `tomlite.rs` in this repository — and
/// it is dumb in the safe direction: it recognizes a command only when
/// the line, after an optional `- ` and an optional `run:`, BEGINS with
/// `cargo `. A comment cannot be mistaken for a command (comment lines
/// are dropped), and a `cargo` mentioned inside an `echo` or a prose
/// comment tail cannot either, because such a line does not begin with
/// it. The failure mode is under-counting, which shows up as a step
/// reported local-only that is not — loud, and never the reverse.
pub fn cargo_invocations(yaml: &str) -> Vec<String> {
    let mut out = Vec::new();
    for raw in yaml.lines() {
        let mut line = raw.trim();
        if line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("- ") {
            line = rest.trim();
        }
        if let Some(rest) = line.strip_prefix("run:") {
            line = rest.trim();
        }
        if let Some(rest) = line.strip_prefix("cargo ") {
            out.push(format!("cargo {}", normalize(rest)));
        }
    }
    out
}

fn normalize(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Steps the gauntlet runs that the workflow does not: local-only gates,
/// where "CI green" says nothing.
pub fn local_only(yaml: &str) -> Vec<&'static str> {
    let have = cargo_invocations(yaml);
    CI_STEPS
        .iter()
        .filter(|s| !have.iter().any(|c| c == s.command))
        .map(|s| s.name)
        .collect()
}

/// Cargo invocations the workflow runs that the gauntlet does not — the
/// other direction of the same drift, and the worse one: a check the
/// author's box cannot reproduce before pushing.
pub fn workflow_only(yaml: &str) -> Vec<String> {
    cargo_invocations(yaml)
        .into_iter()
        .filter(|c| !CI_STEPS.iter().any(|s| s.command == *c))
        .collect()
}

/// Print the coverage, at the end of `ci()`. Reports; never gates — a
/// local gauntlet that goes red because the WORKFLOW is behind would be
/// punishing the author for the repository's state. The gate is the test
/// in `selftest.rs`, which runs on every host, in the `rig` job that was
/// already required.
pub fn report(repo: &Path) -> Result<(), String> {
    let path = repo.join(CI_WORKFLOW);
    let yaml = std::fs::read_to_string(&path).map_err(|e| format!("{CI_WORKFLOW}: {e}"))?;
    let missing = local_only(&yaml);
    let extra = workflow_only(&yaml);
    let covered = CI_STEPS.len() - missing.len();
    println!(
        "ci: {covered} of {} step(s) above also run in {CI_WORKFLOW}",
        CI_STEPS.len()
    );
    if !missing.is_empty() {
        println!(
            "ci: LOCAL-ONLY — {}",
            missing
                .iter()
                .map(|m| format!("`{m}`"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!(
            "ci:   a green CI run on this commit does NOT cover {}. \
             See F-0113 / wolf-std#13: this is how a broken doc example \
             stayed green on three platforms for four sprints.",
            if missing.len() == 1 { "it" } else { "them" }
        );
    }
    for c in &extra {
        println!("ci: WORKFLOW-ONLY — `{c}` runs on the runner and not here");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_both_run_shapes_and_ignores_prose() {
        let yaml = "\
jobs:\n\
\x20 rig:\n\
\x20   steps:\n\
\x20     - name: fmt\n\
\x20       run: cargo fmt --all   --check\n\
\x20     - name: many\n\
\x20       run: |\n\
\x20         # cargo xtask never-run — a comment is not a command\n\
\x20         cargo xtask doctor\n\
\x20         echo \"would run cargo xtask ulp here one day\"\n\
";
        assert_eq!(
            cargo_invocations(yaml),
            vec![
                "cargo fmt --all --check".to_string(),
                "cargo xtask doctor".to_string(),
            ]
        );
    }

    /// The prefix trap: `cargo xtask std-test` and
    /// `cargo xtask std-test --lint-conventions` are two different steps,
    /// and a `contains` match would have called both covered by either.
    #[test]
    fn a_prefix_is_not_a_match() {
        let only_lint = "run: cargo xtask std-test --lint-conventions\n";
        let missing = local_only(only_lint);
        assert!(missing.contains(&"std-test"), "{missing:?}");
        assert!(!missing.contains(&"lint-conventions"), "{missing:?}");
    }

    #[test]
    fn the_pre_sc41_workflow_reports_the_four_of_f0113() {
        // ci.yml exactly as it stood at trunk 1569a0b.
        let seven = "\
\x20     - run: cargo fmt --all --check\n\
\x20     - run: cargo clippy --workspace --all-targets -- -D warnings\n\
\x20     - run: cargo test --workspace\n\
\x20     - run: cargo xtask sync-pin\n\
\x20     - run: cargo xtask doctor\n\
\x20     - run: cargo xtask ledger-check\n\
\x20     - run: cargo xtask std-test\n\
";
        assert_eq!(
            local_only(seven),
            vec![
                "lint-conventions",
                "gen-vectors --check",
                "doc-examples",
                "ulp"
            ]
        );
        assert!(workflow_only(seven).is_empty());
    }

    #[test]
    fn a_step_only_the_runner_has_is_drift_too() {
        let extra = "run: cargo xtask doctor\nrun: cargo xtask sync-pin --write\n";
        assert_eq!(workflow_only(extra), vec!["cargo xtask sync-pin --write"]);
    }

    #[test]
    fn step_names_are_unique_and_nonempty() {
        let mut names: Vec<&str> = CI_STEPS.iter().map(|s| s.name).collect();
        let n = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), n, "duplicate step name in CI_STEPS");
        assert!(CI_STEPS
            .iter()
            .all(|s| !s.name.is_empty() && s.command.starts_with("cargo ")));
    }
}
