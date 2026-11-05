#!/usr/bin/env python3
"""run.py -- sc79's interop rows: std.x.tls.client against real peers.

    python3 -I docs/sc79-witness/interop/run.py --wolf W --std-root STD
        --scratch DIR [--local] [--github] [--want-red]

`--local`: for each row, an OpenSSL `s_server` (the pinned binary of
`vendor/vectors/pki/sc79/openssl.txt`, used as a black-box peer and
checked by digest first) serves one of the sc79 chains on a loopback
port, held to one TLS 1.3 SignatureScheme, and the probe in `client/`
(built with `wolf build` against STD) completes a handshake and one
HTTP/1.0 exchange. The probe is given that chain's root alone, the time
2027-06-01T00:00:00Z and fresh random bytes.

`--github`: one handshake to github.com:443 with the roots of the
distribution's bundle (`/etc/ssl/certs/ca-certificates.crt`, each PEM
block converted to DER and concatenated) supplied explicitly, and the
current UTC time. Exactly one outbound connection, made by the probe.
When github.com does not resolve, the row is a SKIP (exit 77) and no
connection is attempted.

Each row prints `ok   <row>: <the probe's line>` or, at column 0,
`FAIL <row>: <the probe's line>`. Exit 0 when every row passes; 1 when
one fails. `--want-red` inverts the local rows' expectation (every row
but the Ed25519 control must FAIL), for the record of trunk.
"""

import argparse
import datetime
import hashlib
import os
import pathlib
import re
import shutil
import socket
import ssl
import subprocess
import sys
import time

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[2]
PKI = ROOT / "vendor/vectors/pki/sc79"

# row, leaf, chain (or None), root, the server's one SignatureScheme
LOCAL = [
    ("interop-rsa-pss-sha256", "rsa_leaf", "rsa_int", "rsa_root", "rsa_pss_rsae_sha256"),
    ("interop-rsa-pss-sha384", "rsa_leaf", "rsa_int", "rsa_root", "rsa_pss_rsae_sha384"),
    ("interop-rsa-pss-sha512", "rsa_leaf", "rsa_int", "rsa_root", "rsa_pss_rsae_sha512"),
    ("interop-rsa-pss-certificates", "pss_leaf", "pss_int", "rsa_root", "rsa_pss_rsae_sha256"),
    ("interop-p384-chain", "p384_leaf", "p384_int", "p384_root", "ecdsa_secp384r1_sha384"),
    ("interop-p256-under-p384", "p256_leaf", "p384_int", "p384_root", "ecdsa_secp256r1_sha256"),
    ("interop-ed25519-control", "ed_leaf", None, "ed_root", "ed25519"),
]
CONTROL = "interop-ed25519-control"


def pinned_openssl():
    """The OpenSSL binary on PATH, checked against the recorded digest."""
    path = shutil.which("openssl")
    if path is None:
        return None, "no openssl on PATH"
    real = os.path.realpath(path)
    got = hashlib.sha256(pathlib.Path(real).read_bytes()).hexdigest()
    want = (PKI / "openssl.txt").read_text().splitlines()[1].split()[0]
    if got != want:
        return None, f"openssl at {real} is {got[:16]}…, the pin is {want[:16]}…"
    return path, subprocess.run([path, "version"], capture_output=True, text=True).stdout.strip()


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def build_probe(wolf, std_root, scratch):
    src = scratch / "probe"
    shutil.rmtree(src, ignore_errors=True)
    src.mkdir(parents=True)
    shutil.copy(HERE / "client/client.lu", src / "client.lu")
    out = scratch / "probe-bin"
    p = subprocess.run([wolf, "build", "--std-root", str(std_root), str(src / "client.lu"), "-o", str(out)],
                       capture_output=True, text=True)
    if p.returncode != 0:
        print("FAIL interop-probe-builds: " + (p.stdout + p.stderr).strip().replace("\n", " | ")[:600])
        sys.exit(1)
    return out


def run_probe(probe, scratch, addr, host, roots, now):
    (scratch / "random.bin").write_bytes(os.urandom(32))
    (scratch / "scalar.bin").write_bytes(os.urandom(32))
    env = dict(os.environ, SC79_ADDR=addr, SC79_HOST=host, SC79_ROOTS=str(roots), SC79_NOW=now,
               SC79_RANDOM=str(scratch / "random.bin"), SC79_SCALAR=str(scratch / "scalar.bin"))
    start = time.monotonic()
    try:
        p = subprocess.run([str(probe)], capture_output=True, text=True, env=env, timeout=60)
        line = (p.stdout + p.stderr).strip().replace("\n", " | ") or f"exit {p.returncode}"
        rc = p.returncode
    except subprocess.TimeoutExpired:
        line, rc = "timeout after 60 s", 124
    return rc, line, time.monotonic() - start


def local_rows(probe, scratch, openssl, want_red):
    bad = 0
    for row, leaf, chain, root, scheme in LOCAL:
        port = free_port()
        cmd = [openssl, "s_server", "-accept", f"127.0.0.1:{port}", "-cert", str(PKI / f"{leaf}.pem"),
               "-key", str(PKI / f"{leaf}.key"), "-tls1_3", "-ciphersuites", "TLS_CHACHA20_POLY1305_SHA256",
               "-groups", "X25519", "-sigalgs", scheme, "-www", "-quiet"]
        if chain:
            cmd += ["-cert_chain", str(PKI / f"{chain}.pem")]
        log = open(scratch / f"{row}.server.log", "wb")
        server = subprocess.Popen(cmd, stdout=log, stderr=log, stdin=subprocess.DEVNULL)
        try:
            for _ in range(200):
                try:
                    with socket.create_connection(("127.0.0.1", port), timeout=0.2):
                        break
                except OSError:
                    time.sleep(0.05)
            rc, line, secs = run_probe(probe, scratch, f"127.0.0.1:{port}", "localhost",
                                       PKI / f"{root}.der", "20270601000000")
        finally:
            server.terminate()
            try:
                server.wait(timeout=5)
            except subprocess.TimeoutExpired:
                server.kill()
                server.wait()
            log.close()
        passed = rc == 0 and line.startswith("complete: localhost application bytes=")
        expect_pass = (not want_red) or row == CONTROL
        if passed == expect_pass:
            print(f"ok   {row}: {line} ({secs:.1f}s)")
        else:
            print(f"FAIL {row}: {line} ({secs:.1f}s)")
            bad += 1
    return bad


def github_row(probe, scratch):
    try:
        socket.getaddrinfo("github.com", 443, proto=socket.IPPROTO_TCP)
    except OSError as e:
        print(f"skip github-handshake: github.com does not resolve ({e})")
        return 77
    bundle = pathlib.Path("/etc/ssl/certs/ca-certificates.crt").read_text()
    pems = re.findall(r"-----BEGIN CERTIFICATE-----.*?-----END CERTIFICATE-----", bundle, re.S)
    roots = scratch / "distro-roots.der"
    roots.write_bytes(b"".join(ssl.PEM_cert_to_DER_cert(p) for p in pems))
    now = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%d%H%M%S")
    print(f"  github: {len(pems)} roots from the distribution bundle, now={now}")
    rc, line, secs = run_probe(probe, scratch, "github.com:443", "github.com", roots, now)
    if rc == 0 and line.startswith("complete: github.com application bytes="):
        print(f"ok   github-handshake: {line} ({secs:.1f}s)")
        return 0
    print(f"FAIL github-handshake: {line} ({secs:.1f}s)")
    return 1


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--wolf", required=True)
    ap.add_argument("--std-root", required=True)
    ap.add_argument("--scratch", required=True)
    ap.add_argument("--local", action="store_true")
    ap.add_argument("--github", action="store_true")
    ap.add_argument("--want-red", action="store_true")
    args = ap.parse_args()
    scratch = pathlib.Path(args.scratch)
    scratch.mkdir(parents=True, exist_ok=True)
    probe = build_probe(args.wolf, pathlib.Path(args.std_root).resolve(), scratch)
    rc = 0
    if args.local:
        openssl, what = pinned_openssl()
        if openssl is None:
            print(f"FAIL interop-openssl-pin: {what}")
            return 1
        print(f"  peer: {what}")
        if local_rows(probe, scratch, openssl, args.want_red):
            rc = 1
    if args.github:
        g = github_row(probe, scratch)
        if g == 77 and rc == 0:
            return 77
        if g == 1:
            rc = 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
