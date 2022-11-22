# set -e

gir -o .
cargo build
# generate documentation
gir -c Gir.toml -d ./gir-files --doc-target-path docs.md -m doc
rustdoc-stripper -s -n -d ./src/
rustdoc-stripper -g -o docs.md -d ./src/
