#!/usr/bin/env python3
"""Narrow source obligations, not a constant-time verifier or an AST parser."""
import re
import sys
from pathlib import Path

root = Path(sys.argv[1]).resolve()

def body(path, name):
    text = (root / path).read_text()
    match = re.search(r"\bfn " + re.escape(name) + r"\s*\(", text)
    if not match:
        return "MISSING"
    # Selected signatures have at most one error-row brace before the body.
    start = text.index("{", match.end())
    if text[start - 2:start] == "! ":
        start = text.index("{", text.index("}", start) + 1)
    depth = 1
    pos = start + 1
    while depth:
        depth += (text[pos] == "{") - (text[pos] == "}")
        pos += 1
    return re.sub(r"//[^\n]*", "", text[start + 1:pos - 1])

failures = []
def check(name, ok):
    print(("PASS " if ok else "FAIL ") + name)
    if not ok:
        failures.append(name)

zero = body("std/x/crypto/curve25519/curve25519.lu", "is_zero_bytes")
check("x25519-zero-source-fold", "MISSING" not in zero and not re.search(r"\b(if|return|break)\b", zero) and "|" in zero)
curve = (root / "std/x/crypto/curve25519/curve25519.lu").read_text()
check("x25519-shared-secret-doc", "test is on the PUBLIC output" not in curve and "public output). ct: on public data" not in curve)
check("x25519-rendering-doc", "shared secrets are\n/// public once computed" not in curve)
sha = (root / "std/x/crypto/sha2/sha2.lu").read_text()
check("hkdf-hashlen-doc", "prk` must be at least one" not in sha and "HashLen" in sha)
check("sha2-source-contract-doc", "not a fixed-time certification" in sha)
fold = body("std/bytes/bytes.lu", "is_equal_fixed_work")
loop = fold[fold.find("while"):]
check("equality-source-fold", "MISSING" not in fold and "while" in fold and not re.search(r"\b(if|return|break)\b", loop) and "^" in loop and "|" in loop)
opened = body("std/x/crypto/chacha20/chacha20.lu", "open")
check("aead-selected-comparison", "bytes.is_equal_fixed_work(want, got)" in opened)
check("aead-auth-before-decrypt", opened.count("stream_xor(") == 1 and opened.rfind("return tag") < opened.index("stream_xor(") and "stream_xor(" not in opened[:opened.rfind("return tag")])
sys.exit(bool(failures))
