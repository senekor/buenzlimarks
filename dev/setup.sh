#!/bin/bash
set -eo pipefail

cd "$(dirname "$0")/../.."

distro="$(grep ^ID= /etc/os-release | cut -c 4-)"

if [ "$distro" = "fedora" ]; then
    # packages=()
    # sudo dnf install -yq ${packages}

    # echo "Installing sea-orm-cli..."
    # crates=(
    #     sea-orm-cli
    # )
    # cargo install -q ${crates}

    printf "" # noop

else
    echo "This OS is not supported, feel free to fix that."
    exit
fi

if ! which pnpm &> /dev/null ; then
    echo "installing pnpm package manager..."
    sudo npm install -g pnpm
fi

if ! which d2 &> /dev/null ; then
    echo "installing d2 diagram renderer..."
    go install oss.terrastruct.com/d2@latest
fi
