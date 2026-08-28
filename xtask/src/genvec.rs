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
//! - Wycheproof v1 ChaCha20-Poly1305 (sc17,
//!   `wycheproof/chacha20_poly1305_test.json`, into
//!   `tests/x/crypto/chacha20/`): every `valid` vector (sealed AND
//!   opened) and every `invalid` vector (open must answer the `tag`
//!   row) — the invalid cases are the corpus's point: 60 modified
//!   tags. The nine `InvalidNonceSize` vectors are omitted here
//!   because the API answers a wrong-size nonce with a documented
//!   `assert` trap, held by `tests/x/crypto/chacha20/nonce_len_trap.lu`.
//!   The file has no `acceptable` results to decide; if a re-vendored
//!   file grows one — or any flavour these rules have not named — the
//!   generator hard-errors so the omission list cannot rot silently.
//!   RFC 8439's own vectors are hand-written tests beside the
//!   generated ones (`known_answers.lu`, `rfc8439_a*.lu`), hex quoted
//!   from `vendor/vectors/rfc/rfc8439.txt`.

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

/// ChaCha20-Poly1305 chunking and smoke rules (sc17). The full set
/// rides the native lane in chunks (the interpreter column is `slow`:
/// 32 seal+open pairs measure 98s under lupin 0.1.13 against the rig's
/// 60s ceiling — F-0093's program-age curve, re-measured for this
/// module; the chunks are what flip back to `run` per-part when
/// wolf-interp#41 lands). The smoke subsets keep a differential column:
/// deterministic from the source — the first `AEAD_SMOKE_COUNT` valid
/// vectors with msg <= `AEAD_SMOKE_MAX_MSG` and aad <=
/// `AEAD_SMOKE_MAX_AAD` bytes, and the first `AEAD_SMOKE_COUNT`
/// invalid (ModifiedTag) vectors.
const AEAD_VALID_CHUNK: usize = 32;
const AEAD_INVALID_CHUNK: usize = 30;
const AEAD_SMOKE_COUNT: usize = 10;
const AEAD_SMOKE_MAX_MSG: usize = 48;
const AEAD_SMOKE_MAX_AAD: usize = 16;

/// X25519 chunking and smoke rules (sc18). Each x25519 call is a full
/// Montgomery ladder (~6.5s under lupin 0.1.13 — 255 field-heavy ladder
/// steps on a tree-walk), so the full shared set rides the native lane
/// in chunks (`slow` under lupin, the F-0093 program-age curve) while a
/// tiny deterministic smoke (5 vectors, ~33s) keeps the interpreter's
/// differential column under the 60s ceiling. The `zero`-row set
/// (small-order public keys, reject-on-zero per RFC 8446 §7.4.2) is the
/// security core and is kept whole on the native lane.
const XDH_SHARED_CHUNK: usize = 40;
const XDH_SMOKE_COUNT: usize = 5;

/// Ed25519 verify chunking and smoke rules (sc18). Verify is two scalar
/// multiplications (~7-8s under lupin 0.1.13), so the full set rides the
/// native lane in chunks (`slow` under lupin) while a tiny mixed smoke
/// (a few valid AND a few invalid) keeps the interpreter's differential
/// column under the ceiling.
const EDDSA_CHUNK: usize = 40;
const EDDSA_SMOKE_VALID: usize = 1;
const EDDSA_SMOKE_INVALID: usize = 1;

/// ECDSA-P256 chunking (sc23). A P-256 verify is TWO table-free
/// complete-formula scalar multiplications, which exceed lupin's 50M
/// evaluation-step budget AND the checked tier's step budget — so
/// every vector file is NATIVE-ONLY (lupin/wolfc `unsupported`, not
/// `slow`: the interpreter returns a step-budget verdict rather than
/// timing out). The three-lane differential is carried by the
/// hand-written field / DER / early-reject files, whose executed paths
/// never reach a scalar multiplication. 64 cases per part keeps each
/// native program's runtime moderate.
const ECDSA_CHUNK: usize = 64;

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

    // ---- sc17: Wycheproof ChaCha20-Poly1305 (the AEAD rung). ----
    {
        let out_dir = repo.join("tests/x/crypto/chacha20");
        let wp = vec_dir.join("wycheproof/chacha20_poly1305_test.json");
        let corpus = parse_wycheproof_aead(&read(&wp)?, &show(&wp))?;
        let vparts: Vec<&[AeadCase]> = corpus.valid.chunks(AEAD_VALID_CHUNK).collect();
        for (i, part) in vparts.iter().enumerate() {
            let out = out_dir.join(format!("wycheproof_valid_p{}.lu", i + 1));
            let text = fmt(
                &show(&out),
                &emit_wycheproof_aead(
                    part,
                    true,
                    i + 1,
                    vparts.len(),
                    corpus.valid.len(),
                    corpus.nonce_omitted,
                ),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
        let iparts: Vec<&[AeadCase]> = corpus.invalid.chunks(AEAD_INVALID_CHUNK).collect();
        for (i, part) in iparts.iter().enumerate() {
            let out = out_dir.join(format!("wycheproof_invalid_p{}.lu", i + 1));
            let text = fmt(
                &show(&out),
                &emit_wycheproof_aead(
                    part,
                    false,
                    i + 1,
                    iparts.len(),
                    corpus.invalid.len(),
                    corpus.nonce_omitted,
                ),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
        // The differential smoke subsets (see the constants above).
        let vsmoke: Vec<AeadCase> = corpus
            .valid
            .iter()
            .filter(|c| {
                c.msg.len() / 2 <= AEAD_SMOKE_MAX_MSG && c.aad.len() / 2 <= AEAD_SMOKE_MAX_AAD
            })
            .take(AEAD_SMOKE_COUNT)
            .cloned()
            .collect();
        let out = out_dir.join("wycheproof_valid_smoke.lu");
        let text = fmt(
            &show(&out),
            &emit_wycheproof_aead(
                &vsmoke,
                true,
                0,
                0,
                corpus.valid.len(),
                corpus.nonce_omitted,
            ),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
        let ismoke: Vec<AeadCase> = corpus
            .invalid
            .iter()
            .take(AEAD_SMOKE_COUNT)
            .cloned()
            .collect();
        let out = out_dir.join("wycheproof_invalid_smoke.lu");
        let text = fmt(
            &show(&out),
            &emit_wycheproof_aead(
                &ismoke,
                false,
                0,
                0,
                corpus.invalid.len(),
                corpus.nonce_omitted,
            ),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
    }

    // ---- sc18: Wycheproof X25519 (the curve rung). ----
    {
        let out_dir = repo.join("tests/x/crypto/curve25519");
        let wp = vec_dir.join("wycheproof/x25519_test.json");
        let corpus = parse_wycheproof_xdh(&read(&wp)?, &show(&wp))?;
        let sparts: Vec<&[XdhCase]> = corpus.shared.chunks(XDH_SHARED_CHUNK).collect();
        for (i, part) in sparts.iter().enumerate() {
            let out = out_dir.join(format!("wycheproof_x25519_shared_p{}.lu", i + 1));
            let text = fmt(
                &show(&out),
                &emit_wycheproof_xdh(part, true, i + 1, sparts.len(), corpus.shared.len()),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
        let out = out_dir.join("wycheproof_x25519_zero.lu");
        let text = fmt(
            &show(&out),
            &emit_wycheproof_xdh(&corpus.zero, false, 1, 1, corpus.zero.len()),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
        // Differential smoke subsets (deterministic: the first N of each).
        let ssmoke: Vec<XdhCase> = corpus
            .shared
            .iter()
            .take(XDH_SMOKE_COUNT)
            .cloned()
            .collect();
        let out = out_dir.join("wycheproof_x25519_shared_smoke.lu");
        let text = fmt(
            &show(&out),
            &emit_wycheproof_xdh(&ssmoke, true, 0, 0, corpus.shared.len()),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
        let zsmoke: Vec<XdhCase> = corpus.zero.iter().take(XDH_SMOKE_COUNT).cloned().collect();
        let out = out_dir.join("wycheproof_x25519_zero_smoke.lu");
        let text = fmt(
            &show(&out),
            &emit_wycheproof_xdh(&zsmoke, false, 0, 0, corpus.zero.len()),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
    }

    // ---- sc18: Wycheproof Ed25519 (the signature rung). ----
    {
        let out_dir = repo.join("tests/x/crypto/curve25519");
        let wp = vec_dir.join("wycheproof/ed25519_test.json");
        let cases = parse_wycheproof_eddsa(&read(&wp)?, &show(&wp))?;
        let parts: Vec<&[EddsaCase]> = cases.chunks(EDDSA_CHUNK).collect();
        for (i, part) in parts.iter().enumerate() {
            let out = out_dir.join(format!("wycheproof_ed25519_p{}.lu", i + 1));
            let text = fmt(
                &show(&out),
                &emit_wycheproof_eddsa(part, i + 1, parts.len(), cases.len()),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
        let mut smoke: Vec<EddsaCase> = cases
            .iter()
            .filter(|c| c.valid)
            .take(EDDSA_SMOKE_VALID)
            .cloned()
            .collect();
        smoke.extend(
            cases
                .iter()
                .filter(|c| !c.valid)
                .take(EDDSA_SMOKE_INVALID)
                .cloned(),
        );
        let out = out_dir.join("wycheproof_ed25519_smoke.lu");
        let text = fmt(
            &show(&out),
            &emit_wycheproof_eddsa(&smoke, 0, 0, cases.len()),
        )?;
        place(&out, &text, check_only, &mut drift, &mut written)?;
    }

    // ---- sc23: ECDSA-P256 — CAVP SigVer/SigGen + Wycheproof. ----
    {
        let out_dir = repo.join("tests/x/crypto/p256");
        let sv = vec_dir.join("cavp/SigVer.rsp");
        let ver = parse_cavp_sigver(&read(&sv)?, &show(&sv))?;
        let out = out_dir.join("cavp_sigver_p256.lu");
        let text = fmt(&show(&out), &emit_cavp_sigver(&ver))?;
        place(&out, &text, check_only, &mut drift, &mut written)?;

        let sg = vec_dir.join("cavp/SigGen.txt");
        let gen = parse_cavp_siggen(&read(&sg)?, &show(&sg))?;
        let out = out_dir.join("cavp_siggen_p256.lu");
        let text = fmt(&show(&out), &emit_cavp_siggen(&gen))?;
        place(&out, &text, check_only, &mut drift, &mut written)?;

        let wp = vec_dir.join("wycheproof/ecdsa_secp256r1_sha256_test.json");
        let cases = parse_wycheproof_ecdsa(&read(&wp)?, &show(&wp))?;
        let parts: Vec<&[WpEcdsaCase]> = cases.chunks(ECDSA_CHUNK).collect();
        for (i, part) in parts.iter().enumerate() {
            let out = out_dir.join(format!("wycheproof_p256_p{}.lu", i + 1));
            let text = fmt(
                &show(&out),
                &emit_wycheproof_ecdsa(part, i + 1, parts.len(), cases.len()),
            )?;
            place(&out, &text, check_only, &mut drift, &mut written)?;
        }
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

#[derive(Clone)]
struct AeadCase {
    tc_id: u64,
    key: String,
    iv: String,
    aad: String,
    msg: String,
    ct: String,
    tag: String,
}

struct AeadCorpus {
    valid: Vec<AeadCase>,
    invalid: Vec<AeadCase>,
    /// `InvalidNonceSize` vectors omitted by name — the API answers a
    /// wrong-size nonce with a documented trap (`nonce_len_trap.lu`).
    nonce_omitted: usize,
}

/// The ChaCha20-Poly1305 rules (sc17): every `valid` case runs, every
/// `invalid` case must be one of the two flavours this module has
/// DECIDED — `ModifiedTag` at the standard 96-bit nonce (asserted: open
/// answers the `tag` row) or `InvalidNonceSize` off it (omitted by
/// name: the trap file holds it). Anything else — an `acceptable`
/// result, an unknown flag, a flag at the wrong nonce size — is a hard
/// error, so a re-vendored file cannot silently shrink or blur the
/// coverage.
fn parse_wycheproof_aead(text: &str, what: &str) -> Result<AeadCorpus, String> {
    let v: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{what}: bad json: {e}"))?;
    let mut corpus = AeadCorpus {
        valid: Vec::new(),
        invalid: Vec::new(),
        nonce_omitted: 0,
    };
    let groups = v["testGroups"]
        .as_array()
        .ok_or(format!("{what}: no testGroups"))?;
    for g in groups {
        let iv_size = g["ivSize"].as_u64().ok_or(format!("{what}: ivSize"))?;
        for t in g["tests"].as_array().ok_or(format!("{what}: no tests"))? {
            let tc = t["tcId"].as_u64().ok_or(format!("{what}: tcId"))?;
            let result = t["result"].as_str().unwrap_or("");
            let flags: Vec<&str> = t["flags"]
                .as_array()
                .map(|a| a.iter().filter_map(|f| f.as_str()).collect())
                .unwrap_or_default();
            let case = AeadCase {
                tc_id: tc,
                key: t["key"].as_str().unwrap_or_default().to_string(),
                iv: t["iv"].as_str().unwrap_or_default().to_string(),
                aad: t["aad"].as_str().unwrap_or_default().to_string(),
                msg: t["msg"].as_str().unwrap_or_default().to_string(),
                ct: t["ct"].as_str().unwrap_or_default().to_string(),
                tag: t["tag"].as_str().unwrap_or_default().to_string(),
            };
            match result {
                "valid" => {
                    if iv_size != 96 {
                        return Err(format!(
                            "{what}: tcId {tc} is `valid` at ivSize {iv_size} — a shape \
                             this generator has not decided; extend the rules"
                        ));
                    }
                    corpus.valid.push(case);
                }
                "invalid" if flags == ["InvalidNonceSize"] => {
                    if iv_size == 96 {
                        return Err(format!(
                            "{what}: tcId {tc} is InvalidNonceSize AT 96 bits — \
                             contradictory; refusing to guess"
                        ));
                    }
                    corpus.nonce_omitted += 1;
                }
                "invalid" if flags == ["ModifiedTag"] => {
                    if iv_size != 96 {
                        return Err(format!(
                            "{what}: tcId {tc} is ModifiedTag at ivSize {iv_size} — \
                             a shape this generator has not decided; extend the rules"
                        ));
                    }
                    corpus.invalid.push(case);
                }
                other => {
                    return Err(format!(
                        "{what}: tcId {tc} is `{other}` with flags {flags:?} — an \
                         expectation this generator has not decided (the vendored \
                         file has no `acceptable` cases; a new flavour needs a \
                         ruling here, not a guess); extend the rules"
                    ));
                }
            }
        }
    }
    if corpus.valid.is_empty() || corpus.invalid.is_empty() {
        return Err(format!("{what}: suspiciously empty aead corpus"));
    }
    Ok(corpus)
}

#[derive(Clone)]
struct XdhCase {
    tc_id: u64,
    private: String,
    public: String,
    shared: String,
}

struct XdhCorpus {
    /// `valid`/`acceptable` cases with a NONZERO shared secret — the
    /// shared secret is asserted byte-for-byte (a wrong-flag twist or
    /// non-canonical public key is still a conformant computation under
    /// RFC 7748, which masks the high bit and requires no rejection).
    shared: Vec<XdhCase>,
    /// `acceptable` cases flagged `ZeroSharedSecret` — the public key is
    /// small-order, so `x25519` must raise the `zero` row (RFC 8446
    /// §7.4.2's abort). The vendored file's `shared` for these is all
    /// zero, verified here.
    zero: Vec<XdhCase>,
}

/// The X25519 rules (sc18): every vector is `valid` or `acceptable`
/// (the file has no `invalid`), and the split is by the
/// `ZeroSharedSecret` flag — which the parser cross-checks against the
/// actual shared value (a flagged case whose `shared` is not all-zero,
/// or an unflagged case whose `shared` IS all-zero, is a hard error, so
/// the security-critical partition cannot rot). Any other result is a
/// hard error.
fn parse_wycheproof_xdh(text: &str, what: &str) -> Result<XdhCorpus, String> {
    let v: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{what}: bad json: {e}"))?;
    let mut corpus = XdhCorpus {
        shared: Vec::new(),
        zero: Vec::new(),
    };
    let groups = v["testGroups"]
        .as_array()
        .ok_or(format!("{what}: no testGroups"))?;
    for gp in groups {
        for t in gp["tests"].as_array().ok_or(format!("{what}: no tests"))? {
            let tc = t["tcId"].as_u64().ok_or(format!("{what}: tcId"))?;
            let result = t["result"].as_str().unwrap_or("");
            if result != "valid" && result != "acceptable" {
                return Err(format!(
                    "{what}: tcId {tc} is `{result}` — the X25519 file is all \
                     valid/acceptable; a new result needs a ruling here"
                ));
            }
            let flags: Vec<&str> = t["flags"]
                .as_array()
                .map(|a| a.iter().filter_map(|f| f.as_str()).collect())
                .unwrap_or_default();
            let shared = t["shared"].as_str().unwrap_or_default().to_string();
            let is_zero_flag = flags.contains(&"ZeroSharedSecret");
            let is_zero_val = !shared.is_empty() && shared.chars().all(|c| c == '0');
            if is_zero_flag != is_zero_val {
                return Err(format!(
                    "{what}: tcId {tc} — the ZeroSharedSecret flag ({is_zero_flag}) \
                     and the all-zero shared value ({is_zero_val}) disagree; refusing \
                     to guess the partition"
                ));
            }
            let case = XdhCase {
                tc_id: tc,
                private: t["private"].as_str().unwrap_or_default().to_string(),
                public: t["public"].as_str().unwrap_or_default().to_string(),
                shared,
            };
            if is_zero_flag {
                corpus.zero.push(case);
            } else {
                corpus.shared.push(case);
            }
        }
    }
    if corpus.shared.is_empty() || corpus.zero.is_empty() {
        return Err(format!("{what}: suspiciously empty x25519 corpus"));
    }
    Ok(corpus)
}

#[derive(Clone)]
struct EddsaCase {
    tc_id: u64,
    public: String,
    msg: String,
    sig: String,
    valid: bool,
}

/// The Ed25519 rules (sc18): every vector is `valid` or `invalid` (the
/// file has no `acceptable`), and the public key is the GROUP's
/// `publicKey.pk`. `valid` -> verify must answer true; `invalid` ->
/// verify must answer false (a wrong length, non-canonical encoding,
/// S >= L malleability, or a bad point all decide to false). Any other
/// result is a hard error so a re-vendored file cannot blur coverage.
fn parse_wycheproof_eddsa(text: &str, what: &str) -> Result<Vec<EddsaCase>, String> {
    let v: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{what}: bad json: {e}"))?;
    let mut out = Vec::new();
    let groups = v["testGroups"]
        .as_array()
        .ok_or(format!("{what}: no testGroups"))?;
    for gp in groups {
        let pk = gp["publicKey"]["pk"]
            .as_str()
            .ok_or(format!("{what}: group has no publicKey.pk"))?
            .to_string();
        for t in gp["tests"].as_array().ok_or(format!("{what}: no tests"))? {
            let tc = t["tcId"].as_u64().ok_or(format!("{what}: tcId"))?;
            let result = t["result"].as_str().unwrap_or("");
            let valid = match result {
                "valid" => true,
                "invalid" => false,
                other => {
                    return Err(format!(
                        "{what}: tcId {tc} is `{other}` — the Ed25519 file is \
                         valid/invalid only; a new result needs a ruling here"
                    ));
                }
            };
            out.push(EddsaCase {
                tc_id: tc,
                public: pk.clone(),
                msg: t["msg"].as_str().unwrap_or_default().to_string(),
                sig: t["sig"].as_str().unwrap_or_default().to_string(),
                valid,
            });
        }
    }
    if out.is_empty() {
        return Err(format!("{what}: no eddsa vectors parsed"));
    }
    Ok(out)
}

// ------------------------------------------------------- sc23: ECDSA-P256

/// One CAVP SigVer P-256 case: a public key (Qx || Qy), a message, an
/// (r, s) signature and the expected pass/fail verdict.
struct EcdsaVerCase {
    qx: String,
    qy: String,
    msg: String,
    r: String,
    s: String,
    valid: bool,
}

/// Parse the `[P-256,SHA-256]` section of CAVP `SigVer.rsp` — the block
/// runs from that header to the next `[` bracket. Each record is
/// Msg/Qx/Qy/R/S/Result, Result `P` (pass) or `F` (fail). Any other
/// hash/curve section is ignored; a malformed record is a hard error.
fn parse_cavp_sigver(text: &str, what: &str) -> Result<Vec<EcdsaVerCase>, String> {
    let sec = section(text, "[P-256,SHA-256]", what)?;
    let mut out = Vec::new();
    let mut cur: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    for line in sec.lines() {
        let line = line.trim();
        if let Some((k, v)) = line.split_once(" = ") {
            cur.insert(k.trim(), v.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("Result = ") {
            let _ = rest;
        }
        if line.starts_with("Result = ") {
            let res = line.trim_start_matches("Result = ");
            let valid = res.starts_with('P');
            out.push(EcdsaVerCase {
                qx: field(&cur, "Qx", what)?,
                qy: field(&cur, "Qy", what)?,
                msg: field(&cur, "Msg", what)?,
                r: field(&cur, "R", what)?,
                s: field(&cur, "S", what)?,
                valid,
            });
            cur.clear();
        }
    }
    if out.is_empty() {
        return Err(format!("{what}: no P-256 SigVer records parsed"));
    }
    Ok(out)
}

/// One CAVP SigGen P-256 case: the key (Qx || Qy), the message, and the
/// signature (r, s) the generator produced with its own random nonce.
struct EcdsaGenCase {
    qx: String,
    qy: String,
    msg: String,
    r: String,
    s: String,
}

/// Parse the `[P-256,SHA-256]` section of CAVP `SigGen.txt`. Each record
/// is Msg/d/Qx/Qy/k/R/S; std's sign is RFC 6979 (a different, equally
/// valid nonce), so these are asserted through VERIFY (Q against
/// Msg/R/S — the signature the file publishes must verify), not
/// regenerated. A malformed record is a hard error.
fn parse_cavp_siggen(text: &str, what: &str) -> Result<Vec<EcdsaGenCase>, String> {
    let sec = section(text, "[P-256,SHA-256]", what)?;
    let mut out = Vec::new();
    let mut cur: std::collections::HashMap<&str, String> = std::collections::HashMap::new();
    for line in sec.lines() {
        let line = line.trim();
        if let Some((k, v)) = line.split_once(" = ") {
            cur.insert(k.trim(), v.trim().to_string());
        }
        if line.starts_with("S = ") {
            out.push(EcdsaGenCase {
                qx: field(&cur, "Qx", what)?,
                qy: field(&cur, "Qy", what)?,
                msg: field(&cur, "Msg", what)?,
                r: field(&cur, "R", what)?,
                s: field(&cur, "S", what)?,
            });
            cur.clear();
        }
    }
    if out.is_empty() {
        return Err(format!("{what}: no P-256 SigGen records parsed"));
    }
    Ok(out)
}

/// One Wycheproof ECDSA case: the group's uncompressed public key, the
/// message, the DER signature and the verdict.
struct WpEcdsaCase {
    tc_id: u64,
    pubkey: String,
    msg: String,
    sig: String,
    valid: bool,
}

/// The Wycheproof ECDSA rules (sc23): each group carries an uncompressed
/// public key (`publicKey.uncompressed`); every test is `valid` or
/// `invalid` (this file has no `acceptable`), and `verify_der` must
/// answer true / false respectively — the DER manipulations, r/s edge
/// values, invalid-curve points and malleability cases all decided by
/// the strict-DER + FIPS 186-5 verify. Any other result is a hard error.
fn parse_wycheproof_ecdsa(text: &str, what: &str) -> Result<Vec<WpEcdsaCase>, String> {
    let v: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("{what}: bad json: {e}"))?;
    let mut out = Vec::new();
    let groups = v["testGroups"]
        .as_array()
        .ok_or(format!("{what}: no testGroups"))?;
    for gp in groups {
        let pk = gp["publicKey"]["uncompressed"]
            .as_str()
            .ok_or(format!("{what}: group has no publicKey.uncompressed"))?
            .to_string();
        for t in gp["tests"].as_array().ok_or(format!("{what}: no tests"))? {
            let tc = t["tcId"].as_u64().ok_or(format!("{what}: tcId"))?;
            let result = t["result"].as_str().unwrap_or("");
            let valid = match result {
                "valid" => true,
                "invalid" => false,
                other => {
                    return Err(format!(
                        "{what}: tcId {tc} is `{other}` — the ECDSA file is \
                         valid/invalid only; a new result needs a ruling here"
                    ));
                }
            };
            out.push(WpEcdsaCase {
                tc_id: tc,
                pubkey: pk.clone(),
                msg: t["msg"].as_str().unwrap_or_default().to_string(),
                sig: t["sig"].as_str().unwrap_or_default().to_string(),
                valid,
            });
        }
    }
    if out.is_empty() {
        return Err(format!("{what}: no ECDSA vectors parsed"));
    }
    Ok(out)
}

/// The substring from the first line equal to `header` up to the next
/// line beginning with `[` (a CAVP `.rsp`/`.txt` section).
fn section<'a>(text: &'a str, header: &str, what: &str) -> Result<&'a str, String> {
    let start = text
        .find(header)
        .ok_or(format!("{what}: no `{header}` section"))?
        + header.len();
    let rest = &text[start..];
    let end = rest
        .match_indices('[')
        .map(|(i, _)| i)
        .find(|&i| rest[..i].ends_with('\n') || rest[..i].trim_end().ends_with('\n'))
        .unwrap_or(rest.len());
    Ok(&rest[..end])
}

fn field(
    m: &std::collections::HashMap<&str, String>,
    k: &str,
    what: &str,
) -> Result<String, String> {
    m.get(k)
        .cloned()
        .ok_or(format!("{what}: record missing `{k}`"))
}

/// Pad a hex string to 64 chars (32 bytes) — CAVP prints r/s without
/// leading zeros; the raw signature is fixed-width r || s.
fn pad64(h: &str) -> String {
    format!("{:0>64}", h)
}

const ECDSA_HDR: &str = "//! check: run(exit=0)\n//! phase: run\n\
    //! conforms: std.x.crypto.p256, arith.checked\n//!\n\
    //! GENERATED — `cargo xtask gen-vectors`, from the vendored source\n\
    //! named below (provenance: `vendor/vectors/README.md`). Do not edit\n\
    //! by hand; ci's `gen-vectors --check` holds this file byte-identical\n\
    //! to its source. NATIVE-ONLY: a P-256 verify is two table-free\n\
    //! complete-formula scalar multiplications, past lupin's 50M step\n\
    //! budget and the checked tier's step budget (the ledger records\n\
    //! both `unsupported`); the three-lane differential is the\n\
    //! hand-written field / DER / early-reject files.\n//!\n";

fn emit_cavp_sigver(cases: &[EcdsaVerCase]) -> String {
    let mut s = String::new();
    s.push_str(ECDSA_HDR);
    let np = cases.iter().filter(|c| c.valid).count();
    let nf = cases.len() - np;
    let _ = write!(
        s,
        "//! NIST CAVP FIPS 186-4 ECDSA SigVer, [P-256,SHA-256] — all {} cases\n\
         //! ({np} pass + {nf} fail). `verify` must answer true for every P and\n\
         //! FALSE for every F (the R/S/Q/Msg tamper classes the file names).\n\
         //! Source: `vendor/vectors/cavp/SigVer.rsp`.\n",
        cases.len()
    );
    s.push_str("\nuse std.hex\nuse std.x.crypto.p256\n\n");
    s.push_str(
        "fn vrf(qx: str, qy: str, msg: str, r: str, s: str, want: bool) -> bool {\n\
         \x20   let pk = hex.decode(\"04{qx}{qy}\") else List[int]()\n\
         \x20   let sig = hex.decode(\"{r}{s}\") else List[int]()\n\
         \x20   p256.verify(pk, hex.decode(msg) else List[int](), sig) == want\n}\n\n\
         fn main() -> int {\n",
    );
    for (i, c) in cases.iter().enumerate() {
        let _ = writeln!(
            s,
            "    assert(vrf(\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", {}), \"sigver {i}\")",
            c.qx,
            c.qy,
            c.msg,
            pad64(&c.r),
            pad64(&c.s),
            c.valid
        );
    }
    s.push_str("    0\n}\n");
    s
}

fn emit_cavp_siggen(cases: &[EcdsaGenCase]) -> String {
    let mut s = String::new();
    s.push_str(ECDSA_HDR);
    let _ = write!(
        s,
        "//! NIST CAVP FIPS 186-4 ECDSA SigGen, [P-256,SHA-256] — all {} cases,\n\
         //! asserted THROUGH VERIFY. The file's (R,S) were produced with its\n\
         //! own random nonce; std's sign is RFC 6979 deterministic (a\n\
         //! different, equally valid nonce), so the published signature is\n\
         //! checked by verifying it against the key and message (the sign\n\
         //! path is pinned byte-exact by RFC 6979 in `rfc6979_p256.lu`).\n\
         //! Source: `vendor/vectors/cavp/SigGen.txt`.\n",
        cases.len()
    );
    s.push_str("\nuse std.hex\nuse std.x.crypto.p256\n\n");
    s.push_str(
        "fn vrf(qx: str, qy: str, msg: str, r: str, s: str) -> bool {\n\
         \x20   let pk = hex.decode(\"04{qx}{qy}\") else List[int]()\n\
         \x20   let sig = hex.decode(\"{r}{s}\") else List[int]()\n\
         \x20   p256.verify(pk, hex.decode(msg) else List[int](), sig)\n}\n\n\
         fn main() -> int {\n",
    );
    for (i, c) in cases.iter().enumerate() {
        let _ = writeln!(
            s,
            "    assert(vrf(\"{}\", \"{}\", \"{}\", \"{}\", \"{}\"), \"siggen {i}\")",
            c.qx,
            c.qy,
            c.msg,
            pad64(&c.r),
            pad64(&c.s)
        );
    }
    s.push_str("    0\n}\n");
    s
}

fn emit_wycheproof_ecdsa(cases: &[WpEcdsaCase], part: usize, parts: usize, total: usize) -> String {
    let mut s = String::new();
    s.push_str(ECDSA_HDR);
    let nv = cases.iter().filter(|c| c.valid).count();
    let ni = cases.len() - nv;
    let _ = write!(
        s,
        "//! Wycheproof v1 ecdsa_secp256r1_sha256 verify, part {part} of {parts}\n\
         //! ({} of the set's {total}: {nv} valid + {ni} invalid): `verify_der`\n\
         //! must answer true for every valid vector and FALSE for every\n\
         //! invalid one — the DER manipulations, r/s = 0 / = n / > n, the\n\
         //! invalid-curve points and the malleability cases are the point.\n\
         //! Source: `vendor/vectors/wycheproof/ecdsa_secp256r1_sha256_test.json`.\n",
        cases.len()
    );
    s.push_str("\nuse std.hex\nuse std.x.crypto.p256\n\n");
    s.push_str(
        "fn vd(pk: str, msg: str, der: str, want: bool) -> bool {\n\
         \x20   let key = hex.decode(pk) else List[int]()\n\
         \x20   let m = hex.decode(msg) else List[int]()\n\
         \x20   let d = hex.decode(der) else List[int]()\n\
         \x20   p256.verify_der(key, m, d) == want\n}\n\nfn main() -> int {\n",
    );
    for c in cases {
        let _ = writeln!(
            s,
            "    assert(vd(\"{}\", \"{}\", \"{}\", {}), \"tc {}\")",
            c.pubkey, c.msg, c.sig, c.valid, c.tc_id
        );
    }
    s.push_str("    0\n}\n");
    s
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

fn emit_wycheproof_aead(
    cases: &[AeadCase],
    valid: bool,
    part: usize,
    parts: usize,
    total: usize,
    nonce_omitted: usize,
) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.chacha20, arith.wrapping\n//!\n\
         //! GENERATED — `cargo xtask gen-vectors`, from\n\
         //! `vendor/vectors/wycheproof/chacha20_poly1305_test.json` (provenance:\n\
         //! `vendor/vectors/README.md`). Do not edit by hand; ci's\n\
         //! `gen-vectors --check` holds this file byte-identical to its source.\n//!\n"
    );
    if valid && part > 0 {
        let _ = write!(
            s,
            "//! Wycheproof v1 ChaCha20-Poly1305, the VALID set, part {part} of\n\
             //! {parts} ({n} vectors of the set's {total}): each case seals to the\n\
             //! published ciphertext||tag AND opens back to the message —\n\
             //! Ktv, pseudorandom, and the edge-case Poly1305/keystream\n\
             //! constructions. The interpreter column is `slow` (the sc16\n\
             //! ledger word): 32 seal+open pairs measure 98s under lupin\n\
             //! 0.1.13 against the rig's 60s ceiling (F-0093's program-age\n\
             //! curve), the `..._smoke.lu` subset keeps the differential\n\
             //! column, and the chunking is what flips this file back to\n\
             //! `run` when wolf-interp#41 lands.\n",
            n = cases.len(),
        );
    } else if valid {
        let _ = write!(
            s,
            "//! Wycheproof v1 ChaCha20-Poly1305, the DIFFERENTIAL SMOKE SUBSET\n\
             //! of the valid set: the first {n} valid vectors with msg <= {mm}\n\
             //! and aad <= {ma} bytes (of the set's {total}) — small enough that\n\
             //! the interpreter runs the file inside the rig's ceiling, so the\n\
             //! AEAD keeps a differential column while the full set rides the\n\
             //! `..._p*.lu` parts on the native lane.\n",
            n = cases.len(),
            mm = AEAD_SMOKE_MAX_MSG,
            ma = AEAD_SMOKE_MAX_AAD,
        );
    } else if part > 0 {
        let _ = write!(
            s,
            "//! Wycheproof v1 ChaCha20-Poly1305, the INVALID set, part {part} of\n\
             //! {parts} ({n} vectors of the set's {total}, every one a\n\
             //! `ModifiedTag`: bit flips at every tag position, the flipped\n\
             //! MSBs, the truncated-to-prefix shapes): `open` must answer the\n\
             //! `tag` row — witnessed as a negative sentinel from the helper's\n\
             //! handler — and hand back NO plaintext. The {omit} source\n\
             //! `InvalidNonceSize` vectors are omitted by name: the API\n\
             //! answers a wrong-size nonce with the documented `assert` trap\n\
             //! held by `nonce_len_trap.lu`. The file has no `acceptable`\n\
             //! cases to decide; the generator hard-errors if one appears.\n\
             //! The interpreter column is `slow` (F-0093; 30 opens measure\n\
             //! 29s under lupin 0.1.13); `..._invalid_smoke.lu` keeps the\n\
             //! differential column.\n",
            n = cases.len(),
            omit = nonce_omitted,
        );
    } else {
        let _ = write!(
            s,
            "//! Wycheproof v1 ChaCha20-Poly1305, the DIFFERENTIAL SMOKE SUBSET\n\
             //! of the invalid set: the first {n} `ModifiedTag` vectors (of the\n\
             //! set's {total}) — `open` must answer the `tag` row for every one,\n\
             //! on the interpreter lane too, so the reject path stays\n\
             //! differential while the full set rides the `..._p*.lu` parts on\n\
             //! the native lane.\n",
            n = cases.len(),
        );
    }
    s.push_str("\nuse std.hex\nuse std.x.crypto.chacha20\n\n");
    if valid {
        let _ = write!(
            s,
            "fn aead_matches(key_hex: str, iv_hex: str, aad_hex: str, msg_hex: str, box_hex: str) -> bool {{\n\
             \x20   let key = hex.decode(key_hex) else List[int]()\n\
             \x20   let iv = hex.decode(iv_hex) else List[int]()\n\
             \x20   let aad = hex.decode(aad_hex) else List[int]()\n\
             \x20   let msg = hex.decode(msg_hex) else List[int]()\n\
             \x20   let boxed = chacha20.seal(key, iv, aad, msg)\n\
             \x20   if !(chacha20.to_hex(boxed) == box_hex) {{\n\
             \x20       return false\n\
             \x20   }}\n\
             \x20   let opened = chacha20.open(key, iv, aad, boxed) else List[int]()\n\
             \x20   chacha20.to_hex(opened) == msg_hex\n}}\n\nfn main() -> int {{\n"
        );
        for c in cases {
            let _ = writeln!(
                s,
                "    assert(aead_matches(\"{}\", \"{}\", \"{}\", \"{}\", \"{}{}\"), \"tc {}\")",
                c.key, c.iv, c.aad, c.msg, c.ct, c.tag, c.tc_id
            );
        }
    } else {
        let _ = write!(
            s,
            "fn open_verdict(key_hex: str, iv_hex: str, aad_hex: str, box_hex: str) -> int {{\n\
             \x20   let key = hex.decode(key_hex) else List[int]()\n\
             \x20   let iv = hex.decode(iv_hex) else List[int]()\n\
             \x20   let aad = hex.decode(aad_hex) else List[int]()\n\
             \x20   let boxed = hex.decode(box_hex) else List[int]()\n\
             \x20   let opened = chacha20.open(key, iv, aad, boxed) else |_| {{\n\
             \x20       return -1\n\
             \x20   }}\n\
             \x20   opened.len\n}}\n\nfn main() -> int {{\n"
        );
        for c in cases {
            let _ = writeln!(
                s,
                "    assert(open_verdict(\"{}\", \"{}\", \"{}\", \"{}{}\") < 0, \"tc {}\")",
                c.key, c.iv, c.aad, c.ct, c.tag, c.tc_id
            );
        }
    }
    s.push_str("    0\n}\n");
    s
}

fn emit_wycheproof_xdh(
    cases: &[XdhCase],
    shared: bool,
    part: usize,
    parts: usize,
    total: usize,
) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.curve25519, arith.checked\n//!\n\
         //! GENERATED — `cargo xtask gen-vectors`, from\n\
         //! `vendor/vectors/wycheproof/x25519_test.json` (provenance:\n\
         //! `vendor/vectors/README.md`). Do not edit by hand; ci's\n\
         //! `gen-vectors --check` holds this file byte-identical to its source.\n//!\n"
    );
    if shared && part > 0 {
        let _ = write!(
            s,
            "//! Wycheproof v1 X25519, the SHARED set, part {part} of {parts}\n\
             //! ({n} vectors of the set's {total}): every `valid` and nonzero\n\
             //! `acceptable` vector — the shared secret is asserted byte-for-\n\
             //! byte. The `acceptable` twist and non-canonical-public cases are\n\
             //! here because RFC 7748 masks the high bit and requires no twist\n\
             //! or canonicity rejection, so the ladder's answer is conformant.\n\
             //! The interpreter column is `slow` (the sc16 word): each vector\n\
             //! is a full ladder (~6.5s under lupin 0.1.13, F-0093's curve\n\
             //! makes a {n}-call program blow the ceiling), the shared_smoke\n\
             //! file keeps the differential column, and the chunk flips back\n\
             //! to `run` when a lupin release carries is20.\n",
            n = cases.len(),
        );
    } else if shared {
        let _ = write!(
            s,
            "//! Wycheproof v1 X25519, the DIFFERENTIAL SMOKE SUBSET of the\n\
             //! shared set: the first {n} vectors (of {total}) — small enough\n\
             //! that the interpreter runs the file inside the ceiling, so\n\
             //! X25519 keeps a three-lane column while the full set rides the\n\
             //! `..._p*.lu` parts on the native lane.\n",
            n = cases.len(),
        );
    } else if part > 0 {
        let _ = write!(
            s,
            "//! Wycheproof v1 X25519, the ZERO-SHARED-SECRET set: all {n}\n\
             //! `acceptable` vectors flagged `ZeroSharedSecret` — small-order\n\
             //! public keys for which `x25519` must raise the `zero` row\n\
             //! (RFC 8446 §7.4.2 requires a TLS 1.3 endpoint to abort rather\n\
             //! than use a predictable secret). This is the module's security\n\
             //! core, kept WHOLE, and the reject is witnessed as a negative\n\
             //! sentinel from the helper's `zero` handler. The interpreter\n\
             //! column is `slow` (F-0093); `..._zero_smoke.lu` keeps it\n\
             //! differential.\n",
            n = cases.len(),
        );
    } else {
        let _ = write!(
            s,
            "//! Wycheproof v1 X25519, the DIFFERENTIAL SMOKE SUBSET of the\n\
             //! zero set: the first {n} `ZeroSharedSecret` vectors (of\n\
             //! {total}) — `x25519` raises the `zero` row for every one, on\n\
             //! the interpreter lane too, so the reject path stays three-lane\n\
             //! while the full set rides `..._zero.lu` on the native lane.\n",
            n = cases.len(),
        );
    }
    s.push_str("\nuse std.hex\nuse std.x.crypto.curve25519\n\n");
    if shared {
        let _ = write!(
            s,
            "fn shared_matches(priv_hex: str, pub_hex: str, want: str) -> bool {{\n\
             \x20   let sk = hex.decode(priv_hex) else List[int]()\n\
             \x20   let pk = hex.decode(pub_hex) else List[int]()\n\
             \x20   let out = curve25519.x25519(sk, pk) else List[int]()\n\
             \x20   curve25519.to_hex(out) == want\n}}\n\nfn main() -> int {{\n"
        );
        for c in cases {
            let _ = writeln!(
                s,
                "    assert(shared_matches(\"{}\", \"{}\", \"{}\"), \"tc {}\")",
                c.private, c.public, c.shared, c.tc_id
            );
        }
    } else {
        let _ = write!(
            s,
            "fn zero_verdict(priv_hex: str, pub_hex: str) -> int {{\n\
             \x20   let sk = hex.decode(priv_hex) else List[int]()\n\
             \x20   let pk = hex.decode(pub_hex) else List[int]()\n\
             \x20   let out = curve25519.x25519(sk, pk) else |_| {{\n\
             \x20       return -1\n\
             \x20   }}\n\
             \x20   out.len\n}}\n\nfn main() -> int {{\n"
        );
        for c in cases {
            let _ = writeln!(
                s,
                "    assert(zero_verdict(\"{}\", \"{}\") < 0, \"tc {}\")",
                c.private, c.public, c.tc_id
            );
        }
    }
    s.push_str("    0\n}\n");
    s
}

fn emit_wycheproof_eddsa(cases: &[EddsaCase], part: usize, parts: usize, total: usize) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "//! check: run(exit=0)\n//! phase: run\n//! conforms: std.x.crypto.curve25519, arith.checked\n//!\n\
         //! GENERATED — `cargo xtask gen-vectors`, from\n\
         //! `vendor/vectors/wycheproof/ed25519_test.json` (provenance:\n\
         //! `vendor/vectors/README.md`). Do not edit by hand; ci's\n\
         //! `gen-vectors --check` holds this file byte-identical to its source.\n//!\n"
    );
    let nvalid = cases.iter().filter(|c| c.valid).count();
    let ninvalid = cases.len() - nvalid;
    if part > 0 {
        let _ = write!(
            s,
            "//! Wycheproof v1 Ed25519 verify, part {part} of {parts} ({n} of the\n\
             //! set's {total}: {nv} valid + {ni} invalid): `verify` must answer\n\
             //! true for every valid vector and FALSE for every invalid one —\n\
             //! the invalid set is the point (wrong lengths, non-canonical\n\
             //! encodings, S >= L malleability, swapped/garbage signatures), and\n\
             //! the single cofactorless verify with strict S < L decides them\n\
             //! all. The interpreter column is `slow` (verify is two scalar\n\
             //! mults, ~7-8s under lupin 0.1.13, F-0093); the smoke file keeps\n\
             //! the differential column.\n",
            n = cases.len(),
            nv = nvalid,
            ni = ninvalid,
        );
    } else {
        let _ = write!(
            s,
            "//! Wycheproof v1 Ed25519 verify, the DIFFERENTIAL SMOKE SUBSET:\n\
             //! {nv} valid and {ni} invalid vectors (of the set's {total}) — small\n\
             //! enough that the interpreter runs the file inside the ceiling, so\n\
             //! sign/verify keeps a three-lane column while the full set rides\n\
             //! the `..._p*.lu` parts on the native lane.\n",
            nv = nvalid,
            ni = ninvalid,
        );
    }
    s.push_str("\nuse std.hex\nuse std.x.crypto.curve25519\n\n");
    let _ = write!(
        s,
        "fn verify_is(pub_hex: str, msg_hex: str, sig_hex: str, want: bool) -> bool {{\n\
         \x20   let pk = hex.decode(pub_hex) else List[int]()\n\
         \x20   let msg = hex.decode(msg_hex) else List[int]()\n\
         \x20   let sig = hex.decode(sig_hex) else List[int]()\n\
         \x20   curve25519.verify(pk, msg, sig) == want\n}}\n\nfn main() -> int {{\n"
    );
    for c in cases {
        let _ = writeln!(
            s,
            "    assert(verify_is(\"{}\", \"{}\", \"{}\", {}), \"tc {}\")",
            c.public, c.msg, c.sig, c.valid, c.tc_id
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

    fn aead_json(entries: &str) -> String {
        format!(r#"{{"testGroups":[{entries}]}}"#)
    }

    #[test]
    fn aead_sorts_valid_modified_tag_and_nonce_omission() {
        let text = aead_json(
            r#"{"ivSize":96,"tests":[
                {"tcId":1,"result":"valid","flags":["Ktv"],"key":"aa","iv":"bb","aad":"","msg":"cc","ct":"dd","tag":"ee"},
                {"tcId":2,"result":"invalid","flags":["ModifiedTag"],"key":"aa","iv":"bb","aad":"","msg":"","ct":"dd","tag":"ff"}
            ]},
            {"ivSize":64,"tests":[
                {"tcId":3,"result":"invalid","flags":["InvalidNonceSize"],"key":"aa","iv":"bb","aad":"","msg":"","ct":"","tag":"ee"}
            ]}"#,
        );
        let c = parse_wycheproof_aead(&text, "t").unwrap();
        assert_eq!(c.valid.len(), 1);
        assert_eq!(c.valid[0].tc_id, 1);
        assert_eq!(c.invalid.len(), 1);
        assert_eq!(c.invalid[0].tc_id, 2);
        assert_eq!(c.nonce_omitted, 1);
    }

    #[test]
    fn aead_refuses_undecided_flavours() {
        // An `acceptable` result is a hard error — "decided and
        // documented" means a ruling in the parser, never a guess.
        let acceptable = aead_json(
            r#"{"ivSize":96,"tests":[
                {"tcId":9,"result":"acceptable","flags":["SomeFlag"],"key":"aa","iv":"bb","aad":"","msg":"","ct":"","tag":"ee"}
            ]}"#,
        );
        assert!(parse_wycheproof_aead(&acceptable, "t").is_err());
        // An unknown invalid flag is a hard error.
        let unknown = aead_json(
            r#"{"ivSize":96,"tests":[
                {"tcId":10,"result":"invalid","flags":["ZeroLengthIv"],"key":"aa","iv":"","aad":"","msg":"","ct":"","tag":"ee"}
            ]}"#,
        );
        assert!(parse_wycheproof_aead(&unknown, "t").is_err());
        // A valid case off the 96-bit nonce is a hard error too: the
        // emitters would feed it into a trap.
        let off_nonce = aead_json(
            r#"{"ivSize":64,"tests":[
                {"tcId":11,"result":"valid","flags":[],"key":"aa","iv":"bb","aad":"","msg":"","ct":"","tag":"ee"}
            ]}"#,
        );
        assert!(parse_wycheproof_aead(&off_nonce, "t").is_err());
    }

    fn xdh_json(entries: &str) -> String {
        format!(r#"{{"testGroups":[{{"tests":[{entries}]}}]}}"#)
    }

    #[test]
    fn xdh_partitions_on_zero_shared_and_cross_checks_the_flag() {
        let text = xdh_json(
            r#"
            {"tcId":1,"result":"valid","flags":["Normal"],"private":"aa","public":"bb","shared":"1234"},
            {"tcId":2,"result":"acceptable","flags":["Twist"],"private":"cc","public":"dd","shared":"5678"},
            {"tcId":3,"result":"acceptable","flags":["ZeroSharedSecret"],"private":"ee","public":"ff","shared":"0000000000000000000000000000000000000000000000000000000000000000"}
            "#,
        );
        let c = parse_wycheproof_xdh(&text, "t").unwrap();
        assert_eq!(c.shared.len(), 2);
        assert_eq!(c.zero.len(), 1);
        assert_eq!(c.zero[0].tc_id, 3);
    }

    #[test]
    fn eddsa_partitions_valid_invalid_and_refuses_acceptable() {
        let text = r#"{"testGroups":[{"publicKey":{"pk":"aabb"},"tests":[
            {"tcId":1,"result":"valid","msg":"","sig":"11"},
            {"tcId":2,"result":"invalid","msg":"72","sig":"22"}
        ]}]}"#;
        let c = parse_wycheproof_eddsa(text, "t").unwrap();
        assert_eq!(c.len(), 2);
        assert!(c[0].valid && c[0].public == "aabb");
        assert!(!c[1].valid);
        let bad = r#"{"testGroups":[{"publicKey":{"pk":"aa"},"tests":[
            {"tcId":3,"result":"acceptable","msg":"","sig":"11"}
        ]}]}"#;
        assert!(parse_wycheproof_eddsa(bad, "t").is_err());
    }

    #[test]
    fn xdh_refuses_flag_value_disagreement_and_unknown_result() {
        // A ZeroSharedSecret flag whose shared value is NOT all-zero is a
        // hard error — the security partition cannot rot.
        let lying_flag = xdh_json(
            r#"{"tcId":9,"result":"acceptable","flags":["ZeroSharedSecret"],"private":"aa","public":"bb","shared":"1234"}"#,
        );
        assert!(parse_wycheproof_xdh(&lying_flag, "t").is_err());
        // An all-zero shared value WITHOUT the flag is equally a hard error.
        let silent_zero = xdh_json(
            r#"{"tcId":10,"result":"valid","flags":["Normal"],"private":"aa","public":"bb","shared":"0000000000000000000000000000000000000000000000000000000000000000"}"#,
        );
        assert!(parse_wycheproof_xdh(&silent_zero, "t").is_err());
        // Any result other than valid/acceptable is undecided -> hard error.
        let unknown = xdh_json(
            r#"{"tcId":11,"result":"invalid","flags":["X"],"private":"aa","public":"bb","shared":"12"}"#,
        );
        assert!(parse_wycheproof_xdh(&unknown, "t").is_err());
    }
}
