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
//
// sc36 re-counted F-0099's gap at the 982f857 snapshot and it is FOUR
// namespaces, not one: `type` (24 anchors, s121), `os` (17, s38 onward),
// `ct` (14) and `diag` (10) are all in `anchors.json` and none is admitted
// by `[conf.anchor.ns]`. The cost is now concrete rather than theoretical
// — sc36 implements `[os.net.unix]` and its six witnesses CANNOT cite the
// clause they conform to, because `os` is neither registered nor reserved
// and `classify` would fail CI on the tag. Still not added here: the fix
// is the #120 append upstream (the precedent that admitted `pkg`), asked
// for on wolf-lang, and this list follows the clause the day it lands.
pub const REGISTERED_NS: &[&str] = &["gram", "mem", "conc", "abi", "conf", "proto", "pkg"];

/// Reserved forward namespaces, same clause.
// `test` was appended to the clause on 2026-08-11 by s39 (the built-in
// test framework's litmus tier, D34/D36) and this list never followed —
// a lag in the OTHER direction from `pkg`'s, and one that would have
// rejected a legal tag rather than accepted an unregistered one. Added at
// sc36 against the clause's own letter at 982f857; no anchor in the
// snapshot uses it, so nothing in this repository changes verdict.
pub const FORWARD_NS: &[&str] = &[
    "str", "err", "task", "proc", "sync", "generics", "arith", "ffi", "unsafe", "comptime", "perf",
    "mod", "std", "ty", "test",
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

    /// `test` is reserved by `[conf.anchor.ns]` (appended 2026-08-11 by
    /// s39) and this rig lagged it until sc36.
    #[test]
    fn test_namespace_is_reserved() {
        assert!(matches!(
            reg().classify("test.litmus"),
            Ok(TagClass::Forward)
        ));
    }

    /// F-0099's gap, pinned as a test so it cannot be forgotten: the
    /// pinned registry publishes `os.*`, `type.*`, `ct.*` and `diag.*`
    /// anchors that `[conf.anchor.ns]` does not admit, so a `conforms:`
    /// tag naming one is a CI failure here. sc36's own clause
    /// (`[os.net.unix]`) is in that set. When the upstream append lands
    /// (the #120 precedent), these four move to `REGISTERED_NS` and this
    /// test flips to `Ok(Registered)` deliberately.
    #[test]
    fn f0099_the_four_unadmitted_namespaces_still_fail() {
        let r = reg();
        for tag in ["os.net.unix", "type.byte", "ct.eval", "diag.code"] {
            assert!(
                r.classify(tag).is_err(),
                "`{tag}` classified — has [conf.anchor.ns] admitted its namespace? \
                 Then move it to REGISTERED_NS and retire F-0099."
            );
        }
    }

    #[test]
    fn rejects_malformed_registry() {
        assert!(Registry::from_json("{}").is_err());
        assert!(Registry::from_json(r#"{"version":2,"anchors":{}}"#).is_err());
        assert!(Registry::from_json("nope").is_err());
    }
}
