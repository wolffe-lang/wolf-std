#!/usr/bin/env bash
# sc84 census capture: build the census entry's default and release objects
# with the pinned wolf against a given std tree, then disassemble each object.
# Usage: capture.sh WOLF STD_ROOT ENTRY_DIR OUT_DIR
set -eu
wolf=$1 std=$2 entry=$3 out=$4
mkdir -p "$out"
cp "$entry/main.lu" "$out/main.lu"
"$wolf" --version | head -1 > "$out/wolf-version.txt"
for tier in default release; do
  rm -f "$out/$tier".*o "$out/$tier.o"
  flags=()
  [ "$tier" = release ] && flags=(--release)
  "$wolf" build "$out/main.lu" --std-root "$std" --emit=obj -o "$out/$tier.o" --no-cache ${flags[@]+"${flags[@]}"}
  for o in "$out/$tier".o "$out/$tier".*.o; do
    [ -e "$o" ] || continue
    nm "$o" > "${o%.o}.symbols"
    if command -v otool >/dev/null 2>&1; then
      otool -tvV "$o" > "${o%.o}.asm"
    else
      objdump -d --no-show-raw-insn "$o" > "${o%.o}.asm"
    fi
  done
  echo "captured $tier"
done
ls -la "$out"
