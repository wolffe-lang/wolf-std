#!/usr/bin/env python3
"""rsa_check.py -- an independent RSASSA verifier, written from RFC 8017 alone.

sc79's checker for the vendored Wycheproof RSA files. It shares no code
with std.x.crypto.rsa and no library's RSA: the primitives are RFC 8017's
own (I2OSP and OS2IP, section 4; RSAVP1, section 5.2.2; EMSA-PKCS1-v1_5,
section 9.2; EMSA-PSS and MGF1, section 9.1 and appendix B.2.1), the
DigestInfo is built here from its ASN.1 (appendix A.2.4) and X.690's DER
rules, and Python's hashlib is the one call with cryptographic meaning,
used as the SHA-2 oracle.

The key policy is the one std.x.crypto.rsa states: a modulus of 2048 to
4096 bits, odd; an exponent that is odd, at least 3, below n and below
2^256 (RFC 8017 section 3.1, with FIPS 186-5's upper bound).

For every case of every vendored file it computes a verdict and compares
it with the expectation the generator (`cargo xtask gen-vectors`) writes
into the wolf rows: `valid` accepts, `invalid` rejects, and `acceptable`
is decided by flag, where `MissingNull` rejects. A disagreement is an
error naming the file and tcId. `--list` prints the cases whose verdict
under this policy differs from a plain reading of the Wycheproof result,
by file and tcId.

    python3 -I tests/x/crypto/rsa/rsa_check.py [--list] [--vectors DIR]
"""

import argparse
import hashlib
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]

PKCS1_FILES = [
    f"rsa_signature_{bits}_{h}_test.json"
    for bits in (2048, 3072, 4096)
    for h in ("sha256", "sha384", "sha512")
]
PSS_FILES = [
    "rsa_pss_2048_sha256_mgf1_0_test.json",
    "rsa_pss_2048_sha256_mgf1_32_test.json",
    "rsa_pss_2048_sha384_mgf1_48_test.json",
    "rsa_pss_3072_sha256_mgf1_32_test.json",
    "rsa_pss_4096_sha256_mgf1_32_test.json",
    "rsa_pss_4096_sha384_mgf1_48_test.json",
    "rsa_pss_4096_sha512_mgf1_32_test.json",
    "rsa_pss_4096_sha512_mgf1_64_test.json",
]

# The hash functions of the ladder: hashlib's constructor and the OID arc
# of NIST's hash algorithms (2.16.840.1.101.3.4.2.n).
HASHES = {
    "SHA-256": (hashlib.sha256, "2.16.840.1.101.3.4.2.1"),
    "SHA-384": (hashlib.sha384, "2.16.840.1.101.3.4.2.2"),
    "SHA-512": (hashlib.sha512, "2.16.840.1.101.3.4.2.3"),
}


# ---- X.690 DER, the little this checker needs ----

def der_len(n):
    if n < 128:
        return bytes([n])
    body = n.to_bytes((n.bit_length() + 7) // 8, "big")
    return bytes([0x80 | len(body)]) + body


def der_tlv(tag, content):
    return bytes([tag]) + der_len(len(content)) + content


def der_oid(dotted):
    arcs = [int(a) for a in dotted.split(".")]
    out = bytearray()
    for arc in [arcs[0] * 40 + arcs[1]] + arcs[2:]:
        chunk = [arc & 0x7F]
        arc >>= 7
        while arc:
            chunk.append(0x80 | (arc & 0x7F))
            arc >>= 7
        out += bytes(reversed(chunk))
    return der_tlv(0x06, bytes(out))


def digest_info(hname, h):
    """DigestInfo ::= SEQUENCE { digestAlgorithm AlgorithmIdentifier,
    digest OCTET STRING }, the algorithm with NULL parameters
    (RFC 8017 appendix A.2.4 and section 9.2 Note 1)."""
    alg = der_tlv(0x30, der_oid(HASHES[hname][1]) + der_tlv(0x05, b""))
    return der_tlv(0x30, alg + der_tlv(0x04, h))


def parse_rsa_public_key(der):
    """RSAPublicKey ::= SEQUENCE { modulus INTEGER, publicExponent
    INTEGER } (appendix A.1.1), strictly: definite minimal lengths,
    minimal non-negative INTEGERs, no trailing bytes."""
    def tlv(buf, at):
        if at + 2 > len(buf):
            raise ValueError("truncated")
        tag, first = buf[at], buf[at + 1]
        at += 2
        if first < 0x80:
            n = first
        else:
            k = first & 0x7F
            if k == 0 or k > 4 or at + k > len(buf) or buf[at] == 0:
                raise ValueError("bad length")
            n = int.from_bytes(buf[at:at + k], "big")
            if n < 128:
                raise ValueError("non-minimal length")
            at += k
        if at + n > len(buf):
            raise ValueError("overrun")
        return tag, buf[at:at + n], at + n

    tag, body, end = tlv(der, 0)
    if tag != 0x30 or end != len(der):
        raise ValueError("not one SEQUENCE")
    ints = []
    at = 0
    while at < len(body):
        tag, v, at = tlv(body, at)
        if tag != 0x02 or not v:
            raise ValueError("not an INTEGER")
        if v[0] & 0x80:
            raise ValueError("negative")
        if len(v) > 1 and v[0] == 0 and not (v[1] & 0x80):
            raise ValueError("non-minimal INTEGER")
        ints.append(int.from_bytes(v, "big"))
    if len(ints) != 2:
        raise ValueError("not two INTEGERs")
    return ints[0], ints[1]


# ---- RFC 8017 ----

def i2osp(x, length):
    if x >= 256 ** length:
        raise ValueError("integer too large")
    return x.to_bytes(length, "big")


def os2ip(octets):
    return int.from_bytes(octets, "big")


def key_refusal(n, e):
    """The module's key policy, or None when the key is accepted."""
    bits = n.bit_length()
    if bits < 2048:
        return "small_modulus"
    if bits > 4096:
        return "large_modulus"
    if n % 2 == 0:
        return "bad_modulus"
    if e % 2 == 0 or e < 3 or e >= n or e >= 2 ** 256:
        return "bad_exponent"
    return None


def rsavp1(n, e, s):
    if not 0 <= s < n:
        raise ValueError("signature representative out of range")
    return pow(s, e, n)


def pkcs1_v15_verify(n, e, hname, message, signature):
    """RSASSA-PKCS1-V1_5-VERIFY (section 8.2.2)."""
    k = (n.bit_length() + 7) // 8
    if len(signature) != k:
        return False
    try:
        m = rsavp1(n, e, os2ip(signature))
        em = i2osp(m, k)
    except ValueError:
        return False
    t = digest_info(hname, HASHES[hname][0](message).digest())
    if k < len(t) + 11:
        return False
    expected = b"\x00\x01" + b"\xff" * (k - len(t) - 3) + b"\x00" + t
    return em == expected


def mgf1(hname, seed, mask_len):
    """MGF1 (appendix B.2.1)."""
    hf = HASHES[hname][0]
    out = b""
    counter = 0
    while len(out) < mask_len:
        out += hf(seed + i2osp(counter, 4)).digest()
        counter += 1
    return out[:mask_len]


def emsa_pss_verify(hname, message, em, em_bits, s_len):
    """EMSA-PSS-VERIFY (section 9.1.2), MGF1 over the same hash."""
    hf = HASHES[hname][0]
    m_hash = hf(message).digest()
    h_len = len(m_hash)
    em_len = len(em)
    if em_len < h_len + s_len + 2:
        return False
    if em[-1] != 0xBC:
        return False
    masked_db = em[: em_len - h_len - 1]
    h = em[em_len - h_len - 1 : em_len - 1]
    top_bits = 8 * em_len - em_bits
    if top_bits and masked_db[0] >> (8 - top_bits):
        return False
    db_mask = mgf1(hname, h, em_len - h_len - 1)
    db = bytes(a ^ b for a, b in zip(masked_db, db_mask))
    db = bytes([db[0] & (0xFF >> top_bits)]) + db[1:]
    zeros = em_len - h_len - s_len - 2
    if any(db[:zeros]) or db[zeros] != 0x01:
        return False
    salt = db[len(db) - s_len :] if s_len else b""
    return hf(b"\x00" * 8 + m_hash + salt).digest() == h


def pss_verify(n, e, hname, message, signature, s_len):
    """RSASSA-PSS-VERIFY (section 8.1.2)."""
    mod_bits = n.bit_length()
    k = (mod_bits + 7) // 8
    if len(signature) != k:
        return False
    em_bits = mod_bits - 1
    em_len = (em_bits + 7) // 8
    try:
        m = rsavp1(n, e, os2ip(signature))
        em = i2osp(m, em_len)
    except ValueError:
        return False
    return emsa_pss_verify(hname, message, em, em_bits, s_len)


# ---- the vendored files ----

def expected(test, what):
    """The generator's rule, restated."""
    result = test["result"]
    if result == "valid":
        return True
    if result == "invalid":
        return False
    if result == "acceptable" and test["flags"] == ["MissingNull"]:
        return False
    raise SystemExit(f"{what}: tcId {test['tcId']} is `{result}` {test['flags']}: no rule")


def check_file(path, pss):
    data = json.loads(path.read_text())
    rows = []
    for group in data["testGroups"]:
        hname = group["sha"]
        if pss and (group["mgf"] != "MGF1" or group["mgfSha"] != hname):
            raise SystemExit(f"{path.name}: a group outside MGF1-over-the-message-hash")
        n, e = parse_rsa_public_key(bytes.fromhex(group["publicKeyAsn"]))
        pk = group["publicKey"]
        if n != int(pk["modulus"], 16) or e != int(pk["publicExponent"], 16):
            raise SystemExit(f"{path.name}: publicKeyAsn disagrees with publicKey")
        refusal = key_refusal(n, e)
        if refusal:
            raise SystemExit(f"{path.name}: a vendored key is refused ({refusal})")
        for t in group["tests"]:
            msg = bytes.fromhex(t["msg"])
            sig = bytes.fromhex(t["sig"])
            if pss:
                got = pss_verify(n, e, hname, msg, sig, group["sLen"])
            else:
                got = pkcs1_v15_verify(n, e, hname, msg, sig)
            want = expected(t, path.name)
            if got != want:
                raise SystemExit(
                    f"{path.name}: tcId {t['tcId']} computes {got}, the rows expect {want} "
                    f"({t['result']} {t['flags']})"
                )
            rows.append((t["tcId"], t["result"], t["flags"], got, e))
    return rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--vectors", default=str(ROOT / "vendor/vectors/wycheproof"))
    args = ap.parse_args()
    vec = pathlib.Path(args.vectors)
    total = {"cases": 0, "accept": 0, "reject": 0}
    differs = []
    small_e = []
    for name in PKCS1_FILES + PSS_FILES:
        rows = check_file(vec / name, name in PSS_FILES)
        acc = sum(1 for r in rows if r[3])
        print(f"ok   {name}: {len(rows)} cases, {acc} accept, {len(rows) - acc} reject")
        total["cases"] += len(rows)
        total["accept"] += acc
        total["reject"] += len(rows) - acc
        for tc, result, flags, got, e in rows:
            if (result != "invalid") != got:
                differs.append(f"{name} tcId {tc}: {result} {flags} -> {'accept' if got else 'reject'}")
            if e < 2 ** 16 and got:
                small_e.append(f"{name} tcId {tc}: e = {e}, {result} {flags} -> accept")
    print(f"ok   RSA: {total['cases']} cases, {total['accept']} accept, {total['reject']} reject, "
          f"{len(differs)} decided against a plain reading of the result")
    if args.list:
        for line in differs:
            print("  differs: " + line)
        for line in small_e:
            print("  small e: " + line)
    return 0


if __name__ == "__main__":
    sys.exit(main())
