//! sc70: a native witness against the host date binary, on both Unix hosts.
use crate::{bins, exec, repo_root};
use std::process::Command;
use std::time::Duration;

pub fn time_oracle() -> Result<(), String> {
    if cfg!(windows) {
        println!("SKIP: time-oracle needs BSD or GNU date; Windows has neither (1 host skip)");
        return Ok(());
    }
    let repo = repo_root();
    let wolf = bins::resolve(bins::Impl::Wolf, &repo)
        .ok_or("time-oracle: no pinned wolf; this gate never silently skips")?;
    let mut cmd = Command::new("python3");
    cmd.arg("-I")
        .arg(repo.join("tests/time/oracle.py"))
        .arg("--wolf")
        .arg(wolf.path)
        .arg("--std-root")
        .arg(repo.join("std"))
        .arg("--work")
        .arg(repo.join("target/time-oracle"))
        .current_dir(&repo);
    let got = exec::run(cmd, Duration::from_secs(600))?;
    print!("{}", got.stdout);
    eprint!("{}", got.stderr);
    if got.timed_out || got.status != Some(0) {
        return Err("time-oracle: RED (timeout or nonzero exit)".into());
    }
    Ok(())
}
