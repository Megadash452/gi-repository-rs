set -e
proj_root=$(pwd)

# cd $proj_root/gir-files/
# sh fix.sh

cd $proj_root/gi-repository-sys/ || exit
rm Cargo.* || echo "No Cargo files to remove. Proceeding..."
gir -o .
cargo build
cargo test
