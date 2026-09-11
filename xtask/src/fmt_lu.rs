//! `cargo xtask fmt-lu` — D34's gate (sc45, F-0117, wolf-std#21).
//!
//! "`wolf fmt` is law for every committed `.lu` here" has been asserted
//! in commit messages since sc02 and enforced by nobody. `cargo fmt
//! --all --check` gates the RUST; until this module there was no step in
//! `cargo xtask ci` or in `.github/workflows/ci.yml` that gated the
//! WOLF. Measured at sc43 (wolf 0.2.10), re-measured at sc45 (wolf
//! 0.2.11) and at 0.2.9 before either: the SAME thirty-two committed
//! files were not fixed points of the formatter, at three pins, sorted
//! lists equal line for line. Standing drift, never introduced by a
//! bump, invisible because nothing looked.
//!
//! This is F-0113's shape (four gauntlet steps that ran in no workflow)
//! and wolf-std#18's (a SKIP that could not tell absent from present):
//! a claim with no mechanism behind it. The mechanism is one process.
//!
//! **There is no exception list, and that is a measurement rather than a
//! policy.** wolf-lang's own corpus gate carries one — `//! fmt: relaid`
//! (`[gram.fmt.canon]`'s single declared exception, wolf-lang#276), for
//! corpus files that deliberately pin a spelling the formatter re-lays.
//! No file in THIS tree needs it: after sc45's re-lay every one of the
//! 452 committed `.lu` files is a fixed point, `wolf fmt` is idempotent
//! over the tree in one pass, and the set of exempt files is empty. The
//! day a file here must pin a non-canonical layout, the mechanism to
//! copy is wolf-lang's directive, not a path list in this file.
//!
//! The tree walk is deliberately the rig's own (the same shape
//! `runner::collect_lu` uses) rather than handing `wolf fmt` the two
//! directory names: the step then knows, and prints, HOW MANY files it
//! stood over. A gate that cannot say what it covered is how D34 got
//! here.

use crate::bins::{self, Impl};
use crate::exec;
use crate::repo_root;
use crate::stage;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The two roots every committed `.lu` file in this repository lives
/// under. Checked against `git ls-files '*.lu'` at sc45: 51 under
/// `std/`, 401 under `tests/`, none anywhere else.
const ROOTS: [&str; 2] = ["std", "tests"];

/// Files per `wolf fmt --check` invocation. The whole tree fits in one
/// command line on this host today (452 paths, ~14 KB), but windows caps
/// a command line at 32,767 characters and this list only grows — so the
/// step batches, and the batch size is the reason it will not be a
/// mystery failure three sprints from now. Six invocations at 0.2 s each
/// is not a cost worth optimizing against a cliff.
const BATCH: usize = 64;

pub fn fmt_lu() -> Result<(), String> {
    let repo = repo_root();
    let files = collect(&repo)?;
    if files.is_empty() {
        return Err(format!(
            "fmt-lu: no `.lu` file under {} — the walk is broken, not the tree",
            roots_as("or")
        ));
    }
    let Some(wolf) = bins::resolve(Impl::Wolf, &repo) else {
        println!(
            "fmt-lu: wolf — ABSENT (tried $WOLF_BIN, .wolf-bin/wolf, PATH)\n        \
             SKIP: {} committed `.lu` file(s) UNCHECKED. D34 is not enforced on \
             this box; acquire the pinned wolf and re-run.",
            files.len()
        );
        return Ok(());
    };
    let mut drift = Vec::new();
    for batch in files.chunks(BATCH) {
        let mut cmd = Command::new(&wolf.path);
        cmd.arg("fmt").arg("--check").current_dir(&repo);
        for f in batch {
            cmd.arg(f);
        }
        let got = exec::run(cmd, Duration::from_secs(exec::timeout_secs()))?;
        if got.timed_out {
            return Err(format!(
                "fmt-lu: `wolf fmt --check` blew the {}s ceiling on a batch of {}",
                exec::timeout_secs(),
                batch.len()
            ));
        }
        match got.status {
            // 0: every file in the batch is a fixed point.
            Some(0) => {}
            // 1: at least one is not, and the tool names each on its own
            // line. Collected rather than returned, so ONE run reports
            // the whole tree the way sc43's measurement did.
            Some(1) => drift.extend(
                got.stdout
                    .lines()
                    .chain(got.stderr.lines())
                    .filter_map(named)
                    .map(str::to_string),
            ),
            other => {
                return Err(format!(
                    "fmt-lu: `wolf fmt --check` exited {other:?} (neither 0 nor 1)\n{}\n{}",
                    got.stdout.trim_end(),
                    got.stderr.trim_end()
                ));
            }
        }
    }
    if drift.is_empty() {
        let at = bins::probe_version(&wolf.path)
            .map(|v| format!("{} {}", v.name, v.version))
            .unwrap_or_else(|_| stage::show(&wolf.path));
        println!(
            "fmt-lu: {} committed `.lu` file(s) under {}, every one a fixed point \
             of `wolf fmt` at {at}",
            files.len(),
            roots_as("and"),
        );
        return Ok(());
    }
    Err(format!(
        "fmt-lu: {} of {} committed `.lu` file(s) are not fixed points of `wolf fmt`:\n  {}\n\
         D34: `wolf fmt` is law for every committed `.lu` here. Run `wolf fmt {}` \
         and commit the re-lay; if a file must pin a layout the formatter re-lays, \
         that is a wolf-lang filing (the formatter's), not a path to add here.",
        drift.len(),
        files.len(),
        drift.join("\n  "),
        ROOTS.join(" "),
    ))
}

/// [`ROOTS`] as prose, each with the trailing slash that says it is a
/// directory: `std/ and tests/`.
fn roots_as(conj: &str) -> String {
    let with_slash: Vec<String> = ROOTS.iter().map(|r| format!("{r}/")).collect();
    match with_slash.split_last() {
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} {conj} {last}", rest.join(", ")),
        None => String::new(),
    }
}

/// The file the tool named, out of one `--check` line. The spelling is
/// `wolf fmt --check: <path> is not canonically formatted`; a line the
/// rig cannot parse is kept whole rather than dropped, because a gate
/// that silently discards the tool's complaint is the failure mode this
/// module exists to end.
fn named(line: &str) -> Option<&str> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    Some(
        line.strip_prefix("wolf fmt --check:")
            .map(str::trim)
            .and_then(|r| r.strip_suffix("is not canonically formatted"))
            .map(str::trim)
            .unwrap_or(line),
    )
}

/// Every `.lu` under [`ROOTS`], repo-relative, sorted, deterministic.
fn collect(repo: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for root in ROOTS {
        walk(repo, &repo.join(root), &mut out)?;
    }
    out.sort();
    Ok(out)
}

fn walk(repo: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("fmt-lu: read {}: {e}", stage::show(dir)))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            walk(repo, &path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("lu") {
            out.push(
                path.strip_prefix(repo)
                    .map_err(|_| format!("fmt-lu: {} is not under the repo", stage::show(&path)))?
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The walk finds the tree, under both roots, and finds nothing that
    /// is not a `.lu`.
    #[test]
    fn the_walk_covers_both_roots() {
        let files = collect(&repo_root()).unwrap();
        assert!(files.iter().all(|f| f.ends_with(".lu")), "{files:?}");
        assert!(files.iter().any(|f| f.starts_with("std/")));
        assert!(files.iter().any(|f| f.starts_with("tests/")));
        // Sorted and unique — the batches must be reproducible.
        let mut sorted = files.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, files);
    }

    /// wolf-std#21's own output, verbatim, parses back to the path.
    #[test]
    fn a_check_line_parses_to_the_path() {
        assert_eq!(
            named("wolf fmt --check: std/list/list.lu is not canonically formatted"),
            Some("std/list/list.lu")
        );
        // An unrecognized complaint is kept, never dropped.
        assert_eq!(
            named("something else entirely"),
            Some("something else entirely")
        );
        assert_eq!(named("   "), None);
    }

    /// Batching is what keeps the step off the windows command-line
    /// cliff, so it is the part that must not lose a file silently.
    #[test]
    fn batching_covers_every_file_exactly_once() {
        const { assert!(BATCH > 0) };
        let files = collect(&repo_root()).unwrap();
        let batched: Vec<&String> = files.chunks(BATCH).flatten().collect();
        assert_eq!(batched.len(), files.len(), "chunking lost files");
        assert!(
            batched.into_iter().eq(files.iter()),
            "chunking reordered files"
        );
        assert!(
            files
                .chunks(BATCH)
                .all(|b| !b.is_empty() && b.len() <= BATCH),
            "a batch is empty or over the cap"
        );
    }
}
