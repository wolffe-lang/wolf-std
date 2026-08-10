//! A strict subset-of-TOML parser for this repository's own data files
//! (`tests/ledger.toml`, `vendor/tools.toml`). Deliberately tiny so the
//! rig stays dependency-light (sc00): sections `[name]` or
//! `[name."quoted key"]`, entries `key = "string"`, `#` comments, LF.
//! Anything outside that subset is an error, loudly — these are files we
//! author, not files we receive.

/// One parsed section: header parts + key/value pairs with line numbers.
#[derive(Debug, PartialEq)]
pub struct Section {
    /// `[tests."prelude/x.lu"]` → `("tests", Some("prelude/x.lu"))`.
    pub name: String,
    pub key: Option<String>,
    pub entries: Vec<(String, String, usize)>,
}

pub fn parse(text: &str, what: &str) -> Result<Vec<Section>, String> {
    let mut sections: Vec<Section> = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        let lineno = idx + 1;
        let line = strip_comment(raw).trim().to_string();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix('[') {
            let inner = rest
                .strip_suffix(']')
                .ok_or_else(|| format!("{what}:{lineno}: unterminated section header"))?;
            let (name, key) = parse_header(inner).map_err(|e| format!("{what}:{lineno}: {e}"))?;
            sections.push(Section {
                name,
                key,
                entries: Vec::new(),
            });
        } else {
            let (k, v) = parse_kv(&line).map_err(|e| format!("{what}:{lineno}: {e}"))?;
            let section = sections
                .last_mut()
                .ok_or_else(|| format!("{what}:{lineno}: entry before any [section]"))?;
            if section.entries.iter().any(|(ek, _, _)| *ek == k) {
                return Err(format!("{what}:{lineno}: duplicate key `{k}`"));
            }
            section.entries.push((k, v, lineno));
        }
    }
    Ok(sections)
}

fn strip_comment(line: &str) -> &str {
    let mut in_str = false;
    for (i, c) in line.char_indices() {
        match c {
            '"' => in_str = !in_str,
            '#' if !in_str => return &line[..i],
            _ => {}
        }
    }
    line
}

fn parse_header(inner: &str) -> Result<(String, Option<String>), String> {
    match inner.split_once('.') {
        None => {
            check_bare(inner)?;
            Ok((inner.to_string(), None))
        }
        Some((name, quoted)) => {
            check_bare(name)?;
            let key = quoted
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .ok_or_else(|| {
                    format!("subsection key must be quoted: `{quoted}` (subset rule)")
                })?;
            if key.contains('"') || key.contains('\\') {
                return Err(format!("no escapes in subsection keys: `{key}`"));
            }
            Ok((name.to_string(), Some(key.to_string())))
        }
    }
}

fn parse_kv(line: &str) -> Result<(String, String), String> {
    let (k, v) = line
        .split_once('=')
        .ok_or_else(|| format!("expected `key = \"value\"`, got `{line}`"))?;
    let k = k.trim();
    check_bare(k)?;
    let v = v.trim();
    let v = v
        .strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .ok_or_else(|| format!("value for `{k}` must be a quoted string (subset rule)"))?;
    if v.contains('"') || v.contains('\\') {
        return Err(format!("no escapes in values: `{v}`"));
    }
    Ok((k.to_string(), v.to_string()))
}

fn check_bare(s: &str) -> Result<(), String> {
    if !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    {
        Ok(())
    } else {
        Err(format!("bare key/name must be [A-Za-z0-9_-]+: `{s}`"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ledger_shape() {
        let doc = parse(
            "# comment\n[tests.\"prelude/hello.lu\"]\nlupin = \"run\" # ok\nwolfc = \"fail(E0301)\"\n",
            "t",
        )
        .unwrap();
        assert_eq!(doc.len(), 1);
        assert_eq!(doc[0].name, "tests");
        assert_eq!(doc[0].key.as_deref(), Some("prelude/hello.lu"));
        assert_eq!(doc[0].entries[0].0, "lupin");
        assert_eq!(doc[0].entries[1].1, "fail(E0301)");
    }

    #[test]
    fn rejects_outside_subset() {
        assert!(parse("[t]\nkey = 5\n", "t").is_err(), "unquoted value");
        assert!(parse("key = \"v\"\n", "t").is_err(), "entry before section");
        assert!(
            parse("[t]\nk = \"a\"\nk = \"b\"\n", "t").is_err(),
            "dup key"
        );
        assert!(
            parse("[t\nk = \"a\"\n", "t").is_err(),
            "unterminated header"
        );
    }
}
