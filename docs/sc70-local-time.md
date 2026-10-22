# sc70 — local time

GPT-6.1-Sol, crawl 54. Base `3bacc5f`; wolf 0.2.34 / lupin 0.1.55 unchanged.

## Prediction (before implementation)

Aspen’s `src/clock/clock.lu` at `ap01` is sound enough to port for valid
zone files, including Lord Howe’s half-hour DST. It is not a hostile-input
parser: counts, type indexes, abbreviation bounds, trailing POSIX text and
some time fields need validation. Its unnamed default US DST rule should
not become a portable promise. Recommend rejecting DST strings without
explicit rules.

I predict 0 of 10,000 oracle cells disagree on the first complete run
(random seconds in 1900–2100; five zones, ten formats). Failures, if any,
will be historical `%Z`/`%z` or padding, not the calendar conversion.
Lord Howe should not expose a one-hour assumption: offsets and change
instants are seconds throughout the imported arithmetic.

## Rulings requested

- R-sc70-1: recommend `std.time` for `Local`, `local`, `local_in`,
  `offset_at`, `to_unix`, `format`; keep the TZif/POSIX machinery private.
  This is one calendar surface, not a second public zone object API.
- R-sc70-2: recommend that `to_unix` trusts `Local.offset_s`. A fold’s two
  values retain their own offset and invert exactly; a forged gap value
  is interpreted at its supplied offset without consulting the host.
  No guessed one-hour correction and no implicit earliest/latest choice.

## Inputs

The contract’s `sprints/stdlib/index.md` is named `README.md` in this tree.
There is no sc69 prose note: its record is CHANGELOG’s sc69 section,
`docs/findings.md` F-0156 and `docs/sc69-witness/`.
The shared aspen trunk has no src yet; the reader is in
`/Users/mfwolffe/lanes/ap01/aspen/src/clock/clock.lu` (read-only), also on
`origin/ap01`. Source is the maintainer’s own code, not libc or tzcode.
