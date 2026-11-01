#!/bin/bash
# sc75: the gauntlet at the final head, then the evidence by hand at the
# same checkout, in one container. usage: final-inner.sh CHECKOUT OUT
bash /lane/gate/inner.sh "$1"
echo "GAUNTLET_RC=$?"
bash /lane/ev-inner.sh "$1" "$2"
