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
