#!/bin/bash
set -eo pipefail

cd "$(git rev-parse --show-toplevel)"

# install system dependencies
if grep -q "fedora" /etc/os-release; then
    echo "Installing system dependencies..."
    sudo dnf install -y \
        openssl-devel \
        perl-FindBin \
        perl-File-Compare
else
    echo "Unknown OS, make sure you have the necessary"
    echo "packages installed."
fi

if ! which cargo-binstall &> /dev/null ; then
    echo "installing cargo-binstall to install other crates faster..."
    cargo install --locked cargo-binstall
fi
if ! which just &> /dev/null ; then
    echo "installing the just command runner..."
    cargo install -y just
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
if ! which mdbook &> /dev/null ; then
    echo "installing mdbook to build the documentation..."
    cargo binstall -y mdbook
fi

if ! which d2 &> /dev/null ; then
    echo "installing d2 diagram renderer..."
    curl -fsSL https://d2lang.com/install.sh | sh -s -- --version v0.6.0
fi
if ! which watchexec &> /dev/null ; then
    echo "installing watchexec to watch for diagram changes..."
    cargo binstall -y watchexec-cli
fi
