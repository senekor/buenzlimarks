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
