#!/bin/bash
# sc75, wolf-std#98: env.arg0 / env.args_bytes witnesses on the native
# (debug), release and checked machines, and lupin's refusal.
# usage: run.sh STD WORKDIR   (WOLF, LUPIN from the environment)
set -u
STD=$1; WD=$2
W=${WOLF:?}; LU=${LUPIN:?}
rm -rf "$WD"; mkdir -p "$WD"; cd "$WD"
cp -R "$(dirname "$0")/show" "$(dirname "$0")/text" .
hex() { printf '%s' "$1" | od -An -tx1 | tr -d ' \n'; }
echo "host $(uname -srm); $($W --version | head -1); $($LU --version)"
for tier in debug release; do
  flag=; [ $tier = release ] && flag=--release
  for p in show text; do
    $W build $p/main.lu -o $p-$tier $flag --std-root "$STD" --no-cache > $p-$tier.build.log 2>&1
    echo "build $p $tier exit=$?"
  done
done
row() { # label, command...
  local label=$1; shift
  out=$("$@" 2>"$WD/.err"); rc=$?
  echo "--- $label: exit=$rc"
  echo "$out" | sed 's/^/    out: /'
  sed 's/^/    err: /' "$WD/.err"
}
for tier in debug release; do
  echo "=== native $tier"
  row "./show-$tier a b" ./show-$tier a b
  row "absolute path, one empty argument" "$PWD/show-$tier" ''
  mkdir -p pathdir && cp show-$tier pathdir/sc75show-$tier
  row "bare PATH name" env PATH="$PWD/pathdir:$PATH" sc75show-$tier x
  ln -sf show-$tier link-$tier
  row "symlink ./link-$tier" ./link-$tier y
  row "exec -a renamed" bash -c "exec -a renamed ./show-$tier z"
  row "argv[1] is the bytes ff fe" ./show-$tier $'\xff\xfe'
  row "text reader, argv[2] = ff (argv[1] = ok)" ./text-$tier ok $'\xff'
  row "text reader, all text" ./text-$tier "é" "🐺"
  row "exec -a with a name of byte c3" bash -c "exec -a \$'\\xc3' ./show-$tier w"
done
echo "=== checked (conform-run --checked --argv-hex)"
crow() { # label, program, hex...
  local label=$1 prog=$2; shift 2
  local args=()
  for h in "$@"; do args+=("--argv-hex=$h"); done
  out=$($W conform-run $prog/main.lu --checked --std-root "$STD" --json ${args[@]+"${args[@]}"} 2>"$WD/.err"); rc=$?
  echo "--- $label: exit=$rc"
  echo "$out" | grep -o '"verdict":"[^"]*"\|"stdout_inline":"[^"]*"\|"x-unsupported":"[^"]*"\|"message":"[^"]*"' | sed 's/^/    /'
  sed 's/^/    err: /' "$WD/.err" | head -5
}
crow "no --argv-hex" show
crow "prog a b" show $(hex prog) $(hex a) $(hex b)
crow "prog, an empty argument" show $(hex prog) ""
crow "prog, argv[1] = ff fe" show $(hex prog) fffe
crow "text reader, argv[1] = ff" text $(hex prog) ff
crow "text reader, all text" text $(hex prog) $(hex "é") $(hex "🐺")
crow "name is the byte c3" show c3 $(hex w)
echo "=== --argv-hex without --checked is refused"
out=$($W conform-run show/main.lu --native --std-root "$STD" --json --argv-hex=$(hex prog) 2>&1); echo "--- native with --argv-hex: exit=$? $(echo "$out" | head -3 | cut -c1-200)"
echo "=== lupin"
out=$(cd show && $LU conform-run main.lu --std-root "$STD" --json 2>&1); echo "--- lupin show: $(echo "$out" | grep -o '"verdict":"[^"]*"\|"x-unsupported":"[^"]*"' | tr '\n' ' ')"
echo "ARGV_WITNESS_DONE"
