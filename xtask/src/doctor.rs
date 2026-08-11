//! `cargo xtask doctor` — reports, per implementation, which resolution
//! rung won and whether the binary's `--version` agrees with the
//! recorded pin (`vendor/tools.toml`). Three states per binary:
//! **absent** (legal, loud — lanes will SKIP and say why), **present but
//! pin-mismatched** (worse than absent: doctor fails), **matched**.

use crate::bins::{self, Impl, ToolPin};
use crate::repo_root;
use crate::stage;

pub fn doctor() -> Result<(), String> {
    let repo = repo_root();
    let (lupin_pin, wolf_pin) = bins::load_tool_pins(&repo)?;
    let mut reds = Vec::new();
    for (imp, pin) in [(Impl::Lupin, &lupin_pin), (Impl::Wolf, &wolf_pin)] {
        if let Err(e) = check_one(imp, pin, &repo) {
            reds.push(e);
        }
    }
    match bins::native_rt(&repo) {
        Some(lib) => println!(
            "doctor: native rung — libwolf_rt.a at {} (lane lit)",
            stage::show(&lib)
        ),
        None => println!(
            "doctor: native rung — ABSENT (no libwolf_rt.a beside the wolf \
             binary, no $WOLF_RT_LIB)\n        SKIP: the `native` ledger column \
             will not be observed; build it with `cargo build -p wolf_rt` at \
             the recorded pin"
        ),
    }
    if reds.is_empty() {
        Ok(())
    } else {
        Err(reds.join("\n"))
    }
}

fn check_one(imp: Impl, pin: &ToolPin, repo: &std::path::Path) -> Result<(), String> {
    let name = imp.name();
    let Some(resolved) = bins::resolve(imp, repo) else {
        println!(
            "doctor: {name} — ABSENT (tried ${}, .wolf-bin/{name}, PATH)",
            imp.env_var()
        );
        println!(
            "        SKIP: no {name} at pin {} — binary-dependent lanes will \
             skip loudly, not silently",
            short(&pin.pin)
        );
        return Ok(());
    };
    println!(
        "doctor: {name} — {} (source: {})",
        stage::show(&resolved.path),
        resolved.source
    );
    let v = bins::probe_version(&resolved.path)?;
    println!("        version: {} {}", v.name, v.version);
    if v.name != name {
        return Err(format!(
            "doctor: {name}: binary names itself `{}` — wrong tool on the rung",
            v.name
        ));
    }
    if v.version != pin.version {
        return Err(format!(
            "doctor: {name}: version {} does not match recorded {} \
             (vendor/tools.toml) — a lying binary is worse than none",
            v.version, pin.version
        ));
    }
    match &v.pin_short {
        Some(short_pin) => {
            if pin.pin.starts_with(short_pin.as_str()) {
                println!("        pin: {short_pin} matches vendor/tools.toml — OK");
                Ok(())
            } else {
                Err(format!(
                    "doctor: {name}: --version names pin {short_pin}, recorded pin \
                     is {} — re-acquire at the pin or bump vendor/tools.toml \
                     deliberately",
                    short(&pin.pin)
                ))
            }
        }
        None => {
            // `wolf --version` names no pin pre-M1 — reported, not
            // papered over. The recorded pin documents the acquisition;
            // it cannot be verified from the binary itself yet.
            println!(
                "        WARN: --version names no pin (pre-M1); recorded pin {} \
                 is trusted from acquisition, unverifiable from the binary",
                short(&pin.pin)
            );
            Ok(())
        }
    }
}

fn short(sha: &str) -> &str {
    &sha[..7.min(sha.len())]
}
