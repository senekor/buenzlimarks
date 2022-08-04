dev_dir="$(dirname $0)"

[ -d $dev_dir/db      ] || mkdir $dev_dir/db
[ -f $dev_dir/db/data ] || touch $dev_dir/db/data

cd $dev_dir/../service
cargo run --bin migrate -- fresh
cargo run --bin insert_seeds
