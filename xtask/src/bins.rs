//! Binary acquisition honesty (the ls00 doctrine, adopted verbatim):
//! this repository consumes `lupin` and `wolf` as *binaries*, never as
//! source, never as cargo deps. Resolution order per implementation:
//! `$LUPIN_BIN` / `$WOLF_BIN` → `.wolf-bin/` → `PATH`. Absence is legal
//! and LOUD; a version that contradicts the recorded pin is worse than
//! absence and fails doctor.

use crate::exec;
use crate::tomlite;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Impl {
    Lupin,
    Wolf,
}

impl Impl {
    pub fn name(self) -> &'static str {
        match self {
            Impl::Lupin => "lupin",
            Impl::Wolf => "wolf",
        }
    }
    pub fn env_var(self) -> &'static str {
        match self {
            Impl::Lupin => "LUPIN_BIN",
            Impl::Wolf => "WOLF_BIN",
        }
    }
    /// Ledger column name (the compiler's column is `wolfc`, the record's
    /// `impl` string).
    pub fn ledger_name(self) -> &'static str {
        match self {
            Impl::Lupin => "lupin",
            Impl::Wolf => "wolfc",
        }
    }
}

#[derive(Debug)]
pub struct Resolved {
    pub path: PathBuf,
    /// Which rung of the resolution order won ("$LUPIN_BIN", ".wolf-bin",
    /// "PATH").
    pub source: &'static str,
}

pub fn resolve(imp: Impl, repo: &Path) -> Option<Resolved> {
    if let Ok(v) = std::env::var(imp.env_var()) {
        // An explicitly set env var must point at a real file — a broken
        // override is an error surface, not a silent fallthrough.
        let p = PathBuf::from(v);
        return p.is_file().then_some(Resolved {
            path: p,
            source: match imp {
                Impl::Lupin => "$LUPIN_BIN",
                Impl::Wolf => "$WOLF_BIN",
            },
        });
    }
    let exe = if cfg!(windows) {
        format!("{}.exe", imp.name())
    } else {
        imp.name().to_string()
    };
    let cached = repo.join(".wolf-bin").join(&exe);
    if cached.is_file() {
        return Some(Resolved {
            path: cached,
            source: ".wolf-bin",
        });
    }
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let cand = dir.join(&exe);
        if cand.is_file() {
            return Some(Resolved {
                path: cand,
                source: "PATH",
            });
        }
    }
    None
}

/// Recorded pins for the two binaries (`vendor/tools.toml`).
#[derive(Debug, Clone)]
pub struct ToolPin {
    pub version: String,
    /// wolf-lang commit the binary conforms to (lupin names it in
    /// `--version`; wolf cannot yet — doctor reports that honestly).
    pub pin: String,
}

pub fn load_tool_pins(repo: &Path) -> Result<(ToolPin, ToolPin), String> {
    let path = repo.join("vendor/tools.toml");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("vendor/tools.toml: {e}"))?;
    let sections = tomlite::parse(&text, "vendor/tools.toml")?;
    let mut lupin = None;
    let mut wolf = None;
    for s in sections {
        let get = |key: &str| -> Result<String, String> {
            s.entries
                .iter()
                .find(|(k, _, _)| k == key)
                .map(|(_, v, _)| v.clone())
                .ok_or_else(|| format!("vendor/tools.toml: [{}] misses `{key}`", s.name))
        };
        let pin = ToolPin {
            version: get("version")?,
            pin: get("pin")?,
        };
        if !is_full_sha(&pin.pin) {
            return Err(format!(
                "vendor/tools.toml: [{}] pin `{}` is not a 40-hex commit",
                s.name, pin.pin
            ));
        }
        match s.name.as_str() {
            "lupin" => lupin = Some(pin),
            "wolf" => wolf = Some(pin),
            other => return Err(format!("vendor/tools.toml: unknown section [{other}]")),
        }
    }
    match (lupin, wolf) {
        (Some(l), Some(w)) => Ok((l, w)),
        _ => Err("vendor/tools.toml: needs both [lupin] and [wolf]".into()),
    }
}

pub fn is_full_sha(s: &str) -> bool {
    s.len() == 40
        && s.chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
}

#[derive(Debug, PartialEq)]
pub struct VersionLine {
    pub name: String,
    pub version: String,
    /// Short pin when the tool names one: `lupin 0.1.0 (wolf-interp, pin
    /// cbde620)` → `Some("cbde620")`; `wolf 0.0.1 (pre-alpha)` → `None`.
    pub pin_short: Option<String>,
}

pub fn probe_version(bin: &Path) -> Result<VersionLine, String> {
    let mut cmd = Command::new(bin);
    cmd.arg("--version");
    let got = exec::run(cmd, Duration::from_secs(10))
        .map_err(|e| format!("{}: {e}", crate::stage::show(bin)))?;
    if got.timed_out || got.status != Some(0) {
        return Err(format!(
            "{} --version failed (status {:?}): {}",
            crate::stage::show(bin),
            got.status,
            got.stderr.trim()
        ));
    }
    parse_version_line(got.stdout.trim())
        .ok_or_else(|| format!("unrecognized --version line: `{}`", got.stdout.trim()))
}

pub fn parse_version_line(line: &str) -> Option<VersionLine> {
    let mut words = line.split_whitespace();
    let name = words.next()?.to_string();
    let version = words.next()?.to_string();
    if !version.chars().next()?.is_ascii_digit() {
        return None;
    }
    let rest: String = words.collect::<Vec<_>>().join(" ");
    let pin_short = rest
        .split("pin ")
        .nth(1)
        .map(|s| s.trim_end_matches(')').trim().to_string())
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit()));
    Some(VersionLine {
        name,
        version,
        pin_short,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_both_version_shapes() {
        let l = parse_version_line("lupin 0.1.0 (wolf-interp, pin cbde620)").unwrap();
        assert_eq!(l.name, "lupin");
        assert_eq!(l.version, "0.1.0");
        assert_eq!(l.pin_short.as_deref(), Some("cbde620"));
        let w = parse_version_line("wolf 0.0.1 (pre-alpha)").unwrap();
        assert_eq!(w.pin_short, None);
        assert!(parse_version_line("garbage").is_none());
    }

    #[test]
    fn sha_shape() {
        assert!(is_full_sha("cbde62012f10bbdf63e4ec1e2ebeeed026973d23"));
        assert!(!is_full_sha("cbde620"));
        assert!(!is_full_sha("CBDE62012F10BBDF63E4EC1E2EBEEED026973D23"));
    }
}
