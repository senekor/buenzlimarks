#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

diagrams="$(cat < "${1:-/dev/stdin}" | sd '.*/([^/]*).d2' "\$1" | sort -u)"

# first render without change events
if [[ -z "$diagrams" ]] ; then
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
