from rust:bookworm as build-base

run mkdir /work
workdir /work

# install build tools tools
run curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
run cargo binstall -y cargo-run-bin
copy Cargo.toml Cargo.lock .
run cargo bin --install
run rustup target add wasm32-unknown-unknown

from build-base as build-docs

copy docs docs
workdir /work/docs
run cargo bin mdbook build

from build-base as build-deps

copy models models
copy server/Cargo.toml server/

workdir /work/app
copy app/Cargo.toml app/Trunk.toml app/index.html app/tailwind.config.js app/tailwind.css .
copy app/assets assets
run mkdir src && echo "fn main() {}" > src/main.rs
run cargo bin trunk build --release

workdir /work/server
run mkdir src && echo "fn main() {}" > src/main.rs && touch src/lib.rs
run cargo build --release --bin buenzlimarks

from build-deps as build-app

workdir /work/app
copy app/src src
run touch src/main.rs && cargo bin trunk build --release

from build-deps as build-server

workdir /work/server
copy server/src src
run touch src/main.rs src/lib.rs && cargo build --release --bin buenzlimarks

from debian:bookworm-slim

# expected by tokio::signal::ctrl_c
stopsignal SIGINT

copy --from=build-docs /work/target/docs /docs
copy --from=build-app /work/target/app /app
copy --from=build-server /work/target/release/buenzlimarks /usr/local/bin/

cmd [ "/usr/local/bin/buenzlimarks" ]
