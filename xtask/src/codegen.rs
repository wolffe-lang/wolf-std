//! `cargo xtask codegen` (sc84, wolf-std#151): the objects the compiler
//! generates for the secret-handling modules contain no integer division,
//! and their release builds still give the published answers.
//!
//! WHY A GATE ON GENERATED CODE. A division's latency can depend on its
//! operands, and RFC 7748 §5.1 warns about exactly that. Source that never
//! divides a secret is not enough to know: at wolf 0.2.39 the x86-64
//! release ladder divided the clamped scalar byte (`div %sil`, 255 times
//! per scalar multiplication) because a branch chain hid that the divisor
//! was a power of two, and the audit before it counted `idiv` and missed
//! the unsigned `div`. A reading by eye is how that was missed, so this
//! step reads the objects itself, every time.
//!
//! WHAT IT DOES, on the host target:
//!
//! 1. A probe that divides by a runtime value is built in both tiers and
//!    scanned. If the scan sees no division there, the step is BLIND and
//!    red: a disassembler or a parser that cannot see the instruction must
//!    never pass the modules.
//! 2. [`ENTRY`] calls every public function of `std.x.crypto.curve25519`,
//!    `std.x.crypto.chacha20`, `std.x.crypto.sha2` and `std.bech32` on
//!    inputs from `argv`, so nothing folds away. Its default and release
//!    objects are built (`--emit=obj`) and every function in them is
//!    scanned: any `div`/`idiv` (x86-64, any width) or `sdiv`/`udiv`
//!    (arm64) is a `FAIL <function> <tier> +<offset> <instruction>` line
//!    and the step is red. There is no allowlist. Each tier must also
//!    show functions of all four modules, so an empty or misread object
//!    cannot pass.
//! 3. [`RELEASE_ROWS`], the published-vector rows these modules carry, are
//!    built and run in the release tier, which no ledger lane runs; each
//!    must exit 0.
//!
//! WHAT IT DOES NOT COVER, said where the gate is: branches (the census in
//! `docs/sc84-no-division-on-secrets.md` reads them at source and in the
//! objects, once); targets other than the host's (a Linux runner says
//! nothing about arm64, nor macOS about x86-64); Windows, where the
//! objects are COFF and no disassembler is acquired, so the step SKIPs
//! loudly; the interpreters (lupin and the checked machine generate no
//! code); and timing. No instruction-latency measurement is claimed.

use crate::{bins, exec, stage};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// The census entry: every public function of the four modules, fed from
/// `argv`. It must not divide anywhere itself (the scan covers `main`).
pub const ENTRY: &str = r#"// cargo xtask codegen: every public function of the secret-handling
// modules, fed from argv so nothing is a compile-time constant.
use std.bech32
use std.x.crypto.chacha20
use std.x.crypto.curve25519
use std.x.crypto.sha2

fn main() -> int {
    let a = env_args_bytes()
    if a.len < 4 { return 2 }
    let k = copy a[1]
    let u = copy a[2]
    let n = copy a[3]
    let s = str_from_utf8(copy a[1]) else ""
    var acc: int = 0
    let ssec = curve25519.x25519(k, u) else List[byte]()
    acc = acc + ssec.len + curve25519.x25519_base(k).len
    let pk = curve25519.public(k)
    let sig = curve25519.sign(k, u)
    if curve25519.verify(pk, u, sig) { acc = acc + 1 }
    if curve25519.verify_strict(pk, u, sig) { acc = acc + 1 }
    if curve25519.field_self_test() { acc = acc + 1 }
    acc = acc + curve25519.to_hex(ssec).len
    let box = chacha20.seal(k, n, u, u)
    let opened = chacha20.open(k, n, box, u) else List[byte]()
    acc = acc + opened.len + chacha20.poly1305(k, u).len + chacha20.poly_key(k, n).len
    acc = acc + chacha20.block(k, 1, n).len + chacha20.stream_xor(k, 1, n, u).len
    acc = acc + chacha20.to_hex(box).len
    var st = sha2.new256()
    sha2.update256(mut st, u)
    acc = acc + sha2.final256(take st).len + sha2.sum256(u).len + sha2.hmac256(k, u).len
    acc = acc + sha2.hkdf256(n, k, u, 32).len + sha2.hkdf256_extract(n, k).len
    acc = acc + sha2.hkdf256_expand(k, u, 64).len + sha2.sum512(u).len + sha2.sum384(u).len
    acc = acc + sha2.hmac512(k, u).len + sha2.hmac384(k, u).len
    acc = acc + sha2.hkdf512(n, k, u, 32).len + sha2.hkdf384(n, k, u, 32).len
    acc = acc + sha2.to_hex(u).len
    let id = bech32.decode_bytes(s, "age-secret-key-", 32) else List[byte]()
    let d = bech32.decode(s) else bech32.Decoded { hrp: "", data: List[byte]() }
    let enc = bech32.encode_bytes("age-secret-key-", k) else ""
    let enc2 = bech32.encode("age", d.data) else ""
    acc = acc + id.len + d.data.len + enc.len + enc2.len
    acc & 127
}
"#;

/// A program that must divide: `n / d` with both from `argv`.
pub const PROBE: &str = r#"// cargo xtask codegen's self-check: this divides by a runtime value.
fn main() -> int {
    let a = env_args_bytes()
    let n = a.len * 1000 + 7
    let d = a.len + 1
    n / d
}
"#;

/// The modules the gate holds, by the dotted prefix of their functions.
const MODULES: [&str; 4] = [
    "std.x.crypto.curve25519.",
    "std.x.crypto.chacha20.",
    "std.x.crypto.sha2.",
    "std.bech32.",
];

/// Functions the DEFAULT tier must show by name (it emits every function
/// of an imported module): the ones the census found dividing. A rename
/// that drops one from the object without dropping it from the source
/// cannot pass silently.
const DEFAULT_ANCHORS: [&str; 10] = [
    "std.x.crypto.curve25519.ladder",
    "std.x.crypto.curve25519.fe_carry",
    "std.x.crypto.curve25519.fe_freeze",
    "std.x.crypto.curve25519.fe_tobytes",
    "std.x.crypto.curve25519.clamp",
    "std.x.crypto.chacha20.poly1305",
    "std.x.crypto.chacha20.absorb",
    "std.bech32.decode",
    "std.bech32.decode_bytes",
    "std.bech32.encode_bytes",
];

/// Published-vector rows built and run in the release tier, each
/// `check: run(exit=0)`. RFC 7748 (§5.2, §6.1, the 1000-iteration chain,
/// masking, non-canonical u, Wycheproof's shared-secret smoke set),
/// RFC 8032 §7.1, RFC 8439 Poly1305 and AEAD,
/// RFC 4231 and RFC 5869, BIP-173 and the age key examples.
pub const RELEASE_ROWS: [&str; 15] = [
    "x/crypto/curve25519/x25519_rfc7748_s52.lu",
    "x/crypto/curve25519/x25519_rfc7748_s61_dh.lu",
    "x/crypto/curve25519/x25519_rfc7748_chain.lu",
    "x/crypto/curve25519/rfc7748_mask_clamp.lu",
    "x/crypto/curve25519/rfc7748_noncanonical.lu",
    "x/crypto/curve25519/wycheproof_x25519_shared_smoke.lu",
    "x/crypto/curve25519/ed25519_rfc8032_s71.lu",
    "x/crypto/chacha20/rfc8439_a3_poly1305.lu",
    "x/crypto/chacha20/rfc8439_a5_aead.lu",
    "x/crypto/chacha20/tag_positions.lu",
    "x/crypto/sha2/rfc4231_hmac.lu",
    "x/crypto/sha2/rfc5869_hkdf.lu",
    "bech32/bip173.lu",
    "bech32/age_keys.lu",
    "bech32/alphabet_and_widths.lu",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    Default,
    Release,
}

impl Tier {
    fn name(self) -> &'static str {
        match self {
            Tier::Default => "default",
            Tier::Release => "release",
        }
    }
}

/// One integer division found in a listing; `offset` is from the start
/// of its function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Division {
    pub function: String,
    pub offset: u64,
    pub instruction: String,
}

/// One function of a listing: its demangled name and its divisions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub instructions: usize,
    pub divisions: Vec<Division>,
}

pub fn codegen() -> Result<(), String> {
    if cfg!(windows) {
        println!(
            "SKIP: codegen — windows objects are COFF and no disassembler is \
             acquired on this host; the Linux and macOS legs carry this gate"
        );
        return Ok(());
    }
    let repo = crate::repo_root();
    let Some(wolf) = bins::resolve(bins::Impl::Wolf, &repo) else {
        println!(
            "SKIP: no wolf — codegen lane dark (same acquisition story as std-test); \
             set $WOLF_BIN or place it in .wolf-bin/"
        );
        return Ok(());
    };
    let objdump = std::env::var("OBJDUMP").unwrap_or_else(|_| "objdump".to_string());
    let nm = std::env::var("NM").unwrap_or_else(|_| "nm".to_string());
    let version = tool_line(&objdump, &["--version"]).map_err(|e| {
        format!(
            "codegen: no usable disassembler (`{objdump} --version`: {e}); \
             this gate cannot see instructions without one (set $OBJDUMP)"
        )
    })?;
    let wolf_version = tool_line(&wolf.path.to_string_lossy(), &["--version"])?;
    println!(
        "codegen: wolf {} ({}), disassembler {version}",
        stage::show(&wolf.path),
        wolf_version
    );
    let scratch = repo.join("target/stage/codegen");
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch)
            .map_err(|e| format!("codegen: clearing {}: {e}", stage::show(&scratch)))?;
    }
    let std_root = repo.join("std");

    // 1. The self-check: a scan that cannot see a division is blind.
    for tier in [Tier::Default, Tier::Release] {
        let dir = scratch.join(format!("probe-{}", tier.name()));
        let objects = build_objects(&wolf.path, &std_root, &dir, PROBE, tier)?;
        let found: usize = scan_objects(&objdump, &nm, &objects)?
            .iter()
            .map(|f| f.divisions.len())
            .sum();
        if found == 0 {
            return Err(format!(
                "codegen: BLIND — the {} probe divides by a runtime value and \
                 `{objdump}` showed no division instruction in {} object(s); \
                 the scan cannot pass anything until it can see one",
                tier.name(),
                objects.len()
            ));
        }
        println!(
            "PASS codegen self-check {}: the probe's division is seen ({found})",
            tier.name()
        );
    }

    // 2. The modules: no integer division in any function, either tier.
    let mut red = Vec::new();
    for tier in [Tier::Release, Tier::Default] {
        let dir = scratch.join(format!("entry-{}", tier.name()));
        let objects = build_objects(&wolf.path, &std_root, &dir, ENTRY, tier)?;
        let functions = scan_objects(&objdump, &nm, &objects)?;
        coverage(tier, &functions)?;
        let mut count = 0;
        for f in &functions {
            for d in &f.divisions {
                count += 1;
                let line = format!(
                    "FAIL {} {} +0x{:x} {}",
                    d.function,
                    tier.name(),
                    d.offset,
                    d.instruction
                );
                println!("{line}");
                red.push(line);
            }
        }
        let insns: usize = functions.iter().map(|f| f.instructions).sum();
        if count == 0 {
            println!(
                "PASS codegen {}: no integer division in {} function(s), {insns} \
                 instruction(s), {} object(s)",
                tier.name(),
                functions.len(),
                objects.len()
            );
        } else {
            println!(
                "codegen {}: {count} division(s) in {} function(s) scanned",
                tier.name(),
                functions.len()
            );
        }
    }

    // 3. The release tier still gives the published answers.
    let rows_red = release_rows(&repo, &wolf.path, &std_root, &scratch)?;

    if !red.is_empty() || rows_red > 0 {
        return Err(format!(
            "codegen: RED — {} division(s) in the crypto modules' objects, \
             {rows_red} release row(s) failing",
            red.len()
        ));
    }
    println!("codegen: GREEN");
    Ok(())
}

/// First line of `tool args…`'s stdout (or stderr), or why it failed.
fn tool_line(tool: &str, args: &[&str]) -> Result<String, String> {
    let mut cmd = Command::new(tool);
    cmd.args(args);
    let got = exec::run(cmd, Duration::from_secs(30))?;
    if got.status != Some(0) {
        return Err(format!("exit {:?}", got.status));
    }
    let text = if got.stdout.trim().is_empty() {
        got.stderr
    } else {
        got.stdout
    };
    Ok(text.lines().next().unwrap_or("").trim().to_string())
}

/// Build `source` as a staged package in `dir` and return its objects.
fn build_objects(
    wolf: &Path,
    std_root: &Path,
    dir: &Path,
    source: &str,
    tier: Tier,
) -> Result<Vec<PathBuf>, String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("codegen: mkdir: {e}"))?;
    let entry = dir.join("main.lu");
    std::fs::write(&entry, source).map_err(|e| format!("codegen: write entry: {e}"))?;
    let out = dir.join(format!("{}.o", tier.name()));
    let mut cmd = Command::new(wolf);
    cmd.arg("build")
        .arg(&entry)
        .arg("--std-root")
        .arg(std_root)
        .arg("--emit=obj")
        .arg("--no-cache")
        .arg("-o")
        .arg(&out)
        .current_dir(dir);
    if tier == Tier::Release {
        cmd.arg("--release");
    }
    let got = exec::run(cmd, Duration::from_secs(600))?;
    if got.timed_out || got.status != Some(0) {
        return Err(format!(
            "codegen: `wolf build --emit=obj` ({}) failed: {:?}\n{}\n{}",
            tier.name(),
            got.status,
            got.stdout.trim_end(),
            got.stderr.trim_end()
        ));
    }
    let prefix = format!("{}.", tier.name());
    let mut objects: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("codegen: read {}: {e}", stage::show(dir)))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
            name.starts_with(&prefix) && name.ends_with(".o")
        })
        .collect();
    objects.sort();
    if objects.is_empty() {
        return Err(format!(
            "codegen: the {} build wrote no object under {}",
            tier.name(),
            stage::show(dir)
        ));
    }
    Ok(objects)
}

fn scan_objects(objdump: &str, nm: &str, objects: &[PathBuf]) -> Result<Vec<Function>, String> {
    let mut all = Vec::new();
    for obj in objects {
        let mut cmd = Command::new(nm);
        cmd.arg(obj);
        let got = exec::run(cmd, Duration::from_secs(60))?;
        if got.timed_out || got.status != Some(0) {
            return Err(format!(
                "codegen: `{nm}` on {} failed: {:?}\n{}",
                stage::show(obj),
                got.status,
                got.stderr.trim_end()
            ));
        }
        let names = symbol_names(&got.stdout);
        let mut cmd = Command::new(objdump);
        cmd.arg("-d").arg("--no-show-raw-insn").arg(obj);
        let got = exec::run(cmd, Duration::from_secs(120))?;
        if got.timed_out || got.status != Some(0) {
            return Err(format!(
                "codegen: `{objdump} -d` on {} failed: {:?}\n{}",
                stage::show(obj),
                got.status,
                got.stderr.trim_end()
            ));
        }
        all.extend(parse_listing(&got.stdout, &names));
    }
    Ok(all)
}

/// Each tier must show functions of all four modules, and the default
/// tier the named functions the census found dividing.
fn coverage(tier: Tier, functions: &[Function]) -> Result<(), String> {
    let missing: Vec<&str> = MODULES
        .iter()
        .copied()
        .filter(|m| !functions.iter().any(|f| f.name.starts_with(m)))
        .collect();
    if !missing.is_empty() {
        return Err(format!(
            "codegen: the {} objects hold no function of {missing:?} — the build or \
             the listing is not what this gate reads",
            tier.name()
        ));
    }
    if tier == Tier::Default {
        let absent: Vec<&str> = DEFAULT_ANCHORS
            .iter()
            .copied()
            .filter(|a| !functions.iter().any(|f| f.name == *a))
            .collect();
        if !absent.is_empty() {
            return Err(format!(
                "codegen: the default objects lack {absent:?} by name; update \
                 DEFAULT_ANCHORS with the source, never around it"
            ));
        }
    }
    Ok(())
}

/// Parse `objdump -d --no-show-raw-insn` text (GNU on ELF, LLVM on
/// Mach-O) into functions and their integer divisions. `names` maps an
/// address to the object's own symbol there (from `nm`): LLVM's objdump
/// labels the start of a Mach-O section with an assembler temporary
/// (`ltmp0`) and hides the function that begins at the same address, so
/// a temporary label takes the real name when the symbol table has one.
pub fn parse_listing(text: &str, names: &HashMap<u64, String>) -> Vec<Function> {
    let mut out: Vec<Function> = Vec::new();
    let mut start = 0;
    for line in text.lines() {
        if let Some((addr, name)) = label(line) {
            let name = match names.get(&addr) {
                Some(real) if is_temporary(name) => real.as_str(),
                _ => name,
            };
            start = addr;
            out.push(Function {
                name: demangle(name),
                instructions: 0,
                divisions: Vec::new(),
            });
            continue;
        }
        let Some(cur) = out.last_mut() else { continue };
        let Some((address, mnemonic, operands)) = instruction(line) else {
            continue;
        };
        cur.instructions += 1;
        if is_integer_division(&mnemonic) {
            cur.divisions.push(Division {
                function: cur.name.clone(),
                offset: address.saturating_sub(start),
                instruction: format!("{mnemonic} {operands}").trim().to_string(),
            });
        }
    }
    out
}

/// An assembler-local label rather than a function: `ltmp0`, `L…`,
/// `.L…`, `l_…`. Wolf's own symbols begin `_W`, `__W` or `main`.
fn is_temporary(name: &str) -> bool {
    name.starts_with("ltmp")
        || name.starts_with('L')
        || name.starts_with(".L")
        || name.starts_with("l_")
}

/// `nm` output (`ADDRESS TYPE NAME`, GNU or LLVM) → the first symbol at
/// each address that is not an assembler temporary.
pub fn symbol_names(text: &str) -> HashMap<u64, String> {
    let mut names = HashMap::new();
    for line in text.lines() {
        let mut words = line.split_whitespace();
        let (Some(addr), Some(_kind), Some(name), None) =
            (words.next(), words.next(), words.next(), words.next())
        else {
            continue;
        };
        let Ok(addr) = u64::from_str_radix(addr, 16) else {
            continue;
        };
        if !is_temporary(name) {
            names.entry(addr).or_insert_with(|| name.to_string());
        }
    }
    names
}

/// `0000000000000000 <symbol>:` → (0, `symbol`).
fn label(line: &str) -> Option<(u64, &str)> {
    let line = line.trim_end();
    let (addr, rest) = line.split_once(' ')?;
    if addr.is_empty() || !addr.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let name = rest.strip_prefix('<')?.strip_suffix(">:")?;
    Some((u64::from_str_radix(addr, 16).ok()?, name))
}

/// `     23d:\tdiv    %sil` → (0x23d, "div", "%sil").
fn instruction(line: &str) -> Option<(u64, String, String)> {
    let (addr, rest) = line.split_once(':')?;
    let addr = addr.trim();
    if addr.is_empty() || !addr.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let offset = u64::from_str_radix(addr, 16).ok()?;
    let mut words = rest.split_whitespace().peekable();
    // Instruction prefixes GNU objdump prints as their own word.
    while let Some(w) = words.peek() {
        if matches!(
            *w,
            "rex" | "rex.W" | "rex.B" | "rex.WB" | "lock" | "data16" | "cs" | "ds"
        ) || w.starts_with("rex.")
        {
            words.next();
        } else {
            break;
        }
    }
    let mnemonic = words.next()?.to_string();
    let operands = words.collect::<Vec<_>>().join(" ");
    let operands = operands
        .split(['#', '<'])
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    Some((offset, mnemonic, operands))
}

/// An integer divide: x86 `div`/`idiv` with any AT&T width suffix, or
/// arm64 `sdiv`/`udiv`. Floating-point and vector divides (`divsd`,
/// `vdivps`, `fdiv`) are not integer division and are not counted.
pub fn is_integer_division(mnemonic: &str) -> bool {
    let m = mnemonic.to_ascii_lowercase();
    let x86 = m
        .strip_prefix('i')
        .unwrap_or(&m)
        .strip_prefix("div")
        .is_some_and(|rest| matches!(rest, "" | "b" | "w" | "l" | "q"));
    x86 || m == "sdiv" || m == "udiv"
}

/// `__Wstd.x.crypto.curve25519.ladder$0c2559fbca43eaa0` →
/// `std.x.crypto.curve25519.ladder`; a `.cold.N` split keeps its suffix.
pub fn demangle(symbol: &str) -> String {
    let s = symbol.trim_start_matches('_');
    let s = s.strip_prefix('W').unwrap_or(s);
    let (base, tail) = match s.split_once('$') {
        Some((base, rest)) => {
            let hash_len = rest.bytes().take_while(|b| b.is_ascii_hexdigit()).count();
            (base, &rest[hash_len..])
        }
        None => (s, ""),
    };
    format!("{base}{tail}")
}

/// Build and run each of [`RELEASE_ROWS`] in the release tier; returns the
/// number that failed. A dark runtime is a loud SKIP, as for std-test.
fn release_rows(
    repo: &Path,
    wolf: &Path,
    std_root: &Path,
    scratch: &Path,
) -> Result<usize, String> {
    if bins::native_rt(repo).is_none() {
        println!(
            "SKIP: {} — codegen release rows dark",
            bins::native_rt_absence(repo)
        );
        return Ok(0);
    }
    let mut red = 0;
    for row in RELEASE_ROWS {
        let entry = repo.join("tests").join(row);
        let text = std::fs::read_to_string(&entry)
            .map_err(|e| format!("codegen: {}: {e}", stage::show(&entry)))?;
        if !text.lines().any(|l| l.trim() == "//! check: run(exit=0)") {
            return Err(format!(
                "codegen: tests/{row} is not a `check: run(exit=0)` row; \
                 RELEASE_ROWS holds only rows whose verdict is their exit"
            ));
        }
        let dir = scratch.join("rows").join(row.replace('/', "__"));
        let staged = stage::stage_test(&entry, std_root, &dir)?;
        let mut cmd = Command::new(wolf);
        cmd.arg("run")
            .arg("--release")
            .arg("--std-root")
            .arg(&staged.std_root)
            .arg(&staged.entry)
            .current_dir(&staged.root);
        let got = exec::run(cmd, Duration::from_secs(600))?;
        if got.status == Some(0) && !got.timed_out {
            println!("PASS codegen release row {row}");
        } else {
            red += 1;
            println!(
                "FAIL release-row:{row} exit={:?} timed_out={}\n{}\n{}",
                got.status,
                got.timed_out,
                got.stdout.trim_end(),
                got.stderr.trim_end()
            );
        }
    }
    Ok(red)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GNU_X86: &str = "
release.c04.o:     file format elf64-x86-64


Disassembly of section .text:

0000000000000000 <_Wstd.x.crypto.curve25519.ladder$0c2559fbca43eaa0>:
       0:\tpush   %rbp
     239:\tmovzbl (%rcx,%rax,1),%eax
     23d:\tdiv    %sil
     240:\tidivq  0x8(%rsp)
     244:\tdivsd  %xmm1,%xmm0
     248:\trex.W idiv %rcx
     24c:\tret

0000000000000250 <_Wstd.bech32.step$11>:
     250:\tshr    $0x19,%rax
";

    const LLVM_ARM64: &str = "
release.c07.o:\tfile format mach-o arm64

Disassembly of section __TEXT,__text:

0000000000000000 <__Wstd.bech32.decode_bytes$869c9ce9cf1af1c9>:
       0:      \tstp\tx20, x19, [sp, #-0x20]!
     8f8:      \tsdiv\tx28, x21, x27
     8fc:      \tfdiv\td0, d1, d2
     900:      \tudiv\tw1, w2, w3

0000000000000904 <__Wstd.bech32.decode$eed6de10355c1b3a.cold.1>:
     904:      \tbrk\t#0x1
";

    #[test]
    fn gnu_x86_divisions_are_found_by_function_and_offset() {
        let f = parse_listing(GNU_X86, &HashMap::new());
        assert_eq!(f.len(), 2);
        assert_eq!(f[0].name, "std.x.crypto.curve25519.ladder");
        let found: Vec<(u64, &str)> = f[0]
            .divisions
            .iter()
            .map(|d| (d.offset, d.instruction.as_str()))
            .collect();
        assert_eq!(
            found,
            vec![
                (0x23d, "div %sil"),
                (0x240, "idivq 0x8(%rsp)"),
                (0x248, "idiv %rcx")
            ]
        );
        assert_eq!(f[0].instructions, 7);
        assert_eq!(f[1].name, "std.bech32.step");
        assert!(f[1].divisions.is_empty());
    }

    #[test]
    fn llvm_arm64_divisions_are_found_and_float_divides_are_not() {
        let f = parse_listing(LLVM_ARM64, &HashMap::new());
        assert_eq!(f[0].name, "std.bech32.decode_bytes");
        let offsets: Vec<u64> = f[0].divisions.iter().map(|d| d.offset).collect();
        assert_eq!(offsets, vec![0x8f8, 0x900]);
        assert_eq!(f[0].divisions[0].instruction, "sdiv x28, x21, x27");
        assert_eq!(f[1].name, "std.bech32.decode.cold.1");
    }

    #[test]
    fn a_temporary_label_takes_the_symbol_tables_name() {
        let listing = "
0000000000000000 <ltmp0>:
       0:      \tstp\tx20, x19, [sp, #-0x20]!
      10:      \tudiv\tw1, w2, w3

0000000000000020 <__Wstd.bech32.step$22>:
      20:      \tsdiv\tx1, x2, x3
";
        let nm = "0000000000000000 t ltmp0\n\
                  0000000000000000 T __Wstd.bech32.hrp_state$6aa17b7b44e0b5d5\n\
                  0000000000000020 T __Wstd.bech32.step$22\n\
                                   U _wolf_rt_alloc\n";
        let names = symbol_names(nm);
        let f = parse_listing(listing, &names);
        assert_eq!(f[0].name, "std.bech32.hrp_state");
        assert_eq!(f[0].divisions[0].offset, 0x10);
        assert_eq!(f[1].name, "std.bech32.step");
        assert_eq!(f[1].divisions[0].offset, 0);
    }

    #[test]
    fn integer_division_mnemonics() {
        for m in [
            "div", "divb", "divw", "divl", "divq", "idiv", "idivl", "idivq", "sdiv", "udiv", "SDIV",
        ] {
            assert!(is_integer_division(m), "{m}");
        }
        for m in [
            "divsd", "divss", "vdivps", "fdiv", "idivx", "shr", "and", "div.s",
        ] {
            assert!(!is_integer_division(m), "{m}");
        }
    }

    #[test]
    fn symbols_demangle_to_dotted_paths() {
        assert_eq!(
            demangle("__Wstd.x.crypto.chacha20.poly1305$dbd843ba59ba704e"),
            "std.x.crypto.chacha20.poly1305"
        );
        assert_eq!(demangle("_Wmain$70962f7628a8b469"), "main");
        assert_eq!(demangle("_main"), "main");
        assert_eq!(demangle("OUTLINED_FUNCTION_0"), "OUTLINED_FUNCTION_0");
    }

    #[test]
    fn coverage_refuses_a_listing_without_the_modules() {
        let f = parse_listing(GNU_X86, &HashMap::new());
        assert!(coverage(Tier::Release, &f).is_err());
    }

    #[test]
    fn the_release_rows_exist_and_answer_by_exit() {
        let repo = crate::repo_root();
        for row in RELEASE_ROWS {
            let text = std::fs::read_to_string(repo.join("tests").join(row))
                .unwrap_or_else(|e| panic!("tests/{row}: {e}"));
            assert!(
                text.lines().any(|l| l.trim() == "//! check: run(exit=0)"),
                "tests/{row} is not check: run(exit=0)"
            );
        }
    }
}
