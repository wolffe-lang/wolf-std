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
//
// AND AGAIN, TWO DAYS LATER — `sched`, the twelfth, at wolf-lang `ed8f526`
// (s139/#246). `spec/07-schedule-points.md` was ruled NORMATIVE and its
// seven anchors published: 424 -> 431, additive, none dropped, no owner
// moved. This one arrived the way `[conf.anchor.ns.admit]` says an
// admission should: the re-vendored snapshot landed first and
// `every_published_anchor_sits_in_a_registered_namespace` went RED naming
// `sched` and 7 — upstream's own count, reached from the other side,
// BEFORE a human read the issue. sc38 wrote that gate after finding
// sc36's mock-backed pin was green and always would have been; one pin
// bump later it fired on its first real event. The append and the
// re-vendor are one commit, per the clause.
//
// AT sc40 (v0.2.8, `5c729e8`) THIS LIST IS ZERO BEHIND, and wolf-std#10's
// filing said five. It was right when written and stale twice over by the
// time it was read: sc38 took r09's `diag`/`ct`/`type`/`os`, sc39 took
// s139's `sched`, and the clause is byte-identical between `ed8f526` and
// `5c729e8` — twelve registered, fifteen reserved, on both sides. What
// sc40 owes the issue is therefore not an entry but the GATE it asked for
// in the same breath, and the gate is below: every check that existed
// before this sprint asked the anchor REGISTRY, and the registry cannot
// see a clause that moves without publishing.
//
// WHAT THE FIVE HID, measured rather than recalled. At `6ade878` (v0.2.5,
// the pin this repository held into sc38) the four r09 namespaces
// published SEVENTY anchors between them — `type` 24, `os` 22, `ct` 14,
// `diag` 10 — every one of which `classify` rejected here while
// `[conf.tag.valid]` made citing it a CI failure. That is not an
// abstraction: sixteen `os.*` citations and sixteen `type.*` citations
// stand in `std/` and `tests/` today across six and four distinct anchors
// (`os.cpus`, `os.net.accept`, `os.net.listen.opts`, `os.net.unix`,
// `os.net.wait`, `os.proc.inherit`; `type.byte`, `type.char.cast`,
// `type.char.interp`, `type.char.order`), and not one of them could be
// written before the admission. F-0099 is what it cost while it lasted:
// sc36's six `[os.net.unix]` witnesses carried the forward tag
// `std.net.unix` with the true clause in a COMMENT, which is a citation no
// gauntlet reads, and ten `ty.byte` tags named a namespace the registry
// has never published. `sched` hid the mirror image and hid it better —
// seven anchors DECLARED by 07-schedule-points.md and published by
// nothing, cited here zero times, invisible to every gate on both tracks
// for a year, because a document nothing reads raises no alarm.
//
// Upstream fixed the ROOT CAUSE in the same change: four hand-copied
// namespace lists there (link_check's owner map, anchor_index's ownership
// match, its own REGISTERED_NS, spec_extract's document list) collapsed
// into one NS_OWNERS table. This rig keeps ONE list and derives nothing
// from it, so it has no copies to drift — what it lacked was the second
// direction of the check, added below as
// `every_registered_namespace_publishes_at_least_one_anchor`.
pub const REGISTERED_NS: &[&str] = &[
    "gram", "diag", "mem", "conc", "abi", "conf", "proto", "pkg", "ct", "type", "os", "sched",
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

/// `[conf.anchor.ns]` as DATA rather than as a hand-copy.
///
/// Every gate above this point asks the anchor REGISTRY
/// (`vendor/upstream/anchors.json`) what namespaces the pin publishes.
/// That is not the same question as what the CLAUSE registers, and the two
/// can differ in a direction the registry cannot see: a namespace admitted
/// by `[conf.anchor.ns]` whose document publishes nothing yet reads as
/// silence in `anchors.json` and as a rejected-but-legal tag here. That is
/// the restrictive half of `[conf.anchor.ns.admit]`, and it is the half
/// that cost `test` seven weeks — appended to the reserved list on
/// 2026-08-11 by s39 and not carried here until sc36, with nothing in
/// between able to notice, because a reserved namespace publishes no
/// anchors at all and the registry therefore has no opinion about it.
///
/// So the clause ships as vendored data beside the registry
/// (`vendor/upstream/spec/05-conformance.md`, byte-verified against the
/// submodule at the pin by `sync-pin`), and the tests below set-diff both
/// of this file's lists against it BOTH WAYS. wolf-interp did this first
/// at is39 (`2de528d`, wolf-interp#64); this is the mirror, and the
/// parser is deliberately the same shape so a clause change breaks both
/// tracks the same way.
pub mod clause {
    use std::collections::BTreeSet;
    use std::path::Path;

    /// The vendored clause document. Snapshot, not submodule: CI cannot
    /// clone the private upstream repo, which is the whole reason
    /// `vendor/upstream/` exists.
    pub fn text(repo: &Path) -> Result<String, String> {
        let path = repo.join("vendor/upstream/spec/05-conformance.md");
        std::fs::read_to_string(&path)
            .map_err(|e| format!("vendor/upstream/spec/05-conformance.md: {e}"))
    }

    /// The backticked token at each `` `ns` → owner `` pair. Splitting on a
    /// backtick alternates outside/inside, so odd indices are the spans
    /// BETWEEN backticks; the chunk following one decides whether it is a
    /// registration or prose, and an arrow is the registration.
    fn owners(text: &str) -> BTreeSet<String> {
        let parts: Vec<&str> = text.split('`').collect();
        parts
            .iter()
            .enumerate()
            .filter(|(at, _)| at % 2 == 1)
            .filter(|(at, token)| {
                !token.is_empty()
                    && token.chars().all(|c| c.is_ascii_lowercase())
                    && parts
                        .get(at + 1)
                        .is_some_and(|after| after.trim_start().starts_with('\u{2192}'))
            })
            .map(|(_, token)| (*token).to_owned())
            .collect()
    }

    /// Every backticked all-lowercase token in `text`.
    fn tokens(text: &str) -> BTreeSet<String> {
        text.split('`')
            .skip(1)
            .step_by(2)
            .filter(|t| !t.is_empty() && t.chars().all(|c| c.is_ascii_lowercase()))
            .map(ToOwned::to_owned)
            .collect()
    }

    /// The `[conf.anchor.ns]` bullet's two lists: registered, reserved.
    ///
    /// Every `expect`-shaped failure here is a CLAUSE SHAPE change, which
    /// is a thing this repository must be told about rather than parse
    /// around — the parenthetical between the two lists is prose and grows
    /// every time a namespace is admitted, so the split points are the
    /// stable headings and not offsets.
    pub fn lists(text: &str) -> Result<(BTreeSet<String>, BTreeSet<String>), String> {
        let bullet = text
            .split_once("- `[conf.anchor.ns]`")
            .ok_or("the vendored spec carries no `[conf.anchor.ns]` bullet")?
            .1;
        let (registered_half, reserved_half) = bullet
            .split_once("**Reserved forward namespaces**")
            .ok_or("the `[conf.anchor.ns]` bullet has no reserved-namespace heading")?;
        let reserved_half = reserved_half
            .split_once("*forward*):")
            .ok_or("the reserved list does not follow the `*forward*):` parenthetical")?
            .1
            .split_once("A tag outside")
            .ok_or("the reserved list does not end at the `A tag outside` sentence")?
            .0;
        Ok((owners(registered_half), tokens(reserved_half)))
    }
}

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

    /// The admission gate's OTHER half, and the direction r09's guards
    /// could not see: a name in `REGISTERED_NS` that the pinned registry
    /// publishes nothing for. That is the permissive side of
    /// `[conf.anchor.ns.admit]` — it rejects nothing and fails nothing, so
    /// it can only be found by asking. Two ways to get there: a name
    /// appended here speculatively before upstream published it (an
    /// admission running AHEAD of the clause instead of behind it, which
    /// is how `pkg` lagged nine sprints in the mirror), or a pin bump that
    /// withdraws a namespace's last anchor and leaves the entry stranded.
    /// Either way the list has stopped describing the registry, which is
    /// the only thing it is for. sc39 added this beside `sched`'s
    /// admission because the issue that asked for `sched` asked for this
    /// in the same breath, and because upstream's own s139 fix was
    /// structural rather than instance-shaped.
    #[test]
    fn every_registered_namespace_publishes_at_least_one_anchor() {
        let r = Registry::load(&crate::repo_root()).unwrap();
        let published: BTreeSet<&str> = r
            .anchors
            .iter()
            .filter_map(|a| a.split('.').next())
            .collect();
        let barren: Vec<&&str> = REGISTERED_NS
            .iter()
            .filter(|ns| !published.contains(*ns as &str))
            .collect();
        assert!(
            barren.is_empty(),
            "REGISTERED_NS admits namespace(s) {barren:?} that the pinned registry \
             publishes NO anchor for — either the admission ran ahead of \
             `[conf.anchor.ns]` (a name added here before upstream published it) or \
             a pin bump withdrew the last anchor and stranded the entry. A namespace \
             with nothing in it registers nothing; move it to FORWARD_NS if it is a \
             reservation, or drop it."
        );
    }

    fn clause_lists() -> (BTreeSet<String>, BTreeSet<String>) {
        let text = clause::text(&crate::repo_root()).unwrap();
        clause::lists(&text).unwrap()
    }

    /// The negative control the three tests below rest on. A parser that
    /// silently matched nothing would make every set-difference vacuously
    /// empty, which is #246's own defect wearing a test's clothes — and it
    /// is the failure mode sc38 found in sc36's mock-backed pin, one
    /// mechanism over. Asserted against the clause's SHAPE, not its
    /// contents, so an admission does not have to edit this test.
    #[test]
    fn the_clause_parse_finds_the_lists_rather_than_finding_nothing() {
        let (registered, reserved) = clause_lists();
        assert!(
            registered.len() >= 12,
            "the registered list parsed to {registered:?} — `[conf.anchor.ns]`'s \
             shape moved and the parser followed it into silence"
        );
        assert!(registered.contains("gram") && registered.contains("conf"));
        assert!(
            reserved.len() >= 15,
            "the reserved list parsed to {reserved:?} — the clause's shape moved"
        );
        assert!(reserved.contains("str") && reserved.contains("test"));
    }

    /// `[conf.anchor.ns.admit]` read against the CLAUSE, both ways — the
    /// gate wolf-std#10 asked for and the one thing the registry-backed
    /// gates above cannot supply.
    ///
    /// They ask `anchors.json` what the pin PUBLISHES. This asks
    /// `05-conformance.md` what the pin REGISTERS, and the difference is
    /// not academic: a namespace admitted by the clause whose document has
    /// published nothing yet is invisible to every registry gate, while
    /// `classify` here rejects its tags as unregistered. That is exactly
    /// how `sched` sat at ed8f526 for the pin before it, and exactly how
    /// `test` sat reserved-and-uncarried from 2026-08-11 to sc36 — seven
    /// weeks in which no gate anywhere could hold an opinion, because a
    /// reserved namespace publishes no anchors to have one about.
    ///
    /// This test reds the next time `[conf.anchor.ns]` moves and this file
    /// does not, in whichever direction it moves.
    #[test]
    fn the_pinned_clause_registers_exactly_what_this_file_registers() {
        let (clause, _) = clause_lists();
        let ours: BTreeSet<String> = REGISTERED_NS.iter().map(|s| (*s).to_owned()).collect();
        let missing_here: Vec<&String> = clause.difference(&ours).collect();
        let extra_here: Vec<&String> = ours.difference(&clause).collect();
        assert!(
            missing_here.is_empty(),
            "[conf.anchor.ns.admit], RESTRICTIVE half: the pinned clause registers \
             {missing_here:?} and REGISTERED_NS does not, so this rig REJECTS a tag \
             that is legal upstream. Append them in the SAME change as the pin bump."
        );
        assert!(
            extra_here.is_empty(),
            "[conf.anchor.ns.admit], PERMISSIVE half: REGISTERED_NS admits \
             {extra_here:?} and the pinned clause does not. Either the admission ran \
             ahead of `[conf.anchor.ns]` (file it upstream, do not keep it here) or \
             the entry is a typo."
        );
    }

    /// The same read for the reserved list, which is the one that went
    /// unchecked longest. `test` was appended upstream on 2026-08-11 and
    /// carried here at sc36; nothing could have told anyone in between,
    /// because a reserved namespace publishes no anchors and the registry
    /// gates are all the rig had.
    #[test]
    fn the_pinned_clause_reserves_exactly_what_this_file_reserves() {
        let (_, clause) = clause_lists();
        let ours: BTreeSet<String> = FORWARD_NS.iter().map(|s| (*s).to_owned()).collect();
        let missing_here: Vec<&String> = clause.difference(&ours).collect();
        let extra_here: Vec<&String> = ours.difference(&clause).collect();
        assert!(
            missing_here.is_empty(),
            "the pinned clause reserves {missing_here:?} and FORWARD_NS does not — a \
             forward tag that is legal upstream is a CI failure here (`test`'s own \
             seven weeks, 2026-08-11 to sc36)"
        );
        assert!(
            extra_here.is_empty(),
            "FORWARD_NS reserves {extra_here:?} and the pinned clause does not — this \
             rig would classify an unregistered tag as forward and let it through. \
             Unlike wolf-interp's `repl`, this repository claims no reservation of \
             its own: `std` is upstream's."
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
