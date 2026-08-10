//! The spec/05 §2a directive block, parsed verbatim (`[conf.directive.*]`).
//! Tests in this repository reuse the corpus contract — no dialect — so
//! both implementations already parse what the rig parses.

/// Canonical phase ladder (`[conf.directive.phase]`).
pub const PHASES: &[&str] = &[
    "none",
    "lex",
    "parse",
    "resolve",
    "typecheck",
    "mem",
    "wir",
    "run",
];

/// Closed trap-kind set (`[conf.trap.set]`); expectations name kinds,
/// never exit numbers (`[conf.trap.exit]`).
pub const TRAP_KINDS: &[&str] = &[
    "overflow",
    "div-zero",
    "bounds",
    "use-after-move",
    "exclusivity",
    "region-fault",
    "stale-handle",
    "alloc-contract",
    "assert",
    "race",
    "ub",
    "deadlock",
];

#[derive(Debug, Clone, PartialEq)]
pub enum ExitExpect {
    Code(i64),
    /// `trap` (kind unspecified) or `trap(kind)`.
    Trap(Option<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Check {
    Pass,
    Fail(String),
    Run {
        exit: ExitExpect,
        stdout: Option<String>,
    },
}

#[derive(Debug, Default)]
pub struct Directive {
    pub check: Option<Check>,
    pub phase: Option<String>,
    pub conforms: Vec<String>,
    pub member: Option<bool>,
}

impl Directive {
    /// `[conf.directive.member]`: an entry file must carry both `check:`
    /// and `phase:`; `member: true` files carry neither.
    pub fn validate_entry(&self, file: &str) -> Result<(), String> {
        if self.member == Some(true) {
            return Err(format!(
                "{file}: `member: true` — a member file is never conform-run directly \
                 [conf.directive.member]"
            ));
        }
        if self.check.is_none() || self.phase.is_none() {
            return Err(format!(
                "{file}: an entry file must carry both `check:` and `phase:` \
                 [conf.directive.member]"
            ));
        }
        Ok(())
    }
}

/// Parse the leading `//!` block of a `.lu` file. Non-directive `//!`
/// lines are prose (`[conf.directive.block]`); each key appears at most
/// once, duplicates are errors.
pub fn parse(src: &str, file: &str) -> Result<Directive, String> {
    let mut d = Directive::default();
    for line in src.lines() {
        let Some(rest) = line.strip_prefix("//!") else {
            break;
        };
        let rest = rest.trim();
        let Some((key, val)) = rest.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if !matches!(key, "check" | "phase" | "conforms" | "member") {
            continue; // prose that happens to contain a colon
        }
        let val = val.trim();
        match key {
            "check" => {
                set_once(&mut d.check, parse_check(val, file)?, "check", file)?;
            }
            "phase" => {
                if !PHASES.contains(&val) {
                    return Err(format!(
                        "{file}: unknown phase `{val}` (canonical: {}) [conf.directive.phase]",
                        PHASES.join(", ")
                    ));
                }
                set_once(&mut d.phase, val.to_string(), "phase", file)?;
            }
            "conforms" => {
                if !d.conforms.is_empty() {
                    return Err(dup("conforms", file));
                }
                for tag in val.split(',') {
                    let tag = tag.trim().to_string();
                    if tag.is_empty() {
                        return Err(format!("{file}: empty `conforms:` tag"));
                    }
                    if d.conforms.contains(&tag) {
                        return Err(format!(
                            "{file}: duplicate anchor `{tag}` [conf.directive.conforms]"
                        ));
                    }
                    d.conforms.push(tag);
                }
            }
            "member" => {
                let b = match val {
                    "true" => true,
                    "false" => false,
                    other => return Err(format!("{file}: `member: {other}` is not true|false")),
                };
                set_once(&mut d.member, b, "member", file)?;
            }
            _ => unreachable!(),
        }
    }
    Ok(d)
}

fn set_once<T>(slot: &mut Option<T>, val: T, key: &str, file: &str) -> Result<(), String> {
    if slot.is_some() {
        return Err(dup(key, file));
    }
    *slot = Some(val);
    Ok(())
}

fn dup(key: &str, file: &str) -> String {
    format!("{file}: duplicate `{key}:` directive [conf.directive.block]")
}

/// `check: pass | fail(CODE) | run(exit=N | exit=trap | exit=trap(kind)
/// [, stdout="…"])` — `[conf.directive.check]`.
fn parse_check(val: &str, file: &str) -> Result<Check, String> {
    if val == "pass" {
        return Ok(Check::Pass);
    }
    if let Some(code) = val.strip_prefix("fail(").and_then(|s| s.strip_suffix(')')) {
        if code.is_empty() || !code.chars().all(|c| c.is_ascii_alphanumeric()) {
            return Err(format!("{file}: malformed `check: fail({code})`"));
        }
        return Ok(Check::Fail(code.to_string()));
    }
    let Some(args) = val.strip_prefix("run(").and_then(|s| s.strip_suffix(')')) else {
        return Err(format!(
            "{file}: malformed `check: {val}` (pass | fail(CODE) | run(…)) \
             [conf.directive.check]"
        ));
    };
    let (exit_part, stdout_part) = split_run_args(args, file)?;
    let exit = {
        let v = exit_part
            .strip_prefix("exit=")
            .ok_or_else(|| format!("{file}: `check: run(…)` must start with `exit=`"))?;
        if v == "trap" {
            ExitExpect::Trap(None)
        } else if let Some(kind) = v.strip_prefix("trap(").and_then(|s| s.strip_suffix(')')) {
            if !TRAP_KINDS.contains(&kind) {
                return Err(format!(
                    "{file}: unknown trap kind `{kind}` [conf.trap.set]"
                ));
            }
            ExitExpect::Trap(Some(kind.to_string()))
        } else {
            ExitExpect::Code(
                v.parse::<i64>()
                    .map_err(|_| format!("{file}: bad exit status `{v}`"))?,
            )
        }
    };
    let stdout = match stdout_part {
        None => None,
        Some(s) => {
            let inner = s
                .strip_prefix("stdout=\"")
                .and_then(|s| s.strip_suffix('"'))
                .ok_or_else(|| format!("{file}: malformed `stdout=` in check"))?;
            Some(unescape(inner, file)?)
        }
    };
    Ok(Check::Run { exit, stdout })
}

/// Split `exit=…[, stdout="…"]` at the first comma outside quotes.
fn split_run_args<'a>(args: &'a str, file: &str) -> Result<(&'a str, Option<&'a str>), String> {
    let mut in_str = false;
    let mut prev_backslash = false;
    for (i, c) in args.char_indices() {
        match c {
            '"' if !prev_backslash => in_str = !in_str,
            ',' if !in_str => {
                return Ok((args[..i].trim(), Some(args[i + 1..].trim())));
            }
            _ => {}
        }
        prev_backslash = c == '\\' && !prev_backslash;
    }
    if in_str {
        return Err(format!("{file}: unterminated string in `check:`"));
    }
    Ok((args.trim(), None))
}

fn unescape(s: &str, file: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('"') => out.push('"'),
            Some('\\') => out.push('\\'),
            other => return Err(format!("{file}: bad escape `\\{}`", other.unwrap_or(' '))),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_entry_block() {
        let d = parse(
            "//! check: run(exit=0, stdout=\"3 7\")\n//! phase: run\n//! conforms: std.prelude\n//!\n//! prose: not a directive\n\nfn main() {}\n",
            "t.lu",
        )
        .unwrap();
        assert_eq!(
            d.check,
            Some(Check::Run {
                exit: ExitExpect::Code(0),
                stdout: Some("3 7".to_string())
            })
        );
        assert_eq!(d.phase.as_deref(), Some("run"));
        assert_eq!(d.conforms, vec!["std.prelude"]);
        d.validate_entry("t.lu").unwrap();
    }

    #[test]
    fn parses_trap_and_fail_forms() {
        let d = parse(
            "//! check: run(exit=trap(overflow))\n//! phase: run\n",
            "t.lu",
        )
        .unwrap();
        assert_eq!(
            d.check,
            Some(Check::Run {
                exit: ExitExpect::Trap(Some("overflow".into())),
                stdout: None
            })
        );
        let d = parse("//! check: fail(E0301)\n//! phase: resolve\n", "t.lu").unwrap();
        assert_eq!(d.check, Some(Check::Fail("E0301".into())));
    }

    #[test]
    fn rejects_bad_blocks() {
        assert!(
            parse("//! check: pass\n//! check: pass\n", "t").is_err(),
            "dup"
        );
        assert!(parse("//! phase: link\n", "t").is_err(), "unknown phase");
        assert!(
            parse("//! check: run(exit=trap(oops))\n", "t").is_err(),
            "unknown trap kind"
        );
        assert!(parse("//! member: maybe\n", "t").is_err(), "bad member");
        let member = parse("//! member: true\n//! check: pass\n", "t").unwrap();
        assert!(member.validate_entry("t").is_err(), "member run directly");
        let bare = parse("//! prose only\n", "t").unwrap();
        assert!(bare.validate_entry("t").is_err(), "entry needs check+phase");
    }

    #[test]
    fn escapes_in_stdout() {
        let d = parse(
            "//! check: run(exit=0, stdout=\"a\\nb, c\\\"d\")\n//! phase: run\n",
            "t.lu",
        )
        .unwrap();
        match d.check.unwrap() {
            Check::Run {
                stdout: Some(s), ..
            } => assert_eq!(s, "a\nb, c\"d"),
            other => panic!("{other:?}"),
        }
    }
}
