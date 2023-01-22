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
db-reset:
    rm -fr dev/db
    mkdir -p dev/db
    cargo run -p cmd --bin insert_seeds
