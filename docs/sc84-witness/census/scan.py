#!/usr/bin/env python3
"""sc84 census scanner: per function, integer divisions and conditional
branches in otool -tvV (arm64 Mach-O) or objdump -d (x86-64 ELF) text.

Usage: scan.py ASM... [--divs-only]
Prints one line per function with divisions, then a per-function table of
conditional-branch and select counts for the crypto modules.
"""
import re
import sys

DIV_X86 = re.compile(r"^i?div[bwlq]?$")
DIV_ARM = re.compile(r"^[su]div$")
CB_ARM = re.compile(r"^(b\.[a-z]{2}|cbn?z|tbn?z)$")
SEL_ARM = re.compile(r"^(csel|csinc|csinv|csneg|cset|csetm|cinc|cneg)$")
CB_X86 = re.compile(r"^j(?!mp)[a-z]{1,3}$")
SEL_X86 = re.compile(r"^(cmov[a-z]{1,3}|set[a-z]{1,3})$")


def demangle(name):
    name = name.lstrip("_")
    name = re.sub(r"\$[0-9a-f]{16}$", "", name)
    if name.startswith("W"):
        name = name[1:]
    return name


def functions(path):
    """Yield (function, [(offset, mnemonic, operands)]) from one listing."""
    cur, body = None, []
    objdump_label = re.compile(r"^[0-9a-f]+ <(.+)>:$")
    otool_label = re.compile(r"^([^\s].*):$")
    objdump_insn = re.compile(r"^\s+([0-9a-f]+):\s+(\S+)\s*(.*)$")
    otool_insn = re.compile(r"^([0-9a-f]{8,16})\s+(\S+)\s*(.*)$")
    for line in open(path, errors="replace"):
        line = line.rstrip("\n")
        m = objdump_label.match(line)
        if m is None and not line.startswith(("(", "/")) and "section" not in line:
            m2 = otool_label.match(line)
            if m2 and not otool_insn.match(line):
                m = m2
        if m:
            if cur is not None:
                yield cur, body
            cur, body = m.group(1), []
            continue
        if cur is None:
            continue
        mi = objdump_insn.match(line) or otool_insn.match(line)
        if mi:
            off, mnem, ops = mi.groups()
            # objdump may print a prefix (e.g. "rex", "lock"); keep the opcode
            parts = (mnem + " " + ops).split()
            if parts and parts[0] in ("rex", "rex.W", "lock", "data16", "cs", "ds"):
                parts = parts[1:]
            mnem = parts[0] if parts else mnem
            ops = " ".join(parts[1:])
            body.append((int(off, 16), mnem, ops.split("#")[0].split("<")[0].strip()))
    if cur is not None:
        yield cur, body


def main():
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    divs_only = "--divs-only" in sys.argv
    table = []
    for path in args:
        for fn, body in functions(path):
            name = demangle(fn)
            divs = [(o, m, ops) for o, m, ops in body if DIV_X86.match(m) or DIV_ARM.match(m)]
            cbs = sum(1 for _, m, _ in body if CB_ARM.match(m) or CB_X86.match(m))
            sels = sum(1 for _, m, _ in body if SEL_ARM.match(m) or SEL_X86.match(m))
            obj = path.rsplit("/", 1)[-1].replace(".asm", "")
            for o, m, ops in divs:
                print(f"DIV {obj} {name} +0x{o:x} {m} {ops}")
            table.append((name, obj, len(body), len(divs), cbs, sels))
    if not divs_only:
        print()
        print(f"{'function':60} {'object':28} {'insns':>6} {'divs':>5} {'jcc':>5} {'sel':>5}")
        for row in sorted(table):
            print(f"{row[0]:60} {row[1]:28} {row[2]:6} {row[3]:5} {row[4]:5} {row[5]:5}")


if __name__ == "__main__":
    main()
