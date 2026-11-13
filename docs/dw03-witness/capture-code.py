#!/usr/bin/env python3
"""Capture generated objects/assembly for review; never certify timing."""
import argparse
from pathlib import Path
import os
import shutil
import subprocess

p=argparse.ArgumentParser()
p.add_argument('repo',type=Path)
p.add_argument('out',type=Path)
p.add_argument('--wolf',required=True)
a=p.parse_args()
a.out.mkdir(parents=True,exist_ok=True)
for name,row in [('equality','bytes/fixed_work.lu'),('x25519','x/crypto/curve25519/x25519_rfc7748_s52.lu'),('aead','x/crypto/chacha20/rfc8439_a5_aead.lu')]:
    stage=a.out/name
    stage.mkdir(exist_ok=True)
    entry=stage/'main.lu'
    entry.write_text((a.repo/'tests'/row).read_text())
    for backend in ['default','release']:
        obj=stage/(backend+'.o')
        cmd=[a.wolf,'build',str(entry),'--std-root',str(a.repo/'std'),'--emit=obj','-o',str(obj)]
        if backend=='release': cmd.append('--release')
        subprocess.run(cmd,check=True)
        objects = [obj] if obj.exists() else sorted(stage.glob(backend+'.*.o'))
        if not objects: raise RuntimeError('compiler produced no object files')
        for module in objects:
            symbols=subprocess.check_output(['nm',str(module)],text=True)
            module.with_suffix('.symbols').write_text(symbols)
            dis=['otool','-tvV',str(module)] if shutil.which('otool') else ['objdump','-dr',str(module)]
            result=subprocess.check_output(dis,text=True)
            module.with_suffix('.asm').write_text(result)
        print('PASS captured-object '+name+'/'+backend+' (review material, not a timing verdict)',flush=True)
