//! Staging (`[conf.directive.member]`): the package root is the entry
//! file's directory, so a test cannot sit inside the tree it imports.
//! The runner copies the entry file plus the `std/` tree into a scratch
//! package root and runs there — byte-identical copies, sorted
//! `read_dir`, no symlinks (windows tier-1 keeps us honest).
//!
//! **The std tree is staged AS `std/`** (sc02): the wolf lane is invoked
//! with `--std-root <scratch>/std`, so `use std.cmp` resolves against
//! this repository's tree exactly as it will in a released toolchain —
//! the F-0001 interim is retired compiler-side (wolf-lang#1 closed by
//! s26's `--std-root`/`WOLF_STD` loader).
//!
//! THE MIRROR IS RETIRED (sc04, lupin 0.1.2 / wolf-interp#6 closed).
//! Until this pin the scratch root also carried a flat copy of every
//! module directory (`std/cmp/` as `cmp/`) because lupin bound a
//! `use std.cmp` to the path's LAST segment and looked for a package
//! directory of that name. lupin now takes `--std-root` (with `LUPIN_STD`
//! as the flagless spelling) exactly as the compiler does, so all three
//! lanes are pointed at the same staged tree and nothing is duplicated.
//! F-0010 is closed; the last-segment collision rule it forced on module
//! naming is gone with it.

use std::fs;
use std::path::{Path, PathBuf};

/// One staged package root: where the entry landed, and the std root the
/// wolf lane must be pointed at.
#[derive(Debug, Clone)]
pub struct Staged {
    /// The staged entry file (absolute).
    pub entry: PathBuf,
    /// The staged std tree's root (absolute) — `--std-root`'s argument.
    pub std_root: PathBuf,
}

/// Stage one test into `scratch`: the entry file, the `std/` tree, and
/// the lupin mirror described in the module header.
pub fn stage_test(entry: &Path, std_root: &Path, scratch: &Path) -> Result<Staged, String> {
    if scratch.exists() {
        fs::remove_dir_all(scratch)
            .map_err(|e| format!("stage: clearing {}: {e}", show(scratch)))?;
    }
    fs::create_dir_all(scratch).map_err(|e| format!("stage: mkdir: {e}"))?;
    let file_name = entry
        .file_name()
        .ok_or_else(|| format!("stage: entry has no file name: {}", show(entry)))?;
    copy_file(entry, &scratch.join(file_name))?;

    let staged_std = scratch.join("std");
    copy_tree(std_root, &staged_std)?;
    Ok(Staged {
        entry: scratch.join(file_name),
        std_root: staged_std,
    })
}

fn sorted_entries(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut entries: Vec<PathBuf> = Vec::new();
    for e in fs::read_dir(dir).map_err(|e| format!("stage: read {}: {e}", show(dir)))? {
        entries.push(
            e.map_err(|e| format!("stage: read {}: {e}", show(dir)))?
                .path(),
        );
    }
    entries.sort();
    Ok(entries)
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| format!("stage: mkdir {}: {e}", show(to)))?;
    for path in sorted_entries(from)? {
        deny_symlink(&path)?;
        let dest = to.join(path.file_name().unwrap_or_default());
        if path.is_dir() {
            copy_tree(&path, &dest)?;
        } else {
            copy_file(&path, &dest)?;
        }
    }
    Ok(())
}

fn copy_file(from: &Path, to: &Path) -> Result<(), String> {
    // Byte-identical: read + write, never a platform copy with metadata
    // surprises. Spans and stdout hashes are byte-exact.
    let bytes = fs::read(from).map_err(|e| format!("stage: read {}: {e}", show(from)))?;
    fs::write(to, bytes).map_err(|e| format!("stage: write {}: {e}", show(to)))
}

fn deny_symlink(path: &Path) -> Result<(), String> {
    let meta =
        fs::symlink_metadata(path).map_err(|e| format!("stage: stat {}: {e}", show(path)))?;
    if meta.file_type().is_symlink() {
        return Err(format!(
            "stage: {} is a symlink — the staged tree must be plain files \
             (windows tier-1)",
            show(path)
        ));
    }
    Ok(())
}

/// Slash paths in every message, on every OS.
pub fn show(p: &Path) -> String {
    p.display().to_string().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("wolf-std-stage-test-{name}"));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn stages_entry_beside_tree() {
        let root = scratch("round-trip");
        let std_root = root.join("std");
        fs::create_dir_all(std_root.join("prelude")).unwrap();
        fs::create_dir_all(std_root.join("x/deque_int")).unwrap();
        fs::write(std_root.join("prelude/prelude.lu"), "//! member: true\n").unwrap();
        fs::write(std_root.join("x/README.md"), "nursery\n").unwrap();
        fs::write(std_root.join("x/deque_int/d.lu"), "//! member: true\n").unwrap();
        fs::write(root.join("entry.lu"), "//! check: pass\n//! phase: run\n").unwrap();

        let staged = stage_test(&root.join("entry.lu"), &std_root, &root.join("stage")).unwrap();
        assert!(staged.entry.ends_with("entry.lu"));
        assert_eq!(staged.std_root, root.join("stage/std"));
        // The tree is staged as `std/` — what `--std-root` points at, at
        // every depth.
        let member = root.join("stage/std/prelude/prelude.lu");
        assert_eq!(fs::read(member).unwrap(), b"//! member: true\n");
        assert!(root.join("stage/std/x/deque_int/d.lu").is_file());
        // Nothing is mirrored any more (sc04): every lane takes
        // `--std-root`, so the staged root holds the entry and `std/`.
        assert!(!root.join("stage/prelude").exists());
        assert!(!root.join("stage/deque_int").exists());
        // Staging is repeatable (the scratch root is cleared first).
        stage_test(&root.join("entry.lu"), &std_root, &root.join("stage")).unwrap();
    }

    #[test]
    fn same_last_segment_modules_coexist() {
        // What the retired mirror could not represent: two modules whose
        // last path segment is the same. Both stage now.
        let root = scratch("collide");
        let std_root = root.join("std");
        fs::create_dir_all(std_root.join("a/dup")).unwrap();
        fs::create_dir_all(std_root.join("b/dup")).unwrap();
        fs::write(std_root.join("a/dup/x.lu"), "//! member: true\n").unwrap();
        fs::write(std_root.join("b/dup/x.lu"), "//! member: true\n").unwrap();
        fs::write(root.join("entry.lu"), "//! check: pass\n//! phase: run\n").unwrap();
        stage_test(&root.join("entry.lu"), &std_root, &root.join("stage")).unwrap();
        assert!(root.join("stage/std/a/dup/x.lu").is_file());
        assert!(root.join("stage/std/b/dup/x.lu").is_file());
    }

    #[cfg(unix)]
    #[test]
    fn refuses_symlinks() {
        let root = scratch("symlink");
        let std_root = root.join("std");
        fs::create_dir_all(std_root.join("prelude")).unwrap();
        fs::write(root.join("real.lu"), "x").unwrap();
        std::os::unix::fs::symlink(root.join("real.lu"), std_root.join("prelude/link.lu")).unwrap();
        fs::write(root.join("entry.lu"), "e").unwrap();
        let err = stage_test(&root.join("entry.lu"), &std_root, &root.join("stage")).unwrap_err();
        assert!(err.contains("symlink"), "{err}");
    }
}
