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

/// The twelve steps, in the order `ci()` runs them. This list is the
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
    // sc45, F-0117, wolf-std#21 — THE STEP THAT WAS NEVER HERE. D34
    // ("`wolf fmt` is law for every committed `.lu`") had no mechanism:
    // `fmt` above gates the RUST, and until this entry nothing in either
    // list gated the WOLF. Thirty-two files had been off the fixed point
    // at three consecutive pins. It sits beside `lint-conventions`
    // because it is the other static reading of the same `.lu` files,
    // and it costs 0.2 s over the whole tree.
    Step {
        name: "fmt-lu",
        command: "cargo xtask fmt-lu",
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

/// A step of `ci.yml` that RUNS and GATES NOTHING on some host, because
/// `continue-on-error` rewrites what its red becomes.
///
/// sc47 / wolf-std#34. This module already refused to count
/// `nightly.yml` as coverage, and said why: "counting its steps as
/// coverage would let a step be 'in CI' while no merge ever waited on
/// it — the exact confusion F-0113 records." That rule was applied to a
/// whole workflow at the JOB level and never to a STEP inside `ci.yml`,
/// which is the hole `std-test` sat in for four sprints: it ran on three
/// hosts, it was RED on one of them from sc43 onward, and two lanes read
/// the green and reported it.
///
/// What makes it worse than "the job says success" is that the STEP says
/// success too. At run 34672768728 the REST API reports
/// `conclusion: success` for `std-test` on `rig (windows-latest)` while
/// its log ends `xtask: RED` — GitHub rewrites the conclusion at both
/// levels, so no amount of care reading conclusions recovers the truth.
/// The fact that DOES survive is this key, in this file, which is why
/// the marker is gated here rather than moved into a shell variable.
pub struct Advisory {
    /// The cargo invocation the advisory step runs.
    pub command: &'static str,
    /// The `continue-on-error:` value, verbatim after whitespace
    /// normalization. Pinning the EXPRESSION and not just the step is
    /// what stops the advisory set from being widened a host at a time.
    pub expr: &'static str,
    /// The open issue that says why this is advisory and what closes it.
    /// An advisory step without one is a step nobody is going to fix.
    pub issue: &'static str,
}

/// Every advisory step this repository has BLESSED, with its reason.
///
/// `selftest.rs` asserts that what `ci.yml` carries is exactly this
/// list, in both directions, so an advisory marker cannot be added,
/// widened, or outlive its issue without the `rig` job going red on
/// every host — the gate that would have caught wolf-std#34 at sc43,
/// on the author's box, before the push.
pub const ADVISORY_STEPS: &[Advisory] = &[
    // sc47: narrowed from `${{ runner.os != 'macOS' }}`. ubuntu is
    // REQUIRED from this commit — `std-test: GREEN` on
    // `rig (ubuntu-latest)` at four consecutive heads (34602166418,
    // 34618995826, 34668184512, 34672768728) — and windows is the only
    // host left, behind three native net/process rows that arrived when
    // wolf-std#18 lit the native rung there.
    Advisory {
        command: "cargo xtask std-test",
        expr: "${{ runner.os == 'Windows' }}",
        issue: "wolf-std#34",
    },
];

/// One `cargo …` invocation of the workflow, with the advisory marker of
/// the step that runs it.
#[derive(Debug, PartialEq, Eq)]
pub struct Invocation {
    /// The command, whitespace normalized.
    pub command: String,
    /// `Some(expr)` when the step carries `continue-on-error:`. A red
    /// here reaches the job only on the hosts where `expr` is false.
    pub advisory: Option<String>,
}

/// Every `cargo …` invocation the workflow actually runs, whitespace
/// normalized, in file order, each with its step's advisory marker.
///
/// The parse is deliberately dumb — no YAML crate, by the same
/// dependency charter that keeps `tomlite.rs` in this repository — and
/// it is dumb in the safe direction, which is a DIFFERENT direction for
/// each of the two things it reads:
///
///   * For commands, under-counting is safe: it recognizes a command
///     only when the line, after an optional `- ` and an optional
///     `run:`, BEGINS with `cargo `. A comment cannot be mistaken for a
///     command (comment lines are dropped), and a `cargo` mentioned
///     inside an `echo` or a prose comment tail cannot either, because
///     such a line does not begin with it. A miss shows up as a step
///     reported local-only that is not — loud, and never the reverse.
///
///   * For advisory markers, OVER-counting is safe and under-counting is
///     the bug being fixed: a missed marker leaves a step gating nothing
///     with the gate still green, which is wolf-std#34 exactly. So a
///     `continue-on-error:` indented SHALLOWER than the steps — a
///     job-level marker, which would make every step in that job
///     advisory — marks every invocation in the file rather than being
///     skipped. That over-reports across jobs and reds the selftest,
///     which is the loud direction.
///
/// Steps are split on a line whose trimmed form starts with `- name:` or
/// `- uses:`, so a marker and its command are found together however
/// they are ordered within the step.
pub fn invocations(yaml: &str) -> Vec<Invocation> {
    let lines: Vec<&str> = yaml.lines().collect();
    let is_step_start =
        |l: &str| l.trim_start().starts_with("- name:") || l.trim_start().starts_with("- uses:");
    let indent = |l: &str| l.len() - l.trim_start().len();

    // A `continue-on-error:` above/outside the steps is job-level.
    let step_indent = lines.iter().find(|l| is_step_start(l)).map(|l| indent(l));
    let mut job_level: Option<String> = None;
    for raw in &lines {
        let line = raw.trim();
        if line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix("continue-on-error:") {
            if step_indent.is_none_or(|si| indent(raw) < si) {
                job_level = Some(normalize(rest));
            }
        }
    }

    // Split into step blocks; everything before the first step start is
    // the job's own keys and runs no `cargo` of its own here.
    let mut blocks: Vec<Vec<&str>> = Vec::new();
    let mut cur: Vec<&str> = Vec::new();
    for raw in &lines {
        if is_step_start(raw) && !cur.is_empty() {
            blocks.push(std::mem::take(&mut cur));
        }
        cur.push(raw);
    }
    blocks.push(cur);

    let mut out = Vec::new();
    for block in blocks {
        let mut advisory = job_level.clone();
        let mut commands = Vec::new();
        for raw in block {
            let mut line = raw.trim();
            if line.starts_with('#') {
                continue;
            }
            if let Some(rest) = line.strip_prefix("continue-on-error:") {
                advisory = Some(normalize(rest));
                continue;
            }
            if let Some(rest) = line.strip_prefix("- ") {
                line = rest.trim();
            }
            if let Some(rest) = line.strip_prefix("run:") {
                line = rest.trim();
            }
            if let Some(rest) = line.strip_prefix("cargo ") {
                commands.push(format!("cargo {}", normalize(rest)));
            }
        }
        for command in commands {
            out.push(Invocation {
                command,
                advisory: advisory.clone(),
            });
        }
    }
    out
}

/// Every `cargo …` invocation the workflow actually runs, whitespace
/// normalized, in file order — advisory or not.
pub fn cargo_invocations(yaml: &str) -> Vec<String> {
    invocations(yaml).into_iter().map(|i| i.command).collect()
}

/// The invocations that carry a `continue-on-error:` marker, as
/// `(command, expr)` pairs in file order. These are the steps whose red
/// does not reach the job on at least one host.
pub fn advisory_invocations(yaml: &str) -> Vec<(String, String)> {
    invocations(yaml)
        .into_iter()
        .filter_map(|i| i.advisory.map(|a| (i.command, a)))
        .collect()
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
    // sc47 / wolf-std#34 — a step that RUNS is not a step that GATES.
    // Printed on the author's box beside LOCAL-ONLY because it is the
    // same fact one level down: `std-test` was in this list, and green,
    // and red on windows, for four sprints.
    for (command, expr) in advisory_invocations(&yaml) {
        let issue = ADVISORY_STEPS
            .iter()
            .find(|a| a.command == command && a.expr == expr)
            .map(|a| a.issue)
            .unwrap_or("UNBLESSED — selftest will red");
        println!(
            "ci: ADVISORY — `{command}` runs with `continue-on-error: {expr}`, \
             so its RED does not reach the job where that is true ({issue})"
        );
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
        // The four of F-0113 are exactly the four this fixture cannot
        // cover, and they still are. sc45's `fmt-lu` joins them because
        // that workflow did not run it either — it did not exist — so
        // the list is five now and the F-0113 four are the first four
        // in order. Asserted as containment rather than equality: the
        // fixture is frozen history, and the thing it proves is that
        // the reader FINDS a step no runner executes, not how many
        // steps the gauntlet has grown to since.
        assert_eq!(
            local_only(seven),
            vec![
                "lint-conventions",
                "fmt-lu",
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

    /// sc47 / wolf-std#34 — the marker is read per STEP, and a step
    /// that carries it is not coverage.
    #[test]
    fn an_advisory_step_is_read_off_its_own_marker() {
        let yaml = "\
jobs:\n\
\x20 rig:\n\
\x20   steps:\n\
\x20     - name: required\n\
\x20       run: cargo xtask doctor\n\
\x20     - name: advisory\n\
\x20       continue-on-error: ${{ runner.os == 'Windows' }}\n\
\x20       run: cargo xtask std-test\n\
\x20     - name: required again\n\
\x20       run: cargo xtask ulp\n\
";
        assert_eq!(
            advisory_invocations(yaml),
            vec![(
                "cargo xtask std-test".to_string(),
                "${{ runner.os == 'Windows' }}".to_string()
            )]
        );
        // The advisory step still RUNS, so it is still covered — the two
        // questions are separate and this reader answers both.
        assert!(cargo_invocations(yaml).contains(&"cargo xtask std-test".to_string()));
    }

    /// The marker binds to its own step and does not leak to the next
    /// one. This is the assertion that would have failed loudest if the
    /// parse had stayed line-at-a-time.
    #[test]
    fn the_marker_does_not_leak_past_its_step() {
        let yaml = "\
\x20     - name: advisory\n\
\x20       continue-on-error: true\n\
\x20       run: cargo xtask std-test\n\
\x20     - name: next\n\
\x20       run: cargo xtask ulp\n\
";
        let adv = advisory_invocations(yaml);
        assert_eq!(adv.len(), 1, "{adv:?}");
        assert_eq!(adv[0].0, "cargo xtask std-test");
    }

    /// Order within a step does not matter: YAML keys are a mapping, and
    /// `run:` above `continue-on-error:` is the same step.
    #[test]
    fn the_marker_is_found_below_its_own_run() {
        let yaml = "\
\x20     - name: advisory\n\
\x20       run: cargo xtask std-test\n\
\x20       continue-on-error: true\n\
";
        assert_eq!(
            advisory_invocations(yaml),
            vec![("cargo xtask std-test".to_string(), "true".to_string())]
        );
    }

    /// A JOB-level marker makes every step of that job advisory. The
    /// dumb reader cannot scope it to one job, so it marks the whole
    /// file — over-reporting, which reds the selftest. Under-reporting
    /// would leave a step gating nothing with the gate green, which is
    /// wolf-std#34 itself.
    #[test]
    fn a_job_level_marker_over_reports_rather_than_missing() {
        let yaml = "\
jobs:\n\
\x20 nightly:\n\
\x20   continue-on-error: true\n\
\x20   steps:\n\
\x20     - name: one\n\
\x20       run: cargo xtask doctor\n\
\x20     - name: two\n\
\x20       run: cargo xtask ulp\n\
";
        let adv = advisory_invocations(yaml);
        assert_eq!(
            adv.len(),
            2,
            "a job-level marker covers every step: {adv:?}"
        );
        assert!(adv.iter().all(|(_, e)| e == "true"));
    }

    /// Prose about `continue-on-error` is not a marker. `ci.yml` carries
    /// four such comment lines from sc41, sc42 and sc45, and every one of
    /// them would be a false advisory reading if comments were not
    /// dropped first.
    #[test]
    fn prose_about_the_marker_is_not_the_marker() {
        let yaml = "\
\x20     - name: fmt-lu\n\
\x20       # A `continue-on-error` on it would reproduce exactly the\n\
\x20       # thing being fixed. continue-on-error: true\n\
\x20       run: cargo xtask fmt-lu\n\
";
        assert!(advisory_invocations(yaml).is_empty());
        assert_eq!(cargo_invocations(yaml), vec!["cargo xtask fmt-lu"]);
    }

    /// The file this repository actually ships: exactly one advisory
    /// step, and it is the blessed one. `selftest.rs` gates the real
    /// file; this pins the shape the reader expects to find in it.
    #[test]
    fn the_blessed_list_is_well_formed() {
        assert!(ADVISORY_STEPS
            .iter()
            .all(|a| a.command.starts_with("cargo ")
                && !a.expr.is_empty()
                && a.issue.contains('#')));
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
