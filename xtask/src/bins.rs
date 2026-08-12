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

/// The three observation lanes. `Wolf` and `Native` are the SAME binary
/// at two rungs — `conform-run --checked` (the miri-lite interpreter,
/// s23) and `conform-run --native` (compile, link, execute, s28) — and
/// they refuse different things, which is exactly why sc04 records both:
/// the checked tier refuses float literals outright while the native
/// rung executes them, and the native rung refuses `List` and globals
/// while the checked tier runs them.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Impl {
    Lupin,
    Wolf,
    Native,
}

pub const LANES: [Impl; 3] = [Impl::Lupin, Impl::Wolf, Impl::Native];

impl Impl {
    pub fn name(self) -> &'static str {
        match self {
            Impl::Lupin => "lupin",
            Impl::Wolf => "wolf",
            Impl::Native => "wolf",
        }
    }
    pub fn env_var(self) -> &'static str {
        match self {
            Impl::Lupin => "LUPIN_BIN",
            Impl::Wolf => "WOLF_BIN",
            Impl::Native => "WOLF_BIN",
        }
    }
    /// Ledger column name (the compiler's checked column is `wolfc`, its
    /// native column is `native`, the record's `impl` string otherwise).
    pub fn ledger_name(self) -> &'static str {
        match self {
            Impl::Lupin => "lupin",
            Impl::Wolf => "wolfc",
            Impl::Native => "native",
        }
    }
}

/// The native rung links against `libwolf_rt.a`, which the driver looks
/// for next to the `wolf` binary or at `$WOLF_RT_LIB`. Absent, the lane
/// is dark — and says so, like every other absence here.
pub fn native_rt(repo: &Path) -> Option<PathBuf> {
    if let Ok(v) = std::env::var("WOLF_RT_LIB") {
        let p = PathBuf::from(v);
        return p.is_file().then_some(p);
    }
    let wolf = resolve(Impl::Wolf, repo)?;
    let beside = wolf.path.parent()?.join("libwolf_rt.a");
    beside.is_file().then_some(beside)
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
                Impl::Native => "$WOLF_BIN",
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
    /// Read from the FIRST line only (see `pairing`).
    pub pin_short: Option<String>,
    /// The remaining lines of a multi-line `--version`, verbatim.
    ///
    /// wolf 0.1.0 (the r01 release) answers two lines: its own identity,
    /// then `paired with lupin 0.1.8 (reference interpreter), pin
    /// 7886559`. That second sha is the INTERPRETER's commit in the
    /// wolf-interp repository, not a wolf-lang commit, so reading it as
    /// this tool's conformance pin compares two histories and fails
    /// doctor at every pin (measured at sc11's bump). Identity is the
    /// first line's business; the rest is reported and never gated.
    pub pairing: Option<String>,
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

pub fn parse_version_line(text: &str) -> Option<VersionLine> {
    let mut lines = text.lines();
    let line = lines.next()?;
    let pairing: Vec<&str> = lines.map(str::trim).filter(|l| !l.is_empty()).collect();
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
        pairing: (!pairing.is_empty()).then(|| pairing.join(" · ")),
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
        assert_eq!(l.pairing, None);
        let w = parse_version_line("wolf 0.0.1 (pre-alpha)").unwrap();
        assert_eq!(w.pin_short, None);
        assert!(parse_version_line("garbage").is_none());
    }

    /// The r01 release's two-line `--version`: the second line names
    /// the INTERPRETER's sha, which is not this tool's pin. Identity
    /// comes from line one; the pairing is reported, never gated.
    #[test]
    fn a_pairing_line_is_not_this_tools_pin() {
        let w = parse_version_line(
            "wolf 0.1.0 (wolfgang)\n\
             paired with lupin 0.1.8 (reference interpreter), pin 7886559\n",
        )
        .unwrap();
        assert_eq!(w.name, "wolf");
        assert_eq!(w.version, "0.1.0");
        assert_eq!(w.pin_short, None);
        assert_eq!(
            w.pairing.as_deref(),
            Some("paired with lupin 0.1.8 (reference interpreter), pin 7886559")
        );
    }

    #[test]
    fn sha_shape() {
        assert!(is_full_sha("cbde62012f10bbdf63e4ec1e2ebeeed026973d23"));
        assert!(!is_full_sha("cbde620"));
        assert!(!is_full_sha("CBDE62012F10BBDF63E4EC1E2EBEEED026973D23"));
    }
}
