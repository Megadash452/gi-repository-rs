proj_root=$(pwd)

# cd $proj_root/gir-files/
# sh fix.sh

# cd ../gi-repository-sys/
cd $proj_root/gi-repository-sys/ || exit
rm Cargo.*
gir -o .
cargo build
cargo test