//! Rig self-tests over the real repository contents. Hermetic ones run
//! everywhere; binary-needing ones SKIP loudly (never silently) when no
//! implementation is on any resolution rung.

use crate::bins::{self, Impl};
use crate::record::Verdict;
use crate::repo_root;
use crate::{anchors, directive, exec, ledger, record, stage, workflow};
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

/// **wolf-std#8's gate.** `std.net.accept`'s clause and upstream's
/// `[os.net.accept]` describe the same call from two repositories, and
/// for one release they disagreed: the clause was written at sc08 ("no
/// non-blocking accept and no way to poll … waits forever"), sc37 landed
/// `net.wait` thirty lines below it in the same file, and wolf-lang#242
/// bounded the park at v0.2.6. Nothing could see the drift — the words
/// are prose, and prose is what no gauntlet reads — so lobo's ws18 found
/// it by writing a prefork server against it.
///
/// This holds the four things the corrected clause must keep saying, and
/// the two retracted sentences it must never say again. It is a LINT on
/// wording, which is unusual here and deliberate: the alternative is the
/// clause rotting silently a second time. If a future edit rewords it
/// legitimately, this test changes in that same commit — which is the
/// point, because then a human has read both halves together.
///
/// The registry half is the other direction: `os.net.accept` must still
/// be a published anchor at the pin. If upstream ever tombstones or
/// renumbers it, `vendor/upstream/anchors.json` changes at the bump and
/// this reds there rather than leaving a citation pointing at nothing.
#[test]
fn net_accept_clause_agrees_with_os_net_accept() {
    let repo = repo_root();
    let src = std::fs::read_to_string(repo.join("std/net/net.lu")).unwrap();
    let start = src
        .find("pub fn accept(")
        .expect("std/net/net.lu: no `pub fn accept(`");
    // The doc block is everything from the previous function's closing
    // brace to the signature; `///` lines only.
    let head = &src[..start];
    let clause: String = head
        .lines()
        .rev()
        .take_while(|l| l.trim_start().starts_with("///") || l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!clause.is_empty(), "accept's doc block did not parse");

    for retracted in ["no way to poll", "waits forever"] {
        assert!(
            !clause.contains(retracted),
            "std.net.accept's clause says {retracted:?} — wolf-std#8. \
             `net.wait` polls a listener (sc37) and an armed \
             `set_listener_deadline` bounds the park ([os.net.accept], \
             wolf-lang#242)."
        );
    }
    for required in [
        "[os.net.accept]", // the upstream clause it must agree with
        "net.wait",        // the poll answer, by name
        "BOUNDS",          // an armed deadline bounds the call
        "not a claim",     // a readiness wake is not exclusive
        "same budget",     // a lost race re-waits inside the armed budget
    ] {
        assert!(
            clause.contains(required),
            "std.net.accept's clause no longer says {required:?} — \
             wolf-std#8 asked for it and `[os.net.accept]` still says it. \
             If the clause was reworded deliberately, move this needle in \
             the SAME commit."
        );
    }

    let text = std::fs::read_to_string(repo.join("vendor/upstream/anchors.json")).unwrap();
    let reg = anchors::Registry::from_json(&text).unwrap();
    assert!(
        matches!(
            reg.classify("os.net.accept"),
            Ok(anchors::TagClass::Registered)
        ),
        "`os.net.accept` is not a registered anchor at this pin — the \
         clause above cites it and tests/net/accept_bounded_and_pollable.lu \
         tags it"
    );
}

/// **wolf-std#9's gate, and the second use of sc38's shape.** The first
/// retracted-sentence lint was written after a clause went stale for a
/// whole sprint with the call that falsified it thirty lines below.
/// `net.adopt_listener`'s clause went stale the same way and faster: it
/// said "There is no `std.process` half of this pair at this pin", which
/// was true when sc37 wrote it and false the moment `process.start_with`
/// landed. Nothing could have seen it — the two halves are in different
/// modules and no test reads either sentence.
///
/// So the pair is linted from BOTH ends, which is the half sc38's lint
/// did not have: each clause must name the other's function. A future
/// lane that deletes one half has to answer for the other's doc in the
/// same commit, and a legitimate rewording moves these needles where a
/// human reads both together.
#[test]
fn the_inherit_pair_names_itself_from_both_ends() {
    let repo = repo_root();

    let net = std::fs::read_to_string(repo.join("std/net/net.lu")).unwrap();
    let adopt = doc_block_before(&net, "pub fn adopt_listener(");
    assert!(
        !adopt.contains("no `std.process` half"),
        "std.net.adopt_listener's clause still says there is no \
         `std.process` half — `process.start_with` is that half \
         (wolf-std#9, sc39). If the wrapper was WITHDRAWN, this needle \
         moves in the same commit that withdraws it."
    );
    for required in ["process.start_with", "borrows"] {
        assert!(
            adopt.contains(required),
            "std.net.adopt_listener's clause no longer says {required:?} — \
             it is the CHILD half of a pair and must name its parent half, \
             or the next reader writes the handshake through the builtin \
             again (which is exactly what wolf-std#9 was)."
        );
    }

    let proc = std::fs::read_to_string(repo.join("std/process/process.lu")).unwrap();
    let start_with = doc_block_before(&proc, "pub fn start_with(");
    for required in [
        "net.adopt_listener",      // the child half, by name
        "[os.proc.inherit]",       // the clause both halves conform to
        "3, 4, …",                 // the numbering that IS the contract
        "BORROWS",                 // why the set is on the spawn, not the builder
        "EMPTY set",               // the case served on every host and tier
        "BEFORE any child exists", // a bad set spawns nothing
    ] {
        assert!(
            start_with.contains(required),
            "std.process.start_with's clause no longer says {required:?} — \
             each of these is a promise `[os.proc.inherit]` makes that a \
             caller cannot discover from the signature. Reword deliberately \
             and move the needle here."
        );
    }

    let text = std::fs::read_to_string(repo.join("vendor/upstream/anchors.json")).unwrap();
    let reg = anchors::Registry::from_json(&text).unwrap();
    assert!(
        matches!(
            reg.classify("os.proc.inherit"),
            Ok(anchors::TagClass::Registered)
        ),
        "`os.proc.inherit` is not a registered anchor at this pin — both \
         clauses cite it and both witnesses tag it"
    );
}

/// The `///` doc block immediately above `sig` in `src`.
fn doc_block_before(src: &str, sig: &str) -> String {
    let start = src.find(sig).unwrap_or_else(|| panic!("no `{sig}`"));
    let block: String = src[..start]
        .lines()
        .rev()
        .take_while(|l| l.trim_start().starts_with("///") || l.trim().is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!block.is_empty(), "`{sig}`'s doc block did not parse");
    block
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

/// sc41 (F-0113, wolf-std#13): the two lists, diffed, on every host.
///
/// `cargo xtask ci` ran eleven steps and `.github/workflows/ci.yml` ran
/// seven, for four sprints, and the only reason anyone found out was a
/// doc example that broke and stayed green on three platforms. This is
/// the gate that makes the next divergence a red `cargo test` instead of
/// a finding. It lives in `selftest` on purpose: the `rig` job was
/// ALREADY required on all three hosts, so the drift check is required
/// from the moment it lands, without waiting on the lane it describes.
#[test]
fn every_ci_step_runs_in_the_ci_workflow() {
    let repo = repo_root();
    let path = repo.join(workflow::CI_WORKFLOW);
    let yaml =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", workflow::CI_WORKFLOW));
    let invocations = workflow::cargo_invocations(&yaml);
    assert!(
        !invocations.is_empty(),
        "{} parsed to no cargo invocations at all — the reader is broken, \
         not the workflow",
        workflow::CI_WORKFLOW
    );
    let missing = workflow::local_only(&yaml);
    assert!(
        missing.is_empty(),
        "these `cargo xtask ci` steps run in NO workflow: {missing:?}\n  \
         a step no runner executes is a gate the merge does not have \
         (F-0113: that is how a broken `net.write` doc example was green on \
         ubuntu, macOS and windows for four sprints).\n  \
         Add it to {} or take it out of workflow::CI_STEPS.",
        workflow::CI_WORKFLOW
    );
    let extra = workflow::workflow_only(&yaml);
    assert!(
        extra.is_empty(),
        "{} runs cargo commands the local gauntlet does not: {extra:?}\n  \
         the worse direction of the same drift — a check the author cannot \
         reproduce before pushing. Add it to workflow::CI_STEPS.",
        workflow::CI_WORKFLOW
    );
}

/// Every `continue-on-error:` in `ci.yml` is a BLESSED advisory step.
///
/// sc47 / wolf-std#34. `every_ci_step_runs_in_the_ci_workflow` above
/// asks whether a step RUNS on a runner. It cannot ask whether the step
/// GATES anything, and for four sprints `std-test` answered yes to the
/// first question and no to the second on two of three hosts: it was RED
/// on `rig (windows-latest)` from sc43 onward and the job reported
/// success. Two lanes read that green and wrote it into wolf-std#20 as a
/// windows pass, twice.
///
/// Reading harder does not fix it. At run 34672768728 the REST API
/// reports `conclusion: success` for the `std-test` STEP on
/// `rig (windows-latest)` as well as for the job, while the same step's
/// log ends `xtask: RED` — `continue-on-error` rewrites the conclusion at
/// both levels. The durable fact is the marker itself, in the workflow
/// file, so that is what this gates.
///
/// The rule: an advisory step must be named in
/// `workflow::ADVISORY_STEPS` with the expression it carries and the
/// open issue that retires it. Adding a marker, widening one to another
/// host, or leaving one behind after its issue closes all red the `rig`
/// job on all three hosts — and red `cargo test` on the author's box
/// before the push, which is where wolf-std#34 should have been caught.
#[test]
fn every_advisory_step_is_blessed_with_an_issue() {
    let repo = repo_root();
    let path = repo.join(workflow::CI_WORKFLOW);
    let yaml =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", workflow::CI_WORKFLOW));

    let found: Vec<(String, String)> = workflow::advisory_invocations(&yaml);
    let blessed: Vec<(String, String)> = workflow::ADVISORY_STEPS
        .iter()
        .map(|a| (a.command.to_string(), a.expr.to_string()))
        .collect();

    for (command, expr) in &found {
        assert!(
            blessed.contains(&(command.clone(), expr.clone())),
            "{}: `{command}` carries `continue-on-error: {expr}` and is not in \
             workflow::ADVISORY_STEPS.\n  \
             An advisory step RUNS and GATES NOTHING on every host where that \
             expression is true, and GitHub rewrites its conclusion to \
             `success` at the step level as well as the job level — so nothing \
             downstream can tell. Bless it with the issue that retires it, or \
             take the marker off.",
            workflow::CI_WORKFLOW
        );
    }
    for a in workflow::ADVISORY_STEPS {
        assert!(
            found.contains(&(a.command.to_string(), a.expr.to_string())),
            "workflow::ADVISORY_STEPS says `{}` is advisory with \
             `continue-on-error: {}` ({}), and {} does not carry that.\n  \
             If the step went REQUIRED, drop the entry here and close the \
             issue; a stale blessing is a licence nobody is using.",
            a.command,
            a.expr,
            a.issue,
            workflow::CI_WORKFLOW
        );
    }
    for a in workflow::ADVISORY_STEPS {
        assert!(
            a.issue.starts_with("wolf-std#") || a.issue.starts_with("wolf-lang#"),
            "advisory step `{}` is blessed with `{}`, which is not an issue \
             reference — an advisory step nobody has filed against is a red \
             nobody is going to fix",
            a.command,
            a.issue
        );
    }
}

/// Every name in `workflow::CI_STEPS` has a match arm in `run_step`.
///
/// `ci()` iterates the list, so an entry with no arm would fail the
/// gauntlet at that banner — correct, but 20 minutes in and only on the
/// box that runs the gauntlet. This says it at `cargo test` time, on
/// every host, by reading the source the arms live in. Cheap, and the
/// only cross-check between the list and the code that serves it.
#[test]
fn every_ci_step_has_a_runner() {
    let src = std::fs::read_to_string(repo_root().join("xtask/src/main.rs")).unwrap();
    let body = src
        .split_once("fn run_step(")
        .expect("run_step moved — this test reads it by name")
        .1;
    for step in workflow::CI_STEPS {
        let arm = format!("\"{}\" =>", step.name);
        assert!(
            body.contains(&arm),
            "workflow::CI_STEPS names `{}` but run_step has no `{arm}` arm",
            step.name
        );
    }
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
