# std.x — the nursery (D31)

Experimental modules land here as `std.x.<name>` with a **graduation
clock**: at each campaign closeout every resident either graduates into
the facade (a reviewed move — the path is the API) or is deleted.
Nothing lives in `x` across two closeouts. No module may depend on an
`x` resident except through its own tests.

Empty at sc00 by design — the clock starts when the first resident
arrives.
