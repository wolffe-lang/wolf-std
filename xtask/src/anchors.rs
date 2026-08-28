//! `conforms:` tag checking against the vendored anchor registry
//! (`vendor/upstream/anchors.json`, snapshot of the pinned wolf-lang
//! `spec/anchors.json`). `[conf.tag.valid]`: a tag in a registered
//! namespace must name a registered anchor; a tag in a reserved forward
//! namespace (`std` among them, `[conf.anchor.ns]`) is legal, counted,
//! and reported as *forward* — checkable the day a spec std document
//! registers it.

use std::collections::BTreeSet;
use std::path::Path;

/// Registered namespaces and owners, spec/05 `[conf.anchor.ns]`.
// `pkg` was admitted by the clause's own letter at s115 (#120 — the
// extractor emitted 08-package.md's anchors before the clause admitted
// the namespace, and the append reconciled them); this list lagged the
// spec by nine sprints and sc24 caught it chasing the SAME class one
// document over: 10-types.md's `type.*` anchors (s121) are in the
// snapshot while [conf.anchor.ns] does not admit `type` — filed as
// F-0099 rather than added here, because this rig mirrors the clause's
// LETTER, not the extractor's output.
pub const REGISTERED_NS: &[&str] = &["gram", "mem", "conc", "abi", "conf", "proto", "pkg"];

/// Reserved forward namespaces, same clause.
pub const FORWARD_NS: &[&str] = &[
    "str", "err", "task", "proc", "sync", "generics", "arith", "ffi", "unsafe", "comptime", "perf",
    "mod", "std", "ty",
];

pub struct Registry {
    anchors: BTreeSet<String>,
}

pub enum TagClass {
    Registered,
    Forward,
}

impl Registry {
    pub fn load(repo: &Path) -> Result<Registry, String> {
        let path = repo.join("vendor/upstream/anchors.json");
        let text = std::fs::read_to_string(&path)
            .map_err(|e| format!("vendor/upstream/anchors.json: {e}"))?;
        Self::from_json(&text)
    }

    pub fn from_json(text: &str) -> Result<Registry, String> {
        let v: serde_json::Value =
            serde_json::from_str(text).map_err(|e| format!("anchors.json does not parse: {e}"))?;
        if v.get("version").and_then(|x| x.as_i64()) != Some(1) {
            return Err("anchors.json: expected `version: 1`".into());
        }
        let obj = v
            .get("anchors")
            .and_then(|a| a.as_object())
            .ok_or("anchors.json: no `anchors` object")?;
        Ok(Registry {
            anchors: obj.keys().cloned().collect(),
        })
    }

    /// Classify one `conforms:` tag; Err = CI failure.
    pub fn classify(&self, tag: &str) -> Result<TagClass, String> {
        let ns = tag.split('.').next().unwrap_or("");
        if REGISTERED_NS.contains(&ns) {
            if self.anchors.contains(tag) {
                Ok(TagClass::Registered)
            } else {
                Err(format!(
                    "`{tag}` is in registered namespace `{ns}` but names no anchor \
                     in the pinned registry [conf.tag.valid]"
                ))
            }
        } else if FORWARD_NS.contains(&ns) {
            Ok(TagClass::Forward)
        } else {
            Err(format!(
                "`{tag}`: namespace `{ns}` is neither registered nor reserved \
                 [conf.anchor.ns]"
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reg() -> Registry {
        Registry::from_json(
            r#"{"version":1,"anchors":{"gram.item.use":"01-grammar.md","proto.invoke":"06"}}"#,
        )
        .unwrap()
    }

    #[test]
    fn classifies_tags() {
        let r = reg();
        assert!(matches!(
            r.classify("gram.item.use"),
            Ok(TagClass::Registered)
        ));
        assert!(matches!(r.classify("std.prelude"), Ok(TagClass::Forward)));
        assert!(r.classify("gram.not.registered").is_err());
        assert!(r.classify("bogus.ns").is_err());
    }

    #[test]
    fn rejects_malformed_registry() {
        assert!(Registry::from_json("{}").is_err());
        assert!(Registry::from_json(r#"{"version":2,"anchors":{}}"#).is_err());
        assert!(Registry::from_json("nope").is_err());
    }
}
