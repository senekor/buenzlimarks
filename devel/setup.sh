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
    curl -L --proto '=https' --tlsv1.2 -sSf \
        https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
        | bash
fi
if ! which cargo-bin &> /dev/null ; then
    echo "installing cargo-run-bin to manage other devel tools..."
    cargo binstall -y cargo-run-bin
fi

if ! which d2 &> /dev/null ; then
    echo "installing d2 diagram renderer..."
    curl -fsSL https://d2lang.com/install.sh | sh -s -- --version v0.6.1
fi
