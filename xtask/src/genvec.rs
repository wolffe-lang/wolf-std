//! `gen-vectors` — the sc16 crypto vector generator (the committed-
//! generated-source pattern §10 ruled for unicode tables, applied to
//! test vectors): reads the vendored public corpora under
//! `vendor/vectors/` and emits the `tests/x/crypto/sha2/` vector files.
//! The generated files are COMMITTED; `gen-vectors --check` regenerates
//! in memory and fails on any byte of drift, and `cargo xtask ci` runs
//! the check, so the vendored source and the staged tests cannot
//! disagree silently.
//!
//! Sources and what is taken from each (omissions are named here and in
//! `vendor/vectors/README.md`, never silent):
//! - NIST CAVP SHS byte-oriented vectors (`cavp/*.rsp`): the SHA-256/
//!   384/512 ShortMsg, LongMsg and Monte files, EVERY vector. The
//!   SHA-1/SHA-224/SHA-512_224/SHA-512_256 files are not vendored: the
//!   algorithms are outside D53's ladder.
//! - Wycheproof v1 HKDF (`wycheproof/hkdf_sha{256,384,512}_test.json`):
//!   every `result: "valid"` vector, split into a small differential
//!   smoke file and the full native file. The nine `invalid` vectors
//!   (all `SizeTooLarge`, L = 255 * HashLen + 1) are omitted here
//!   because the API answers them with a documented `assert` trap,
//!   held by `tests/x/crypto/sha2/hkdf_expand_cap_trap.lu`.
//! - RFC 4231 / RFC 5869 vector files are hand-written tests (the RFC
//!   text is the vendored source), not generated — their hex is quoted
//!   from `vendor/vectors/rfc/*.txt` with the test-case numbers cited.

use std::fmt::Write as _;
use std::path::Path;

/// One family's naming: the CAVP file stem and the sha2 fn suffix.
const FAMILIES: &[(&str, &str)] = &[("SHA256", "256"), ("SHA384", "384"), ("SHA512", "512")];

/// Wycheproof smoke-subset rule (deterministic from the source): the
/// first `SMOKE_COUNT` valid vectors whose okm size is at most
/// `SMOKE_MAX_OKM` bytes — small enough that the interpreter lane runs
/// the file well inside the rig's ceiling, so HKDF keeps a differential
/// column per digest while the full set rides the native lane.
const SMOKE_COUNT: usize = 6;
const SMOKE_MAX_OKM: u64 = 96;

/// Short-set chunking (see the loop): at most this many vectors per
/// generated file, sized so the slowest part stays far under the rig's
/// per-test ceiling on the interpreter lane.
const SHORT_CHUNK: usize = 33;

pub fn gen_vectors(check_only: bool) -> Result<(), String> {
    let repo = crate::repo_root();
    let vec_dir = repo.join("vendor/vectors");
    let out_dir = repo.join("tests/x/crypto/sha2");
    // `wolf fmt` is law (D34) for every committed `.lu`, generated files
    // included: the emitter's output is passed through the pinned
    // formatter before it is written or compared, so the committed files
    // are canonical and the drift check compares canonical to canonical.
    let wolf = crate::bins::resolve(crate::bins::Impl::Wolf, &repo)
        .ok_or(
            "gen-vectors needs the pinned `wolf` (it runs `wolf fmt` on its \
             output) — set $WOLF_BIN or place .wolf-bin/wolf",
        )?
        .path;
    let mut drift: Vec<String> = Vec::new();
    let mut written = 0usize;
    let fmt = |name: &str, text: &str| -> Result<String, String> { fmt_lu(&wolf, name, text) };

    for (stem, suffix) in FAMILIES {
        // The short sets are chunked into parts of at most SHORT_CHUNK
        // vectors: the whole set is differential under the interpreter,
        // and per-vector cost there grows with the number of vectors
        // already run in the same program (superlinear — the finding is
        // in docs/findings.md), so one big file blows the ceiling that
        // several small ones sit far under.
        let src = vec_dir.join(format!("cavp/{stem}ShortMsg.rsp"));
        let vectors = parse_rsp_msgs(&read(&src)?, &show(&src))?;
        let parts: Vec<&[MsgVector]> = vectors.chunks(SHORT_CHUNK).collect();
        for (i, part) in parts.iter().enumerate() {
            let out = out_dir.join(format!("cavp_{}_short_p{}.lu", stem.to_lowercase(), i + 1));
            let text = fmt(
                &show(&out),
                &emit_cavp_msgs(
                    stem,
                    suffix,
                    "Short",
                    false,
                    part,
                    &format!(
                        "part {} of {} ({} vectors of the set's {})",
                        i + 1,
                        parts.len(),
                        part.len(),
                        vectors.len()
                    ),
                ),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
        {
            let src = vec_dir.join(format!("cavp/{stem}LongMsg.rsp"));
            let vectors = parse_rsp_msgs(&read(&src)?, &show(&src))?;
            let out = out_dir.join(format!("cavp_{}_long.lu", stem.to_lowercase()));
            let text = fmt(
                &show(&out),
                &emit_cavp_msgs(stem, suffix, "Long", true, &vectors, "the whole set"),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
        let src = vec_dir.join(format!("cavp/{stem}Monte.rsp"));
        let (seed, checkpoints) = parse_rsp_monte(&read(&src)?, &show(&src))?;
        let out = out_dir.join(format!("cavp_{}_monte.lu", stem.to_lowercase()));
        let text = fmt(
            &show(&out),
            &emit_cavp_monte(stem, suffix, &seed, &checkpoints),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;

        let wp = vec_dir.join(format!(
            "wycheproof/hkdf_sha{}_test.json",
            stem.trim_start_matches("SHA")
        ));
        let cases = parse_wycheproof(&read(&wp)?, &show(&wp))?;
        let smoke: Vec<&WpCase> = cases
            .iter()
            .filter(|c| c.size <= SMOKE_MAX_OKM)
            .take(SMOKE_COUNT)
            .collect();
        let out = out_dir.join(format!("wycheproof_hkdf{suffix}_smoke.lu"));
        let text = fmt(
            &show(&out),
            &emit_wycheproof(suffix, &smoke, true, cases.len()),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
        let all: Vec<&WpCase> = cases.iter().collect();
        let out = out_dir.join(format!("wycheproof_hkdf{suffix}_full.lu"));
        let text = fmt(
            &show(&out),
            &emit_wycheproof(suffix, &all, false, cases.len()),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
    }

    if !drift.is_empty() {
        return Err(format!(
            "gen-vectors --check: {} file(s) drift from their vendored source \
             (regenerate with `cargo xtask gen-vectors` and commit both):\n  {}",
            drift.len(),
            drift.join("\n  ")
        ));
    }
    if check_only {
        println!("gen-vectors --check: all generated files match their vendored sources");
    } else {
        println!("gen-vectors: {written} file(s) written");
    }
    Ok(())
}

// ---------------------------------------------------------------- parsing

struct MsgVector {
    len_bits: u64,
    msg_hex: String,
    md_hex: String,
}

/// `Len = n` / `Msg = hex` / `MD = hex` triples. CAVP quirk, handled
/// here so no generated file carries it: the `Len = 0` row spells its
/// message `00`, but the message is EMPTY — `Len` is authoritative and
/// the message is truncated to `Len / 8` bytes.
fn parse_rsp_msgs(text: &str, what: &str) -> Result<Vec<MsgVector>, String> {
    let mut out = Vec::new();
    let mut len: Option<u64> = None;
    let mut msg: Option<String> = None;
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("Len = ") {
            len = Some(v.parse().map_err(|e| format!("{what}: bad Len: {e}"))?);
        } else if let Some(v) = line.strip_prefix("Msg = ") {
            msg = Some(v.to_string());
        } else if let Some(v) = line.strip_prefix("MD = ") {
            let len_bits = len.take().ok_or(format!("{what}: MD before Len"))?;
            if len_bits % 8 != 0 {
                return Err(format!(
                    "{what}: Len {len_bits} is not byte-oriented — wrong vector set"
                ));
            }
            let raw = msg.take().ok_or(format!("{what}: MD before Msg"))?;
            let byte_len = (len_bits / 8) as usize;
            if raw.len() < byte_len * 2 {
                return Err(format!("{what}: Msg shorter than Len at Len {len_bits}"));
            }
            out.push(MsgVector {
                len_bits,
                msg_hex: raw[..byte_len * 2].to_string(),
                md_hex: v.to_lowercase(),
            });
        }
    }
    if out.is_empty() {
        return Err(format!("{what}: no vectors parsed"));
    }
    Ok(out)
}

/// `Seed = hex` then 100 `COUNT = j` / `MD = hex` checkpoints.
fn parse_rsp_monte(text: &str, what: &str) -> Result<(String, Vec<String>), String> {
    let mut seed = None;
    let mut mds = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(v) = line.strip_prefix("Seed = ") {
            seed = Some(v.to_lowercase());
        } else if let Some(v) = line.strip_prefix("MD = ") {
            mds.push(v.to_lowercase());
        }
    }
    let seed = seed.ok_or(format!("{what}: no Seed"))?;
    if mds.len() != 100 {
        return Err(format!(
            "{what}: expected 100 checkpoints, got {}",
            mds.len()
        ));
    }
    Ok((seed, mds))
}

struct WpCase {
    tc_id: u64,
    ikm: String,
    salt: String,
    info: String,
    size: u64,
    okm: String,
}

/// Every `result: "valid"` test, in file order. Anything non-valid is
/// counted and must be `SizeTooLarge` (the omission this module names);
/// a new non-valid flavour in a re-vendored file is a hard error so the
/// omission list cannot rot silently.
fn parse_wycheproof(text: &str, what: &str) -> Result<Vec<WpCase>, String> {
    let v: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{what}: bad json: {e}"))?;
    let mut out = Vec::new();
    let groups = v["testGroups"]
        .as_array()
        .ok_or(format!("{what}: no testGroups"))?;
    for g in groups {
        for t in g["tests"].as_array().ok_or(format!("{what}: no tests"))? {
            let result = t["result"].as_str().unwrap_or("");
            if result != "valid" {
                let flags: Vec<&str> = t["flags"]
                    .as_array()
                    .map(|a| a.iter().filter_map(|f| f.as_str()).collect())
                    .unwrap_or_default();
                if flags != ["SizeTooLarge"] {
                    return Err(format!(
                        "{what}: tcId {} is `{result}` with flags {flags:?} — an \
                         omission this generator has not named; extend the rules",
                        t["tcId"]
                    ));
                }
                continue;
            }
            out.push(WpCase {
                tc_id: t["tcId"].as_u64().ok_or(format!("{what}: tcId"))?,
                ikm: t["ikm"].as_str().unwrap_or_default().to_string(),
                salt: t["salt"].as_str().unwrap_or_default().to_string(),
                info: t["info"].as_str().unwrap_or_default().to_string(),
                size: t["size"].as_u64().ok_or(format!("{what}: size"))?,
                okm: t["okm"].as_str().unwrap_or_default().to_string(),
            });
        }
    }
    if out.is_empty() {
        return Err(format!("{what}: no valid vectors parsed"));
    }
    Ok(out)
}

// --------------------------------------------------------------- emission

fn emit_cavp_msgs(
    stem: &str,
    suffix: &str,
    kind: &str,
    slow: bool,
    vectors: &[MsgVector],
    coverage: &str,
) -> String {
    let mut s = String::new();
    let max_bits = vectors.iter().map(|v| v.len_bits).max().unwrap_or(0);
    let _ = write!(
        s,
        "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.sha2, arith.wrapping\n//!\n\
         //! GENERATED — `cargo xtask gen-vectors`, from\n\
         //! `vendor/vectors/cavp/{stem}{kind}Msg.rsp` (provenance and source\n\
         //! hashes: `vendor/vectors/README.md`). Do not edit by hand; ci's\n\
         //! `gen-vectors --check` holds this file byte-identical to its source.\n//!\n\
         //! NIST CAVP SHS byte-oriented {lkind}-message set for {dash},\n\
         //! {coverage}: {n} vectors here (Len = {min_bits}..{max_bits} bits),\n\
         //! each asserted through the one-shot `sum{suffix}` against its\n\
         //! published digest.\n",
        lkind = kind.to_lowercase(),
        dash = dashed(stem),
        n = vectors.len(),
        min_bits = vectors.iter().map(|v| v.len_bits).min().unwrap_or(0),
    );
    if slow {
        let _ = write!(
            s,
            "//!\n\
             //! The interpreter column is `slow` (the sc16 ledger word): the\n\
             //! same module's short-message file is `run` there, so the lane's\n\
             //! SEMANTICS are measured; a tree-walk of {kb} KB of message at\n\
             //! ~40 ms per block would blow the rig's ceiling by minutes, and\n\
             //! the split is the contract's honest-slow-skip.\n",
            kb = vectors.iter().map(|v| v.len_bits / 8).sum::<u64>() / 1024,
        );
    }
    s.push_str("\nuse std.hex\nuse std.x.crypto.sha2\n\n");
    // The helper answers a bool and each call site asserts it with a
    // LITERAL message naming the vector: the native rung refuses an
    // assert message with effects (even a parameter read), and the
    // per-line literal keeps a failing vector identifiable by its span.
    let _ = write!(
        s,
        "fn digest_matches(msg_hex: str, want: str) -> bool {{\n    let msg = hex.decode(msg_hex) else List[int]()\n    sha2.to_hex(sha2.sum{suffix}(msg)) == want\n}}\n\nfn main() -> int {{\n"
    );
    for v in vectors {
        let _ = writeln!(
            s,
            "    assert(digest_matches(\"{}\", \"{}\"), \"Len {}\")",
            v.msg_hex, v.md_hex, v.len_bits
        );
    }
    s.push_str("    0\n}\n");
    s
}

fn emit_cavp_monte(stem: &str, suffix: &str, seed: &str, checkpoints: &[String]) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.sha2, arith.wrapping\n//!\n\
         //! GENERATED — `cargo xtask gen-vectors`, from\n\
         //! `vendor/vectors/cavp/{stem}Monte.rsp` (provenance and source\n\
         //! hashes: `vendor/vectors/README.md`). Do not edit by hand; ci's\n\
         //! `gen-vectors --check` holds this file byte-identical to its source.\n//!\n\
         //! NIST CAVP SHS Monte Carlo test for {dash}: the published seed,\n\
         //! 100 checkpoints of 1000 chained digests each (the CAVS 11.x\n\
         //! byte-oriented MCT: M(i) = MD(i-3) || MD(i-2) || MD(i-1), the\n\
         //! checkpoint is MD(1002), and each checkpoint seeds the next\n\
         //! sequence) — 100000 digest computations against 100 published\n\
         //! values. The interpreter column is `slow` (sc16): the same MCT\n\
         //! would take hours on a tree-walk, and the short-message file\n\
         //! already measures that lane's semantics.\n",
        dash = dashed(stem),
    );
    s.push_str("\nuse std.hex\nuse std.x.crypto.sha2\n\n");
    s.push_str(
        "fn dup(xs: List[int]) -> List[int] {\n    var out = List[int]()\n    for v in xs {\n        (mut out).push(v)\n    }\n    out\n}\n\n",
    );
    s.push_str(
        "fn concat3(a: List[int], b: List[int], c: List[int]) -> List[int] {\n    var out = List[int]()\n    for v in a {\n        (mut out).push(v)\n    }\n    for v in b {\n        (mut out).push(v)\n    }\n    for v in c {\n        (mut out).push(v)\n    }\n    out\n}\n\n",
    );
    s.push_str("fn main() -> int {\n    var exp = List[str]()\n");
    for md in checkpoints {
        let _ = writeln!(s, "    (mut exp).push(\"{md}\")");
    }
    let _ = write!(
        s,
        "    var seed = hex.decode(\"{seed}\") else List[int]()\n\
         \x20   var j: int = 0\n\
         \x20   while j < 100 {{\n\
         \x20       var md0 = dup(seed)\n\
         \x20       var md1 = dup(seed)\n\
         \x20       var md2 = dup(seed)\n\
         \x20       var i: int = 0\n\
         \x20       while i < 1000 {{\n\
         \x20           let m = concat3(md0, md1, md2)\n\
         \x20           let md = sha2.sum{suffix}(m)\n\
         \x20           md0 = md1\n\
         \x20           md1 = md2\n\
         \x20           md2 = md\n\
         \x20           i = i + 1\n\
         \x20       }}\n\
         \x20       assert(sha2.to_hex(md2) == exp[j], \"monte checkpoint\")\n\
         \x20       seed = dup(md2)\n\
         \x20       j = j + 1\n\
         \x20   }}\n\
         \x20   0\n}}\n"
    );
    s
}

fn emit_wycheproof(suffix: &str, cases: &[&WpCase], smoke: bool, total_valid: usize) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.sha2, arith.wrapping\n//!\n\
         //! GENERATED — `cargo xtask gen-vectors`, from\n\
         //! `vendor/vectors/wycheproof/hkdf_sha{suffix}_test.json` (provenance:\n\
         //! `vendor/vectors/README.md`). Do not edit by hand; ci's\n\
         //! `gen-vectors --check` holds this file byte-identical to its source.\n//!\n"
    );
    if smoke {
        let ids: Vec<String> = cases.iter().map(|c| c.tc_id.to_string()).collect();
        let _ = write!(
            s,
            "//! Wycheproof v1 HKDF-SHA-{suffix}, the DIFFERENTIAL SMOKE SUBSET:\n\
             //! the first {n} valid vectors with okm length <= {max} bytes\n\
             //! (tcIds {ids}) — small enough that the interpreter lane runs the\n\
             //! file inside the rig's ceiling, so HKDF keeps a lupin column\n\
             //! while the full {total} ride `..._full.lu` on the native lane.\n",
            n = cases.len(),
            max = SMOKE_MAX_OKM,
            ids = ids.join(", "),
            total = total_valid,
        );
    } else {
        let _ = write!(
            s,
            "//! Wycheproof v1 HKDF-SHA-{suffix}, THE FULL VALID SET: all {total}\n\
             //! `result: \"valid\"` vectors (extract-then-expand through\n\
             //! `hkdf{suffix}`). The nine source vectors omitted across the three\n\
             //! digests are every non-valid entry — all `SizeTooLarge`,\n\
             //! L = 255 * HashLen + 1 — which this API answers with the\n\
             //! documented `assert` trap held by `hkdf_expand_cap_trap.lu`.\n\
             //! The interpreter column is `slow` (sc16): ~0.15 s per HMAC on a\n\
             //! tree-walk puts the full set minutes past the ceiling, and the\n\
             //! `..._smoke.lu` subset keeps the differential column.\n",
            total = total_valid,
        );
    }
    s.push_str("\nuse std.hex\nuse std.x.crypto.sha2\n\n");
    let _ = write!(
        s,
        "fn okm_matches(ikm_hex: str, salt_hex: str, info_hex: str, n: int, want: str) -> bool {{\n\
         \x20   let ikm = hex.decode(ikm_hex) else List[int]()\n\
         \x20   let salt = hex.decode(salt_hex) else List[int]()\n\
         \x20   let info = hex.decode(info_hex) else List[int]()\n\
         \x20   sha2.to_hex(sha2.hkdf{suffix}(salt, ikm, info, n)) == want\n}}\n\nfn main() -> int {{\n"
    );
    for c in cases {
        let _ = writeln!(
            s,
            "    assert(okm_matches(\"{}\", \"{}\", \"{}\", {}, \"{}\"), \"tc {}\")",
            c.ikm, c.salt, c.info, c.size, c.okm, c.tc_id
        );
    }
    s.push_str("    0\n}\n");
    s
}

fn dashed(stem: &str) -> String {
    format!("SHA-{}", stem.trim_start_matches("SHA"))
}

/// Pass emitted source through the pinned `wolf fmt` (a scratch file
/// under target/, formatted in place, read back). The formatter is
/// wolfc's own parser, so this also proves every generated file parses
/// at generation time rather than at first ci.
fn fmt_lu(wolf: &Path, name: &str, text: &str) -> Result<String, String> {
    let dir = crate::repo_root().join("target/genvec");
    std::fs::create_dir_all(&dir).map_err(|e| format!("mkdir {}: {e}", show(&dir)))?;
    let scratch = dir.join("fmt-scratch.lu");
    std::fs::write(&scratch, text).map_err(|e| format!("write {}: {e}", show(&scratch)))?;
    let out = std::process::Command::new(wolf)
        .arg("fmt")
        .arg(&scratch)
        .output()
        .map_err(|e| format!("run wolf fmt: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "wolf fmt failed on generated {name}:\n{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    let warn = String::from_utf8_lossy(&out.stderr);
    if warn.contains("W0301") {
        return Err(format!(
            "wolf fmt could only partially format generated {name} (W0301 — \
             syntax errors in the emitter's output)"
        ));
    }
    read(&scratch)
}

// ------------------------------------------------------------------ io

fn read(p: &Path) -> Result<String, String> {
    std::fs::read_to_string(p).map_err(|e| format!("read {}: {e}", show(p)))
}

fn show(p: &Path) -> String {
    p.display().to_string()
}

fn place(
    out: &Path,
    text: &str,
    check_only: bool,
    drift: &mut Vec<String>,
    written: &mut usize,
) -> Result<(), String> {
    if check_only {
        let existing = std::fs::read_to_string(out).unwrap_or_default();
        if existing != text {
            drift.push(show(out));
        }
        return Ok(());
    }
    if let Some(dir) = out.parent() {
        std::fs::create_dir_all(dir).map_err(|e| format!("mkdir {}: {e}", show(dir)))?;
    }
    std::fs::write(out, text).map_err(|e| format!("write {}: {e}", show(out)))?;
    *written += 1;
    println!("gen-vectors: wrote {}", show(out));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsp_len_is_authoritative_over_msg() {
        // The CAVP quirk: Len = 0 spells its message `00`, and the
        // message is EMPTY.
        let v = parse_rsp_msgs(
            "Len = 0\nMsg = 00\nMD = ABCD\n\nLen = 8\nMsg = d3\nMD = 12ef\n",
            "t",
        )
        .unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].msg_hex, "");
        assert_eq!(v[0].md_hex, "abcd");
        assert_eq!(v[1].msg_hex, "d3");
        assert_eq!(v[1].len_bits, 8);
    }

    #[test]
    fn rsp_rejects_bit_oriented_sets() {
        assert!(parse_rsp_msgs("Len = 5\nMsg = a8\nMD = 00\n", "t").is_err());
    }

    #[test]
    fn monte_needs_all_hundred_checkpoints() {
        assert!(parse_rsp_monte("Seed = ab\nMD = cd\n", "t").is_err());
    }

    #[test]
    fn wycheproof_takes_valid_and_names_its_omission() {
        let ok = r#"{"testGroups":[{"tests":[
            {"tcId":1,"result":"valid","ikm":"aa","salt":"","info":"","size":16,"okm":"bb"},
            {"tcId":2,"result":"invalid","flags":["SizeTooLarge"],"ikm":"aa","salt":"","info":"","size":8161,"okm":""}
        ]}]}"#;
        let v = parse_wycheproof(ok, "t").unwrap();
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].tc_id, 1);
        // A non-valid flavour the omission list has not named is a hard
        // error, so a re-vendored file cannot silently shrink coverage.
        let bad = r#"{"testGroups":[{"tests":[
            {"tcId":3,"result":"invalid","flags":["EmptyOkm"],"ikm":"aa","salt":"","info":"","size":0,"okm":""}
        ]}]}"#;
        assert!(parse_wycheproof(bad, "t").is_err());
    }
}
