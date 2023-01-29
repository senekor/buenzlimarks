#!/bin/bash

cd $(dirname $0)/../..

distro="$(cat /etc/os-release | grep ^ID= | cut -c 4-)"

if [ "$distro" = "fedora" ]; then
    # packages=()
    # sudo dnf install -yq ${packages}

    # echo "Installing sea-orm-cli..."
    # crates=(
    #     sea-orm-cli
    # )
    # cargo install -q ${crates}

    echo "No distro specific packages to install."

else
    echo "This OS is not supported, feel free to fix that."
    exit
fi

# d2 diagram renderer
go install oss.terrastruct.com/d2@latest
