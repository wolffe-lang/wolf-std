// Thin runner over the repository's real directive parser and classifier.
// No copied namespace or anchor rules; no program execution or dark lanes.
#[path = "/home/mfwolffe/lanes/sc70/work/xtask/src/anchors.rs"]
mod anchors;
#[path = "/home/mfwolffe/lanes/sc70/work/xtask/src/directive.rs"]
mod directive;
use std::path::{Path, PathBuf};
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in std::fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() { walk(&p, out) }
        else if p.extension().and_then(|s| s.to_str()) == Some("lu") { out.push(p) }
    }
}
fn main() {
    let root = PathBuf::from(std::env::args().nth(1).expect("repo root"));
    let registry = anchors::Registry::load(&root).unwrap();
    let mut paths = Vec::new(); walk(&root.join("tests"), &mut paths); paths.sort();
    let mut tags = 0; let mut forward = 0; let mut errors = 0;
    for p in &paths {
        let rel = p.strip_prefix(&root).unwrap().to_str().unwrap();
        let src = std::fs::read_to_string(p).unwrap();
        let d = directive::parse(&src, rel).unwrap(); d.validate_entry(rel).unwrap();
        for tag in d.conforms {
            tags += 1;
            match registry.classify(&tag) {
                Ok(anchors::TagClass::Registered) => {},
                Ok(anchors::TagClass::Forward) => forward += 1,
                Err(e) => { errors += 1; println!("{rel}: {e}"); },
            }
        }
    }
    println!("conform-tag-lint: {} files, {tags} tags, {forward} forward, {errors} errors, 0 skips", paths.len());
    if errors != 0 { std::process::exit(1) }
}
