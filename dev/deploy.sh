#!/bin/bash
set -e

dev_dir="$(dirname $0)"
cd $dev_dir/..

git checkout main
git pull

cd ./service
source .env
cargo build --release --bin buenzlimarks
cd ..

cd ./app
pnpm build
cd ..

killall buenzlimarks || true
./service/target/release/buenzlimarks &
