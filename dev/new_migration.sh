dev_dir="$(dirname $0)"

cd $dev_dir/../server
# cargo run --bin migrate -- --migration-dir ./src/db/migration generate $1

# note that this only works based on my fork remlse/sea-orm.
# upstream expects migration-dir to be its own crate,
# (with src/ and src/lib.rs) not just a module directory.
sea-orm-cli migrate --migration-dir ./src/migrations generate $1
