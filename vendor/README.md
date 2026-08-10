# vendor/

Tracked pin data, kept because the `upstream/` submodule is private and
CI cannot clone it (org policy disables deploy keys — the lesson ls00 §3
already paid for). Retire the snapshot half when the upstream repo
becomes readable to CI.

- `upstream/PIN` — the wolf-lang commit the repo is pinned to (data pin:
  `spec/` + `corpus/` semantics).
- `upstream/anchors.json` — byte-identical snapshot of
  `spec/anchors.json` at the PIN; the `conforms:` registry
  (`[conf.tag.valid]`) CI checks tags against.
- `tools.toml` — recorded versions + pins for the two implementation
  binaries (`lupin`, `wolf`); `cargo xtask doctor`'s contract.

Rules: NEVER edit snapshots by hand (`cargo xtask sync-pin` verifies
snapshot == submodule whenever the submodule is initialized); re-vendor
on pin bumps, each bump its own commit (see CONTRIBUTING.md).
