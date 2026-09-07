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
//
// IT LANDED. `diag`, `ct`, `type` and `os` appended to `[conf.anchor.ns]`
// upstream at wolf-lang v0.2.6 (`a369b22`, #239 — and #165, the same gap
// filed ten days earlier by a lane that had not found the first filing),
// on the #120 precedent exactly: additive, nothing renumbered,
// `[conf.anchor.stable]` untouched. Seventy-one published anchors stood
// outside the clause's letter at the moment of the append. This list
// follows it here, at the same pin bump, per the clause's own new
// `[conf.anchor.ns.admit]` ("in one change or not at all"), and F-0099
// retires. `ty` is NOT touched: 10-types.md chose `type`, `ty` stays
// RESERVED below, and withdrawing a reservation is the one direction
// that can reject legal input. The two lists are different names and
// both stand — `no_namespace_is_registered_and_reserved_at_once` holds
// them apart.
pub const REGISTERED_NS: &[&str] = &[
    "gram", "diag", "mem", "conc", "abi", "conf", "proto", "pkg", "ct", "type", "os",
];

/// Reserved forward namespaces, same clause.
// `test` was appended to the clause on 2026-08-11 by s39 (the built-in
// test framework's litmus tier, D34/D36) and this list never followed —
// a lag in the OTHER direction from `pkg`'s, and one that would have
// rejected a legal tag rather than accepted an unregistered one. Added at
// sc36 against the clause's own letter at 982f857; no anchor in the
// snapshot uses it, so nothing in this repository changes verdict.
//
// `ty` stays here at sc38, reserved, while `type` joins REGISTERED_NS
// above. It is reserved-and-UNUSED upstream and reserved-and-USED here:
// twelve wolf-std tags cite it, and only ten of them (`ty.byte`) name a
// clause the registry actually publishes (`type.byte`). Those ten move to
// `type.byte` at sc38. `ty.match.exhaustive` and `ty.method.receiver-mode`
// name clauses NO document has written yet, so they stay forward — which
// is what a reserved namespace is for, and the concrete reason not to
// tidy `ty` away.
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

    /// `[conf.anchor.ns.admit]`'s downstream half, and the gate F-0099
    /// needed and never had. sc36 pinned the gap against the two-anchor
    /// MOCK above: `classify("os.net.unix")` errs whether `os` is
    /// unregistered (the gap) or registered-with-no-such-anchor (the mock),
    /// so the pin could not see the event it was pinning — measured at
    /// sc38, green at the v0.2.6 snapshot with the gap already closed
    /// upstream. This one asks the PINNED REGISTRY instead, which is the
    /// only place the answer lives: every anchor the snapshot publishes
    /// must sit in a namespace this list registers. At the sc37 snapshot
    /// it names 70 violations across `os`/`type`/`ct`/`diag`; at v0.2.6
    /// with the four admitted it is silent. It reds at the NEXT pin bump
    /// that publishes a namespace this rig has not admitted — which is
    /// the whole of `[conf.anchor.ns.admit]`'s "silent on whichever side
    /// is permissive".
    #[test]
    fn every_published_anchor_sits_in_a_registered_namespace() {
        let r = Registry::load(&crate::repo_root()).unwrap();
        let mut stranded: BTreeSet<&str> = BTreeSet::new();
        let mut count = 0usize;
        for a in &r.anchors {
            let ns = a.split('.').next().unwrap_or("");
            if !REGISTERED_NS.contains(&ns) {
                stranded.insert(ns);
                count += 1;
            }
        }
        assert!(
            stranded.is_empty(),
            "the pinned registry publishes {count} anchors in namespace(s) {stranded:?} \
             that REGISTERED_NS does not admit — [conf.tag.valid] makes citing any one \
             of them a CI failure here. If [conf.anchor.ns] admits them upstream, \
             append them here in the SAME change ([conf.anchor.ns.admit]) and retire \
             F-0099."
        );
    }

    /// The other direction, the one that rejects LEGAL input: a namespace
    /// may be registered or reserved, never both. `ty` is reserved and
    /// `type` is registered; they are different names and both stand.
    #[test]
    fn no_namespace_is_registered_and_reserved_at_once() {
        for ns in FORWARD_NS {
            assert!(
                !REGISTERED_NS.contains(ns),
                "`{ns}` is in both lists — admission is one-way \
                 ([conf.anchor.ns.admit]) and a forward tag would be \
                 silently re-classified"
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
