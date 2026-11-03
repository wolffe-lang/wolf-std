# sc77 prediction: a strict Ed25519 verify

Written 2026-11-03 at base `d407430`, before the generator, any test row or any change to `std/`. Each line below is checked against what the lane measures, and a miss is reported as a miss.

## P1. The small-order points

- There are exactly **eight** points of order dividing 8 on edwards25519. The cofactor is 8 (RFC 8032 §5.1: c = 3), and the torsion subgroup is cyclic, so there is 1 point of order 1, 1 of order 2, 2 of order 4 and 4 of order 8.
- The source is the curve equation `-x^2 + y^2 = 1 + d x^2 y^2` and its doubling map, solved by our own Python (`tests/x/crypto/curve25519/small_order.py`). The generator does not read std's wolf code. A second derivation projects arbitrary curve points into the torsion subgroup with `[L]P` and must find the same eight.
- Expected points: the identity `(0, 1)`; `(0, -1)` of order 2; `(±sqrt(-1), 0)` of order 4. The four of order 8 have `x^2 = -y^2` and `d y^4 + 2 y^2 - 1 = 0`.
- Expected canonical encodings, as published in the prose literature on EdDSA validation criteria; the generator must reproduce them:
  - `0100000000000000000000000000000000000000000000000000000000000000` (identity)
  - `ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f` (order 2)
  - `0000000000000000000000000000000000000000000000000000000000000000` and `…0080` (order 4)
  - `c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a` and `…03fa`, `26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05` and `…fc85` (order 8)
- Non-canonical encodings: **six**. They are y = p and y = p + 1 (the only small-order y below 19, so the only ones with a second 255-bit encoding), each with either sign bit, plus the identity and the order-2 point with the sign bit set (x = 0, x_0 = 1). `ge_decompress` refuses all six today, so `verify` and `verify_strict` both answer false on every row that carries one.

## P2. The rows

- **As A**, with R = the identity and S = 0, over the empty message and one other. `[S]B` is the identity, so `verify` accepts exactly when `[k]A` is the identity, that is, when ord(A) divides k mod L. It accepts the two identity rows for certain, and each other row with probability 1/ord(A). The generator computes k for every row and predicts `verify`'s exact column. `verify_strict` refuses all 16.
- **As R**, under RFC 8032 §7.1 TEST 1's key, with S = k·a mod L (a is the key's clamped scalar). `verify` accepts only the two identity rows: for any other small-order R, R + [k]A carries a torsion component that [S]B cannot. `verify_strict` refuses all 16.
- **Non-canonical**, the six encodings as A and as R: both functions answer false on all 12 rows.

## P3. Wycheproof ed25519 (151 cases)

- **No case changes verdict** under `verify_strict`: 0 of 151. A scan of the vendored JSON for the encodings above finds no small-order A in any of the 52 keys and no small-order R in any `valid` case.
- Eleven `invalid` cases carry a small-order R: tcId 10–14 and 60 (`00…00`) and tcId 15–19 (the identity). They stay invalid, and `verify_strict` refuses them on two grounds.
- So the expected strict verdict equals Wycheproof's `result` for all 151: 88 valid, 63 invalid. The generator decides each verdict from the flags and from its own decoding, never from std.

## P4. The cost

- `verify` is two 256-step double-and-add scalar multiplications (about 2 × (256 doublings × 8 field multiplications + ~126 additions × 9)), two decompressions (~270 multiplications each) and one SHA-512: about 6,900 field multiplications plus the hash.
- The small-order check adds three doublings and one `ge_eq` per point, about 56 multiplications. Predicted added cost: **+0.5% to +1.5%** of `verify`, under 3% measured, which is inside run-to-run noise.
- The mixed-order check `[L]P` (R-sc77-2, measured, not built) costs one more 256-step scalar multiplication per point; L has 73 set bits, so about 2,700 multiplications. Predicted: **+35% to +45% per point**, **+70% to +90%** for both A and R.

## P5. What does not change

- `verify`'s body stays byte-for-byte; only its doc comment changes. `sign`, `public`, X25519 and every caller are untouched. Every existing row keeps its ledger word: RFC 8032 §7.1, the Wycheproof ed25519 (151) and x25519 (518) sets, `bad_ed25519_params.hex`, and the jose, tls/cert and tls/client suites.
- The new strict refusal rows short-circuit before any scalar multiplication, so they are predicted to run on all three lanes. The rows that pin `verify`'s accepting column, and the full strict Wycheproof parts, are predicted native-only, as the existing verify parts are (the step budget).
- No toolchain pin moves: wolf 0.2.36, lupin 0.1.58.

## P6. The plant

- Dropping the R check from `verify_strict` turns exactly the two identity-R rows red: those are the only R rows that `verify` accepts. No Wycheproof row moves.
