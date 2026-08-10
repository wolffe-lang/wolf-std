//! `cargo xtask doc-examples` — the API-CONVENTIONS §4 extractor, live
//! from sc01 on: every fenced ```wolf-doc-example``` block in the std
//! tree is executable, always. Each block becomes a staged entry file —
//! a `let`/`var` line passes through verbatim; every other line `EXPR`
//! becomes `if EXPR { } else { return N }` (N = the 1-based line index)
//! — imported against the staged module and run under both lanes.
//!
//! The doc-truth gate: lupin must reach `exit(0)` — a documented
//! example the reference machine cannot execute truthfully is a doc
//! bug, and an unrunnable example belongs in prose (§4). wolfc must
//! reach `exit(0)` or refuse honestly (`unsupported`); a static
//! rejection (`fail(E…)`) is a doc bug too. sc01 delta, recorded in
//! the campaign closeout: outcomes are printed and gated here, not yet
//! ledgered per-block in tests/ledger.toml (whose keys are tests/
//! files); folding examples into the ledger is sc02 rig work.

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
const WOLFC_WAIVERS: &[(&str, &str, &str)] = &[
    // The iter module's list_next body trips the pinned mem tier's
    // List reads-as-moves; every import of `iter` fails E1001.
    ("iter", "E1001", "F-0005"),
];

struct Block {
    /// std module directory name (`cmp`, `prelude`, …).
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
    let mut lanes = Vec::new();
    for imp in [Impl::Lupin, Impl::Wolf] {
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
        let entry_src = render_entry(b);
        let scratch = repo
            .join("target/stage")
            .join(format!("doc-example__{}_{k}", b.module));
        std::fs::create_dir_all(&scratch).map_err(|e| format!("doc-examples: mkdir: {e}"))?;
        let entry_path = scratch.join("example.lu");
        std::fs::write(&entry_path, &entry_src).map_err(|e| format!("doc-examples: write: {e}"))?;
        let staged_root = scratch.join("pkg");
        let staged = stage::stage_test(&entry_path, &repo.join("std"), &staged_root)?;
        for (imp, bin) in &lanes {
            let verdict = run_lane(*imp, bin, &staged, ceiling)?;
            let waived = matches!((&verdict, imp), (Verdict::Fail(code), Impl::Wolf)
                if WOLFC_WAIVERS.iter().any(|(m, c, _)| *m == b.module && c == code));
            let ok = waived
                || matches!(
                    (&verdict, imp),
                    (Verdict::Exit(0), _) | (Verdict::Unsupported, Impl::Wolf)
                );
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
    staged_entry: &Path,
    ceiling: Duration,
) -> Result<Verdict, String> {
    let mut cmd = Command::new(bin);
    cmd.arg("conform-run");
    if imp == Impl::Wolf {
        cmd.arg("--checked");
    }
    cmd.arg(staged_entry.as_os_str());
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
    Ok(record::parse(&got.stdout, imp.ledger_name())?.verdict)
}

fn render_entry(b: &Block) -> String {
    let mut out = String::new();
    out.push_str("//! check: run(exit=0)\n//! phase: run\n//!\n");
    out.push_str(&format!(
        "//! GENERATED from {} — the §4 extractor; do not edit.\n\n",
        b.origin
    ));
    out.push_str(&format!("use {}\n\nfn main() -> !int {{\n", b.module));
    for (i, line) in b.lines.iter().enumerate() {
        let l = line.trim();
        if l.starts_with("let ") || l.starts_with("var ") {
            out.push_str(&format!("    {l}\n"));
        } else {
            out.push_str(&format!("    if {l} {{ }} else {{ return {} }}\n", i + 1));
        }
    }
    out.push_str("    0\n}\n");
    out
}

fn collect_blocks(std_root: &Path) -> Result<Vec<Block>, String> {
    let mut blocks = Vec::new();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(std_root)
        .map_err(|e| format!("read {}: {e}", stage::show(std_root)))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();
    dirs.sort();
    for dir in dirs {
        let module = dir
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .map_err(|e| format!("read {}: {e}", stage::show(&dir)))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("lu"))
            .collect();
        files.sort();
        for f in files {
            let text =
                std::fs::read_to_string(&f).map_err(|e| format!("{}: {e}", stage::show(&f)))?;
            blocks.extend(scan_file(&module, &f, &text)?);
        }
    }
    Ok(blocks)
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
        assert!(entry.contains("use iter\n"), "{entry}");
        assert!(entry.contains("    var it = iter.range_iter(1, 2)\n"));
        assert!(
            entry.contains(
                "    if (iter.range_next(mut it) else 0 - 1) == 1 { } else { return 2 }\n"
            ),
            "{entry}"
        );
    }

    #[test]
    fn unclosed_fence_is_an_error() {
        let src = "/// ```wolf-doc-example\n/// 1 == 1\nfn x() {}\n";
        assert!(scan_file("m", Path::new("f.lu"), src).is_err());
    }
}
