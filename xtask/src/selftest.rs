//! Rig self-tests over the real repository contents. Hermetic ones run
//! everywhere; binary-needing ones SKIP loudly (never silently) when no
//! implementation is on any resolution rung.

use crate::bins::{self, Impl};
use crate::record::Verdict;
use crate::repo_root;
use crate::{anchors, directive, exec, ledger, record, stage};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[test]
fn pins_are_well_formed() {
    let repo = repo_root();
    let pin = std::fs::read_to_string(repo.join("vendor/upstream/PIN")).unwrap();
    assert!(bins::is_full_sha(pin.trim()), "PIN: {pin:?}");
    let (lupin, wolf) = bins::load_tool_pins(&repo).unwrap();
    assert!(bins::is_full_sha(&lupin.pin) && bins::is_full_sha(&wolf.pin));
    let anchors_text = std::fs::read_to_string(repo.join("vendor/upstream/anchors.json")).unwrap();
    anchors::Registry::from_json(&anchors_text).unwrap();
}

#[test]
fn every_test_is_ledgered_and_entry_shaped() {
    let repo = repo_root();
    let text = std::fs::read_to_string(repo.join("tests/ledger.toml")).unwrap();
    let ledger = ledger::parse(&text, "tests/ledger.toml").unwrap();
    let mut found = 0;
    for path in walk_lu(repo.join("tests")) {
        let rel = path
            .strip_prefix(repo.join("tests"))
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let src = std::fs::read_to_string(&path).unwrap();
        let d = directive::parse(&src, &rel).unwrap();
        d.validate_entry(&rel).unwrap();
        assert!(ledger.contains_key(&rel), "no ledger entry for tests/{rel}");
        found += 1;
    }
    assert_eq!(found, ledger.len(), "ledger names files that do not exist");
    assert!(
        found > 0,
        "the rig must always carry at least one staged test"
    );
}

#[test]
fn std_tree_files_are_members() {
    // The layout law (D32): everything under std/ is module mass, staged
    // through an entry — `member: true`, never `check:`/`phase:`
    // ([conf.directive.member]).
    let repo = repo_root();
    for path in walk_lu(repo.join("std")) {
        let rel = stage::show(&path);
        let src = std::fs::read_to_string(&path).unwrap();
        let d = directive::parse(&src, &rel).unwrap();
        assert_eq!(
            d.member,
            Some(true),
            "{rel}: std files carry `member: true`"
        );
        assert!(
            d.check.is_none() && d.phase.is_none(),
            "{rel}: member files carry neither check: nor phase:"
        );
    }
}

/// The staging round-trip the rig exists to prove: the exemplar entry
/// FAILS to resolve its module in place (no std root, and the package
/// root — the entry file's directory — holds no modules), and runs green
/// once staged and rooted. Since sc04 the interpreter takes
/// `--std-root` like the compiler (wolf-interp#6, F-0010 closed), so the
/// staged tree is pointed at rather than mirrored. Needs lupin; SKIPs
/// loudly without it.
#[test]
fn staging_round_trip_under_lupin() {
    let repo = repo_root();
    let Some(lupin) = bins::resolve(Impl::Lupin, &repo) else {
        eprintln!("SKIP: no lupin on any resolution rung — staging round-trip not exercised");
        return;
    };
    let entry = repo.join("tests/prelude/prelude_smoke.lu");

    // Unstaged and unrooted: must NOT satisfy its directive.
    let unstaged = conform(&lupin.path, &entry, None);
    assert_eq!(
        unstaged.verdict,
        Verdict::Unsupported,
        "entry-beside-tree must fail without staging (got {})",
        unstaged.verdict
    );

    // Staged: green, stdout hash checked.
    let scratch = repo.join("target/stage-selftest");
    let staged = stage::stage_test(&entry, &repo.join("std"), &scratch).unwrap();
    // One staged tree, one root, all three lanes.
    assert!(staged.std_root.join("prelude/prelude.lu").is_file());
    assert!(
        !scratch.join("prelude").exists(),
        "the flat mirror is retired: nothing is duplicated beside std/"
    );
    let rec = conform(&lupin.path, &staged.entry, Some(&staged.std_root));
    assert_eq!(rec.verdict, Verdict::Exit(0), "staged exemplar runs green");
    let src = std::fs::read_to_string(&entry).unwrap();
    let d = directive::parse(&src, "prelude_smoke.lu").unwrap();
    match d.check.unwrap() {
        directive::Check::Run {
            stdout: Some(expected),
            ..
        } => {
            assert!(record::stdout_matches(&rec, &expected), "stdout hash");
        }
        other => panic!("exemplar should expect run+stdout, has {other:?}"),
    }
}

fn conform(bin: &PathBuf, entry: &PathBuf, std_root: Option<&std::path::Path>) -> record::Record {
    let mut cmd = Command::new(bin);
    cmd.arg("conform-run");
    if let Some(root) = std_root {
        cmd.arg("--std-root").arg(root.as_os_str());
    }
    cmd.arg(entry).arg("--json");
    let got = exec::run(cmd, Duration::from_secs(exec::timeout_secs())).unwrap();
    assert_eq!(got.status, Some(0), "tool error: {}", got.stderr);
    record::parse(&got.stdout, "lupin").unwrap()
}

fn walk_lu(dir: PathBuf) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir];
    while let Some(d) = stack.pop() {
        let mut entries: Vec<PathBuf> = std::fs::read_dir(&d)
            .unwrap_or_else(|e| panic!("read {}: {e}", stage::show(&d)))
            .map(|e| e.unwrap().path())
            .collect();
        entries.sort();
        for p in entries {
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().and_then(|e| e.to_str()) == Some("lu") {
                out.push(p);
            }
        }
    }
    out
}
