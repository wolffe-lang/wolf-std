//! `cargo xtask doc-examples` — the API-CONVENTIONS §4 extractor, live
//! from sc01 on: every fenced ```wolf-doc-example``` block in the std
//! tree is executable, always. Each block becomes a staged entry file
//! importing the documented module (`use std.<dotted path>`, nested
//! modules included) and runs under both lanes.
//!
//! Line classification is §4's sc02 amendment, implemented in
//! `is_assertion`/`validate_line`: a line carrying a relational operator
//! is an ASSERTION and becomes `if LINE { } else { return N }` (N = the
//! 1-based line index); every other line is a STATEMENT and passes
//! through verbatim (`let`/`var`, an assignment, a unit-returning
//! mutating call, a one-line `for`). A line that is neither is a hard
//! error, so a typo cannot become a silently unchecked statement.
//!
//! The doc-truth gate: lupin must reach `exit(0)` — a documented
//! example the reference machine cannot execute truthfully is a doc
//! bug, and an unrunnable example belongs in prose (§4). The compiler's
//! two rungs (checked and, from sc04, native) must reach `exit(0)` or
//! refuse honestly (`unsupported`); a static rejection (`fail(E…)`) is a
//! doc bug too — which is why `std.range`, whose one function names a
//! type the compiler does not resolve, carries prose instead of a fence.
//!
//! sc01 left "fold examples into tests/ledger.toml" to sc02; sc02's
//! answer, for the closeout to confirm: NO. The ledger's value is
//! recording a per-implementation *depth* that may legitimately be
//! shallow, while an example has exactly one legitimate depth on the
//! reference machine (`exit(0)`) — the gate here is already stricter
//! than any row could be, and duplicating 45 blocks into a file keyed by
//! `tests/` paths would add bookkeeping without adding truth. What DID
//! move into the ledger's spirit is the waiver list below: a wolfc
//! rejection is tolerated only when it names a filed finding.

use crate::bins::{self, Impl};
use crate::exec;
use crate::record::{self, Verdict};
use crate::repo_root;
use crate::stage;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Waivers for wolfc rejections caused by FILED upstream bugs, not by
/// the example: (module, error code, finding). A waiver is loud in the
/// output and dies with the pin bump that fixes its finding.
/// Empty at the sc03 pin, and that is the point: sc02's one waiver
/// (`iter` / `E1001` / F-0005 — the pinned mem tier's List
/// reads-as-moves) died with the pin bump that closed wolf-lang#6, exactly
/// as the comment above it promised. A waiver is a debt with a name, not a
/// permanent allowance.
const WOLFC_WAIVERS: &[(&str, &str, &str)] = &[];

/// Modules whose lanes are honestly unequal, so an `unsupported` verdict
/// is acceptable on EVERY lane while the doc-truth rule is kept by a
/// stronger requirement instead: at least one lane must reach `exit(0)`,
/// so a documented example is still a program that ran somewhere.
///
/// The list is deliberately tiny and deliberately named by module rather
/// than by capability: a module lands here when its whole surface is
/// something an implementation may not have, never to excuse one
/// function.
///
/// Two reasons put a module here, and sc10 added the second.
///
/// 1. **An OS CAPABILITY (I13) an implementation may honestly not have**
///    (sc07): `fs`, `io`, `net`, `time` (`Clock`), `env` (`env`) and — from
///    sc11 — `process` (`Exec`). At the sc11 pin lupin has the `time_*`
///    and `env_*` tiers and declines `fs`, `net`, `json` and the process
///    trio, each with a sentence in its refusal; the list stays as it is
///    because a lane an implementation may honestly not have is what the
///    entry records, not the lane it happens to have today.
/// 2. **A builtin tier only one rung has landed** (sc10): `x.json`
///    reaches no capability at all — the `json_*` family is the one PURE
///    builtin family, no I13 tag, no sandbox category — and its kernels
///    execute on the checked tier alone at s40, because the native mirror
///    and the interpreter's own are unwritten. `process` is in this class
///    too, from the other direction: the native rung refuses the trio by
///    name, so its examples run on the checked lane alone. The honesty this
///    list encodes is the same either way: the example ran somewhere, and
///    the lanes that refused said so.
const CAPABILITY_MODULES: &[&str] = &["fs", "io", "net", "time", "env", "process", "x.json"];

struct Block {
    /// Dotted std module path, without the `std.` head (`cmp`,
    /// `x.deque_int`, …). The bound name is its last segment.
    module: String,
    /// Source file and 1-based line of the opening fence, for messages.
    origin: String,
    /// The example lines, fence markers stripped.
    lines: Vec<String>,
}

pub fn doc_examples() -> Result<(), String> {
    let repo = repo_root();
    let blocks = collect_blocks(&repo.join("std"))?;
    if blocks.is_empty() {
        println!("doc-examples: no wolf-doc-example blocks in std/");
        return Ok(());
    }
    let native_rt = bins::native_rt(&repo);
    let mut lanes = Vec::new();
    for imp in bins::LANES {
        if imp == Impl::Native && native_rt.is_none() {
            println!("SKIP: no libwolf_rt.a — doc-example native lane dark");
            continue;
        }
        match bins::resolve(imp, &repo) {
            Some(r) => lanes.push((imp, r.path)),
            None => println!(
                "SKIP: no {} — doc-example lane dark (same acquisition story as std-test)",
                imp.name()
            ),
        }
    }
    let ceiling = Duration::from_secs(exec::timeout_secs());
    let mut reds = Vec::new();
    for (k, b) in blocks.iter().enumerate() {
        for line in &b.lines {
            validate_line(line, &b.origin)?;
        }
        let entry_src = render_entry(b);
        let scratch = repo
            .join("target/stage")
            .join(format!("doc-example__{}_{k}", b.module));
        std::fs::create_dir_all(&scratch).map_err(|e| format!("doc-examples: mkdir: {e}"))?;
        let entry_path = scratch.join("example.lu");
        std::fs::write(&entry_path, &entry_src).map_err(|e| format!("doc-examples: write: {e}"))?;
        let staged_root = scratch.join("pkg");
        let staged = stage::stage_test(&entry_path, &repo.join("std"), &staged_root)?;
        let capability = CAPABILITY_MODULES.contains(&b.module.as_str());
        let mut any_ran = false;
        for (imp, bin) in &lanes {
            let (verdict, warnings) = run_lane(*imp, bin, &staged, ceiling)?;
            if !warnings.is_empty() {
                reds.push(format!(
                    "{} [{}]: {} warning(s) — this rig denies warnings \
                     (F-0046/F-0053):\n    {}",
                    b.origin,
                    imp.ledger_name(),
                    warnings.len(),
                    warnings.join("\n    ")
                ));
            }
            let compiler_lane = matches!(imp, Impl::Wolf | Impl::Native);
            let waived = matches!(&verdict, Verdict::Fail(code)
                if compiler_lane
                    && WOLFC_WAIVERS
                        .iter()
                        .any(|(m, c, _)| *m == b.module && c == code));
            if matches!(&verdict, Verdict::Exit(0)) {
                any_ran = true;
            }
            let ok = waived
                || matches!(&verdict, Verdict::Exit(0))
                || ((compiler_lane || capability) && matches!(&verdict, Verdict::Unsupported));
            let tag = if waived {
                let (_, _, finding) = WOLFC_WAIVERS
                    .iter()
                    .find(|(m, _, _)| *m == b.module)
                    .expect("matched above");
                format!("WAIVED: upstream {finding}, not a doc bug")
            } else if ok {
                "ok".to_string()
            } else {
                "DOC BUG".to_string()
            };
            println!(
                "doc-example {} [{}]: {} ({})",
                b.origin,
                imp.ledger_name(),
                verdict,
                tag
            );
            if !ok {
                reds.push(format!(
                    "{} [{}]: {} — examples are executable, always (§4); \
                     move it to prose or fix the doc",
                    b.origin,
                    imp.ledger_name(),
                    verdict
                ));
            }
        }
        // The doc-truth rule for a capability module: no lane is REQUIRED
        // to have the capability, but the example must have run somewhere,
        // or it is documentation nothing checks.
        if capability && !any_ran && !lanes.is_empty() {
            reds.push(format!(
                "{}: no lane reached exit(0) — a capability module's example still has to \
                 run somewhere (§4)",
                b.origin
            ));
        }
    }
    if reds.is_empty() {
        println!("doc-examples: {} block(s), GREEN", blocks.len());
        Ok(())
    } else {
        Err(reds.join("\n"))
    }
}

fn run_lane(
    imp: Impl,
    bin: &Path,
    staged: &stage::Staged,
    ceiling: Duration,
) -> Result<(Verdict, Vec<String>), String> {
    let mut cmd = Command::new(bin);
    // The staged package root is the working directory (sc07), so an
    // os-facing example writes its scratch files there and nowhere else.
    cmd.current_dir(&staged.root);
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
        return Err(format!(
            "doc-examples: timed out after {}s",
            ceiling.as_secs()
        ));
    }
    if got.status != Some(0) {
        return Err(format!(
            "doc-examples: tool error (exit {:?}): {}",
            got.status,
            got.stderr.trim()
        ));
    }
    let rec = record::parse(&got.stdout, imp.ledger_name())?;
    // The verdict AND the warnings: the gate applies to documentation too
    // (sc09), and a warning is reported like any other doc bug — collected,
    // named, and never allowed to stop the sweep, so one bad example does
    // not hide the next one. See `runner`'s gate and F-0046/F-0053.
    Ok((rec.verdict, rec.warnings))
}

/// Relational operators an assertion line must carry (§4 amendment,
/// sc02): an example line is an ASSERTION when it relates two
/// expressions, and a STATEMENT otherwise. Without the rule a
/// unit-returning call (`list.push(mut xs, 1)`) could not appear in an
/// example at all, and a bare predicate call would silently stop being
/// checked — so `is_empty(xs) == true` is how a predicate is asserted.
const RELATIONS: &[&str] = &["==", "!=", "<=", ">=", "<", ">"];

fn is_assertion(line: &str) -> bool {
    let bare = mask_strings(line);
    RELATIONS.iter().any(|op| bare.contains(op))
}

/// The line with every string literal's CONTENTS blanked out, so that an
/// operator inside a literal cannot decide the line's kind. sc05 found
/// this the hard way: `let bad = base64.decode("T!==") else List[int]()`
/// is a statement whose payload contains `!=`, and classifying it as an
/// assertion wrapped a `let` in an `if` (E0201). Escapes are honoured so
/// that a `\"` inside a literal does not end it early.
fn mask_strings(line: &str) -> String {
    let mut out = String::with_capacity(line.len());
    let mut in_string = false;
    let mut escaped = false;
    for c in line.chars() {
        if in_string {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                in_string = false;
                out.push(c);
                continue;
            }
            out.push(' ');
        } else {
            if c == '"' {
                in_string = true;
            }
            out.push(c);
        }
    }
    out
}

fn render_entry(b: &Block) -> String {
    let mut out = String::new();
    out.push_str("//! check: run(exit=0)\n//! phase: run\n//!\n");
    out.push_str(&format!(
        "//! GENERATED from {} — the §4 extractor; do not edit.\n\n",
        b.origin
    ));
    out.push_str(&format!("use std.{}\n\nfn main() -> !int {{\n", b.module));
    for (i, line) in b.lines.iter().enumerate() {
        let l = line.trim();
        if is_assertion(l) {
            out.push_str(&format!("    if {l} {{ }} else {{ return {} }}\n", i + 1));
        } else {
            // Statements pass through verbatim: `let`/`var` bindings, a
            // mutating call, a one-line `for`. Every line is real wolf
            // either way — the extractor never rewrites what a reader
            // sees (§4's doc-truth rule).
            out.push_str(&format!("    {l}\n"));
        }
    }
    out.push_str("    0\n}\n");
    out
}

/// Reject an example line that is neither an assertion nor a plausible
/// statement — the shape most likely to be a typo that would otherwise
/// pass through and be silently unchecked.
fn validate_line(line: &str, origin: &str) -> Result<(), String> {
    let l = line.trim();
    if l.is_empty() {
        return Err(format!("{origin}: blank line in a wolf-doc-example"));
    }
    if is_assertion(l) {
        return Ok(());
    }
    let statement = l.starts_with("let ")
        || l.starts_with("var ")
        || l.starts_with("for ")
        // an assignment (`m["k"] = 1`, `d.head = 0`) — ` = ` cannot be a
        // relation, which `is_assertion` has already ruled out
        || l.contains(" = ")
        || l.ends_with(')')
        // a propagating call (`fs.write_text(p, t)?`) — the whole shape of
        // an os-facing statement (sc07): the operation returns a row, the
        // example's generated `main` is `-> !int`, and `?` is how a
        // statement that can fail is written honestly. Without this a
        // fallible statement could only be spelled with an `else`, which
        // would document handling the caller never wrote.
        || l.ends_with('?')
        || l.ends_with('}');
    if statement {
        Ok(())
    } else {
        Err(format!(
            "{origin}: `{l}` is neither an assertion (it relates nothing) nor a \
             statement — a predicate is asserted as `p(x) == true` (§4)"
        ))
    }
}

/// Walk the whole std tree — nested modules included (`std/x/deque_int`
/// is the module `x.deque_int`, imported as `use std.x.deque_int` and
/// bound to its last segment).
fn collect_blocks(std_root: &Path) -> Result<Vec<Block>, String> {
    let mut blocks = Vec::new();
    walk_modules(std_root, &[], &mut blocks)?;
    Ok(blocks)
}

fn walk_modules(dir: &Path, path: &[String], blocks: &mut Vec<Block>) -> Result<(), String> {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("read {}: {e}", stage::show(dir)))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    entries.sort();
    let module = path.join(".");
    for p in &entries {
        if p.is_dir() {
            let mut child = path.to_vec();
            child.push(
                p.file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            );
            walk_modules(p, &child, blocks)?;
        } else if p.extension().and_then(|e| e.to_str()) == Some("lu") {
            let text =
                std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", stage::show(p)))?;
            blocks.extend(scan_file(&module, p, &text)?);
        }
    }
    Ok(())
}

fn scan_file(module: &str, path: &Path, text: &str) -> Result<Vec<Block>, String> {
    let mut blocks = Vec::new();
    let mut current: Option<Block> = None;
    for (n, raw) in text.lines().enumerate() {
        let line = raw.trim_start();
        let Some(doc) = line
            .strip_prefix("/// ")
            .or_else(|| line.strip_prefix("///"))
        else {
            if let Some(b) = current.take() {
                return Err(format!(
                    "{}: wolf-doc-example fence opened at {} never closes",
                    stage::show(path),
                    b.origin
                ));
            }
            continue;
        };
        let doc = doc.trim_end();
        match current.as_mut() {
            None => {
                if doc.trim() == "```wolf-doc-example" {
                    current = Some(Block {
                        module: module.to_string(),
                        origin: format!("{}:{}", stage::show(path), n + 1),
                        lines: Vec::new(),
                    });
                }
            }
            Some(b) => {
                if doc.trim() == "```" {
                    blocks.push(current.take().expect("checked"));
                } else {
                    b.lines.push(doc.to_string());
                }
            }
        }
    }
    if let Some(b) = current {
        return Err(format!(
            "{}: wolf-doc-example fence opened at {} never closes",
            stage::show(path),
            b.origin
        ));
    }
    Ok(blocks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scans_and_renders() {
        let src = "/// docs\n/// ```wolf-doc-example\n/// var it = iter.range_iter(1, 2)\n/// (iter.range_next(mut it) else 0 - 1) == 1\n/// ```\npub fn x() {}\n";
        let blocks = scan_file("iter", Path::new("std/iter/iter.lu"), src).unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].lines.len(), 2);
        let entry = render_entry(&blocks[0]);
        assert!(entry.contains("use std.iter\n"), "{entry}");
        assert!(entry.contains("    var it = iter.range_iter(1, 2)\n"));
        assert!(
            entry.contains(
                "    if (iter.range_next(mut it) else 0 - 1) == 1 { } else { return 2 }\n"
            ),
            "{entry}"
        );
    }

    #[test]
    fn statements_pass_through_and_assertions_wrap() {
        let src = "/// ```wolf-doc-example\n/// var xs = List[int]()\n/// list.push(mut xs, 1)\n/// for x in xs { }\n/// list.len(xs) == 1\n/// ```\npub fn x() {}\n";
        let blocks = scan_file("list", Path::new("std/list/list.lu"), src).unwrap();
        let entry = render_entry(&blocks[0]);
        assert!(entry.contains("    list.push(mut xs, 1)\n"), "{entry}");
        assert!(entry.contains("    for x in xs { }\n"), "{entry}");
        assert!(
            entry.contains("    if list.len(xs) == 1 { } else { return 4 }\n"),
            "{entry}"
        );
        // A bare predicate is not an assertion — §4 wants `== true`.
        assert!(validate_line("list.is_empty(xs)", "t").is_ok(), "a call");
        assert!(validate_line("list.is_empty xs", "t").is_err(), "typo");
        assert!(validate_line("list.is_empty(xs) == true", "t").is_ok());
        assert!(validate_line("m[\"a\"] = 1", "t").is_ok(), "assignment");
    }

    #[test]
    fn a_propagating_statement_is_a_statement() {
        // sc07: the os-facing statement shape. `?` ends the line, the
        // generated `main` is `-> !int`, and the example documents the
        // propagation the caller actually writes.
        assert!(validate_line("fs.write_text(\"a.tmp\", \"x\")?", "t").is_ok());
        assert!(!is_assertion("fs.write_text(\"a.tmp\", \"x\")?"));
        // Still a typo, still rejected.
        assert!(validate_line("fs write_text", "t").is_err());
    }

    #[test]
    fn capability_modules_are_the_unequal_lane_facades() {
        // The list is a review surface: it must stay tiny, and a module
        // joins it only when a lane can honestly refuse ANY of its
        // examples. `net` joined in sc08 (its two address helpers are pure,
        // but the seven socket functions can be refused); `time` and `env`
        // joined in sc10 for the same reason at the `Clock` and `env`
        // tiers; `x.json` joined in sc10 for a DIFFERENT reason, which is
        // why the constant's doc names two — it reaches no capability at
        // all and its builtin tier has one rung. `process` joined in sc11
        // for BOTH reasons at once: `Exec` is a capability an
        // implementation may decline entirely (lupin does, by design) and
        // the trio's native rung is unwritten, so its examples run on the
        // checked lane alone.
        assert_eq!(
            CAPABILITY_MODULES,
            &["fs", "io", "net", "time", "env", "process", "x.json"]
        );
    }

    #[test]
    fn unclosed_fence_is_an_error() {
        let src = "/// ```wolf-doc-example\n/// 1 == 1\nfn x() {}\n";
        assert!(scan_file("m", Path::new("f.lu"), src).is_err());
    }
}

#[cfg(test)]
mod classifier_tests {
    use super::{is_assertion, mask_strings};

    #[test]
    fn a_relation_inside_a_string_literal_does_not_make_an_assertion() {
        // sc05's base64 example: a STATEMENT whose payload carries `!=`.
        let line = r#"let bad = base64.decode("T!==") else List[int]()"#;
        assert!(!is_assertion(line));
        // The real relation, outside any literal, still decides.
        assert!(is_assertion(r#"base64.encode(b) == "TQ==""#));
        assert!(is_assertion("fmt.to_str(3) == \"3\""));
        assert!(is_assertion("math.int_max() > 0"));
        assert!(!is_assertion("(mut xs).push(1)"));
    }

    #[test]
    fn masking_keeps_structure_and_honours_escapes() {
        assert_eq!(mask_strings(r#"f("a==b") == 1"#), r#"f("    ") == 1"#);
        // An escaped quote does not end the literal early, so the `<`
        // after it is still inside.
        assert_eq!(mask_strings(r#"f("a\"<b")"#), r#"f("     ")"#);
        assert_eq!(mask_strings("no strings here"), "no strings here");
    }
}
