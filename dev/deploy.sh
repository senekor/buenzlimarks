#!/bin/bash
set -e

dev_dir="$(dirname $0)"
cd $dev_dir/..

git checkout main
git pull

cd ./app
pnpm build
cd ..

cd ./service
source .env
cargo build --release --bin buenzlimarks

killall buenzlimarks || true
./target/release/buenzlimarks &
