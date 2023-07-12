#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# distro="$(grep ^ID= /etc/os-release | cut -c 4-)"

# if [ "$distro" = "fedora" ]; then
#     # packages=()
#     # sudo dnf install -yq ${packages}

#     # echo "Installing sea-orm-cli..."
#     # crates=(
#     #     sea-orm-cli
#     # )
#     # cargo install -q ${crates}

#     printf "" # noop

# else
#     echo "This OS is not supported, feel free to fix that."
#     exit
# fi

if ! which cargo-binstall &> /dev/null ; then
    echo "installing cargo-binstall to install other crates faster..."
    cargo install --locked cargo-binstall
fi
if ! which trunk &> /dev/null ; then
    echo "installing trunk for webassembly frontend..."
    cargo binstall -y trunk
fi
if ! which cargo-watch &> /dev/null ; then
    echo "installing cargo-watch for backend auto-recompilation..."
    cargo binstall -y cargo-watch
fi
if ! which zellij &> /dev/null ; then
    echo "installing zellij for the terminal workspace..."
    cargo binstall -y zellij
fi

# if ! which d2 &> /dev/null ; then
#     echo "installing d2 diagram renderer..."
#     go install oss.terrastruct.com/d2@latest
# fi
