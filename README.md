# wolf-std

The wolf standard library: the modules behind `use std.*`.

Written in wolf, tested from day one under two independent
implementations — lupin (the reference interpreter) executes every
module's tests now; the wolf compiler joins as it reaches native
execution. Library code lands here; the compiler-side half (prelude
wiring, intrinsics, `wolf build` integration) lives in wolf-lang.

Phase A covers the pure-computational core: collections, strings,
math, sorting, formatting, encodings, error and testing conventions.
OS-facing modules (io, fs, net, time, process) are scoped but wait on
the compiled runtime.

Sprint plan: the `std` track (`scNN`) in the wolf metarepo.
Dual-licensed MIT or Apache-2.0.
