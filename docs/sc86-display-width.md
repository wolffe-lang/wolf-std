# Display width

## Scope and prediction

Add a locale-independent display-column family without changing byte-counted
`str.len` or `fmt.pad_left`, `pad_right` and `center`. Unicode 15.1.0 data
and the explicitly enumerated compatibility exceptions below determine scalar
widths. Extended grapheme boundaries determine where truncation may cut.

Before implementation, the predictions are:

- Every one of the 287,346 assigned scalars agrees with black-box `wcwidth`
  in glibc 2.39 under `C.UTF-8`, with NUL returning zero and other controls
  returning absence rather than a negative width.
- The unadjusted Unicode derivation disagrees on exactly the 350 scalars below;
  each exception is necessary, with no catch-all compatibility adjustment.
- Grapheme segmentation passes all Unicode 15.1.0 GraphemeBreakTest cases,
  including Indic conjuncts, regional-indicator parity and emoji ZWJ rules.
- Every scalar, segmentation, truncation and padding row executes on native,
  checked and lupin. Generated-source drift, conventions and doc examples are
  independently gated. A one-hunk table mutation is red by its own row alone.
- Existing byte padding keeps its answers. ASCII scalar lookup is constant
  time; other lookup is a generated balanced decision tree. String walks are
  linear in UTF-8 bytes, with bounded segmentation state, not prefix rescans.

## API and decisions

Recommended public API, all core tier:

- `unicode.display_width(c: char) -> int ! {none}`: zero, one or two columns;
  `none` for a nonprinting scalar. No invalid scalar can be a `char`.
- `str.graphemes(s: str) -> List[str]`: Unicode 15.1.0 default extended
  grapheme clusters, as byte-offset slices of the input.
- `str.display_width(s: str) -> int ! {none}`: the sum of scalar widths,
  walking complete graphemes; `none` if any scalar is nonprinting.
- `str.truncate_display(s: str, columns: int) -> str ! {none}`: the longest
  prefix of complete graphemes whose summed widths fit. Validate the whole
  input before returning, including an omitted suffix. Zero-width graphemes
  fit a zero budget. A negative budget traps `assert`.
- `str.pad_display_left(s: str, columns: int) -> str ! {none}` and
  `str.pad_display_right(s: str, columns: int) -> str ! {none}`: add ASCII
  spaces to a minimum column count; never truncate. Negative columns trap
  `assert`. `none` has the same meaning as for width.

Ambiguous East Asian width is always narrow, independent of locale. NUL is
zero; all other Cc controls, unassigned scalars and noncharacters are
nonprinting. Private-use scalars are narrow. No function reads the locale,
terminal or environment, interprets escapes, expands tabs, or measures fonts.

**Composition policy needs a decision before implementation.** The scalar
compatibility requirement does not determine a grapheme's width. Recommend
additive scalar widths within each grapheme, matching `wcswidth` and the
measured flag pairs. This makes `"🇺🇸"` two columns, but `"👩‍💻"` four rather
than two, and `"❤️"` one rather than two. ZWJ and variation selectors have
zero scalar width; they do not request shaping in this model. Segmentation
protects these clusters from being split, not from being overestimated.
The alternative is a separately specified emoji-presentation policy that
collapses recognized ZWJ sequences and changes VS16/keycap widths. It must
have its own pinned data and sequence oracle; it is not implied by `wcwidth`.

Absence rather than a `-1` sentinel follows API-CONVENTIONS §2. Generated
classification code lives under `std.unicode.tables`, following §10. Existing
byte-oriented formatting is intentionally unchanged, following §11. No
new general-purpose locale or ambiguous-width knob is proposed.

## Data and exact exceptions

Source data: Unicode 15.1.0 `UnicodeData.txt`, `EastAsianWidth.txt`,
`emoji-data.txt`, `DerivedCoreProperties.txt`, `GraphemeBreakProperty.txt`
and `GraphemeBreakTest.txt`, with original Unicode license. Vendor immutable
bytes and SHA-256 digests. A checked generator derives both width and
segmentation tables; `gen-vectors --check` regenerates and compares them in
CI, including the generated exhaustive scalar and grapheme fixtures.

Base width: Mn/Me and default-ignorable Cf are zero; East Asian W/F or
Emoji_Presentation are two (zero takes precedence); other assigned scalars
are one. Separately apply NUL/control/nonprinting handling. Amend only these
five measured classes, cited from warren's width-policy measurements:

| Class | Exact ranges | Count | Base | Compatible |
| --- | --- | ---: | ---: | ---: |
| Soft hyphen | U+00AD | 1 | 0 | 1 |
| Hangul medial/final jamo | U+1160–11FF, U+D7B0–D7C6, U+D7CB–D7FB | 232 | 1 | 0 |
| Interlinear and Egyptian format controls | U+FFF9–FFFB, U+13430–1343F | 19 | 1 | 0 |
| Enclosed numbers and hexagrams | U+3248–324F, U+4DC0–4DFF | 72 | 1 | 2 |
| Regional indicators | U+1F1E6–1F1FF | 26 | 2 | 1 |

These 350 scalars are individually enumerable from disjoint inclusive ranges.
Soft hyphen is printable in the compatibility model; jamo and the named format
controls occupy no independent column; the symbols occupy two; each regional
indicator occupies one. These are observed compatibility decisions, not
claims about Unicode assigning terminal widths. Warren's public measurement
files are `notes/wr01-the-emulator.md` (width-policy section),
`tests/vt/width-divergences.json` and
`tests/vt/reference/width-columns/result.json`. No terminal or libc source
was read.

## Validation and size budget

The black-box oracle must run in the digest-pinned Linux image with glibc
2.39 and `C.UTF-8`; record its image id, libc version and locale. Compare
every assigned scalar, count every exception, and reject an additional or
missing difference. The same exhaustive expected scalar ranges then execute
against the actual Wolf API on all three lanes. Controls are explicitly
classified, not skipped. Include unassigned, noncharacter and private-use
boundary tests outside the assigned census.

Segmentation uses the normative conformance corpus, not a third-party
implementation. Integration rows cover decomposed accents, leading marks,
Hangul, CRLF, Indic conjuncts, flags (single/pair/triple), ZWJ emoji, VS15/VS16,
keycaps, zero budgets, too-wide clusters and an invalid suffix. Each public
function carries executable doc examples. Each trap has its own entry file.

No binary-size figure is claimed before a native build. Measure a minimal
scalar user and a minimal string user against a no-width control with the
same pinned compiler and profile, recording executable and text/rodata sizes.
Report the generated source size and interval count separately; source bytes
are not binary bytes. The implementation is not accepted until these figures
and the exact-head recorder evidence are present.

## Limits and delivery

This is a deterministic compatibility model, not a prediction for every
terminal. Fonts, ambiguous-width preferences, emoji shaping and Unicode
versions differ. Grapheme boundaries are text boundaries, not rendering
measurements. Escape sequences and cursor movement are outside this family.

The composition decision above blocks implementation: additive scalar widths
and emoji-shaped grapheme widths produce observably different answers for
ordinary terminal text, so tests must not silently choose between them.
After that decision, implement the recommendation, run wiring proofs first
in every take, record a one-file one-hunk plant using recorder plant mode,
then run the exact-head gauntlet and one audit. CI wiring changes, if any,
update their recorded baseline in the same commit. Footprint will be
cross-cutting if generation tooling or workflow wiring changes.
