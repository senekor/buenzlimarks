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
    cd server && cargo run --bin insert_seeds
