# sc76 — wolf-std: incremental SHA-2 in bounded memory

Materialised from PR #115's body at the merge (`5b3a599`); the lane handed its note back in the PR body.

Footprint: module

## Forbidden

No crypto-library implementation source read; only the org's own sources and the cited FIPS/spec clauses. No public API, digest, ct comment, toolchain pin or consumer repository changed. Every growing run was on Linux under `prlimit --as=2147483648`; no Mac build or growing run. kasumi CPUs 8–11, one long flock; scratch under the lane, never system temp. No labels, merges, tags or issues changed.

These five sections are the lane note; the coordinator will materialise `docs/sc76-the-bounded-hash.md` from this scrubbed body at the merge.

## Inputs, verified

- Base `dee6206`; final head `5b3a59942b2ee69b99682544341ce8a12c984e9e`.
- wolf 0.2.36 / lupin 0.1.58 unchanged. Linux release API digests read twice, archives hashed twice: wolf `d383c74c8cd123b9d9dfaa268271d4f004294f85c2b230237d0ef07027d9500d`, lupin `34f52792bb59bc7565e0edc1235cd47d9a6ad13d5e6528993627f2e699b62193`. Binaries/version lines and runtime archive matched sc75, binary hashes read twice.
- API-CONVENTIONS is at the repository root, not the contract's missing `docs/` path. Read with sc75 and the region specification.
- State-owned schedules are allocated once, overwritten by round index, and block buffers use `reset()`. Caller ambient allocation `[mem.region.create.3]` and wholesale-only freeing `[mem.region.intra.2]` explain the leak. Reuse needs no scratch allocation or region escape (`[mem.region.escape]`). Public function signatures and every ct comment were mechanically compared equal to trunk. SHA-384 shares the SHA-512 core.

## Prediction

Committed first in `docs/sc76-prediction.txt`. The region-bound tests were committed at `1bcd350` and seen `trap(alloc-contract)` at trunk on native and checked before `e239dcb` fixed them. The long RSS rows were red under the cap, not run uncapped on Mac.

Afterward both families satisfy the incremental bound: native SHA-256 delta **+56 KiB**, SHA-512 **−68 KiB**; release SHA-256 **−88 KiB**, SHA-512 **+168 KiB** (all strictly below 8192 KiB). All completed 16/64/1024 MiB digests, including one-shot sums, match the independent hashlib API oracle.

## Evidence index

`docs/sc76-witness/` contains the public SHA-2 drivers, source measurements and standard-library inventory; consumer-specific artifacts are retained privately. Native/release peak RSS and wall time follow; `CAP` is exit 134 at the hard 2 GiB address-space limit, not a digest result. The original before timer also includes the small reference-hash verification; the corrected after timer isolates child wall time.

| Tier | SHA | API | MiB | Before KiB / s | After KiB / s |
|---|---|---|---:|---:|---:|
| native | 256 | update | 16 | 307708 / 0.527 | 2412 / 0.481 |
| native | 256 | update | 64 | 1223332 / 2.129 | 2472 / 1.913 |
| native | 256 | update | 1024 | 2094824 / 3.604 CAP | 2528 / 30.485 |
| native | 256 | sum | 16 | 324064 / 0.555 | 18948 / 0.494 |
| native | 256 | sum | 64 | 1288644 / 2.234 | 68052 / 1.974 |
| native | 256 | sum | 1024 | 2094656 / 3.556 CAP | 1050984 / 32.101 |
| native | 512 | update | 16 | 561968 / 0.479 | 2564 / 0.342 |
| native | 512 | update | 64 | 2094824 / 1.762 CAP | 2564 / 1.359 |
| native | 512 | update | 1024 | 2094776 / 1.765 CAP | 2496 / 22.547 |
| native | 512 | sum | 16 | 578236 / 0.513 | 18868 / 0.361 |
| native | 512 | sum | 64 | 2094788 / 1.829 CAP | 68056 / 1.468 |
| native | 512 | sum | 1024 | 2094712 / 2.682 CAP | 1051116 / 23.189 |
| release | 256 | update | 16 | 307732 / 0.193 | 2436 / 0.101 |
| release | 256 | update | 64 | 1223432 / 0.762 | 2528 / 0.403 |
| release | 256 | update | 1024 | 2094760 / 1.296 CAP | 2440 / 6.368 |
| release | 256 | sum | 16 | 324000 / 0.2 | 18908 / 0.11 |
| release | 256 | sum | 64 | 1288900 / 0.808 | 67912 / 0.443 |
| release | 256 | sum | 1024 | 2094816 / 1.246 CAP | 1050868 / 7.006 |
| release | 512 | update | 16 | 562012 / 0.243 | 2452 / 0.066 |
| release | 512 | update | 64 | 2094760 / 0.849 CAP | 2288 / 0.26 |
| release | 512 | update | 1024 | 2094792 / 0.845 CAP | 2456 / 4.135 |
| release | 512 | sum | 16 | 578468 / 0.25 | 18888 / 0.076 |
| release | 512 | sum | 64 | 2094592 / 0.858 CAP | 68024 / 0.298 |
| release | 512 | sum | 1024 | 2094816 / 1.028 CAP | 1051024 / 4.75 |

Checked measurements for every family × API × 16/64/1024 MiB are preserved in the public `trunk-checked-linux.log` and private mixed-consumer evidence: incremental runs and 16 MiB sums exhaust the step budget; 64 MiB and 1 GiB one-shot inputs hit the 2 GiB cap while constructing the checked caller list (3.5/56 GiB reservation requests), with no JSON. These are explicitly **not passing RSS witnesses**. Both new small region-bound rows, existing FIPS known answers, incremental chunking, RFC 4231 HMAC, RFC 5869 HKDF and Ed25519 smoke pass native, release, checked and lupin (28 named exits, warnings empty).

Caller inventory, public std file:line list in `std-callers.txt` and downstream source inventory retained privately:
- SHA-2 itself: SHA-384 delegation, HMAC-256/512 and HKDF-256.
- `std/x/crypto/curve25519/curve25519.lu:1436,1479,1483,1485,1530` (Ed25519 SHA-512).
- `std/x/crypto/p256/p256.lu:2279,2403` and its HMAC paths; `std/x/jose/jose.lu:319`.
- `std/x/tls/handshake/handshake.lu:487,558,566,625`; `std/x/tls/record/record.lu:203,226` through HKDF.
- A downstream package reader uses incremental SHA-256; its source inventory is retained privately.
- den `516e238b`: `src/hash/sha256.lu:6`, facade `hash.sum256`, consumed at `src/oid/oid.lu:67`.
- boreutils uses its own SHA (`src/checksum/sha.lu:49`, dispatched at `src/checksum/digest.lu:85`), not std SHA-2.

A downstream package reader witness, fixed std overridden only in the lane's archived copy: 64 MiB payload / 64 MiB + 1536 archive at **3168 KiB, 2.034 s**; **1 GiB archive**, payload 1 GiB − 1536, at **3116 KiB, 32.199 s**. Both inventory lengths and digests match. The exact downstream 1 GiB *payload* row was not run: its archive would violate the explicit no-file-over-1-GiB rule. No consumer branch or pin changed.

Plant: draft #114, `sc76-plant-schedule` at `727bfbe`. Only the original fresh SHA-256 schedule is restored; buffer reset and SHA-512 stay fixed. Named row `x/crypto/sha2/bounded256.lu` is locally `trap(alloc-contract)` on native/release/checked; its run ledger must be red in CI. The rewritten plant preserves exactly the same source break; its new CI run is read once after the rewrite. CI red at the new head is not claimed seen.

The completed gauntlet ran on kasumi in the isolated non-root container, foreign init, own session, CPUs 8–11. First attempt exposed `/root/.rustfmt.toml` permissions; container HOME was corrected to the lane's scratch home and the gauntlet restarted at the same head. The complete green END lines:

```text
END time-oracle exit=0
END fmt exit=0
END clippy exit=0
END test exit=0
END sync-pin exit=0
END doctor exit=0
END ledger-check exit=0
END lint-conventions exit=0
END fmt-lu exit=0
END gen-vectors exit=0
END std-test exit=0
END doc-examples exit=0
END ulp exit=0
END final-gauntlet exit=0
```

The green kasumi gauntlet at `14274305b5afeac1593fd232bb9361b9ce50f828` also applies to `5b3a59942b2ee69b99682544341ce8a12c984e9e` because `std/` and `tests/` are byte-identical.

The complete gauntlet covered all existing SHA-2/CAVP and caller tests, not just focused samples. Log `/home/mfwolffe/lanes/sc76/logs/gauntlet2-1427430.log`; exit marker `/home/mfwolffe/lanes/sc76/gauntlet2.exit` is 0. Private consumer-specific witness files were moved byte-identically outside the public repository. All eleven original commit messages, author/committer metadata and unrelated content were preserved; only those witness artifacts were removed from history.

<details><summary>Every original witness END line (consumer label anonymised)</summary>

```text
END observe-trunk-bounded256-native exit=0
END observe-trunk-bounded256-checked exit=0
END observe-trunk-bounded512-native exit=0
END observe-trunk-bounded512-checked exit=0
END SHA256-RSS-64MiB-1GiB exit=1
END SHA512-RSS-64MiB-1GiB exit=1
END trunk-RSS-native exit=1
END SHA256-RSS-64MiB-1GiB exit=1
END SHA512-RSS-64MiB-1GiB exit=1
END trunk-RSS-release exit=1
END sha2/bounded256-native exit=0
END sha2/bounded256-release exit=0
END sha2/bounded256-checked exit=0
END sha2/bounded256-lupin exit=0
END sha2/bounded512-native exit=0
END sha2/bounded512-release exit=0
END sha2/bounded512-checked exit=0
END sha2/bounded512-lupin exit=0
END sha2/known_answers-native exit=0
END sha2/known_answers-release exit=0
END sha2/known_answers-checked exit=0
END sha2/known_answers-lupin exit=0
END sha2/incremental_chunking-native exit=0
END sha2/incremental_chunking-release exit=0
END sha2/incremental_chunking-checked exit=0
END sha2/incremental_chunking-lupin exit=0
END sha2/rfc4231_hmac-native exit=0
END sha2/rfc4231_hmac-release exit=0
END sha2/rfc4231_hmac-checked exit=0
END sha2/rfc4231_hmac-lupin exit=0
END sha2/rfc5869_hkdf-native exit=0
END sha2/rfc5869_hkdf-release exit=0
END sha2/rfc5869_hkdf-checked exit=0
END sha2/rfc5869_hkdf-lupin exit=0
END curve25519/ed25519_smoke-native exit=0
END curve25519/ed25519_smoke-release exit=0
END curve25519/ed25519_smoke-checked exit=0
END curve25519/ed25519_smoke-lupin exit=0
END SHA256-RSS-64MiB-1GiB exit=0
END SHA512-RSS-64MiB-1GiB exit=0
END fixed-RSS-native exit=0
END SHA256-RSS-64MiB-1GiB exit=0
END SHA512-RSS-64MiB-1GiB exit=0
END fixed-RSS-release exit=0
END plant-bounded256-native-expected-red exit=0
END plant-bounded256-release-expected-red exit=0
END plant-bounded256-checked-expected-red exit=0
END checked-budget-observations exit=0
END a downstream package reader-RSS-64MiB-exactly1GiB-archive exit=0
```

The `observe-trunk-*` zero exits mean a conformance record was obtained, not a green test: those records say trap. The RSS gate lines are actually red. `expected-red` zeros mean the plant validator observed the required trap. Checked-budget observation completion is not a bound pass.
</details>

## Done-when

- [x] Red-first regression committed and observed at trunk under cap.
- [x] State schedules reused, block buffers reset; API/digests/ct/pins unchanged.
- [x] Native and release RSS bound and complete one-shot digest table through 1 GiB.
- [x] Checked budget/cap limitations measured, small bound rows pass all four machines.
- [x] Focused HMAC/HKDF/Ed25519 witnesses and full caller/consumer inventory.
- [x] A downstream package reader safe-size witness; exact 1 GiB payload remains the ruling below.
- [x] README read and refreshed (561 tests, bounded-hash cost); changelog refreshed.
- [x] Draft plant open with locally named red row.
- [ ] Plant CI red by name; orchestrator watches, no polling here.
- [x] Complete kasumi gauntlet green; unchanged code/tests carry the verdict across the privacy-only rewrite.
- [ ] Main tier pr green at the rewritten head; both new run IDs are read once, then the coordinator watches.
- [ ] Coordinator audit at the rewritten head; not rerun by this lane.

The coordinator will audit the rewritten head; no audit was rerun for this privacy-only rewrite.

R-list, recommendations only (no authority assumed):
- **R-sc76-1:** interpret the 8 MiB RSS delta as the incremental auxiliary-storage bound, and keep raw one-shot RSS separate. Recommended: accept the complete raw sum table plus bounded incremental evidence. A literal 1 GiB one-shot input must hold 960 MiB more caller-owned storage than a 64 MiB input; no API-preserving implementation can make total-process sum RSS differ by under 8 MiB. No API change was attempted.
- **R-sc76-2:** the exact downstream 1 GiB payload needs a 1 GiB + 1536 byte archive, forbidden by the per-file ceiling. Recommended: accept the exactly-1-GiB archive witness (payload 1536 bytes smaller); alternative is explicit permission for the oversized archive, then rerun the original row. The exact requested payload row remains unfulfilled pending that choice.

No issue to file or close is proposed. Windows and Mac large-stream witnesses were not run locally; checked long-stream bounds remain unqualified. The missing Markdown note is replaced by this five-section body, not silently omitted.

Commits:
- a81d214 Record bounded SHA-2 predictions
- 1bcd350 Add bounded SHA-2 regression witnesses
- e239dcb Reuse SHA-2 schedules and block buffers
- 5385314 Ledger bounded SHA-2 rows on all three lanes
- c59e64c Gate focused records and isolate child wall time
- f59b096 Format the expanded SHA-256 round expression
- 350a414 Refresh README for bounded SHA-2 and test count
- d2c15f9 Preserve capped trunk evidence and consumer witnesses
- 178765a Describe bounded incremental SHA-2 in changelog
- 4581f97 Preserve fixed RSS evidence and capped checked outcomes
- 5b3a599 Preserve consumer and checked witnesses with final gauntlet rig

Rewrite CI run: `41571365760` — queued when read once; the coordinator watches.

