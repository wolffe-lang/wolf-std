//! The spec/06 observation record (`[proto.record]`): parse, validate,
//! and model verdicts. The record is the whole interface to both
//! implementations — the rig never parses front-door exit codes
//! (`[proto.invoke.exit]`: tool exit 0 iff a well-formed record was
//! produced; a rejected or trapped *program* is a recorded outcome).

use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Pass,
    Fail(String),
    Exit(i64),
    Trap(String),
    Ub(String),
    Unsupported,
}

impl std::fmt::Display for Verdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Verdict::Pass => write!(f, "pass"),
            Verdict::Fail(c) => write!(f, "fail({c})"),
            Verdict::Exit(n) => write!(f, "exit({n})"),
            Verdict::Trap(k) => write!(f, "trap({k})"),
            Verdict::Ub(a) => write!(f, "ub({a})"),
            Verdict::Unsupported => write!(f, "unsupported"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Record {
    pub impl_name: String,
    pub phase_reached: String,
    pub verdict: Verdict,
    pub stdout_sha256: Option<String>,
    pub stdout_inline: Option<String>,
}

/// Validate against `[proto.record.fields]` and parse the fields the
/// rig compares. Anything malformed is a tool-level failure, loudly.
pub fn parse(stdout: &str, who: &str) -> Result<Record, String> {
    let v: Value = serde_json::from_str(stdout.trim())
        .map_err(|e| format!("{who}: stdout is not one JSON record: {e}"))?;
    let obj = v
        .as_object()
        .ok_or_else(|| format!("{who}: record is not a JSON object"))?;
    for req in [
        "protocol",
        "impl",
        "impl_version",
        "commit",
        "file",
        "phase_reached",
        "seeded",
        "diagnostics",
        "verdict",
    ] {
        if !obj.contains_key(req) {
            return Err(format!(
                "{who}: record missing required field `{req}` [proto.record.fields]"
            ));
        }
    }
    if obj["protocol"].as_i64() != Some(1) {
        return Err(format!(
            "{who}: protocol {} is not 1 [proto.record]",
            obj["protocol"]
        ));
    }
    let verdict_str = obj["verdict"]
        .as_str()
        .ok_or_else(|| format!("{who}: `verdict` is not a string"))?;
    let verdict = parse_verdict(verdict_str)
        .ok_or_else(|| format!("{who}: unknown verdict `{verdict_str}` [proto.record.verdict]"))?;
    let phase_reached = obj["phase_reached"]
        .as_str()
        .ok_or_else(|| format!("{who}: `phase_reached` is not a string"))?
        .to_string();
    if !crate::directive::PHASES.contains(&phase_reached.as_str()) {
        return Err(format!("{who}: unknown phase_reached `{phase_reached}`"));
    }
    let opt_str = |key: &str| obj.get(key).and_then(Value::as_str).map(str::to_string);
    let rec = Record {
        impl_name: obj["impl"].as_str().unwrap_or("?").to_string(),
        phase_reached,
        verdict,
        stdout_sha256: opt_str("stdout_sha256"),
        stdout_inline: opt_str("stdout_inline"),
    };
    if matches!(rec.verdict, Verdict::Exit(_))
        && rec.stdout_sha256.is_none()
        && rec.stdout_inline.is_none()
    {
        // Legal only when the program wrote no output ([proto.record.fields]);
        // treat null/null as "no output" — the hash of "" is compared then.
    }
    Ok(rec)
}

pub fn parse_verdict(s: &str) -> Option<Verdict> {
    if s == "pass" {
        return Some(Verdict::Pass);
    }
    if s == "unsupported" {
        return Some(Verdict::Unsupported);
    }
    if let Some(c) = s.strip_prefix("fail(").and_then(|x| x.strip_suffix(')')) {
        return Some(Verdict::Fail(c.to_string()));
    }
    if let Some(n) = s.strip_prefix("exit(").and_then(|x| x.strip_suffix(')')) {
        return n.parse().ok().map(Verdict::Exit);
    }
    if let Some(k) = s.strip_prefix("trap(").and_then(|x| x.strip_suffix(')')) {
        return Some(Verdict::Trap(k.to_string()));
    }
    if let Some(a) = s.strip_prefix("ub(").and_then(|x| x.strip_suffix(')')) {
        return Some(Verdict::Ub(a.to_string()));
    }
    None
}

/// sha256 of a byte string, lowercase hex — for checking a directive's
/// `stdout="…"` against the record's `stdout_sha256`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(bytes);
    let out = h.finalize();
    out.iter().map(|b| format!("{b:02x}")).collect()
}

/// Does the record's stdout match the directive's expectation? The
/// corpus rule (hello.lu): matching ignores one trailing newline.
pub fn stdout_matches(rec: &Record, expected: &str) -> bool {
    let hash = match &rec.stdout_sha256 {
        Some(h) => h.clone(),
        None => sha256_hex(b""),
    };
    let mut with_nl = expected.to_string();
    with_nl.push('\n');
    hash == sha256_hex(expected.as_bytes()) || hash == sha256_hex(with_nl.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    const OK: &str = r#"{"protocol":1,"impl":"lupin","impl_version":"0.1.0",
        "commit":"abc","file":"t.lu","phase_reached":"run","seeded":false,
        "diagnostics":[],"verdict":"exit(0)",
        "stdout_sha256":"c3ae8e12d0891aa8d4363f1c26baa0226806566f16fce94852d176e7976cfa8c",
        "stdout_inline":"3 7\n"}"#;

    #[test]
    fn parses_a_real_record() {
        let r = parse(OK, "lupin").unwrap();
        assert_eq!(r.verdict, Verdict::Exit(0));
        assert!(stdout_matches(&r, "3 7"), "trailing-newline rule");
        assert!(!stdout_matches(&r, "3 8"));
    }

    #[test]
    fn rejects_malformed_records() {
        assert!(parse("not json", "x").is_err());
        assert!(parse(r#"{"protocol":2}"#, "x").is_err(), "wrong protocol");
        let missing = OK.replace("\"seeded\":false,", "");
        assert!(parse(&missing, "x").is_err(), "missing required field");
        let bad = OK.replace("exit(0)", "explode(9)");
        assert!(parse(&bad, "x").is_err(), "unknown verdict");
    }

    #[test]
    fn verdict_grammar() {
        assert_eq!(
            parse_verdict("trap(overflow)"),
            Some(Verdict::Trap("overflow".into()))
        );
        assert_eq!(
            parse_verdict("ub(mem.ub)"),
            Some(Verdict::Ub("mem.ub".into()))
        );
        assert_eq!(
            parse_verdict("fail(E0301)"),
            Some(Verdict::Fail("E0301".into()))
        );
        assert_eq!(parse_verdict("exit(x)"), None);
    }
}
