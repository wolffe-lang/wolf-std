# vendor/vectors/pki — the sc22 Ed25519 test PKI (pinned DER)

The certificate corpus `std.x.tls.cert`'s tests quote (hand-written
tests under `tests/x/tls/cert/`, hex quoted verbatim from these files).
Unlike every earlier corpus in `vendor/vectors/` there is no public
X.509 vector suite shaped for an Ed25519-scoped validator, so this one
is AUTHORED — by `gen_pki.py`, a from-scratch reference (RFC 8032
Ed25519 + a hand-built X.690 DER encoder, no library) — and
CROSS-CHECKED with the independent known tool, openssl 3.6.3:

- `openssl verify -CAfile root.pem leaf.pem` answers OK (the chain,
  including the Ed25519 signatures, validated by the independent
  implementation), and
- `openssl x509 -text` re-parses every authored field (version v3,
  serial 0x1337, the DNs, the validity window, the Ed25519 SPKI, the
  BC/KU/SAN extensions) exactly as written.

So a builder bug and a `std.x.tls.cert` parser bug cannot agree by
construction (the sc17/sc20 extractor+reference discipline, inverted:
here the reference AUTHORS and the tool checks).

Ed25519 keys are DETERMINISTIC (seeds are SHA-256 of fixed strings
printed by the script), so re-running `gen_pki.py` reproduces every
Ed25519 certificate byte-for-byte. The script is provenance, not rig
machinery: CI never runs it (no build scripts, D33); the committed
`.hex` files are the corpus, exactly as the RFC texts are. The two
non-Ed25519 certificates are openssl-generated (see below) and pinned
as vendored bytes (RSA/EC keygen is not deterministic).

Each `.hex` file is one DER certificate, lowercase hex, one line.
`root.pem`/`leaf.pem` are PEM copies of the same bytes for the openssl
cross-check.

## The chain and its negatives

- `root.hex` — self-signed Ed25519 CA, CN=wolf-std sc22 test root
  (UTF8String), serial 1, UTCTime validity 2025-01-01..2035-12-31,
  basicConstraints CRITICAL cA TRUE pathLen 1, keyUsage CRITICAL
  keyCertSign|cRLSign, plus subjectKeyIdentifier as an unrecognized
  NON-critical extension (the carry-and-ignore witness).
- `leaf.hex` — signed by root, CN=wolf.example (PrintableString),
  serial 0x1337, UTCTime validity 2026-01-01..2027-01-01, SAN
  dNSName wolf.example + www.wolf.example, basicConstraints
  non-critical cA FALSE (DER: empty SEQUENCE), keyUsage
  digitalSignature, plus an unknown non-critical extension
  (1.3.6.1.4.1.55555.2).
- `other_root.hex` — a SECOND self-signed Ed25519 CA (CN=wolf-std sc22
  other root): the wrong-trust-anchor negative (`issuer_mismatch`).
- `rogue_nonca.hex` — signed by the LEAF's key with issuer =
  the leaf's subject: the not-a-CA-signing negative (`not_ca`; chain =
  rogue || leaf against the root anchor).
- `critext.hex` — signed by root, carries a CRITICAL unrecognized
  extension (1.3.6.1.4.1.55555.1): parses (structure is sound),
  validation rejects `unknown_critical` (RFC 5280 §4.2's MUST).

Expired / not-yet-valid / wrong-name / bad-signature need NO extra
certificates: validity is judged against a SUPPLIED `now`, the
hostname against a supplied target, and the bad-signature witness
flips one byte of `leaf.hex`'s signature at runtime.

## The malformed-encoding conformance set (`bad_*.hex`)

Each violates exactly ONE X.690/RFC 5280 clause (the clause is in
`gen_pki.py` beside the variant and in the test file that pins it);
every one must be a clean named `der` rejection from
`std.x.tls.cert.parse` — no crash, no trap, no out-of-bounds read:

| file | clause |
|---|---|
| `bad_gentime_pre2050.hex` | RFC 5280 §4.1.2.5 (dates through 2049 MUST be UTCTime) |
| `bad_utctime_no_seconds.hex` | RFC 5280 §4.1.2.5.1 (seconds mandatory) |
| `bad_time_month_13.hex` | §4.1.2.5 profile of ISO 8601 (month range) |
| `bad_version_v2.hex` | RFC 5280 §4.1.2.1 (v2 rejected by this profile) |
| `bad_negative_serial.hex` | RFC 5280 §4.1.2.2 (serial MUST be positive) |
| `bad_nonminimal_serial.hex` | X.690 §8.3.2 (INTEGER minimality) |
| `bad_nonminimal_length.hex` | X.690 §10.1 (shortest length form mandatory) |
| `bad_boolean_01.hex` | X.690 §11.1 (DER TRUE is 0xFF) |
| `bad_critical_false_encoded.hex` | X.690 §11.5 (DEFAULT encoded by absence) |
| `bad_alg_mismatch.hex` | RFC 5280 §4.1.2.3 (TBS signature == outer alg) |
| `bad_ed25519_params.hex` | RFC 8410 §3 (Ed25519 has NO parameters) |
| `bad_sig_unused_bits.hex` | RFC 5280 §4.1.1.3 (signature BIT STRING whole octets) |
| `bad_duplicate_ext.hex` | RFC 5280 §4.2 (at most one instance per extension) |
| `bad_empty_san.hex` | RFC 5280 §4.2.1.6 (SAN MUST contain >= 1 name) |
| `bad_unsorted_set.hex` | X.690 §11.6 (SET OF sorted by encoding) |
| `bad_unique_id.hex` | RFC 5280 §4.1.2.8 (unique IDs MUST NOT be generated) |
| `bad_oid_padded_arc.hex` | X.690 §8.19.2 (no 0x80 padding octet) |
| `bad_pathlen_without_ca.hex` | RFC 5280 §4.2.1.9 (pathLen requires cA) |

The runtime-mutation classes (truncation, indefinite length, a length
past the buffer, trailing garbage, a flipped signature byte) are
exercised in the tests directly on `leaf.hex`'s pinned bytes — the
outer TLV header's offsets are stable by construction.

## The parse-but-named-unsupported pair (openssl-generated)

- `rsa.hex` — self-signed RSA-2048 / sha256WithRSAEncryption,
  CN=rsa.wolf.example, SAN dns rsa.wolf.example.
- `ecdsa.hex` — self-signed ECDSA P-256 / ecdsa-with-SHA256,
  CN=ecdsa.wolf.example, SAN dns ecdsa.wolf.example.

Generated with openssl 3.6.3 (`openssl req -x509 -newkey rsa:2048` /
`-newkey ec -pkeyopt ec_paramgen_curve:P-256`, `-days 365`, SAN via
`-addext`), 2026-08-27, keys discarded. They stand in for the
real-world corpus (this environment builds offline; the structures —
NULL RSA parameters, named-curve EC parameters, AKI/SKI/BC extensions,
multi-byte lengths in the RSA modulus — are exactly the public-CA
shapes). Both must PARSE structurally in `std.x.tls.cert` and answer
validation with the NAMED `unsupported_alg` rejection, never a silent
accept — the sprint's load-bearing honesty split.

sha256 of the vendored files, as authored/retrieved 2026-08-27, are in
`sha256sums.txt` (regenerate with `sha256sum *.hex`).
