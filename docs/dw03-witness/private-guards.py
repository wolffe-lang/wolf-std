#!/usr/bin/env python3
"""Exercise private guards without exposing a production test-injection API."""
import argparse
import json
from pathlib import Path
import subprocess

p=argparse.ArgumentParser()
p.add_argument('repo',type=Path)
p.add_argument('out',type=Path)
p.add_argument('--wolf',required=True)
a=p.parse_args()
a.out.mkdir(parents=True,exist_ok=True)
curve=(a.repo/'std/x/crypto/curve25519/curve25519.lu').read_text()
start=curve.index('fn is_zero_bytes(')
end=curve.index('\n}',start)+2
zero=curve[start:end]
zero+='''
fn main() -> int {
    var z = List[byte]()
    for _ in 0..32 { (mut z).push(0 as byte) }
    assert(is_zero_bytes(z))
    for i in 0..32 {
        var v = copy z
        v[i] = 1 as byte
        assert(!is_zero_bytes(v))
        v[i] = 255 as byte
        assert(!is_zero_bytes(v))
    }
    0
}
'''
random=(a.repo/'std/os/random/random.lu').read_text()
# Substitute ONLY the platform observation in a test copy, not the OS provider.
# Production has no injection argument, seed, environment switch or test flag.
pax=random.replace('//! member: true','').replace('let platform = os_name()', 'let platform = "pax"')
assert pax != random
pax+='''
fn main() -> int {
    var refused = false
    let out = bytes(32) else |entropy_unqualified| {
        refused = true
        List[byte]()
    }
    assert(refused && out.len == 0)
    0
}
'''
for name,source in [('zero-byte-positions',zero),('pending-pax-refusal',pax)]:
    case=a.out/name
    case.mkdir(exist_ok=True)
    entry=case/'main.lu'
    entry.write_text('//! check: run(exit=0)\n//! phase: run\n'+source)
    for tier in ['checked','native','release']:
        cmd=[a.wolf,'conform-run',str(entry),'--std-root',str(a.repo/'std'),'--json']
        cmd += ['--native','--release'] if tier=='release' else ['--'+tier]
        result=subprocess.run(cmd,cwd=case,capture_output=True,text=True,timeout=120)
        (case/(tier+'.json')).write_text(result.stdout)
        (case/(tier+'.stderr')).write_text(result.stderr)
        record=json.loads(result.stdout)
        ok=result.returncode==0 and record['verdict']=='exit(0)' and not record.get('warnings')
        print(('PASS ' if ok else 'FAIL ')+name+'/'+tier+' '+record['verdict'],flush=True)
        if not ok: raise SystemExit(1)
