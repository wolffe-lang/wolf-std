//! `cargo xtask ulp` — the accuracy harness sc04's acceptance asks for.
//!
//! `std.math.float` implements its transcendentals in pure wolf and
//! documents a per-function ulp bound. This subcommand is what makes
//! that bound a fact rather than a claim, and it checks two different
//! things that are easy to confuse:
//!
//! 1. **Accuracy.** Every row of `tests/ulp/reference.txt` carries the
//!    correctly rounded reference value for one call and the value std
//!    actually produces. The distance between them, in units of the last
//!    place, must be within the row's bound — and the bound must be the
//!    one the function's doc comment states.
//! 2. **Determinism, demonstrated.** The pure-wolf decision claims every
//!    implementation gets the same bits. The harness generates a program
//!    that asserts each call is EXACTLY its recorded value and runs it on
//!    every lit lane; a lane that disagrees in the last bit fails here.
//!
//! The reference values are committed, not recomputed: a host libm is
//! itself a moving target across platforms, and a table that changes
//! under CI cannot pin anything. The harness re-derives each reference
//! from the host's own `f64` maths as a REPORT-ONLY cross-check, so a
//! table that drifts from every libm on earth is noticed without a
//! platform's rounding making CI red.
//!
//! Format of `tests/ulp/reference.txt`, one call per line, `#` comments:
//! `fn | args | got | reference | bound`, where `args` are
//! comma-separated wolf float literals, `got` and `reference` are decimal
//! `f64` literals, and `bound` is the ulp budget.

use crate::bins::{self, Impl};
use crate::exec;
use crate::record::{self, Verdict};
use crate::repo_root;
use crate::stage;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub struct Row {
    pub func: String,
    pub args: Vec<String>,
    pub got: f64,
    pub reference: f64,
    pub bound: i64,
    pub line: usize,
}

/// Distance in units in the last place between two finite `f64`s, using
/// the monotone signed-magnitude ordering of their encodings.
pub fn ulps_between(a: f64, b: f64) -> i64 {
    fn ordered(x: f64) -> i128 {
        let b = x.to_bits() as i64 as i128;
        if b >= 0 {
            b
        } else {
            -(b - i64::MIN as i128)
        }
    }
    let d = ordered(a) - ordered(b);
    d.clamp(i64::MIN as i128, i64::MAX as i128) as i64
}

pub fn parse_table(text: &str, what: &str) -> Result<Vec<Row>, String> {
    let mut rows = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();
        if parts.len() != 5 {
            return Err(format!(
                "{what}:{}: expected `fn | args | got | reference | bound`, got {} field(s)",
                i + 1,
                parts.len()
            ));
        }
        let parse_f = |s: &str, which: &str| -> Result<f64, String> {
            s.parse::<f64>()
                .map_err(|_| format!("{what}:{}: {which} `{s}` is not an f64", i + 1))
        };
        rows.push(Row {
            func: parts[0].to_string(),
            args: parts[1].split(',').map(|a| a.trim().to_string()).collect(),
            got: parse_f(parts[2], "got")?,
            reference: parse_f(parts[3], "reference")?,
            bound: parts[4]
                .parse::<i64>()
                .map_err(|_| format!("{what}:{}: bound `{}` is not an integer", i + 1, parts[4]))?,
            line: i + 1,
        });
    }
    if rows.is_empty() {
        return Err(format!("{what}: no reference rows"));
    }
    Ok(rows)
}

/// The host's own answer for a row, when it has one — the report-only
/// cross-check described in the module header.
fn host_reference(r: &Row) -> Option<f64> {
    let a: Vec<f64> = r
        .args
        .iter()
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    if a.len() != r.args.len() {
        return None;
    }
    Some(match (r.func.as_str(), a.len()) {
        ("sqrt", 1) => a[0].sqrt(),
        ("cbrt", 1) => a[0].cbrt(),
        ("hypot", 2) => a[0].hypot(a[1]),
        ("exp", 1) => a[0].exp(),
        ("exp2", 1) => a[0].exp2(),
        ("ln", 1) => a[0].ln(),
        ("log2", 1) => a[0].log2(),
        ("log10", 1) => a[0].log10(),
        ("sin", 1) => a[0].sin(),
        ("cos", 1) => a[0].cos(),
        ("tan", 1) => a[0].tan(),
        ("asin", 1) => a[0].asin(),
        ("acos", 1) => a[0].acos(),
        ("atan", 1) => a[0].atan(),
        ("atan2", 2) => a[0].atan2(a[1]),
        ("powf", 2) => a[0].powf(a[1]),
        _ => return None,
    })
}

/// The bounds each function's doc comment states, which the table may
/// not exceed — the two documents cannot drift apart silently.
const DOCUMENTED_BOUNDS: &[(&str, i64)] = &[
    ("sqrt", 2),
    ("cbrt", 2),
    ("hypot", 2),
    ("exp", 2),
    ("exp2", 2),
    ("ln", 2),
    ("log2", 2),
    ("log10", 2),
    ("sin", 2),
    ("cos", 2),
    ("tan", 2),
    ("asin", 2),
    ("acos", 2),
    ("atan", 2),
    ("atan2", 2),
    ("powf", 4),
];

pub fn ulp() -> Result<(), String> {
    let repo = repo_root();
    let path = repo.join("tests/ulp/reference.txt");
    let text =
        std::fs::read_to_string(&path).map_err(|e| format!("tests/ulp/reference.txt: {e}"))?;
    let rows = parse_table(&text, "tests/ulp/reference.txt")?;

    let mut reds = Vec::new();
    let mut worst: std::collections::BTreeMap<String, i64> = std::collections::BTreeMap::new();
    let mut host_drift = 0usize;
    for r in &rows {
        let documented = DOCUMENTED_BOUNDS
            .iter()
            .find(|(f, _)| *f == r.func)
            .map(|(_, b)| *b);
        match documented {
            Some(b) if r.bound > b => reds.push(format!(
                "tests/ulp/reference.txt:{}: row budgets {} ulp for `{}`, whose doc \
                 comment promises {b} — the table cannot loosen the contract",
                r.line, r.bound, r.func
            )),
            None => reds.push(format!(
                "tests/ulp/reference.txt:{}: `{}` states no documented bound",
                r.line, r.func
            )),
            _ => {}
        }
        let d = ulps_between(r.got, r.reference);
        let e = worst.entry(r.func.clone()).or_insert(0);
        if d.abs() > e.abs() {
            *e = d;
        }
        if d.abs() > r.bound {
            reds.push(format!(
                "tests/ulp/reference.txt:{}: float.{}({}) is {d} ulp from the \
                 reference, budget {}",
                r.line,
                r.func,
                r.args.join(", "),
                r.bound
            ));
        }
        if let Some(h) = host_reference(r) {
            if ulps_between(h, r.reference) != 0 {
                host_drift += 1;
            }
        }
    }
    for (f, d) in &worst {
        println!("ulp: {f:8} worst {d:+} of the recorded set");
    }
    if host_drift > 0 {
        println!(
            "ulp: NOTE {host_drift} committed reference value(s) differ from this \
             host's libm — report only (platform rounding is not a wolf-std \
             regression); investigate if the count grows"
        );
    }

    // The determinism half: every recorded value, asserted exactly, on
    // every lit lane.
    let entry_src = render_assertions(&rows);
    let scratch = repo.join("target/stage/ulp");
    std::fs::create_dir_all(&scratch).map_err(|e| format!("ulp: mkdir: {e}"))?;
    let entry = scratch.join("entry.lu");
    std::fs::write(&entry, &entry_src).map_err(|e| format!("ulp: write: {e}"))?;
    let staged = stage::stage_test(&entry, &repo.join("std"), &scratch.join("pkg"))?;

    let native_rt = bins::native_rt(&repo);
    let ceiling = Duration::from_secs(exec::timeout_secs());
    let mut lanes_run = 0usize;
    for imp in bins::LANES {
        if imp == Impl::Native && native_rt.is_none() {
            println!("SKIP: no libwolf_rt.a — ulp determinism lane `native` dark");
            continue;
        }
        let Some(bin) = bins::resolve(imp, &repo) else {
            println!(
                "SKIP: no {} — ulp determinism lane `{}` dark",
                imp.name(),
                imp.ledger_name()
            );
            continue;
        };
        let verdict = run_lane(imp, &bin.path, &staged, ceiling)?;
        match (&verdict, imp) {
            (Verdict::Exit(0), _) => {
                lanes_run += 1;
                println!(
                    "ulp: {} reproduces all {} recorded values exactly",
                    imp.ledger_name(),
                    rows.len()
                );
            }
            // The checked tier refuses float literals outright; that is a
            // recorded refusal, not a disagreement.
            (Verdict::Unsupported, Impl::Wolf) | (Verdict::Unsupported, Impl::Native) => {
                println!(
                    "ulp: {} — unsupported (honest refusal, no evidence either way)",
                    imp.ledger_name()
                );
            }
            (Verdict::Exit(n), _) => reds.push(format!(
                "ulp: {} disagrees with the recorded value of row {n} \
                 (`float.{}`) — the pure-wolf determinism claim is the thing \
                 this catches",
                imp.ledger_name(),
                rows.get((*n as usize).saturating_sub(1))
                    .map(|r| r.func.as_str())
                    .unwrap_or("?")
            )),
            (v, _) => reds.push(format!("ulp: {} answered {v}", imp.ledger_name())),
        }
    }
    if lanes_run == 0 {
        println!("ulp: no lane executed the determinism program (all dark or refusing)");
    }

    if reds.is_empty() {
        println!("ulp: {} reference row(s), GREEN", rows.len());
        Ok(())
    } else {
        Err(reds.join("\n"))
    }
}

fn render_assertions(rows: &[Row]) -> String {
    let mut out = String::new();
    out.push_str("//! check: run(exit=0)\n//! phase: run\n//!\n");
    out.push_str("//! GENERATED by `cargo xtask ulp` from tests/ulp/reference.txt.\n\n");
    out.push_str("use std.math.float\n\nfn main() -> !int {\n");
    for (i, r) in rows.iter().enumerate() {
        out.push_str(&format!(
            "    if float.{}({}) == {} {{ }} else {{ return {} }}\n",
            r.func,
            r.args.join(", "),
            fmt_lit(r.got),
            i + 1
        ));
    }
    out.push_str("    0\n}\n");
    out
}

/// A wolf float literal for a finite `f64` — shortest round-trip, with a
/// decimal point forced so the lexer reads it as a float.
fn fmt_lit(x: f64) -> String {
    let s = format!("{x:?}");
    if s.contains('.') || s.contains('e') || s.contains("inf") || s.contains("NaN") {
        s
    } else {
        format!("{s}.0")
    }
}

fn run_lane(
    imp: Impl,
    bin: &Path,
    staged: &stage::Staged,
    ceiling: Duration,
) -> Result<Verdict, String> {
    let mut cmd = Command::new(bin);
    cmd.arg("conform-run");
    if imp == Impl::Wolf {
        cmd.arg("--checked");
    }
    if imp == Impl::Native {
        cmd.arg("--native");
    }
    cmd.arg("--std-root").arg(staged.std_root.as_os_str());
    cmd.arg(staged.entry.as_os_str());
    cmd.arg("--json");
    let got = exec::run(cmd, ceiling)?;
    if got.timed_out {
        return Err(format!("ulp: timed out after {}s", ceiling.as_secs()));
    }
    if got.status != Some(0) {
        return Err(format!(
            "ulp: tool error (exit {:?}): {}",
            got.status,
            got.stderr.trim()
        ));
    }
    Ok(record::parse(&got.stdout, imp.ledger_name())?.verdict)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ulp_distance_is_signed_and_symmetric() {
        assert_eq!(ulps_between(1.0, 1.0), 0);
        assert_eq!(ulps_between(f64::from_bits(1.0f64.to_bits() + 1), 1.0), 1);
        assert_eq!(ulps_between(1.0, f64::from_bits(1.0f64.to_bits() + 1)), -1);
        // Across zero the ordering is signed-magnitude, so the two
        // neighbours of zero are two ulp apart.
        assert!(ulps_between(f64::MIN_POSITIVE, -f64::MIN_POSITIVE).abs() > 0);
        assert_eq!(ulps_between(0.0, -0.0), 0);
    }

    #[test]
    fn table_parses_and_rejects_junk() {
        let t = "# comment\nsqrt | 2.0 | 1.4142135623730951 | 1.4142135623730951 | 2\n";
        let rows = parse_table(t, "t").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].func, "sqrt");
        assert_eq!(rows[0].bound, 2);
        assert!(
            parse_table("sqrt | 2.0 | 1.0\n", "t").is_err(),
            "field count"
        );
        assert!(parse_table("", "t").is_err(), "empty table");
    }

    #[test]
    fn literals_keep_their_point() {
        assert_eq!(fmt_lit(2.0), "2.0");
        assert_eq!(fmt_lit(0.5), "0.5");
    }
}
