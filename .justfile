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
    @echo "- dist"
    @echo "- test"
    @echo "- clean"
    @echo

fmt:
	cargo fmt

fix:
	clear && cargo fmt && cargo check

check:
	clear && cargo check

clippy:
	clear && cargo fmt && cargo clippy --no-deps

run:
	clear && cargo fmt && cargo run

build:
	clear && cargo fmt && cargo build

release:
	clear && cargo fmt && cargo build --release

dist:
    ./target/release/dione

test:
    # cargo test
    # mostrar los println
    # clear && cargo fmt && cargo test --lib --tests -- --nocapture
    #
    @echo "inciando el backend"
    # clear && ./target/debug/dione && cargo fmt && cargo test --lib --tests -- --nocapture
    # clear && cargo fmt && cargo test --lib --tests -- --nocapture
    clear && cargo fmt && cargo test --lib --tests

# cleaning the house...
clean:
	@echo "cleaning the house..."
	# cargo clean
