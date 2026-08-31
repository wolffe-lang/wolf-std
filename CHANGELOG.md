# Changelog

## sc28 — 2026-08-30 — the library writes the new words

Pins advance to wolf v0.2.0 and lupin 0.1.18, both D57-bare release
builds, and doctor now gates wolf's provenance from `--version` itself
— a binary naming no pin is a red. The four `divergent(…)` ledger rows
flip to `run` exactly as 0.1.18's notes predicted. The c33/c39 surface
is adopted, not just measured: `str +` at 23 of 27 candidate sites,
tuple destructuring at 10 sites, comma-grouped binders, and list
slices in 12 loops at 9 sites — with one lend retreat filed rather
than papered (F-0101, wolf-lang#184). `gram.lex.ident` returns to the
anchor registry; the ledger stays flat throughout.

## sc27 — 2026-08-30 — the divergences re-measure

Pins converge on wolf 0a5c1af and lupin 0.1.17; drift was predicted
zero/zero and measured zero/zero. Residue row 6 heals — `str + str`
runs on all three lanes (D62) — and #50 healed with the sc22 cursor
boundary unmoved. The divergent four (wolf-interp#47/#48) stayed
unhealed at their third measurement, commented in place. Anchors +5,
clean; adoption candidates filed for the next contract, not taken.

## sc26 — 2026-08-29 — the native lane comes home

wolf at c1ca543 (the s59 apple wave plus s125–s127) un-parks the
staticlib: all three lanes light on macOS/arm64, with 364 of 365 rows
confirming the linux record silently, one s123 healing, and zero
divergences. Native x/ rows run in ≤1.4 s against the 60 s ceiling.
F-0096 re-measured unhealed; F-0100 filed (wolf-lang#170, the
spec-extract anchor loss).

## sc25 — 2026-08-28 — the waivers die

lupin 0.1.15 brings both binaries and the data pin to one sha: three
waivers dead and three lupin ledger columns flip to `run`.
std.unicode is retyped on `char` per D58 — `from_code -> char !
{none}`, the predicates compare char literals, `code`/`from_code` the
only int doors, `code_points` staying int deliberately. The four
divergent rows re-measured unhealed; the `slow` word retires on the
arm64 rig's own refusal.

## sc24 — 2026-08-28 — the twenty-eight re-measured

F-0018's 28 boundary-blocked contracts, re-tested clause by clause
across two wolf acquisitions: 24 of 28 shipped — including three new
this sprint on the D58 char surface (`str.to_list_chars`,
`strbuf.push(char)`, `unicode.code(char)`) — and a four-item residue
owned by name. `^n` healed; F-0096–F-0099 filed; the `divergent(…)`
ledger word is born for rows whose wrongness is the counterparty's.
35 rows deeper, 25 honest-downs. Pins a900b8c + lupin 0.1.14.

## The TLS rungs — sc20 → sc23 (2026-08-27)

- **sc23** — ECDSA-P256 as evidence: RFC 6979, Wycheproof
  secp256r1 (484 vectors) and CAVP 186-4 SigVer/SigGen vendored, every
  vector reproduced by a from-scratch big-int reference first; the
  emitters are native-only — a P-256 verify is two table-free ladders,
  past the checked and lupin step budgets. The data-pin bump moved
  zero ledger rows.
- **sc22** — the certificate: an X.690 DER decoder, RFC 5280 X.509
  profile, and Ed25519 chain validation over an in-repo test PKI; 25
  malformed inputs reject by name, and an unsupported algorithm is
  named, never accepted. F-0079 found fixed upstream (lupin 0.1.13)
  and closed.
- **sc21** — the handshake: message flow, transcript hash, the X25519
  key schedule, Finished, and the client state machine, RFC 8448-gated
  on all three lanes. The ws05 seam freezes.
- **sc20** — the record layer: HKDF-Expand-Label, per-record nonce,
  TLSCiphertext seal/open over ChaCha20-Poly1305, RFC 8448-gated,
  three-lane. F-0094 and F-0095 (wolf-lang#139) filed.

## The crypto ladder — sc16 → sc19 (2026-08-26/27)

- **sc19** — the ACME signature: JWS/EdDSA compact and flattened, OKP
  JWK with the RFC 7638 thumbprint, and the ACME request body
  (RFC 7515/8037/7638/8555), byte-exact against the RFC vectors; the
  ws06 seam freezes. The native/wolfc rows are the environment
  crypto-drift class; lupin green, CI-gated.
- **sc18** — the curve and the signature: X25519 and Ed25519 from
  RFC 7748/8032 over a checked-int field, 669 Wycheproof vectors with
  every flag decided; the dark lane lights — 41 rows. Typed-int limb
  discipline kills the i32-literal default.
- **sc17** — the cipher that needs no tables: ChaCha20-Poly1305 from
  RFC 8439, 353 vectors green; the tag compare marked for the
  constant-time tier.
- **sc16** — the digest ladder: SHA-2, HMAC and HKDF written from the
  documents, 1210 vectors green; the `slow` ledger word is born for a
  lane whose semantics are right and whose clock is not.

## The text and json turn — sc12 → sc15 (2026-08-13 → 08-26)

- **sc15** — the DOM half: std.json gains a checked handle over the
  declared json kernels, rows verbatim.
- **sc14** (2026-08-24) — each word, without the list: `each_word` as
  the lazy walk and `words_count` as the counting fold.
- **sc13** (2026-08-21) — the callable core, std's first higher-order
  tier: list predicates and relations as `fn` values; both F-0052
  wildcard handlers retire (the arms discriminate at lupin 0.1.13),
  the json F-0079 split retires and its witness catches the sequel
  (F-0084); F-0082–F-0086 filed from the sprint's probes.
- **sc14** (2026-08-14) — the four owed contracts, all paid:
  `json.parse`, `json.unescape`, `escape`'s totality, and
  `hex.decode_str`.
- **sc13** (2026-08-13) — the json DOM, and a way back from bytes:
  `bytes.to_str` lands on s81's `str_from_utf8` (F-0057 closed after
  four sprints of refusing an ASCII-only border post).
- **sc12** (2026-08-13) — eight functions walk the s77 byte view
  instead of copying.

(The ids sc13 and sc14 were each used twice in the track's history;
the entries above are ordered by landing date.)

## The os surface — sc07 → sc11 (2026-08-11/12)

- **sc11** — std.process: the pure builder over the process trio
  (`start` not spawn, `slot` not handle — both were keywords,
  F-0062), checked-lane honest; the signal row joins the taxonomy;
  `io.input_all` and `net.read_all` rewritten on the now-legal
  re-raising loop; seven ledger rows flip on the s71/s72 rulings.
- **sc10** — std.time (instant and duration facades, exact RFC 3339
  render, the Clock tag), std.env (argv arrives in std), and
  std.x.json — the nursery's first tenant, with its banner and
  three-outcome graduation clock.
- **sc09** — the F-0018 prize spent: std.str grows 16 → 37 functions,
  std.bytes is born at 9, twenty census rows flip, and the first
  all-three-lane test block lands.
- **sc08** — std.net: ten functions and two types over the s39
  builtins, the row vocabulary verbatim, take-consumed close;
  `read_all` written then withdrawn on the sprint's headline finding
  (F-0052, filed).
- **sc07** — std.fs (15 functions plus File with take-consumed close)
  and std.io (the write family, `input_line`, `prompt`); `copy`
  renamed `copy_file` (reserved keyword), `read_line` renamed
  `input_line` (prelude shadowing recursed under lupin); ten blocked
  contracts, filed.

## The core library — sc01 → sc06 (2026-08-10/11)

- **sc06** — std.option lands, six functions executing under lupin;
  errors at five and testing at thirteen with static-contract freight
  stated per function; the lowercase-tag rename (148 occurrences,
  zero ledger movement).
- **sc05** — fmt and encoding: exact float formatting (half-even
  fixed and exp, genuinely shortest `to_str`, correctly rounded
  `parse_float`), hex and base64 over a documented 0..255 byte
  contract, json escaping with 22 bodies; `hex.encode(str)` stated
  impossible rather than worked around.
- **sc04** — math, sort, search: 71 of 74 contracts with bodies;
  pure-wolf transcendentals at measured ulp 1 (3 for `powf`); the
  pcg32 stream pinned and cross-checked.
- **sc03** — strings and bytes: str 18 via the one safe primitive,
  strbuf 8, unicode 9 with the 25-entry case table pinned per code
  point; ascii-only find/split refused on principle; 28 contracts
  blocked on the missing boundary primitive — F-0018, the track's
  central filing.
- **sc02** — collections: std.list 19 executing, std.map 13, pool
  reserve/init; set and deque generics held as reviewed contracts
  blocked on `struct[T]`.
- **sc01** — cmp complete (Ordering with IEEE total order, Eq/Ord,
  min/max/clamp), testing five, iter prototypes; option blocked on
  row-position parsing, filed.

## sc00 — 2026-08-10 — the rig

The track opens: pins to the wolf-lang data snapshot and both
implementation binaries, an xtask `std-test` runner staging every
module through both implementations' `conform-run` with record
validation and bidirectional ledger enforcement, `doctor`,
`sync-pin`, the findings register (F-0001 on day one), and the
prelude proof module — `least`, `greatest`, `magnitude` — green under
both implementations with identical stdout hashes.
