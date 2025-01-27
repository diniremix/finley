# Just task runner

# 2025-01-12
set dotenv-load := false
just_home := justfile_directory()

# help
default: help

# show justfile config file
help:
    @echo
    @echo "The config file is at: {{ just_home }}"
    @echo "projects tasks:"
    @echo
    @echo "- fmt"
    @echo "- fix"
    @echo "- check"
    @echo "- clippy"
    @echo "- serve"
    @echo "- run"
    @echo "- build"
    @echo "- release"
    @echo "- docs"
    @echo "- enc"
    @echo "- dec"
    @echo "- dist"
    @echo "- test"
    @echo "- clean"
    @echo

fmt:
	clear && cargo fmt

fix: fmt
	cargo check

check: fmt
	cargo check

clippy: fmt
	cargo clippy --no-deps

run: fmt
	cargo run

build: fmt
	cargo build

release: fmt
	cargo build --release

docs:
	RUSTDOCFLAGS="--cfg=docsrs" cargo doc --no-deps --document-private-items --release

enc: clean build
    # fnl
    clear && ./target/debug/finley enc --input ./test/demo1_num.txt --output ./test/salida2.fnl

dec: build
    clear && ./target/debug/finley dec --input ./test/salida2.fnl --output ./test/salida-orig.txt

dist:
    ./target/release/finley

test: fmt
    # cargo test
    cargo test

# cleaning the house...
clean:
    clear
    @echo "cleaning the house..."
    # cargo clean
    rm -f ./test/salida2.fnl
    rm -f ./test/salida-orig.txt
