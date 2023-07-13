#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

if [[ -n "$WATCHEXEC_EVENTS_FILE" && -n "$(cat "$WATCHEXEC_EVENTS_FILE")" ]] ; then
    # echo script was called by watchexec change event
    # parse and deduplicate change events from watchexec
    diagrams="$(sd '.*/([^/]*).d2' "\$1" "$WATCHEXEC_EVENTS_FILE" | sort -u)"
else
    diagrams=""
    for diagram in docs/diagrams/* ; do
        diagrams="$diagrams $(basename "${diagram%.*}")"
    done
fi

for diagram in $diagrams
do
    d2 --layout=elk --pad=32 --theme 200 \
        "docs/diagrams/$diagram.d2" \
        "docs/src/assets/gen/$diagram.png"
done
