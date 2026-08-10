//! `cargo xtask std-test` — the dual-implementation loop. Every entry
//! test is staged, then observed under both `lupin conform-run` and
//! `wolf conform-run --checked` (`[proto.invoke]`); records are compared
//! against the directive, the ledger, and — where both implementations
//! reach run — against each other (`[proto.cmp]`-style). A divergence on
//! std code is a finding for the upstream divergence log, and the reason
//! this track runs two machines.

use crate::bins::{self, Impl};
use crate::directive::{self, Check, ExitExpect};
use crate::exec;
use crate::ledger::{self, Expect};
use crate::record::{self, Record, Verdict};
use crate::stage;
use crate::{anchors, repo_root};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// What one implementation achieved on one test, in ledger vocabulary.
#[derive(Debug, Clone, PartialEq)]
enum Achieved {
    /// The record satisfies the directive's `check:` in full ("run").
    Satisfies,
    Fail(String),
    Unsupported,
}

impl Achieved {
    fn as_expect(&self) -> Expect {
        match self {
            Achieved::Satisfies => Expect::Run,
            Achieved::Fail(c) => Expect::Fail(c.clone()),
            Achieved::Unsupported => Expect::Unsupported,
        }
    }
}

pub fn ledger_check() -> Result<(), String> {
    let repo = repo_root();
    let (tests, ledger) = load_tests_and_ledger(&repo)?;
    let mut errors = Vec::new();
    for t in &tests {
        if !ledger.contains_key(t) {
            errors.push(format!(
                "tests/{t} has no ledger entry — record both columns"
            ));
        }
    }
    for k in ledger.keys() {
        if !tests.contains(k) {
            errors.push(format!("ledger entry `{k}` names no file under tests/"));
        }
    }
    if errors.is_empty() {
        println!("ledger-check: {} test(s), all ledgered", tests.len());
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}

fn load_tests_and_ledger(repo: &Path) -> Result<(Vec<String>, ledger::Ledger), String> {
    let ledger_path = repo.join("tests/ledger.toml");
    let text =
        std::fs::read_to_string(&ledger_path).map_err(|e| format!("tests/ledger.toml: {e}"))?;
    let ledger = ledger::parse(&text, "tests/ledger.toml")?;
    let mut tests = Vec::new();
    collect_lu(&repo.join("tests"), &repo.join("tests"), &mut tests)?;
    tests.sort();
    Ok((tests, ledger))
}

fn collect_lu(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("read {}: {e}", stage::show(dir)))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_lu(root, &path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("lu") {
            let rel = path
                .strip_prefix(root)
                .map_err(|_| "strip_prefix".to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            out.push(rel);
        }
    }
    Ok(())
}

pub fn std_test() -> Result<(), String> {
    let repo = repo_root();
    let (tests, ledger) = load_tests_and_ledger(&repo)?;
    ledger_check()?;
    let registry = anchors::Registry::load(&repo)?;
    let (lupin_pin, wolf_pin) = bins::load_tool_pins(&repo)?;

    let mut lanes = Vec::new();
    for (imp, pin) in [(Impl::Lupin, &lupin_pin), (Impl::Wolf, &wolf_pin)] {
        match bins::resolve(imp, &repo) {
            Some(r) => {
                println!(
                    "lane {}: {} (source: {})",
                    imp.ledger_name(),
                    stage::show(&r.path),
                    r.source
                );
                lanes.push((imp, Some(r)));
            }
            None => {
                // Silence is what turns a lane dark (ls00): the SKIP is
                // structured, names the pin, and says how to light it.
                println!(
                    "SKIP: no {} at pin {} — lane dark; set ${} or place \
                     .wolf-bin/{} (tried ${}, .wolf-bin, PATH)",
                    imp.name(),
                    &pin.pin[..7.min(pin.pin.len())],
                    imp.env_var(),
                    imp.name(),
                    imp.env_var()
                );
                lanes.push((imp, None));
            }
        }
    }

    let ceiling = Duration::from_secs(exec::timeout_secs());
    let mut reds: Vec<String> = Vec::new();
    let mut forward_tags = 0usize;
    let mut conservatism: Vec<String> = Vec::new();
    let mut divergences: Vec<serde_json::Value> = Vec::new();
    let mut ran = 0usize;

    for test in &tests {
        let entry = repo.join("tests").join(test);
        let src = std::fs::read_to_string(&entry).map_err(|e| format!("tests/{test}: {e}"))?;
        let d = directive::parse(&src, &format!("tests/{test}"))?;
        d.validate_entry(&format!("tests/{test}"))?;
        for tag in &d.conforms {
            match registry.classify(tag) {
                Ok(anchors::TagClass::Forward) => forward_tags += 1,
                Ok(anchors::TagClass::Registered) => {}
                Err(e) => reds.push(format!("tests/{test}: {e}")),
            }
        }
        let check = d.check.clone().expect("validated");
        let phase = d.phase.clone().expect("validated");

        let scratch = repo.join("target/stage").join(test.replace('/', "__"));
        let staged = stage::stage_test(&entry, &repo.join("std"), &scratch)?;
        let expect = &ledger[test];

        let mut runtime_records: Vec<(Impl, Record)> = Vec::new();
        for (imp, resolved) in &lanes {
            let Some(resolved) = resolved else { continue };
            let rec = match invoke(*imp, &resolved.path, &staged, &check, &phase, ceiling) {
                Ok(rec) => rec,
                Err(e) => {
                    reds.push(format!("tests/{test} [{}]: {e}", imp.ledger_name()));
                    continue;
                }
            };
            let achieved = match classify(&rec, &check) {
                Ok(a) => a,
                Err(mismatch) => {
                    reds.push(format!(
                        "tests/{test} [{}]: directive mismatch — {mismatch}",
                        imp.ledger_name()
                    ));
                    continue;
                }
            };
            if matches!(
                rec.verdict,
                Verdict::Exit(_) | Verdict::Trap(_) | Verdict::Ub(_)
            ) {
                runtime_records.push((*imp, rec.clone()));
            }
            let want = match imp {
                Impl::Lupin => &expect.lupin,
                Impl::Wolf => &expect.wolfc,
            };
            let got = achieved.as_expect();
            if got != *want {
                let direction = match ledger::depth(&got).cmp(&ledger::depth(want)) {
                    std::cmp::Ordering::Greater => {
                        "deeper than the ledger claims — advancement is deliberate: \
                         update tests/ledger.toml in its own commit"
                    }
                    std::cmp::Ordering::Less => {
                        "shallower than the ledger claims — a regression or an \
                         upstream behavior change; investigate before touching the ledger"
                    }
                    std::cmp::Ordering::Equal => "same depth, different outcome",
                };
                reds.push(format!(
                    "tests/{test} [{}]: ledger says `{want}`, observed `{got}` ({direction})",
                    imp.ledger_name()
                ));
            } else {
                match &got {
                    Expect::Unsupported => {
                        conservatism.push(format!("unsupported({}): {test}", imp.ledger_name()))
                    }
                    Expect::Fail(c) => {
                        conservatism.push(format!("rejects({}, {c}): {test}", imp.ledger_name()))
                    }
                    Expect::Run => {}
                }
            }
        }

        // Where both implementations reach run, diff the records
        // ([proto.cmp]): verdict; trap kind by name; stdout hash. `ub` on
        // one side where the other ran defined is the highest-severity
        // class ([proto.record.ub]).
        if let [(_, a), (_, b)] = &runtime_records[..] {
            let class = diff_class(a, b);
            if let Some(class) = class {
                divergences.push(serde_json::json!({
                    "file": format!("tests/{test}"),
                    "class": class,
                    "a": {"impl": a.impl_name, "verdict": a.verdict.to_string(),
                           "phase_reached": a.phase_reached,
                           "stdout_sha256": a.stdout_sha256},
                    "b": {"impl": b.impl_name, "verdict": b.verdict.to_string(),
                           "phase_reached": b.phase_reached,
                           "stdout_sha256": b.stdout_sha256},
                }));
                reds.push(format!(
                    "tests/{test}: DIVERGENCE ({class}) — {} says {}, {} says {} \
                     (a finding for the upstream divergence log)",
                    a.impl_name, a.verdict, b.impl_name, b.verdict
                ));
            }
        }
        ran += 1;
        println!(
            "test tests/{test}: staged, {} lane(s) observed",
            runtime_records.len()
        );
    }

    if !divergences.is_empty() {
        let path = repo.join("target/std-test-divergences.jsonl");
        let mut f = std::fs::File::create(&path)
            .map_err(|e| format!("write {}: {e}", stage::show(&path)))?;
        for d in &divergences {
            writeln!(f, "{d}").map_err(|e| e.to_string())?;
        }
        println!("divergence report: {}", stage::show(&path));
    }
    println!(
        "std-test: {ran} test(s); forward tags: {forward_tags}; \
         conservatism ledger: {} entr{}",
        conservatism.len(),
        if conservatism.len() == 1 { "y" } else { "ies" }
    );
    for line in &conservatism {
        println!("  {line}");
    }
    if reds.is_empty() {
        println!("std-test: GREEN");
        Ok(())
    } else {
        Err(reds.join("\n"))
    }
}

fn invoke(
    imp: Impl,
    bin: &Path,
    staged_entry: &Path,
    check: &Check,
    phase: &str,
    ceiling: Duration,
) -> Result<Record, String> {
    let mut cmd = Command::new(bin);
    cmd.arg("conform-run");
    if imp == Impl::Wolf {
        // The compiler's run rung is `--checked`-gated until s31
        // (miri-lite execution, budget-bounded; a blown budget reports an
        // honest `unsupported`, never a hang).
        cmd.arg("--checked");
    }
    // Absolute entry path: the package root is the entry file's
    // directory, and a bare relative name resolves to an empty root.
    cmd.arg(staged_entry.as_os_str());
    if matches!(check, Check::Pass) {
        // `pass` means "stopped at the requested phase, clean"
        // ([proto.record.verdict]) — only reachable with --phase.
        cmd.arg(format!("--phase={phase}"));
    }
    cmd.arg("--json");
    let got = exec::run(cmd, ceiling)?;
    if got.timed_out {
        return Err(format!(
            "timed out after {}s (ceiling; STD_TEST_TIMEOUT_SECS overrides)",
            ceiling.as_secs()
        ));
    }
    // [proto.invoke.exit]: 0 iff a well-formed record was produced; the
    // record — never the front-door exit code — is the interface.
    if got.status != Some(0) {
        return Err(format!(
            "tool error (exit {:?}, no record): {}",
            got.status,
            got.stderr.trim()
        ));
    }
    record::parse(&got.stdout, imp.ledger_name())
}

/// Compare a record against the directive's `check:`. `Ok` classifies in
/// ledger vocabulary; `Err` is a hard directive mismatch (the tool ran
/// the program and disagrees with what the test says happens).
fn classify(rec: &Record, check: &Check) -> Result<Achieved, String> {
    match (&rec.verdict, check) {
        (Verdict::Unsupported, _) => Ok(Achieved::Unsupported),
        (Verdict::Fail(code), Check::Fail(want)) => {
            if code == want {
                Ok(Achieved::Satisfies)
            } else {
                Err(format!("expected fail({want}), observed fail({code})"))
            }
        }
        (Verdict::Fail(code), _) => Ok(Achieved::Fail(code.clone())),
        (Verdict::Pass, Check::Pass) => Ok(Achieved::Satisfies),
        (
            Verdict::Exit(n),
            Check::Run {
                exit: ExitExpect::Code(want),
                stdout,
            },
        ) => {
            if n != want {
                return Err(format!("expected exit({want}), observed exit({n})"));
            }
            match stdout {
                Some(expected) if !record::stdout_matches(rec, expected) => Err(format!(
                    "stdout hash mismatch (expected sha256 of {expected:?} \
                     modulo one trailing newline, observed {:?})",
                    rec.stdout_sha256
                )),
                _ => Ok(Achieved::Satisfies),
            }
        }
        (
            Verdict::Trap(kind),
            Check::Run {
                exit: ExitExpect::Trap(want),
                ..
            },
        ) => match want {
            Some(w) if w != kind => Err(format!("expected trap({w}), observed trap({kind})")),
            _ => Ok(Achieved::Satisfies),
        },
        (got, want) => Err(format!("expected {want:?}, observed verdict {got}")),
    }
}

fn diff_class(a: &Record, b: &Record) -> Option<&'static str> {
    let a_ub = matches!(a.verdict, Verdict::Ub(_));
    let b_ub = matches!(b.verdict, Verdict::Ub(_));
    if a_ub != b_ub {
        return Some("soundness-candidate");
    }
    if a.verdict != b.verdict {
        return Some("verdict");
    }
    if matches!(a.verdict, Verdict::Exit(_)) && a.stdout_sha256 != b.stdout_sha256 {
        return Some("stdout");
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rec(verdict: Verdict, sha: Option<&str>) -> Record {
        Record {
            impl_name: "t".into(),
            phase_reached: "run".into(),
            verdict,
            stdout_sha256: sha.map(str::to_string),
            stdout_inline: None,
        }
    }

    #[test]
    fn classify_covers_the_matrix() {
        let run0 = Check::Run {
            exit: ExitExpect::Code(0),
            stdout: None,
        };
        assert_eq!(
            classify(&rec(Verdict::Unsupported, None), &run0).unwrap(),
            Achieved::Unsupported
        );
        assert_eq!(
            classify(&rec(Verdict::Fail("E1".into()), None), &run0).unwrap(),
            Achieved::Fail("E1".into())
        );
        assert_eq!(
            classify(&rec(Verdict::Exit(0), None), &run0).unwrap(),
            Achieved::Satisfies
        );
        assert!(classify(&rec(Verdict::Exit(1), None), &run0).is_err());
        let trap_any = Check::Run {
            exit: ExitExpect::Trap(None),
            stdout: None,
        };
        assert_eq!(
            classify(&rec(Verdict::Trap("bounds".into()), None), &trap_any).unwrap(),
            Achieved::Satisfies
        );
        let trap_kind = Check::Run {
            exit: ExitExpect::Trap(Some("overflow".into())),
            stdout: None,
        };
        assert!(classify(&rec(Verdict::Trap("bounds".into()), None), &trap_kind).is_err());
        let fail = Check::Fail("E0301".into());
        assert!(classify(&rec(Verdict::Fail("E9999".into()), None), &fail).is_err());
        assert_eq!(
            classify(&rec(Verdict::Fail("E0301".into()), None), &fail).unwrap(),
            Achieved::Satisfies
        );
    }

    #[test]
    fn diff_classes_rank() {
        let e0 = || rec(Verdict::Exit(0), Some("aa"));
        assert_eq!(diff_class(&e0(), &e0()), None);
        assert_eq!(
            diff_class(&e0(), &rec(Verdict::Exit(0), Some("bb"))),
            Some("stdout")
        );
        assert_eq!(
            diff_class(&e0(), &rec(Verdict::Exit(1), Some("aa"))),
            Some("verdict")
        );
        assert_eq!(
            diff_class(&e0(), &rec(Verdict::Ub("mem.ub".into()), None)),
            Some("soundness-candidate")
        );
    }
}
