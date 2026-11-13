# Unicode data

Unicode 15.1.0 normative data is vendored without modification in `15.1.0/`.
`sources.json` records each original URL and SHA-256. The license is
[Unicode-3.0](15.1.0/license.txt), copyright Unicode, Inc. Its notice applies
also to the derived tables and conformance fixtures. No C-library or
terminal implementation source is used.

Regenerate with the pinned compiler formatter:

```
python3 -I xtask/unicode/generate.py --wolf "$WOLF_BIN"
```

`cargo xtask gen-vectors --check` runs the same generator and checks every
committed byte, including the table, derived metadata, 36 exhaustive scalar
parts, 19 grapheme conformance parts and their generated ledger block. When
the compiler is absent, the existing gate's loud presence-only posture applies
to Wolf source only; data digests and derived metadata are still checked.
Generator invariants run under `cargo test`, including corrupted-input refusal.

`derived.json` enumerates the complete assigned census and all 350 named
compatibility scalars. Base width follows Mn/Me, default-ignorable Cf, East
Asian W/F and Emoji_Presentation, with nonprinting assignment/control/separator
handling separate from the five compatibility classes. `oracle.py` checks
every assigned scalar by calling glibc 2.39 `wcwidth` in `C.UTF-8` as a black
box. `rows.py` requires all three pinned executors and checks the matching
portable fixtures; missing/unsupported lanes are errors, not successful skips.
The ordinary `std-test` gate also executes every fixture on those three lanes.

Grapheme boundaries implement the default extended rules of UAX #29 revision
43 (Unicode 15.1.0), including GB9c. `GraphemeBreakTest.txt` supplies all 1,187
conformance cases. String widths remain additive scalar widths, not shaped
emoji widths; boundaries protect truncation without changing the sum.
