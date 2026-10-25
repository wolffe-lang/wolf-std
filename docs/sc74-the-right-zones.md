# sc74 — the right zones

Materialised from wolf-std#107 (head fad69dfe) at the merge.

R-sc74-1 A propagates unreadable/malformed zone errors instead of silent UTC. Commit `b2ad4d0` retires precisely the old fallback assertion; it is the intended behavior change, not a moved expectation masking a defect. Empty TZ remains UTC.

R-sc74-2 uses explicit `Local.leap_correction_s`: zero for ordinary locals; cumulative LEAPCORR minus a positive occurrence's displayed-second increment for leap-aware locals. `second == 60` is valid; offset_s/nanos/abbrev keep their real meanings. This retained net correction also distinguishes duplicate civil fields with historical sub-minute offsets. `to_unix(Local)` remains pure and exact, including at leap occurrences; caller-edited literals supply both explicit offset and correction.

**Source compatibility:** all literal constructors must add `leap_correction_s: 0` for ordinary zones. Commit `833de67` only adds this field to five existing std test literals, preserving their assertions. All other existing time assertions are retained except the approved silent-UTC retirement. In-org `Local {` grep at these checked-out pinned program heads found **no consumer constructors**: aspen `5f99a37e` (std `e036bec`), lope `ce00c738` (prose-only repository, no code/pin), pelt `887ce121` (std `2386522e`), boreutils `37e3825c` (std `2386522e`), yew `12442b8e` (std `2386522e`). The 0.2.36 re-pin lanes must still check later commits and any new literal sites.

The first full-gate launch was prevented by the pool reboot (not a red row). After recovery, two container setup faults were preserved as environment-red logs. A subsequent Rust staging self-test exposed the acquired archive’s bundled std fallback; the lane moved its own bundled copy aside exactly as existing CI does at ci.yml251–260, leaving the pinned binaries/runtime untouched. The resumed full gate at `7a47c8c` uses the existing Ubuntu pack-ci image with host Rust1.97.1 and registry mounted read-only, no installation or rig changes. The one doc-example follow-up at `fad69df` is validated by the two gates it feeds (`fmt-lu`, `doc-examples`) after that full gate; CI at the final head covers both. These two final doc gates were queued but correctly refused to run after the environment-red full gate; they remain owed, as does ULP.

The same integer timestamp means UNIX leap time when the selected file carries leap records, and POSIX UNIX time otherwise, matching date; this is not an implicit POSIX→leap conversion. `local`/`local_in` apply the correction; `offset_at` returns UT offset only; `to_unix` and `%s` invert with retained metadata.

