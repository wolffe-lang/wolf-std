#!/usr/bin/env python3
"""Capped Linux process-RSS witness; region release is not secret erasure."""
import argparse
import os
from pathlib import Path
import resource
import subprocess

p=argparse.ArgumentParser()
p.add_argument('repo',type=Path)
p.add_argument('out',type=Path)
p.add_argument('--wolf',required=True)
a=p.parse_args()
a.out.mkdir(parents=True,exist_ok=True)
source=(a.repo/'tests/x/crypto/chacha20/chunk_lifetimes.lu').read_text()
assert source.count('for i in 0..4 {')==1
for count in [4,256]:
    case=a.out/str(count)
    case.mkdir(exist_ok=True)
    entry=case/'main.lu'
    entry.write_text(source.replace('for i in 0..4 {','for i in 0..'+str(count)+' {'))
    exe=case/'chunks'
    subprocess.run([a.wolf,'build',str(entry),'--std-root',str(a.repo/'std'),'-o',str(exe)],check=True)
    floor=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    with (case/'stdout.txt').open('w') as out, (case/'stderr.txt').open('w') as err:
        child=subprocess.Popen(['prlimit','--as=1073741824','--','timeout','300',str(exe)],stdout=out,stderr=err,cwd=case)
        _,status,usage=os.wait4(child.pid,0)
        child.returncode=os.waitstatus_to_exitcode(status)
    ok=child.returncode==0 and usage.ru_maxrss <= 256*1024
    print(('PASS ' if ok else 'FAIL ')+'bounded-chunks count='+str(count)+' bytes='+str(count*65536)+' parent_floor_kib='+str(floor)+' peak_kib='+str(usage.ru_maxrss)+' exit='+str(child.returncode),flush=True)
    if not ok: raise SystemExit(1)
