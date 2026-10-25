# TZif fixtures

Copied byte-for-byte (symlinks dereferenced) from this Mac’s
`/usr/share/zoneinfo` on 2026-10-22. IANA timezone data is public domain.
No libc, tzcode or coreutils source was read. RFC 8536 is the format.

| Zone | Bytes |
|---|---:|
| America/New_York | 3552 |
| Europe/London | 3664 |
| Asia/Kolkata | 285 |
| Australia/Lord_Howe | 1846 |
| UTC | 114 |

The parser reads the v2/v3 64-bit block and POSIX footer. Tests also
construct a small v1 file and corrupt its counts, flags and indexes.

## Leap-aware fixtures (sc74)

`right/{UTC,America/New_York,Europe/London}` are byte-identical copies
from kasumi's system tzdata `2026e-1.1` (2026-10-25), public-domain data.
SHA256, in that order:

- `ba3f58c59f1dd7b807fb3fe09c4dbabac2465307d7f5044c18cdd7f721c44fcb`
- `7bb140f56dad3d2bb3a13a1de9fa49dbd99cda666eb71e78cb125da17e1dd672`
- `d64800146d1579a6df7d5f7abc1163d7cc299030e8b28abc1886b873db8b2fce`

`right/glibc.tsv` contains 264 black-box `date` rows. `right/oracle.py`
regenerates them using content-addressed Ubuntu 24.04 image
`69f919497cada9a8611f3de61df902d9f94bf20f84552ef5bd93125c38608ef9`
(repository digest `sha256:f610ab94648195aa356059f5b41d6085c9d4d903c072430cdd1af7bdb646106b`),
glibc 2.39-0ubuntu8.9 and GNU date 9.4. The copied system database is
mounted read-only; Ubuntu's minimal image contains no tzdata package.
Each of three zones covers pre-1972, all 27 occurrences +/-1 second,
1783000020, and the 2024 New York spring transition +/-1 second.
The input follows UNIX leap time for these files, not an implicit
conversion from POSIX time. The standard library bundles no database.

`leap-synthetic/generate.py` constructs small v1/v3 files directly from
RFC 8536 section 3: negative leaps, an occurrence after 2038, a positive
leap with a 30-second offset, and a leap-aware recurring POSIX footer.
No implementation source was read.
