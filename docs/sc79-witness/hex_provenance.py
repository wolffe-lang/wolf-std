#!/usr/bin/env python3
"""hex_provenance.py -- every long hex literal in sc79's hand rows is vendored.

    python3 -I docs/sc79-witness/hex_provenance.py

The hand rows quote certificates, keys and signatures from
`vendor/vectors/pki/sc79/` as hex literals. This check reads each row
file and accounts for every hex literal of 40 or more digits, one of:

- `exact`: the hex of a vendored file (sc79's DER and signatures, or an
  sc22 `.hex` file of `vendor/vectors/pki/`);
- `flipped`: a vendored file with its last octet's low bit flipped (the
  bad-signature rows);
- `framed`: a mutation of the vendored 2049-bit RSAPublicKey that keeps
  its body or its modulus whole, or changes exactly one octet (the
  key-policy rows' re-framed and negated keys);
- `small`: octets that are all 0x00 or 0x01 (the synthetic r and s
  values of the early-refusal rows);
- `curve`: the published P-384 constants (SP 800-186) the early-refusal
  rows build their points and signatures from.

Anything else is a `FAIL` line naming the file, so a row cannot quietly
carry bytes no vendored source holds.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
PKI = ROOT / "vendor/vectors/pki/sc79"
ROWS = [
    "tests/x/crypto/rsa/key_policy_rows.lu",
    "tests/x/crypto/rsa/rsa2049_rows.lu",
    "tests/x/crypto/rsa/verify_reject_early.lu",
    "tests/x/crypto/rsa/hash_selector_trap.lu",
    "tests/x/crypto/p384/verify_reject_early.lu",
    "tests/x/tls/cert/rsa_chain_valid.lu",
    "tests/x/tls/cert/p384_chain_valid.lu",
    "tests/x/tls/cert/root_set_rows.lu",
    "tests/x/tls/cert/public_pki_refusals.lu",
    "tests/x/tls/cert/unsupported_alg_row.lu",
    "tests/x/tls/client/cert_verify_schemes.lu",
]
CURVE = [
    "fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeffffffff0000000000000000ffffffff",
    "ffffffffffffffffffffffffffffffffffffffffffffffffc7634d81f4372ddf581a0db248b0a77aecec196accc52973",
    "aa87ca22be8b05378eb1c71ef320ad746e1d3b628ba79b9859f741e082542a385502f25dbf55296c3a545e3872760ab7",
    "3617de4a96262c6f5d9e98bf9292dc29f8f41dbd289a147ce9da3113b5f0b8c00a60b1ce1d7e819d7a431d7c90ea0e5f",
]


def main():
    vendored = {p.name: p.read_bytes().hex() for p in PKI.iterdir() if p.suffix in (".der", ".sig")}
    for p in (ROOT / "vendor/vectors/pki").glob("*.hex"):
        vendored[p.name] = p.read_text().strip()
    flipped = {h[:-1] + "%x" % (int(h[-1], 16) ^ 1): n for n, h in vendored.items()}
    pub = vendored["rsa2049.pub.der"]
    body = pub[8:]  # the RSAPublicKey's content: both INTEGERs
    modulus = pub[16:16 + 2 * 257]
    bad = 0
    counts = {"exact": 0, "flipped": 0, "framed": 0, "small": 0, "curve": 0}

    def one_octet(h):
        if len(h) != len(pub):
            return False
        return sum(1 for i in range(0, len(h), 2) if h[i:i + 2] != pub[i:i + 2]) == 1

    for rel in ROWS:
        text = (ROOT / rel).read_text()
        for h in re.findall(r'"([0-9a-f]{40,})"', text):
            if h in vendored.values():
                counts["exact"] += 1
            elif h in flipped:
                counts["flipped"] += 1
            elif (body in h or modulus in h or one_octet(h)) and "key_policy" in rel:
                counts["framed"] += 1
            elif set(h[i:i + 2] for i in range(0, len(h), 2)) <= {"00", "01"}:
                counts["small"] += 1
            elif any(c in h for c in CURVE) and "p384" in rel:
                counts["curve"] += 1
            else:
                print(f"FAIL hex-provenance {rel}: a literal of {len(h) // 2} octets no vendored file accounts for")
                bad += 1
    print("ok   hex provenance: " + ", ".join(f"{k} {v}" for k, v in counts.items()))
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
