//! Staging (`[conf.directive.member]`): the package root is the entry
//! file's directory, so a test cannot sit inside the tree it imports.
//! The runner copies the entry file plus the `std/` tree into a scratch
//! package root and runs there — byte-identical copies, sorted
//! `read_dir`, no symlinks (windows tier-1 keeps us honest).
//!
//! INTERIM (finding F-0001, filed upstream): neither implementation can
//! reach a `std/` directory through `use std.*` today — wolfc routes the
//! `std` head to a builtin stub table (`resolve_std_use`) and never
//! consults the package, and lupin resolves no nested package directory
//! at all. Until the std search path lands (s37+ plumbing), staging maps
//! each `std/<mod>/` to `<mod>/` in the scratch root and tests import
//! the bare module name (`use prelude`). The tree in this repository is
//! still the namespace (D32) — only the staged spelling is interim.

use std::fs;
use std::path::{Path, PathBuf};

/// Stage one test: returns the staged entry path (absolute).
pub fn stage_test(entry: &Path, std_root: &Path, scratch: &Path) -> Result<PathBuf, String> {
    if scratch.exists() {
        fs::remove_dir_all(scratch)
            .map_err(|e| format!("stage: clearing {}: {e}", show(scratch)))?;
    }
    fs::create_dir_all(scratch).map_err(|e| format!("stage: mkdir: {e}"))?;
    let file_name = entry
        .file_name()
        .ok_or_else(|| format!("stage: entry has no file name: {}", show(entry)))?;
    copy_file(entry, &scratch.join(file_name))?;
    for module_dir in sorted_dirs(std_root)? {
        let name = module_dir.file_name().unwrap_or_default().to_owned();
        copy_tree(&module_dir, &scratch.join(name))?;
    }
    Ok(scratch.join(file_name))
}

fn sorted_dirs(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(root).map_err(|e| format!("stage: read {}: {e}", show(root)))? {
        let entry = entry.map_err(|e| format!("stage: read {}: {e}", show(root)))?;
        let path = entry.path();
        deny_symlink(&path)?;
        if path.is_dir() {
            dirs.push(path);
        }
    }
    dirs.sort();
    Ok(dirs)
}

fn copy_tree(from: &Path, to: &Path) -> Result<(), String> {
    fs::create_dir_all(to).map_err(|e| format!("stage: mkdir {}: {e}", show(to)))?;
    let mut entries: Vec<PathBuf> = Vec::new();
    for e in fs::read_dir(from).map_err(|e| format!("stage: read {}: {e}", show(from)))? {
        entries.push(
            e.map_err(|e| format!("stage: read {}: {e}", show(from)))?
                .path(),
        );
    }
    entries.sort();
    for path in entries {
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
    fn stages_entry_beside_module_tree() {
        let root = scratch("round-trip");
        let std_root = root.join("std");
        fs::create_dir_all(std_root.join("prelude")).unwrap();
        fs::write(std_root.join("prelude/prelude.lu"), "//! member: true\n").unwrap();
        fs::write(root.join("entry.lu"), "//! check: pass\n//! phase: run\n").unwrap();

        let staged = stage_test(&root.join("entry.lu"), &std_root, &root.join("stage")).unwrap();
        assert!(staged.ends_with("entry.lu"));
        // The layout law after staging: std/<mod> is reachable as <mod>/
        // in the package root (the F-0001 interim, documented above).
        let member = root.join("stage/prelude/prelude.lu");
        assert_eq!(fs::read(member).unwrap(), b"//! member: true\n");
        // Staging is repeatable (the scratch root is cleared first).
        stage_test(&root.join("entry.lu"), &std_root, &root.join("stage")).unwrap();
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
