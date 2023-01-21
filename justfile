_default:
    @just --list

set dotenv-load

# run the backend
br:
    cd server && cargo run

# run the frontend
fr:
    cd app && pnpm dev

# initialize a new development database
db-init:
    [ -d dev/db      ] || mkdir dev/db
    [ -f dev/db/data ] || touch dev/db/data
    cd server && cargo run --bin migrate -- fresh
    cd server && cargo run --bin insert_seeds
