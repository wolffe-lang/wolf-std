# Findings register

The track's house rule (sc00): a gap in what the language can express is
a **finding filed to wolf-lang**, never a workaround invented here. Every
finding gets a row; the filing link is the proof it left the building.

| id | filed | title | routed to | status |
|---|---|---|---|---|
| F-0001 | 2026-08-10 | No std search path: `use std.*` resolves against a builtin stub (wolfc) / builtin ambient (lupin), never the package; lupin resolves no nested package directory | wolf-lang (s37+ prelude/build plumbing), interp counterpart noted in the filing | [filed: wolf-lang#1](https://github.com/tenseleyFlow/wolf-lang/issues/1) |

## F-0001 — the std search path

Evidence, both implementations at the sc00 pins (wolfc f1f9d80, lupin
0.1.0/cbde620), staged package root containing `std/prelude/prelude.lu`
with a `pub fn`, entry `use std.prelude`:

- **wolfc** routes any `use` whose head segment is `std` to
  `resolve_std_use` -> the builtin stub table (`prelude.rs`
  `STD_MODULES`: only `std.fs.read_text`); the package's `std/` directory
  is never consulted -> `fail(E0301)`. Nested non-std directories DO
  resolve (`use outer.inner` binds `inner` and runs to `exit(0)`).
- **lupin** treats `std.fs` as builtin-ambient (wordcount.lu reaches
  run) but resolves no nested package directory at all: `use outer.inner`
  -> `unsupported` at resolve.

Interim (documented in `xtask/src/stage.rs` and the tests themselves):
the rig stages `std/<mod>/` as `<mod>/` in the scratch package root and
tests import bare module names. The repository tree remains the
namespace (D32); only the staged spelling is interim. When the search
path lands upstream, tests flip to `use std.<mod>` in the same commit as
the pin bump.
