#!/bin/bash

cd $(dirname $0)/../..

distro="$(cat /etc/os-release | grep ^ID= | cut -c 4-)"

if [ "$distro" = "fedora" ]; then
    packages=()
    sudo dnf install -yq ${packages}

    pip install -q gaphor

    crates=(
        sea-orm-cli
    )
    cargo install -q ${crates}

else
    echo "This OS is not supported, feel free to fix that."
    exit
fi
