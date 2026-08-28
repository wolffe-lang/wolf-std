//! `tests/ledger.toml` — per-test, per-implementation expectations (the
//! is00 corpus-overlay pattern). The two implementations diverge by
//! design (lupin runs what wolfc refuses, and vice versa), so each test
//! records what each implementation is expected to do *today*:
//! `lupin = "run" | "unsupported" | "slow" | "divergent(…)"`,
//! `wolfc = "run" | "unsupported" | "fail(E…)"`,
//! `native = "run" | "unsupported" | "fail(E…)"` — the compiler's second
//! rung (sc04: `conform-run --native`, s28's compile-link-execute), which
//! refuses a different set of shapes than the checked tier and therefore
//! earns its own column rather than hiding inside `wolfc`.
//! A test passing deeper than its ledger claims fails CI — advancement
//! is deliberate, in its own commit.

use crate::tomlite;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Expect {
    /// Satisfies its directive's `check:` in full.
    Run,
    Unsupported,
    /// Statically rejected with exactly this code (wolfc only).
    Fail(String),
    /// **The pin answers nondeterministically** — spelled
    /// `unstable(run|unsupported)` (sc07). The listed outcomes are the ones
    /// observed for the SAME program, same binary, same inputs, and any of
    /// them is accepted; anything else is red.
    ///
    /// **The lane's semantics reach it, its SPEED does not** — spelled
    /// `slow`, lupin-only (sc16, the digest ladder's honest-slow-skip).
    /// The runner does not invoke the lane at all: the program is inside
    /// the interpreter's modelled surface (the same module's short-vector
    /// files are `run` there, which is what keeps the claim honest), but a
    /// tree-walk of the full input blows the per-test ceiling by minutes,
    /// and a red-on-timeout would record the machine's speed, not its
    /// depth, while `unsupported` would be a lie about its semantics.
    /// Every `std-test` run prints a slow ledger naming each skipped row,
    /// so the entry is louder than a `run`, not quieter — and each row is
    /// owed a re-measure at pin bumps (an interpreter perf wave is the
    /// exit; F-0078's history is the precedent).
    Slow,
    /// **The lane EXECUTES the program and its honest observation cannot
    /// satisfy the directive** — spelled `divergent(trap(kind))`,
    /// `divergent(exit(N))` or `divergent(stdout)`, lupin-only (sc24, the
    /// first release whose interpreter carries the net/process tiers).
    /// Two shapes forced the word, both filed upstream the day they were
    /// measured: a handler over a BUILTIN-raised row takes its first arm
    /// under lupin 0.1.14 (F-0097 — the F-0079 mechanism at a third
    /// address), which costs two net row-witnesses their stdout; and the
    /// take-mode staleness discipline is STATIC on the compiler lanes
    /// (fail(E1001)) while the interpreter executes the same program to
    /// its honest dynamic outcome (F-0098 — trap(use-after-move), or the
    /// row path when execution diverts before the reuse). A directive
    /// speaks one expectation for three lanes, so a lane whose truthful
    /// answer differs needs its own word: `run` would be false,
    /// `unsupported` would lie about semantics, and a red would drown the
    /// signal that catches the NEXT change. The runner invokes the lane,
    /// demands EXACTLY the named observation (anything else is red — a
    /// heal shows up as a red the day it lands, and the flip to `run` is
    /// deliberate), keeps the record out of the cross-lane differ (the
    /// divergence is already filed; re-reporting it every run is noise),
    /// and prints a divergence ledger naming every such row, so the entry
    /// is louder than a `run`, not quieter. Each row's comment cites its
    /// finding.
    Divergent(DivObs),
    /// This is not a relaxation, it is a truthful record: at the sc07 pin
    /// two `str`-heavy tests get `run` or
    /// `unsupported — place projection outside the modelled surface` from
    /// the checked lane at random (measured 5/12 versus 7/12 on one file),
    /// and a ledger that claimed either one would fail CI at random too.
    /// F-0048 was the finding and it CLOSED at the sc08 pin: both rows were
    /// re-measured deterministic (14 consecutive runs each) and narrowed to
    /// `run`, so this variant is used by nothing in `tests/ledger.toml` today.
    /// It stays because the honest record of a nondeterministic pin is worth
    /// having ready, and because a row here is louder than a `run` row, not
    /// quieter: `std-test` prints an instability ledger and names the finding
    /// every run.
    Unstable(Vec<Expect>),
}

/// The exact observation a `divergent(…)` row expects from its lane.
#[derive(Debug, Clone, PartialEq)]
pub enum DivObs {
    /// The directive's exit code is reached and the stdout hash is NOT the
    /// directive's — the wrong-answer shape (F-0097's two rows).
    Stdout,
    /// The program runs to exactly this exit where the directive expects a
    /// static rejection (F-0098's row whose execution diverts before the
    /// reuse the compilers reject).
    Exit(i64),
    /// The program traps with exactly this kind where the directive
    /// expects a static rejection (F-0098's dynamic half).
    Trap(String),
}

impl std::fmt::Display for DivObs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DivObs::Stdout => write!(f, "stdout"),
            DivObs::Exit(n) => write!(f, "exit({n})"),
            DivObs::Trap(k) => write!(f, "trap({k})"),
        }
    }
}

impl std::fmt::Display for Expect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expect::Run => write!(f, "run"),
            Expect::Unsupported => write!(f, "unsupported"),
            Expect::Slow => write!(f, "slow"),
            Expect::Fail(c) => write!(f, "fail({c})"),
            Expect::Unstable(set) => write!(
                f,
                "unstable({})",
                set.iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join("|")
            ),
            Expect::Divergent(obs) => write!(f, "divergent({obs})"),
        }
    }
}

/// Depth order for the "deeper than the ledger claims" gate:
/// unsupported < fail < run.
pub fn depth(e: &Expect) -> u8 {
    match e {
        Expect::Unsupported => 0,
        Expect::Fail(_) => 1,
        Expect::Run => 2,
        // A slow row claims run-depth semantics (its fast siblings prove
        // it); the runner never observes the lane, so this depth is
        // documentation rather than a gate.
        Expect::Slow => 2,
        // An unstable row is as deep as its deepest outcome: a pin that
        // sometimes runs a program has at least that much capability, and
        // the gate must still catch a REGRESSION below the whole set.
        Expect::Unstable(set) => set.iter().map(depth).max().unwrap_or(0),
        // A divergent lane EXECUTES the program (run-depth reach); what it
        // answers is wrong or differently-shaped, which the runner checks
        // exactly. Documentation rather than a gate, like `slow`.
        Expect::Divergent(_) => 2,
    }
}

/// Does `got` satisfy `want`? Equality, except that an unstable row is
/// satisfied by any of its recorded outcomes.
pub fn satisfies(want: &Expect, got: &Expect) -> bool {
    match want {
        Expect::Unstable(set) => set.contains(got),
        _ => want == got,
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub lupin: Expect,
    pub wolfc: Expect,
    pub native: Expect,
}

pub type Ledger = BTreeMap<String, Entry>;

pub fn parse(text: &str, what: &str) -> Result<Ledger, String> {
    let mut ledger = Ledger::new();
    for section in tomlite::parse(text, what)? {
        if section.name != "tests" {
            return Err(format!("{what}: unknown section `[{}]`", section.name));
        }
        let Some(test) = section.key else {
            return Err(format!("{what}: `[tests]` needs a quoted test path"));
        };
        if test.contains('\\') {
            return Err(format!("{what}: `{test}` — ledger paths use `/` only"));
        }
        let mut lupin = None;
        let mut wolfc = None;
        let mut native = None;
        for (k, v, line) in &section.entries {
            let slot = match k.as_str() {
                "lupin" => &mut lupin,
                "wolfc" => &mut wolfc,
                "native" => &mut native,
                other => {
                    return Err(format!(
                        "{what}:{line}: unknown lane `{other}` (lupin, wolfc, native)"
                    ))
                }
            };
            let expect =
                parse_expect(v).ok_or_else(|| format!("{what}:{line}: bad expectation `{v}`"))?;
            if k == "lupin"
                && match &expect {
                    Expect::Fail(_) => true,
                    Expect::Unstable(set) => set.iter().any(|e| matches!(e, Expect::Fail(_))),
                    _ => false,
                }
            {
                return Err(format!(
                    "{what}:{line}: `fail(…)` is a wolfc expectation; lupin is \
                     run | unsupported | slow | divergent(…)"
                ));
            }
            if k != "lupin" && expect == Expect::Slow {
                return Err(format!(
                    "{what}:{line}: `slow` is a lupin expectation (the tree-walk's \
                     speed, sc16); a compiled lane is measured or it is `unsupported`"
                ));
            }
            if k != "lupin" && matches!(expect, Expect::Divergent(_)) {
                return Err(format!(
                    "{what}:{line}: `divergent(…)` is a lupin expectation (sc24); \
                     the compiler lanes' answers define the directive, so a \
                     divergence there is a red to investigate, never a row"
                ));
            }
            *slot = Some(expect);
        }
        let (Some(lupin), Some(wolfc), Some(native)) = (lupin, wolfc, native) else {
            return Err(format!(
                "{what}: `{test}` must record all three lanes (`lupin`, `wolfc`, `native`)"
            ));
        };
        if ledger
            .insert(
                test.clone(),
                Entry {
                    lupin,
                    wolfc,
                    native,
                },
            )
            .is_some()
        {
            return Err(format!("{what}: duplicate entry for `{test}`"));
        }
    }
    Ok(ledger)
}

fn parse_expect(v: &str) -> Option<Expect> {
    match v {
        "run" => Some(Expect::Run),
        "unsupported" => Some(Expect::Unsupported),
        "slow" => Some(Expect::Slow),
        _ if v.starts_with("unstable(") => {
            let inner = v.strip_prefix("unstable(")?.strip_suffix(')')?;
            let mut set = Vec::new();
            for part in inner.split('|') {
                let e = parse_expect(part.trim())?;
                if matches!(e, Expect::Unstable(_) | Expect::Slow | Expect::Divergent(_))
                    || set.contains(&e)
                {
                    return None; // no nesting, no skips, no filed divergences, no duplicates
                }
                set.push(e);
            }
            (set.len() >= 2).then_some(Expect::Unstable(set))
        }
        _ if v.starts_with("divergent(") => {
            let inner = v.strip_prefix("divergent(")?.strip_suffix(')')?;
            parse_div_obs(inner).map(Expect::Divergent)
        }
        _ => {
            let code = v.strip_prefix("fail(")?.strip_suffix(')')?;
            let ok = code.starts_with('E')
                && code.len() > 1
                && code[1..].chars().all(|c| c.is_ascii_digit());
            ok.then(|| Expect::Fail(code.to_string()))
        }
    }
}

/// The inner of `divergent(…)`: `stdout`, `exit(N)`, or `trap(kind)` —
/// trap kinds are the toolchain's own names ([conf.trap.set] plus the
/// interpreter's dynamic-move kind): lowercase words and dashes, never
/// empty.
fn parse_div_obs(inner: &str) -> Option<DivObs> {
    if inner == "stdout" {
        return Some(DivObs::Stdout);
    }
    if let Some(n) = inner
        .strip_prefix("exit(")
        .and_then(|s| s.strip_suffix(')'))
    {
        return Some(DivObs::Exit(n.parse().ok()?));
    }
    let k = inner.strip_prefix("trap(")?.strip_suffix(')')?;
    (!k.is_empty()
        && k.chars()
            .all(|c| c.is_ascii_lowercase() || c == '-' || c == '_'))
    .then(|| DivObs::Trap(k.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_orders() {
        let l = parse(
            "[tests.\"prelude/hello.lu\"]\nlupin = \"run\"\nwolfc = \"fail(E0301)\"\nnative = \"unsupported\"\n",
            "ledger",
        )
        .unwrap();
        let e = &l["prelude/hello.lu"];
        assert_eq!(e.lupin, Expect::Run);
        assert_eq!(e.wolfc, Expect::Fail("E0301".into()));
        assert_eq!(e.native, Expect::Unsupported);
        assert!(depth(&Expect::Run) > depth(&Expect::Fail("E1".into())));
        assert!(depth(&Expect::Fail("E1".into())) > depth(&Expect::Unsupported));
    }

    #[test]
    fn parses_unstable_rows() {
        let l = parse(
            "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"unstable(run|unsupported)\"\n\
             native = \"unsupported\"\n",
            "l",
        )
        .unwrap();
        let w = &l["a.lu"].wolfc;
        assert_eq!(w.to_string(), "unstable(run|unsupported)");
        assert!(satisfies(w, &Expect::Run));
        assert!(satisfies(w, &Expect::Unsupported));
        assert!(!satisfies(w, &Expect::Fail("E1".into())));
        // As deep as its deepest outcome, so a drop to `unsupported` on
        // BOTH observations still reads as a regression.
        assert_eq!(depth(w), depth(&Expect::Run));
        // A single-outcome or nested `unstable(…)` is a malformed row.
        assert!(parse_expect("unstable(run)").is_none());
        assert!(parse_expect("unstable(run|run)").is_none());
        assert!(parse_expect("unstable(run|unstable(run|unsupported))").is_none());
        assert!(parse_expect("unstable(run|fail(E0806))").is_some());
    }

    #[test]
    fn slow_is_lupin_only_and_never_unstable() {
        // The sc16 honest-slow-skip: `slow` parses for the interpreter
        // lane, claims run-depth, and is refused everywhere else — a
        // compiled lane is measured or it is `unsupported`, and an
        // unstable set may not hide a skip.
        let l = parse(
            "[tests.\"a.lu\"]\nlupin = \"slow\"\nwolfc = \"unsupported\"\nnative = \"run\"\n",
            "l",
        )
        .unwrap();
        assert_eq!(l["a.lu"].lupin, Expect::Slow);
        assert_eq!(l["a.lu"].lupin.to_string(), "slow");
        assert_eq!(depth(&Expect::Slow), depth(&Expect::Run));
        assert!(parse(
            "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"slow\"\nnative = \"run\"\n",
            "l"
        )
        .is_err());
        assert!(parse(
            "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"run\"\nnative = \"slow\"\n",
            "l"
        )
        .is_err());
        assert!(parse_expect("unstable(slow|run)").is_none());
    }

    #[test]
    fn divergent_is_lupin_only_and_exact() {
        // The sc24 word: the lane executes, its honest observation cannot
        // satisfy the directive, and the row demands EXACTLY the named
        // observation (F-0097/F-0098's four rows).
        let l = parse(
            "[tests.\"a.lu\"]\nlupin = \"divergent(stdout)\"\nwolfc = \"run\"\nnative = \"run\"\n",
            "l",
        )
        .unwrap();
        assert_eq!(l["a.lu"].lupin, Expect::Divergent(DivObs::Stdout));
        assert_eq!(l["a.lu"].lupin.to_string(), "divergent(stdout)");
        assert_eq!(
            parse_expect("divergent(trap(use-after-move))"),
            Some(Expect::Divergent(DivObs::Trap("use-after-move".into())))
        );
        assert_eq!(
            parse_expect("divergent(exit(1))"),
            Some(Expect::Divergent(DivObs::Exit(1)))
        );
        // Run-depth reach, documentation not gate (the `slow` pattern).
        assert_eq!(
            depth(&Expect::Divergent(DivObs::Stdout)),
            depth(&Expect::Run)
        );
        // A divergent row is never satisfied by a normal observation: the
        // runner's special path is the only acceptance, so a heal reads as
        // a red and the flip to `run` is deliberate.
        assert!(!satisfies(&Expect::Divergent(DivObs::Stdout), &Expect::Run));
        // Lupin-only: the compiler lanes' answers define the directive.
        assert!(parse(
            "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"divergent(stdout)\"\nnative = \"run\"\n",
            "l"
        )
        .is_err());
        assert!(parse(
            "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"run\"\nnative = \"divergent(exit(1))\"\n",
            "l"
        )
        .is_err());
        // Malformed inners and smuggling attempts.
        assert!(parse_expect("divergent()").is_none());
        assert!(parse_expect("divergent(run)").is_none());
        assert!(parse_expect("divergent(trap())").is_none());
        assert!(parse_expect("divergent(trap(Bounds))").is_none());
        assert!(parse_expect("divergent(exit(x))").is_none());
        assert!(parse_expect("unstable(run|divergent(stdout))").is_none());
    }

    #[test]
    fn rejects_dishonest_shapes() {
        assert!(
            parse(
                "[tests.\"a.lu\"]\nlupin = \"fail(E1)\"\nwolfc = \"run\"\nnative = \"run\"\n",
                "l"
            )
            .is_err(),
            "lupin never records fail"
        );
        assert!(
            parse("[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"run\"\n", "l").is_err(),
            "all three lanes, always"
        );
        assert!(
            parse(
                "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"fail(bogus)\"\nnative = \"run\"\n",
                "l"
            )
            .is_err(),
            "fail codes are E-numbers"
        );
        let two = "[tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"run\"\nnative = \"run\"\n\
                   [tests.\"a.lu\"]\nlupin = \"run\"\nwolfc = \"run\"\nnative = \"run\"\n";
        assert!(parse(two, "l").is_err(), "duplicate test entry");
    }
}
